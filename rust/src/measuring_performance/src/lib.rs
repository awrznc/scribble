pub fn measuring<F>(f: F, retry: usize) -> std::time::Duration
where
    F: Fn(),
{
    let start = std::time::Instant::now();
    for _ in 0..retry {
        f();
    }
    start.elapsed()
}

pub fn print(duration: &std::time::Duration) {
    println!(
        "result: {}.{:09}",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = measuring(|| println!("hello"), 10);
        print(&result);
    }
}
