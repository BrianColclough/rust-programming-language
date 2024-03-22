Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

1. Packages: A Cargo feature that lets you build, test, and share crates.
2. Crates: A tree of modules that produces a library or executable.
3. Modules and use: Let you control the organization, scope, and privacy of paths.
4. Paths: A way of naming an item, such as a struct, function, or module.

# Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules and modules can be defined in other files that get compiled with the crate.

