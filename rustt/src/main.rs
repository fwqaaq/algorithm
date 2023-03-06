// 声明一个外部函数 abs，它接受一个 i32 类型的参数，返回一个 i32 类型的值
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // 调用 abs 函数，传入 -3 作为参数
    let x = unsafe { abs(-3) };
    // 打印返回值，应该是 3
    println!("The absolute value of -3 is {}", x);
}
