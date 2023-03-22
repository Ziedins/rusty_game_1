use std::time::Duration;

use rusty_engine::prelude::*;

const MARBLE_SPEED: f32 = 600.0;

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
        spawn_timer: Timer::new(Duration::new(0, 0), false)
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

    let mut labels_to_delete = Vec::new();

    for (label, marble) in engine.sprites
        .iter_mut()
        .filter(|(label,sprite)| label.starts_with("marble"))
    {
        marble.translation.y += MARBLE_SPEED * engine.delta_f32;
    }

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

}
