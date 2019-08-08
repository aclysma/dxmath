
# dxmath

A port of [DirectXMath](https://github.com/microsoft/DirectXMath) to Rust

## Development Status

Barely even started! I haven't decided how much time to commit to this yet. I'm just exploring the 
feasibility and value for now.

## Alternatives

Even if this port were complete, many people will be better served by one of the following libraries:
* `nalgebra`/`nalgebra_glm`
* `cgmath`
* `glam`
* `hektor`

## Why Port DirectXMath

* It's battle-hardened
* It has excellent [documentation](https://docs.microsoft.com/en-us/windows/win32/dxmath/directxmath-portal)
* It has reference implementations for many common instruction sets (SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, AVX, F16C, FMA, ARM-NEON) - and many of them are even well commented! :)
* The upstream code is likely to be well-maintained with new instruction sets adopted as they become relevant
* Many developers coming from C++ will be very familiar with it
* I'll probably learn something along the way

## Goals

* Port the C++ library to Rust as directly as possible, with no attempt to be Rust-idiomatic. 
** This makes the porting job a bit more straightforward, and correctness should be easier to verify.
** It will also make it easier to pull in any upstream changes.
* An additional layer (similar to [SimpleMath](https://github.com/microsoft/DirectXTK/blob/master/Inc/SimpleMath.h)
could be layered on top of this to make it more Rust-idiomatic.

## Conventions

DirectXMath uses row-major matrices, row vectors, and pre-multiplication. Handedness is determined 
by which function version is used (RH vs. LH), otherwise the function works with either left-handed 
or right-handed view coordinates.

More information in the [docs](https://docs.microsoft.com/en-us/windows/win32/dxmath/pg-xnamath-getting-started)

## Contribution

All contributions are assumed to be dual-licensed under MIT/Apache-2. 

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

The original DirectXMath library is MIT-licensed (NOT Apache 2.0).