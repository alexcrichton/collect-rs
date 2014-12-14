//! Proof-of-concept trait for intrusive iterators.

/// Internal Iterators.
pub trait InternalIterator<T> {
    /// Run this Iterator using the provided closure.
    fn run<F: FnMut(T) -> bool>(self, F);
}

/// Extension methods for Internal Iterators
pub trait InternalIteratorExt<T> : InternalIterator<T> {
    /// Get another intrusive iterator with its contents modified by the closure.
    fn map<O, F: FnMut(T) -> O>(self, f: F) -> Map<T, O, Self, F> {
        Map { iter: self, closure: f }
    }

    fn count(self) -> uint {
        let mut count = 0;
        self.run(|_| { count += 1; false });
        count
    }
}

impl<T, I: InternalIterator<T>> InternalIteratorExt<T> for I {}

/// An InternalIterator that maps over the contents of
/// another IntrusiveIterator.
pub struct Map<T, O, I: InternalIterator<T>, F: FnMut(T) -> O> {
    iter: I,
    closure: F
}

impl<T, O, I: InternalIterator<T>, F: FnMut(T) -> O> InternalIterator<O> for Map<T, O, I, F> {
    fn run<F1: FnMut(O) -> bool>(self, mut f: F1) {
        let mut closure = self.closure;
        self.iter.run(move |t: T| {
            f(closure(t))
        });
    }
}

