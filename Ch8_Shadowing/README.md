## Chapter 8 – Shadowing

In the previous chapter, we learned that variables are immutable by default.

However, sometimes you want to reuse a variable name for a new value—or even a new type—without making the variable mutable.

This is called **Shadowing**.

It allows you to declare a new variable with the **same name** as a previous variable. The new variable "shadows" the previous one.

---

## Shadowing in Action

```rust
fn main() {
    let x = 5;

    let x = x + 1; // Shadows the first x

    {
        let x = x * 2; // Shadows inside this scope only
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

```

### Output

```text
The value of x in the inner scope is: 12
The value of x is: 6

```

### What’s happening here?

1. **Line 2:** We bind `x` to `5`.
2. **Line 4:** We use `let x =` again. This creates a **brand new variable** named `x`. It takes the original value (`5`), adds `1`, and stores `6`. The original `x` is now shadowed (hidden).
3. **Line 6 (Inner Scope):** We start a new block `{}`.
4. **Line 7:** We use `let x =` again. This creates a third `x` specific to this inner scope. It takes the current `x` (`6`), multiplies by `2`, and stores `12`.
5. **Line 8:** We print the inner `x` (`12`).
6. **Line 9:** The scope ends. The inner `x` is dropped. The shadowing ends.
7. **Line 11:** We are back to the second `x`. The value is `6`.

---

## Shadowing vs Mutability

It is easy to confuse **Shadowing** with **Mutability**, but they are very different.

### 1. Re-using `let`

With shadowing, we use the `let` keyword again. We are effectively creating a fresh variable that just happens to have the same name.

### 2. Changing Types

Because we are creating a **new** variable, we can change the **data type**.

```rust
fn main() {
    let spaces = "   ";       // Type: &str (String slice)
    let spaces = spaces.len(); // Type: usize (Integer)

    println!("There are {} spaces", spaces);
}

```

If we tried this with `mut`, it would fail:

```rust
let mut spaces = "   ";
spaces = spaces.len(); // ❌ ERROR: expected string, found integer

```

### 3. Immutability Preservation

After the shadowing is done, the new variable is **immutable** (unless we explicitly write `let mut`). This keeps our code safe—we calculate a new value, but once it's set, it won't change accidentally.

---

## Shadowing vs Mutability Summary

| Feature         | Shadowing                      | Mutability                 |
| --------------- | ------------------------------ | -------------------------- |
| **Keyword**     | `let variable_name = ...`      | `variable_name = ...`      |
| **Memory**      | Allocates new memory           | Overwrites existing memory |
| **Type Change** | ✅ Allowed                     | ❌ Not Allowed             |
| **Scope**       | Can be limited to a block `{}` | Persists in current scope  |

---

## Key Takeaways

- Shadowing allows you to **reuse variable names**.
- It creates a **new variable** rather than updating the old one.
- It is useful for **transforming values** (e.g., input string → parsed number) without creating names like `x_str`, `x_int`.
- Inner scopes can shadow outer variables, but the original returns when the scope ends.
