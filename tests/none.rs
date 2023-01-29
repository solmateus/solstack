use solstack::{prelude::*, stack_tick};
use solstack::macros::trans_none;

// No macros
struct STest;
impl State<()> for STest {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        Trans::None
    }
}

#[test]
fn nonning() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));

    stack.tick(&mut ());
    stack.tick(&mut ());
    stack.tick(&mut ());
        
    assert_eq!(stack.len(), 3);
}

// With macros
struct STestMacro;
impl State<()> for STestMacro {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        trans_none!()
    }
}

#[test]
fn nonning_macro() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(STestMacro));
    stack.push(&mut (), Box::new(STestMacro));
    stack.push(&mut (), Box::new(STestMacro));

    stack_tick!(stack, ());
    stack_tick!(stack, ());
    stack_tick!(stack, ());

    assert_eq!(stack.len(), 3);
}

