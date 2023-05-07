mod gen;

pub fn call_lcg() {
    let (next_val, val) = gen::lcg(100u128);
    
    println!("Next Value: {}", next_val);
    println!("Value: {}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        call_lcg();
    }
}
