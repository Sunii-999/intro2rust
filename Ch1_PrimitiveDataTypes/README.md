# Primitive Data Types

Primitive data types are the **basic building blocks** of Rust programs.  
They represent **single, simple values** and are stored directly on the **stack**, making them very fast.

Rust has **four main primitive data types**:

- Integers
- Floating-point numbers
- Booleans
- Characters

---

## Project Setup

Create a new folder and file:

```bash
PrimitiveDataTypes/primitiveData.rs
```

Initial file structure:

```rust
// PrimitiveDataTypes/primitiveData.rs
fn main() {

}
```

---

## Integers (`int`)

Integers represent **whole numbers**.

Rust supports:

- **Signed integers** (can be positive or negative)
- **Unsigned integers** (only positive)

### Signed Integers

Signed integers can store **positive and negative** values.

| Type   | Size    |
| ------ | ------- |
| `i8`   | 8-bit   |
| `i16`  | 16-bit  |
| `i32`  | 32-bit  |
| `i64`  | 64-bit  |
| `i128` | 128-bit |

### Unsigned Integers

Unsigned integers store **only positive** values.

| Type   | Size    |
| ------ | ------- |
| `u8`   | 8-bit   |
| `u16`  | 16-bit  |
| `u32`  | 32-bit  |
| `u64`  | 64-bit  |
| `u128` | 128-bit |

---

### Integer Example

```rust
let x: i32 = -42; // signed integer (positive or negative)
let y: u64 = 100; // unsigned integer (only positive)

println!("Signed Integer: {}", x);
println!("Unsigned Integer: {}", y);
```

---

### Integer Size and Range

The size of the integer determines **how large the number can be**.

```rust
// Maximum values
let e: i32 = 2147483647;
let i: i64 = 9223372036854775807;

println!("Maximum value of i32: {}", e);
println!("Maximum value of i64: {}", i);
```

Explanation:

- `i32` → uses 32 bits
- `i64` → uses 64 bits
- Larger sizes allow larger numbers but use more memory

---

## Floating-Point Numbers (`float`)

Floating-point numbers represent **decimal values**.

Rust provides two types:

| Type  | Size   | Precision                  |
| ----- | ------ | -------------------------- |
| `f32` | 32-bit | Single precision           |
| `f64` | 64-bit | Double precision (default) |

### Why `f64` is the Default

- Higher precision
- Better accuracy
- Similar performance on modern hardware

Use `f32` only when:

- Memory usage is critical
- You need SIMD optimizations (e.g. games, graphics)

---

### Float Example

```rust
let pi: f64 = 3.14;
println!("Value of pi: {}", pi);
```

> Rust does **not** automatically convert between `f32` and `f64`.
> You must explicitly cast using `as`.

---

## Booleans (`bool`)

Booleans represent **true or false** values.

```rust
let is_snowing: bool = true;
println!("Is it snowing? {}", is_snowing);
```

Booleans are commonly used in:

- Conditions (`if`, `while`)
- Logic checks
- Program flow control

---

## Characters (`char`)

A `char` represents a **single Unicode character**.

```rust
let letter: char = 'a';
println!("First letter of alpha: {}", letter);
```

Important notes:

- `char` uses **single quotes**
- Supports Unicode (emojis, accents, symbols)
- Takes **4 bytes**, not 1 byte

Example:

```rust
let emoji: char = '🔥';
```

---

## Full Example

```rust
fn main() {
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of alpha: {}", letter);
}
```

---

## Summary

| Type      | Example      | Description              |
| --------- | ------------ | ------------------------ |
| Integer   | `i32`, `u64` | Whole numbers            |
| Float     | `f32`, `f64` | Decimal numbers          |
| Boolean   | `bool`       | `true` or `false`        |
| Character | `char`       | Single Unicode character |

---

## Key Takeaways

- Primitive types store **one value**
- They are **fast and efficient**
- Rust is **strict about types**
- Explicit typing improves safety and clarity

These fundamentals are essential before moving on to **compound data types**, **ownership**, and **borrowing**.
