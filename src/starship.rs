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
            MenuItem {order_number: 0, character: 's', name: "Starmap", process: "Zolar Star System".to_string()},
            MenuItem {order_number: 1, character: 'n', name: "Navigate", process: "Course Selection".to_string()},
            MenuItem {order_number: 2, character: 'o', name: "Orbit", process: "Orbital Body".to_string()},
            MenuItem {order_number: 3, character: 'x', name: "Back", process: "Test".to_string()},
        ];
        menu(&astro_menu, 3);
    }
}

// impl Process for Starship
// {
//     fn do_thingy(&self)
//     {
//         println!("Thingy Done.");
//     }
// }