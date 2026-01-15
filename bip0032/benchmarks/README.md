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
                        time:   [1.0536 µs 1.0552 µs 1.0569 µs]
keygen/coins-bip32 (k256)
                        time:   [42.808 µs 42.881 µs 42.959 µs]
keygen/bip32 (k256)     time:   [989.04 ns 989.97 ns 990.95 ns]
keygen/bip32 (k256::ecdsa)
                        time:   [42.690 µs 42.755 µs 42.829 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [137.46 µs 137.74 µs 138.08 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [259.79 µs 260.15 µs 260.59 µs]
derive/bip32 (k256)     time:   [341.58 µs 342.15 µs 342.80 µs]
derive/bip32 (k256::ecdsa)
                        time:   [226.02 µs 233.43 µs 243.35 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.020 µs 10.033 µs 10.046 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.354 µs 47.488 µs 47.648 µs]
xprv_decode/bip32 (k256)
                        time:   [5.3320 µs 5.3435 µs 5.3553 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.015 µs 47.132 µs 47.286 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.6704 µs 9.6960 µs 9.7261 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.5441 µs 9.5671 µs 9.5911 µs]
xprv_encode/bip32 (k256)
                        time:   [9.3745 µs 9.6120 µs 9.9728 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.3006 µs 9.3234 µs 9.3478 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [14.181 µs 14.484 µs 14.849 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.570 µs 10.593 µs 10.617 µs]
xpub_decode/bip32 (k256)
                        time:   [10.466 µs 10.488 µs 10.511 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.469 µs 10.495 µs 10.522 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [9.5311 µs 9.5600 µs 9.5939 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.4479 µs 9.4688 µs 9.4924 µs]
xpub_encode/bip32 (k256)
                        time:   [9.1272 µs 9.1897 µs 9.2837 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.1551 µs 9.1777 µs 9.2024 µs]
```
