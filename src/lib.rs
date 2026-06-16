use core::hash::Hash;
use std::time::{Duration, Instant};

use bevy::{
    input::{InputSystems, keyboard::Key},
    platform::collections::HashMap,
    prelude::*,
};

pub const DEFAULT_MULTI_INPUT_DELAY: Duration = Duration::from_millis(250);

pub struct MultiInputPlugin;

impl Plugin for MultiInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MultiInputDelay>()
            .init_resource::<MultiInput<KeyCode>>()
            .init_resource::<MultiInput<Key>>()
            .add_systems(PreUpdate, keyboard_multi_input.after(InputSystems));
    }
}

#[derive(Resource, Clone, Deref, DerefMut, Debug)]
pub struct MultiInputDelay {
    max: Duration,
}

impl Default for MultiInputDelay {
    fn default() -> Self {
        Self {
            max: DEFAULT_MULTI_INPUT_DELAY,
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct MultiInput<T: Clone + Eq + Hash + Send + Sync + 'static> {
    last_pressed_time: HashMap<T, Instant>,
    press_count: HashMap<T, u8>,
}

impl<T: Clone + Eq + Hash + Send + Sync + 'static> Default for MultiInput<T> {
    fn default() -> Self {
        Self {
            last_pressed_time: Default::default(),
            press_count: Default::default(),
        }
    }
}

impl<T: Clone + Eq + Hash + Send + Sync + 'static> MultiInput<T> {
    fn cleanup(&mut self, max_delay: Duration)
    where
        T: std::fmt::Debug,
    {
        let now = Instant::now();
        for (input, last_pressed) in self.last_pressed_time.clone() {
            if now.duration_since(last_pressed) > max_delay {
                self.last_pressed_time.remove(&input);
                self.press_count.remove(&input);
            }
        }
    }

    fn handle_just_pressed(&mut self, input: T, max_delay: Duration) {
        let now = Instant::now();
        if let Some(last_pressed) = self.last_pressed_time.get(&input)
            && let Some(press_count) = self.press_count.get_mut(&input)
        {
            if now.duration_since(*last_pressed) <= max_delay {
                *press_count += 1;
            } else {
                *press_count = 1;
            }
        } else {
            self.press_count.insert(input.clone(), 1);
        }
        self.last_pressed_time.insert(input, now);
    }

    pub fn last_pressed_time(&self, input: T) -> Option<Instant> {
        self.last_pressed_time.get(&input).copied()
    }

    pub fn press_count(&self, input: T) -> u8 {
        self.press_count.get(&input).copied().unwrap_or_default()
    }

    pub fn just_multi_pressed(&self, input: T, count: u8, button_input: &ButtonInput<T>) -> bool {
        button_input.just_pressed(input.clone()) && self.press_count(input) == count
    }

    pub fn any_just_multi_pressed(
        &self,
        inputs: impl IntoIterator<Item = T>,
        count: u8,
        button_input: &ButtonInput<T>,
    ) -> bool {
        inputs
            .into_iter()
            .any(|input| self.just_multi_pressed(input, count, button_input))
    }

    pub fn all_just_multi_pressed(
        &self,
        inputs: impl IntoIterator<Item = T>,
        count: u8,
        button_input: &ButtonInput<T>,
    ) -> bool {
        inputs
            .into_iter()
            .all(|input| self.just_multi_pressed(input, count, button_input))
    }
}

pub fn keyboard_multi_input(
    max_delay: Res<MultiInputDelay>,
    keycode_input: Res<ButtonInput<KeyCode>>,
    key_input: Res<ButtonInput<Key>>,
    mut keycode_multi: ResMut<MultiInput<KeyCode>>,
    mut key_multi: ResMut<MultiInput<Key>>,
) {
    // Avoid clearing if not empty to ensure change detection is not triggered.
    keycode_multi.bypass_change_detection().cleanup(**max_delay);
    key_multi.bypass_change_detection().cleanup(**max_delay);

    for key_code in keycode_input.get_just_pressed() {
        keycode_multi.handle_just_pressed(*key_code, **max_delay);
    }
    for logical_key in key_input.get_just_pressed() {
        key_multi.handle_just_pressed(logical_key.clone(), **max_delay);
    }
}
