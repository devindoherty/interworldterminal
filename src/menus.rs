use crate::prompt;

// pub trait Menus
// {
//     fn menu(&self, items: &[&str], options: &[&str]);
// }

pub struct MenuItem<'a>
{
    pub order_number: u8,
    pub character: char,
    pub name: &'a str,
    pub process: String,
}

impl MenuItem<'_>
{
    fn process(&self)
    {
        println!("{}", self.process);
    }
}


pub fn menu(items: &[MenuItem], quantity: u8)
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
                item.process()
            }
        }
    }
}

