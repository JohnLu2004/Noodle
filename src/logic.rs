// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com
use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::{Battlesnake, Board, Game, Coord};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"]

    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);

    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width: i32 = _board.width;
    let board_height: i32 = _board.height;
    if my_head.x == 0 { // Head is about to hit left wall
        is_move_safe.insert("left", false);
        println!("Don't go left")
    }
    if my_head.x == board_width - 1 { // Head is about to hit right wall
        is_move_safe.insert("right", false);
        println!("Don't go right")
    }
    if my_head.y == 0 { // Head is about to hit bottom wall
        is_move_safe.insert("down", false);
        println!("Don't go down")
    }
    if my_head.y == board_height - 1{ // Head is about to hit upper wall
        is_move_safe.insert("up", false);
        println!("Don't go up")
    }

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    let my_body = &you.body;
    
    let mut i: usize = 2;
    while i < my_body.len(){
        let my_part: &Coord = &you.body[i];
        if my_head.x != 0 && my_head.x - 1== my_part.x && my_head.y == my_part.y{ // Head is right of body
            is_move_safe.insert("left", false);
            println!("don't go left");

        }else if my_head.x != board_width - 1 && my_head.x + 1== my_part.x  && my_head.y == my_part.y{ // Head is left of body
            is_move_safe.insert("right", false);
            println!("don't go right");

        }else if my_head.y != 0 && my_head.x == my_part.x && my_head.y - 1 == my_part.y{ // Head is above body
            is_move_safe.insert("down", false);
            println!("don't go down");
        
        }else if my_head.x != board_height && my_head.x == my_part.x && my_head.y + 1 == my_part.y{ // Head is below body
            is_move_safe.insert("up", false); 
            println!("don't go up");

        }
        i+=1;
    }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    // Choose a random move from the safe ones
    
    let mut chosen: &&str = safe_moves.choose(&mut rand::thread_rng()).unwrap();
    println!("Chosen: {}",chosen);
    
    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    
    let food = &_board.food;
    let mut closest_food: &Coord = &_board.food[0];
    let mut closest: i32 = 11+11;
    let mut x_distance: i32;
    let mut y_distance: i32;
    for piece in  food{
        x_distance = i32::abs(my_head.x - piece.x);
        y_distance = i32::abs(my_head.y - piece.y);
        if x_distance+y_distance < closest{
            closest = x_distance+y_distance;
            closest_food = piece;
        }
    }
    x_distance = i32::abs(my_head.x - closest_food.x);
    y_distance = i32::abs(my_head.y - closest_food.y);
    
    if x_distance > y_distance{
        if safe_moves.contains(&"left") && my_head.x - closest_food.x > 0{
            //go left
            chosen = &"left";
        }else if safe_moves.contains(&"right") && my_head.x - closest_food.x < 0{
            //go right
            chosen = &"right";
        }else if safe_moves.contains(&"down") && my_head.y - closest_food.y > 0{
            //go down
            chosen = &"down";
        }else if safe_moves.contains(&"up") && my_head.y - closest_food.y < 0{
            //go up
            chosen = &"up";
        }
    }else{
        if safe_moves.contains(&"down") && my_head.y - closest_food.y > 0{
            //go down
            chosen = &"down";
        }else if safe_moves.contains(&"up") && my_head.y - closest_food.y < 0{
            //go up
            chosen = &"up";
        }else if safe_moves.contains(&"left") && my_head.x - closest_food.x > 0{
            //go left
            chosen = &"left";
        }else if safe_moves.contains(&"right") && my_head.x - closest_food.x < 0{
            //go right
            chosen = &"right";
        }
    }
    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}