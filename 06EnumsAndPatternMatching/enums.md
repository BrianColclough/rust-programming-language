# Enums

Enums have lots of features in rust

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

The name of the enum variant that we define also becomes a function that constructs and instance of the enum.

Another advantage to useing an enm over a struct: each variant can have different types and amounts of associated data. You could store the IP addresses like this:

```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

You can put any kind of data inside an enum: strings, numeric types, or structs.

You can also define methods on enums.

```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
```

A similarity between structs and enums is that you can define methods on enums with `impl`

```rust
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

## The Option Enum and Its Advantages Over Null Values

The `Option` type encodes the very common scenario in which a value could be sometime or it could be nothing.

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. The enum is `Option<T>`, and it is defined by the standard library:

```rust
enum Option<T> {
    None,
    Some(T),
}
```


