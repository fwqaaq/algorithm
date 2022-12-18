// pub use std::array::binary_search;
use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        if input[i] < 0 {
            input.to_mut()[i] = -input[i];
        }
    }
}
fn main() {
    // let x = &[1, 2, 4];
    // let x_ptr = x.as_ptr();

    // unsafe {
    //     // for i in 0..x.len() {
    //     //     assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    //     // }
    //     println!("{:?}", x_ptr);
    //     println!("{:?}", x_ptr.add(1));
    // }
    let s1 = [1, -2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("{:?}", s1);
    println!("{:?}", i1);
}
