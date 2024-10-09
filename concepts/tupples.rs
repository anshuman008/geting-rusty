fn main() {
    // A tuple in Rust is a collection of values of different types, all stored in a single structure.
    // Tuples have a fixed length and can contain a mix of data types.

    // Defining a tuple with three different types: u8 (unsigned 8-bit integer), bool (boolean), and f32 (floating point number).
    let tuple: (u8, bool, f32) = (2, false, 9.88);

    // Defining another tuple with multiple types, including integers, floating points, a string, and a character.
    let tuple2 = (7, 99, 0.88, 0.0034, -84, "sfsfpsaji", 's');

    // Accessing and printing elements of the first tuple using dot notation (tuple.0, tuple.1, etc.).
    println!("First tuple elements: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    // Printing all elements of the second tuple using debug formatting.
    println!("{:?}", tuple2);

    // Destructuring a tuple: You can break a tuple into individual variables.
    let (a, b, c) = tuple;
    println!("Destructured tuple: {}, {}, {}", a, b, c);

    // Tuple as a return type example:
    let coordinates = get_coordinates();
    println!("Coordinates: x = {}, y = {}", coordinates.0, coordinates.1);
}

// Example of using a tuple as a return type from a function.
// This function returns two values, an x and y coordinate, packaged in a tuple.
fn get_coordinates() -> (i32, i32) {
    (10, 20) // Returning a tuple with two integers.
}
