# Why use Rust?

Rust is a programming language that is built to be fast, secure, reliable and supposedly easier to program. _Aims to be an all-purpose programming language._

## Features

- **Speed**
  Extremely fast when it comes to compilation and execution time.
- **Zero Cost Abstraction**
  Generics, collections, etc… do not come with a run time cost, only compile time cost. Any operation on zero-cost abstractions is as fast as you would write out matching functionality by hand using lower-level programming concepts like for loops, counters, ifs and using raw pointers.
  So in summary _zero-cost abstraction tools, functions, templates, classes and such come with "zero cost" for the performance of your code._
- **Code should be able to run without crashing *first time***
  Statically typed language, which leaves almost no room for program to crash
- **Builds optimized code for Generics**
  Rust’s compiler can smartly identify the code for, say, Lists and Sets, and optimize it differently for both.
  ![funny meme shoutout frank](https://pbs.twimg.com/media/Ei-vydPWoAEvpxR.png)
- **Compilation Error’s**
  Rust’s compiler has the really helpful error messages if you mess up your code. In some cases, the compiler will give you code that you can copy/paste in order for it to work.

# Installation

## **Windows**

On Windows, the recommended way to install Rust is via `rustup-init.exe`.

1. Download the installer from the official website: https://rustup.rs/
2. Run the executable and follow the on-screen instructions.

After installation, you can verify it by opening a new terminal and typing:

```powershell
rustc --version
```

---

## macOs

On macOS, you can install Rust using the `curl` command in your terminal. This downloads and runs the `rustup` script.

1. Open your terminal.
2. Run the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

1. Follow the prompts to proceed with the installation (usually selecting option `1`).
2. Once finished, configure your current shell:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
```

---

## Linux

The installation process for Linux is identical to macOS, using `curl`.

1. Open your terminal.
2. Run the installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSf [<https://sh.rustup.rs>](<https://sh.rustup.rs>) | sh
```

1. If `curl` is not installed, you may need to install it first (e.g., `sudo apt install curl` on Ubuntu/Debian).
2. After the script finishes, source the environment variables:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
```

> Note: You may also need a C linker installed. On Ubuntu/Debian, you can get this by running sudo apt install build-essential.

## Update & Uninstall

Once Rust is installed via `rustup`, updating to a newly released version is easy. From your shell, run the following update script:

```bash
rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your shell:

```bash
rustup self uninstall
```

To build  `cargo projects`, run the following build script from your shell:

```bash
cargo build
```

---

# MUST KNOW

Everything labeled with Ch it mostly basically side by side following the tutorial of [BekBrace](https://www.youtube.com/watch?v=rQ_J9WH6CGk&t=4539s)

I also did my own research for small changes that have happened in Rust since then (small name changes).


---

# Sources

[Everything-You-Need-To-Know-About-Rust](https://medium.com/codex/rust-101-everything-you-need-to-know-about-rust-f3dd0ae99f4c)

[BekBrace](https://www.youtube.com/watch?v=rQ_J9WH6CGk&t=4539s)

[Zero cost abstraction Rust](https://stackoverflow.com/questions/69178380/what-does-zero-cost-abstraction-mean/69178445#69178445)

[TheRustBook](https://doc.rust-lang.org/book/title-page.html)

## Copyright & Attribution

© 2025 [Sunii](https://www.sunii.me/) .

This repository documents original research and learning conducted by the author. All explanations and implementations were written independently, with external sources referenced where relevant.

You are free to study, learn from, and reference this work. If you reuse or adapt significant portions, please provide appropriate attribution and preserve references to original sources.
