use std::cell::RefCell;

use mono_proto_transport::PaddedIntermediate;

use crate::tcp::ObfuscatedTcp;

/// Reused obfuscated TCP for sequential RPCs on one client (new socket per RPC times out).
pub struct LiveTransport {
    pub ping_sent: Option<std::time::Instant>,
    pub updates: Vec<Vec<u8>>,
    pub updates_bytes: usize,
    pub dc_id: i32,
    pub endpoint: String,
    pub conn: ObfuscatedTcp,
    pub framing: PaddedIntermediate,
    pub input: Vec<u8>,
    /// Last send/recv on this TCP; stale reuse probes ping first.
    pub last_io: std::time::Instant,
}

// Per-lane stack. Nested media must not swap away the home TCP. Drop only the top frame.
thread_local! {
    static LIVE: RefCell<Vec<Option<LiveTransport>>> = const { RefCell::new(Vec::new()) };
    pub static SUPERVISING: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
}

pub fn with_live_transport<T>(slot: &mut Option<LiveTransport>, body: impl FnOnce() -> T) -> T {
    LIVE.with(|cell| {
        struct Pop<'a>(&'a RefCell<Vec<Option<LiveTransport>>>);
        impl Drop for Pop<'_> {
            fn drop(&mut self) {
                self.0.borrow_mut().pop();
                SUPERVISING.with(|stack| {
                    stack.borrow_mut().pop();
                });
            }
        }
        cell.borrow_mut().push(slot.take());
        SUPERVISING.with(|stack| stack.borrow_mut().push(false));
        let _pop = Pop(cell);
        let result = body();
        *slot = cell.borrow_mut().last_mut().and_then(Option::take);
        result
    })
}

pub fn drop_live_transport() {
    LIVE.with(|cell| {
        if let Some(top) = cell.borrow_mut().last_mut() {
            *top = None;
        }
    });
}

pub fn take_live(dc_id: i32) -> Option<LiveTransport> {
    LIVE.with(|cell| {
        let mut stack = cell.borrow_mut();
        let top = stack.last_mut()?;
        match top.as_ref() {
            Some(existing) if existing.dc_id == dc_id => top.take(),
            _ => {
                *top = None;
                None
            }
        }
    })
}

pub fn queue_rpc_updates(dc_id: i32, body: Vec<u8>) -> Result<(), crate::MtprotoError> {
    let mut transport = take_live(dc_id).ok_or_else(|| {
        crate::MtprotoError::Message("management updates require recovery".into())
    })?;
    super::framing::queue_update(&mut transport, body)?;
    put_live(transport);
    Ok(())
}

pub fn put_live(transport: LiveTransport) {
    LIVE.with(|cell| {
        let mut stack = cell.borrow_mut();
        if let Some(top) = stack.last_mut() {
            *top = Some(transport);
        }
    });
}

#[cfg(test)]
pub fn live_stack_depth() -> usize {
    LIVE.with(|cell| cell.borrow().len())
}

#[cfg(test)]
pub fn live_top_occupied() -> bool {
    LIVE.with(|cell| {
        cell.borrow()
            .last()
            .and_then(|slot| slot.as_ref())
            .is_some()
    })
}
