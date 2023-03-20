use rusty_engine::prelude::*;

struct GameState {
    health: i32,
}

fn main() {
    let mut game = Game::new();

    let sprite = game.add_sprite("player", SpritePreset::RacingCarGreen);
    sprite.scale = 1.9;

    game.audio_manager.play_music(MusicPreset::MysteriousMagic, 1.0);

    game.add_logic(game_logic);

    game.run(GameState {health: 100});
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut("player").unwrap();

    player.rotation += std::f32::consts::PI * engine.delta_f32;

    if player.translation.x > 100.0 {
        game_state.health -=1;
    }

}
