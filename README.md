# Rust Programming Fundamentals

Rust is a system programming language
- Safety
- Concurrency
- Speed
  
```rust
fn main() {
	println!("Hello World");
}
```

Rust - 2015
Cargo
	- package manager
	- build system
	- test runner
	- doc generator
It has all the good parts of npm, pip

Variables are by default immutable in Rust
- A lot of bugs wont happen if the data never change - Safety is ensured
- Data that never changes could be shared across multiple threads without locks so - concurrency is improved
- Compiler can do some extra optimizations on data it knows wont change so speed is improved

## Scope of Variable
- Refers to where it is created and extends to the end of the block, along the way it is extended to nested block.
- Variable can be shadowed in the inner block
- Variables are always local to their scope
- We can shadow the  variable in the same scope

```rust
  let x = 15;
  {
    let x = x + 2;
    let y = 51;
    println!("x = {}, y = {}", x, y); // prints 17, 51
  }
  println!("x = {}, y = {}", x, y); // Error
```

## Memory Safety
Rust guarantees memory safety at compile time
that means variables must be initialized before we can use them.
```rust
let enigma: i32;
println!("enigma = {}", enigma);

```
This code snip will not even compile and give error that enigma variable is un-initialized
```rust
 let yellow: i32;
  if true {
    yellow = 27
  } else {
    yellow = 42
  }
 println!("yellow = {}", yellow);
```
This will be executed as Rust can figure out at Compile time that in case the if block isn't executed then variable initialization will happen in the else block.

In case there is just the If block then Rust will throw the error that the variable is possibly un-initialized.

## Functions
Here is the snippet for a Function
```rust
fn do_sum(num1 : i32, num2 : i32) -> i32 {
  return num1 + num2;
}
```
