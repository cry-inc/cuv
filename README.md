# Compressed Unit Vectors
[![Build Status](https://github.com/cry-inc/cuv/workflows/Build/badge.svg)](https://github.com/cry-inc/cuv/actions)
[![Crates.io](https://img.shields.io/crates/v/cuv.svg)](https://crates.io/crates/cuv)
[![Documentation](https://docs.rs/cuv/badge.svg)](https://docs.rs/cuv)
[![No Unsafe](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Dependencies](https://deps.rs/repo/github/cry-inc/cuv/status.svg)](https://deps.rs/repo/github/cry-inc/cuv)

A Rust library for compressed unit vectors.
It is a simple Rust port of [some C/C++ code](#original-source).

It can be used to efficiently store and transfer things like normal vectors used in computer graphics.
You can use it lower the memory footsprint or reduce the size of files on disk.
Intead of three 32 bit floats you can represent the unit vector with a single 16 bit unsigned integer.

The compression itself is **lossy**, so most input values will be mapped to something slighty different when being unpacked.
For many use cases this loss is acceptable. Please make sure this applies to your case as well.

## Getting Started
The library provides low level functions to pack and unpack the values and a high level "rustified" interface.

The low level interface requires manual creation of a lookup-table:

```
let packed: u16 = cuv::pack(1.0, 0.0, 0.0);
let lut = cuv::create_lut();
let unpacked = cuv::unpack(packed, &lut);
assert_eq!(unpacked, [1.0, 0.0, 0.0]);
```

The high level interface will take care of the lookup-table for you and is a bit more convinient to use:

```
let input = [1.0, 0.0, 0.0];
let packed = cuv::CompUnitVec::from_slice(&input);
let unpacked = packed.get();
assert_eq!(unpacked, [1.0, 0.0, 0.0]);
```

## Original Source
The original C/C++ code comes from Rafael Baptista who published the idea in a long lost post.
The code was originally described as "unit vector to 16-bit word conversion algorithm".

In 2013 Rafael reposted the description and code again on his blog at
[https://oroboro.com/compressed-unit-vectors/](https://web.archive.org/web/20201022211551/https://oroboro.com/compressed-unit-vectors/).

## Benchmarks
This repo includes some benchmarks implemented with Criterion.rs to be able to detect performance regressions during development.
You can run the benchmarks using the command `cargo bench`.
