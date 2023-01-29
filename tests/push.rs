use solstack::{prelude::*, stack_tick};
use solstack::macros::{stack_push, trans_push};

// No macros
struct SPush;
impl State<()> for SPush {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        Trans::Push(Box::new(SPush))
    }
}

#[test]
fn pushing() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(SPush));
    assert_eq!(stack.len(), 1);

    stack.tick(&mut ());
    assert_eq!(stack.len(), 2);

    stack.tick(&mut ());
    assert_eq!(stack.len(), 3);
}

// With macros
struct SPushMacro;
impl State<()> for SPushMacro {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        trans_push!(SPushMacro)
    }
}

#[test]
fn pushing_macro() {
    let mut stack = Stack::new();

    stack_push!(stack, (), SPushMacro);
    assert_eq!(stack.len(), 1);

    stack_tick!(stack, ());
    assert_eq!(stack.len(), 2);

    stack_tick!(stack, ());
    assert_eq!(stack.len(), 3);

    stack_push!(stack, (), SPushMacro, SPushMacro, SPushMacro);
    assert_eq!(stack.len(), 6);

    stack_tick!(stack, ());
    assert_eq!(stack.len(), 7);
}