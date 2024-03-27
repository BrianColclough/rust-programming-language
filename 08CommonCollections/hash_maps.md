# Hash Maps

Types the implement the copy trait will be copied into the hashmap but others that don't will have their ownership transferred.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```

Rust provides a way to check if a key is already in a map, in this example we only want to insert the values if the associated key does not exist

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

Update a value based on an old value. We will count the number of times a word appears in text. If the word is not present as a key then a new key will be inserted and associated with value 0:

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

Here we are dereferencing `count` for some reason that I have not learned yet. I know tha because `count` is a mutable referece for some reason we need to dereference it first. Maybe because its a reference we have to deref to ensure the modifications are made to the actual value in memory.


