//! # Solstack
//!
//! **Solstack** is a library that enables you to manage the control flow of your application through what's known as a *state stack machine*.
//!
//! This library provides a `Stack` type that holds a stack of `State`s. When the stack is ticked (a tick being an update/frame call) it executes the methods on the topmost state it holds. This means that only one state is run at a time; the one at the top.
//!
//!
//!
//! - You can read [The Book](https://solmateus.gitbook.io/solstack/). 
//!
//! - Take a look at the examples on the [git repository](https://github.com/solmateus/solstack). 
//!
//! - Search documentation at the [crate's docs](https://crates.io/crates/solstack).
//!
//!
//!
//! > *Project is in early development, things may change around! Take a look at the changelog before updating.*
//!
//!
//!
//! ## Features
//!
//! - A simple `State` trait to implement.
//! - An easy to use `Stack` State Machine.
//! - Multiple `Trans`itions between states. 
//! - Multiple state methods, such as `on_start`, `on_pause`, `on_tick`, etc.
//!
//!
//!
//! ## Use case
//!
//! When you are writing a game, for instance, you'll probably need a way of organizing the different states of the application as a whole.  When the application starts, you might have a simple `MenuState` on your stack, that asks the player for input. If the player clicks on "Play", you want to push another state on top of the stack, say, `GameState`. 
//!
//! Only the `State` on top of the stack has it's methods executed by the stack; this means that the `MenuState` is essentially paused. If the player wants to pause the game, inside your `GameState` there could be logic that says that on pressing `Esc` the stack should push a `PauseMenuState` on top of the stack. This won't delete the `GameState`, it will simply pause it, since now the stack will run only the topmost state `PauseMenuState`. 
//!
//! If the player wants to exit the game, you can pop everything from the stack, leaving it empty. If the player wants to get back to the game, you can simply pop the `PauseMenuState` out of the stack, resuming the one under it `GameState`.
//!
//! > For a more detailed usage example, see The Book (liked at the beginning of this page).
//!
//!
//!
//! ## Get started
//!
//! You can find a *very* simple example here. For more detailed examples, take a look at the `examples` folder on the project's github repo. For a complete tutorial, take a look at The Book.
//!
//! > Links are available at the start of this page.
//!
//! ```rust
//! use solstack::prelude::*;
//! use solstack::macros::*; // Easy abstractions over boilerplate-y code.
//!
//! #[derive(Default)]
//! struct GameData {
//!     value: i32
//! }
//!
//! struct AddOneAndPrintState;
//! impl State<GameData> for AddOneAndPrintState {
//!     fn on_start(&mut self, data: &mut GameData) {
//!         data.value = 10;
//!         println!("on_start `make data be 10` > GameData({})", data.value);
//!     }
//!  
//!     fn on_tick(&mut self, data: &mut GameData) -> Trans<GameData> {
//!         data.value += 1;
//!         println!("on_tick `add one to data` > GameData({})", data.value);
//!         Trans::None
//!     }
//! }
//!
//! fn main() {
//!     let mut data = GameData::default();
//!     let mut stack = Stack::<GameData>::new();
//!    
//!     stack_push!(stack, data, AddOneAndPrintState);
//!     stack_tick!(stack, data);
//!     stack_tick!(stack, data);
//! }
//! ```
//!
//!
//!
//! ## Thanks
//!
//! The documentation will always be updated.
//!
//! Thank you for using `solstack`!
//!
//! *By Sol* <solmateusbraga@gmail.com>

#![warn(clippy::pedantic)]
#![allow(clippy::needless_doctest_main)]

pub mod macros;
pub mod stack;
pub mod trans;
pub mod state;

pub mod prelude {

    pub use crate::stack::Stack;
    pub use crate::trans::Trans;
    pub use crate::state::State;
}
