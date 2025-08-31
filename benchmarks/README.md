# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.89.0 (29483883e 2025-08-04)

## bip0039

```bash
cargo bench --bench bip0039 -- --quiet
```

```text
generate/tiny-bip39     time:   [528.39 ns 531.82 ns 537.70 ns]
generate/coins-bip39    time:   [645.39 ns 650.44 ns 657.79 ns]
generate/bip0039        time:   [647.57 ns 648.70 ns 650.03 ns]

from_entropy/tiny-bip39 time:   [407.69 ns 409.55 ns 412.94 ns]
from_entropy/coins-bip39
                        time:   [486.93 ps 490.94 ps 497.31 ps]
from_entropy/bip0039    time:   [406.61 ns 410.27 ns 414.84 ns]

from_phrase/tiny-bip39  time:   [1.2294 µs 1.2386 µs 1.2555 µs]
from_phrase/coins-bip39 time:   [2.3634 µs 2.4598 µs 2.6273 µs]
from_phrase/bip0039     time:   [1.5290 µs 1.5372 µs 1.5520 µs]

to_seed/tiny-bip39      time:   [988.97 µs 1.0045 ms 1.0267 ms]
to_seed/coins-bip39     time:   [1.0036 ms 1.0422 ms 1.0986 ms]
to_seed/bip0039         time:   [993.45 µs 1.0023 ms 1.0141 ms]
```
