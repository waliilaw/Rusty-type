#[derive(Clone, Copy)]
pub enum Difficulty {
    Easy,  
    Medium, 
    Hard   
}

impl Difficulty {
    pub fn as_str(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}