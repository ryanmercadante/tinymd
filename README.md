# tinymd

## Table of Contents

- [General Info](#general-info)
- [Technologies](#technologies)
- [Setup](#setup)
- [How to Use](#how-to-use)
- [Features](#features)
- [Inspiration](#inspiration)
- [Contact](#contact)

## General Info

Tinymd is a markdown compiler built in Rust. It allows you to run `tinymd <your_file.md>` to compile one markdown file at a time into an HTML file. It was bootstrapped with Cargo. Original project by Jesse Lawson, see [Inspiration](#inspiration) for more details.


## Technologies

- [Rust](https://www.rust-lang.org/) - The Rust programming language.
- [Cargo](https://doc.rust-lang.org/stable/cargo/) - The Rust package manager.
- [Github Actions](https://github.com/features/actions) - CI/CD tool to build, test, and deploy code right from GitHub.

## Setup

To run this project, clone it from github:

```bash
$ git clone https://github.com/ryanmercadante/tinymd.git
$ cd tinymd/
$ cargo run `<your_file.md>`
$ cargo build 
$ cargo build --release
```

## How to Use

Currently, the production version of this compiler can only be installed through Homebrew.

```bash
$ brew tap ryanmercadante/tinymd
$ brew install tinymd
$ tinymd <your_file.md>
```

## Features

### Done

- Compile `# Heading 1` to `<h1>Heading 1</h1>`
- Compile `## Heading 2` to `<h2>Heading 2</h2>`
- Compile `text` to `<p>text</p>`
- Compile `*emphasis*` to `<em>emphasis</em>`

### To Do:

- Add remainging heading tags
- Add strong tag
- Add lists
- Add more advanced tags

## Inspiration

First, I followed this excellent [tutorial](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/) by Jesse Lawson to get started. I then continued the project by adding additional tags to compile to, such as em and h2 HTML tags. I plan to continue adding more functionality to this markdown compiler.

## Contact

- [LinkedIn](https://www.linkedin.com/in/ryan-mercadante-11a035152/)
- [Github](https://www.github.com/ryanmercadante)
- [Twitter](https://twitter.com/RyanMercadante)
- [Email](mailto:ryan.a.mercadante@gmail.com)
