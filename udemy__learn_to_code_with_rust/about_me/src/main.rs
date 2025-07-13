/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

// Akbarsha's comments:::
// Akbarsha's comments:::
// Akbarsha's comments:::
// Akbarsha's comments:::
// Akbarsha's comments:::

/*
* first - cargo fmt - format the files
* second - cargo check - check the code syntax and throws errors
* third - cargo build - build the project with executables
* fourth - cargo run - run the applciation/project
* 5 - cargo clean - removes all executables in the target folder
*/

/*
METHODS to run the rust project:
*/

/* METHOD - 1:
* else move into /src folder
* then, use 'rustc main.rs' to create a executable rust file
* then at the main.rs file location, use './main' to manually run the executable file
*
* METHOD - 2:- CARGO
* cargo build - only compile - parses the Rust source code and compiles it to the executable program that the computer runs
* cargo run - compile and run
*
* METHOD - 3:- DEBUG
* from the root folder location, run './target/debug/about_me'
*/

/*
* macro - println! -> takes the entire line - like a block element - h1, p
* macro - print! -> takes the content width only, won't break the line - like inline elements - span
*/

fn main() {
    print!("Hi, Akbarsha here!");
    println!("Software Engineer!");
    println!("From CHennai!");
}
