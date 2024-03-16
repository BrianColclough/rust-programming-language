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



