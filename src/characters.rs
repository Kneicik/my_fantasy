pub struct Character {
    pub name: String,
    pub health: i32,
    pub defense: i32,
    pub magic_points: i32,
    pub attack: i32,
    pub magic: i32,
    pub level: i32,
    pub exp: i32,
}

impl Character {
    pub fn new(name: &str, health: i32, defense: i32, magic_points: i32, attack: i32, magic: i32, level: i32, exp: i32) -> Self {
        Self {
            name: name.to_string(),
            health,
            defense,
            magic_points,
            attack,
            magic,
            level,
            exp,
        }
    }
}