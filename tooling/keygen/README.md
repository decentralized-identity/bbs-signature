# Key Generation CLI

A reference tool for creating a BBS key pair.

## Build

You need to have Rust install. After, run the following from the current directory to build the tool

```
cargo build -p keygen
```

## Running

From the current directory, run the tool using

```
cargo run -p keygen -- -h
```
This will print the help screen with the following different options for using the tool.

```
USAGE:
    keygen [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>            
        --ikm <ikm>               [default: <DEFAULT IKM>]
        --key-info <key-info>     [default: <DEFAULT KEY_INFO>]
    -o, --out <out>               [default: Print]
```

Note: The `<DEFAULT IKM>` and `<DEFAULT KEY_INFO>` are the following.

```
DEFAULT IKM = 746869732d49532d6a7573742d616e2d546573742d494b4d2d746f2d67656e65726174652d246528724074232d6b6579

DEFAULT KEY_INFO = 746869732d49532d6a7573742d616e2d546573742d494b4d2d746f2d67656e65726174652d246528724074232d6b6579
```

## Saving the key pair to a file
From the current directory, run
```
cargo run -p keygen -- -o file
```
This will save the keygen to the default output direction `../fixtures/fixture_data/keyPair.json`. You can supply a new destination using
```
cargo run -p keygen -- -o file -f <NEW FILEPATH>
```
Note: `<NEW FILEPATH>` will be relative to the current directory, not the project's root.