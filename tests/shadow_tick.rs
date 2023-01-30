#![allow(unused)]
use solstack::{prelude::*, stack_push, trans_none};
use solstack::macros::*;

struct SDummy1;
struct SDummy2;
struct SDummy3;

#[derive(Default)]
struct Data {
    number: i32,
}

impl State<Data> for SDummy1 {
    fn on_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = -1;
        trans_none!()
    }

    fn on_shadow_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = 1;
        trans_none!()
    }
}
impl State<Data> for SDummy2 {
    fn on_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = -1;
        trans_none!()
    }

    fn on_shadow_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = 2;
        trans_none!()
    }
}
impl State<Data> for SDummy3 {
    fn on_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = -1;
        trans_none!()
    }

    fn on_shadow_tick(&mut self, data: &mut Data) -> Trans<Data> {
        data.number = 3;
        trans_none!()
    }
}

fn main() {
    let mut stack = Stack::new();
    let mut data = Data::default();

    stack_push!(stack, data, SDummy1, SDummy2, SDummy3);

    stack_tick!(stack, data);

    assert_eq!(data.number, 3);
}

