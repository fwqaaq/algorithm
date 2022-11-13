use std::rc::Rc;
fn main() {
    let a = Rc::new(String::from("hello, world"));
    let mut b = a.clone();

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
}
