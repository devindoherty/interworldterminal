use rand::Rng;

pub struct Stats
{
    pub command: u8,
    pub tactical: u8,
    pub operations: u8,
}

impl Stats
{
    pub fn roll_command(&self, target: u8)
    {
        let dice = rand::thread_rng().gen_range(1..20);
        let roll = dice + self.command;
        println!("Rolling Command: {} + {} = {} vs. Challenge Rating: {}", 
                dice, self.command, roll, target);
        if roll >= target
        {
            println!("Success!\n");
        }
        else
        {
            println!("Failure.\n");
        }
    }
}