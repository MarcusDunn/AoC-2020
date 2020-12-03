#[macro_use]
pub mod time {
    #[macro_export]
    macro_rules! timed {
        ($expr:expr) => {{
            use std::time::SystemTime;
            let start = SystemTime::now();
            let result = $expr;
            println!(
                "took {}ns",
                SystemTime::now().duration_since(start).unwrap().as_nanos()
            );
            result
        }};
        ($expr:expr, $message:literal) => {{
            print!($message);
            print!(" ");
            timed!($expr)
        }};
    }
}
