use std::io; 
use std::io::Write; 

mod statistics;
mod world;
mod starship;
pub use crate::statistics::Stats;
pub use crate::world::System;
pub use crate::world::StarSystem;
pub use crate::starship::Starship;

const TITLE: &str = r"
 __  .__   __. .___________. _______ .______     ____    __    ____  ______   .______       __       _______  
|  | |  \ |  | |           ||   ____||   _  \    \   \  /  \  /   / /  __  \  |   _  \     |  |     |       \ 
|  | |   \|  | `---|  |----`|  |__   |  |_)  |    \   \/    \/   / |  |  |  | |  |_)  |    |  |     |  .--.  |
|  | |  . `  |     |  |     |   __|  |      /      \            /  |  |  |  | |      /     |  |     |  |  |  |
|  | |  |\   |     |  |     |  |____ |  |\  \----.  \    /\    /   |  `--'  | |  |\  \----.|  `----.|  '--'  |
|__| |__| \__|     |__|     |_______|| _| `._____|   \__/  \__/     \______/  | _| `._____||_______||_______/ 
                                                                                                               ";

const CRAWL: &str = 
"
              .   88 years ago, a war between the six worlds  .        .     .
                 of the Zolar star system threatenend the lives     .  .
     .       .  of billions of sentient beings, and lead to the            .
.        .     destruction of entire planets.                           .
   .            On one side, the Inner System Alliance opposing    .      .
             the Outer Rim Powers - the reasoning for the war 
          . myriad but also superficial, both sides suffered losses .   .
   .       almost unimaginable. Vast fleets, overwhelming armies, and        .  .
.      .  death reigned for ten long years, with no hope in sight.     .             .
    .      Until, eventually, with the dead too numerous to count, the   .    .
.      war was brought to an end with the signing of the Armstice. Many    .
    . observers in the Six Worlds thought the peace only temporary -
     with a second war merely on the horizon.                                    .
       To that end, the Alliance and the Rim came together to form an org-   .
 . anization dedicated to the prospect of promoting peace and cooperation
  between all of the major worlds of the Zolar star system. For 88 years,
 this Interworld Service has maintained a fragile peace. Until now.
    As the captain of the Interworld Service Starship Worthwhile Endeavour,
charged with preventing a second Interworld War, you must decide how to best
prevent another conflict from embroiling the star system in bloodshed once
again. However, mysterious powers on the edges of known space have machinations
all their own...      \n";

struct GameState
{
    state: u32,
    player: Stats,
    ring: System,
    ship: Starship,
}

fn main()
{
    print!("{}[2J", 27 as char); // Clear terminal screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor to Row 1, Column 1
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
        ship: Starship
        {
            name: String::from("ISS Worthwhile Endeavour"),
            crew: 250,
            cargo: vec!["Disruptor Gun".to_string()],
        }
    };
    
    println!("{}", TITLE);

    let mut quit: bool = false;
    while quit == false
    {
        while game_state.state == 0
        {
            let main_menu = ["n. New Game", "q. Quit"];
            menu(&main_menu);
            let main_selection = prompt("Selection: ".to_string());
            if main_selection == 'n'
            {
                println!("{}", CRAWL);
                game_state.state = 1;
            }
            if main_selection == 'q'
            {
                quit = true;
                break;
            }
        }
        
        while game_state.state == 1
        {
            // game_state.player.roll_command(15);
            println!("CHARACTER GENERATION");
            chargen(&mut game_state);
        }
        while game_state.state == 2
        {
            println!("ORBITING THE RING | Postwar Year 88\n");
            println!("{}", game_state.ring.description);
            let menu_options = ["a. Astrogation", "b. Bridge", "c. Communications", "e. Engineering", "s. Science", "y. Security", "g. Cargo", "o. Officers", "r. Crew", "z. Captain's Quarters"];
            menu(&menu_options);
            prompt("Orders: ".to_string());
        }
    }
}

fn chargen(game_state: &mut GameState)
{
    let mut points: u8 = 10;
    println!("Assign a point by typing a, b, or c. Type ? for help.\n");
    while points > 0
    {
        println!
        (
            "\na. Command: {}\nb. Tactical: {}\nc. Operations: {}\nPoints Left: {}",
            game_state.player.command,
            game_state.player.tactical,
            game_state.player.operations,
            points, 
        );
        let choice = prompt("Selection: ".to_string()); 
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
            println!("Command represents leadership ability and charisma.\nTactical represents lethality and skill in combat.\nOperations represents scientific knowledge and technical knowhow.\n");
        }

        // println!("The value of choice: {}", choice);
    }
    // let life_path == false;

    game_state.state = 2;
}

fn menu(menu_options: &[&str])
{
    for option in menu_options
    {
        println!("{}", option);
    }
}

fn prompt(query: String) -> char
{
    print!("{}", query);
    io::stdout().flush().unwrap();
    let mut _selection = String::new();
    loop
    {
        io::stdin().read_line(&mut _selection).expect("Readline failed!");
        let _selection: char = match _selection.trim().parse()
        {
            Ok(char) => return char,
            Err(_) => continue,
        };
    }
}