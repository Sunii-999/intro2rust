# Guessing game

[source](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

Set up a new project, create a new directory in your root project folder named `RustBookExercises`, like so:

```shell
cargo new guessing_game
cd guessing_game
```

## Processing a Guess

The first part of the guessing game program will ask for user input, process that input, and check that the input is in the expected form. To start, we’ll allow the player to input a guess. Enter the code listed below into `guessing_game/src/main.rs`.

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

This code contains a lot of information. To obtain user input and then print the result as output, we need to bring the io input/output library into scope. The io library comes from the standard library, known as std:

```rust
use std::io;
```

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.

---

## Comparing the Guess

Now that we can read a user’s guess, the next step is to compare it to the secret number. We’ll use the `cmp` method to compare the two numbers and give feedback to the player.

Update your `main.rs` file with the following code:

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Key Points

1. **Using `rand` to generate a number:**
   The `rand` crate lets us generate a random number. Here, `thread_rng().gen_range(1..=100)` gives a number between 1 and 100.

2. **Parsing input safely:**
   We convert the string input into a number using `trim().parse()`. If the input is invalid, `match` allows us to continue the loop without crashing.

3. **Comparing values with `cmp`:**
   The `cmp` method returns an `Ordering` (`Less`, `Greater`, or `Equal`) so we can respond appropriately to the user’s guess.

4. **Looping until correct:**
   The `loop` keyword allows the program to continue prompting the user until they guess the secret number.
