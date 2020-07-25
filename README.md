# CircularBuffer

A zero dependencies, zero run-time allocation, circular buffer.

This crates provide a simple circular buffer that does not do any allocation at run time. The
main focus of this crate is correctess and performances.

The circular buffer never wait, if the buffer is full, it overwrite the first element.

The API is extremelly simple, you create the buffer specify how many elements the buffer can
hold. Then you can start pushing elements into it.

```
use rbl_circular_buffer::*;

let mut buffer = CircularBuffer::new(3);
assert_eq!(0, buffer.len());

buffer.push(1);
assert_eq!(1, buffer.len());

buffer.push(2);
assert_eq!(2, buffer.len());

buffer.push(3);
assert_eq!(3, buffer.len());

// now the buffer is full, we can insert the next element, but it will overwrite the first one
buffer.push(4);
assert_eq!(3, buffer.len());

let v: Vec<u32> = buffer.collect();
assert_eq!(vec![2,3,4], v);
```
There are two ways to read the elements from the buffer.
1. `CircularBuffer` implement the `Iterator` trait, you can loop over it.
2. `CircularBuffer` provided the `.fill()` method.

## Using the iterator

The iterator will consume the elements in the buffer.

```
use rbl_circular_buffer::*;

let mut buffer = CircularBuffer::new(3);
buffer.push(1);
buffer.push(2);
buffer.push(3);

let mut sum = 0;
for element in &mut buffer {
    sum += element;
}
assert_eq!(1 + 2 + 3, sum);
assert_eq!(0, buffer.len());
```

## Filling a vector

In demanding application, the iterator can be a bad choice.

Think about communication between threads, each thread can have a reference to the
`CircularBuffer` and take a lock while reading from it. If the reading operation are not fast
enough, or simply if there are too many elements, the lock will be hold for a long period of
time. The alternative is to fill a vector.

```
use rbl_circular_buffer::*;

// let's make a bigger vector
let mut buffer = CircularBuffer::new(5);
for i in 1..=5 {
    buffer.push(i);
}

// with this vector we will remove the first 3 elements
let mut v = Vec::with_capacity(3);

buffer.fill(&mut v);
assert_eq!(vec![1, 2, 3], v);

// in the vector there are still 4 and 5
assert_eq!(2, buffer.len());

buffer.push(6);
buffer.push(7);
buffer.push(8);

// the fill avoid any allocation even in the vector to fill.
// if we remove one element, and refill, we will push only one element.
// this because `.fill()` does not allocate any memory.

v.remove(0);

buffer.fill(&mut v);

assert_eq!(vec![2, 3, 4], v);
assert_eq!(4, buffer.len())
```



