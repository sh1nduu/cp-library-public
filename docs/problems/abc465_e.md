## Code 1

```text
next_rem = (rem * 10 + d) % 3
````


## Code 2

```text
0057
```


## Code 3

```text
b == 0 かつ d == 0
```


## Code 4

```text
100
```


## Code 5

```text
dp[1 << 10][3][2]
```


## Code 6

```text
if mask == 0 && d == 0:
    next_mask = 0
else:
    next_mask = mask | (1 << d)
```


## Code 7

```text
next_rem = (rem * 10 + d) % 3
```


## Code 8

```text
next_less = less || (d < limit)
```


## Code 9

```text
cond = 0
if rem == 0: cond += 1
if mask & (1 << 3): cond += 1
if popcount(mask) == 3: cond += 1
```
