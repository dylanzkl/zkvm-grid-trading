# Grid trading example base on zkvm

This is a simple example developed based on zkvm. The host inputs an account with initial state of two tokens:usd and eth, and a bunch of transactions which swap between usd and eth two the guest. Then the zkvm executes all the transactions and transfer to a post state accordingly. Bysides, the zkvm will calculate hash of initial state and post state, and then commit these two hashes to the host. Finally, the host will verify the two hash.

## Quick Start

To build all methods  run the following
command:

```bash
cargo build --release
```

then to exectute the binary use the following command 

```bash
./target/release/host [transaction_amount]
```

## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── core                                   <-- [lib of struct definition]
├── host
│   ├── Cargo.toml
│   └── srcg
│       └── main.rs                        <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── bin
    │           └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```
