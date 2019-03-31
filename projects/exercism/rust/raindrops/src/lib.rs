pub fn raindrops(n: u32) -> String {
    let limit = (n as f64).sqrt().floor();
    let mut pairs = Vec::new();
    
    for i in 1..(limit as u32) {
        if n % i == 0 {
            pairs.push(i);
            pairs.push(n);
        }
    }

    parse_factors(pairs)
}

fn parse_factors(factors : Vec<u32>) -> String {
    let mut result = String::new();
    let mut found :bool = false;

    if factors.binary_search(&3).is_ok() {
        result.push_str("Pling");
        found = true;
    }

    if factors.binary_search(&5).is_ok() {
        result.push_str("Plang");
        found = true;
    }

    if factors.binary_search(&7).is_ok() {
        result.push_str("Plong");
        found = true;
    }

    if !found {
        factors.into_iter().map(|i| i.to_string()).collect::<String>()
    } else {
        result
    }

}
