# Day 03: Functions in Rust

## Introduction

Functions are reusable blocks of code that perform specific tasks. They help break large programs into smaller, manageable pieces, improve code readability, and eliminate code duplication.

Rust uses the `fn` keyword to define functions and follows the **snake_case** naming convention for function names.

---

## Basic Function Syntax

```rust
fn function_name() {
    // function body
}
```

### Example

```rust
fn greet() {
    println!("Hello, Rust!");
}

fn main() {
    greet();
}
```

### Output

```text
Hello, Rust!
```

---

## Calling Functions

A function executes only when it is called.

```rust
fn say_hello() {
    println!("Hello!");
}

fn main() {
    say_hello();
    say_hello();
}
```

### Output

```text
Hello!
Hello!
```

---

## Function Parameters

Parameters allow functions to receive input values.

### Syntax

```rust
fn function_name(parameter: data_type) {
    // function body
}
```

### Example

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Abhishek");
}
```

### Output

```text
Hello, Abhishek!
```

> Rust requires explicit type annotations for all function parameters.

---

## Multiple Parameters

Functions can accept multiple parameters separated by commas.

```rust
fn add(a: i32, b: i32) {
    println!("Sum: {}", a + b);
}

fn main() {
    add(10, 20);
}
```

### Output

```text
Sum: 30
```

---

## Parameters vs Arguments

- **Parameters** are variables defined in the function declaration.
- **Arguments** are the actual values passed to the function during a function call.

```rust
fn multiply(a: i32, b: i32) {
    println!("{}", a * b);
}

fn main() {
    multiply(5, 4);
}
```

In this example:

- `a` and `b` are parameters.
- `5` and `4` are arguments.

---

## Functions with Return Values

Functions can return values using the `->` syntax.

### Syntax

```rust
fn function_name() -> return_type {
    expression
}
```

### Example

```rust
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let result = square(6);

    println!("Square: {}", result);
}
```

### Output

```text
Square: 36
```

> Rust implicitly returns the last expression of a function.

---

## Using the `return` Keyword

You can explicitly return a value using the `return` keyword.

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("{}", add(15, 25));
}
```

### Output

```text
40
```

---

## Expressions vs Statements

Understanding the difference between expressions and statements is essential in Rust.

### Statements

Statements perform actions but do not return values.

```rust
let x = 5;
```

### Expressions

Expressions evaluate to a value.

```rust
let y = {
    let x = 3;
    x + 1
};

println!("{}", y);
```

### Output

```text
4
```

> Notice that `x + 1` does not end with a semicolon because it is an expression.

---

## Semicolons and Return Values

Adding a semicolon converts an expression into a statement.

### Correct

```rust
fn double(num: i32) -> i32 {
    num * 2
}
```

### Incorrect

```rust
fn double(num: i32) -> i32 {
    num * 2;
}
```

The second example causes a compilation error because the semicolon prevents the value from being returned.

---

## Function Scope

Variables declared inside a function are accessible only within that function.

```rust
fn demo() {
    let message = "Rust";
    println!("{}", message);
}

fn main() {
    demo();

    // println!("{}", message); // Error
}
```

---

## Complete Example

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    greet("Abhishek");

    let result = add(10, 15);

    println!("Result: {}", result);
}
```

### Output

```text
Hello, Abhishek!
Result: 25
```

---

## Key Takeaways

- Functions are defined using the `fn` keyword.
- Rust follows the `snake_case` naming convention for function names.
- Parameter types must always be specified.
- Functions can accept multiple parameters.
- Return types are declared using `->`.
- The last expression without a semicolon is automatically returned.
- The `return` keyword can be used for explicit returns.
- Semicolons convert expressions into statements.
- Variables inside a function are limited to that function's scope.
- Functions improve code reusability, readability, and maintainability.

## Expressions vs Statements

Understanding the difference between expressions and statements is essential in Rust because Rust is an expression-based language.

### Statements

Statements perform an action but do not return a value.

Examples:

```rust
let x = 5;

let y = {
    let z = 10;
};
```

In the example above:

- `let x = 5;` is a statement.
- `let z = 10;` is also a statement.

Statements usually end with a semicolon (`;`).

---

### Expressions

Expressions evaluate to a value.

Examples:

```rust
5 + 3
```

```rust
{
    let x = 3;
    x + 1
}
```

Expressions can be assigned to variables.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {}", y);
}
```

### Output

```text
y = 4
```

In this example:

- `x + 1` is an expression.
- The block itself is also an expression.
- The value `4` is assigned to `y`.

---

### Semicolons Matter

Adding a semicolon converts an expression into a statement.

Correct:

```rust
fn square(num: i32) -> i32 {
    num * num
}
```

Incorrect:

```rust
fn square(num: i32) -> i32 {
    num * num;
}
```

The second example causes a compilation error because `num * num;` becomes a statement and does not return a value.

---

### Key Difference

| Statements | Expressions |
|------------|-------------|
| Perform actions | Produce values |
| Do not return values | Always evaluate to a value |
| Usually end with `;` | Usually do not end with `;` |
| Cannot be assigned directly | Can be assigned to variables |

> In Rust, `if`, `match`, loops, and code blocks can all act as expressions.