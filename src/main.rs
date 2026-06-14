// Day 02 : Compound Data Types;

// {Array✅, Tuplse✅, Slices✅, Strings✅ & (Slice String)✅}

fn main() {
    
    //Array
    // Array is a collection of items of the same type stored in contiguous memory locations.

    let numbers:[i32; 5] = [1, 2, 3, 4, 5];
    
    let fruits: [&str; 4] = ["Apple", "Orange", "Mango", "Papaya"];

    println!("Numbers: {:?}", numbers);

    println!("Mixed: {:?}", fruits); // all fruits

    println!("First Fruit: {}", fruits[0]); // first fruit
    println!("Second fruit: {}", fruits[1]);
    println!("Third fruit: {}", fruits[2]);
    println!("Fourth fruit : {}", fruits[3]);


    //Tuples
    // A tuple is a collection of values of different types. It is denoted by parentheses () and can contain any number of values.

     let human: (String, i32, bool) = ("Kratos".to_string(), 35, true); // human is a tuple that contains a String, an i32, and a bool

    // // kraatos is a slice of string, but we need to convert it to a String type to store it in the tuple. We can do this using the to_string() method.

     println!("Human tuple: {:?}", human);

     let mix_tuple = ("Alice", 30, true, [1, 2, 3, 4]);

     println!("Mixed tuple: {:?}", mix_tuple); // mixed tuple contains a string, an integer, a boolean, and an array.


    // Slices
    // A slice is a reference to a contiguous sequence of elements in an array. It is denoted by square brackets [] and can be used to access a portion of an array.

    let slice_numbers: &[i32] = &[1, 2, 3, 4];

    println!("Slice of numbers: {:?}", slice_numbers);

    let animal_slice: &[&str] = &["Dog", "Lion", "Tiger"];

    println!("Slice animals: {:?}", animal_slice);

    let slice_books: &[&String] = &[
        &"The Great Gatsby".to_string(), 
        &"To Kill a Mockingbird".to_string(), 
        &"1984".to_string()
        ];

    println!("Slice books: {:?}", slice_books);

    // String Slices vs String

    //String (mutable, growable, owned string)

}