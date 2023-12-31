[![Rust](https://github.com/EthanJamesLew/concave-hull/actions/workflows/rust.yml/badge.svg)](https://github.com/EthanJamesLew/concave-hull/actions/workflows/rust.yml)

# Fast Concave Hull

This is a fast implementation of concave hull using a k-nearest neighbour approach.

![Concave Hull Approximation of Sparse Polynomial Zonotope](./doc/img/spz.png)

## Setup Rust

Use Cargo 

```shell
cargo build
```

## Setup Python (Development)

Use PyO3 + Maturin

```shell
maturin develop
```