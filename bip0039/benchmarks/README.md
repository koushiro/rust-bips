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
                        time:   [510.34 ns 511.44 ns 512.60 ns]
generate/bip39 (12 words)
                        time:   [585.92 ns 587.70 ns 589.44 ns]
generate/coins-bip39 (12 words)
                        time:   [679.68 ns 681.18 ns 682.70 ns]
generate/bip0039 (12 words)
                        time:   [494.18 ns 495.15 ns 496.27 ns]

generate/tiny-bip39 (15 words)
                        time:   [589.61 ns 591.39 ns 592.99 ns]
generate/bip39 (15 words)
                        time:   [633.80 ns 657.68 ns 710.30 ns]
generate/coins-bip39 (15 words)
                        time:   [754.56 ns 755.94 ns 757.53 ns]
generate/bip0039 (15 words)
                        time:   [565.11 ns 566.52 ns 567.96 ns]

generate/tiny-bip39 (18 words)
                        time:   [658.73 ns 660.51 ns 662.73 ns]
generate/bip39 (18 words)
                        time:   [686.83 ns 688.94 ns 691.04 ns]
generate/coins-bip39 (18 words)
                        time:   [848.04 ns 851.16 ns 854.73 ns]
generate/bip0039 (18 words)
                        time:   [631.66 ns 633.12 ns 634.48 ns]

generate/tiny-bip39 (24 words)
                        time:   [796.19 ns 797.70 ns 799.28 ns]
generate/bip39 (24 words)
                        time:   [841.13 ns 843.60 ns 846.30 ns]
generate/coins-bip39 (24 words)
                        time:   [1.0055 µs 1.0122 µs 1.0239 µs]
generate/bip0039 (24 words)
                        time:   [765.61 ns 771.53 ns 778.24 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [496.68 ns 498.45 ns 500.22 ns]
from_entropy/bip39 (12 words)
                        time:   [587.38 ns 610.53 ns 646.46 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [548.97 ns 551.21 ns 553.85 ns]
from_entropy/bip0039 (12 words)
                        time:   [489.02 ns 491.12 ns 493.42 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [577.92 ns 579.27 ns 580.72 ns]
from_entropy/bip39 (15 words)
                        time:   [618.19 ns 619.04 ns 619.79 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [600.17 ns 623.76 ns 669.40 ns]
from_entropy/bip0039 (15 words)
                        time:   [554.35 ns 555.49 ns 556.82 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [658.90 ns 665.83 ns 676.43 ns]
from_entropy/bip39 (18 words)
                        time:   [673.41 ns 714.33 ns 782.13 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [659.99 ns 666.06 ns 674.00 ns]
from_entropy/bip0039 (18 words)
                        time:   [618.48 ns 620.44 ns 622.59 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [784.05 ns 788.27 ns 793.13 ns]
from_entropy/bip39 (24 words)
                        time:   [820.16 ns 901.21 ns 1.0273 µs]
from_entropy/coins-bip39 (24 words)
                        time:   [774.26 ns 777.93 ns 782.24 ns]
from_entropy/bip0039 (24 words)
                        time:   [742.34 ns 746.56 ns 751.98 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5477 µs 1.5554 µs 1.5663 µs]
from_phrase/bip39 (12 words)
                        time:   [3.2387 µs 3.2454 µs 3.2535 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.8943 µs 2.9048 µs 2.9150 µs]
from_phrase/bip0039 (12 words)
                        time:   [928.42 ns 949.60 ns 984.64 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.7950 µs 1.8542 µs 1.9560 µs]
from_phrase/bip39 (15 words)
                        time:   [3.9781 µs 3.9912 µs 4.0083 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [3.4690 µs 3.4802 µs 3.4924 µs]
from_phrase/bip0039 (15 words)
                        time:   [1.0961 µs 1.1024 µs 1.1112 µs]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0296 µs 2.0335 µs 2.0377 µs]
from_phrase/bip39 (18 words)
                        time:   [4.6921 µs 4.7046 µs 4.7184 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [4.1568 µs 4.1710 µs 4.1869 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.2578 µs 1.2605 µs 1.2636 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6549 µs 2.7577 µs 2.9668 µs]
from_phrase/bip39 (24 words)
                        time:   [6.1632 µs 6.1869 µs 6.2140 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [5.3191 µs 5.3350 µs 5.3498 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.6057 µs 1.6088 µs 1.6125 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9888 µs 1.9945 µs 2.0007 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [865.45 ns 867.51 ns 869.88 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.4202 µs 2.4347 µs 2.4550 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [1.0248 µs 1.0262 µs 1.0277 µs]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.8493 µs 2.8549 µs 2.8616 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [1.1793 µs 1.2267 µs 1.3133 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.7213 µs 3.7300 µs 3.7398 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.5051 µs 1.5164 µs 1.5347 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [981.56 µs 983.98 µs 986.64 µs]
to_seed/bip39 (12 words)
                        time:   [1.0847 ms 1.0863 ms 1.0884 ms]
to_seed/coins-bip39 (12 words)
                        time:   [980.23 µs 981.28 µs 982.43 µs]
to_seed/bip0039 (12 words)
                        time:   [982.38 µs 985.13 µs 988.23 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [978.38 µs 981.72 µs 986.31 µs]
to_seed/bip39 (15 words)
                        time:   [1.0914 ms 1.1414 ms 1.2388 ms]
to_seed/coins-bip39 (15 words)
                        time:   [979.72 µs 982.73 µs 986.73 µs]
to_seed/bip0039 (15 words)
                        time:   [982.71 µs 984.91 µs 987.12 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [982.45 µs 985.87 µs 990.05 µs]
to_seed/bip39 (18 words)
                        time:   [1.0884 ms 1.0901 ms 1.0923 ms]
to_seed/coins-bip39 (18 words)
                        time:   [980.72 µs 982.23 µs 984.03 µs]
to_seed/bip0039 (18 words)
                        time:   [979.83 µs 981.50 µs 983.54 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [981.74 µs 991.87 µs 1.0111 ms]
to_seed/bip39 (24 words)
                        time:   [1.0873 ms 1.0888 ms 1.0904 ms]
to_seed/coins-bip39 (24 words)
                        time:   [985.59 µs 1.0074 ms 1.0481 ms]
to_seed/bip0039 (24 words)
                        time:   [983.85 µs 985.96 µs 988.07 µs]
```
