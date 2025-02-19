
use cli_table::Color;
use cli_table::{format::Justify, Table};
use crate::traits::gen_data_id::GenDataId;
use crate::{data_collection::DataCollection,
 enums::difficulty::Difficulty};
use crate::traits::collect::Collect;
#[derive(Debug, Clone, Table)]
pub struct Topic {
    #[table(title = "ID", justify = "Justify::Right", color = "Color::Green")]
    id: u32,

    #[table(title = "Title", color = "Color::Green")]
    title: String,

    #[table(title = "Difficulty", color = "Color::Green")]
    difficulty: Difficulty
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::from(""),
            difficulty: Difficulty::EAsy,
        }
    }

    

    pub fn set_title (&mut self, title: String) {
        self.title = title
    }

    pub fn set_difficulty (&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty
    }

    

    pub  fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
}

impl Collect for Topic {
    fn collect() -> Self {
        let mut topic = Self::new();
        // println!("Enter the topics.\n Type 'done' when completd");

        
    topic.title = DataCollection::input("Enster topic title");

        
        let difficulty = DataCollection::input("Enter topic difficulty");
        topic.difficulty = Difficulty::from(difficulty.as_str());

        topic
    }
}

impl GenDataId<u32> for Topic {
    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
