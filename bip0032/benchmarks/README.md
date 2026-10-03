# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.99.0 (b940084d7 2026-09-28)

## Master key generation

```bash
cargo bench --bench keygen -- --quiet
# Or just bench keygen
```

```text
keygen/bitcoin (secp256k1)
                        time:   [1.0658 µs 1.0714 µs 1.0783 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [43.618 µs 44.211 µs 44.973 µs]
keygen/bip32 (k256)     time:   [405.47 ns 406.71 ns 408.04 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [40.046 µs 40.400 µs 40.847 µs]
keygen/bip0032 (k256)   time:   [430.04 ns 437.97 ns 448.34 ns]
keygen/bip0032 (secp256k1)
                        time:   [441.05 ns 459.60 ns 487.84 ns]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [138.19 µs 138.84 µs 139.69 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [262.25 µs 265.80 µs 270.97 µs]
derive/bip32 (k256)     time:   [327.58 µs 343.45 µs 367.75 µs]
derive/bip32 (k256::ecdsa)
                        time:   [200.28 µs 200.99 µs 201.79 µs]
derive/bip0032 (k256)   time:   [202.77 µs 207.42 µs 213.72 µs]
derive/bip0032 (secp256k1)
                        time:   [183.44 µs 185.56 µs 188.60 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.017 µs 10.234 µs 10.497 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [48.985 µs 50.304 µs 51.893 µs]
xprv_decode/bip32 (k256)
                        time:   [5.6993 µs 5.7919 µs 5.9074 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [44.668 µs 44.784 µs 44.927 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.5472 µs 5.5789 µs 5.6145 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.5494 µs 5.6478 µs 5.7959 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.4260 µs 9.5149 µs 9.6294 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.3220 µs 9.3819 µs 9.4480 µs]
xprv_encode/bip32 (k256)
                        time:   [9.3163 µs 9.4196 µs 9.5609 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.3387 µs 9.4570 µs 9.6224 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.1619 µs 9.2066 µs 9.2593 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.1718 µs 9.2516 µs 9.3790 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.608 µs 13.690 µs 13.792 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.580 µs 10.666 µs 10.778 µs]
xpub_decode/bip32 (k256)
                        time:   [10.824 µs 11.414 µs 12.230 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.536 µs 10.598 µs 10.675 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.772 µs 11.013 µs 11.367 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.3324 µs 9.3723 µs 9.4153 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.6097 µs 9.8291 µs 10.114 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.5486 µs 9.7343 µs 9.9613 µs]
xpub_encode/bip32 (k256)
                        time:   [9.2181 µs 9.2850 µs 9.3815 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.2762 µs 9.6086 µs 10.126 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.2211 µs 9.2647 µs 9.3222 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.2803 µs 9.3826 µs 9.4988 µs]
```
