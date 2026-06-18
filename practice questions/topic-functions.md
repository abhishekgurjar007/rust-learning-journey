# Rust Practice Set – Functions

Practice these questions without looking at the solution immediately. Write the code yourself, run it, and compare your output with the expected result.

## Topics Covered

- Defining functions with `fn`
- Calling functions
- Function parameters
- Arguments
- Return values
- Expressions vs statements
- Functions with multiple parameters
- Functions with explicit return types
- Block expressions
- Variable scope inside functions

---

## Question 1 – Basic Function

Create a function named `greet` that prints:

```text
Welcome to Rust!
```

Call the function from `main()`.

### Expected Output

```text
Welcome to Rust!
```

---

## Question 2 – Function with One Parameter

Create a function named `print_name` that accepts a string slice parameter.

Pass `"Abhishek"` to the function and print:

### Expected Output

```text
Hello, Abhishek!
```

---

## Question 3 – Function with Multiple Parameters

Create a function named `introduce` that accepts:

- `name: &str`
- `age: u8`

Print both values.

### Expected Output

```text
Name: Abhishek
Age: 20
```

---

## Question 4 – Function Returning a Value

Create a function named `square` that accepts an `i32` and returns its square.

Call the function with `6`.

### Expected Output

```text
Square: 36
```

---

## Question 5 – Addition Function

Create a function named `add` that accepts two integers and returns their sum.

Pass `15` and `25`.

### Expected Output

```text
Sum: 40
```

---

## Question 6 – Multiplication Function

Create a function named `multiply` that accepts two `i32` values and returns their product.

Pass `7` and `8`.

### Expected Output

```text
Result: 56
```

---

## Question 7 – Even or Odd

Create a function named `is_even` that accepts an integer and returns a `bool`.

Check whether `24` is even.

### Expected Output

```text
Is even: true
```

---

## Question 8 – Maximum of Two Numbers

Create a function named `max_number` that accepts two integers and returns the larger value.

Pass `45` and `78`.

### Expected Output

```text
Maximum: 78
```

---

## Question 9 – Celsius to Fahrenheit

Create a function named `celsius_to_fahrenheit`.

Formula:

```text
F = (C × 9 / 5) + 32
```

Pass `25.0` as the Celsius value.

### Expected Output

```text
Fahrenheit: 77
```

---

## Question 10 – Expression vs Statement

Predict the output before running the code.

Complete the missing part of the function so it returns `15` using an expression instead of the `return` keyword.

```rust
fn get_value() -> i32 {
    let x = 10;

    // Write your expression here
}
```

### Expected Output

```text
Value: 15
```

---

## Question 11 – Block Expressions

Complete the following code using a block expression:

```rust
let result = {
    let x = 5;
    let y = 10;

    // Return x + y as an expression
};
```

Print `result`.

### Expected Output

```text
Result: 15
```

---

## Question 12 – Function Scope

Create a variable named `message` inside a function named `show_message`.

Print the variable inside the function.

Try printing the same variable from `main()` and observe the error.

Comment out the line causing the error.

### Expected Output

```text
Rust is awesome!
```

---

## Question 13 – Return Type Annotation

Create a function named `cube` that accepts an `i32` and returns its cube.

Pass `4` to the function.

### Expected Output

```text
Cube: 64
```

---

## Question 14 – String Length Function

Create a function named `string_length` that accepts a string slice (`&str`) and returns its length.

Pass `"Rust"`.

### Expected Output

```text
Length: 4
```

---

## Question 15 – Mixed Challenge

Create the following functions:

- `full_name(first: &str, last: &str) -> String`
- `birth_year(age: u32) -> u32`

Assume the current year is `2026`.

Call both functions using:

- First name: `"Abhishek"`
- Last name: `"Gurjar"`
- Age: `20`

Print the results.

### Expected Output

```text
Full Name: Abhishek Gurjar
Birth Year: 2006
```

---

## Bonus Challenges

### Bonus 1 – Factorial

Create a function named `factorial` that returns the factorial of a number.

Calculate the factorial of `5`.

### Expected Output

```text
Factorial: 120
```

---

### Bonus 2 – Simple Calculator

Create four functions:

- `add`
- `subtract`
- `multiply`
- `divide`

Use the numbers `20` and `5`.

### Expected Output

```text
Add: 25
Subtract: 15
Multiply: 100
Divide: 4
```

---

## Rules

- Write the function signature before writing the logic.
- Use explicit parameter and return types.
- Prefer expressions over the `return` keyword when possible.
- Predict the output before running the program.
- Practice writing functions without looking at previous examples.
- Commit your solutions to GitHub after completing all questions.

Happy coding! 🦀