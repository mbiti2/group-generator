use cli_table::{format::Justify, Table};
use cli_table::Color;
use crate::{data_collection::DataCollection,
    traits::{collect::Collect, 
           gen_data_id::GenDataId}};

#[derive(Debug, Clone, Table)]
pub struct Student {
    #[table(title = "ID", justify = "Justify::Right", color = "Color::Blue")]
    id: u32,

    #[table(title = "Name", color = "Color::Blue")]
    name: String,
}
impl Student {
    pub fn new() -> Self {
        Self { 
            id: 0, 
             name:  String::new()
        
        }
    }
    // pub fn from(name: String) -> Self {
    //     Self { id: 0, name}
    // }

    
    pub fn set_name (&mut self, name: String) {
        self.name = name
    }

   

    pub  fn get_name(&self) -> String {
        self.name.clone()
    }

}


impl Collect for Student {
    fn collect() -> Self {
        let mut student = Self::new();

        student.name = DataCollection::input("Enter student's name: ");

        student

        // let prompt = Some("Enter student's name:".to_string());
        // let name = DataCollection::input("Enter student's name");

       
        // Self::from(name)
    }
}

impl GenDataId<u32> for Student {
    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_id(&self) -> u32 {
        self.id
    }

}