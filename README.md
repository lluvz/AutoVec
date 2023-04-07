# AutoVec
//!
Vec that automatically remove the child that is droped
**This crate is still in development.**
Many method of the default vec is still to be implemented for it.

Note that AutoVec stores the raw pointer of the child, meaning dereferencing is unsafe, which also means that you can freely mutate the child's field.
However, mutating the child it self, e.g. `child1 = child2` will cause child1 to be dropped thus removing it from the vec.
Also, as using `mem::swap` does not drop the child, but changes the child's parent, which is not memory safe, the child is pinned to prevent from being swapped.

# Examples

```
use auto_vec::*;
let mut t1 = AutoChild::new(0);
let mut t2 = AutoChild::new(1);
let mut v = AutoVec::new();
v.add(&mut t1);
v.add(&mut t2);
println!("{:?}", v);
drop(t1);
println!("{:?}", v);
// t1 has been automatically removed from the vector
```