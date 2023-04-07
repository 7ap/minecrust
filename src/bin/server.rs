use minecrust::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .run();
}
