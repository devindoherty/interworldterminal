use std::io; 
use std::io::Write; 

mod structures;
pub use crate::structures::*;

struct System
{
    name: String,
    bodies: u32,
    faction: String,
    description: String,
}

struct StarSystem
{
    ring: System,
}

struct Starship
{
    name: String,
    crew: u32,
    cargo: Vec<String>,
}

struct GameState
{
    state: u32,
    player: Stats,
    ring: System,
}

fn main() 
{
    println!("Welcome to the Interworld Service, captain.");

    let mut game_state = GameState
    {
        state: 0,
        player: Stats {command: 0, tactical: 0, operations: 0},
        ring: System 
        {
            name: String::from("Ring"), 
            bodies: 1, 
            faction: String::from("Inner System"),
            description: String::from("Formerly the planet Medon, during the Interworld War both sides \ntargeted the planet with devastating Terror-class weapons. The \nshattered remnants of the planet and its moon were engineered into \na somewhat habitable ringworld structure. The final segment of the \nring is set to be completed just before the Interworld War's centennial.\n"),
        },
    };

    let quit: bool = false;
    while quit == false
    {
        while game_state.state == 0
        {
            game_state.player.roll_command(15);
            chargen(&mut game_state);
        }
        while game_state.state == 1
        {
            println!("ORBITING THE RING | Postwar Year 88\n");
            println!("{}", game_state.ring.description);
            let menu_options = ["a. Astrogation", "c. Communications", "e. Engineering", "s. Science", "y. Security", "g. Cargo", "o. Officers", "r. Crew"];
            menu(&mut game_state, &menu_options);
        }
    }
}

fn chargen(game_state: &mut GameState)
{
    let mut points: u8 = 10;
    println!("Assign a point by typing a, b, or c. Type ? for help.\n");
    while points > 0
    {
        let mut choice = String::new(); 
        println!
        (
            "a. Command: {}\nb. Tactical: {}\nc. Operations: {}\nPoints Left: {}",
            game_state.player.command,
            game_state.player.tactical,
            game_state.player.operations,
            points, 
        );
        print!("Selection: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).expect("Readline failed!");
        let choice: char = match choice.trim().parse()
        {
            Ok(char) => char,
            Err(_) => continue,
        };
        if choice == 'a'
        {
            game_state.player.command += 1;
            points -= 1;
        }
        if choice == 'b'
        {
            game_state.player.tactical += 1;
            points -= 1;
        }
        if choice == 'c'
        {
            game_state.player.operations += 1;
            points -= 1;
        }
        if choice =='?'
        {
            println!("Command represents leadership ability and charisma.\nTactical represents lethality and skill in combat.\nOperations represents scientific knowledge and technical ability.\n");
        }

        // println!("The value of choice: {}", choice);
    }
    game_state.state = 1;
}

fn menu(game_state: &mut GameState, menu_options: &[&str])
{
    loop
    {
        let mut selection = String::new();
        for option in menu_options
        {
            println!("{}", option);
        }
        io::stdin().read_line(&mut selection).expect("Readline failed!");
        let selection: char = match selection.trim().parse()
        {
            Ok(char) => char,
            Err(_) => continue,
        };
    }
}