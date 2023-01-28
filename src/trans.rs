use crate::state::State;

/// A transition between [`State`](crate::state::State)s.
///
/// See the crates docs or the book for more information.
/// ### Generics
/// - D: Data available to all [`State`](crate::state::State)s to perform actions to.
pub enum Trans<D> {

    /// Pushes a new [`State`](crate::state::State) above the current one.
    /// Effectively pauses the current [`State`](crate::state::State) until everything above it is
    /// popped.
    Push(Box<dyn State<D>>),

    /// Pops the current [`State`](crate::state::State) from the stack.
    Pop,

    /// Pops everything from the stack.
    /// Effectively ends the state stack machine.
    Quit,

    /// Does nothing to the stack.
    /// Effectively keeps the current [`State`](crate::state::State).
    None,
}

