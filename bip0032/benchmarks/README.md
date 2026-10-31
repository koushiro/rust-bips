# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13)

## Master key generation

```bash
cargo bench --bench keygen -- --quiet
# Or just bench keygen
```

```text
keygen/bitcoin (secp256k1)
                        time:   [1.0487 µs 1.0551 µs 1.0642 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [18.976 µs 19.010 µs 19.049 µs]
keygen/bip32 (k256)     time:   [429.29 ns 430.23 ns 431.22 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [18.829 µs 18.966 µs 19.231 µs]
keygen/bip0032 (k256)   time:   [456.56 ns 458.03 ns 459.91 ns]
keygen/bip0032 (secp256k1)
                        time:   [453.60 ns 454.56 ns 455.67 ns]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [137.03 µs 138.38 µs 140.63 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [97.306 µs 98.216 µs 99.919 µs]
derive/bip32 (k256)     time:   [151.24 µs 151.49 µs 151.77 µs]
derive/bip32 (k256::ecdsa)
                        time:   [95.802 µs 95.999 µs 96.262 µs]
derive/bip0032 (k256)   time:   [96.290 µs 96.740 µs 97.350 µs]
derive/bip0032 (secp256k1)
                        time:   [181.08 µs 181.57 µs 182.12 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [9.8629 µs 9.9777 µs 10.197 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [23.526 µs 23.581 µs 23.641 µs]
xprv_decode/bip32 (k256)
                        time:   [5.4301 µs 5.4603 µs 5.5051 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [23.835 µs 23.942 µs 24.114 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4656 µs 5.4840 µs 5.5038 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.4477 µs 5.4622 µs 5.4776 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.4059 µs 9.4221 µs 9.4408 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [8.7630 µs 8.7923 µs 8.8286 µs]
xprv_encode/bip32 (k256)
                        time:   [8.9820 µs 9.0774 µs 9.2691 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.0192 µs 9.0603 µs 9.1247 µs]
xprv_encode/bip0032 (k256)
                        time:   [8.9589 µs 8.9723 µs 8.9866 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [8.9467 µs 8.9796 µs 9.0311 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.529 µs 13.576 µs 13.632 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [9.9580 µs 9.9850 µs 10.016 µs]
xpub_decode/bip32 (k256)
                        time:   [10.419 µs 10.549 µs 10.799 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.446 µs 10.477 µs 10.511 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.466 µs 10.515 µs 10.572 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.2570 µs 9.3233 µs 9.4144 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.3495 µs 9.3661 µs 9.3839 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [8.8644 µs 8.8956 µs 8.9323 µs]
xpub_encode/bip32 (k256)
                        time:   [9.1113 µs 9.1304 µs 9.1514 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.1492 µs 9.1841 µs 9.2267 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.1966 µs 9.2148 µs 9.2331 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.1282 µs 9.1495 µs 9.1730 µs]
```
