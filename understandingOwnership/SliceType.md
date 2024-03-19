# The Slice Type

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is kind of like a reference, so it does not have ownership.

## String Slices

a *string slice* is a reference to part of a `String`. 

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

## Other Slices

Just as you refer to a string slice you can refer to part of an array

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
