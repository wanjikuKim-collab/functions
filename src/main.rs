fn add(a: i32, b: i32) -> i32{
    a + b
}

fn main() {
    println!("Hello, world!");
    let x = add(5, 6);
    let y = add(7,8);
    let z = add(x, y);

    println!("{},{},{}", x, y, z);
}
