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
                        time:   [514.64 ns 522.07 ns 531.46 ns]
generate/bip39 (12 words)
                        time:   [1.6379 µs 1.7240 µs 1.8205 µs]
generate/coins-bip39 (12 words)
                        time:   [719.23 µs 720.65 µs 722.02 µs]
generate/bip0039 (12 words)
                        time:   [312.86 ns 316.63 ns 323.17 ns]

generate/tiny-bip39 (15 words)
                        time:   [554.81 ns 559.83 ns 568.43 ns]
generate/bip39 (15 words)
                        time:   [1.6082 µs 1.6152 µs 1.6244 µs]
generate/coins-bip39 (15 words)
                        time:   [900.94 µs 904.34 µs 909.27 µs]
generate/bip0039 (15 words)
                        time:   [367.49 ns 368.81 ns 370.17 ns]

generate/tiny-bip39 (18 words)
                        time:   [614.19 ns 616.14 ns 618.02 ns]
generate/bip39 (18 words)
                        time:   [1.6521 µs 1.6714 µs 1.7067 µs]
generate/coins-bip39 (18 words)
                        time:   [1.0806 ms 1.0824 ms 1.0845 ms]
generate/bip0039 (18 words)
                        time:   [411.78 ns 418.61 ns 429.96 ns]

generate/tiny-bip39 (24 words)
                        time:   [739.40 ns 741.77 ns 744.72 ns]
generate/bip39 (24 words)
                        time:   [1.8042 µs 1.8104 µs 1.8174 µs]
generate/coins-bip39 (24 words)
                        time:   [1.4402 ms 1.4438 ms 1.4475 ms]
generate/bip0039 (24 words)
                        time:   [519.20 ns 520.55 ns 521.95 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [488.11 ns 492.23 ns 497.50 ns]
from_entropy/bip39 (12 words)
                        time:   [616.22 ns 619.91 ns 625.66 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [718.63 µs 719.91 µs 721.41 µs]
from_entropy/bip0039 (12 words)
                        time:   [308.69 ns 314.00 ns 322.16 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [546.63 ns 548.30 ns 550.06 ns]
from_entropy/bip39 (15 words)
                        time:   [667.49 ns 670.29 ns 674.54 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [900.45 µs 908.99 µs 925.77 µs]
from_entropy/bip0039 (15 words)
                        time:   [366.70 ns 368.05 ns 369.38 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [598.79 ns 600.31 ns 602.10 ns]
from_entropy/bip39 (18 words)
                        time:   [717.63 ns 720.09 ns 723.01 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [1.0850 ms 1.0900 ms 1.0961 ms]
from_entropy/bip0039 (18 words)
                        time:   [412.61 ns 417.85 ns 426.02 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [726.80 ns 736.56 ns 752.37 ns]
from_entropy/bip39 (24 words)
                        time:   [877.97 ns 880.68 ns 883.58 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [1.4397 ms 1.4433 ms 1.4474 ms]
from_entropy/bip0039 (24 words)
                        time:   [515.11 ns 520.96 ns 529.87 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5478 µs 1.5531 µs 1.5586 µs]
from_phrase/bip39 (12 words)
                        time:   [2.0008 µs 2.0169 µs 2.0370 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [1.6925 ms 1.7051 ms 1.7242 ms]
from_phrase/bip0039 (12 words)
                        time:   [745.89 ns 749.98 ns 754.29 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.8002 µs 1.8141 µs 1.8356 µs]
from_phrase/bip39 (15 words)
                        time:   [2.4639 µs 2.4743 µs 2.4859 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [2.1265 ms 2.1471 ms 2.1793 ms]
from_phrase/bip0039 (15 words)
                        time:   [901.03 ns 903.92 ns 907.13 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0624 µs 2.1266 µs 2.2174 µs]
from_phrase/bip39 (18 words)
                        time:   [2.9114 µs 2.9938 µs 3.1651 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [2.5546 ms 2.5772 ms 2.6129 ms]
from_phrase/bip0039 (18 words)
                        time:   [1.0707 µs 1.0800 µs 1.0911 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6592 µs 2.6922 µs 2.7427 µs]
from_phrase/bip39 (24 words)
                        time:   [3.7147 µs 3.7344 µs 3.7551 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [3.4162 ms 3.4500 ms 3.4998 ms]
from_phrase/bip0039 (24 words)
                        time:   [1.3794 µs 1.3853 µs 1.3923 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.8928 µs 1.8989 µs 1.9055 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [686.03 ns 695.27 ns 707.47 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.3316 µs 2.3482 µs 2.3677 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [832.49 ns 836.58 ns 841.92 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.7424 µs 2.7544 µs 2.7674 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [981.09 ns 992.49 ns 1.0110 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.6033 µs 3.6186 µs 3.6356 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.2917 µs 1.3001 µs 1.3131 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [970.41 µs 972.44 µs 974.60 µs]
to_seed/bip39 (12 words)
                        time:   [1.0816 ms 1.0891 ms 1.1003 ms]
to_seed/coins-bip39 (12 words)
                        time:   [1.1490 ms 1.1596 ms 1.1795 ms]
to_seed/bip0039 (12 words)
                        time:   [457.65 µs 458.97 µs 460.24 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [970.50 µs 972.52 µs 974.72 µs]
to_seed/bip39 (15 words)
                        time:   [1.0817 ms 1.0845 ms 1.0879 ms]
to_seed/coins-bip39 (15 words)
                        time:   [1.3227 ms 1.3276 ms 1.3335 ms]
to_seed/bip0039 (15 words)
                        time:   [457.54 µs 458.40 µs 459.30 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [968.53 µs 970.61 µs 972.99 µs]
to_seed/bip39 (18 words)
                        time:   [1.0802 ms 1.0824 ms 1.0852 ms]
to_seed/coins-bip39 (18 words)
                        time:   [1.4935 ms 1.4986 ms 1.5048 ms]
to_seed/bip0039 (18 words)
                        time:   [457.73 µs 458.76 µs 459.74 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [968.94 µs 970.94 µs 973.23 µs]
to_seed/bip39 (24 words)
                        time:   [1.0821 ms 1.0847 ms 1.0878 ms]
to_seed/coins-bip39 (24 words)
                        time:   [1.8365 ms 1.8393 ms 1.8424 ms]
to_seed/bip0039 (24 words)
                        time:   [457.88 µs 462.89 µs 472.78 µs]
```
