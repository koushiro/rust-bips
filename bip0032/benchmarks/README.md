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
                        time:   [1.0607 µs 1.0862 µs 1.1374 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [42.825 µs 42.925 µs 43.034 µs]
keygen/bip32 (k256)     time:   [992.64 ns 994.86 ns 997.24 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [42.851 µs 42.956 µs 43.072 µs]
keygen/bip0032 (k256)   time:   [1.0072 µs 1.0092 µs 1.0114 µs]
keygen/bip0032 (secp256k1)
                        time:   [1.0158 µs 1.0178 µs 1.0201 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [137.57 µs 138.00 µs 138.51 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [259.93 µs 260.55 µs 261.26 µs]
derive/bip32 (k256)     time:   [342.67 µs 343.38 µs 344.13 µs]
derive/bip32 (k256::ecdsa)
                        time:   [217.50 µs 217.97 µs 218.46 µs]
derive/bip0032 (k256)   time:   [216.79 µs 217.39 µs 218.06 µs]
derive/bip0032 (secp256k1)
                        time:   [76.699 µs 80.643 µs 87.096 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.076 µs 10.096 µs 10.119 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.536 µs 47.741 µs 47.994 µs]
xprv_decode/bip32 (k256)
                        time:   [5.4388 µs 5.4676 µs 5.5003 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.416 µs 47.600 µs 47.822 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4610 µs 5.4995 µs 5.5431 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.4711 µs 5.6594 µs 5.9702 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.9890 µs 10.022 µs 10.060 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.6526 µs 9.7873 µs 9.9959 µs]
xprv_encode/bip32 (k256)
                        time:   [9.7661 µs 9.7950 µs 9.8252 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.9763 µs 10.105 µs 10.241 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.8094 µs 9.8699 µs 9.9398 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.7993 µs 9.8549 µs 9.9156 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [14.389 µs 14.502 µs 14.637 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.941 µs 10.999 µs 11.057 µs]
xpub_decode/bip32 (k256)
                        time:   [10.845 µs 10.908 µs 10.988 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.954 µs 11.987 µs 14.156 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.942 µs 11.079 µs 11.248 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.8025 µs 9.9168 µs 10.056 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [10.145 µs 10.240 µs 10.356 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [10.185 µs 10.272 µs 10.368 µs]
xpub_encode/bip32 (k256)
                        time:   [9.5383 µs 9.6208 µs 9.7104 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.5187 µs 9.5900 µs 9.6686 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.6266 µs 9.7398 µs 9.9000 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.5780 µs 9.6362 µs 9.7008 µs]
```
