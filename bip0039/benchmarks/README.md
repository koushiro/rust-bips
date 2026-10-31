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
                        time:   [487.75 ns 496.69 ns 514.78 ns]
generate/bip39 (12 words)
                        time:   [1.5623 µs 1.5865 µs 1.6301 µs]
generate/coins-bip39 (12 words)
                        time:   [695.24 µs 696.25 µs 697.32 µs]
generate/bip0039 (12 words)
                        time:   [310.68 ns 311.72 ns 312.88 ns]

generate/tiny-bip39 (15 words)
                        time:   [554.29 ns 555.32 ns 556.35 ns]
generate/bip39 (15 words)
                        time:   [1.6017 µs 1.6054 µs 1.6097 µs]
generate/coins-bip39 (15 words)
                        time:   [870.99 µs 884.36 µs 911.04 µs]
generate/bip0039 (15 words)
                        time:   [363.71 ns 364.65 ns 365.67 ns]

generate/tiny-bip39 (18 words)
                        time:   [609.54 ns 611.15 ns 612.96 ns]
generate/bip39 (18 words)
                        time:   [1.6454 µs 1.6631 µs 1.6934 µs]
generate/coins-bip39 (18 words)
                        time:   [1.0427 ms 1.0469 ms 1.0524 ms]
generate/bip0039 (18 words)
                        time:   [406.00 ns 407.46 ns 409.22 ns]

generate/tiny-bip39 (24 words)
                        time:   [734.24 ns 742.27 ns 755.36 ns]
generate/bip39 (24 words)
                        time:   [1.7945 µs 1.8200 µs 1.8691 µs]
generate/coins-bip39 (24 words)
                        time:   [1.3923 ms 1.3970 ms 1.4030 ms]
generate/bip0039 (24 words)
                        time:   [515.29 ns 516.20 ns 517.08 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [505.04 ns 507.53 ns 510.03 ns]
from_entropy/bip39 (12 words)
                        time:   [632.04 ns 675.98 ns 725.24 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [705.00 µs 709.26 µs 715.03 µs]
from_entropy/bip0039 (12 words)
                        time:   [305.07 ns 312.27 ns 324.23 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [564.27 ns 566.03 ns 567.94 ns]
from_entropy/bip39 (15 words)
                        time:   [661.13 ns 672.29 ns 689.26 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [872.48 µs 875.29 µs 878.67 µs]
from_entropy/bip0039 (15 words)
                        time:   [361.12 ns 361.92 ns 362.78 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [615.50 ns 622.97 ns 633.88 ns]
from_entropy/bip39 (18 words)
                        time:   [706.72 ns 708.31 ns 709.96 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [1.0482 ms 1.0518 ms 1.0569 ms]
from_entropy/bip0039 (18 words)
                        time:   [405.55 ns 418.93 ns 442.14 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [726.16 ns 737.39 ns 753.59 ns]
from_entropy/bip39 (24 words)
                        time:   [867.76 ns 876.65 ns 894.28 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [1.3944 ms 1.3971 ms 1.3997 ms]
from_entropy/bip0039 (24 words)
                        time:   [498.78 ns 503.93 ns 510.80 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5576 µs 1.6082 µs 1.6854 µs]
from_phrase/bip39 (12 words)
                        time:   [2.0014 µs 2.0062 µs 2.0106 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [1.7176 ms 1.7626 ms 1.8228 ms]
from_phrase/bip0039 (12 words)
                        time:   [738.96 ns 741.05 ns 743.49 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.8218 µs 1.8821 µs 1.9770 µs]
from_phrase/bip39 (15 words)
                        time:   [2.4727 µs 2.4983 µs 2.5396 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [2.1131 ms 2.1172 ms 2.1221 ms]
from_phrase/bip0039 (15 words)
                        time:   [899.85 ns 902.41 ns 905.44 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0721 µs 2.1466 µs 2.2608 µs]
from_phrase/bip39 (18 words)
                        time:   [2.8941 µs 2.9060 µs 2.9188 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [2.5294 ms 2.5373 ms 2.5472 ms]
from_phrase/bip0039 (18 words)
                        time:   [1.0518 µs 1.0543 µs 1.0572 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.7292 µs 2.8227 µs 2.9640 µs]
from_phrase/bip39 (24 words)
                        time:   [3.7201 µs 3.7322 µs 3.7455 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [3.5340 ms 3.6473 ms 3.7735 ms]
from_phrase/bip0039 (24 words)
                        time:   [1.4115 µs 1.4301 µs 1.4556 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9490 µs 1.9778 µs 2.0121 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [690.59 ns 694.31 ns 700.06 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.4779 µs 2.5810 µs 2.7133 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [845.91 ns 857.62 ns 874.02 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.7777 µs 2.7895 µs 2.8038 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [974.63 ns 990.24 ns 1.0197 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.5801 µs 3.5949 µs 3.6172 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.2816 µs 1.2860 µs 1.2905 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [967.10 µs 973.45 µs 985.76 µs]
to_seed/bip39 (12 words)
                        time:   [1.0821 ms 1.0844 ms 1.0872 ms]
to_seed/coins-bip39 (12 words)
                        time:   [1.1769 ms 1.1918 ms 1.2177 ms]
to_seed/bip0039 (12 words)
                        time:   [458.02 µs 459.49 µs 461.19 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [968.98 µs 971.52 µs 974.15 µs]
to_seed/bip39 (15 words)
                        time:   [1.0814 ms 1.0899 ms 1.1066 ms]
to_seed/coins-bip39 (15 words)
                        time:   [1.3536 ms 1.3561 ms 1.3592 ms]
to_seed/bip0039 (15 words)
                        time:   [456.71 µs 458.59 µs 461.04 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [964.14 µs 966.63 µs 969.93 µs]
to_seed/bip39 (18 words)
                        time:   [1.0770 ms 1.0788 ms 1.0806 ms]
to_seed/coins-bip39 (18 words)
                        time:   [1.5333 ms 1.5522 ms 1.5866 ms]
to_seed/bip0039 (18 words)
                        time:   [458.37 µs 460.56 µs 463.10 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [970.38 µs 972.17 µs 974.17 µs]
to_seed/bip39 (24 words)
                        time:   [1.0796 ms 1.0815 ms 1.0836 ms]
to_seed/coins-bip39 (24 words)
                        time:   [1.8936 ms 1.8966 ms 1.9008 ms]
to_seed/bip0039 (24 words)
                        time:   [456.38 µs 457.25 µs 458.12 µs]
```
