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
                        time:   [1.0648 µs 1.0667 µs 1.0688 µs]
keygen/coins-bip32 (k256::ecdsa)
                        time:   [43.217 µs 44.112 µs 45.681 µs]
keygen/bip32 (k256)     time:   [998.93 ns 1.0014 µs 1.0042 µs]
keygen/bip32 (k256::ecdsa)
                        time:   [43.075 µs 43.165 µs 43.255 µs]
keygen/bip0032 (k256)   time:   [1.0135 µs 1.0169 µs 1.0208 µs]
keygen/bip0032 (secp256k1)
                        time:   [1.0212 µs 1.0283 µs 1.0402 µs]
keygen/bip0032 (libsecp256k1)
                        time:   [1.0843 µs 1.1091 µs 1.1497 µs]
```

## Derivation

```bash
cargo bench --bench derive -- --quiet
# Or just bench derive
```

```text
derive/bitcoin (secp256k1)
                        time:   [138.67 µs 138.97 µs 139.31 µs]
derive/coins-bip32 (k256::ecdsa)
                        time:   [261.19 µs 261.81 µs 262.48 µs]
derive/bip32 (k256)     time:   [345.40 µs 348.09 µs 351.97 µs]
derive/bip32 (k256::ecdsa)
                        time:   [218.42 µs 219.08 µs 219.81 µs]
derive/bip0032 (k256)   time:   [218.20 µs 218.86 µs 219.54 µs]
derive/bip0032 (secp256k1)
                        time:   [76.739 µs 76.929 µs 77.148 µs]
derive/bip0032 (libsecp256k1)
                        time:   [169.77 µs 170.30 µs 170.92 µs]
```

## Serialization

### xprv decode

```bash
cargo bench --bench xprv_decode -- --quiet
# Or just bench xprv_decode
```

```text
xprv_decode/bitcoin (secp256k1)
                        time:   [10.047 µs 10.222 µs 10.516 µs]
xprv_decode/coins-bip32 (k256::ecdsa)
                        time:   [47.813 µs 49.163 µs 51.496 µs]
xprv_decode/bip32 (k256)
                        time:   [5.4437 µs 5.4713 µs 5.5023 µs]
xprv_decode/bip32 (k256::ecdsa)
                        time:   [47.540 µs 47.661 µs 47.781 µs]
xprv_decode/bip0032 (k256)
                        time:   [5.4782 µs 5.4909 µs 5.5046 µs]
xprv_decode/bip0032 (secp256k1)
                        time:   [5.4897 µs 5.5100 µs 5.5360 µs]
xprv_decode/bip0032 (libsecp256k1)
                        time:   [5.5588 µs 5.6191 µs 5.7048 µs]
```

### xprv encode

```bash
cargo bench --bench xprv_encode -- --quiet
# Or just bench xprv_encode
```

```text
xprv_encode/bitcoin (secp256k1)
                        time:   [9.6542 µs 9.6834 µs 9.7183 µs]
xprv_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.3697 µs 9.5882 µs 9.8989 µs]
xprv_encode/bip32 (k256)
                        time:   [9.4330 µs 9.4547 µs 9.4785 µs]
xprv_encode/bip32 (k256::ecdsa)
                        time:   [9.4844 µs 9.8837 µs 10.475 µs]
xprv_encode/bip0032 (k256)
                        time:   [9.4586 µs 9.5561 µs 9.6590 µs]
xprv_encode/bip0032 (secp256k1)
                        time:   [9.3629 µs 9.5844 µs 9.9966 µs]
xprv_encode/bip0032 (libsecp256k1)
                        time:   [9.3425 µs 9.3703 µs 9.3987 µs]
```

### xpub decode

```bash
cargo bench --bench xpub_decode -- --quiet
# Or just bench xpub_decode
```

```text
xpub_decode/bitcoin (secp256k1)
                        time:   [13.959 µs 13.991 µs 14.025 µs]
xpub_decode/coins-bip32 (k256::ecdsa)
                        time:   [10.813 µs 10.924 µs 11.067 µs]
xpub_decode/bip32 (k256)
                        time:   [10.703 µs 10.739 µs 10.777 µs]
xpub_decode/bip32 (k256::ecdsa)
                        time:   [10.988 µs 11.293 µs 11.717 µs]
xpub_decode/bip0032 (k256)
                        time:   [10.819 µs 10.914 µs 11.033 µs]
xpub_decode/bip0032 (secp256k1)
                        time:   [9.4395 µs 9.6606 µs 10.035 µs]
xpub_decode/bip0032 (libsecp256k1)
                        time:   [13.243 µs 13.495 µs 13.934 µs]
```

### xpub encode

```bash
cargo bench --bench xpub_encode -- --quiet
# Or just bench xpub_encode
```

```text
xpub_encode/bitcoin (secp256k1)
                        time:   [10.393 µs 10.859 µs 11.493 µs]
xpub_encode/coins-bip32 (k256::ecdsa)
                        time:   [9.4440 µs 9.5008 µs 9.5660 µs]
xpub_encode/bip32 (k256)
                        time:   [10.011 µs 10.280 µs 10.576 µs]
xpub_encode/bip32 (k256::ecdsa)
                        time:   [9.5320 µs 9.5713 µs 9.6128 µs]
xpub_encode/bip0032 (k256)
                        time:   [9.5190 µs 9.5649 µs 9.6153 µs]
xpub_encode/bip0032 (secp256k1)
                        time:   [9.4779 µs 9.5268 µs 9.5832 µs]
xpub_encode/bip0032 (libsecp256k1)
                        time:   [9.7771 µs 10.185 µs 10.745 µs]
```
