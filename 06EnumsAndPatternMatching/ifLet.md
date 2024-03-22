# Concise Control Flow with if let

lets you combine `if` and `let` into a less verbose way to match one pattern while ignoring the rest.

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

This allows you to handle the `Some` case while not having to add code to handle the `None`

`if let` takes a pattern and an expression separated by an equal sign.

you can combine `if let` with an else to handle other cases just like `_` in match

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```
