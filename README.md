# zk-proofs for substrate

This library supports for using zk-SNARKs on [substrate](https://github.com/paritytech/substrate) and is implemented as an extension of [librustzcash](https://github.com/zcash/librustzcash). 

Primary purpose of this library is to be used with [zero-chain](https://github.com/LayerXcom/zero-chain). However, it is designed to be as flexible as possible and might be suited well for any other projects to use zk-SNARKs on Substrate.

In order to use the library on Substrate, it is needed to be `no_std` compatible and add some attributes (like `Encode`, `Decode`, etc...)for binary serialization and deserialization. These codec is implemented as a [parity-codec](https://github.com/paritytech/parity-codec).
In addition to these extension, it is added some extra fixes like `Rng` fixes to be compatible with wasm.

## Security Warnings

These libraries are currently under development and have not been fully-reviewed.

## License

All code in this workspace is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
