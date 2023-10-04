Rust vs Go Data Processing Benchmark

Machine: MBP Intel i5 gen10 16GB
Rust Version: 1.72.0
Go Version: 1.21.1

Info:
Rust:
- Using MiMalloc allocator for both versions instead of system allocator.
- Using FxHashMap for both versions instead of std hashmap

Go:
- Using github.com/goccy/go-json for both versions instead of std json serde.
- Using arenas(https://github.com/golang/go/blob/master/src/arena/arena.go) for Go Concurrent version.

|                                 | Rust     | Go      | Rust Rayon | Go Concurrent |
|---------------------------------|----------|---------|------------|---------------|
| Processing Time                 | 28.82ms  | 26.90ms | 8.94ms     | 8.22ms        |
| Processing Time with serde      | 33.97ms  | 38.53ms | 13.08ms    | 16.48ms       |
| Processing Time with IO & serde | 34.91ms  | 38.58ms | 13.96ms    | 23.83ms       |
| Overhead serde                  | 5.15ms   | 11.62ms | 4.13ms     | 8.25ms        |
| Overhead IO                     | 935.54us | 51.28us | 884.22us   | 7.35ms        |
| Overhead IO & serde             | 6.09ms   | 11.67ms | 5.02ms     | 15.60ms       |

Result:
Fastest Processing time  : Go 8.22ms
Fastest Total Time       : Rust 13.08ms
Lowest IO Overhead       : Go 51.28us
Lowest Serde Overhead    : Rust 4.13ms
Lowest Total Overhead    : Rust 5.02ms
