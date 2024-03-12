# Common Programming Concepts

## Variables and Mutability

- Variables are immutable by default, can opt out but favor immutability.
- Compiler will catch compile time errors like assigning an immutable variable 2 times

### Constants

- _constants_ are values that are bound to a name and are not allowed to change, but there are a few differences
  between constants and variables
  - You aren't allowed to use `mut` with constants.
  - Constants are declared using the `const` keyword and the type of the value must be annotated.
  - Must be set to a constant expression, not the result of a value that could only be computed at runtime.


