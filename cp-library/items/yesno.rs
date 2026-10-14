#[macro_export]
macro_rules! yesno {
    ($condition:expr) => {
        println!("{}", if $condition { "Yes" } else { "No" });
    };
}
