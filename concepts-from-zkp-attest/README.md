# Elements learnt



## Primitive type

### Slice

Slices are a view into a block of memory represented as a pointer and a length:
```
// slicing a Vec
let vec = vec![1,2,3];
let int_slice = &vec[..];
// coercing an array to a slice
let str_slice: &[&str] = &["one", "two", "three"];
```

`copy_from_slice(&mut self, src: &[T])` 
example:
```
let src = [1, 2, 3, 4];
let mut dst = [0, 0];

// Because the slices have to be the same length,
// we slice the source slice from four elements
// to two. It will panic if we don't do this.
dst.copy_from_slice(&src[2..]);

assert_eq!(src, [1, 2, 3, 4]);
assert_eq!(dst, [3, 4]);
```

## Box

## Lifetimes

- [x] [Lifetime annotations YouTube video](https://www.youtube.com/watch?v=rAl-9HwD858)

## Some and Option types

The if let statement is used to test whether a value matches a pattern, and if it does, it binds the values of the pattern to variables that can be used within the statement's block. If the value does not match the pattern, the statement does nothing.

When used with Option, the if let Some(...) syntax allows you to test whether an Option value is present and extract the wrapped value in a single line of code. Here's an example:

```rust
let x: Option<i32> = Some(42);

if let Some(value) = x {
    println!("The value is {}", value);
} else {
    println!("There is no value");
}
```

In this example, the if let statement tests whether x is Some, and if it is, the value variable is bound to the wrapped value (42 in this case). The println! macro is used to print the value to the console.

If x is None, the if let statement does nothing, and the else branch is executed instead. In this case, the message "There is no value" is printed.

Using if let Some(...) can make your code more concise and easier to read, especially when you are only interested in the case where the Option value is present. However, if you need to handle both the Some and None cases separately, you may prefer to use a match statement instead.


