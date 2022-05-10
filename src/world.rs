pub struct System
{
    pub name: String,
    pub bodies: u32,
    pub faction: String,
    pub description: String,
}

pub struct StarSystem
{
    pub ring: System,
}