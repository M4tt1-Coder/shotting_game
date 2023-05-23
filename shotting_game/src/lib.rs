//structs

struct Player{
    x: u8,
    y: u8,
    kills: u32,
    bullets: Vec<Bullet>,
    health: u8,
}

impl Player{
    pub fn new(x: u8, y: u8, kills: u32, health: u8) -> Player{
        Player{x: x, y: y, kills: kills, bullets: Vec::new(), health: health}
    }

    
    pub fn MovePlayer(p: *mut Self){

    }
}

struct Bullet{
    x: u8,
    y: u8,
    speed: u8, //2 makes the bullet move 2 fields, 3 => 3 fields
    killed: bool,
    on_field: bool
}

impl Bullet{
    pub fn new(x: u8, y: u8, speed: u8) -> Bullet{
        Bullet {
            x: x, y: y, speed: speed, killed: false, on_field: true 
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
    on_field: bool//changes to false when reaches player area or was hit by a bullet
}

impl Enemy{
    pub fn new(x: u8, y: u8, speed: u8) -> Enemy{
        Enemy{
            x: x, y: y, speed: speed, hit: false, on_field: true
        }
    }

    fn MoveEnemy(e: *mut Self){

    }
    
}

//constants
const HEIGHT: u8 = 30;
const WIDTH: u8 = 80;
const BULLET_SPEED: u8 = 2;
const P_START_POS: (u8, u8) = (3, 15);//

//Starts the game 
pub fn start(){
    //the default player instance 
    let mut player = Player::new(P_START_POS.0, P_START_POS.1,
    0, 3 );

    render(&mut player);
}

fn render(player: *mut Player){
    
}


 