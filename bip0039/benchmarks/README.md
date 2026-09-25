# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.98.1 (48a229cea 2026-09-01)

## generate

```bash
cargo bench --bench generate -- --quiet
# Or just bench generate
```

```text
generate/tiny-bip39 (12 words)
                        time:   [515.39 ns 528.93 ns 547.92 ns]
generate/bip39 (12 words)
                        time:   [1.5167 µs 1.5220 µs 1.5285 µs]
generate/coins-bip39 (12 words)
                        time:   [643.24 ns 651.62 ns 664.25 ns]
generate/bip0039 (12 words)
                        time:   [318.17 ns 318.81 ns 319.42 ns]

generate/tiny-bip39 (15 words)
                        time:   [581.55 ns 582.87 ns 584.21 ns]
generate/bip39 (15 words)
                        time:   [1.5611 µs 1.5893 µs 1.6321 µs]
generate/coins-bip39 (15 words)
                        time:   [705.58 ns 707.04 ns 708.77 ns]
generate/bip0039 (15 words)
                        time:   [386.39 ns 387.03 ns 387.71 ns]

generate/tiny-bip39 (18 words)
                        time:   [644.96 ns 652.31 ns 666.92 ns]
generate/bip39 (18 words)
                        time:   [1.6007 µs 1.6034 µs 1.6063 µs]
generate/coins-bip39 (18 words)
                        time:   [802.68 ns 809.41 ns 822.74 ns]
generate/bip0039 (18 words)
                        time:   [440.43 ns 450.48 ns 465.81 ns]

generate/tiny-bip39 (24 words)
                        time:   [795.30 ns 798.04 ns 801.40 ns]
generate/bip39 (24 words)
                        time:   [1.7482 µs 1.7528 µs 1.7580 µs]
generate/coins-bip39 (24 words)
                        time:   [943.59 ns 955.83 ns 979.74 ns]
generate/bip0039 (24 words)
                        time:   [568.66 ns 573.07 ns 581.81 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [498.45 ns 502.18 ns 508.70 ns]
from_entropy/bip39 (12 words)
                        time:   [573.57 ns 575.24 ns 577.23 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [541.39 ns 552.72 ns 577.47 ns]
from_entropy/bip0039 (12 words)
                        time:   [319.11 ns 323.65 ns 330.57 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [574.66 ns 576.15 ns 577.82 ns]
from_entropy/bip39 (15 words)
                        time:   [626.76 ns 627.94 ns 629.14 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [593.58 ns 594.71 ns 595.80 ns]
from_entropy/bip0039 (15 words)
                        time:   [390.52 ns 394.13 ns 399.18 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [629.12 ns 634.68 ns 645.91 ns]
from_entropy/bip39 (18 words)
                        time:   [675.97 ns 685.59 ns 703.89 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [662.53 ns 664.63 ns 667.52 ns]
from_entropy/bip0039 (18 words)
                        time:   [436.53 ns 437.38 ns 438.33 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [757.67 ns 770.60 ns 789.39 ns]
from_entropy/bip39 (24 words)
                        time:   [821.44 ns 822.70 ns 824.02 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [772.45 ns 776.19 ns 780.71 ns]
from_entropy/bip0039 (24 words)
                        time:   [564.07 ns 589.92 ns 627.27 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5196 µs 1.5415 µs 1.5787 µs]
from_phrase/bip39 (12 words)
                        time:   [2.0819 µs 2.1226 µs 2.1971 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.9320 µs 2.9473 µs 2.9680 µs]
from_phrase/bip0039 (12 words)
                        time:   [774.72 ns 787.86 ns 805.41 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.7643 µs 1.7757 µs 1.7902 µs]
from_phrase/bip39 (15 words)
                        time:   [2.5457 µs 2.5568 µs 2.5687 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [3.4953 µs 3.5556 µs 3.6491 µs]
from_phrase/bip0039 (15 words)
                        time:   [950.28 ns 953.86 ns 957.99 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0111 µs 2.0206 µs 2.0327 µs]
from_phrase/bip39 (18 words)
                        time:   [3.0086 µs 3.0392 µs 3.0897 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [4.2136 µs 4.2336 µs 4.2565 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.1296 µs 1.1540 µs 1.1901 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6304 µs 2.6975 µs 2.8025 µs]
from_phrase/bip39 (24 words)
                        time:   [3.8885 µs 3.9063 µs 3.9290 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [5.3827 µs 5.4383 µs 5.5231 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.4875 µs 1.4970 µs 1.5064 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9966 µs 2.0049 µs 2.0149 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [720.54 ns 728.83 ns 740.52 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.4290 µs 2.4403 µs 2.4533 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [877.30 ns 885.08 ns 898.27 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.8823 µs 2.9203 µs 2.9705 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [1.0412 µs 1.0482 µs 1.0567 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.7389 µs 3.7568 µs 3.7784 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.3613 µs 1.3903 µs 1.4415 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [977.76 µs 979.82 µs 981.89 µs]
to_seed/bip39 (12 words)
                        time:   [1.1274 ms 1.1347 ms 1.1477 ms]
to_seed/coins-bip39 (12 words)
                        time:   [977.84 µs 995.52 µs 1.0317 ms]
to_seed/bip0039 (12 words)
                        time:   [417.67 µs 418.66 µs 419.67 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [981.20 µs 992.69 µs 1.0122 ms]
to_seed/bip39 (15 words)
                        time:   [1.1276 ms 1.1414 ms 1.1665 ms]
to_seed/coins-bip39 (15 words)
                        time:   [981.44 µs 983.31 µs 985.27 µs]
to_seed/bip0039 (15 words)
                        time:   [418.22 µs 421.98 µs 428.96 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [980.96 µs 992.69 µs 1.0171 ms]
to_seed/bip39 (18 words)
                        time:   [1.1252 ms 1.1289 ms 1.1340 ms]
to_seed/coins-bip39 (18 words)
                        time:   [972.05 µs 974.17 µs 976.34 µs]
to_seed/bip0039 (18 words)
                        time:   [413.48 µs 418.94 µs 429.89 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [968.91 µs 970.09 µs 971.32 µs]
to_seed/bip39 (24 words)
                        time:   [1.1156 ms 1.1225 ms 1.1363 ms]
to_seed/coins-bip39 (24 words)
                        time:   [970.34 µs 981.80 µs 1.0012 ms]
to_seed/bip0039 (24 words)
                        time:   [412.84 µs 415.05 µs 419.05 µs]
```
