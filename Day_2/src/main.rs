fn main() {
    // Tuple with three elements
    let my_tuple = (10, 20, 30);

    // Slice with four elements
    let my_slice = &[40, 50, 60, 70];

    // Your task:
    // 1. Extract the second element from the tuple and bind it to variable `a`.
    // 2. Use destructuring assignment to extract the last two elements from the slice and bind them to variables `b` and `c`.
    // 3. Uncomment and fill in the blank in the assert_eq! macro to make the code work.

    let a = my_tuple.1;
    let [_, b, c @ ..] = my_slice;

    // Fill in the blank to make the code work
    assert_eq!((a, *b, c[0]), (20, 50, 60));

    println!("Success!");
}

