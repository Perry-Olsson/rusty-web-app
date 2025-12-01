use askama::Template;
use serde::{Deserialize, Serialize};

use crate::models::id::Id;

#[derive(Serialize, Deserialize, Debug, Template)]
#[template(path = "post.html")]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub content: String,
}

