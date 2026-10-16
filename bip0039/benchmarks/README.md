# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13)

## generate

```bash
cargo bench --bench generate -- --quiet
# Or just bench generate
```

```text
generate/tiny-bip39 (12 words)
                        time:   [523.59 ns 524.50 ns 525.71 ns]
generate/bip39 (12 words)
                        time:   [1.5540 µs 1.5558 µs 1.5576 µs]
generate/coins-bip39 (12 words)
                        time:   [726.52 µs 728.22 µs 730.16 µs]
generate/bip0039 (12 words)
                        time:   [341.26 ns 342.33 ns 343.60 ns]

generate/tiny-bip39 (15 words)
                        time:   [604.25 ns 619.77 ns 648.99 ns]
generate/bip39 (15 words)
                        time:   [1.6137 µs 1.6212 µs 1.6314 µs]
generate/coins-bip39 (15 words)
                        time:   [908.87 µs 912.40 µs 916.67 µs]
generate/bip0039 (15 words)
                        time:   [410.52 ns 412.44 ns 414.31 ns]

generate/tiny-bip39 (18 words)
                        time:   [663.81 ns 668.18 ns 674.12 ns]
generate/bip39 (18 words)
                        time:   [1.6627 µs 1.6721 µs 1.6829 µs]
generate/coins-bip39 (18 words)
                        time:   [1.0872 ms 1.0902 ms 1.0934 ms]
generate/bip0039 (18 words)
                        time:   [461.72 ns 465.73 ns 471.93 ns]

generate/tiny-bip39 (24 words)
                        time:   [812.13 ns 814.46 ns 817.03 ns]
generate/bip39 (24 words)
                        time:   [1.8060 µs 1.8133 µs 1.8212 µs]
generate/coins-bip39 (24 words)
                        time:   [1.4543 ms 1.4585 ms 1.4634 ms]
generate/bip0039 (24 words)
                        time:   [591.94 ns 594.03 ns 596.34 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [518.84 ns 520.80 ns 522.91 ns]
from_entropy/bip39 (12 words)
                        time:   [614.78 ns 621.05 ns 631.25 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [724.39 µs 726.90 µs 730.19 µs]
from_entropy/bip0039 (12 words)
                        time:   [340.42 ns 341.66 ns 343.00 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [592.58 ns 594.24 ns 596.44 ns]
from_entropy/bip39 (15 words)
                        time:   [663.83 ns 667.19 ns 671.91 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [904.16 µs 905.83 µs 907.69 µs]
from_entropy/bip0039 (15 words)
                        time:   [404.65 ns 407.57 ns 412.63 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [644.93 ns 646.83 ns 648.94 ns]
from_entropy/bip39 (18 words)
                        time:   [707.15 ns 722.63 ns 752.99 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [1.0879 ms 1.0918 ms 1.0962 ms]
from_entropy/bip0039 (18 words)
                        time:   [456.64 ns 458.06 ns 459.59 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [774.94 ns 780.52 ns 789.05 ns]
from_entropy/bip39 (24 words)
                        time:   [872.75 ns 875.24 ns 877.97 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [1.4522 ms 1.4635 ms 1.4821 ms]
from_entropy/bip0039 (24 words)
                        time:   [567.81 ns 568.97 ns 570.33 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5675 µs 1.5737 µs 1.5817 µs]
from_phrase/bip39 (12 words)
                        time:   [2.0082 µs 2.0135 µs 2.0202 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [1.6889 ms 1.6942 ms 1.7016 ms]
from_phrase/bip0039 (12 words)
                        time:   [777.33 ns 778.93 ns 780.74 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.8220 µs 1.8404 µs 1.8654 µs]
from_phrase/bip39 (15 words)
                        time:   [2.4656 µs 2.4783 µs 2.4931 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [2.1241 ms 2.1606 ms 2.2289 ms]
from_phrase/bip0039 (15 words)
                        time:   [952.73 ns 957.80 ns 966.29 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0556 µs 2.0662 µs 2.0826 µs]
from_phrase/bip39 (18 words)
                        time:   [2.9061 µs 2.9539 µs 3.0314 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [2.5123 ms 2.5294 ms 2.5574 ms]
from_phrase/bip0039 (18 words)
                        time:   [1.0970 µs 1.1045 µs 1.1164 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6467 µs 2.6578 µs 2.6689 µs]
from_phrase/bip39 (24 words)
                        time:   [3.6906 µs 3.7371 µs 3.8265 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [3.3377 ms 3.3448 ms 3.3520 ms]
from_phrase/bip0039 (24 words)
                        time:   [1.4222 µs 1.4272 µs 1.4344 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9007 µs 1.9402 µs 1.9990 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [704.45 ns 706.89 ns 710.01 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.2996 µs 2.3055 µs 2.3114 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [884.84 ns 894.36 ns 907.62 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.7791 µs 2.7876 µs 2.7982 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [1.0290 µs 1.0312 µs 1.0341 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.6202 µs 3.6621 µs 3.7238 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.3564 µs 1.3694 µs 1.3920 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [977.20 µs 980.63 µs 985.20 µs]
to_seed/bip39 (12 words)
                        time:   [1.0918 ms 1.0954 ms 1.0992 ms]
to_seed/coins-bip39 (12 words)
                        time:   [1.1631 ms 1.1666 ms 1.1705 ms]
to_seed/bip0039 (12 words)
                        time:   [463.57 µs 466.12 µs 469.75 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [975.90 µs 978.51 µs 982.07 µs]
to_seed/bip39 (15 words)
                        time:   [1.0904 ms 1.0958 ms 1.1020 ms]
to_seed/coins-bip39 (15 words)
                        time:   [1.3396 ms 1.3451 ms 1.3518 ms]
to_seed/bip0039 (15 words)
                        time:   [464.11 µs 465.92 µs 467.89 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [980.42 µs 986.43 µs 993.44 µs]
to_seed/bip39 (18 words)
                        time:   [1.0894 ms 1.0920 ms 1.0949 ms]
to_seed/coins-bip39 (18 words)
                        time:   [1.5161 ms 1.5211 ms 1.5269 ms]
to_seed/bip0039 (18 words)
                        time:   [464.60 µs 466.42 µs 468.69 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [978.29 µs 981.81 µs 985.51 µs]
to_seed/bip39 (24 words)
                        time:   [1.0917 ms 1.0971 ms 1.1058 ms]
to_seed/coins-bip39 (24 words)
                        time:   [1.8658 ms 1.8931 ms 1.9396 ms]
to_seed/bip0039 (24 words)
                        time:   [463.52 µs 464.72 µs 465.96 µs]
```
