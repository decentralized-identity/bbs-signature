# Message Generator CLI

The following is a rust based tool used to produce the set of public parameters known as message generators which are used by the BBS Signature scheme.

# Installation

To build this CLI you must have rust installed. The recommended way to set this up is via [rustup](https://www.rust-lang.org/tools/install).

# Build

Run the following to build the CLI tool

```bash
cargo build
```

# Running

Run the following to produce the help screen for the CLI tool

```bash
./target/debug/bbs-signature-generator-demo -h
```

*Note* This CLI tool is used to automatically populate the `../fixtures/generators.json` file required by the spec tool which is responsible for automatically populating the spec with the latest fixtures.

# Usage

The CLI accepts two arguments

```bash
USAGE:
    bbs-signature-generator-demo [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --generator-type <generator-type>     [default: Global]
    -l, --length <length>                     [default: 10]
    -s, --suite <suite>                       [default: Shake]
```

1. `-g` accepted values are Global and Signer
   1. Global creates the generators for a global setting
   2. Signer creates the generators for a signer specific setting
2. `-l` accepts any positive integer
3. `-s` accepted values are Shake, xof, Sha and xmd
   1. Shake or xof creates generators for the [BLS12-381-SHAKE-256](https://identity.foundation/bbs-signature/draft-looker-cfrg-bbs-signatures.html#name-bls12-381-shake-256) ciphersuite
   2. Sha or xmd creates generators for the [BLS12-381-SHA-256](https://identity.foundation/bbs-signature/draft-looker-cfrg-bbs-signatures.html#name-bls12-381-sha-256) ciphersuite

The demo will output the generators in compressed format hex encoded, an example of which is

```
G_1 = a9b48966d6ed474ff66dc68ec717704a6b4fe40c1cbcbd3f1ca4feeed708893868b879e1d2d3ee0af1cca5fa35c28dcd
G_2 = 93db6ae63cf4491e2323ba5c5f5f4383f7bb7d333d6c2aa301f96c3c6afdb5bdce69f5ad3c908977b6c5febaf0840d61
G_3 = a384953d5ea2f88219a91da5942d9ad3d76b9e2048eb22a1002659dc44e8a174167cfa191e7a7eefc6888cb90e72c8b3
G_4 = a4961c6d98f4212cff26f51cc303c05ee699552042b65dfe45cc4f9f7f354ec458395405a879b45f898be3c31ac1e291
G_5 = 904580545192ce5b623072e013e4172dac9a28ae28e4816b7f95b91cf8baa18504ac7025e1eff5dec935c228862c7359
G_6 = 8e3803894adfd3e7882caa45199a7a4d51e797f09b56173d6d9b0e98f946736485d39a9c1451708e1958e4e1e4ece5d1
G_7 = b6ceacbd6198d20d9f224395be3e9560fd50e97d3b061edc4eecfd186f738c0d0964dba23a48c8ca564c1af20a1e5d23
G_8 = adc6113b820926ecd41a05082e0ada9a5625c20c591e2e6d7de1732730a67e06298d26054cdb7ec3ed12b6e92c817821
G_9 = 85cf61e7a7a8b5074eeac147066366feab925e8239126da7e0c341deed5be180b34808a8275e2ffc476ce8dc613a38cb
G_10 = b4800a3c8260068b65bee8b687f99d39cac1a66292d39afb88610ad023b861df1f1424566d9be2ffcdc624c65d8cad5b
```
