# Introduction to Notes

I'm a note-taker and it's how I learn. I also like writing documentation in
Markdown format, which is the format in which most documentation is written.
Finally, I want to keep a log of concepts as I encounter them so as to help
me describe to the Rust community where the book (currently v1.1.0) could
stand to be improved so as to be more accessible to absolute beginners like me.

These notes won't always be helpful to you, but I hope that if you're reading,
you do find more info than mere regurgitations of the source material!

# Introduction to Rust

Developed by teams at Mozilla Research for [Servo](https://servo.org/), Rust
is a low-level systems language with no garbage collector and three main goals:

 1. Safety
 2. Speed
 3. Concurrency

Safety is achieved through the wizardy of _static analysis_ which allows for
checks to be performed at compile-time so as not to impact its runtime. Without
the overhead of this or a garbage collector, the language can excel in low
resource environments like embedded devices, as well as in performance-critical
environments like operating systems.

A side-goal of the language is to provide some of the luxuries of a higher
level language without taking performance hits in order to do so.

# Contributing to _The Book_

_The Rust Programming Language_ is the comprehensive resource for learning
Rust. Also known as _The Book_, it's an open-source project on
[GitHub](https://github.com/rust-lang/rust/tree/master/src/doc/book) to which
pull requests can be made.

Additional sources of documentation include:

 - [The Rust Reference](https://doc.rust-lang.org/reference.html): An often
    out-of-date attempt to describe Rust's behavior, despite lacking a standard
    specification.

 - [Standard Library API Reference](https://doc.rust-lang.org/std): A reference
    for the Rust Standard Library.

 - [The Rustnomicon](https://doc.rust-lang.org/nomicon): Advanced literature on
    writing unsafe Rust code.

 - [Compiler Error Index](https://doc.rust-lang.org/error-index.html): An index
    describing the errors from the compiler in-depth.
