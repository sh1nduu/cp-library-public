#[macro_export]
macro_rules! chmin {
    ($target:expr, $first:expr $(, $rest:expr)* $(,)?) => {{
        let candidate = $first;
        $(
            let value = $rest;
            let candidate = if value < candidate { value } else { candidate };
        )*

        let target = &mut $target;
        if *target > candidate {
            *target = candidate;
            true
        } else {
            false
        }
    }};
}
