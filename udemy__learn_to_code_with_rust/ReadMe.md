# Rust - Lear to code with Rust

Boris Paskhavar, New York

Rust -> System's programming language

System's rogramming -> refers to a category of languages that are optimized for situations where resources (memory and CPU) are limited

- Rust lang. design enables the speed of those lang. with added satbility and safety
- Build safe, fast and efficinet programs

## Rust Compiler

- translates the soucre code into an executable program (binary/binary executable)

## Syntax:

- Rust is among the strictest of them all. Every detils matters to the complier.

- case sensitivity matters
- space amtters
- symbols matters
- line breaks matter

## INstalling Rust on MacOS:

- visit rust-lang.org

- Using rustup (Recommended)
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Updating / Uninstallaing the rust

- rustup update
- rustup self uninstall

## Documentation

- rustup doc
- opens rust documentation file in the browser

# Create Rust project:

## Cargo:

- a command line tool that helps manage rust projects
- to create projects, to compile the executable programs from the source code
- recommended way to create a new rust project.

  - give nice starter template
  - enforces consistency across project

- project
- package
- crate

## create a rust project:

- cargo new project_name
  - snake_case are prefered

Output:
➜ udemy\_\_learn_to_code_with_rust git:(dev) ✗ cargo new hello_world
Creating binary (application) `hello_world` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

## Binary crate vs Rust packages/crates/library crates

- binary crate:

  - a standaone rust application whose purpose is to be run bu itself in isolation

- ## Rust packages/crates/library crates:
  - one that exists to be used in other projects
