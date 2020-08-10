# Piscine Rust

Force them to think in another way, producing stronger programs.

#### Note

Exercises will always be run with the `RUSTFLAGS` env variable set to `--forbid unsafe_code`. Programs must pass the `cargo test` command and **after** will be executed using `cargo run -- [args]...`.

The `Cargo.lock` file will be pushed _only_ if the exercise asks for an executable, the `cargo new --bin/--lib` should do the job automagically.



### Day00

Introduce everything related to compilation and make them read the documentation of the standard library, read and read more, everything they can find, they must read it.

Install [`rustup`](https://rustup.rs/) and make them create there first project using [`cargo`](https://doc.rust-lang.org/stable/cargo/). Every exercise will be created using the following command: `cargo new --bin ex00`.

Create a simple hello world program, another program that uses simple argument parsing (e.g. alpha to int functions) and correct error handling.

Make them create their first libraries using `cargo new --lib ex07`, these libraries will contain functions with some specifc signatures like `fn average(numbers: &[i32]) -> Option<f64>` and must be runnable by a binary accepting arguments.



### Day01

Use the [Iterator](https://doc.rust-lang.org/std/iter) trait, [closure](https://doc.rust-lang.org/book/second-edition/ch13-01-closures.html) and [pattern matching](https://doc.rust-lang.org/book/second-edition/ch18-03-pattern-syntax.html) (i.e `match`) on simple types like numbers.

Implement programs:

- a FizzBuzz
- a recursive and an iterative Fibonacci function
- a program that count the number of times a character is present in a string
- a "replace in string" program (i.e. take a string in parameter contains `X` and replace every occurences by a word)
- a NOTE program (i.e. read the standard input and append to a file named "NOTES.txt")



### Day02

Create libraries, use them in their programs, make them use the [`Cargo.toml`](https://doc.rust-lang.org/stable/cargo/guide/cargo-toml-vs-cargo-lock.html#cargotoml-vs-cargolock) file (i.e. `version/git/path = "…"`). Create tests for their library using the `#[test]` attribute.

Each exercise contains the libraries of the previous exercise and a new library. Each contains a main binary which shows the features of the libraries.

- First exercise contains a library named `ex00` with a function that just uppercase a string.
- Second contains a function that replaces each `X` in a string by the given word and the previous library.
- Third contains a function that ...



### Day03

Create a `Stack` library which uses the principle of wrapping. The `Stack` type is a simple `Vec` but with less fonctionnality, the internal [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) is private. Make them pass some tests all along the way they create it, using test driven developpement.

All their libraries must be properly documented, we will ensure that using (i.e. `--forbid missing_docs`). The documentation will be generated and opened using `cargo doc --open`. Each function or struct must have an _Examples_ section showing possible usages.

The library will be tested using `cargo test` (documentation code parts are also tested).



### Day04

Create a simple Reverse Polish Notation calculator with syntax parsing. This program must use the `Stack` and a custom parser, made by hand. Introduce them to the [`io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait, this will make their parser use anything that can be read from (e.g. a file, the standard input, network).



### Rush00

Implement a `yasl` interpreter or something more simple. Use their `Stack` and a custom parser which accepts anything implementing `io::Read`.



### Day05

Use the [`std/net`](https://doc.rust-lang.org/stable/std/net/) module. They will be able to create a program that listen for new clients. Each message that a client send must be returned to it. A simple echo server in other words. Messages are separated by newlines, this will force them to use the [`BufRead::read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line) trait method.

Once this has been done, make them accept only some commands of the form `command: arg1, arg2, arg3`. Commands could be `echo`, `sum`, `div`, `add`... Arguments could be strings or integers...

They should have created an enum which can be constructed using an `&str`, returning a `Result<Command, String>`. The error must be returned to the client under the form `e: error message`.



### Day06

Make them use an external library called [`rand`](https://docs.rs/rand), generate a bunch of random integers and compute the sum of them all. The random number generator must be seeded (i.e. using `42` for example).

Once this is done show them that the code could be compiled using the `--release` flag which will make their program execution faster.

And the last step here is to make them use multithreading and show them that this is hard to make mistakes in Rust, especially when using [`rayon`](https://docs.rs/rayon). (c.f. [Powered by Rayon](https://old.reddit.com/r/rust/comments/9hwqt9/powered_by_rayon/?ref=share&ref_source=link))



### Day07

Show us that traits are powerfull by making then implement some sort of `IteratorExt` trait.

This trait must have some useful methods, functions signatures will be imposed and iterators must be fuzed (i.e. must not return `Some` if a previous call returned `None`).

- `fn denumerate(self, index: usize) -> Denumerate<Self>`: returns the index associated with the element (i.e. `(12, Self::Item)`) but decrementing from the `index` given and the iterator must stop when it touch zero, zero is returned.

- `fn min_max(self) -> Option<(Self::Item, Self::Item)>`: returns the `max` and the `min` values of it. Note that you will need to specify that `Self::Item` must be `PartialOrd`.
- `fn intersperse(self, value: Self::Item) -> Intersperse<Self>`: returns an iterator that will alternate an iterator value and the given value. Note that `Self::Item` must be `Clone`.
- `fn chunks(self, size: usize) -> Chunks<Self>`: returns an iterator which will yield a `Vec` at each iteration, the `Vec` will contain at most `size` elements.



 ### Day08

Introdution to combinations/chaining.

Little funny exercises where you force the students to not use the `match` or the `if` keywords. Force them to only use `Option` and `Result` methods chaining to achieve the goals. It will make them read and read again the methods documentation of these types.

Same thing with the loop keywords (i.e. `for`, `while`, `loop`), it becomes forbidden for the exercises. Force them to use the `Iterator` methods chaining.

Obiously all the previous exercices must not contain the `return` keyword, even for error handling. Note that the [`Try` operator trait](https://doc.rust-lang.org/std/ops/trait.Try.html) can be used (i.e. the `?` syntax).



### Day09

Make them use external libraries like `structopt`, `serde::json/toml`.

- construct a program that will take a file path in first argument, if the content could be parsed in `serde_json::Value` so display it in json using the pretty printer.
- same program but if it is not parseable as json, try to parse it in `toml::Value` and display it in toml using the pretty printer.
- Same program but with a `--display-format` argument that can be either `json` or `toml` that will be used to specify the output format to use. Using the `structopt` library is mandatory.
- Add a new argument `--replace X,Y` that will replace the value corresponding to `X` by the value `Y` and continue to display the content prettily.



### Rush01

Could be really cool to show that Rust can do physics too. Using [the testbed usage tutorial](https://nphysics.org/nphysics_testbed/) and make them love it with [this demo page](https://nphysics.org/demo_body_status2/).