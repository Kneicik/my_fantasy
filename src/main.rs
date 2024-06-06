mod enemies;
mod characters;
mod battle;
mod input;

use rand::Rng;
use enemies::Enemy;
use characters::Character;
use input::read_string;
use battle::battle;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::thread;
use std::time::Duration;

const BATTLE_THEME: &'static [u8] = include_bytes!("music/battle_theme.mp3");

fn main() {
    let mut rng = rand::thread_rng();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut characters: Vec<Character> = Vec::new();

    enemies.push(Enemy::new(
        "Forgg", 
        5, 
        5, 
        0, 
        10, 
        0, 
        1, 
        0));

    println!("Hello, in this new world! \n What is your name?\n");
    let char_name: String = read_string();
    println!("Hmm... {}... Sounds familiar.", char_name);
    characters.push(Character::new(
        &char_name,
        rng.gen_range(20..30),
        rng.gen_range(10..20), 
        rng.gen_range(10..15), 
        rng.gen_range(5..8), 
        rng.gen_range(8..12), 
        1, 
        0));

    let character = &characters[0];
    let enemy = &enemies[0];
    
    println!("Well your name is {}. Your enemy will be: {}", character.name, enemy.name);

    let music_handle = thread::spawn(|| {
        play_battle_theme();
    });

    battle(character, enemy);

    music_handle.join().unwrap();
}

fn play_battle_theme() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    loop {
        let cursor = std::io::Cursor::new(BATTLE_THEME);
        let source = rodio::Decoder::new(cursor).unwrap();
        sink.append(source);
        sink.sleep_until_end();
        thread::sleep(Duration::from_secs(1));
    }
}