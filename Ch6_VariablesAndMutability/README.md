## Chapter 6 – Variables and Mutability

In most programming languages, variables can change freely.

In Rust, variables are **immutable by default**. This is a deliberate design choice to ensure **safety and concurrency**.

If you want to change a value, you must explicitly ask for it using `mut`.

---

## The Mutability Keyword

To allow a variable's value to change, we use the `mut` keyword.

```rust
fn main() {
    // 1. Declare a mutable variable
    let mut a: i32 = 5;
    println!("The value of a is {}", a);

    // 2. Modify the value
    a = 10;

    println!("The value of a is now {}", a);
}

```

### What’s happening here?

- `let`: Creates a new variable binding.
- `mut`: Tells the compiler "I intend to change this value later."
- `i32`: Explicitly sets the type to a **32-bit signed integer** (though Rust can usually infer this).
- `a = 10`: Updates the value stored in `a`.

---

## Why Immutable by Default?

Rust prefers immutability to prevent **bugs**.

If a section of code assumes a value won't change, but another part of the code changes it unexpectedly, bugs happen. By defaulting to immutable, Rust guarantees that values generally stay put unless you explicitly mark them otherwise.

---

## Common Mistake (Missing `mut`)

If you try to assign a new value to a variable without `mut`, Rust will stop you.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6; ❌ ERROR: cannot assign twice to immutable variable `x`
}

```

### The Fix

Simply add `mut` before the variable name:

```rust
let mut x = 5;
x = 6; // ✅ Works

```

---

## Type Annotations (`: i32`)

In the example: `let mut a : i32 = 5;`

Rust is a **statically typed** language, meaning it must know the types of all variables at compile time. However, the compiler is smart enough to infer types most of the time.

- **Explicit:** `let x: i32 = 5;` (You tell Rust the type)
- **Implicit:** `let x = 5;` (Rust guesses it's an `i32`)

Both are valid, but explicit types are useful for documentation or when the compiler is unsure.

---

## Variable Declaration Summary

| Keyword       | Syntax             | Can Change Value?            |
| ------------- | ------------------ | ---------------------------- |
| **Immutable** | `let x = 5;`       | ❌ No                        |
| **Mutable**   | `let mut x = 5;`   | ✅ Yes                       |
| **Constant**  | `const MAX = 100;` | ❌ Never (must be annotated) |

---

## Key Takeaways

- Variables are **immutable** by default.
- You must use `mut` to make them mutable.
- Rust prevents accidental state changes at **compile time**.
- Type annotations (like `: i32`) are optional but helpful for clarity.
