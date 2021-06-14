use std::time;
use std::thread;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use address_space::{AddressSpace, Section};

use rand::prelude::*;



fn address_add_many(entry_count: u32) {
    let mut address_space = AddressSpace::new();


    let mut rng = StdRng::seed_from_u64(100); // rand::thread_rng();

    for i in 0..entry_count  {
        let addr: u32 = rng.gen(); 
        let data: u8 = rng.gen();
        address_space.update_byte(addr, data);
    }
}

fn address_add_range(entry_count: u32) {
    let mut address_space = AddressSpace::new();

    let mut rng = StdRng::seed_from_u64(100); // rand::thread_rng();

    for i in 0..entry_count  {
        let addr: u32 = rng.gen(); 
        let mut data: Vec<u8> = Vec::new();
        for j in 0..100 {
            data.push(rng.gen());
        }
        address_space.update(addr, &data);
    }
    
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("address many", |b| b.iter(|| address_add_many(black_box(1_000))));
    c.bench_function("address range", |b| b.iter(|| address_add_range(black_box(1_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
