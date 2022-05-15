use crate::prompt;

// pub trait Menus
// {
//     fn menu(&self, items: &[&str], options: &[&str]);
// }

pub struct MenuItem
{
    pub order_number: u8,
    pub character: char,
    pub name: String,
    pub process: String,
}

pub fn menu(items: &[MenuItem], quantity: u8)
{
    // Display menu
    for item in items
    {
        println!("{}. {}", item.character, item.name);
    }
    loop
    {
        let selection = prompt("Selection: ");
        for item in items
        {
            if selection == item.character
            {
                println!("{}", item.process);
            }
        }
    }
}