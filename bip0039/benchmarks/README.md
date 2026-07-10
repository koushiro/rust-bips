# Benchmarks

- Hardware: Apple M1 Pro
- Toolchain: rustc 1.96.1 (31fca3adb 2026-06-26)

## generate

```bash
cargo bench --bench generate -- --quiet
# Or just bench generate
```

```text
generate/tiny-bip39 (12 words)
                        time:   [517.46 ns 524.93 ns 535.60 ns]
generate/bip39 (12 words)
                        time:   [595.67 ns 600.45 ns 606.67 ns]
generate/coins-bip39 (12 words)
                        time:   [660.26 ns 666.40 ns 676.60 ns]
generate/bip0039 (12 words)
                        time:   [324.17 ns 326.71 ns 329.86 ns]

generate/tiny-bip39 (15 words)
                        time:   [592.59 ns 595.06 ns 598.11 ns]
generate/bip39 (15 words)
                        time:   [647.31 ns 649.07 ns 651.27 ns]
generate/coins-bip39 (15 words)
                        time:   [732.32 ns 733.65 ns 735.64 ns]
generate/bip0039 (15 words)
                        time:   [389.90 ns 391.36 ns 394.18 ns]

generate/tiny-bip39 (18 words)
                        time:   [651.37 ns 654.16 ns 658.44 ns]
generate/bip39 (18 words)
                        time:   [696.47 ns 699.09 ns 703.09 ns]
generate/coins-bip39 (18 words)
                        time:   [827.50 ns 832.17 ns 841.60 ns]
generate/bip0039 (18 words)
                        time:   [443.80 ns 446.10 ns 450.46 ns]

generate/tiny-bip39 (24 words)
                        time:   [796.31 ns 800.54 ns 808.60 ns]
generate/bip39 (24 words)
                        time:   [849.82 ns 854.58 ns 862.69 ns]
generate/coins-bip39 (24 words)
                        time:   [987.78 ns 991.17 ns 998.32 ns]
generate/bip0039 (24 words)
                        time:   [569.99 ns 576.27 ns 586.85 ns]
```

## from_entropy

```bash
cargo bench --bench from_entropy -- --quiet
# Or just bench from_entropy
```

```text
from_entropy/tiny-bip39 (12 words)
                        time:   [510.04 ns 520.69 ns 534.57 ns]
from_entropy/bip39 (12 words)
                        time:   [590.57 ns 593.60 ns 596.90 ns]
from_entropy/coins-bip39 (12 words)
                        time:   [538.48 ns 552.87 ns 578.71 ns]
from_entropy/bip0039 (12 words)
                        time:   [319.27 ns 321.07 ns 323.08 ns]

from_entropy/tiny-bip39 (15 words)
                        time:   [587.02 ns 589.38 ns 592.08 ns]
from_entropy/bip39 (15 words)
                        time:   [644.01 ns 647.02 ns 650.58 ns]
from_entropy/coins-bip39 (15 words)
                        time:   [591.27 ns 593.93 ns 596.98 ns]
from_entropy/bip0039 (15 words)
                        time:   [398.00 ns 402.61 ns 409.18 ns]

from_entropy/tiny-bip39 (18 words)
                        time:   [661.24 ns 673.01 ns 691.24 ns]
from_entropy/bip39 (18 words)
                        time:   [698.96 ns 726.76 ns 762.74 ns]
from_entropy/coins-bip39 (18 words)
                        time:   [653.99 ns 659.86 ns 668.52 ns]
from_entropy/bip0039 (18 words)
                        time:   [458.00 ns 462.22 ns 468.80 ns]

from_entropy/tiny-bip39 (24 words)
                        time:   [785.84 ns 792.85 ns 803.27 ns]
from_entropy/bip39 (24 words)
                        time:   [855.77 ns 867.40 ns 884.12 ns]
from_entropy/coins-bip39 (24 words)
                        time:   [759.57 ns 778.85 ns 809.28 ns]
from_entropy/bip0039 (24 words)
                        time:   [578.56 ns 588.69 ns 606.30 ns]
```

## from_phrase

```bash
cargo bench --bench from_phrase -- --quiet
# Or just bench from_phrase
```

```text
from_phrase/tiny-bip39 (12 words)
                        time:   [1.5697 µs 1.5943 µs 1.6239 µs]
from_phrase/bip39 (12 words)
                        time:   [3.2415 µs 3.2825 µs 3.3576 µs]
from_phrase/coins-bip39 (12 words)
                        time:   [2.9139 µs 2.9298 µs 2.9492 µs]
from_phrase/bip0039 (12 words)
                        time:   [778.08 ns 782.13 ns 786.79 ns]

from_phrase/tiny-bip39 (15 words)
                        time:   [1.8039 µs 1.8327 µs 1.8898 µs]
from_phrase/bip39 (15 words)
                        time:   [3.9946 µs 4.0694 µs 4.1972 µs]
from_phrase/coins-bip39 (15 words)
                        time:   [3.4672 µs 3.5534 µs 3.7207 µs]
from_phrase/bip0039 (15 words)
                        time:   [952.77 ns 956.39 ns 960.10 ns]

from_phrase/tiny-bip39 (18 words)
                        time:   [2.0745 µs 2.1266 µs 2.2107 µs]
from_phrase/bip39 (18 words)
                        time:   [4.7363 µs 4.8473 µs 4.9929 µs]
from_phrase/coins-bip39 (18 words)
                        time:   [4.1656 µs 4.1830 µs 4.2011 µs]
from_phrase/bip0039 (18 words)
                        time:   [1.1242 µs 1.1300 µs 1.1374 µs]

from_phrase/tiny-bip39 (24 words)
                        time:   [2.6974 µs 2.7689 µs 2.8869 µs]
from_phrase/bip39 (24 words)
                        time:   [6.1604 µs 6.2162 µs 6.2989 µs]
from_phrase/coins-bip39 (24 words)
                        time:   [5.3663 µs 5.5312 µs 5.7487 µs]
from_phrase/bip0039 (24 words)
                        time:   [1.4760 µs 1.5055 µs 1.5554 µs]

from_normalized_phrase/bip39 (12 words)
                        time:   [1.9908 µs 2.0270 µs 2.0805 µs]
from_normalized_phrase/bip0039 (12 words)
                        time:   [721.20 ns 748.06 ns 782.21 ns]

from_normalized_phrase/bip39 (15 words)
                        time:   [2.4292 µs 2.4791 µs 2.5595 µs]
from_normalized_phrase/bip0039 (15 words)
                        time:   [876.45 ns 879.91 ns 883.66 ns]

from_normalized_phrase/bip39 (18 words)
                        time:   [2.8799 µs 2.8957 µs 2.9113 µs]
from_normalized_phrase/bip0039 (18 words)
                        time:   [1.0368 µs 1.0482 µs 1.0657 µs]

from_normalized_phrase/bip39 (24 words)
                        time:   [3.7277 µs 3.7531 µs 3.7885 µs]
from_normalized_phrase/bip0039 (24 words)
                        time:   [1.3606 µs 1.3907 µs 1.4376 µs]
```

## to_seed

```bash
cargo bench --bench to_seed -- --quiet
# Or just bench to_seed
```

```text
to_seed/tiny-bip39 (12 words)
                        time:   [975.50 µs 976.54 µs 977.64 µs]
to_seed/bip39 (12 words)
                        time:   [1.1209 ms 1.1223 ms 1.1238 ms]
to_seed/coins-bip39 (12 words)
                        time:   [976.70 µs 977.84 µs 979.11 µs]
to_seed/bip0039 (12 words)
                        time:   [417.55 µs 418.43 µs 419.25 µs]

to_seed/tiny-bip39 (15 words)
                        time:   [1.0769 ms 1.1476 ms 1.2318 ms]
to_seed/bip39 (15 words)
                        time:   [1.1350 ms 1.1465 ms 1.1625 ms]
to_seed/coins-bip39 (15 words)
                        time:   [977.15 µs 985.74 µs 999.74 µs]
to_seed/bip0039 (15 words)
                        time:   [417.16 µs 418.50 µs 420.25 µs]

to_seed/tiny-bip39 (18 words)
                        time:   [979.32 µs 982.45 µs 985.63 µs]
to_seed/bip39 (18 words)
                        time:   [1.1226 ms 1.1242 ms 1.1259 ms]
to_seed/coins-bip39 (18 words)
                        time:   [989.97 µs 1.0043 ms 1.0238 ms]
to_seed/bip0039 (18 words)
                        time:   [417.40 µs 420.01 µs 424.16 µs]

to_seed/tiny-bip39 (24 words)
                        time:   [981.02 µs 987.93 µs 998.72 µs]
to_seed/bip39 (24 words)
                        time:   [1.1258 ms 1.1339 ms 1.1482 ms]
to_seed/coins-bip39 (24 words)
                        time:   [984.54 µs 991.33 µs 999.80 µs]
to_seed/bip0039 (24 words)
                        time:   [424.53 µs 432.25 µs 443.40 µs]
```
