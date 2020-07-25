use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use rbl_circular_buffer::*;

fn bench_filles(c: &mut Criterion) {
    let buffer_size = vec![1, 3, 8, 10, 30, 80, 100, 300, 800, 1000, 3000, 8000];
    let drain_elements = vec![1, 3, 8, 10, 30, 80, 100, 300, 800, 1000, 3000, 8000];
    let mut inputs = Vec::new();
    for bs in &buffer_size {
        for de in &drain_elements {
            if (*bs * 10) < *de {
                break;
            }
            inputs.push(vec![bs, de]);
        }
    }
    println!("{}", inputs.len());
    let mut group = c.benchmark_group("fills");
    for config in inputs {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("fast_fill {} - {}", config[0], config[1])),
            &config,
            |bencher, c| {
                let (size, drains) = (c[0], c[1]);
                let mut buffer = CircularBuffer::new(*size);
                for i in 0..*size {
                    buffer.push(i);
                }
                let mut drainer = Vec::with_capacity(*drains);
                bencher.iter(|| buffer.fast_fill(&mut drainer));
            },
        );

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("slow_fill {} - {}", config[0], config[1])),
            &config,
            |bencher, c| {
                let (size, drains) = (c[0], c[1]);
                let mut buffer = CircularBuffer::new(*size);
                for i in 0..*size {
                    buffer.push(i);
                }
                let mut drainer = Vec::with_capacity(*drains);
                bencher.iter(|| buffer.fill(&mut drainer));
            },
        );
    }
}

criterion_group!(benches, bench_filles);
criterion_main!(benches);
