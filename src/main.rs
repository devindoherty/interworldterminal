use std::env;
use std::fs;
use std::io; 
use std::io::Read;
use std::io::Write; 

mod encounters;
mod menus;
mod starship;
mod statistics;
mod world;

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


struct GameState
{
    state: u32,
    player: Stats,
    ring: System,
    ship: Starship,
}

fn main()
{
    let crawl: String = fs::read_to_string("art.txt").expect("Error reading file!");

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
            let main_selection = prompt("Selection: ");
            if main_selection == 'n'
            {
                println!("{}", crawl);
                pause();
                clear_screen();
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
            let game_selection = prompt("Orders: ");
            if game_selection == 'a'
            {
                Starship::astrogation();
            }
        
        
        }
    }
}

fn chargen(game_state: &mut GameState)
{
    let mut points: u8 = 10;
    while points > 0
    {
        println!("Assign a point by typing a, b, or c. Type ? for help.\n");
        println!
        (
            "\na. Command: {}\nb. Tactical: {}\nc. Operations: {}\nPoints Left: {}",
            game_state.player.command,
            game_state.player.tactical,
            game_state.player.operations,
            points, 
        );
        let choice = prompt("Selection: "); 
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

        clear_screen();

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

fn prompt(query: &str) -> char
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

fn clear_screen()
{
    if cfg!(target_os = "linux") || cfg!(target_os = "macos")
    {
        std::process::Command::new("clear").status().unwrap();
    }
    if cfg!(target_os = "windows")
    {
        std::process::Command::new("cls").status().unwrap();
    }
}

fn pause() 
{
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press ENTER to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}