fn main() {
    let b = "str".as_bytes().to_vec();
    let s = b.as_slice();

    println!("{}", String::from_utf8(s.to_vec()).unwrap());
}
