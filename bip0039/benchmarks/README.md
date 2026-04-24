# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.95.0 (59807616e 2026-04-14)

## generate

```bash
cargo bench --bench generate -- --quiet
# Or just bench generate
```

```text
generate/tiny-bip39 (12 words)
                        time:   [502.53 ns 503.30 ns 504.31 ns]
generate/bip39 (12 words)
                        time:   [585.60 ns 586.52 ns 587.73 ns]
generate/coins-bip39 (12 words)
                        time:   [655.83 ns 656.67 ns 657.54 ns]
generate/bip0039 (12 words)
                        time:   [311.02 ns 311.59 ns 312.35 ns]

generate/tiny-bip39 (15 words)
                        time:   [588.04 ns 588.88 ns 589.72 ns]
generate/bip39 (15 words)
                        time:   [643.20 ns 643.93 ns 644.74 ns]
generate/coins-bip39 (15 words)
                        time:   [728.49 ns 731.57 ns 736.21 ns]
generate/bip0039 (15 words)
                        time:   [384.97 ns 385.44 ns 385.97 ns]

generate/tiny-bip39 (18 words)
                        time:   [659.10 ns 659.89 ns 660.72 ns]
generate/bip39 (18 words)
                        time:   [689.01 ns 689.99 ns 691.27 ns]
generate/coins-bip39 (18 words)
                        time:   [816.87 ns 818.65 ns 820.70 ns]
generate/bip0039 (18 words)
                        time:   [448.15 ns 450.73 ns 453.81 ns]

generate/tiny-bip39 (24 words)
                        time:   [794.98 ns 796.06 ns 797.56 ns]
generate/bip39 (24 words)
                        time:   [848.14 ns 849.00 ns 849.92 ns]
generate/coins-bip39 (24 words)
                        time:   [980.40 ns 981.73 ns 983.37 ns]
generate/bip0039 (24 words)
                        time:   [569.09 ns 569.74 ns 570.48 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [492.76 ns 493.65 ns 494.80 ns]
from_entropy/bip39 (12 words)
                        time:   [560.46 ns 561.16 ns 561.84 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [523.66 ns 524.32 ns 525.10 ns]
from_entropy/bip0039 (12 words)
                        time:   [310.03 ns 310.36 ns 310.78 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [565.51 ns 566.12 ns 566.85 ns]
from_entropy/bip39 (15 words)
                        time:   [610.56 ns 611.45 ns 612.28 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [574.96 ns 575.60 ns 576.33 ns]
from_entropy/bip0039 (15 words)
                        time:   [379.04 ns 379.57 ns 380.08 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [625.62 ns 626.45 ns 627.42 ns]
from_entropy/bip39 (18 words)
                        time:   [657.64 ns 658.72 ns 659.73 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [633.82 ns 635.12 ns 636.77 ns]
from_entropy/bip0039 (18 words)
                        time:   [428.85 ns 429.42 ns 430.01 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [760.84 ns 762.06 ns 763.56 ns]
from_entropy/bip39 (24 words)
                        time:   [803.60 ns 805.54 ns 807.44 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [749.43 ns 755.17 ns 763.06 ns]
from_entropy/bip0039 (24 words)
                        time:   [552.53 ns 553.40 ns 554.36 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5368 µs 1.5392 µs 1.5413 µs]
from_phrase/bip39 (12 words)
                        time:   [3.2115 µs 3.2169 µs 3.2231 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.8701 µs 2.8756 µs 2.8820 µs]
from_phrase/bip0039 (12 words)
                        time:   [749.89 ns 750.62 ns 751.37 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.7845 µs 1.7960 µs 1.8101 µs]
from_phrase/bip39 (15 words)
                        time:   [3.9503 µs 3.9595 µs 3.9690 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [3.4371 µs 3.4451 µs 3.4528 µs]
from_phrase/bip0039 (15 words)
                        time:   [924.64 ns 925.59 ns 926.49 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0260 µs 2.0300 µs 2.0349 µs]
from_phrase/bip39 (18 words)
                        time:   [4.6605 µs 4.6681 µs 4.6755 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [4.1174 µs 4.1465 µs 4.1856 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.0875 µs 1.0903 µs 1.0935 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6444 µs 2.6484 µs 2.6529 µs]
from_phrase/bip39 (24 words)
                        time:   [6.0882 µs 6.1023 µs 6.1191 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [5.2840 µs 5.3027 µs 5.3281 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.4256 µs 1.4278 µs 1.4303 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9433 µs 1.9460 µs 1.9492 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [691.75 ns 692.74 ns 693.79 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.3817 µs 2.3855 µs 2.3899 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [849.60 ns 850.50 ns 851.50 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.8021 µs 2.8040 µs 2.8063 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [998.14 ns 1.0005 µs 1.0033 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.6450 µs 3.6490 µs 3.6534 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.3141 µs 1.3164 µs 1.3193 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [973.25 µs 974.24 µs 975.21 µs]
to_seed/bip39 (12 words)
                        time:   [1.0909 ms 1.0960 ms 1.1018 ms]
to_seed/coins-bip39 (12 words)
                        time:   [972.41 µs 973.45 µs 974.86 µs]
to_seed/bip0039 (12 words)
                        time:   [412.67 µs 413.38 µs 414.53 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [971.26 µs 972.64 µs 974.46 µs]
to_seed/bip39 (15 words)
                        time:   [1.0882 ms 1.0899 ms 1.0919 ms]
to_seed/coins-bip39 (15 words)
                        time:   [973.03 µs 973.96 µs 974.76 µs]
to_seed/bip0039 (15 words)
                        time:   [412.83 µs 414.96 µs 418.27 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [972.53 µs 973.77 µs 975.36 µs]
to_seed/bip39 (18 words)
                        time:   [1.0881 ms 1.0896 ms 1.0912 ms]
to_seed/coins-bip39 (18 words)
                        time:   [973.74 µs 975.20 µs 976.97 µs]
to_seed/bip0039 (18 words)
                        time:   [411.89 µs 412.39 µs 413.00 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [971.10 µs 972.42 µs 974.00 µs]
to_seed/bip39 (24 words)
                        time:   [1.0863 ms 1.0904 ms 1.0973 ms]
to_seed/coins-bip39 (24 words)
                        time:   [971.75 µs 972.64 µs 973.82 µs]
to_seed/bip0039 (24 words)
                        time:   [412.49 µs 412.82 µs 413.23 µs]
```
