# Introduction

Create a new project folder, i am doing this on my desktop where i have a dedicated folder for learning languages. Rust files always end with the *.rs* extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use *hello_world.rs* rather than *helloworld.rs*.

```bash
cd Desktop && mkdir LearningLanguages && cd LearningLanguages && mkdir Rust && cd Rust && mkdir Basics && cd Basics && mkDir Introduction && cd Introduction && touch main.rs
```

Save the file and go back to your terminal window in the *~Desktop/LearningLanguages/Rust/Basics* directory.

Inside [`main.rs`](http://main.rs)put the following:

```rust
fn main() {
    println!("Hello, DRGT!")
}
```

Run this command in your terminal / shell:

```bash
rustc main.rs
./main
```

Output:

```rust
stijnwalravens@Thomass-Mac-mini Basics % rustc main.rs
stijnwalravens@Thomass-Mac-mini Basics % ./main
Hello, DRGT!
```

After every change in here you have to recompile with `rustc main.rs` and run the command again. Manual recompiling is only needed if the files are also manually created. If you create a project via cargo this isnt needed anymore than you are able to run `cargo run`

## Cargo

Cargo downloads your Rust [package](https://doc.rust-lang.org/cargo/appendix/glossary.html#package)’s dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io/), the Rust community’s [_package registry_](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-registry).

To create one you do the following:

```bash
 stijnwalravens@Thomass-Mac-mini Basics % cargo new basicProject
```

Now a new file has been created inside your project

![image.png](attachment:fdc8eba9-f383-418e-adc5-fcc10f6ccf5d:image.png)

Inside `src/main.rs` you will have a hello world print as default but once you change this and save to text of choice and

Example:

```
fn main() {
    println!("Hello, DRGT via CARgo!");
}
```

cd into your new project directory and to run this cargo you do the following:

```bash
cd basicProject
cargo run
```

Output:

```bash
stijnwalravens@Thomass-Mac-mini basicProject % cargo run
   Compiling basicProject v0.1.0 (/Users/stijnwalravens/Desktop/LearningLanguages/Rust/Basics/basicProject)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/basicProject`
Hello, DRGT via CARgo!
```
