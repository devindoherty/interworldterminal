use crate::prompt;
use crate::Starship;
use crate::GameState;

// pub trait Menus
// {
//     fn menu(&self, items: &[&str], options: &[&str]);
// }

pub struct MenuItem<'a>
{
    pub order_number: u8,
    pub character: char,
    pub name: &'a str,
    pub process: fn(),
}

impl MenuItem<'_>
{

}


pub fn menu(items: &[MenuItem], quantity: u8, game_state: &GameState)
{

    // Display menu
    for item in items
    {
        println!("{}. {}", item.character, item.name);
    }
    
    // Get selection
    loop
    {
        let selection = prompt("Selection: ");
        for item in items
        {
            if selection == item.character
            {
                (item.process)();
                game_state.menu_history.push(item.process);
            }
        }
    }
}

