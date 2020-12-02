#[macro_use]
pub mod time {
    #[macro_export]
    macro_rules! timed {
        ($expr:expr) => {{
            use std::time::SystemTime;
            let start = SystemTime::now();
            let result = $expr;
            println!("took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
            result
        }};
        ($expr:expr, $message:literal) => {{
            print!($message);
            print!(" ");
            timed!($expr)
        }}
    }
}