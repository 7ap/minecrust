use futures::executor;

use minecrust;

fn main() {
    env_logger::init();

    executor::block_on(minecrust::run());
}
