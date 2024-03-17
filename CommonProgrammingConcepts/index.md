# Common Programming Concepts

## Variables and Mutability

- Variables are immutable by default, can opt out but favor immutability.
- Compiler will catch compile time errors like assigning an immutable variable 2 times

### Constants

- constants_ are values that are bound to a name and are not allowed to change, but there are a few differences
  between constants and variables
  - You aren't allowed to use `mut` with constants.
  - Constants are declared using the `const` keyword and the type of the value must be annotated.
  - Must be set to a constant expression, not the result of a value that could only be computed at runtime.

### Shadowing

- Shadowed variables are what the compiler will see when you use the name of the variable.

```rust 
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

The curly braces here are creating a new scope, and in that scope x can be shadowed. The result here is:

```
The value of x in the inner scope is: 12
The value of x is: 6
```

- different from marking variables as `mut`
- because using `let` is effectively creating a new variable you can even change the type of the variable.

We can change the type here: 
```rust
let spaces = "   ";
let spaces = spaces.len();
```

You cannot use `mut` here because you cannot change the type of a mutable variable.


## Data Types

- Rust is statically types, so the compiler must know the types of all the variables at compile time.
 
To add a type to a variable it is specifed after the variable name and with a colon

```rust 
let guess: u32 = "42".parse().expect("Not a number!");
```

### Scalar Types 

1. Integers 
2. Floating-point numbers
3. Booleans
4. Characters

This is similar to other languages.
 
#### Integer Types  

- Signed and unsigned
    - signed means the number must have a sign with it so it can be position or negative
    - unsigned means the number can only ever be positive and does not need to be represented with a sign.

#### Floating point numbers 

- Numbers with decimal points
- `f32` and `f64`
- all floating point types are signed

### Compound Types
 
- Tuples
    - General way of grouping together a number of values with different types into one compound type.
    ```rust 
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    ```
     
### Arrays

- Arrays are a fixed length
- *Useful with you want your data allocated on the stack rather than the heap*
- Vectors are similar to arrays but they are allowed to grow or shrink in size.

You specify the type and length of an array like this:

```rust 
let a: [i32; 5] = [1, 2, 3, 4, 5]; 
```

shorthand for specifying the same value for each element of the array:

```rust
let a = [3; 5];
```

## Functions

### Statements and Expressions

- **Statements** are insructions the perform some action and do not return a value.
- **Expressions** evaluate to a resultant value.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
 
- The value of 4 gets bound to `y` as part of the `let` statement. Expressions do not include the ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

