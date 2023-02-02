use solstack::macros::{stack_pop, trans_pop};
use solstack::{prelude::*, stack_push};

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

    stack_push!(
        stack,
        (),
        SPopMacro,
        SPopMacro,
        SPopMacro,
        SPopMacro,
        SPopMacro,
        SPopMacro
    );

    assert_eq!(stack.len(), 6);

    stack_pop!(stack, ());

    assert_eq!(stack.len(), 5);

    stack_pop!(stack, (), 3);

    assert_eq!(stack.len(), 2);
}
