#[macro_export]
macro_rules! log {
    ($to_print:expr) => {
        println!($to_print)
    };
    ($to_print:expr,&arguments:tt) => {
        println!($to_print, &arguments: expr)
    };
}
