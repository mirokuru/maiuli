# Maiuli

![Maiuli](/static/maiuli-1200x630.png)

A modification of a finnish version of the word guessing game [Wordle](https://www.powerlanguage.co.uk/wordle/) implemented in [Rust](https://www.rust-lang.org).

This is directly based on the [MIT licensed Sanuli source code](https://github.com/Cadiac/sanuli) by Cadiac (Jaakko Husso).

## Quick start

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

For the UI you may need to install these as well:

```
sudo apt install pkg-config
sudo apt install libssl-dev
```

To build the WASM based [yew](https://yew.rs/) UI, further wasm tooling is required

```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

Create word list files and populate them with uppercase words, one per line

```
touch common-words.txt
touch full-words.txt
touch profanities.txt
touch easy-words.txt
```

Start the UI in development mode
```
trunk serve
```

## Word lists

Four separate word list files in the root of this project containing all the words are required. The lists are not included in this repository.

The lists are:
- `full-words.txt` - Full list of all accepted 4, 5, 6 and 7 character words. The checks if a word real or not is done against this list
- `common-words.txt` - Subset of the full words list, intended for the default game mode. Note that all these words _must_ exist on the `full-words.txt`
- `easy-words.txt` - Subset of the full words list, intended for easier game mode. Note that all these words _must_ exist on the `full-words.txt`
- `profanities.txt` - Words filtered out when profanities filter is enabled

Additionally, for Maiuli, all the words from the common and easy word lists should have entries in the components/quotes.rs file so that the user can be shown the corresponding quote and link for a word.

Beware that these are _included in the release binary_, and anyone can obtain the lists!

## Generating base word lists

To create a word list, a dictionary like the "nykysuomen sanalista" by [Kotus](https://kaino.kotus.fi/sanat/nykysuomi/),
licensed with [CC BY 3.0](https://creativecommons.org/licenses/by/3.0/deed.fi), can be used as a baseline to build from.

A parser for parsing `kotus-sanalista_v1.xml` file from [Kotus](https://kaino.kotus.fi/sanat/nykysuomi/) is included:

```bash
cargo run --bin parse-kotus-word-list your/path/to/kotus-sanalista_v1.xml
```

which creates a `full-words-generated.txt` file in the working directory.

## Development

**NOTE:** Rust flag `--cfg=web_sys_unstable_apis` is required for copying to clipboard to work.
Clipboard API also only works in HTTPS context.

To set the flag manually with environment variables, run:
```
export RUSTFLAGS=--cfg=web_sys_unstable_apis
```

You can ignore this at manual development (as the clipboard API won't work without HTTPS anywaays)
or run your trunk commands with it set.

For normal development, start the web server with

```
trunk serve
```

This should make the UI available at 0.0.0.0:8080 with hot reload on code changes.

To change the default port, use

```
trunk serve --port=9090
```

## Release build

Pass the rust flags for building clipboard features & strip your home library paths from the binary.

```
RUSTFLAGS="--cfg=web_sys_unstable_apis --remap-path-prefix $HOME=~" trunk build --release
```

and copy the produced `dist` directory to your target server.

### Optimizing .wasm binary size

The `.wasm` binary is quite large, as it includes the full word lists and bunch of code.
`wasm-opt` from [binaryen](https://github.com/WebAssembly/binaryen/releases) tools can be to
optimized to further reduce the size of the binary.

```
wasm-opt -Os -o output.wasm input.wasm
```

replacing the input and output files with your binary name, ie. `dist/index-fea16a946b74a1d4_bg.wasm`.

Some automation for this should be made.
