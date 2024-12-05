#![allow(unused_imports)]
use bevy::{color::palettes::css, prelude::*};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, start)
    .add_systems(Update, update)
    .run();
}

#[derive(Component)]
pub struct MyName {
    name: String
}

fn start(
    mut commands: Commands
) {
    commands.spawn(MyName {name: "Pedro".to_string()});
}

fn update(
    name_q: Query<&MyName>
) {
    for name in name_q.iter() {
        println!("Hello, my name is {}", name.name)
    }
}