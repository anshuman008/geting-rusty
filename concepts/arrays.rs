fn main() {
    // Arrays in Rust: These allocate memory on the stack because their size is fixed at compile time.

    // A u8 array with 5 elements, where each element is an unsigned 8-bit integer.
    let arr: [u8; 5] = [1, 99, 3, 4, 5];

    // A u16 array with 8 elements, where each element is an unsigned 16-bit integer.
    // Initialized with all elements set to zero (0).
    let other_arr: [u16; 8] = [0; 8];

    // Iterating through the u16 array and printing each element.
    for i in 0..other_arr.len() {
        println!("{}", other_arr[i]);
    }

    // Printing the length of the u8 array.
    println!("This is the length of the array: {}", arr.len());

    // A simple message indicating that the guess is incorrect.
    println!("Your guess is not correct, sorry!");
}
