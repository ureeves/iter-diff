
# iter-diff

#### Differences between iterators

[![CI][ci-badge]][ci-url]
[![codecov][codecov-badge]][codecov-url]
[![docs.rs][docs-badge]][docs-url]

[ci-badge]: https://img.shields.io/github/workflow/status/ureeves/iter-diff/main?logo=github
[ci-url]: https://github.com/ureeves/iter-diff/actions/workflows/main.yml
[codecov-badge]: https://img.shields.io/codecov/c/gh/ureeves/iter-diff?logo=codecov
[codecov-url]: https://codecov.io/gh/ureeves/iter-diff
[docs-badge]: https://img.shields.io/docsrs/iter-diff?color=blue&logo=rust&logoColor=orange
[docs-url]: https://docs.rs/iter-diff

---

The `IterDiff` trait can be used to iterate through the differences between
two iterators. The differences between each element are enumerated by `Diff`.
The variants of the enum express the changes one would need to make to the
left-hand iterator in order to attain the right-hand iterator.

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
