pub struct Movement;

impl Plugin for Movement {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(move_spaceship.system()),
        );
    }
}

fn move_spaceship(mut query: Query<&mut Transform, With<Spaceship>>, keys: Res<Input<KeyCode>>) {
    for mut transform in query.iter_mut() {}
}
