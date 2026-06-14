# Day 02 - Compound Data Types in Rust

## Topics Covered

- Arrays
- Tuples
- Slices
- Strings
- String Slices (`&str`)

---

# Arrays

An array is a collection of elements of the same data type stored in contiguous memory locations.

## Example

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

### Breakdown

- `i32` = type of each element
- `5` = length of array
- Array size is fixed at compile time

---

## Array of Strings

```rust
let fruits: [&str; 4] = ["Apple", "Orange", "Mango", "Papaya"];
```

Here:

- `&str` represents string slices
- All elements must be of the same type

### Accessing Elements

```rust
println!("{}", fruits[0]);
println!("{}", fruits[1]);
println!("{}", fruits[2]);
println!("{}", fruits[3]);
```

Array indexing starts from `0`.

---

## Printing Entire Arrays

```rust
println!("{:?}", numbers);
```

Output:

```text
[1, 2, 3, 4, 5]
```

### Why `{:?}`?

Arrays cannot be printed using `{}` because they do not implement the `Display` trait.

Instead, Rust provides Debug formatting:

```rust
println!("{:?}", numbers);
```

---

# Tuples

A tuple is a collection of values that can have different data types.

## Example

```rust
let human: (String, i32, bool) =
    ("Kratos".to_string(), 35, true);
```

This tuple contains:

1. String
2. Integer
3. Boolean

---

## Why `.to_string()`?

```rust
"Kratos"
```

is a string slice (`&str`).

The tuple expects a `String`, so we convert it:

```rust
"Kratos".to_string()
```

This creates an owned String value.

---

## Printing Tuples

```rust
println!("{:?}", human);
```

Output:

```text
("Kratos", 35, true)
```

---

## Mixed Tuple

```rust
let mix_tuple = ("Alice", 30, true, [1, 2, 3, 4]);
```

A tuple can contain:

- Strings
- Numbers
- Booleans
- Arrays
- Other complex types

Output:

```text
("Alice", 30, true, [1, 2, 3, 4])
```

---

# Slices

A slice is a reference to a collection of elements.

Slices do not own the data.

They only borrow it.

---

## Slice of Numbers

```rust
let slice_numbers: &[i32] = &[1, 2, 3, 4];
```

Explanation:

- `&` = borrow
- `[i32]` = slice of integers

Output:

```text
[1, 2, 3, 4]
```

---

## Slice of String Slices

```rust
let animal_slice: &[&str] =
    &["Dog", "Lion", "Tiger"];
```

Explanation:

- Outer `&` → borrow the array
- Inner `&str` → each element is a string slice

Output:

```text
["Dog", "Lion", "Tiger"]
```

---

## Slice of References to String

```rust
let slice_books: &[&String] = &[
    &"The Great Gatsby".to_string(),
    &"To Kill a Mockingbird".to_string(),
    &"1984".to_string()
];
```

### Understanding the Multiple `&`

Each book title is:

```rust
"The Great Gatsby".to_string()
```

which creates a `String`.

Then:

```rust
&String
```

creates a reference to that String.

Finally:

```rust
&[ ... ]
```

creates a slice containing references to Strings.

Type breakdown:

```rust
&[&String]
```

means:

- Outer `&` → slice reference
- Inner `&String` → references to String values

Output:

```text
[
  "The Great Gatsby",
  "To Kill a Mockingbird",
  "1984"
]
```

---

# String vs String Slice

## String

Owned string stored on the heap.

```rust
let name = String::from("Kratos");
```

Characteristics:

- Growable
- Mutable (if declared mut)
- Owns its data

---

## String Slice (`&str`)

Reference to string data.

```rust
let name = "Kratos";
```

Characteristics:

- Fixed size
- Borrowed
- Faster and lightweight

---

# Key Learnings

- Arrays store values of the same type.
- Array size is fixed.
- Tuples can store different data types together.
- `{:?}` is used for Debug formatting.
- Slices borrow data instead of owning it.
- `String` owns data.
- `&str` borrows string data.
- `.to_string()` converts a string slice into an owned String.
- `&[&String]` means a slice containing references to String values.

---

# Day 02 Summary

Today I learned Rust's compound data types: Arrays, Tuples, Slices, Strings, and String Slices. I practiced creating collections, accessing elements, borrowing data through slices, understanding ownership with Strings, and using Debug formatting for printing complex data structures.

**Status:** Day 02 Completed ✅