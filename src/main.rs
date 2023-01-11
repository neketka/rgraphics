mod engine;

fn main() {
    pollster::block_on(async {
        engine::start().await;
    });
}
