# CCWC

## Description

> [!IMPORTANT]  
> **WIP**
>
> This is a work in progress and is not yet ready for use (and may never be).

(Educational & Recreational) implementation of [wc unix command](https://fr.wikipedia.org/wiki/Wc_(Unix)) in Rust

## Usage

> [!IMPORTANT]
> **Format support**
> 
> For now, only text files are supported. The program will not work correctly with other file types.

```bash
cargo run -- [OPTIONS] [FILE]
```

or

```bash
cargo build
```

and then :

```bash
./target/debug/ccwc [OPTIONS] [FILE]
```

or install the binary in your path and use it as a regular command :

```bash
ccwc [OPTIONS] [FILE]
```

## Options (for now)

- `-c` : Print the byte counts
- `-m` : Print the character counts
- `-l` : Print the newline counts
- `-w` : Print the word counts

## Contribution

Any remarks or suggestions are very welcome