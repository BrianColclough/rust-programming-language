# Ownership

set of rules that governs how a rust program manages memory.

Memory is managed through a system of ownership with a set of files that the compiler checks. If rules are violated the program will not compile. 

## The Stack and the Heap

The language behaves differently when for values that are on the stack vs the heap. This affects how you make decisions in rust.

The stack stores values in the order it gets them and removes the values in the opposite order. *last in, first out*. All data stored on teh stack must have a known, fixed size. Data with an unknown size or a size that might change at compile time must be stored on the heap instead.

When you store something in the hearp. You put data on the heap and the memory allocator finds a spot for it marks it as in use and then returns a pointer, which is the address of that location.

Pushing to the stack is faster than allocating on the heap because there is never a search for a place to store new data. The location is always the top of the stack.
