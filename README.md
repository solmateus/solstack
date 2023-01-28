**Solstack** is a library that enables you to manage the control flow of your application through what's known as a *state stack machine*.
This library provides a `Stack` type that holds a stack of `State`s. When the stack is ticked (a tick being an update/frame call) it executes the methods on the topmost state it holds. This means that only one state is run at a time; the one at the top.

You can read [The Book](https://solmateus.gitbook.io/solstack/). 
Or maybe take a look at the examples on the [git repository](https://github.com/solmateus/solstack). 
And you can search documentation at the [crate's docs](https://crates.io/crates/solstack).

Project is in early development, things may change around!
The documentation will always be updated.

*By Sol*
