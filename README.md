# **rs**olver

`rsolver` is a simple DNS resolver CLI utility written in Rust.

## Installation

To install `rsolver`, follow these steps:

1. Clone the repository: `git clone https://github.com/CarlosEduardoL/rsolver.git`
2. Change into the `rsolver` directory: `cd rsolver`
3. Build the project: `cargo build --release`
4. Install the binary: `cargo install --path .`

## Usage

To use `rsolver`, run the following command:

```bash
rsolver [OPTIONS] [DOMAIN]
```

### Arguments:

`<DOMAIN>`  The domain to resolve

### Options:

- `-f <FLAGS>`      These flags are used to control the behavior of DNS queries and responses [possible values: aa, tc, rd, ra, ad, cd]
- `-h, --help`      Print help (see more with '--help')
- `-V, --version`   Print version

## Examples

Resolve the IP address for `example.com`:

```bash
rsolver example.com
```


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

`rsolver` is licensed under the [MIT License](LICENSE).
