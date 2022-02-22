# iter-diff

Differences between iterators

The `IterDiff` trait can be used to iterate through the differences between
two iterators. The differences between each element are enumerated by `Diff`.
The variants of the enum express the changes one would need to make to the
original iterator in order to attain the second.

```rust
use iter_diff::prelude::*;

let a = [0, 1, 2, 3];
let b = [0, 2, 2];

let diffs: Vec<_> = a.iter_diff(b).collect();
assert_eq!(diffs.len(), 4);

assert_eq!(diffs[0], Diff::Keep);
assert_eq!(diffs[1], Diff::Change(2));
assert_eq!(diffs[2], Diff::Keep);
assert_eq!(diffs[3], Diff::Remove);
```
