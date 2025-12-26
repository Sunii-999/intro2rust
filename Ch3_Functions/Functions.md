# Functions, Expressions, and Statements in Rust

In this chapter, we learn how **functions work in Rust** and understand one of the most important concepts in the language:

> **The difference between expressions and statements**

This concept is critical because Rust relies heavily on expressions to return values safely and predictably.

---

## Entry Point: `main`

Every Rust program starts execution in the `main` function.

```rust
fn main() {
    hello_world();
    tell_height(232);
    human_id("Sunii", 55, 182.67);

    // Expression block
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // expression (no semicolon)
    };
    println!("Result is: {}", x);

    let y: i32 = add(4, 6);
    println!("The answer to fn 'add' = {}", y);

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your bmi is {:.2}", bmi);
}
```

---

## Calling Functions

Rust allows you to call functions **anywhere in the file**, even before they are defined.

> This is **not hoisting** like in JavaScript — Rust simply parses the entire file before execution.

---

## Simple Function (No Parameters)

```rust
fn hello_world() {
    println!("Hello, world");
}
```

- Takes no input
- Returns nothing (`()`)
- Executes a side effect (printing)

---

## Function with Input Parameters

```rust
fn tell_height(height: i32) {
    println!("My height is {}", height);
}
```

- Accepts one parameter
- Parameter type **must be declared**
- Returns nothing

---

## Function with Multiple Parameters

```rust
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm",
        name, age, height
    );
}
```

Explanation:

- `&str` → borrowed string slice
- `u32` → unsigned integer
- `f32` → floating-point number
- Functions can accept **multiple values of different types**

---

## Expressions vs Statements

Understanding this difference is **essential** in Rust.

---

### Expressions

An **expression**:

- Returns a value
- Can be assigned to a variable

Examples:

```rust
5
true
add(3, 4)
```

---

### Expression Block Example

```rust
let x: i32 = {
    let price: i32 = 5;
    let qty: i32 = 10;
    price * qty
};
```

Explanation:

- The block `{}` is an **expression**
- The last line has **no semicolon**
- The value of `price * qty` is returned and stored in `x`

---

## Functions That Return Values (Expressions)

### Example: `add`

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Explanation:

- `-> i32` specifies the return type
- The last line is an **expression**
- No `return` keyword needed
- No semicolon → value is returned

Usage:

```rust
let y: i32 = add(4, 6);
```

---

### Example: BMI Calculation

```rust
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```

Usage:

```rust
let bmi: f64 = calculate_bmi(weight, height);
```

---

## Statements

A **statement**:

- Does **not** return a value
- Performs an action
- Ends with a semicolon

Examples:

```rust
let x = 5;
println!("Hello");
```

---

## Statement Function Example

This function performs an action but returns **nothing**.

```rust
fn print_greeting(name: &str) {
    println!("Hello, {}! Welcome to Rust.", name);
}
```

Usage:

```rust
print_greeting("Sunii");
```

Explanation:

- No return type
- Always returns `()` (unit type)
- Used for side effects (printing, logging, etc.)

---

## Expression vs Statement (Side-by-Side)

```rust
// Expression
let sum: i32 = add(2, 3);

// Statement
print_greeting("Sunii");
```

| Feature         | Expression | Statement |
| --------------- | ---------- | --------- |
| Returns value   | ✅         | ❌        |
| Can be assigned | ✅         | ❌        |
| Ends with `;`   | ❌         | ✅        |

---

## Key Takeaways

- Rust uses **expressions** to return values
- A trailing semicolon turns an expression into a statement
- Functions return the **last expression**
- Statement functions are used for **side effects**
- This design helps Rust prevent bugs at compile time
