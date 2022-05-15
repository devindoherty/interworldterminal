// pub use crate::menus::Menus;
use crate::menus::MenuItem;
use crate::menus::menu;

pub struct Starship
{
    pub name: String,
    pub crew: u32,
    pub cargo: Vec<String>,
}

// impl Menus for Starship
// {
//     fn menu(&self, items: &[&str], options: &[&str])
//     {
//         println!("{}, {}", items[0], options[0]);
//     }
// }


impl Starship
{
    pub fn bridge(&self)
    {
        println!("The {} is at: Condition Green.", self.name);
        println!("{} crew present and accounted for at their posts.", self.crew);
        println!("All systems nominal.");
    }

    pub fn astrogation()
    {
        let current_location = String::from("The Ring");
        println!("Current Location: {}", current_location);
    
        let astro_menu = 
        [
            MenuItem {order_number: 0, character: 's', name: "Starmap".to_string(), process: "Zolar Star System".to_string()},
            MenuItem {order_number: 1, character: 'n', name: "Navigate".to_string(), process: "Course Selection".to_string()},
            MenuItem {order_number: 2, character: 'o', name: "Orbit".to_string(), process: "Orbital Body".to_string()}
        ];

        menu(&astro_menu, 3);

    
    }
}