# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## bip0039

```bash
cargo bench --bench bip0039 -- --quiet
```

```text
generate/tiny-bip39     time:   [522.95 ns 523.86 ns 524.79 ns]
generate/bip39          time:   [587.87 ns 589.02 ns 590.20 ns]
generate/coins-bip39    time:   [665.02 ns 691.86 ns 723.01 ns]
generate/bip0039        time:   [659.25 ns 660.11 ns 661.10 ns]

from_entropy/tiny-bip39 time:   [406.60 ns 407.36 ns 408.14 ns]
from_entropy/bip39      time:   [241.30 ns 242.55 ns 244.87 ns]
from_entropy/coins-bip39
                        time:   [487.28 ps 490.99 ps 498.07 ps]
from_entropy/bip0039    time:   [404.98 ns 406.86 ns 409.21 ns]

from_phrase/tiny-bip39  time:   [1.2218 µs 1.2233 µs 1.2249 µs]
from_phrase/bip39       time:   [1.2177 µs 1.2205 µs 1.2237 µs]
from_phrase/coins-bip39 time:   [2.3764 µs 2.3815 µs 2.3863 µs]
from_phrase/bip0039     time:   [953.19 ns 955.01 ns 957.02 ns]

to_seed/tiny-bip39      time:   [975.05 µs 976.48 µs 978.08 µs]
to_seed/bip39           time:   [1.0783 ms 1.0798 ms 1.0813 ms]
to_seed/coins-bip39     time:   [974.05 µs 975.17 µs 976.39 µs]
to_seed/bip0039         time:   [980.59 µs 990.79 µs 1.0071 ms]
```
