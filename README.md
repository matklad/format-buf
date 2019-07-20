[![Build Status](https://travis-ci.org/matklad/format-buf.svg?branch=master)](https://travis-ci.org/matklad/format-buf)
[![Crates.io](https://img.shields.io/crates/v/format-buf.svg)](https://crates.io/crates/format-buf)
[![API reference](https://docs.rs/format-buf/badge.svg)](https://docs.rs/format-buf/)

# Overview

A drop-in replacement for `std::format!`, which can optionally accept a an
existing `String` buffer.

```rust
use format_buf::format;

let mut buf = format!("Roses are {},\n", "red");
let () = format!(buf, "Violets are {}.", "blue");
assert_eq!(buf, "\
    Roses are red,\n\
    Violets are blue.\
")
