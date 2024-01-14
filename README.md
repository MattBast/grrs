# grrs

Following this tutorial to better undertsand building CLI apps with Rust: https://rust-cli.github.io/book/index.html

# Man page
A man page is generated from the clap parsed struct in the `args.rs` module every time the application is compiled. They are outputted to the target folders under the filename `head.1`. They can be found using the bash command: `find . -name 'head.1'`. Once you've found one you can view it using the command `man <filepath>`.