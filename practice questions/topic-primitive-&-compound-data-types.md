# Rust Practice Set – Primitive & Compound Data Types

Practice these questions without looking at the solution immediately. Write the code yourself, run it, and compare your output with the expected result.

## Topics Covered

- Variables and mutability
- Integer and floating-point types
- Boolean and character types
- Type annotations
- Arrays and array indexing
- Tuples and tuple destructuring
- Slices
- String slices (`&str`)
- `String` (heap-allocated strings)
- Basic `String` methods

---

## Question 1 – Variables and Mutability

Create a mutable variable `score` with value `10`.

Add `15` to it and print the result.

### Expected Output

```text
score = 25
```

---

## Question 2 – Integer Types

Create variables using explicit types:

- `age: u8 = 20`
- `temperature: i16 = -12`

Print both values.

### Expected Output

```text
Age: 20
Temperature: -12
```

---

## Question 3 – Floating Point Numbers

Create two variables:

- `price = 99.99` (`f64`)
- `discount = 10.5` (`f32`)

Print both values.

### Expected Output

```text
Price: 99.99
Discount: 10.5
```

---

## Question 4 – Boolean and Character

Create:

- `is_rust_fun = true`
- `grade = 'A'`

Print both values.

### Expected Output

```text
Rust is fun: true
Grade: A
```

---

## Question 5 – Array Basics

Create an array named `numbers` containing five integers:

```text
10, 20, 30, 40, 50
```

Print the complete array using debug formatting.

### Expected Output

```text
[10, 20, 30, 40, 50]
```

---

## Question 6 – Array Indexing

Using the array from Question 5, print the first and last elements.

### Expected Output

```text
First: 10
Last: 50
```

---

## Question 7 – Array Length

Create an array of three cities:

- Mumbai
- Delhi
- Pune

Print the length of the array.

### Expected Output

```text
Total cities: 3
```

---

## Question 8 – Tuples

Create a tuple named `student` containing:

```rust
("Abhishek", 20, true)
```

Print the entire tuple using debug formatting.

### Expected Output

```text
("Abhishek", 20, true)
```

---

## Question 9 – Tuple Destructuring

Using the tuple from Question 8, destructure it into separate variables and print each value.

### Expected Output

```text
Name: Abhishek
Age: 20
Enrolled: true
```

---

## Question 10 – String Slice (`&str`)

Create a string slice variable:

```rust
let language = "Rust";
```

Print the value.

### Expected Output

```text
Language: Rust
```

---

## Question 11 – String

Create a `String` using:

```rust
String::from("Hello")
```

Append `" Rust"` using the appropriate `String` method and print it.

### Expected Output

```text
Hello Rust
```

---

## Question 12 – String Length

Create:

```rust
let name = String::from("Abhishek");
```

Print the number of bytes in the `String`.

### Expected Output

```text
Length: 8
```

---

## Question 13 – Slices

Create an array:

```rust
[5, 10, 15, 20, 25]
```

Create a slice containing `10`, `15`, and `20`.

Print the slice using debug formatting.

### Expected Output

```text
[10, 15, 20]
```

---

## Question 14 – String Slice from String

Create:

```rust
let text = String::from("Programming");
```

Create a slice containing only `"Program"` and print it.

### Expected Output

```text
Program
```

---

## Question 15 – Mixed Challenge

Create:

- A tuple: `("The Hobbit", 1937)`
- A mutable `String` containing the author's name: `"J.R.R. Tolkien"`
- Append `" (Author)"` to the `String`

Print the book title, year, and updated author name.

### Expected Output

```text
Book: The Hobbit
Year: 1937
Author: J.R.R. Tolkien (Author)
```

---

## Rules

- Do not search for solutions immediately.
- Run every program before checking the expected output.
- Try adding explicit type annotations even when Rust can infer them.
- Experiment with different variable names and values.
- Commit your solutions to GitHub after completing all questions.

Happy coding! 🦀