Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

1. Packages: A Cargo feature that lets you build, test, and share crates.
2. Crates: A tree of modules that produces a library or executable.
3. Modules and use: Let you control the organization, scope, and privacy of paths.
4. Paths: A way of naming an item, such as a struct, function, or module.

# Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules and modules can be defined in other files that get compiled with the crate.

Crates come in either binary or library form. Binary crades can compile to an exe, like a command line tool or server. Each must have a function called `main` that defined what happend when the exe runs

_Library_ crates don't define a `main` and they don't compile into an exe. They define functionality that is intended to be shared with multiple projects.

## Modules Cheat Sheet

- Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
- Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following mod garden
  - In the file src/garden.rs
  - In the file src/garden/mod.rs
- Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
  - In the file src/garden/vegetables.rs
  - In the file src/garden/vegetables/mod.rs
- Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
- Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
- The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.

## Grouping Related Code in Modules

Modules allow organization of code and the ability to control the privacy of items.

Looking at the code in this directory, you loll see modules structured like this

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

`lib.rs` and `main.rs` are called crate roots, either of these two files form a module named crate at the root of the crate's module structure, known as the module tree.

Everything is private by default, so if you want to expose API you have to be explicit about it. Exposing a parent struct or module does no expose its children. You have to manually expose each child.

In contrast, exposing an `enum` all of its variants are then public.

## Bringing Paths into Scope with the `use` keyword

`use` is basically a shorcut so you don't have to write out the whole path everytime

```Rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

You can re-export items to make them public by using `pub use`. This is useful to expose a module while not exposing the codes internal structure.

## Using Nested Paths

You can bring in multiple items with a single `use` statement:

```Rust
use std::{cmp::Ordering, io};
```

To merge use statements that share a common path you can do this:

```Rust
// Original
use std::io;
use std::io::Write;

// Merged
use std::io::{self, Write};
```

## Glob

If you want to bring in *all* public items defined in a path. Use the `glob` operator: `*`:

```Rust
use std::collections::*;
```
