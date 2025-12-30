## Chapter 7 – Comprehensive Exercise

To solidify your understanding, we will combine **Ownership**, **Borrowing**, **References**, and **Mutability** into a single challenge.

This exercise simulates a simple **RPG Game System**. You will manage a game character's lifecycle: creating them, reading their stats, modifying their health, and finally transferring them to a "Graveyard" (moving ownership).

---

## The Challenge: "The Hero's Journey"

Your goal is to complete the code below so that it compiles and runs without errors.

### Requirements

1. **Fix the Struct:** The `Character` struct needs to be defined.
2. **Implement Methods:**

- `inspect`: Borrows the character **immutably** to print details.
- `heal`: Borrows the character **mutably** to increase health.

3. **Handle Ownership:** Implement a function `retire_character` that **takes ownership** of the character, preventing it from being used again.
4. **Fix `main`:** Ensure variables are mutable where necessary and references are passed correctly.

---

## Starter Code (Copy this!)

```rust
// 1. Define the struct 'Character' with fields: name (String), health (i32)
struct Character {
    // ???
}

impl Character {
    // 2. Complete this method to PRINT the name and health
    // It should NOT take ownership, only READ access
    fn inspect(_______) {
        println!("Stats: {} has {} HP", self.name, self.health);
    }

    // 3. Complete this method to MODIFY the health
    // It must allow changing values
    fn heal(_______, amount: i32) {
        self.health += amount;
        println!("{} healed for {} HP!", self.name, amount);
    }
}

// 4. Complete this function to TAKE OWNERSHIP
// The character should be moved into this function and dropped here
fn retire_character(character: Character) {
    println!("{} has retired from adventuring.", character.name);
} // character is dropped here

fn main() {
    // 5. Fix this variable declaration
    // We need to modify 'hero' later, so is 'let' enough?
    let hero = Character {
        name: String::from("Rustacean"),
        health: 50,
    };

    // 6. Call inspect()
    hero.inspect();

    // 7. Call heal() - heal the hero by 20 points
    hero.heal(20);

    // 8. Call inspect() again to see the change
    hero.inspect();

    // 9. Call retire_character()
    retire_character(hero);

    // 10. UNCOMMENT the line below. It should CAUSE AN ERROR if you did step 9 correctly.
    // hero.inspect();
}

```

---

## Solution & Explanation

Try to solve it yourself first! When you are ready, check the solution below.

<details>
<summary><strong>Click to Reveal Solution</strong></summary>

```rust
struct Character {
    name: String,
    health: i32,
}

impl Character {
    // Uses &self because we only need to READ
    fn inspect(&self) {
        println!("Stats: {} has {} HP", self.name, self.health);
    }

    // Uses &mut self because we need to MODIFY
    fn heal(&mut self, amount: i32) {
        self.health += amount;
        println!("{} healed for {} HP!", self.name, amount);
    }
}

fn retire_character(character: Character) {
    println!("{} has retired from adventuring.", character.name);
} // character is dropped here

fn main() {
    // Must use 'mut' because we call heal() later
    let mut hero = Character {
        name: String::from("Rustacean"),
        health: 50,
    };

    // Immutable borrow (implicit &hero)
    hero.inspect();

    // Mutable borrow (implicit &mut hero)
    hero.heal(20);

    // Immutable borrow again
    hero.inspect();

    // MOVES ownership of 'hero' to the function
    retire_character(hero);

    // ❌ This would error because 'hero' has moved!
    // hero.inspect();
}

```

</details>

---

## Why this works (The Logic)

| Step              | Concept Used     | Explanation                                                                                                 |
| ----------------- | ---------------- | ----------------------------------------------------------------------------------------------------------- |
| `let mut hero`    | **Mutability**   | We needed `mut` because `hero.heal()` changes the data inside the struct.                                   |
| `inspect(&self)`  | **Borrowing**    | We only wanted to read the data. We used `&` so ownership didn't move.                                      |
| `heal(&mut self)` | **Mutable Refs** | We updated the health. We needed exclusive access (`&mut`) to avoid data races.                             |
| `retire(hero)`    | **Ownership**    | We passed `hero` directly (no `&`). Ownership moved into the function, and `hero` became invalid in `main`. |

---

## Key Takeaways

This exercise demonstrates the lifecycle of Rust data:

1. **Creation** (allocating memory)
2. **Modification** (mutating safely via references)
3. **Destruction** (moving ownership to a final scope where it is dropped)

If you can understand this flow, you have mastered the hardest learning curve in Rust!
