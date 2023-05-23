//structs

struct Player{
    x: u8,
    y: u8,
    kills: u32,
    bullets: Vec<Bullet>
}

impl Player{
    pub fn new(x: u8, y: u8, kills: u32) -> Player{
        Player{x: x, y: y, kills: kills, bullets: Vec::new()}
    }

    
    pub fn MovePlayer(p: *mut Self){

    }
}

struct Bullet{
    x: u8,
    y: u8,
    speed: u8, //2 makes the bullet move 2 fields, 3 => 3 fields
    killed: bool,
}

impl Bullet{
    pub fn new(x: u8, y: u8, speed: u8) -> Bullet{
        Bullet {
            x: x, y: y, speed: speed, killed: false 
        }
    }

    pub fn MoveBullet(b: *mut Self){

    }
}

struct Enemy{
    x: u8,
    y: u8,
    speed: u8,
    hit: bool,
}

impl Enemy{
    pub fn new(x: u8, y: u8, speed: u8) -> Enemy{
        Enemy{
            x: x, y: y, speed: speed, hit: false
        }
    }

    fn MoveEnemy(e: *mut Self){

    }
    
}

//constants
const HEIGHT: u8 = 30;
const WIDTH: u8 = 80;
const BulletSpeed: u8 = 2;
const P_Start_Pos: (u8, u8) = (3, 15);//

//Starts the game 
pub fn Start(){
    let player = Player::new(x, y, kills);
}

fn render(){

}


