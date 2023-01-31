//! # Solstack
//!
//! **Solstack** is a library that enables the management of an application's control flow through a **state stack machine**.
//!
//! A `Stack` `struct` holds any number of `State`s. When said `Stack` is ticked, it executes the main methods of the **topmost `State`**. This means that only the `State` at the top of the `Stack` is executed, while the ones below it are effectively **paused**. An exception are the `State`'s `on_shadow_tick` method introduced in `v0.3.0` which is always executed independently of the `State`'s position on the `Stack`.
//!
//! - You can read [The Book](https://solmateus.gitbook.io/solstack/). 
//! - Take a look at the examples on the [git repository](https://github.com/solmateus/solstack). 
//! - Search documentation at the [crate's docs](https://crates.io/crates/solstack).
//! - Watch the project develop through its [changelog](https://github.com/solmateus/solstack/blob/main/CHANGELOG.md)!
//!
//!
//! > *Project is in early development, things may change around! Take a look at the changelog before updating. Some text may imply that things are done, when they're not, like The Book. There may be bugs, though the tests should guarantee there aren't many.*
//!
//!
//! ## Overview
//!
//! `State`s have all of the methods you'll need. Here's an overview:
//!
//! * `on_start` — executed when this `State` is first pushed onto the `Stack`.
//! * `on_stop` — executed when this `State` is popped from the `Stack`.
//! * `on_pause` — executed when another `State` is pushed on top of this one in the `Stack`.
//! * `on_resume` — executed when this all of the `State`s above this one are popped from the `Stack`.
//! * `on_tick` — executed every time the `Stack` holding this `State` has it's `tick` method called.
//! * `on_shadow_tick` — same as `on_tick`, but is always run independently of this `State`'s position in the `Stack`.
//!
//! To hop between `State`s you'll return a `Trans`ition `enum` from `on_tick` or similar methods, requesting the `Stack` to perform such transition next tick. Here's an overview:
//!
//! * `Trans::None` — requests the `Stack` to do nothing.
//! * `Trans::Quit` — requests the `Stack` to `Pop` every `State` it is holding.
//! * `Trans::Push(Box::new(State))` — requests the `Stack` to `Push` the provided `State` on top.
//! * `Trans::Pop` — requests the `Stack` to pop the topmost `State` it is holding, deleting it.
//! * `Trans::Replace(Box::new(State))` — requests the `Stack` to `Pop` once and `Push` the provided `State`.
//! * `Trans::Isolate(Box::new(State))` — requests the `Stack` to `Pop` everything and `Push` the provided `State`.
//!
//! > Transitions may be requested directly of the `Stack` or by returning a `Trans` from inside a `on_tick` or `on_shadow_tick` method of a `State`.
//!
//!
//! ## Features
//!
//! - An easy to implement `State` `trait`.
//! - An easy to use **State `Stack` Machine** `struct`.
//! - An easy to use set of **`Trans`itions between `States`**.
//!
//! - New in `v0.3.0`:
//!   - `on_shadow_tick` is provided on `State`s; similar to `on_tick` but *is always executed independently of the `State`'s position* in the `Stack`.
//!
//!
//! ## Use case
//!
//! Imagine you're writing a game. You need a way of controlling the flow of your program. From the main menu to the game itself; from the game to the pause menu; or from the pause menu to quitting the program. 
//!
//! ***Solstack** will help you structure these `State`s and the  `Trans`itions between them*. 
//!
//!
//! Let's model something. Here are our `State`s (prefixed with `S`):
//!
//! - `SMainMenu`: where the player lands on initializing the game.
//! - `SGame`: where the actual gameplay logic resides.
//! - `SPauseMenu`: where the player can save, resume playing or exit.
//!
//!
//! Since the `SMainMenu` is the first thing the user will encounter, we'll manually `Push` that `State` on our `Stack` when the program begins. Then we'll tick the `Stack` in a main loop until there are no `State`s inside it anymore.
//!
//! > You can manually perform transitions on a `Stack` by using it's methods on your local instance.
//! >
//! > The loop can be achieved by using the `Stack`'s `is_running` method.
//!
//!  The `Stack`, at the beginning of our program, will look like this: 
//!
//! 1.  `SMainMenu`
//!
//!
//! With only one sate on the `Stack`, it is on top; and so it will have it's methods called. `SMainMenu`'s logic is simple: when the player presses *START*, it requests the `Stack` to `Push` an `SGame`. If that happens, the `Stack` would look like this:
//!
//! 1. `SGame`
//! 2. `SMainMenu`
//!
//!
//! Since only the topmost state is run by the stack's `tick`, `SMainMenu` just sits there. Now the player is enjoying their game; but they wish to pause! Well, inside `SGame` all we have to do is request the stack to `Push` an `SPauseMenu` if the player ever presses *ESC*. Simple! Let's see the `Stack` again:
//!
//! 1. `SPauseMenu`
//! 2. `SGame`
//! 3. `SMainMenu`
//!
//!
//! Now `SPauseMenu` is at the top. `SGame` will be **paused**; it's still there, but it is not being executed. Inside the `SPauseMenu` there should be logic saying that if the player presses *ESC* again, the `Stack` should `Pop`. `Pop`ping means removing or completely deleting the topmost `State` at the `Stack`. In this case, `SPauseMenu` itself. The `Stack` would then, again, look like this:
//!
//! 1. `SGame`
//! 2. `SMainMenu`
//!
//!
//! Finally, `SGame` is at the top again! And so, it will resume **exactly** where it left off! If the player chooses to **quit the game**, you simply request the `Stack` to `Quit`, which will `Pop` every `State` it has, making the main loop end.
//!
//!
//! This concept can be extended to much bigger patterns; and hopefully you'll find joy in structuring your application with `solstack`!
//!
//!
//! ## Get started
//!
//! This is just a **very simplistic** example in code. Take a look at the links at the start of this page, specially the examples on the project's repository. The tests are also a good way of seeing how the library works internally.
//!
//! ```rust
//! use solstack::prelude::*;
//! use solstack::macros::*; // easy abstractions over boilerplate-y code.
//!
//! // data available to `State`s for writing and reading
//! #[derive(Default)]
//! struct GameData {
//!     value: i32
//! }
//!
//! // a `State` that does what it says
//! struct AddOneAndPrintState;
//! impl State<GameData> for AddOneAndPrintState {
//!     // run when this `State` is first pushed onto a `Stack`
//!     fn on_start(&mut self, data: &mut GameData) {
//!         data.value = 41;
//!         println!("on_start `make data be 41` > GameData({})", data.value);
//!     }
//!
//!     // run every time the `Stack` is ticked.
//!     fn on_tick(&mut self, data: &mut GameData) -> Trans<GameData> {
//!         data.value += 1;
//!         println!("on_tick `add one to data` > GameData({})", data.value);
//!         Trans::None
//!     }
//! }
//!
//! fn main() {
//!     // initializing
//!     let mut data = GameData::default();
//!     let mut stack = Stack::<GameData>::new();
//!    
//!     assert_eq!(data.value, 0);
//!    
//!     // manually pushing and ticking the `Stack`
//!    
//!     stack_push!(stack, data, AddOneAndPrintState);
//!     assert_eq!(data.value, 41);
//!    
//!     stack_tick!(stack, data);
//!     assert_eq!(data.value, 42);
//!    
//!     stack_tick!(stack, data);
//!     assert_eq!(data.value, 43);
//!    
//! }
//! ```
//!
//!
//! ## Thanks
//!
//! > The documentation will always be up to date.
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
