use crate::enemies::Enemy;
use crate::characters::Character;
use crate::input::read_string;


pub fn battle(player: &Character, enemy: &Enemy){
    let mut player_health = player.health;
    let mut player_defense = player.defense;
    let mut enemy_health = enemy.health;

    println!("Oh gods, They're fighting!!! {} with {}", player.name, enemy.name);

    loop{
        println!("{}:{} HP", player.name, player_health);
        println!("{}:{} HP", enemy.name, enemy_health);
        player_defense = player.defense;

        println!("Actions:");
        println!("1. Attack");
        println!("2. Defend");
        println!("3. Run");

        let action = read_string();

        match action.trim() {
            "1" => {
                let damage = player.attack - enemy.defense;
                if damage > 0 {
                    enemy_health -= damage;
                    println!("{} dealt {} damage!!", player.name, damage);
                } else {
                    println!("Pathetic, {} can't even hurt {}", player.name, enemy.name);
                } 

                if enemy_health <= 0 {
                    println!("Congratulations! {} was defeated", enemy.name);
                    break;
                }
            }
            "2" => {
                player_defense *=2;
            }
            "3" => {
                println!("Coward!! There is no escape!");
            }
            _=> {
                println!("Wrong action.");
            }
        }

        println!("Enemy turn.");

        let damage = enemy.attack - player_defense;
        if damage > 0 {
            player_health -= damage;
            println!("{} dealt {} damage!!", enemy.name, damage);
        } else {
            println!("{} missed.", enemy.name);
        }

        if player_health <= 0 {
            println!("{} was defeated.", player.name);
            break;
        }


    }

}