## Code 1

```text
zero: 現在位置を右端とする区間のうち、値が 0 のものの個数
one:  現在位置を右端とする区間のうち、値が 1 のものの個数
````


## Code 2

```rust
(zero, one) = match a {
    '0' => (1, zero + one),
    '1' => (one, zero + 1),
    _ => unreachable!(),
};
```


## Code 3

```rust
ans += one;
```
