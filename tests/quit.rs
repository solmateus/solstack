use solstack::{prelude::*, stack_tick};
use solstack::macros::{trans_quit, stack_quit};

// No macros
struct STest;
impl State<()> for STest {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        Trans::Quit
    }
}

#[test]
fn quitting() {
    let mut stack = Stack::new();

    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));

    assert_eq!(stack.len(), 3);

    stack.quit(&mut ());

    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));

    stack.tick(&mut ());

    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    assert!(!stack.is_running());
}

// With macros
struct STestMacro;
impl State<()> for STestMacro {
    fn on_tick(&mut self, _data: &mut ()) -> Trans<()> {
        trans_quit!()
    }
}

#[test]
fn quitting_macro() {
    let mut stack = Stack::new();
    
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));

    assert_eq!(stack.len(), 3);

    stack_quit!(stack, ());    

    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));
    stack.push(&mut (), Box::new(STest));

    stack_tick!(stack, ());

    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    assert!(!stack.is_running());
}
