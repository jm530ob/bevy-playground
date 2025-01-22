use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, print_names);
    }
}
#[derive(Resource)]
pub struct GreetTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Jakub".to_string(),
    });
}

pub fn print_names(time: Res<Time>, mut timer: ResMut<GreetTimer>, persons_query: Query<&Person>) {
    if timer.0.tick(time.delta()).just_finished() {
        for person in persons_query.iter() {
            println!("{}", person.name);
        }
    }
}

#[derive(Component)]
pub struct Person {
    name: String,
}

pub fn hello() {
    println!("Hello");
}
