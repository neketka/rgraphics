use engine::world::World;

mod engine;

fn main() {
    pollster::block_on(async {
        let behaviors = vec![];

        engine::start(World::new(behaviors)).await;
    });
}
