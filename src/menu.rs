use crate::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadingState::Done), enter_menu)
            //Main Menu systems
            .add_systems(OnEnter(LevelState::Menu), main_menu_setup)
            .add_systems(
                Update,
                (leave_main_menu, animate_menu_text).run_if(in_state(LevelState::Menu)),
            )
            .add_systems(OnExit(LevelState::Menu), main_menu_cleanup);
    }
}

#[derive(Component)]
struct MainMenuAsset;

#[derive(Component)]
struct AnimateRotation;

fn main_menu_setup(
    mut commands: Commands,
    image_asset: Res<ImageAssets>,
    sound_asset: Res<SoundAssets>,
) {
    commands.spawn((Sprite::from_image(image_asset.test.clone()), MainMenuAsset));
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                top: percent(15),
                ..default()
            },
            MainMenuAsset,
            AnimateRotation,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Click To Start!"),
                TextFont {
                    font_size: 60.0,
                    ..Default::default()
                },
                MainMenuAsset,
                AnimateRotation,
            ));
        });

    commands.spawn((Camera2d::default(), MainMenuAsset));
    commands.spawn((SamplePlayer::new(sound_asset.test.clone()), MainMenuAsset));
}

fn animate_menu_text(time: Res<Time>, mut query: Query<&mut UiTransform, With<AnimateRotation>>) {
    for mut transform in &mut query {
        transform.rotation = Rot2::radians(ops::cos(time.elapsed_secs()) / 3.0);
    }
}

fn leave_main_menu(
    mut next_level: ResMut<NextState<LevelState>>,
    mouse_click: Res<ButtonInput<MouseButton>>,
) {
    if mouse_click.just_pressed(MouseButton::Left) {
        next_level.set(LevelState::Level);
    }
}

fn main_menu_cleanup(mut commands: Commands, entity: Query<Entity, With<MainMenuAsset>>) {
    entity.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn enter_menu(mut next_level: ResMut<NextState<LevelState>>) {
    next_level.set(LevelState::Menu);
}
