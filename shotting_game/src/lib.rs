use core::time;
use std::{thread,io::Error};
use rand::Rng;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use termion::raw::RawTerminal;

//use thread::

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
const WIDTH: u8 = 150;
const BULLET_SPEED: u8 = 2;
const P_START_POS: (u8, u8) = (3, 15);//
const P_FIELD_BORDER: u8 = 5;

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
        spawn_enemy(2);
        //work with threads: one for displaying, one for adding enemies

        //move player with key press events
        //move_player(&get_pressed_key(), &mut player);
        /*another call for the player control*/let _ = control_player(&mut player);

        //move the enemies and bullets 
        move_bullet(&mut player.bullets);       
        move_enemies(&mut enemies);

        //check all conditions 
        enemy_reached_playerfield(&mut enemies);
        bullet_hit_enemy(&mut player.bullets, &mut enemies);

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

            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == P_FIELD_BORDER/*restricts the space where the player can move*/{
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

//the actual controlling of the player in one function
fn control_player(player: &mut Player) -> Result<(), Error>{
    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    for k in stdin.keys(){
        write!(stdout, "").unwrap();

        match k.unwrap(){
            Key::Char('w') => player.y -= 1,
            Key::Char('s') => player.y += 1,
            Key::Char('a') => player.x -= 1,
            Key::Char('d') => player.x += 1,
            Key::Char('e') => player.bullets.push(spawn_bullet(player.x, player.y)),
            _ => ()
        }
    }

    Ok(())    
}

//leave this as a comment for testing
// fn get_pressed_key() -> String{
//     let stdin = stdin();

//     let mut stdout = stdout().into_raw_mode().unwrap();//maybe handle error in better way
//     stdout.flush().unwrap();

//     let mut command = String::new();

//     for k in stdin.keys(){
//         command = match k.unwrap(){
//             Key::Char('w') => String::from("Up"),
//             Key::Char('s') => String::from("Down"),
//             Key::Char('a') => String::from("Left"),
//             Key::Char('d') => String::from("Right"),
//             Key::Char('e') => String::from("Shot"),
//             _ => String::new()
//         };
//     }

//     command
// }

fn move_enemies(enemies: &mut Vec<Enemy>){
    if enemies.len() > 0{
        for e in enemies{
            e.x -= e.speed;

        }
    }
    
}

fn move_bullet(bullets: &mut Vec<Bullet>){
    if bullets.len() > 0{
        for b in bullets{
            b.x += b.speed;
        }
    }
}

fn spawn_enemy(speed: u8) -> Enemy{
    let mut enemy = Enemy::new(WIDTH - 1, rand::thread_rng().gen_range(0..HEIGHT),
        speed);

    enemy
}
 
fn spawn_bullet(x: u8, y: u8) -> Bullet{
    let bullet = Bullet::new(x, y, 2);

    bullet
}

fn bullet_hit_enemy(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>){
    for e in enemies{
        for b in &mut *bullets{
            if b.x == e.x && b.y == e.y{
                e.on_field = false;
                b.killed = true;
                b.on_field = false;
            }            
        }
    }
}

//comment out for testing / developing
// fn move_player(key: &str, player: &mut Player) {
//     match key{
//         "Up" => player.y -= 1,
//         "Down" => player.y += 1,
//         "Left" => player.x -= 1,
//         "Right" => player.x += 1,
//         "Shot" => player.bullets.push(Bullet::new(player.x, player.y,2)),
//         _ => ()
//     }
// }

fn enemy_reached_playerfield(enemies: &mut Vec<Enemy>){
    for e in &mut *enemies{
        if e.x == P_FIELD_BORDER{
            e.hit = true;
            e.on_field = false;
        }
    }
}