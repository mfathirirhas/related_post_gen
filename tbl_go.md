Go Data Processing Benchmark

Machine: MBP Intel i5 gen10 16GB
Go Version: 1.21.1

## std:
- Using **std serde** for both versions instead of std json serde.
- Using **arenas**(https://github.com/golang/go/blob/master/src/arena/arena.go) for Go Concurrent version.

|                                 | Go      | Go Concurrent |
|---------------------------------|---------|---------------|
| Processing Time                 | 24.91ms |  8.26ms       |
| Processing Time with serde      | 49.02ms | 27.34ms       |
| Processing Time with IO & serde | 49.09ms | 34.89ms       |
| Overhead serde                  | 24.10ms | 19.08ms       |
| Overhead IO                     | 72.60µs |  7.54ms       |
| Overhead IO & serde             | 24.18ms | 26.63ms       |

Result:
Fastest Processing time  : Go Concurrent     8.26ms
Fastest Total Time       : Go Concurrent    34.89ms
Lowest IO Overhead       : Go               72.60us
Lowest Serde Overhead    : Go Concurrent    19.08ms
Lowest Total Overhead    : Go               24.18ms

## sonic:
- Using **https://github.com/bytedance/sonic** for both versions instead of std json serde.
- Using **arenas**(https://github.com/golang/go/blob/master/src/arena/arena.go) for Go Concurrent version.
Processing time: 8.036035ms
Processing time with serde: 20.800783ms
Processing time with IO and serde: 27.50806ms
Overhead serde: 12.764748ms
Overhead IO: 6.707277ms
Overhead IO & serde: 19.472025ms
|                                 | Go       | Go Concurrent |
|---------------------------------|----------|---------------|
| Processing Time                 |  24.77ms |  8.03ms       |
| Processing Time with serde      |  40.28ms | 20.80ms       |
| Processing Time with IO & serde |  40.39ms | 27.50ms       |
| Overhead serde                  |  15.51ms | 12.76ms       |
| Overhead IO                     | 104.64µs |  6.70ms       |
| Overhead IO & serde             |  15.61ms | 19.47ms       |

Result:
Fastest Processing time  : Go Concurrent     8.03ms
Fastest Total Time       : Go Concurrent     27.50ms
Lowest IO Overhead       : Go               104.64us
Lowest Serde Overhead    : Go Concurrent     12.76ms
Lowest Total Overhead    : Go                15.61ms

## go-json:
- Using **github.com/goccy/go-json** for both versions instead of std json serde.
- Using **arenas**(https://github.com/golang/go/blob/master/src/arena/arena.go) for Go Concurrent version.

|                                 | Go      | Go Concurrent |
|---------------------------------|---------|---------------|
| Processing Time                 | 26.90ms | 8.22ms        |
| Processing Time with serde      | 38.53ms | 16.48ms       |
| Processing Time with IO & serde | 38.58ms | 23.83ms       |
| Overhead serde                  | 11.62ms | 8.25ms        |
| Overhead IO                     | 51.28us | 7.35ms        |
| Overhead IO & serde             | 11.67ms | 15.60ms       |

Result:
Fastest Processing time  : Go Concurrent     8.22ms
Fastest Total Time       : Go Concurrent    23.83ms
Lowest IO Overhead       : Go               51.28us
Lowest Serde Overhead    : Go Concurrent     8.25ms
Lowest Total Overhead    : Go               11.67ms
