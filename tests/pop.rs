use solstack::prelude::*;
use solstack::macros::{stack_pop, trans_pop};

// No macros
struct SPop;
impl State<()> for SPop {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        Trans::Pop
    }
}

#[test]
fn popping() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(SPop));

    assert_eq!(stack.len(), 1);

    stack.pop(&mut ());

    assert_eq!(stack.len(), 0);
}

// With macros
struct SPopMacro;
impl State<()> for SPopMacro {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        trans_pop!()
    }
}

#[test]
fn popping_macro() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(SPopMacro));
    stack.push(&mut (), Box::new(SPopMacro));
    stack.push(&mut (), Box::new(SPopMacro));
    stack.push(&mut (), Box::new(SPopMacro));
    stack.push(&mut (), Box::new(SPopMacro));
    stack.push(&mut (), Box::new(SPopMacro));

    assert_eq!(stack.len(), 6);

    stack_pop!(stack, ());

    assert_eq!(stack.len(), 5);

    stack_pop!(stack, (), 3);

    assert_eq!(stack.len(), 2);
}