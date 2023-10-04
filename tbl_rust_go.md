Rust vs Go Data Processing Benchmark

Machine: MBP Intel i5 gen10 16GB
Rust Version: 1.72.0
Go Version: 1.21.1

Refer: [Rust](tbl_rust.md) & [Go](tbl_go.md)

|                                 | Rust                | Go                | Rust Rayon          | Go Concurrent     |
|---------------------------------|---------------------|-------------------|---------------------|-------------------|
| Processing Time                 |  28.82ms (mimalloc) | 24.77ms (sonic)   |   8.94ms (mimalloc) |  8.03ms (sonic)   |
| Processing Time with serde      |  33.97ms (mimalloc) | 38.53ms (go-json) |  13.08ms (mimalloc) | 16.48ms (go-json) |
| Processing Time with IO & serde |  34.91ms (mimalloc) | 38.58ms (go-json) |  13.96ms (mimalloc) | 23.83ms (go-json) |
| Overhead serde                  |   5.15ms (mimalloc) | 11.62ms (go-json) |   4.13ms (mimalloc) |  8.25ms (go-json) |
| Overhead IO                     | 935.54us (mimalloc) | 51.28us (go-json) | 884.22us (mimalloc) |  6.70ms  (sonic)  |
| Overhead IO & serde             |   6.09ms (mimalloc) | 11.67ms (go-json) |   5.02ms (mimalloc) | 15.60ms (go-json) |

Result:
Fastest Processing time  : Go Concurrent     8.03ms (sonic)
Fastest Total Time       : Rust Rayon       13.96ms (mimalloc)
Lowest IO Overhead       : Go               51.28us (go-json)
Lowest Serde Overhead    : Rust Rayon        4.13ms (mimalloc)
Lowest Total Overhead    : Rust Rayon        5.02ms (mimalloc)

Fastest Allocator for this kind of Benchmark: mimalloc
Fastest serde for golang: go-json

