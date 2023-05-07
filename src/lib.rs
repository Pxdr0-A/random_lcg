pub mod prelude;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (next_val, val) = prelude::lcg(100u128);

        println!("Next Value: {}", next_val);
        println!("Value: {}", val);
    }
}
