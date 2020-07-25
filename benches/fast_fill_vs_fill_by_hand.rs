use rbl_circular_buffer::*;

fn main() {
    let mut buffer = CircularBuffer::new(10240);

    for i in 0..10240 {
        buffer.push(i);
    }

    let mut d = Vec::with_capacity(570);
    let mut t1 = buffer;

    let mut t = std::time::Duration::new(0, 0);
    for _ in 0..10_000 {
        let now = std::time::Instant::now();
        t1.fast_fill(&mut d);
        let d = now.elapsed();
        t += d;
        for i in 10..600 {
            t1.push(i);
        }
    }

    println!("{:?}", t);
    let mut buffer = CircularBuffer::new(10240);

    for i in 0..10240 {
        buffer.push(i);
    }

    let mut d = Vec::with_capacity(570);
    let mut t1 = buffer;

    let mut t = std::time::Duration::new(0, 0);
    for _ in 0..10_000 {
        let now = std::time::Instant::now();
        t1.fill(&mut d);
        let d = now.elapsed();
        t += d;
        for i in 10..600 {
            t1.push(i);
        }
    }
    println!("{:?}", t);
}
