use bevy::prelude::*;

pub struct ProgressbarPlugin;

impl Plugin for ProgressbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

#[derive(Component)]
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
struct ProgressbarChildren {
    filled_sprite_entity: Entity,
    background_sprite_entity: Entity,
}

impl Progressbar {
    pub fn spawn(commands: &mut Commands, progressbar: Progressbar) -> Entity {
        let filled_sprite_entity = commands.spawn(SpriteBundle::default()).id();
        let background_sprite_entity = commands.spawn(SpriteBundle::default()).id();

        commands
            .spawn((
                progressbar,
                ProgressbarChildren {
                    filled_sprite_entity,
                    background_sprite_entity,
                },
            ))
            .id()
    }
}

fn update(
    query: Query<(&Progressbar, &ProgressbarChildren), Changed<Progressbar>>,
    mut q_sprites: Query<(&mut Transform, &mut Visibility, &mut Sprite)>,
) {
    for (progressbar, progressbar_children) in query.iter() {
        // change background spritesheet
        let Ok((mut transform, mut visibility, mut sprite)) = q_sprites.get_mut(progressbar_children.background_sprite_entity) else {
            panic!("Can't find background spritesheet");
        };
        let new_size = Vec2::new(progressbar.width, progressbar.height);
        sprite.custom_size = Some(new_size);
        sprite.color = progressbar.background_color;
        transform.translation = Vec3::new(
            progressbar.position.x,
            progressbar.position.y,
            progressbar.position.z - 0.1,
        );
        visibility.is_visible = progressbar.visible;

        // change filled spritesheet
        let Ok((mut transform, mut visibility, mut sprite)) = q_sprites.get_mut(progressbar_children.filled_sprite_entity) else {
            panic!("Can't find filled spritesheet");
        };
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
}
