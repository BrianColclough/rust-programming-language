# Defining and Instantiating Structs

Structs are like types

To define a struct, use the keyword `struct` and name the entire struct. The name should describe the significance of the pieces of data being grouped together.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

You can create an instance of the struct like this:

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

You use dot notation to get a specific value from a struct. To access the email of `user1` you would use `user1.email`.

To make a struct mutable you must make the entire instance mutable.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

If you have a function that takes in options and returns a struct you can use the shorthand if the functions parameteres have the same name as the struct's fields

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

In this example we can no longer user `user1` because the username was moved from `user1` to `user2`. If we had given `user2` a new username and new email, we could still use `user1` because both `active` and `sign_in_count` implement the `copy` trait.

## Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit-Like Stucts Without Any fields

Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

```rust
struct AlwaysEqual

fn main() {
    let subject = AlwaysEqual;
}
```

## Associated functions

Assocated functions are all functions defined within an `impl` block. We also define functions that do not take self as the first parameter

For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function we use the `::` syntax. `let sq = Rectangle::square(3);`

Structs can have multiple `impl` blocks. 
