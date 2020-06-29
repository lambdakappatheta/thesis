fn main() {
    async_std::task::block_on(async_await_sieve::sieve())
}
