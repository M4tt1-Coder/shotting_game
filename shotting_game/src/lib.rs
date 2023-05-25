use std::sync::{Arc, Mutex};


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

    pub fn MoveBullet(mut self){
        self.x += self.speed;
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

    fn MoveEnemy(mut self){
       self.x -=  self.speed;
    }
    
}

//constants
const HEIGHT: u8 = 30;
const WIDTH: u8 = 80;
const BULLET_SPEED: u8 = 2;
const P_START_POS: (u8, u8) = (3, 15);//


//Starts the game 
pub fn start(){
    //stops when the player is dead
    render();
}

fn render(){
    //the default player instance 
    let mut player = Player::new(P_START_POS.0, P_START_POS.1,
        0, 3 );
    

    let mut enemies: Vec<Enemy> = vec![];
    
    //infinite loop: stops when the  
    loop{
        //here the for-loops for the displaying 
        for y in 0..HEIGHT{
            for x in 0..WIDTH{

            }
            println!();
        }

        //add a enemy

        //move the enemies and bullets 
        if enemies.len() != 0{
            for e in  enemies{
                e.MoveEnemy();
            }
        }
        
        if player.bullets.len() != 0{
            for b in player.bullets{
                b.MoveBullet();
            }
        }
    }
    
}
 
fn Add_Enemy_To_List(){

}
 