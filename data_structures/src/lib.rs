use std::cell::Cell;


#[derive(Debug)]
pub struct LazyCell<T: Copy> {
    value: Cell<Option<T>>
}

impl<T: Copy> LazyCell<T> {
    pub fn new() -> Self {
        LazyCell { value: Cell::new(None) }
    }

    pub fn get<F: FnOnce() -> T>(&self, init: F) -> T {
        if let Some(x) = self.value.get() {
            return x;
        }
        let x = init();
        self.value.set(Some(x));
        return x;
    }
}

