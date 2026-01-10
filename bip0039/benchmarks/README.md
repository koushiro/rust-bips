# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## bip0039

```bash
cargo bench --bench bip0039 -- --quiet
```

### generate

```text
generate/tiny-bip39 (12 words)
                        time:   [513.52 ns 517.16 ns 523.92 ns]
generate/bip39 (12 words)
                        time:   [593.69 ns 595.46 ns 597.45 ns]
generate/coins-bip39 (12 words)
                        time:   [675.79 ns 681.15 ns 690.57 ns]
generate/bip0039 (12 words)
                        time:   [488.52 ns 493.38 ns 502.23 ns]

generate/tiny-bip39 (15 words)
                        time:   [598.83 ns 604.95 ns 615.44 ns]
generate/bip39 (15 words)
                        time:   [639.48 ns 641.62 ns 643.90 ns]
generate/coins-bip39 (15 words)
                        time:   [742.49 ns 745.99 ns 750.82 ns]
generate/bip0039 (15 words)
                        time:   [563.52 ns 570.99 ns 584.11 ns]

generate/tiny-bip39 (18 words)
                        time:   [660.29 ns 662.70 ns 667.19 ns]
generate/bip39 (18 words)
                        time:   [684.78 ns 690.02 ns 700.12 ns]
generate/coins-bip39 (18 words)
                        time:   [823.31 ns 829.79 ns 842.34 ns]
generate/bip0039 (18 words)
                        time:   [623.42 ns 626.79 ns 631.22 ns]

generate/tiny-bip39 (24 words)
                        time:   [799.87 ns 809.91 ns 826.99 ns]
generate/bip39 (24 words)
                        time:   [844.91 ns 850.70 ns 859.77 ns]
generate/coins-bip39 (24 words)
                        time:   [974.55 ns 983.04 ns 998.05 ns]
generate/bip0039 (24 words)
                        time:   [751.34 ns 758.68 ns 773.09 ns]
```

### from_entropy

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [411.14 ns 412.43 ns 413.78 ns]
from_entropy/bip39 (12 words)
                        time:   [488.33 ns 493.71 ns 503.76 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [496.39 ns 497.99 ns 499.62 ns]
from_entropy/bip0039 (12 words)
                        time:   [371.39 ns 372.29 ns 373.27 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [468.05 ns 473.97 ns 481.46 ns]
from_entropy/bip39 (15 words)
                        time:   [518.23 ns 525.77 ns 538.56 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [533.37 ns 538.22 ns 546.35 ns]
from_entropy/bip0039 (15 words)
                        time:   [417.25 ns 421.79 ns 428.64 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [536.64 ns 541.24 ns 545.84 ns]
from_entropy/bip39 (18 words)
                        time:   [555.40 ns 562.66 ns 575.03 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [585.39 ns 589.64 ns 597.39 ns]
from_entropy/bip0039 (18 words)
                        time:   [475.26 ns 476.80 ns 478.58 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [644.08 ns 649.09 ns 657.70 ns]
from_entropy/bip39 (24 words)
                        time:   [656.49 ns 658.37 ns 660.42 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [683.60 ns 686.46 ns 690.00 ns]
from_entropy/bip0039 (24 words)
                        time:   [563.10 ns 567.99 ns 575.46 ns]
```

### from_phrase

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.2942 µs 1.3212 µs 1.3532 µs]
from_phrase/bip39 (12 words)
                        time:   [2.0757 µs 2.0948 µs 2.1194 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.4165 µs 2.4560 µs 2.5149 µs]
from_phrase/bip0039 (12 words)
                        time:   [741.51 ns 747.89 ns 756.25 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.5384 µs 1.5580 µs 1.5760 µs]
from_phrase/bip39 (15 words)
                        time:   [2.5566 µs 2.5865 µs 2.6230 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [2.8880 µs 2.9162 µs 2.9510 µs]
from_phrase/bip0039 (15 words)
                        time:   [863.76 ns 872.79 ns 881.13 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [1.7517 µs 1.7888 µs 1.8310 µs]
from_phrase/bip39 (18 words)
                        time:   [3.0339 µs 3.0630 µs 3.0953 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [3.4959 µs 3.5556 µs 3.6406 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.0766 µs 1.1184 µs 1.1624 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.3406 µs 2.3629 µs 2.3853 µs]
from_phrase/bip39 (24 words)
                        time:   [3.9380 µs 3.9804 µs 4.0311 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [4.5477 µs 4.5738 µs 4.6101 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.2758 µs 1.2834 µs 1.2914 µs]
```

### to_seed

```text
to_seed/tiny-bip39 (12 words)
                        time:   [981.02 µs 990.71 µs 1.0081 ms]
to_seed/bip39 (12 words)
                        time:   [1.0949 ms 1.1042 ms 1.1176 ms]
to_seed/coins-bip39 (12 words)
                        time:   [983.71 µs 987.11 µs 990.55 µs]
to_seed/bip0039 (12 words)
                        time:   [984.78 µs 994.06 µs 1.0107 ms]

to_seed/tiny-bip39 (15 words)
                        time:   [982.56 µs 987.60 µs 995.10 µs]
to_seed/bip39 (15 words)
                        time:   [1.0954 ms 1.1023 ms 1.1096 ms]
to_seed/coins-bip39 (15 words)
                        time:   [988.55 µs 1.0071 ms 1.0340 ms]
to_seed/bip0039 (15 words)
                        time:   [984.13 µs 990.86 µs 1.0049 ms]

to_seed/tiny-bip39 (18 words)
                        time:   [982.20 µs 984.69 µs 987.27 µs]
to_seed/bip39 (18 words)
                        time:   [1.0901 ms 1.0978 ms 1.1104 ms]
to_seed/coins-bip39 (18 words)
                        time:   [984.16 µs 989.41 µs 995.96 µs]
to_seed/bip0039 (18 words)
                        time:   [987.03 µs 994.99 µs 1.0083 ms]

to_seed/tiny-bip39 (24 words)
                        time:   [992.69 µs 1.0174 ms 1.0532 ms]
to_seed/bip39 (24 words)
                        time:   [1.0887 ms 1.1006 ms 1.1202 ms]
to_seed/coins-bip39 (24 words)
                        time:   [976.81 µs 984.97 µs 1.0020 ms]
to_seed/bip0039 (24 words)
                        time:   [984.46 µs 986.86 µs 989.51 µs]
```
