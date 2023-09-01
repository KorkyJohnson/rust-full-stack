use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee{
    pub name: String,
    pub title: String,
    pub salary: Option<f32>,
    pub phone: Option<String>,
}