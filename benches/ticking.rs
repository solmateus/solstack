use criterion::{criterion_group, criterion_main, Criterion};

use solstack::{prelude::*, stack_push};

struct MakeItA;
struct MakeItB;
struct Dummy;

impl State<String> for MakeItA {
    fn on_tick(&mut self, data: &mut String) -> Trans<String> {
        *data = "A".to_owned();
        Trans::Replace(Box::new(MakeItB))
    }
}

impl State<String> for MakeItB {
    fn on_tick(&mut self, data: &mut String) -> Trans<String> {
        *data = "B".to_owned();
        Trans::Replace(Box::new(MakeItA))
    }
}

impl State<String> for Dummy {}

pub fn tick(c: &mut Criterion) {
    let mut data = String::new();
    let mut stack = Stack::<String>::new();

    c.bench_function("tick", |b| {
        b.iter(|| {
            stack_push!(stack, data, Dummy);
            stack.tick(&mut data)
        })
    });
}

criterion_group!(benches, tick);
criterion_main!(benches);
