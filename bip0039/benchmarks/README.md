# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## bip0039

```bash
cargo bench --bench bip0039 -- --quiet
```

```text
generate/tiny-bip39     time:   [527.36 ns 529.05 ns 530.98 ns]
generate/coins-bip39    time:   [644.87 ns 646.28 ns 647.96 ns]
generate/bip39          time:   [593.55 ns 594.89 ns 596.57 ns]
generate/bip0039        time:   [661.24 ns 663.00 ns 665.20 ns]

from_entropy/tiny-bip39 time:   [411.69 ns 412.30 ns 412.91 ns]
from_entropy/bip39      time:   [242.64 ns 242.96 ns 243.27 ns]
from_entropy/coins-bip39
                        time:   [493.04 ps 494.29 ps 495.46 ps]
from_entropy/bip0039    time:   [410.42 ns 411.94 ns 413.48 ns]

from_phrase/tiny-bip39  time:   [1.2370 µs 1.2409 µs 1.2446 µs]
from_phrase/bip39       time:   [1.2458 µs 1.2474 µs 1.2493 µs]
from_phrase/coins-bip39 time:   [2.4056 µs 2.4099 µs 2.4147 µs]
from_phrase/bip0039     time:   [1.5468 µs 1.5506 µs 1.5550 µs]

to_seed/tiny-bip39      time:   [979.66 µs 981.75 µs 984.18 µs]
to_seed/bip39           time:   [1.0900 ms 1.0919 ms 1.0939 ms]
to_seed/coins-bip39     time:   [984.12 µs 986.20 µs 989.16 µs]
to_seed/bip0039         time:   [987.35 µs 989.50 µs 991.54 µs]
```
