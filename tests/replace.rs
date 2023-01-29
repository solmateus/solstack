use solstack::{prelude::*, stack_tick, stack_push};
use solstack::macros::{stack_replace, trans_replace};

// TODO: test multiple argument use of `stack_replace!`.

// No macros
type SData = i32;

struct SDummy1;
impl State<SData> for SDummy1 {
    fn on_tick(&mut self, data: &mut SData) -> Trans<SData> {
        *data = 1;
        Trans::Replace(Box::new(SDummy2))
    }
}

struct SDummy2;
impl State<SData> for SDummy2 {
    fn on_tick(&mut self, data: &mut SData) -> Trans<SData> {
        *data = 2;
        Trans::Replace(Box::new(SDummy1))
    }
}

struct SDummy3;
impl State<SData> for SDummy3 {
    fn on_tick(&mut self, data: &mut SData) -> Trans<SData> {
        *data = 3;
        Trans::None
    }
}

#[test]
fn replacing() {
    let mut stack = Stack::new();
    let mut data = 0;

    stack.push(&mut data, Box::new(SDummy1));

    assert_eq!(data, 0);

    stack.tick(&mut data);

    assert_eq!(data, 1);

    stack.tick(&mut data);

    assert_eq!(stack.len(), 1);

    assert_eq!(data, 2);

    stack.replace(&mut data, Box::new(SDummy3));
    stack.replace(&mut data, Box::new(SDummy3));
    stack.replace(&mut data, Box::new(SDummy3));

    stack.tick(&mut data);

    assert_eq!(data, 3);
    assert_eq!(stack.len(), 1);
}

// With macro
type SDataMacro = i32;

struct SDummy1Macro;
impl State<SData> for SDummy1Macro {
    fn on_tick(&mut self, data: &mut SDataMacro) -> Trans<SDataMacro> {
        *data = 1;
        trans_replace!(SDummy2Macro)
    }
}

struct SDummy2Macro;
impl State<SDataMacro> for SDummy2Macro {
    fn on_tick(&mut self, data: &mut SDataMacro) -> Trans<SDataMacro> {
        *data = 2;
        trans_replace!(SDummy1Macro)
    }
}

struct SDummy3Macro;
impl State<SDataMacro> for SDummy3Macro {
    fn on_tick(&mut self, data: &mut SDataMacro) -> Trans<SDataMacro> {
        *data = 3;
        Trans::None
    }
}

#[test]
fn replacing_macro() {
    let mut stack = Stack::new();
    let mut data = 0;

    stack_push!(stack, data, SDummy1Macro);

    assert_eq!(data, 0);

    stack_tick!(stack, data);

    assert_eq!(data, 1);

    stack_tick!(stack, data);

    assert_eq!(stack.len(), 1);

    assert_eq!(data, 2);

    stack_replace!(stack, data, SDummy3Macro);
    stack_replace!(stack, data, SDummy3Macro);
    stack_replace!(stack, data, SDummy3Macro);

    stack_tick!(stack, data);

    assert_eq!(data, 3);
    assert_eq!(stack.len(), 1);
}
