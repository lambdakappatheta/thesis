/*
fn main() {
    async_await_channel::example();
}
*/

/*
#[async_std::main]
async fn main() {
    async_await_channel::example().await;
}
*/


fn main() {
    async_std::task::block_on(async_await_channel::example());
}
