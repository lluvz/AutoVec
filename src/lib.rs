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
                parent: 0 as _,
                child,
            }
        )
    }
    fn set_parent(&mut self, parent: *const AutoVec<T>) {
        self.parent = parent;
    }
}
impl<T> Drop for AutoChild<T> {
    fn drop(&mut self) {
        if self.parent!= 0 as _ {
            unsafe {(self.parent as *mut AutoVec<T>).as_mut()}.unwrap().called_remove(self as *const AutoChild<T>);
        }
    }
}
impl<T> Drop for AutoVec<T> {
    fn drop(&mut self) {
        for i in &self.children {
            unsafe {(*i as *mut AutoChild<T>).as_mut().unwrap()}.set_parent(0 as _);
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
        child.set_parent(self as *const _);
        self.children.push(child as *const _);
    }
    fn called_remove(&mut self, child: *const AutoChild<T>) {
        for i in 0..self.children.len() {
            if self.children[i] == child as *const AutoChild<T> {
                self.children.swap_remove(i);
                break
            }
        }
    }
    pub fn remove(&mut self, child: &mut AutoChild<T>) {
        for i in 0..self.children.len() {
            if self.children[i] == child as *const _ {
                self.children.swap_remove(i);
                child.set_parent(0 as _);
                break
            }
        }
    }
}