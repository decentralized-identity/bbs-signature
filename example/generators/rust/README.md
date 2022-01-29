# Demo using Rust for creating BBS+ generators

The demo by default creates 10 global generators.

The command line accepts two arguments

```bash
USAGE:
    bbs-signature-generator-demo [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --generator-type <generator-type>     [default: Global]
    -l, --length <length>                     [default: 10]
```

1. `-g` accepted values are Global and Signer
   1. Global creates the generators for a global setting
   2. Signer creates the generators for a signer specific setting
2. `-l` accepts any positive integer

The demo will output the generators in compressed format hex encoded.