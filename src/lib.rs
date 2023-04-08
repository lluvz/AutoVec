/*!
# AutoVec
Vec that automatically remove the child when the child is being dropped.  
**This crate is still in development! It is not stable yet!** 
## Purpose of this crate
The purpose of this crate is create a container to gather variables to be read or processed collectively, while still allowing individual variables to be mutated freely.  
This means that the container need to store raw pointers rather than references. However, if the child variables are moved or dropped, the raw pointers will pointer to invalid datas. Thus, the dropped child need to be removed from the vector, and its place in the memory cannot be changed.  
Luckily, Rust provide the `Drop` trait which allows to define a `drop()` callback to be called when the variable is dropped, which can be used to inform the container to remove the dropped child.  
Rust also provide `pin` to prevent the child from begin moved thus changing its position in the memory.  
Also note that mutating the child itself, e.g. `child1 = child2` will cause child1 to be dropped thus removing it from the vec. This is needed for the reason that the above operation will change the pointer to its container.

# Example
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
 */
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::pin::Pin;
/// Vector that automatically remove the child that is being dropped.
/// # Example
/// ```
/// use auto_vec::*;
/// #[derive(Debug)]
/// struct Obj;
/// fn main() {
///     let mut v = AutoVec::new();
///     let mut c1 = AutoChild::new(Obj);
///     let mut c2 = AutoChild::new(Obj);
///     v.add(&mut c1);
///     v.add(&mut c2);
///     for i in 0..v.len() {
///         // Collective processing
///         println!("{:?}", v[i]);
///         // Do not use v[i] = ..., for it will change v[i]'s pointer to its container.
///     }
/// }
/// ```
#[derive(Debug)]
pub struct AutoVec<T> {
    pub children: Vec<*const AutoChild<T>>,
}
#[derive(Debug)]
pub struct AutoChild<T> {
    parent: *const AutoVec<T>,
    index: usize,
    pub child: T,
}
impl<T> Deref for AutoChild<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.child
    }
}
impl<T> DerefMut for AutoChild<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
impl<T> AutoChild<T> {
    pub fn new(child: T) -> Pin<Box<Self>> {
        Box::pin(
            Self {
                parent: 0 as _, index: 0,
                child,
            }
        )
    }
}
impl<T> Drop for AutoChild<T> {
    fn drop(&mut self) {
        if self.parent!= 0 as _ {
            unsafe {(self.parent as *mut AutoVec<T>).as_mut()}.unwrap().called_remove(self);
        }
    }
}
impl<T> Drop for AutoVec<T> {
    fn drop(&mut self) {
        for i in &self.children {
            unsafe {(*i as *mut AutoChild<T>).as_mut().unwrap()}.parent = 0 as _;
        }
    }
}
impl<T> AutoVec<T> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn add(&mut self, child: &mut AutoChild<T>) {
        child.parent = self as *const _;
        child.index = self.children.len();
        self.children.push(child as *const _);
    }
    fn called_remove(&mut self, child: &AutoChild<T>) {
        self.children.swap_remove(child.index);
    }
    pub fn remove(&mut self, child: &mut AutoChild<T>) {
        self.children.swap_remove(child.index);
        child.parent = 0 as _;
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.children.len()
    }
}
impl<T: 'static> Index<usize> for AutoVec<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {self.children[index].as_ref().unwrap()}
    }
}
impl<T: 'static> std::ops::IndexMut<usize> for AutoVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {(self.children[index] as *mut AutoChild<T>).as_mut().unwrap()}
    }
}