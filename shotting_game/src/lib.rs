use core::time;
use std::{sync::{Arc, Mutex}, thread};


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
    
    let delay = time::Duration::from_millis(50);

    //infinite loop: stops when the  
    loop{
        display(&enemies, &player);    
        //add a enemy


        //work with threads: one for displaying, one for adding enemies

        //move the enemies and bullets 
        MoveBullet(&mut player.bullets);       
        MoveEnemies(&mut enemies);

        thread::sleep(delay);
    }
    
}
 
fn display(enemies: &Vec<Enemy>, player: &Player){
    //here the for-loops for the displaying 
    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            
            //says if a enemy or bullet has been displayed
            let mut con_loop = false;

            //displays bullet
            for b in &player.bullets{
                if y == b.y && x == b.x{
                    print!("➼");
                    con_loop = true;
                }
            }

            //displays enemy
            for e in enemies{
                if y == e.y && x == e.x{
                    print!("☢");
                    con_loop = true;
                }
            }

            if con_loop {
                continue;
            }

            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == 5{
                print!("⍂");
            }else if y == player.y && x == player.x{
                print!("♞");
            }else{
                print!(" ");
            }
        }
        println!();
    }

}

fn MoveEnemies(enemies: &mut Vec<Enemy>){
    if enemies.len() > 0{
        for e in enemies{
            e.x -= 1;

        }
    }
    
}

fn MoveBullet(bullets: &mut Vec<Bullet>){
    if bullets.len() > 0{
        for b in bullets{
            b.x += 1;
        }
    }
}

fn Add_Enemy_To_List(){

}
 