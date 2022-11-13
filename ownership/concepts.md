## The Stack and the Heap

Stack: last in, first out. 

- Pushing onto the stack
- Popping off the stack
- All data has known, fixed size.

- Fast to push to the stack.


Heap: less organized

- Mamory allocator finds empty spot. Returns pointer (can be saved to stack)

- Slow to allocate on the heap because the allocator has to search for a place to store new data.


## Ownership rules

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Definition (scope of variables):
- A scope is the range within a program for which an item is valid.

## The rules of references

1. At any given time, you can have either
-- one mutable reference or
-- any number of immutable references
2. References must always be valid.
