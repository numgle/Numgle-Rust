use std::{string, sync::Arc};
use rocket::{serde::{Deserialize, Serialize}, Data};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Range{
    pub start:u32,
    pub end:u32,
}

impl Range{
    pub fn to_tuple(&self) -> (u32,u32){
        (self.start,self.end)
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ranges{
    pub completeHangul:Range,
    pub notCompleteHangul:Range,
    pub uppercase:Range,
    pub lowercase:Range,
    pub number:Range,
    pub special:Arc<Vec<u32>>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DataSet{
    pub cho:Arc<Vec<String>>,
    pub jong:Arc<Vec<String>>,
    pub jung:Arc<Vec<String>>,
    pub cj:Arc<Vec<Vec<String>>>,
    pub han:Arc<Vec<String>>,
    pub englishUpper:Arc<Vec<String>>,
    pub englishLower:Arc<Vec<String>>,
    pub number:Arc<Vec<String>>,
    pub special:Arc<Vec<String>>,
    pub range:Arc<Ranges>,
}

unsafe impl Sync for DataSet {}
unsafe impl Send for DataSet {}


