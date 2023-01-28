use crate::state::BoxState;

/// A transition between [`State`](crate::state::State)s.
///
/// See the crates docs or the book for more information.
/// ### Generics
/// - D: Data available to all [`State`](crate::state::State)s to perform actions to.
pub enum Trans<D> {

    /// Pushes a new state above the current one.
    /// Effectively pauses the current state until everything above it is
    /// popped.
    Push(BoxState<D>),

    /// Pops the current state from the stack.
    Pop,

    /// Pops and pushes a new state.
    /// Effectively replaces the current state with a new one.
    Replace(BoxState<D>),

    /// Pops every state from the stack and pushes a new one.
    /// Effectively isolates it as the only state on the stack.
    Isolate(BoxState<D>),
    
    /// Pops everything from the stack.
    /// Effectively ends the state stack machine.
    Quit,

    /// Does nothing to the stack.
    /// Effectively keeps the current state and the stack the way it is.
    None,
}

