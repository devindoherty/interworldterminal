use std::io; 
use std::io::Write; 
struct Stats
{
    command: u8,
    tactical: u8,
    operations: u8,
}

impl Stats
{
    fn roll_command(&self, target: u8)
    {
        let roll = 10 + self.command;
        println!("Rolling Command: {} vs. Challenge Rating: {}", roll, target);
        if roll >= target
        {
            println!("Success!");
        }
        else
        {
            println!("Failure.");
        }
    }
}

struct System
{
    name: String,
    bodies: u32,
    faction: String,
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
    player: Stats,
    ring: System,
}

fn main() 
{
    println!("Welcome to the Interworld Service, captain.");

    let mut game_state = GameState
    {
        player: Stats {command: 0, tactical: 0, operations: 0},
        ring: System {name: String::from("Ring"), bodies: 1, faction: String::from("Interworld")},
    };

    let quit = false;
    while quit == false
    {
        game_state.player.roll_command(15);
        chargen(&mut game_state);
        game_state.player.roll_command(15);
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
            println!("Command represents leadership ability and knowhow.\nTactical represents lethality and skill in combat.\nOperations represents scientific knowledge and technical knowhow.\n");
        }

        // println!("The value of choice: {}", choice);
    
        
    }
    
}