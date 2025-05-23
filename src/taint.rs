#[macro_export]
macro_rules! taint_block {
    ($e:block) => {
        taint_macros::taint_block!($e)
    };
}

#[macro_export]
macro_rules! taint_block_return {
    ($e:block) => {
        taint_macros::taint_block_return!($e)
    };
}

#[derive(Clone, Default)]
pub struct Tainted<T>
where
    T: SafeTaintValue,
{
    val: T,
}

/*
 * The type that represents a tainted value in the programming model
 */
impl<T> Tainted<T>
where
    T: SafeTaintValue,
{
    pub unsafe fn new(val: T) -> Self {
        Self { val }
    }

    /*
     * Only one sanitize function because we always want to consume the tainted type and return
     * the inner value
     */
    pub fn sanitize<F>(self: Self, closure: F) -> T
    where
        F: FnOnce(T) -> T,
    {
        closure(self.val)
    }

    /*
     * Extract tainted value and consume the tainted struct, called from closure guards
     */
    pub unsafe fn extract_and_consume(self) -> T {
        self.val
    }

    /*
     * Extract tainted value as reference, called from closure guards
     */
    pub unsafe fn extract_as_ref(&self) -> &T {
        &self.val
    }

    /*
     * Extract tainted value as mutable reference, called from closure guards
     */
    pub unsafe fn extract_as_mut_ref(&mut self) -> &mut T {
        &mut self.val
    }
}

/*
 * Calls a closure that will only capture variables according to annotated traits. Used by a
 * procedural macro expansion.
 */
pub fn closure_guard<F>(closure: F)
where
    F: FnOnce() + SideEffectFreeCapture,
{
    closure()
}

/*
 * Calls a closure that will only capture variables according to annotated traits. Returns a value
 * that is tainted. Used by a procedural macro expansion.
 */
pub fn closure_guard_return<F, T>(closure: F) -> Tainted<T>
where
    F: FnOnce() -> Tainted<T> + SideEffectFreeCapture,
    T: SafeTaintValue,
{
    closure()
}

/*
 * Create a trait that will not be implemented by Tainted<T> types
 */
pub unsafe auto trait NonTaintValues {}
impl<T: ?Sized> !NonTaintValues for Tainted<T> {}

/*
 * Trait that implies implementors can safely be captured by closure guards
 */
pub unsafe auto trait SideEffectFreeCapture {}

/*
 * Specify that unsafe types are not implementing the SideEffectFreeCapture trait
 */
impl<T: NonTaintValues> !SideEffectFreeCapture for &mut T {}
impl<T: NonTaintValues> !SideEffectFreeCapture for *mut T {}
impl<T: ?Sized> !SideEffectFreeCapture for std::cell::UnsafeCell<T> {}

/*
 * Specify that Tainted references implement SideEffectFreeCapture and thus okay to capture in
 * closure guards.
 */
unsafe impl<T: SafeTaintValue> SideEffectFreeCapture for &mut Tainted<T> {}
unsafe impl<T: SafeTaintValue> SideEffectFreeCapture for &mut &mut Tainted<T> {}

/*
 * Implement the Immutable trait for all types except for an UnsafeCell and mutable references
 */
pub unsafe auto trait Immutable {}
impl<T: ?Sized> !Immutable for std::cell::UnsafeCell<T> {}
impl<T: ?Sized> !Immutable for &mut T {}

/*
 * Mark all Immutable types (no interior mutability) as SafeTaintValues that can be used in a
 * Tainted struct
 */
pub unsafe trait SafeTaintValue {}
unsafe impl<T> SafeTaintValue for T where T: Immutable {}
