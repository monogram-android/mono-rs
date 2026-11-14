//! Controlled native baselines; run with `cargo test -p mono-proto-runtime --lib
//! transport_performance -- --ignored --nocapture --test-threads=1`.
//! All peers and keys are synthetic. No public endpoint or persistence is used.
//! Raw wall-clock samples are observations, not cross-machine speed assertions.
use super::*;
use crate::request_control as control;
use crate::scheduler::{LaneGate, RequestClass};
use mono_proto_crypto::{AesCtr, Direction, MessageToEncrypt, decrypt_message, encrypt_message};
use mono_proto_transport::Connection;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

const KEY: [u8; 256] = [7; 256];
const WARMUPS: usize = 3;
const SAMPLES: usize = 11;
const MAX_BATCH: usize = 100;

#[derive(Default)]
struct Sample {
    ns: u128,
    sent: usize,
    received: usize,
    queue_ns: u128,
    max_queue_ns: u128,
    cancel_ns: u128,
}

fn measure(label: &str, frames: usize, mut run: impl FnMut() -> Sample) {
    for _ in 0..WARMUPS {
        std::hint::black_box(run());
    }
    for sample in 0..SAMPLES {
        let result = run();
        println!(
            "native_perf stage={label} frames={frames} warmups={WARMUPS} sample={sample} ns={} sent_bytes={} received_bytes={} queue_ns={} max_queue_ns={} cancel_ns={}",
            result.ns,
            result.sent,
            result.received,
            result.queue_ns,
            result.max_queue_ns,
            result.cancel_ns
        );
    }
}

fn snapshot() -> Snapshot {
    let mut state = Snapshot::new(2, &mut OsRandom).unwrap();
    state.set_auth_key(KEY.to_vec()).unwrap();
    state
}

fn policy() -> ExponentialBackoff {
    ExponentialBackoff {
        timeout_micros: 5_000_000,
        initial_delay_micros: 0,
        max_attempts: 2,
    }
}

fn request() -> Vec<u8> {
    encode_boxed_bytes(&mono_proto::latest::api::HelpGetNearestDcRequest {}).unwrap()
}

fn response() -> Vec<u8> {
    encode_boxed_bytes(&mono_proto::latest::api::NearestDcConstructor {
        country: "ZZ".into(),
        this_dc: 2,
        nearest_dc: 2,
    })
    .unwrap()
}

/// Minimal local server: real native crypto, but no Telegram server semantics.
struct Peer {
    socket: TcpStream,
    incoming: AesCtr,
    outgoing: AesCtr,
    sent: usize,
    received: usize,
}

impl Peer {
    fn accept(listener: &TcpListener) -> Self {
        let (mut socket, _) = listener.accept().unwrap();
        socket.set_nodelay(true).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        socket
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut header = [0; 64];
        socket.read_exact(&mut header).unwrap();
        let mut incoming = AesCtr::new(&header[8..40], &header[40..56]).unwrap();
        let mut decoded = header;
        incoming.apply(&mut decoded);
        assert_eq!(&decoded[56..60], &0xdddddddd_u32.to_le_bytes());
        header.reverse();
        let outgoing = AesCtr::new(&header[8..40], &header[40..56]).unwrap();
        Self {
            socket,
            incoming,
            outgoing,
            sent: 0,
            received: 64,
        }
    }

    fn receive(&mut self, session_id: i64) -> mono_proto_crypto::DecryptedMessage {
        let mut length = [0; 4];
        self.socket.read_exact(&mut length).unwrap();
        self.incoming.apply(&mut length);
        let length = u32::from_le_bytes(length) as usize;
        assert!((24..=128 * 1024).contains(&length));
        let mut bytes = vec![0; length];
        self.socket.read_exact(&mut bytes).unwrap();
        self.received += 4 + length;
        self.incoming.apply(&mut bytes);
        let sealed_len = 24 + ((bytes.len() - 24) / 16) * 16;
        decrypt_message(
            &KEY,
            &bytes[..sealed_len],
            Direction::ClientToServer,
            session_id,
            128 * 1024,
        )
        .unwrap()
    }

    fn respond(&mut self, request_id: i64, session_id: i64, ordinal: usize) {
        let mut body = RPC_RESULT.to_le_bytes().to_vec();
        body.extend_from_slice(&request_id.to_le_bytes());
        body.extend_from_slice(&response());
        let sealed = encrypt_message(
            &KEY,
            MessageToEncrypt {
                server_salt: 0,
                session_id,
                message_id: ((SystemClock.unix_micros() / 1_000_000) << 32)
                    | 1
                    | ((ordinal as i64) << 2),
                sequence: (ordinal as i32) * 2 + 1,
                body: &body,
                padding: &framing::make_padding(body.len()).unwrap(),
            },
            Direction::ServerToClient,
        )
        .unwrap();
        let mut framed = PaddedIntermediate::default().encode(&sealed).unwrap();
        self.outgoing.apply(&mut framed);
        self.sent += framed.len();
        // Force fragmentation for every response, including the length prefix.
        self.socket.write_all(&framed[..2]).unwrap();
        self.socket.write_all(&framed[2..]).unwrap();
    }
}

#[test]
#[ignore = "controlled native performance baseline"]
fn native_localhost_rpc_round_trips_1_80_1000() {
    // Timing includes client connect/header, native batch pump and final ACKs;
    // excludes listener/thread setup and server join. Byte counts include the
    // obfuscation header and service frames read by the peer, plus a final EOF drain.
    for frames in [1, 80, 1000] {
        measure("rpc_loopback", frames, || {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap().to_string();
            let state = snapshot();
            let session_id = state.session_id;
            let peer = std::thread::spawn(move || {
                let mut peer = Peer::accept(&listener);
                let expected = request();
                let mut completed = 0;
                while completed < frames {
                    let message = peer.receive(session_id);
                    if message.body != expected {
                        let constructor = u32::from_le_bytes(message.body[..4].try_into().unwrap());
                        assert!(
                            constructor == MSGS_ACK
                                || constructor == mono_proto::transport::GetFutureSaltsRequest::ID,
                            "unexpected service constructor {constructor:#x}"
                        );
                        continue;
                    }
                    peer.respond(message.message_id, session_id, completed);
                    completed += 1;
                }
                // Include trailing ACK bytes and wait for the client to finish.
                let mut tail = Vec::new();
                peer.socket.read_to_end(&mut tail).unwrap();
                peer.received += tail.len();
                (peer.received, peer.sent)
            });
            let started = Instant::now();
            let mut live = open_live_addr(2, &address, None, 5).unwrap();
            let mut engine = Engine::new(state, policy()).unwrap();
            for offset in (0..frames).step_by(MAX_BATCH) {
                let bodies = vec![request(); (frames - offset).min(MAX_BATCH)];
                assert!(bodies.len() <= MAX_BATCH);
                let deadline = Instant::now() + Duration::from_secs(5);
                let results = invoke::invoke_batch_until_results(
                    &mut engine,
                    &mut live,
                    &bodies,
                    &SystemClock,
                    deadline,
                    deadline,
                    deadline,
                    false,
                    &mut false,
                )
                .unwrap();
                assert_eq!(results.len(), bodies.len());
                for result in results {
                    assert_eq!(result.unwrap(), response());
                }
                assert_eq!(engine.pending_count(), 0);
            }
            let ns = started.elapsed().as_nanos();
            drop(live);
            let (sent, received) = peer.join().unwrap();
            assert!(sent >= 64 + frames * 4);
            assert!(received >= frames * 4);
            Sample {
                ns,
                sent,
                received,
                ..Default::default()
            }
        });
    }
}

#[test]
#[ignore = "controlled native performance baseline"]
fn native_crypto_and_frame_payload_workloads() {
    // Opaque bodies isolate engine/session/AES-IGE/framing cost. They are not
    // claimed to be schema-valid RPC requests or actual file-transfer traffic.
    for (frames, body_len) in [(1, 32), (80, 4096), (1000, 4096)] {
        measure(&format!("crypto_frame_{body_len}B"), frames, || {
            let state = snapshot();
            let mut engine = Engine::new(state, policy()).unwrap();
            let body = vec![0x5a; body_len];
            let mut sent = 0;
            let mut received = 0;
            let started = Instant::now();
            for index in 0..frames {
                let handle = engine
                    .invoke(&RawMethod { body: body.clone() }, &SystemClock)
                    .unwrap();
                let outbound = engine.next_outbound().unwrap();
                let sealed = engine
                    .seal_outbound(&outbound, &framing::make_padding(body.len()).unwrap())
                    .unwrap();
                let wire = PaddedIntermediate::default().encode(&sealed).unwrap();
                let packet = PaddedIntermediate::default().decode(&wire).unwrap();
                assert_eq!(packet.consumed, wire.len());
                let sealed_len = 24 + ((packet.payload.len() - 24) / 16) * 16;
                let decoded = decrypt_message(
                    &KEY,
                    &packet.payload[..sealed_len],
                    Direction::ClientToServer,
                    engine.session.session_id,
                    body_len,
                )
                .unwrap();
                assert_eq!(decoded.body, body);
                sent += wire.len();
                let server_id =
                    ((SystemClock.unix_micros() / 1_000_000) << 32) | 1 | ((index as i64) << 2);
                let reply = encrypt_message(
                    &KEY,
                    MessageToEncrypt {
                        server_salt: 0,
                        session_id: engine.session.session_id,
                        message_id: server_id,
                        sequence: 1,
                        body: &body,
                        padding: &framing::make_padding(body.len()).unwrap(),
                    },
                    Direction::ServerToClient,
                )
                .unwrap();
                let reply_wire = PaddedIntermediate::default().encode(&reply).unwrap();
                let packet = PaddedIntermediate::default().decode(&reply_wire).unwrap();
                let sealed_len = 24 + ((packet.payload.len() - 24) / 16) * 16;
                let inbound = engine
                    .open_inbound(&packet.payload[..sealed_len], &SystemClock, true, body_len)
                    .unwrap();
                assert_eq!(inbound.disposition, ReceivedMessageResult::Accepted);
                engine
                    .receive_result(outbound.message_id, inbound.message.body, server_id)
                    .unwrap();
                assert_eq!(
                    engine.take_response::<RawMethod>(&handle).unwrap(),
                    Some(body.clone())
                );
                received += reply_wire.len();
                assert_eq!(engine.pending_count(), 0);
            }
            Sample {
                ns: started.elapsed().as_nanos(),
                sent,
                received,
                ..Default::default()
            }
        });
    }
}

#[test]
#[ignore = "controlled native performance baseline"]
fn native_retry_local_reconnect_and_request_cancellation() {
    // Reconnect is explicitly driven to a second localhost socket. This proves
    // Engine retry/correlation and socket shutdown, not public-DC retry routing.
    measure("retry_reconnect_cancel", 1, || {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        let mut engine = Engine::new(snapshot(), policy()).unwrap();
        let session_id = engine.session.session_id;
        let (closed_tx, closed_rx) = std::sync::mpsc::channel();
        let peer = std::thread::spawn(move || {
            let mut first = Peer::accept(&listener);
            let original = first.receive(session_id);
            assert_eq!(original.body, request());
            let mut received = first.received;
            drop(first);
            let mut second = Peer::accept(&listener);
            let retried = second.receive(session_id);
            assert_ne!(original.message_id, retried.message_id);
            assert_eq!(original.body, retried.body);
            second.respond(retried.message_id, session_id, 0);
            let mut tail = Vec::new();
            second.socket.read_to_end(&mut tail).unwrap();
            closed_tx.send(()).unwrap();
            received += second.received + tail.len();
            (received, second.sent)
        });
        let started = Instant::now();
        let mut first = open_live_addr(2, &address, None, 5).unwrap();
        let handle = engine
            .invoke(&RawMethod { body: request() }, &SystemClock)
            .unwrap();
        let original = engine.next_outbound().unwrap();
        let sealed = engine
            .seal_outbound(
                &original,
                &framing::make_padding(original.body.len()).unwrap(),
            )
            .unwrap();
        framing::send_framed(&mut first.conn, &mut first.framing, &sealed).unwrap();
        assert!(matches!(first.conn.receive(&mut [0; 1]), Ok(0) | Err(_)));
        engine
            .fail_request(
                original.message_id,
                SystemClock.unix_micros(),
                &mono_proto_engine::Error::Transport(mono_proto_transport::Error::Closed),
            )
            .unwrap();
        assert!(engine.poll(&SystemClock).unwrap().is_empty());
        let retry = engine.next_outbound().unwrap();
        assert_ne!(retry.message_id, original.message_id);
        assert!(
            engine
                .receive_result(original.message_id, vec![], 1)
                .is_err()
        );
        drop(first);
        let request_id = control::create();
        let previous = control::bind(request_id);
        let mut second = open_live_addr(2, &address, None, 5).unwrap();
        let sealed = engine
            .seal_outbound(&retry, &framing::make_padding(retry.body.len()).unwrap())
            .unwrap();
        framing::send_framed(&mut second.conn, &mut second.framing, &sealed).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        let packet = framing::recv_framed(
            &mut second.conn,
            &mut second.framing,
            &mut second.input,
            &mut 0,
            deadline,
            deadline,
            false,
            500,
        )
        .unwrap();
        let packet = trim_padded_mtproto_packet(&packet);
        let inbound = engine
            .open_inbound(packet, &SystemClock, true, 4096)
            .unwrap();
        let events = parse_authenticated(
            &inbound.message.body,
            inbound.message.message_id,
            &mut engine.session,
            &SystemClock,
        )
        .unwrap();
        for event in events {
            if let InboundEvent::RpcResult { req_msg_id, body } = event {
                engine
                    .receive_result(req_msg_id, body, inbound.message.message_id)
                    .unwrap();
            }
        }
        assert_eq!(
            engine.take_response::<RawMethod>(&handle).unwrap(),
            Some(response())
        );
        assert_eq!(engine.take_response::<RawMethod>(&handle).unwrap(), None);
        assert_eq!(engine.pending_count(), 0);
        let cancel_started = Instant::now();
        control::cancel(request_id);
        assert!(control::check().is_err());
        assert!(matches!(second.conn.receive(&mut [0; 1]), Ok(0) | Err(_)));
        // The peer observes EOF before the client socket is dropped.
        closed_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let cancel_ns = cancel_started.elapsed().as_nanos();
        control::bind(previous);
        control::release(request_id);
        let ns = started.elapsed().as_nanos();
        drop(second);
        let (sent, received) = peer.join().unwrap();
        Sample {
            ns,
            sent,
            received,
            cancel_ns,
            ..Default::default()
        }
    });
}

#[test]
#[ignore = "controlled native performance baseline"]
fn native_scheduler_contention_and_request_control() {
    measure("scheduler_queued_cancel", 1, || {
        let gate = Arc::new(LaneGate::new());
        let held = gate.acquire(RequestClass::InteractiveRead).unwrap();
        let id = control::create();
        let waiting_gate = gate.clone();
        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            control::with_shared_binding(id, || {
                done_tx
                    .send(waiting_gate.acquire(RequestClass::BackgroundRead).is_err())
                    .unwrap();
            });
        });
        let started = Instant::now();
        let deadline = started + Duration::from_secs(5);
        while gate.waiter_count() != 1 {
            assert!(Instant::now() < deadline, "cancel target did not queue");
            std::thread::yield_now();
        }
        let cancel_started = Instant::now();
        control::cancel(id);
        assert!(done_rx.recv_timeout(Duration::from_secs(5)).unwrap());
        let cancel_ns = cancel_started.elapsed().as_nanos();
        worker.join().unwrap();
        assert_eq!(gate.waiter_count(), 0);
        // Cancellation removes the waiter without stealing the occupied lane.
        assert!(gate.try_acquire(RequestClass::InteractiveRead).is_none());
        drop(held);
        assert!(gate.try_acquire(RequestClass::BackgroundRead).is_some());
        control::release(id);
        Sample {
            ns: started.elapsed().as_nanos(),
            cancel_ns,
            ..Default::default()
        }
    });
    for operations in [1, 80, 1000] {
        measure("scheduler_request_control", operations, || {
            let gate = Arc::new(LaneGate::new());
            // A queued foreground write must beat a background ticket.
            let held = gate.acquire(RequestClass::InteractiveRead).unwrap();
            let background = gate.enqueue(RequestClass::BackgroundRead).unwrap();
            let write = gate.enqueue(RequestClass::InteractiveWrite).unwrap();
            assert!(write.try_acquire().is_none());
            drop(held);
            assert!(background.try_acquire().is_none());
            drop(write.try_acquire().unwrap());
            drop(write);
            drop(background.try_acquire().unwrap());
            drop(background);
            let workers = operations.min(8);
            let startup_lease = gate.acquire(RequestClass::InteractiveRead).unwrap();
            let barrier = Arc::new(Barrier::new(workers + 1));
            let mut threads = Vec::new();
            for worker in 0..workers {
                let gate = gate.clone();
                let barrier = barrier.clone();
                threads.push(std::thread::spawn(move || {
                    barrier.wait();
                    let mut total = 0;
                    let mut maximum = 0;
                    let mut completed = 0;
                    for _ in (worker..operations).step_by(workers) {
                        let id = control::create();
                        let previous = control::bind(id);
                        control::promote(id, RequestClass::InteractiveRead);
                        assert_eq!(
                            control::promoted_class(),
                            Some(RequestClass::InteractiveRead)
                        );
                        let queued = Instant::now();
                        let held = gate.acquire(RequestClass::InteractiveRead).unwrap();
                        let latency = queued.elapsed().as_nanos();
                        total += latency;
                        maximum = maximum.max(latency);
                        control::while_active(|| std::hint::black_box(id)).unwrap();
                        // Yield under the lease to create real lane contention.
                        std::thread::yield_now();
                        drop(held);
                        control::cancel(id);
                        assert!(gate.acquire(RequestClass::InteractiveRead).is_err());
                        control::bind(previous);
                        control::release(id);
                        completed += 1;
                    }
                    (total, maximum, completed)
                }));
            }
            let started = Instant::now();
            barrier.wait();
            let admission_deadline = Instant::now() + Duration::from_secs(5);
            while gate.waiter_count() != workers {
                assert!(Instant::now() < admission_deadline, "workers did not queue");
                std::thread::yield_now();
            }
            drop(startup_lease);
            let mut queue_ns = 0;
            let mut max_queue_ns = 0;
            let mut completed = 0;
            for thread in threads {
                let (total, maximum, count) = thread.join().unwrap();
                queue_ns += total;
                max_queue_ns = max_queue_ns.max(maximum);
                completed += count;
            }
            assert_eq!(completed, operations);
            assert_eq!(gate.waiter_count(), 0);
            assert!(gate.try_acquire(RequestClass::BackgroundRead).is_some());
            Sample {
                ns: started.elapsed().as_nanos(),
                queue_ns,
                max_queue_ns,
                ..Default::default()
            }
        });
    }
}
