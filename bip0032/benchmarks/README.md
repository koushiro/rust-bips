# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.96.1 (31fca3adb 2026-06-26)

## Master key generation

```bash
cargo bench --bench keygen -- --quiet
# Or just bench keygen
```

```text
keygen/bitcoin (secp256k1)
                        time:   [1.0769 µs 1.0838 µs 1.0929 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [43.364 µs 43.645 µs 43.990 µs]
keygen/bip32 (k256)     time:   [1.0012 µs 1.0105 µs 1.0220 µs]
keygen/bip32 (k256::ecdsa)
                        time:   [43.052 µs 43.153 µs 43.273 µs]
keygen/bip0032 (k256)   time:   [431.68 ns 439.12 ns 450.34 ns]
keygen/bip0032 (secp256k1)
                        time:   [438.02 ns 447.83 ns 467.70 ns]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [144.49 µs 147.53 µs 151.71 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [265.27 µs 271.19 µs 280.64 µs]
derive/bip32 (k256)     time:   [348.96 µs 352.74 µs 357.92 µs]
derive/bip32 (k256::ecdsa)
                        time:   [221.79 µs 229.41 µs 240.10 µs]
derive/bip0032 (k256)   time:   [206.97 µs 214.89 µs 224.22 µs]
derive/bip0032 (secp256k1)
                        time:   [78.433 µs 80.177 µs 82.276 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.119 µs 10.292 µs 10.620 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [48.613 µs 49.799 µs 51.206 µs]
xprv_decode/bip32 (k256)
                        time:   [5.3985 µs 5.4366 µs 5.4924 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.657 µs 48.294 µs 49.526 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4222 µs 5.5168 µs 5.6636 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.5842 µs 5.8271 µs 6.1521 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [10.066 µs 10.293 µs 10.593 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [10.001 µs 10.210 µs 10.458 µs]
xprv_encode/bip32 (k256)
                        time:   [9.4213 µs 9.5580 µs 9.8152 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.5011 µs 9.5584 µs 9.6250 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.4665 µs 9.6579 µs 9.8936 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.5851 µs 9.8317 µs 10.162 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [14.017 µs 14.157 µs 14.387 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.747 µs 10.966 µs 11.328 µs]
xpub_decode/bip32 (k256)
                        time:   [10.684 µs 10.848 µs 11.042 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.594 µs 10.649 µs 10.727 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.587 µs 10.720 µs 10.921 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.3394 µs 9.4212 µs 9.5349 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.7441 µs 9.8197 µs 9.9193 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [10.053 µs 10.242 µs 10.490 µs]
xpub_encode/bip32 (k256)
                        time:   [9.2920 µs 9.3930 µs 9.5319 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.6393 µs 10.481 µs 12.008 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.4464 µs 9.7275 µs 10.134 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.3078 µs 9.4452 µs 9.6146 µs]
```
