//! Differences between iterators
//!
//! The [`IterDiff`] trait can be used to iterate through the differences
//! between two iterators. The differences between each element are enumerated
//! by [`Diff`]. The variants of the enum express the changes one would need to
//! make to the original iterator in order to attain the second.
//!
//! ```
//! use iter_diff::prelude::*;
//!
//! let a = [0, 1, 2, 3];
//! let b = [0, 2, 2];
//!
//! let diffs: Vec<_> = a.iter_diff(b).collect();
//! assert_eq!(diffs.len(), 4);
//!
//! assert_eq!(diffs[0], Diff::Keep);
//! assert_eq!(diffs[1], Diff::Change(2));
//! assert_eq!(diffs[2], Diff::Keep);
//! assert_eq!(diffs[3], Diff::Remove);
//! ```

#![deny(missing_docs)]
#![deny(clippy::all)]

pub mod prelude;

/// The difference between two iterator elements.
#[derive(Debug, Hash, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Diff<T> {
    /// Changed item. The element of the left-hand side iterator differs from
    /// that of the right-hand side iterator. The new element is returned
    /// contained in this variant.
    Change(T),
    /// Removed item. The element of the left-hand side iterator is not present
    /// in the right-hand side iterator.
    Remove,
    /// Kept item. The element of the left-hand side iterator is the same as
    /// the element of the right-hand side iterator.
    Keep,
    /// Added item. The left-hand side iterator does not contain this element
    /// of the right-hand side iterator.
    Add(T),
}

/// An iterator of the differences between of two iterators.
#[derive(Debug, Hash, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct DiffIter<Lhs, Rhs> {
    lhs: Lhs,
    rhs: Rhs,
}

impl<T, U, Lhs, Rhs> Iterator for DiffIter<Lhs, Rhs>
where
    T: PartialEq<U>,
    Lhs: Iterator<Item = T>,
    Rhs: Iterator<Item = U>,
{
    type Item = Diff<U>;

    fn next(&mut self) -> Option<Self::Item> {
        let l = self.lhs.next();
        let r = self.rhs.next();

        match (l, r) {
            (None, None) => None,
            (None, Some(r)) => Some(Diff::Add(r)),
            (Some(_), None) => Some(Diff::Remove),
            (Some(l), Some(r)) => match l == r {
                true => Some(Diff::Keep),
                false => Some(Diff::Change(r)),
            },
        }
    }
}

/// Iterate through the differences between each element.
pub trait IterDiff<T>: IntoIterator<Item = T> + sealed::Sealed<T> {
    /// Return an iterator through the differences of each element.
    ///
    /// ```
    /// use iter_diff::prelude::*;
    ///
    /// let a = [0, 1, 2];
    /// let b = [0, 3, 2, 3];
    ///
    /// let diffs: Vec<_> = a.iter_diff(b).collect();
    /// assert_eq!(diffs.len(), 4);
    ///
    /// assert_eq!(diffs[0], Diff::Keep);
    /// assert_eq!(diffs[1], Diff::Change(3));
    /// assert_eq!(diffs[2], Diff::Keep);
    /// assert_eq!(diffs[3], Diff::Add(3));
    /// ```
    fn iter_diff<U, Rhs>(
        self,
        rhs: Rhs,
    ) -> DiffIter<Self::IntoIter, Rhs::IntoIter>
    where
        T: PartialEq<U>,
        Rhs: IntoIterator<Item = U>;
}

impl<T, Lhs> IterDiff<T> for Lhs
where
    Lhs: IntoIterator<Item = T>,
{
    fn iter_diff<U, Rhs>(
        self,
        rhs: Rhs,
    ) -> DiffIter<Lhs::IntoIter, Rhs::IntoIter>
    where
        T: PartialEq<U>,
        Rhs: IntoIterator<Item = U>,
    {
        let lhs = self.into_iter();
        let rhs = rhs.into_iter();
        DiffIter { lhs, rhs }
    }
}

mod sealed {
    pub trait Sealed<T> {}

    impl<T, I> Sealed<T> for I where I: IntoIterator<Item = T> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove() {
        let a = [0, 1, 2, 4];
        let b = [0, 2];

        let diffs: Vec<_> = a.iter_diff(b).collect();
        assert_eq!(diffs.len(), 4);

        assert_eq!(diffs[0], Diff::Keep);
        assert_eq!(diffs[1], Diff::Change(2));
        assert_eq!(diffs[2], Diff::Remove);
        assert_eq!(diffs[3], Diff::Remove);
    }

    #[test]
    fn add() {
        let a = [0, 2];
        let b = [0, 1, 2, 4];

        let diffs: Vec<_> = a.iter_diff(b).collect();
        assert_eq!(diffs.len(), 4);

        assert_eq!(diffs[0], Diff::Keep);
        assert_eq!(diffs[1], Diff::Change(1));
        assert_eq!(diffs[2], Diff::Add(2));
        assert_eq!(diffs[3], Diff::Add(4));
    }

    #[test]
    fn multi_change() {
        let a = [0, 1, 2, 3];
        let b = [0, 3, 1, 3];

        let diffs: Vec<_> = a.iter_diff(b).collect();
        assert_eq!(diffs.len(), 4);

        assert_eq!(diffs[0], Diff::Keep);
        assert_eq!(diffs[1], Diff::Change(3));
        assert_eq!(diffs[2], Diff::Change(1));
        assert_eq!(diffs[3], Diff::Keep);
    }

    struct TestInt(i32);
    impl PartialEq<i32> for TestInt {
        fn eq(&self, other: &i32) -> bool {
            self.0 == *other
        }
    }

    #[test]
    fn add_mixed() {
        let a = [TestInt(0), TestInt(2)];
        let b = [0, 1, 2, 4];

        let diffs: Vec<_> = a.iter_diff(b).collect();
        assert_eq!(diffs.len(), 4);

        assert_eq!(diffs[0], Diff::Keep);
        assert_eq!(diffs[1], Diff::Change(1));
        assert_eq!(diffs[2], Diff::Add(2));
        assert_eq!(diffs[3], Diff::Add(4));
    }
}
