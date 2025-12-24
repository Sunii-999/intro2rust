## Primitive data types

Rust has 4 data types: int, float, bool, char

Created a new folder and file `PrimitiveDataTypes/primitiveData.rs`

```rust
//PrimitiveDataTypes/primitiveData.rs
fn main(){

}
```

### Integer - int

Rust has signed (+ and -) and unsigned integer (only +) types of different sizes

Signed integers = i8, i16, i32, i64, i128
Unsigned integers = u8, u16, u32, u64, u128

```rust
let x: i32 = -42; // negative or positive
    let y: u64 = 100; //has to be positive


    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

 //diff bet i32 (32 bits) and i64 (64bits)
    // range:
    // i32 - 2147483647
    // i64 - 9223372036854775807

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maxiumum value of i32: {}", e);
    println!("Maxiumum value of i64: {}", i);
```

### Float - f32, f64

In Rust, `f32` and `f64` are primitive floating-point types, differing in size and precision: `f32` uses 32 bits (single-precision) for lower memory/faster SIMD, while `f64` uses 64 bits (double-precision) for higher accuracy, representing a much wider range, and is the default float type on modern hardware because it offers better precision for similar performance. Use `f64` by default unless memory is critical (like in games) or specific performance needs (like SIMD) dictate `f32`, and remember you need `as` to convert between them due to type safety.

```
let pi: f64 = 3.14;

    println!("Value of pi: {}", pi);
```

### Booleans - bool

```
let is_snowing: bool = true;
println!("Is it snowing? {}", is_snowing);

```

### Character - char

```rust
let letter: char = 'a';
println!("First letter of alpha: {}", letter);
```

### Example

```rust
fn main(){
    let x: i32 = -42;
    let y: u64 = 100;


    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maxiumum value of i32: {}", e);
    println!("Maxiumum value of i64: {}", i);

    let pi: f64 = 3.14;

    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of alpha: {}", letter);

}
```
