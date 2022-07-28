# Line Audiolizer
Turn lines of code (or text files) into sound, written in Rust.

Prints stdin, playing a tone for each line. The pitch of the tone depends on the length (or rather, UTF-8 width) of the line. Tabs count as 8 spaces.

Idea by [scplusplus](https://github.com/scplusplus).

## Example

Play own source code:
```sh
cargo run < src/main.rs
```

Fancy print while playing, using [batcat](https://github.com/sharkdp/bat):
```sh
cargo run < src/main.rs | batcat --paging never --language rust
```
