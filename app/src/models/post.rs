use serde::{Deserialize, Serialize};

use crate::models::id::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub content: String,
}

