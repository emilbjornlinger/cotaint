pub struct Tainted<T> {
    val: T
}

impl<T> Tainted<T> {
    pub fn new(val: T) -> Self {
        Self {
            val
        }
    }

    pub fn sanitize(self: Self) -> T {
        self.val
    }
}
