pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_example() {
        let mut seed = 12345u128;
        let mut val: f64;
        for _ in 0..10 {
            val = prelude::lcg(&mut seed);
            println!("Value Update: {}", val);
            println!("Seed Update: {}", seed);
        }
    }
}
