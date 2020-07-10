pub fn sieve(n: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    let mut number = 2;
    while primes.len() < n {

        let mut has_divisor = false;
        for prime in &primes {
            // if 'number' can be evenly divided by 'prime'
            if number % prime == 0 {
                // than 'number' has a divisor other than 1 and itself
                // i.e. 'number' is not a prime
                has_divisor = true;
                break;
            }
        }
        // if none of the primes found so far divides 'number' evenly,
        // then 'number' is a prime
        if !has_divisor {
            primes.push(number);
        }

        number += 1;
    }
    primes
}
