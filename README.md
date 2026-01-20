# rust-dump-bignum-bytes
## Dump non negtive decimal number to its big endian representation
* limitless number bound ( numbers beyond u128 works as well ) *

```
> cargo run -- 1024
7 6 5 4 3 2 1 0
---------------
0 0 0 0 0 1 0 0 |1
0 0 0 0 0 0 0 0 |0
