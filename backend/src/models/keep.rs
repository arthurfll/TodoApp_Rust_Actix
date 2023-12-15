use serde::{Deserialize,Serialize,};


#[derive(Deserialize,Serialize)]
pub struct Nota {
    pub title       : String ,
    pub description : String ,
}