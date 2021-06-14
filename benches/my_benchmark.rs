use std::time;
use std::thread;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use address_space::{AddressSpace, Section};

// fn fibonacci(n: u64) -> u64 {
//     match n {
// 	0 => 1,
// 	1 => 1,
// 	n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

fn fibonacci(n: u64) -> u64 {
    let mut p = FibStorage::new();
    p.compute(n)
}

struct FibStorage {
    data: Vec<u64>
}

impl FibStorage {
    pub fn new() -> Self {
	let mut p = Vec::new();
	p.push(1);
	p.push(1);
	Self {
	    data: p
	}
    }

    pub fn compute(&mut self, n: u64) -> u64 {
	if (self.data.len() as u64) <= n {
	    let k = self.compute(n-1);
	    let k2 = self.compute(n-2);
	    self.data.push(k + k2);

	}
	self.data[n as usize]
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("fib 21", |b| b.iter(|| fibonacci(black_box(21))));
    c.bench_function("fib 22", |b| b.iter(|| fibonacci(black_box(22))));    

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
