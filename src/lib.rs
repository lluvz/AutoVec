use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;
#[derive(Debug)]
pub struct AutoVec<T> {
    pub children: Vec<*const AutoChild<T>>,
}
#[derive(Debug)]
pub struct AutoChild<T> {
    pub parent: *const AutoVec<T>,
    pub index: usize,
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
}