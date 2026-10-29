/// Lower [`RequestClass::priority`] wins inside a lane family.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestClass {
    InteractiveRead,
    BackgroundRead,
    InteractiveMedia,
    BackgroundMedia,
    InteractiveWrite,
}

impl RequestClass {
    pub fn bits(self) -> u8 {
        match self {
            RequestClass::InteractiveRead => 0,
            RequestClass::BackgroundRead => 1,
            RequestClass::InteractiveMedia => 2,
            RequestClass::BackgroundMedia => 3,
            RequestClass::InteractiveWrite => 4,
        }
    }

    pub fn priority(self) -> u8 {
        match self {
            RequestClass::InteractiveWrite => 0,
            RequestClass::InteractiveRead => 1,
            RequestClass::InteractiveMedia => 2,
            RequestClass::BackgroundRead => 3,
            RequestClass::BackgroundMedia => 4,
        }
    }

    pub fn from_bits(bits: u8) -> Option<RequestClass> {
        match bits {
            0 => Some(RequestClass::InteractiveRead),
            1 => Some(RequestClass::BackgroundRead),
            2 => Some(RequestClass::InteractiveMedia),
            3 => Some(RequestClass::BackgroundMedia),
            4 => Some(RequestClass::InteractiveWrite),
            _ => None,
        }
    }
}

impl std::fmt::Display for RequestClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            RequestClass::InteractiveRead => "read",
            RequestClass::BackgroundRead => "background-read",
            RequestClass::InteractiveMedia => "media",
            RequestClass::BackgroundMedia => "background-media",
            RequestClass::InteractiveWrite => "write",
        };
        f.write_str(name)
    }
}

thread_local! {
    /// Set per request by the bridge, in the same context that carries cancellation.
    static DISPATCH_CLASS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

pub fn set_current_class(class: RequestClass) {
    DISPATCH_CLASS.with(|cell| cell.set(class.bits()));
}

pub fn current_class() -> RequestClass {
    DISPATCH_CLASS
        .with(|cell| RequestClass::from_bits(cell.get()))
        .unwrap_or(RequestClass::InteractiveRead)
}

pub fn with_class<T>(class: RequestClass, body: impl FnOnce() -> T) -> T {
    struct Restore(u8);
    impl Drop for Restore {
        fn drop(&mut self) {
            DISPATCH_CLASS.with(|cell| cell.set(self.0));
        }
    }
    let previous = DISPATCH_CLASS.with(|cell| cell.replace(class.bits()));
    let _restore = Restore(previous);
    body()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LaneFamily {
    Read,
    Media,
}

pub fn family(class: RequestClass) -> LaneFamily {
    match class {
        RequestClass::InteractiveWrite
        | RequestClass::InteractiveRead
        | RequestClass::BackgroundRead => LaneFamily::Read,
        RequestClass::InteractiveMedia | RequestClass::BackgroundMedia => LaneFamily::Media,
    }
}
