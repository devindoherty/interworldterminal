use crate::prompt;
use crate::Starship;

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
        if self.name == "Starmap"
        {
            let starmap: &str = r"
          ~+
                 *       +
           '                  |
       ()    .-.,==``==.    - o -
             '=/_       \     |
          *   |  '=._    |
               \     `=./`,        '
            .   '=.__.=' `='      *
   +                         +
        O      *        '       .";
            println!("{}", starmap);
        }
        if self.name == "Back"
        {
            Starship::astrogation();
        }
        else
        {
            println!("{}", self.process);
        }
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

