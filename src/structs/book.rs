use serde::{Deserialize, Serialize};

use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub name: Option<String>,
    pub intro: Option<String>,
    pub time: Option<NaiveDateTime>,
}