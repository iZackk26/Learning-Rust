fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn print() -> () {
    println!("Hello, world!");
}

//fn never_return() -> ! {
//loop{}
//panic!("This call never returns!");
//}

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x // If you remove the semicolon, you will get a tuple
    };

    // First Solution
    let v = {
        let x = 1;
        x + 2
    };

    assert_eq!(v, 3);
    println!("Success!");

    let w = {
        let mut x = 1;
        x += 2;
        x
    };
    println!("Double Success!");

    let i = 3;
    let l = i;

    assert_eq!(i, 3);
    println!("Triple Success!");

    let j = sum(1, 3);
    assert_eq!(j, 4);
    println!("Sum {} Success!", j);

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    print();
}
