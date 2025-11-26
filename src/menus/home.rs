use bevy::prelude::*;

#[derive(Component)]
pub struct HomeMenu;

pub fn spawn_home(mut commands: Commands) {
    commands.spawn((
        HomeMenu,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (ElapsedTime, Text::new("")),
            (
                Button,
                Node {
                    border: px(1.0).all(),
                    padding: px(8.0).all(),
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                children![(Text::new("Start"))],
            )
        ],
    ));
}

#[derive(Component)]
pub struct ElapsedTime;

pub fn update_elapsed_time(time: Res<Time>, mut text: Single<&mut Text, With<ElapsedTime>>) {
    text.0 = format!("{:.2}", time.elapsed_secs());
}
