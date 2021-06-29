fn main() {
    fn five() -> i32 {
        5
    }
    println!("five() 的值为: {}", five());
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
