fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<usize>().unwrap();
    let primes = thread_sieve::sieve(n);
    println!("{:?}", primes);
}
