fn binary_search_answer<F>(mut ok: i64, mut ng: i64, mut is_ok: F) -> i64
where
    F: FnMut(i64) -> bool,
{
    while ok.abs_diff(ng) > 1 {
        let mid = ok.midpoint(ng);
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
