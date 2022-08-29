use bevy::prelude::*;


//derive will give all the properties of 'component' to the struct 'snakehead'
#[derive(Component)]
struct SnakeHead;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);


//startup system to put the snake on screen
//we pass the commands parameter telling the compiler there is a que of 
//commands to mutate the world + resources
fn spawn_snake(mut commands: Commands) {
    commands
    //spawn_bundle is part of the bevy standard library
    //here we're using it to spawn the snake head. 
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}


//setting up this 2d camera gives us a 2d view into the render, we're looking at it from above
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


//just to test the movement of the snake
//query finds anything with the struct 'SnakeHead' or 'Transform'
fn snake_movement(

    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {

    //here we're using the query to find the head_position of anything with struct SnakeHead
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left){
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right){
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down){
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up){
            transform.translation.y += 2.;
        }
    }
}





fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
