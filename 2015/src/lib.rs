#[macro_export]
macro_rules! time_execution {
    ($fn:expr $(, $args:expr)*) => {{
        let start = std::time::Instant::now();
        let result = $fn($($args),*);
        let duration = start.elapsed();
        println!("Execution time: {:?}", duration);
        result
    }};
}