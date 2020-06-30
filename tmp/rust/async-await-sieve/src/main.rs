fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<usize>().unwrap();
    let primes = async_std::task::block_on(async_await_sieve::sieve(n));
    println!("{:?}", primes);
}
