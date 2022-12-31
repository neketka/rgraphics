mod engine;

fn main() {
    pollster::block_on(async {
        engine::Engine::new().await.start();
    });
}
