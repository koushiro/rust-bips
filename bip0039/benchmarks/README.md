# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.92.0 (ded5c06cf 2025-12-08)

## generate

```bash
cargo bench --bench generate -- --quiet
# Or just bench generate
```

```text
generate/tiny-bip39 (12 words)
                        time:   [511.31 ns 512.22 ns 513.28 ns]
generate/bip39 (12 words)
                        time:   [589.13 ns 589.73 ns 590.39 ns]
generate/coins-bip39 (12 words)
                        time:   [646.37 ns 647.38 ns 648.67 ns]
generate/bip0039 (12 words)
                        time:   [486.75 ns 487.49 ns 488.32 ns]

generate/tiny-bip39 (15 words)
                        time:   [590.71 ns 593.44 ns 598.10 ns]
generate/bip39 (15 words)
                        time:   [647.21 ns 675.35 ns 724.53 ns]
generate/coins-bip39 (15 words)
                        time:   [708.61 ns 710.66 ns 712.96 ns]
generate/bip0039 (15 words)
                        time:   [560.29 ns 561.11 ns 562.09 ns]

generate/tiny-bip39 (18 words)
                        time:   [661.35 ns 663.50 ns 665.94 ns]
generate/bip39 (18 words)
                        time:   [686.88 ns 688.97 ns 691.15 ns]
generate/coins-bip39 (18 words)
                        time:   [799.61 ns 800.71 ns 801.96 ns]
generate/bip0039 (18 words)
                        time:   [624.62 ns 633.58 ns 650.84 ns]

generate/tiny-bip39 (24 words)
                        time:   [801.77 ns 829.43 ns 893.04 ns]
generate/bip39 (24 words)
                        time:   [841.98 ns 843.54 ns 845.08 ns]
generate/coins-bip39 (24 words)
                        time:   [940.16 ns 943.53 ns 948.46 ns]
generate/bip0039 (24 words)
                        time:   [752.66 ns 767.00 ns 790.62 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [496.95 ns 498.20 ns 499.57 ns]
from_entropy/bip39 (12 words)
                        time:   [571.76 ns 573.90 ns 576.07 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [540.85 ns 541.77 ns 542.86 ns]
from_entropy/bip0039 (12 words)
                        time:   [484.98 ns 487.71 ns 490.76 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [580.28 ns 581.86 ns 583.75 ns]
from_entropy/bip39 (15 words)
                        time:   [619.91 ns 622.14 ns 624.24 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [594.66 ns 622.75 ns 667.58 ns]
from_entropy/bip0039 (15 words)
                        time:   [540.41 ns 541.49 ns 542.83 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [627.58 ns 628.66 ns 629.97 ns]
from_entropy/bip39 (18 words)
                        time:   [660.28 ns 674.99 ns 700.61 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [657.68 ns 659.17 ns 660.97 ns]
from_entropy/bip0039 (18 words)
                        time:   [588.40 ns 589.59 ns 590.68 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [758.17 ns 759.91 ns 761.50 ns]
from_entropy/bip39 (24 words)
                        time:   [809.90 ns 840.94 ns 888.83 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [766.62 ns 768.01 ns 769.30 ns]
from_entropy/bip0039 (24 words)
                        time:   [700.47 ns 702.64 ns 705.24 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5470 µs 1.5507 µs 1.5550 µs]
from_phrase/bip39 (12 words)
                        time:   [3.2437 µs 3.2506 µs 3.2581 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.8801 µs 2.8853 µs 2.8908 µs]
from_phrase/bip0039 (12 words)
                        time:   [928.75 ns 931.71 ns 935.26 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.8005 µs 1.8030 µs 1.8059 µs]
from_phrase/bip39 (15 words)
                        time:   [3.9846 µs 4.1398 µs 4.4807 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [3.4806 µs 3.4895 µs 3.4989 µs]
from_phrase/bip0039 (15 words)
                        time:   [1.1016 µs 1.1040 µs 1.1065 µs]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0517 µs 2.0572 µs 2.0633 µs]
from_phrase/bip39 (18 words)
                        time:   [4.7275 µs 4.7463 µs 4.7713 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [4.1535 µs 4.3375 µs 4.7050 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.2838 µs 1.2864 µs 1.2897 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.7044 µs 2.8255 µs 3.0776 µs]
from_phrase/bip39 (24 words)
                        time:   [6.1414 µs 6.1512 µs 6.1606 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [5.3374 µs 5.3675 µs 5.4096 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.6139 µs 1.6176 µs 1.6216 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9845 µs 1.9874 µs 1.9903 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [880.27 ns 883.09 ns 885.95 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.4396 µs 2.4565 µs 2.4775 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [1.0365 µs 1.0771 µs 1.1465 µs]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.8373 µs 2.8429 µs 2.8485 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [1.2011 µs 1.2033 µs 1.2056 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.7039 µs 3.7152 µs 3.7295 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.5054 µs 1.5084 µs 1.5120 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [976.59 µs 987.81 µs 1.0060 ms]
to_seed/bip39 (12 words)
                        time:   [1.0816 ms 1.0826 ms 1.0835 ms]
to_seed/coins-bip39 (12 words)
                        time:   [977.50 µs 978.55 µs 979.77 µs]
to_seed/bip0039 (12 words)
                        time:   [979.98 µs 981.89 µs 984.49 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [977.02 µs 978.18 µs 979.20 µs]
to_seed/bip39 (15 words)
                        time:   [1.0823 ms 1.0838 ms 1.0855 ms]
to_seed/coins-bip39 (15 words)
                        time:   [979.52 µs 981.10 µs 983.00 µs]
to_seed/bip0039 (15 words)
                        time:   [982.92 µs 994.37 µs 1.0123 ms]

to_seed/tiny-bip39 (18 words)
                        time:   [975.39 µs 977.23 µs 979.30 µs]
to_seed/bip39 (18 words)
                        time:   [1.0847 ms 1.0867 ms 1.0892 ms]
to_seed/coins-bip39 (18 words)
                        time:   [976.24 µs 977.18 µs 978.15 µs]
to_seed/bip0039 (18 words)
                        time:   [983.08 µs 993.13 µs 1.0064 ms]

to_seed/tiny-bip39 (24 words)
                        time:   [987.41 µs 992.07 µs 998.69 µs]
to_seed/bip39 (24 words)
                        time:   [1.1047 ms 1.1120 ms 1.1226 ms]
to_seed/coins-bip39 (24 words)
                        time:   [989.70 µs 991.79 µs 993.79 µs]
to_seed/bip0039 (24 words)
                        time:   [1.0205 ms 1.0289 ms 1.0396 ms]
```
