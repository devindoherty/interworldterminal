pub struct Starship
{
    pub name: String,
    pub crew: u32,
    pub cargo: Vec<String>,
}

impl Starship
{
    pub fn bridge(&self)
    {
        println!("The {} is at: Condition Green.", self.name);
        println!("{} crew present and accounted for at their posts.", self.crew);
        println!("All systems nominal.");
    }

    pub fn astrogation(&self)
    {
        let current_location = String::from("The Ring");
        println!("Current Location: {}", current_location);
    }
}