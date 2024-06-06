mod enemies;
mod characters;

use rand::Rng;
use enemies::Enemy;
use characters::Character;

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut characters: Vec<Character> = Vec::new();

    enemies.push(Enemy::new(
        "Forgg", 
        5, 
        5, 
        0, 
        2, 
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

}
