# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## Master key generation

```bash
cargo bench --bench keygen -- --quiet
# Or just bench keygen
```

```text
keygen/bitcoin (secp256k1)
                        time:   [1.0571 µs 1.0620 µs 1.0679 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [42.643 µs 42.727 µs 42.833 µs]
keygen/bip32 (k256)     time:   [989.21 ns 1.0338 µs 1.0952 µs]
keygen/bip32 (k256::ecdsa)
                        time:   [42.724 µs 42.800 µs 42.907 µs]
keygen/bip0032 (k256)   time:   [985.76 ns 988.72 ns 992.99 ns]
keygen/bip0032 (k256::ecdsa)
                        time:   [42.683 µs 42.793 µs 42.934 µs]
keygen/bip0032 (secp256k1)
                        time:   [989.48 ns 991.73 ns 994.38 ns]
keygen/bip0032 (libsecp256k1)
                        time:   [1.0528 µs 1.0554 µs 1.0584 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [137.77 µs 144.02 µs 153.25 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [260.14 µs 260.64 µs 261.19 µs]
derive/bip32 (k256)     time:   [347.31 µs 389.29 µs 448.41 µs]
derive/bip32 (k256::ecdsa)
                        time:   [218.24 µs 220.13 µs 222.87 µs]
derive/bip0032 (k256)   time:   [215.40 µs 215.75 µs 216.13 µs]
derive/bip0032 (k256::ecdsa)
                        time:   [216.21 µs 231.92 µs 256.45 µs]
derive/bip0032 (secp256k1)
                        time:   [75.871 µs 78.464 µs 82.932 µs]
derive/bip0032 (libsecp256k1)
                        time:   [167.36 µs 167.72 µs 168.15 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.037 µs 10.058 µs 10.079 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.526 µs 48.165 µs 49.106 µs]
xprv_decode/bip32 (k256)
                        time:   [5.3579 µs 5.3727 µs 5.3872 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.097 µs 47.191 µs 47.291 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.3869 µs 5.4933 µs 5.6569 µs]
xprv_decode/bip0032 (k256::ecdsa)
                        time:   [47.260 µs 47.390 µs 47.530 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.3387 µs 5.4364 µs 5.5901 µs]
xprv_decode/bip0032 (libsecp256k1)
                        time:   [5.4011 µs 5.4272 µs 5.4571 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.5511 µs 9.9616 µs 10.792 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.5800 µs 9.6068 µs 9.6349 µs]
xprv_encode/bip32 (k256)
                        time:   [9.3561 µs 9.3782 µs 9.4029 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.4581 µs 9.5086 µs 9.5728 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.2092 µs 9.2233 µs 9.2380 µs]
xprv_encode/bip0032 (k256::ecdsa)
                        time:   [9.2908 µs 9.6634 µs 10.267 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.2091 µs 9.2249 µs 9.2428 µs]
xprv_encode/bip0032 (libsecp256k1)
                        time:   [9.2422 µs 9.5118 µs 9.9111 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.881 µs 13.926 µs 13.983 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.583 µs 10.957 µs 11.673 µs]
xpub_decode/bip32 (k256)
                        time:   [10.456 µs 10.489 µs 10.524 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.486 µs 10.511 µs 10.538 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.423 µs 10.462 µs 10.515 µs]
xpub_decode/bip0032 (k256::ecdsa)
                        time:   [10.421 µs 10.604 µs 10.865 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.1352 µs 9.1593 µs 9.1862 µs]
xpub_decode/bip0032 (libsecp256k1)
                        time:   [12.733 µs 12.761 µs 12.792 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.7373 µs 9.7743 µs 9.8244 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.7952 µs 9.8231 µs 9.8522 µs]
xpub_encode/bip32 (k256)
                        time:   [9.3016 µs 9.7414 µs 10.296 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.1835 µs 9.2108 µs 9.2412 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.1716 µs 9.1968 µs 9.2259 µs]
xpub_encode/bip0032 (k256::ecdsa)
                        time:   [9.1805 µs 9.4061 µs 9.7267 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.1181 µs 9.1369 µs 9.1575 µs]
xpub_encode/bip0032 (libsecp256k1)
                        time:   [9.1753 µs 9.5126 µs 10.089 µs]
```
