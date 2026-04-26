use crate::*;

pub struct LightPlugin;

impl Plugin for LightPlugin{
    fn build(&self, app:&mut App){
        app.add_systems(OnEnter(LevelState::Level), light_setup);
    }
}

fn light_setup(
    mut ambient_light: ResMut<GlobalAmbientLight>
    ){
        ambient_light.color = Color::srgb(0.8,0.2,0.1);
        ambient_light.brightness= 1000.0;
        ambient_light.affects_lightmapped_meshes= true;
}
