use super::*;

use proptest::prelude::*;
#[test]
fn empty_buffer_has_len_zero() {
    let b = CircularBuffer::<u32>::new(16);
    assert_eq!(0, b.len());
}

#[test]
fn insert_one_element_return_len_minus_one() {
    let mut b = CircularBuffer::<u32>::new(16);
    let empty_spots = b.push(0);
    assert_eq!(1, b.len());
    assert_eq!(15, empty_spots);
}

#[test]
fn insert_too_many_elements_will_return_zero_empty_spots() {
    let mut b = CircularBuffer::<u32>::new(8);
    let mut empty_spots = Vec::new();
    let mut lens = Vec::new();
    for i in 0..10 {
        let empty_spot = b.push(i);
        let len = b.len();
        empty_spots.push(empty_spot);
        lens.push(len);
    }
    assert_eq!(lens, vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 8]);
    assert_eq!(empty_spots, vec![7, 6, 5, 4, 3, 2, 1, 0, 0, 0]);
}

#[test]
fn insert_one_element_and_removing_it() {
    let mut b = CircularBuffer::<u32>::new(8);
    b.push(10);
    let mut r = Vec::with_capacity(8);
    let returned = b.fill(&mut r);
    assert_eq!(returned, 1, "wrong returned element");
    assert_eq!(r[0], 10, "wrong element returned");
    assert_eq!(b.len(), 0, "wrong len of buffer");
}

#[test]
fn try_to_fill_on_full_vector() {
    let mut b = CircularBuffer::<u32>::new(8);
    b.push(10);
    let mut r = Vec::with_capacity(0);
    let returned = b.fill(&mut r);
    assert_eq!(returned, 0);
    assert_eq!(r.len(), 0);
    assert_eq!(r.capacity(), 0);
    assert_eq!(b.len(), 1);
}

// discovered with proptest!
#[test]
fn adding_to_size_one_works() {
    let mut b = CircularBuffer::<u32>::new(1);
    b.push(4);
    assert_eq!(1, b.len());
}

#[test]
fn size_2_adding_to_size_2() {
    let mut b = CircularBuffer::<u32>::new(2);
    assert_eq!(0, b.len());

    assert_eq!(1, b.push(0));
    assert_eq!(1, b.len());

    assert_eq!(0, b.push(0));
    assert_eq!(2, b.len());

    assert_eq!(0, b.push(0));
    assert_eq!(2, b.len());
}

#[test]
fn removing_more_element_than_capacity() {
    let mut b = CircularBuffer::<u32>::new(2);
    assert_eq!(0, b.len());

    assert_eq!(1, b.push(1));
    assert_eq!(1, b.len());

    assert_eq!(0, b.push(2));
    assert_eq!(2, b.len());

    assert_eq!(0, b.push(3));
    assert_eq!(2, b.len());

    let mut v = Vec::with_capacity(4);
    let returned = b.fill(&mut v);
    assert_eq!(2, returned);

    assert_eq!(2, v[0]);
    assert_eq!(3, v[1]);

    assert_eq!(0, b.len(), "len should be zero but it is {}", b.len());
    assert_eq!(2, v.len());
}

#[test]
fn size_2_adding_3_remove_2_add_1_remove_0() {
    let mut b = CircularBuffer::<u32>::new(2);
    let one = b.push(1);
    assert_eq!(1, one, "there should be one empty spot");

    b.push(2);
    b.push(3);

    let mut v1 = Vec::with_capacity(2);
    let returned = b.fill(&mut v1);
    assert_eq!(2, returned);

    assert_eq!(2, v1[0]);
    assert_eq!(3, v1[1]);

    assert_eq!(0, b.len());

    let one = b.push(4);
    assert_eq!(1, one, "there should be one empty spot");
    assert_eq!(1, b.len(), "there should be one elmenet in the buffer");
}

#[test]
fn fast_fill_test1() {
    let mut a = CircularBuffer::new(4);
    let mut b = CircularBuffer::new(4);
    for i in vec![1, 2, 3, 4] {
        a.push(i);
        b.push(i);
    }
    let mut drainer_a = Vec::with_capacity(1);
    let mut drainer_b = Vec::with_capacity(1);
    a.fill(&mut drainer_a);
    b._fast_fill(&mut drainer_b);
    assert_eq!(vec![1], drainer_b);
    assert_eq!(drainer_a, drainer_b);
    a.push(5);
    b.push(5);
    let mut drainer_a = Vec::with_capacity(4);
    let mut drainer_b = Vec::with_capacity(4);
    a.fill(&mut drainer_a);
    b._fast_fill(&mut drainer_b);
    assert_eq!(vec![2, 3, 4, 5], drainer_b);
    assert_eq!(drainer_a, drainer_b);
}

#[test]
fn fast_fill_test2() {
    let mut a = CircularBuffer::new(1);
    let mut b = CircularBuffer::new(1);
    a.push(1);
    b.push(1);
    let mut drainer_a = Vec::with_capacity(1);
    let mut drainer_b = Vec::with_capacity(1);
    a.fill(&mut drainer_a);
    b._fast_fill(&mut drainer_b);
    assert_eq!(vec![1], drainer_b);
    assert_eq!(drainer_a, drainer_b);
    a.push(2);
    b.push(2);
    let mut drainer_a = Vec::with_capacity(1);
    let mut drainer_b = Vec::with_capacity(1);
    a.fill(&mut drainer_a);
    b._fast_fill(&mut drainer_b);
    assert_eq!(vec![2], drainer_b);
    assert_eq!(drainer_a, drainer_b);
}

proptest! {
    #[test]
    fn the_len_of_the_buffer_is_always_between_0_and_the_max_requested(
            size in 1..25usize,
            v in proptest::collection::vec(0..1000u32, 0..1000)) {
        let mut b = CircularBuffer::<u32>::new(size);
        let r = 0..=size;
        for i in v {
            b.push(i);
            assert!(r.contains(&b.len()))
        }
    }

    #[test]
    fn keep_track_of_len(
        size in 1..100usize,
        // matrix => vector to add & element to remove
        matrix in proptest::collection::vec(
            (proptest::collection::vec(0..1000u32, 0..100), 0..100usize),
            0..100)
        ) {
        let mut b = CircularBuffer::<u32>::new(size);
        let r = 0..=size;
        let mut counted_len = 0;
        for (to_add, to_remove) in matrix {
            for i in to_add {
                b.push(i);
                counted_len = std::cmp::min(size, counted_len+1);
                assert_eq!(counted_len, b.len());
                assert!(r.contains(&counted_len))
            }
            let mut drainer = Vec::with_capacity(to_remove);
            let removed = b.fill(&mut drainer);
            assert_eq!(std::cmp::min(counted_len, to_remove), removed);
            counted_len = counted_len - removed;
            assert_eq!(counted_len, b.len());
            assert!(r.contains(&counted_len));
        }
    }

    #[test]
    fn keep_track_of_values(
        size in 1..100usize,
        matrix in proptest::collection::vec(
            (proptest::collection::vec(0..1000u32, 0..100), 0..100usize),
            0..100)
        ) {
        let mut b = CircularBuffer::<u32>::new(size);
        let mut v = Vec::new();
        for (to_add, to_remove) in matrix {
            for i in to_add {
                b.push(i);
                v.push(i)
            }
            while v.len() > b.len() {
                v.remove(0);
            }

            let mut v_drainer = Vec::with_capacity(to_remove);
            for _ in 0..std::cmp::min(to_remove, v.len()) {
                v_drainer.push(v.remove(0));
            }

            let mut drainer = Vec::with_capacity(to_remove);
            b.fill(&mut drainer);

            assert_eq!(drainer, v_drainer);
        }
    }

    #[test]
    fn fast_fill_vs_fill(
        size in 1..100usize,
        matrix in proptest::collection::vec(
            (proptest::collection::vec(0..1000u32, 0..100), 0..100usize),
            0..100)
        ) {
        let mut a = CircularBuffer::<u32>::new(size);
        let mut b = CircularBuffer::<u32>::new(size);
        for (to_add, to_remove) in matrix {
            for i in to_add {
                a.push(i);
                b.push(i);
            }


            let mut a_drainer = Vec::with_capacity(to_remove);
            let mut b_drainer = Vec::with_capacity(to_remove);
            a.fill(&mut a_drainer);
            b._fast_fill(&mut b_drainer);

            assert_eq!(a_drainer, b_drainer, "the left/first is correct");
        }
    }
}

#[test]
fn test_display() {
    let mut b = CircularBuffer::<u32>::new(4);
    assert_eq!("CircularBuffer(<empty>)", format!("{}", b));
    b.push(1);
    assert_eq!("CircularBuffer(1)", format!("{}", b));
    b.push(2);
    assert_eq!("CircularBuffer(1, 2)", format!("{}", b));
    b.push(3);
    assert_eq!("CircularBuffer(1, 2, 3)", format!("{}", b));
    b.push(4);
    assert_eq!("CircularBuffer(1, 2, 3, 4)", format!("{}", b));
    b.push(5);
    assert_eq!("CircularBuffer(2, 3, 4, 5)", format!("{}", b));

    b.fill(&mut Vec::with_capacity(2));
    assert_eq!("CircularBuffer(4, 5)", format!("{}", b));
    b.fill(&mut Vec::with_capacity(1));
    assert_eq!("CircularBuffer(5)", format!("{}", b));
    b.fill(&mut Vec::with_capacity(1));
    assert_eq!("CircularBuffer(<empty>)", format!("{}", b));
}

#[test]
fn copy_works_as_expected() {
    let mut b = CircularBuffer::new(5);

    for i in 0..10 {
        b.push(i);
    }

    let mut b_copy = b;
    let mut v1 = Vec::with_capacity(5);
    let mut v2 = Vec::with_capacity(5);

    b.fill(&mut v1);
    b_copy.fill(&mut v2);

    assert_eq!(v1, v2);
}

#[derive(Clone)]
struct Foo {
    a: String,
}

#[test]
fn test_clone() {
    let mut b = CircularBuffer::new(3);

    b.push(Foo {
        a: String::from("1"),
    });

    // this should not work, Foo does not implement Copy
    //let c = b;
    //b.push(Foo { a: String::new() });

    let c = b.clone();
    b.push(Foo {
        a: String::from("2"),
    });

    let b1: Vec<_> = b.collect();
    assert_eq!(b1[0].a, "1");
    assert_eq!(b1[1].a, "2");
    let c1: Vec<_> = c.collect();
    assert_eq!(c1[0].a, "1");
}
