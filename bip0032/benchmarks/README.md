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
                        time:   [1.0677 µs 1.0716 µs 1.0760 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [43.365 µs 43.891 µs 44.595 µs]
keygen/bip32 (k256)     time:   [998.66 ns 1.0283 µs 1.0683 µs]
keygen/bip32 (k256::ecdsa)
                        time:   [42.928 µs 43.378 µs 44.135 µs]
keygen/bip0032 (k256)   time:   [1.0065 µs 1.0232 µs 1.0475 µs]
keygen/bip0032 (k256::ecdsa)
                        time:   [42.758 µs 42.968 µs 43.251 µs]
keygen/bip0032 (secp256k1)
                        time:   [1.0103 µs 1.0221 µs 1.0390 µs]
keygen/bip0032 (libsecp256k1)
                        time:   [1.0731 µs 1.0813 µs 1.0923 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [137.24 µs 137.72 µs 138.28 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [258.20 µs 258.61 µs 259.09 µs]
derive/bip32 (k256)     time:   [339.93 µs 340.72 µs 341.63 µs]
derive/bip32 (k256::ecdsa)
                        time:   [215.44 µs 215.85 µs 216.31 µs]
derive/bip0032 (k256)   time:   [215.30 µs 219.02 µs 225.56 µs]
derive/bip0032 (k256::ecdsa)
                        time:   [215.90 µs 216.39 µs 216.94 µs]
derive/bip0032 (secp256k1)
                        time:   [76.803 µs 78.815 µs 81.823 µs]
derive/bip0032 (libsecp256k1)
                        time:   [169.07 µs 171.39 µs 175.42 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.087 µs 10.127 µs 10.174 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.638 µs 48.021 µs 48.578 µs]
xprv_decode/bip32 (k256)
                        time:   [5.3659 µs 5.3945 µs 5.4358 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.544 µs 48.406 µs 49.839 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4247 µs 5.5417 µs 5.7256 µs]
xprv_decode/bip0032 (k256::ecdsa)
                        time:   [47.198 µs 47.753 µs 48.708 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.4504 µs 5.5436 µs 5.6790 µs]
xprv_decode/bip0032 (libsecp256k1)
                        time:   [5.4581 µs 5.5517 µs 5.7160 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.9382 µs 10.056 µs 10.227 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.9155 µs 10.032 µs 10.213 µs]
xprv_encode/bip32 (k256)
                        time:   [9.3768 µs 9.4101 µs 9.4474 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.4510 µs 9.5183 µs 9.6018 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.5574 µs 9.8932 µs 10.427 µs]
xprv_encode/bip0032 (k256::ecdsa)
                        time:   [9.4275 µs 9.4679 µs 9.5113 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.4068 µs 9.5163 µs 9.6834 µs]
xprv_encode/bip0032 (libsecp256k1)
                        time:   [9.3506 µs 9.4720 µs 9.6702 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.834 µs 13.869 µs 13.903 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.633 µs 10.669 µs 10.710 µs]
xpub_decode/bip32 (k256)
                        time:   [10.624 µs 10.681 µs 10.752 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.670 µs 10.850 µs 11.129 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.618 µs 10.715 µs 10.846 µs]
xpub_decode/bip0032 (k256::ecdsa)
                        time:   [10.620 µs 10.694 µs 10.788 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.3692 µs 9.4883 µs 9.7016 µs]
xpub_decode/bip0032 (libsecp256k1)
                        time:   [12.973 µs 13.071 µs 13.181 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.7554 µs 10.011 µs 10.447 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.3106 µs 9.8880 µs 11.169 µs]
xpub_encode/bip32 (k256)
                        time:   [9.3383 µs 9.4263 µs 9.5637 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.3872 µs 9.5523 µs 9.7894 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.2440 µs 9.2816 µs 9.3260 µs]
xpub_encode/bip0032 (k256::ecdsa)
                        time:   [9.3408 µs 9.4502 µs 9.6331 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.1744 µs 9.1906 µs 9.2100 µs]
xpub_encode/bip0032 (libsecp256k1)
                        time:   [9.4494 µs 9.8265 µs 10.427 µs]
```
