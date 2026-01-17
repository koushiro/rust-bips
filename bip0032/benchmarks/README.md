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
                        time:   [1.0637 µs 1.0749 µs 1.0972 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [42.922 µs 42.983 µs 43.045 µs]
keygen/bip32 (k256)     time:   [989.84 ns 991.27 ns 992.71 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [42.953 µs 43.061 µs 43.206 µs]
keygen/bip0032 (k256)   time:   [1.0093 µs 1.0182 µs 1.0327 µs]
keygen/bip0032 (k256::ecdsa)
                        time:   [42.964 µs 43.439 µs 44.395 µs]
keygen/bip0032 (secp256k1)
                        time:   [1.0166 µs 1.0211 µs 1.0265 µs]
keygen/bip0032 (libsecp256k1)
                        time:   [1.0776 µs 1.0852 µs 1.0976 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [138.72 µs 139.15 µs 139.66 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [261.46 µs 262.07 µs 262.75 µs]
derive/bip32 (k256)     time:   [343.23 µs 345.52 µs 349.53 µs]
derive/bip32 (k256::ecdsa)
                        time:   [217.95 µs 218.46 µs 219.00 µs]
derive/bip0032 (k256)   time:   [217.64 µs 218.26 µs 219.02 µs]
derive/bip0032 (k256::ecdsa)
                        time:   [217.54 µs 220.08 µs 224.89 µs]
derive/bip0032 (secp256k1)
                        time:   [76.648 µs 76.829 µs 77.027 µs]
derive/bip0032 (libsecp256k1)
                        time:   [168.29 µs 168.69 µs 169.18 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.106 µs 10.248 µs 10.533 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.433 µs 47.579 µs 47.755 µs]
xprv_decode/bip32 (k256)
                        time:   [5.3838 µs 5.4017 µs 5.4228 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.395 µs 47.842 µs 48.561 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.3631 µs 5.3802 µs 5.3995 µs]
xprv_decode/bip0032 (k256::ecdsa)
                        time:   [47.352 µs 47.463 µs 47.587 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.3895 µs 5.4389 µs 5.5060 µs]
xprv_decode/bip0032 (libsecp256k1)
                        time:   [5.4699 µs 5.5417 µs 5.6770 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.6401 µs 9.6633 µs 9.6918 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.2443 µs 9.2848 µs 9.3401 µs]
xprv_encode/bip32 (k256)
                        time:   [9.2807 µs 9.3377 µs 9.4257 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.2731 µs 9.3155 µs 9.3743 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.1623 µs 9.1800 µs 9.2001 µs]
xprv_encode/bip0032 (k256::ecdsa)
                        time:   [9.2278 µs 9.2578 µs 9.2897 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.1615 µs 9.1789 µs 9.1984 µs]
xprv_encode/bip0032 (libsecp256k1)
                        time:   [9.1920 µs 9.2669 µs 9.3936 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.942 µs 14.006 µs 14.118 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.659 µs 10.692 µs 10.724 µs]
xpub_decode/bip32 (k256)
                        time:   [10.553 µs 10.592 µs 10.644 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.580 µs 10.606 µs 10.635 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.602 µs 10.652 µs 10.717 µs]
xpub_decode/bip0032 (k256::ecdsa)
                        time:   [10.630 µs 10.710 µs 10.813 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.2600 µs 9.2822 µs 9.3055 µs]
xpub_decode/bip0032 (libsecp256k1)
                        time:   [12.918 µs 12.967 µs 13.024 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.8041 µs 9.8488 µs 9.9121 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.5077 µs 9.6931 µs 10.037 µs]
xpub_encode/bip32 (k256)
                        time:   [9.1251 µs 9.1493 µs 9.1759 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.1390 µs 9.1722 µs 9.2086 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.2158 µs 9.2887 µs 9.4080 µs]
xpub_encode/bip0032 (k256::ecdsa)
                        time:   [9.1633 µs 9.1971 µs 9.2357 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.1715 µs 9.2105 µs 9.2570 µs]
xpub_encode/bip0032 (libsecp256k1)
                        time:   [9.1430 µs 9.1948 µs 9.2866 µs]
```
