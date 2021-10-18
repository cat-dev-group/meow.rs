# reference

This book is the primary reference for the Meow programming language.

This document is not normative. It may include details specific to `meowc`, and
should not be taken as a specification for the Meow language. We intend to
produce such a document someday, but this is what we have for now.

## Dependencies

- rustc (the Rust compiler)
- mdbook (use `cargo install mdbook` to install it)

## Build steps

To build the reference, follow the steps given below:

Clone the project by running the following command:

```
git clone https://github.com/cat-dev-group/meow
```

Change the directory to the reference folder:

```sh
cd meow/reference
```

To generate a local instance of the book, run:

```sh
mdbook build
```

The generated HTML will be in the `book` folder.

For local writing, it may be easiest to have `mdbook` watch for live changes.
To see this changes on http://localhost:3000, run:

```sh
mdbook serve
```
