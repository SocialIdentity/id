# Nameserver - simple ENS style name resolver


Builds on top of the metadata pattern in `cw721-metadata-onchain`.

All of the CW-721 logic and behaviour you would expect for an NFT is implemented as normal, but additionally at mint time,
verification and expiry info can be attached to a token.

Exposes two new query message types that can be called:

```rust

```

The responses are:

```rust

```


To set this information, new meta fields are available on mint:

```rust

```

Note that the `xxxx` could of course be a single address, a multisig, or a DAO.
