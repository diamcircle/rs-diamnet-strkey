# rs-diamnet-strkey

Library and CLI containing types and functionality for working with Diamnet
Strkeys.

**This repository contains code that is in early development, incomplete,
not tested, and not recommended for use. The API is unstable, experimental,
and is receiving breaking changes frequently.**

### Usage

#### Library
To use the library, include in your toml:

```toml
diamnet-strkey = "..."
```

#### CLI

To use the CLI:

```console
cargo install --locked diamnet-strkey --version ... --features cli
```

##### Examples

Decode a `G` account/public-key strkey:
```console
$ diamnet-strkey decode GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF
PublicKeyEd25519(PublicKey(0000000000000000000000000000000000000000000000000000000000000000))
```

Decode a `C` contract strkey:
```console
$ diamnet-strkey decode CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSC4
Contract(Contract(0000000000000000000000000000000000000000000000000000000000000000))
```

License: Apache-2.0
