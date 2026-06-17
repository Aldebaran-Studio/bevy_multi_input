# bevy_multi_input
Multi-presses input detection for the Bevy Engine. It works by adding `MultiInput<T>`, a resource similar to `ButtonInput<T>` that keeps track of repeat counts for each input.

## Quick Start
First, add the `MultiInputPlugin` to your Bevy app. Here, we also add the `DefaultPlugins` because it contains `InputPlugin`, which is required to use the methods of `MultiInput<T>`.
```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MultiInputPlugin)
        .add_systems(Update, handle_inputs)
        .run();
}
```
Then you can access the relevant resources in your system to handle multi-presses.
```rust
fn handle_inputs(
    input: Res<ButtonInput<KeyCode>>,
    multi_input: Res<MultiInput<KeyCode>>,
) {
    // Check if key was just double-pressed
    if multi_input.just_multi_pressed(KeyCode::Space, 2, &input) {
        println!("You just double-pressed the space bar!");
    }

    // Check if ANY of the keys was just multi-pressed
    if multi_input.any_just_multi_pressed([KeyCode::KeyW, KeyCode::ArrowUp], 2, &input) {
        println!("RUN to the top!!!");
    }

    // Check if ALL of the keys were just multi-pressed together
    if multi_input.all_just_multi_pressed([KeyCode::Digit1, KeyCode::Digit2], 3, &input) {
        println!("Let's GO!");
    }
}
```
To change the multi-press delay, you need to modify the `MultiInputDelay` resource.
```rust
fn change_max_delay(mut delay: ResMut<MultiInputDelay>) {
    // Modify the maximum delay for multi-presses
    delay.max = Duration::from_millis(1000);
}
```
