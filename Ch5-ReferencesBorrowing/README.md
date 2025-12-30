## Chapter 5 – References and Borrowing

While ownership rules ensure memory safety, passing ownership back and forth can be tedious.

**Borrowing** allows us to access data without taking ownership.

However, when we need to **change** that data, we must use **Mutable References**.

---

## The Basics of Mutable References

### Modifying data without moving ownership

To change a value without taking ownership, we use `&mut`.

```rust
fn main() {
    let mut x: i32 = 5;

    // Create a mutable reference
    let r: &mut i32 = &mut x;

    *r += 1; // De-reference and modify
    *r -= 3;

    println!("Value of x via reference is: {}", r);
}

```

### What’s happening here?

- `let mut x` allows `x` to be changed
- `&mut x` creates a **mutable reference** to `x`
- `*r` (dereference) allows us to access and modify the underlying value
- Ownership of `x` does **not** move

---

## The Golden Rule of Borrowing

Rust enforces strict rules for references to ensure **safety and performance**.

### The Rule

> At any given time, you can have **EITHER**:
>
> 1. **One** mutable reference (`&mut T`)
> 2. **Any number** of immutable references (`&T`)

You **cannot** have both at the same time.

### Why does Rust do this?

This prevents **Data Races** at compile time. A data race occurs when:

1. Two or more pointers access the same data at the same time.
2. At least one of them is writing to the data.
3. There is no mechanism to synchronize access.

Rust solves this by ensuring that if you can **change** data, you are the **only one** looking at it.

---

## Real-World Demonstration: The Bank Account

Let's apply this to a struct method context.

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Needs mutable access to modify balance
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // Only needs read access
    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}

```

### Method Syntax Breakdown

- `&self`: Short for `self: &Self`. The method **borrows** the instance immutably. It can read, but not change.
- `&mut self`: Short for `self: &mut Self`. The method **borrows mutably**. It can change the fields of the struct.

---

## Executing the Logic

```rust
fn main() {
    let mut account = BankAccount {
        owner: "Sunii".to_string(),
        balance: 150.54,
    };

    // 1. Immutable borrow (Read-only)
    account.check_balance();
    // Borrow ends here

    // 2. Mutable borrow (Read & Write)
    account.withdraw(54.65);
    // Borrow ends here

    // 3. Immutable borrow again
    account.check_balance();
}

```

### Flow Analysis

1. **Creation**: `account` is created as `mut`. This is required to use `&mut` methods later.
2. **Read**: `check_balance` takes a snapshot view. No other changes can happen _during_ this function call.
3. **Write**: `withdraw` takes exclusive control. It modifies the balance.
4. **Read**: Access is returned, and we can read the new balance.

---

## What Happens if We Break the Rules?

If we tried to copy the reference and use them simultaneously, Rust would stop us.

```rust
// ❌ THIS WILL FAIL TO COMPILE
let r1 = &account;      // Immutable
let r2 = &mut account;  // Mutable - ERROR!

println!("{}, {}", r1.balance, r2.balance);

```

**Error:** `cannot borrow 'account' as mutable because it is also borrowed as immutable`.

---

## Immutable vs Mutable Summary

| Feature      | Immutable Ref (`&T`)       | Mutable Ref (`&mut T`)      |
| ------------ | -------------------------- | --------------------------- |
| **Syntax**   | `&variable`                | `&mut variable`             |
| **Access**   | Read-only                  | Read and Write              |
| **Quantity** | Unlimited                  | **Exactly One**             |
| **Analogy**  | Audience watching a screen | Editor typing on the screen |

---

## Key Takeaways

- Use `&mut` only when you need to **change** data.
- You must declare the variable as `mut` to create a `&mut` reference.
- **Methods** use `&self` (read) and `&mut self` (write) to define their permissions.
- Rust prevents **data races** by ensuring exclusive access when writing.
