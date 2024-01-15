# grrs

This app is following this tutorial so I can better understand building CLI apps with Rust: https://rust-cli.github.io/book/index.html

In short it searches a file for a given string of text. For instance this will search the file at `path` for the `text` pattern:
```bash
cargo run -q <pattern> <path>
```

For example, you could try searching the `Cargo.toml` file in this codebase for the word "dependencies":
```bash
cargo run -q dependencies Cargo.toml
```

## Codebase guide
The diagram below describes the dependencies between the source files of this project:
```
main.rs
|____lib.rs
	 |____args.rs
	 |____init.rs
	 |____run.rs
```

And here is a description of each of the files:

- **main.rs:** the controlling function that runs the application.
- **lib.rs:** lists the modules that make up the application.
- **args.rs:** a list of the arguments that the user needs to input to run the application.
- **init.rs:** checks that the inputs that the user provided are valid. Also checks that the file can be read.
- **run.rs:** the logic for searching a file for the given `pattern`.

## Man page
A man page is generated from the clap parsed struct in the `args.rs` module every time the application is compiled. They are outputted to the target folders under the filename `head.1`. They can be found using the bash command: `find . -name 'head.1'`. Once you've found one you can view it using the command `man <filepath>`.