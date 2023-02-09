use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ProgressbarPlugin;

impl Plugin for ProgressbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

#[derive(Component, Clone)]
pub struct Progressbar {
    pub max: f32,
    pub value: f32,
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub filled_color: Color,
    pub background_color: Color,
    pub visible: bool,
}

#[derive(Component)]
enum ComponentType {
    Filled,
    Background,
}

#[derive(Bundle)]
pub struct ProgressbarBundle {
    progressbar: Progressbar,
    component_type: ComponentType,

    #[bundle]
    sprite: SpriteBundle,
}

impl ProgressbarBundle {
    pub fn spawn(commands: &mut Commands, progressbar: Progressbar) -> Entity {
        commands.spawn(ProgressbarBundle {
            progressbar: progressbar.clone(),
            component_type: ComponentType::Background,
            sprite: SpriteBundle::default(),
        });
        commands
            .spawn(ProgressbarBundle {
                progressbar,
                component_type: ComponentType::Filled,
                sprite: SpriteBundle::default(),
            })
            .id()
    }
}

fn update(
    mut query: Query<
        (
            &ComponentType,
            &Progressbar,
            &mut Visibility,
            &mut Transform,
            &mut Sprite,
        ),
        Changed<Progressbar>,
    >,
) {
    for (component_type, progressbar, mut visibility, mut transform, mut sprite) in query.iter_mut()
    {
        match component_type {
            ComponentType::Filled => {
                let percentage = progressbar.value / progressbar.max;
                let new_size = Vec2::new(progressbar.width * percentage, progressbar.height);
                sprite.custom_size = Some(new_size);
                sprite.color = progressbar.filled_color;
                transform.translation = Vec3::new(
                    progressbar.position.x - ((progressbar.width - new_size.x) / 2.0),
                    progressbar.position.y,
                    progressbar.position.z,
                );
                visibility.is_visible = progressbar.visible;
            }
            ComponentType::Background => {
                let new_size = Vec2::new(progressbar.width, progressbar.height);
                sprite.custom_size = Some(new_size);
                sprite.color = progressbar.background_color;
                transform.translation = Vec3::new(
                    progressbar.position.x,
                    progressbar.position.y,
                    progressbar.position.z - 1.0,
                );
                visibility.is_visible = progressbar.visible;
            }
        }
    }
}
