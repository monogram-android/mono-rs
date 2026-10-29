use std::cell::Cell;

thread_local! {
    static WITHOUT_UPDATES: Cell<bool> = const { Cell::new(false) };
}

pub fn invoking_without_updates() -> bool {
    WITHOUT_UPDATES.with(Cell::get)
}

pub fn with_invoke_without_updates<T>(body: impl FnOnce() -> T) -> T {
    WITHOUT_UPDATES.with(|cell| {
        struct Restore<'a>(&'a Cell<bool>, bool);
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                self.0.set(self.1);
            }
        }
        let previous = cell.replace(true);
        let _restore = Restore(cell, previous);
        body()
    })
}
