use crate::state::BoxState;
use crate::trans::Trans;

/// Handles the control flow of a stack of [`State`](crate::state::States).
/// Essentialy the `State Stack Machine`.
///
/// See the crates docs or the book for more information.
/// ### Generics
/// - D: Data available to all [`State`s](crate::state::State) to perform writes and reads to.
#[derive(Default)]
pub struct Stack<D> {
    stack: Vec<BoxState<D>>,
}

// Constructors
impl<D> Stack<D> {
    /// New empty stack state machine; a.k.a. [`Stack`].
    #[must_use]
    pub fn new() -> Self {
        Self { stack: vec![] }
    }
}

// Utilities
impl<D> Stack<D> {
    /// Checks if there is at least one state on the stack.
    #[must_use]
    pub fn is_running(&self) -> bool {
        !self.stack.is_empty()
    }

    /// Gets the length of the stack.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Checks if the stack is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

// Pure transitions.
impl<D> Stack<D> {
    /// Pushes a new state above the current one.
    /// Effectively pauses the current state until everything above it is
    /// popped.
    pub fn push(&mut self, data: &mut D, mut state: BoxState<D>) {
        // Run current `State`'s `.on_pause()`.
        if let Some(curr_state) = self.stack.last_mut() {
            curr_state.on_pause(data);
        }

        // Starts the new state.
        state.on_start(data);
        // Pushes the new state onto the stack.
        self.stack.push(state);
    }

    /// Pops the current [`State`](crate::state::State) from the stack.
    pub fn pop(&mut self, data: &mut D) {
        // Runs this `State`'s `.on_stop()`.
        // Also pops it from the stack (rhs of if-let).
        if let Some(mut curr_state) = self.stack.pop() {
            curr_state.on_stop(data);
        }

        // Runs the `State` below's `.on_resume()`.
        if let Some(state) = self.stack.last_mut() {
            state.on_resume(data);
        }
    }

    /// Pops everything from the stack.
    /// Effectively ends the state stack machine.
    pub fn quit(&mut self, data: &mut D) {
        // Pops everything out of the stack.
        // Also calls `.on_stop()` for all of the stack (rhs of while-let).
        while let Some(mut state) = self.stack.pop() {
            state.on_stop(data);
        }
    }
}

// Composed transitions.
impl<D> Stack<D> {
    /// Pops and pushes a new state.
    /// Effectively replaces the current state with a new one.
    #[inline]
    pub fn replace(&mut self, data: &mut D, state: BoxState<D>) {
        self.pop(data);
        self.push(data, state);
    }

    /// Pops every state from the stack and pushes a new one.
    /// Effectively isolates it as the only state on the stack.
    #[inline]
    pub fn isolate(&mut self, data: &mut D, state: BoxState<D>) {
        self.quit(data);
        self.push(data, state);
    }
}

impl<D> Stack<D> {
    #[inline]
    fn transition(&mut self, data: &mut D, trans: Trans<D>) {
        match trans {
            Trans::Push(state) => self.push(data, state),
            Trans::Pop => self.pop(data),
            Trans::Replace(state) => self.replace(data, state),
            Trans::Isolate(state) => self.isolate(data, state),
            Trans::None => (),
            Trans::Quit => self.quit(data),
        };
    }

    /// Ticks the current (topmost at the stack) state once.
    pub fn tick(&mut self, data: &mut D) {
        // Initializing an empty vector of transitions.
        let mut transitions: Vec<Trans<D>> = vec![];

        // Looping through every state.
        // Performing `on_shadow_tick` on every state.
        // Storing the transitions returned on the `transitions` vector.
        self.stack.iter_mut().for_each(|state| {
            transitions.push(state.on_shadow_tick(data))
        });

        // Getting the topmost state at the sack.
        // Ticking that state.
        // Storing the transition returned on the `transitions` vector.
        if let Some(state) = self.stack.last_mut() {
            transitions.push(state.on_tick(data));
        }

        // Performing the transitions stored in the `transitions` vector.
        transitions.into_iter().for_each(|trans| {
            self.transition(data, trans);
        });
    }
}

