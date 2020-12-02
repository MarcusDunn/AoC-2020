#[macro_use]
pub mod time {
    #[macro_export]
    macro_rules! time {
        ($expr:expr) => {{
            use std::time::SystemTime;
            let start = SystemTime::now();
            let result = $expr;
            println!("test took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
            result
        }};
        ($expr:expr, $message:literal) => {{
            println!($message);
            time!($expr)
        }}
    }
}