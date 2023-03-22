use std::time::Duration;
use rand::{self, Rng, seq::IteratorRandom};

use rusty_engine::prelude::*;

const MARBLE_SPEED: f32 = 600.0;
const CARS_SPEED: f32 = 250.0;

struct GameState {
    marble_labels: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer
}

fn main() {
    let mut game = Game::new();

    let game_state = GameState {
        marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, false)
    };

    game.window_settings(WindowDescriptor{
        title: "Pew Road".into(),

        ..Default::default()
    })
    .audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    let mut player = game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.layer = 10.0;
    player.translation.y = -325.0;

    let text = game.add_text("cars left", format!("Cars left: {}", game_state.cars_left));
    text.translation = Vec2::new(540.0,  -320.0);
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine : &mut Engine, game_state: &mut GameState) {
    // Pea shooter

    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;

    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(marble_label) = game_state.marble_labels.pop() {
            let marble = engine.add_sprite(marble_label, SpritePreset::RollingBallBlue);
            marble.translation.x = player_x;
            marble.translation.y = -275.0;
            marble.layer = 5.0;
            marble.collision = true;
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        }
    }

    engine.sprites.values_mut()
        .filter(|sprite| sprite.label.starts_with("marble"))
        .for_each(|marble| marble.translation.y += MARBLE_SPEED * engine.delta_f32);

    //Deleting sprites off screen
    let mut labels_to_delete = Vec::new();

    for marble in engine.sprites.values() {
        if marble.translation.y > 400.0 || marble.translation.x > 750.0 {
            labels_to_delete.push(marble.label.clone());
        }
    }

    for label in labels_to_delete
    {
        engine.sprites.remove(&label);
        if label.starts_with("marble") {
            game_state.marble_labels.push(label);
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        game_state.spawn_timer = Timer::from_seconds(rand::thread_rng().gen_range(0.1..1.25), false);

        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let text = engine.texts.get_mut("cars left").unwrap();
            text.value = format!("Cars left: {}", game_state.cars_left);
            let car_label = format!("car{}", game_state.cars_left);
            let car_choices = vec![
                SpritePreset::RacingCarBlack,
                SpritePreset::RacingCarBlue,
                SpritePreset::RacingCarGreen,
                SpritePreset::RacingCarRed,
                SpritePreset::RacingCarYellow
            ];
            let car_choice = car_choices.iter().choose(&mut rand::thread_rng()).unwrap().clone();
            let car = engine.add_sprite(car_label, car_choice);
            car.translation.x = -740.0;
            car.translation.y = rand::thread_rng().gen_range(-100.0..325.0);
            car.collision = true;
        }
    }

    engine.sprites.values_mut()
        .filter(|sprite| sprite.label.starts_with("car"))
        .for_each(|car| car.translation.x += CARS_SPEED * engine.delta_f32);

    for event in engine.collision_events.drain(..)
    {
        match event.state {
            CollisionState::Begin => {
                if !(event.pair.0.starts_with("marble") || event.pair.1.starts_with("marble")) {
                    continue;
                }

                for label in event.pair {
                    engine.sprites.remove(&label);
                    if label.starts_with("marble") {
                        game_state.marble_labels.push(label);
                    }
                }
                engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.2);
            },
            CollisionState::End => {
                continue;
            }
        }
    }
}
