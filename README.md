# Rotten Potato

In progress parser for [Eww](https://github.com/elkowar/eww). A language for creating widgets!

## Running the tests

```sh
cargo test
```

or if you have `cargo-nextest`:

```sh
cargo nextest run
```

## Running

The project is not designed to be run directly, but if you want to experiment, simply create a `main.rs` file.

### Calling a specific parser:

```rs
fn main() {
    let lexer = rotten_potato::lexer::Lexer::new(r#"(defvar foo "bar")"#);
    let mut errs = Vec::new();
    let ast = rotten_potato::grammar::DefVarParser::new().parse(&mut errs, lexer);

    println!("errs:{:?}\nast:{:?}", errs, ast);
}
```

### Parsing "a file"

```rs
fn main() {
    let source = r#"
        (include "pollers.yuck")
        (include "revealer.yuck")

        (defwidget bar []
          (centerbox :orientation "h"
            (box :orientation "h" :space-evenly false (workspaces) )
            (box :orientation "h" :space-evenly false (label :text " ${time} - ") (weather) )
            (sidestuff)
          )
        )

        (defwidget bottombar []
          (centerbox :orientation "h"
            (box :halign "start" :orientation "h" :space-evenly false (workspaces))
            (box :halign "end" :orientation "h" :space-evenly false)
            (box :orientation "h" :halign "end" :space-evenly false
              (network)
            )
          )
        )"#;

    let result = rotten_potato::parse_top_level(source);

    println!("{:#?}", result);
}
```

---

All rights reserved.
