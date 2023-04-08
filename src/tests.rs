use super::*;
struct MyData {
    n: i32,
}
#[test]
fn main() {
    let mut t1 = AutoChild::new(MyData { n: 1 });
    let mut t2 = AutoChild::new(MyData { n: 0 });
    let mut v = AutoVec::new();
    v.add(&mut t1);
    v.add(&mut t2);
    println!("{}", t1.n);
}