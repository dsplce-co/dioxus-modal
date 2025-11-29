pub struct Modal<T> {
    pub open: Box<dyn Fn(T)>,
    pub close: fn(),
}

impl<T> Modal<T> {
    pub fn open(&self, ctx: T) {
        (self.open)(ctx);
    }
}
