## Chapter 4 – Ownership

Ownership is one of Rust’s **most important concepts**.  
It allows Rust to manage memory **without a garbage collector**, while still being safe and fast.

Rust enforces ownership through **three core rules**.

---

## Ownership Rule 1

### Each value in Rust has a variable that owns it

Every value in Rust is owned by **exactly one variable**.

```rust
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);

    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### What’s happening here?

- `s1` **owns** the `String` `"RUST"`
- We pass `&s1` to the function (a **reference**, not ownership)
- Ownership **does NOT move**
- `s1` is still valid after the function call

### Why use `&String`?

- Prevents ownership transfer
- Avoids unnecessary memory copying
- This is called **borrowing**

---

## Borrowing (Important Concept)

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- `&String` means: _borrow the value_
- The function can **read** the value
- The function **does not own** the value
- Ownership stays with the original variable

---

## Ownership Rule 2

### There can only be one owner at a time

When ownership is transferred, the old owner becomes **invalid**.

```rust
fn main() {
    let s1 = String::from("RUST");
    let s2 = s1;

    // println!("{}", s1); ❌ ERROR
    println!("{}", s2);   // ✅ Works
}
```

### What happened?

- `s1` owned `"RUST"`
- `s2 = s1` **moves ownership** to `s2`
- `s1` is no longer valid
- Rust prevents **double free memory bugs**

This is called a **move**, not a copy.

---

## Why does Rust do this?

Because `String` data lives on the **heap**:

- Copying heap data is expensive
- Two owners could try to free the same memory
- Rust prevents this at **compile time**

---

## Copy Types (Exception)

Some types are copied instead of moved:

- Integers (`i32`, `u32`, etc.)
- Floats
- Booleans
- Characters
- Tuples (if all elements are Copy)

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("{}", x); // ✅ Still valid
    println!("{}", y);
}
```

These types live on the **stack**, so copying is cheap and safe.

---

## Ownership Rule 3

### When the owner goes out of scope, the value is dropped

```rust
fn main() {
    let s1 = String::from("RUST");
    println!("{}", s1);
} // s1 goes out of scope here → memory is freed
```

### What does “dropped” mean?

- Rust automatically calls `drop`
- Heap memory is freed
- No memory leaks
- No manual cleanup needed

This is called **RAII** (Resource Acquisition Is Initialization).

---

## Common Mistake Example (Incorrect Code)

```rust
fn printLost(s: &string) {
    println!("{}", &s1);
}
```

### Why is this wrong?

- `string` ❌ should be `String`
- `s1` is not in scope
- Function parameters must be used directly

### Correct Version

```rust
fn print_lost(s: &String) {
    println!("{}", s);
}
```

---

## Ownership vs Borrowing Summary

| Concept       | Ownership       | Borrowing          |
| ------------- | --------------- | ------------------ |
| Who owns data | One variable    | Original owner     |
| Uses `&`      | ❌              | ✅                 |
| Can modify    | ✅ (if mutable) | ❌ (unless `&mut`) |
| Moves value   | ✅              | ❌                 |

---

## Key Takeaways

- Every value has **one owner**
- Ownership can **move**
- Borrowing allows **safe access**
- Values are **automatically dropped**
- Ownership rules prevent memory bugs **at compile time**
