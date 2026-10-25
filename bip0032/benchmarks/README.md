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
                        time:   [1.1091 µs 1.1597 µs 1.2225 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [20.086 µs 20.860 µs 21.836 µs]
keygen/bip32 (k256)     time:   [442.38 ns 445.00 ns 448.04 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [20.448 µs 21.321 µs 22.818 µs]
keygen/bip0032 (k256)   time:   [476.96 ns 489.49 ns 509.86 ns]
keygen/bip0032 (secp256k1)
                        time:   [466.61 ns 471.15 ns 476.45 ns]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [140.77 µs 141.73 µs 142.84 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [100.26 µs 101.03 µs 101.93 µs]
derive/bip32 (k256)     time:   [157.51 µs 160.03 µs 162.90 µs]
derive/bip32 (k256::ecdsa)
                        time:   [101.90 µs 104.00 µs 107.86 µs]
derive/bip0032 (k256)   time:   [98.910 µs 102.03 µs 106.47 µs]
derive/bip0032 (secp256k1)
                        time:   [189.29 µs 192.43 µs 196.19 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [9.9199 µs 10.080 µs 10.377 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [23.738 µs 23.820 µs 23.910 µs]
xprv_decode/bip32 (k256)
                        time:   [5.4202 µs 5.4342 µs 5.4484 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [24.022 µs 24.146 µs 24.282 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4571 µs 5.5057 µs 5.5926 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.4505 µs 5.4667 µs 5.4837 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.5047 µs 9.5347 µs 9.5675 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [8.7005 µs 8.7392 µs 8.7860 µs]
xprv_encode/bip32 (k256)
                        time:   [9.0248 µs 9.1062 µs 9.2230 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.0370 µs 9.0594 µs 9.0864 µs]
xprv_encode/bip0032 (k256)
                        time:   [8.9886 µs 9.0146 µs 9.0436 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.0377 µs 9.2503 µs 9.5029 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.732 µs 14.425 µs 15.818 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.130 µs 10.152 µs 10.173 µs]
xpub_decode/bip32 (k256)
                        time:   [10.487 µs 10.521 µs 10.561 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.500 µs 10.529 µs 10.564 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.491 µs 10.517 µs 10.544 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.2711 µs 9.3670 µs 9.5555 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.5350 µs 9.5693 µs 9.6063 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [8.9610 µs 9.0024 µs 9.0534 µs]
xpub_encode/bip32 (k256)
                        time:   [9.2538 µs 9.4143 µs 9.7080 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.2394 µs 9.2678 µs 9.2980 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.2503 µs 9.2694 µs 9.2904 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.2391 µs 9.3440 µs 9.5067 µs]
```
