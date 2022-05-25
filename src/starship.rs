use std::fs;

// pub use crate::menus::Menus;
use crate::menus::MenuItem;
use crate::menus::menu;
use crate::{freeform_prompt, prompt};


pub struct Starship<'a>
{
    pub name: String,
    pub crew: u32,
    pub cargo: Vec<&'a str>,
}

// impl Menus for Starship
// {
//     fn menu(&self, items: &[&str], options: &[&str])
//     {
//         println!("{}, {}", items[0], options[0]);
//     }
// }


impl Starship<'_>
{
    pub fn bridge(&self)
    {
        println!("The {} is at: Condition Green.", self.name);
        println!("{} crew present and accounted for at their posts.", self.crew);
        println!("All systems nominal.");
    }

    pub fn astrogation()
    {    
        let astro_menu = 
        [
            MenuItem {order_number: 0, character: 's', name: "Starmap", process: Starship::astrogation_starmap},
            MenuItem {order_number: 1, character: 'n', name: "Navigate", process: Starship::astrogation_navigate},
            MenuItem {order_number: 2, character: 'o', name: "Orbit", process: Starship::astrogation_orbit},
            MenuItem {order_number: 3, character: 'x', name: "Back", process: Starship::astrogation_back},
        ];
        menu(&astro_menu, 3);
    }
    
    pub fn astrogation_starmap()
    {
        let starmap: String = fs::read_to_string("art/starmap_art.txt").expect("Error reading file!");
        println!("{}", starmap);
    }
    
    pub fn astrogation_navigate()
    {
        let mut current_location = String::from("The Ring");
        println!("Current Location: {}", current_location);

        let destination = freeform_prompt("Enter New Destination: ");
        println!("Course laid in, Captain. Initiating warp in 3... 2... 1... Mark.");
        current_location = destination;
        println!("Current Location: {}", current_location);
        Starship::astrogation();
    }

    pub fn astrogation_orbit()
    {

    }

    pub fn astrogation_back()
    {

    }
}

// impl Process for Starship
// {
//     fn do_thingy(&self)
//     {
//         println!("Thingy Done.");
//     }
// }