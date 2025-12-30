# Compound Data Types

In the previous chapter, we learned about **primitive data types** such as integers, floats, booleans, and characters.  
Primitive types store **a single value**.

In this chapter, we move on to **compound data types** — types that can **group multiple values together**.  
These values can be of the **same type** or **different types**, depending on the structure.

Rust’s most common compound data types are:

- Arrays
- Tuples
- Slices
- Strings and String Slices (`&str`)

---

## Arrays

An **array** is a fixed-size collection of elements **of the same type**.  
Once an array is created, its size **cannot change**.

### Example: Integer Array

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
println!("Number Array: {:?}", numbers);
```

Explanation:

- `i32` → type of each element
- `5` → number of elements
- All elements must be `i32`
- Arrays are stored on the **stack**
- `{:?}` is used to print arrays using the `Debug` trait

---

### Example: String Slice Array

```rust
let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
println!("Fruits Array: {:?}", fruits);
```

Here:

- Each element is a **string slice** (`&str`)
- The array size is fixed at `3`

You can access elements using **indexing**:

```rust
println!("First Fruit: {}", fruits[0]);
println!("Second Fruit: {}", fruits[1]);
println!("Third Fruit: {}", fruits[2]);
```

> ⚠️ Accessing an index outside the array bounds will cause a **runtime panic**.

---

## Tuples

A **tuple** is a collection of values that can be of **different types**.
Tuples have a fixed size, just like arrays, but are more flexible because the types do not have to match.

### Example: Simple Tuple

```rust
let human: (&str, i16, bool) = ("Sunii", 30, false);
println!("Human: {:?}", human);
```

Explanation:

- `&str` → name
- `i16` → age
- `bool` → status
- Tuples group **related data together**

---

### Example: Mixed Tuple

```rust
let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
println!("My Mix Tuple: {:?}", my_mix_tuple);
```

This tuple contains:

- A string slice
- An integer
- A boolean
- An array

Tuples are useful when returning **multiple values from a function** or modeling simple data structures.

---

## Slices

A **slice** is a reference to a portion of a collection.
It does **not own the data** — it only borrows it.

Slices are written using `&[]`.

### Example: Integer Slice

```rust
let number_slices: &[i32] = &[1, 2, 3, 4, 5];
println!("Number Slice: {:?}", number_slices);
```

Explanation:

- `&[i32]` → a borrowed slice of integers
- The data itself is not owned by the slice

---

### Example: String Slice Collection

```rust
let animal_slices: &[&str] = &["Lion", "Zebra", "Giraffe", "Hippo"];
println!("Madagascar main characters: {:?}", animal_slices);
```

Each element is:

- `&str` → string slice
- The entire structure is borrowed using `&[...]`

---

### Example: Slice of `String` References

```rust
let book_slices: &[&String] = &[
    &"IT".to_string(),
    &"Zebra".to_string(),
    &"Giraffe".to_string(),
    &"Hippo".to_string()
];
println!("Madagascar main characters: {:?}", book_slices);
```

This example demonstrates:

- `String` → heap-allocated, owned string
- `&String` → borrowed reference to a `String`
- The slice itself does not own the data

> In real projects, prefer `&str` when possible for simplicity.

---

## Strings vs String Slices (`String` vs `&str`)

Understanding the difference between `String` and `&str` is **very important** in Rust.

### `String`

- Growable
- Mutable
- Owns its data
- Stored on the heap

### Example: Mutable String

```rust
let mut stone_cold: String = String::from("Hell, ");
stone_cold.push_str("YEAH");
println!("Stone Cold Says {}", stone_cold);
```

Explanation:

- `mut` → allows modification
- `push_str` → appends text
- The string owns its data

---

### `&str` (String Slice)

- Immutable
- Borrowed
- Does not own data
- Often points to part of a `String`

### Example: String Slice

```rust
let string: String = String::from("Hello, World");
let slice: &str = &string[0..5];
println!("Slice Value: {}", slice);
```

Explanation:

- `&string[0..5]` borrows part of the string
- No memory is copied
- Very efficient

---

## Scope Rules Reminder

A slice **cannot be used outside its scope**.

❌ This will not compile:

```rust
fn print() {
    println!("Slice: {}", slice);
}
```

Because:

- `slice` is defined inside `main`
- Rust prevents accessing data that may no longer exist

This is part of Rust’s **ownership and borrowing safety model**.

---

## Summary

| Type   | Fixed Size | Can Mix Types | Owns Data | Mutable |
| ------ | ---------- | ------------- | --------- | ------- |
| Array  | ✅         | ❌            | ✅        | ❌      |
| Tuple  | ✅         | ✅            | ✅        | ❌      |
| Slice  | ❌         | ❌            | ❌        | ❌      |
| String | ❌         | ❌            | ✅        | ✅      |
| &str   | ❌         | ❌            | ❌        | ❌      |

---

## Key Takeaways

- Use **arrays** for fixed-size collections of the same type
- Use **tuples** to group different types together
- Use **slices** to borrow parts of data
- Use **String** when you need ownership and mutability
- Use **&str** when borrowing text data

```

```
