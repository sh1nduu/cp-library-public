## Code 1

```text
attack(state):
    while 最小必要強さの敵を倒せる:
        敵を倒して強さを増やす
        子を追加する

dp[0] = 初期状態
attack(dp[0])

for mask:
    for 発見済みかつ未使用の薬 j:
        next = dp[mask] をコピー
        薬 j を飲む
        attack(next)
        dp[mask | (1 << j)] を更新
```
