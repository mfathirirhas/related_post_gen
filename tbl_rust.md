Rust Data Processing Benchmark

Machine: MBP Intel i5 gen10 16GB
Rust Version: 1.72.0

## system:
- Using **system allocator** for both versions instead of std json serde.
- Using **FxHashMap** for both versions instead of std hashmap

|                                 | Rust    | Rust Rayon |
|---------------------------------|---------|------------|
| Processing Time                 | 32.59ms | 16.27ms    |
| Processing Time with serde      | 40.61ms | 23.09ms    |
| Processing Time with IO & serde | 41.70ms | 24.39ms    |
| Overhead serde                  |  8.01ms |  6.82ms    |
| Overhead IO                     |  1.08ms |  1.29ms    |
| Overhead IO & serde             |  9.10ms |  8.11ms    |

Result:
Fastest Processing time  : Rust Rayon   16.27ms
Fastest Total Time       : Rust Rayon   24.39ms
Lowest IO Overhead       : Rust          1.08ms
Lowest Serde Overhead    : Rust Rayon    6.82ms
Lowest Total Overhead    : Rust Rayon    8.11ms

## jemalloc:
- Using **jemalloc** for both versions instead of std json serde.
- Using **FxHashMap** for both versions instead of std hashmap

|                                 | Rust      | Rust Rayon |
|---------------------------------|-----------|------------|
| Processing Time                 | 137.92ms  |    62.85ms |
| Processing Time with serde      | 146.35ms  |    69.04ms |
| Processing Time with IO & serde | 147.53ms  |    70.15ms |
| Overhead serde                  | 8.42ms    |     6.18ms |
| Overhead IO                     | 1.18ms    |     1.11ms |
| Overhead IO & serde             | 9.60ms    |     7.29ms |

Result:
Fastest Processing time  : Rust Rayon   62.85ms
Fastest Total Time       : Rust Rayon   69.04ms
Lowest IO Overhead       : Rust Rayon    1.11ms
Lowest Serde Overhead    : Rust Rayon    6.18ms
Lowest Total Overhead    : Rust Rayon    7.29ms

## mimalloc:
- Using **MiMalloc** allocator for both versions instead of system allocator.
- Using **FxHashMap** for both versions instead of std hashmap

|                                 | Rust     | Rust Rayon |
|---------------------------------|----------|------------|
| Processing Time                 | 28.82ms  | 8.94ms     |
| Processing Time with serde      | 33.97ms  | 13.08ms    |
| Processing Time with IO & serde | 34.91ms  | 13.96ms    |
| Overhead serde                  | 5.15ms   | 4.13ms     |
| Overhead IO                     | 935.54us | 884.22us   |
| Overhead IO & serde             | 6.09ms   | 5.02ms     |

Result:
Fastest Processing time  : Rust Rayon     8.94ms 
Fastest Total Time       : Rust Rayon    13.96ms
Lowest IO Overhead       : Rust Rayon   884.22us
Lowest Serde Overhead    : Rust Rayon     4.13ms
Lowest Total Overhead    : Rust Rayon     5.02ms
