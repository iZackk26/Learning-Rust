fn add2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 6;
    let z: i32 = add2(x, y);
    println!("{} + {} = {}", x, y, z);
}
