# Ownership

set of rules that governs how a rust program manages memory.

Memory is managed through a system of ownership with a set of files that the compiler checks. If rules are violated the program will not compile. 

## The Stack and the Heap

The language behaves differently when for values that are on the stack vs the heap. This affects how you make decisions in rust.

The stack stores values in the order it gets them and removes the values in the opposite order. *last in, first out*. All data stored on teh stack must have a known, fixed size. Data with an unknown size or a size that might change at compile time must be stored on the heap instead.

When you store something in the hearp. You put data on the heap and the memory allocator finds a spot for it marks it as in use and then returns a pointer, which is the address of that location.

Pushing to the stack is faster than allocating on the heap because there is never a search for a place to store new data. The location is always the top of the stack. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership address.

## Ownership rules

- Each value in rust has an owner.
- There can only be one owner at time.
- When the owner goes out of scope, the value will be dropped.

To create a sting type that is stored on the heap because the size is unknown at compile time you can do this:

```rust
let s = String::from("hello");
```

The double color `::` operator allows us to namespace this particular `from` under the `String` type rather than using some sort of name like `string_from`.

This kind of string can be mutated and grow or shink in size.

```rust
let mut s = string::from("hello");

s.push_str(", world!");

println!("{}", s);
```

## Memory and Allocation

In order for the string to support a mutable, growable piece of text we need to allocate an amount of memory on the heap that is unknown at compile time. This means:

- The memory must be requested from the memory allocator at runtime
- We need a way of returning this memory to the allocator when we are done with our `String`.

The first part is done when you call `String::from`. Instead of garbage collection, rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

## Variables and Data interacting with Move

complex data types or data that might change in size will not automatically be copied over to other variables. Instead it will 'move' the pointer that references the data on heap to the new variable on the stack. This means thatonce you set a variable equal to a new value of the same type, the old variable is invalidated and the memory will only be freed one the new variable goes out of scope. 

This is not the case for simple data types where the size is known at compile time because they can simply be stored on the stack.

## Ownsership and Functions

Passing a value to a function is similar to assigning a value to a variable. It will move or copy, just a assignment does. 

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

Instead of having to return the values that are passed in to keep using them in their initial scope you can use references and borrowing. 

## References and borrowing

Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust 
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The `&` symbol represents a reference, and they allow you to refer to some value without taking ownership of it.
 
Above were creating a reference that refers to the value of `s1` but does not own it. Because it does not have ownership of the value, the value it points to will not be dropped when the reference stops being used.

The action of creating a reference is called borrowing. References are immutable by default 

## Mutable references

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

When creating a mutable reference there is one big rule: you can only have one mutable reference to a particular piece of data in a particular scope.

The benefit of having this restriction is that rust can prevent data races at compile time. A data race happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism being used to synchronize access to the data.

We also cannot have a mutable reference while we have a immutable one to the same value.

## Dangling References

The compiler guarantees that references will never be dangling references. The compiler will ensure that the data will not go out of scope before the reference to the data does.


## **The Rules of References**

- At any given time, you can have *either* one mutable reference or any number of immutable references.
- References must always be valid.
