# rust-dump-bignum-bytes
## Dump non negtive decimal number to its big endian representation
* limitless number bound ( numbers beyond u128 works as well ) *

```
> cargo run -- 12345678
0 1 2 3 4 5 6 7
---------------
1 0 1 1 1 1 0 0 |2
0 1 1 0 0 0 0 1 |1
0 1 0 0 1 1 1 0 |0
