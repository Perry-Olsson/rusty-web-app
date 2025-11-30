use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct Id {
    val: u64
}

impl Id {
    fn new() -> Id {
        Id { val: 1 }
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.val)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u64::deserialize(deserializer)?;
        Ok(Id { val })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: Id,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetPostQuery {
    upper: Option<bool>,
}

#[get("/post")]
pub async fn get_post(query: web::Query<GetPostQuery>) -> impl Responder {
    let post = _get_post(&query);
    web::Json(post)
}

fn _get_post(query: &GetPostQuery) -> Post {
    let mut post = Post {
        id: Id::new(),
        title: "My First Post".to_string(),
        content: "This is some dummy content for the post.".to_string(),
    };

    if query.upper.unwrap_or(false) {
        post.title = post.title.to_uppercase();
    }

    post
}

#[post("/post")]
pub async fn create_post(post: web::Json<NewPost>) -> impl Responder {
    match _create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

fn _create_post(new_post: NewPost) -> Result<Post, String> {
    if new_post.title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }

    if new_post.content.trim().is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    Ok(Post {
        id: Id::new(),
        title: new_post.title,
        content: new_post.content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_post_default() {
        let query = GetPostQuery { upper: None };
        let post = _get_post(&query);

        assert_eq!(post.title, "My First Post");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }

    #[test]
    fn test_get_post_upper_false() {
        let query = GetPostQuery { upper: Some(false) };
        let post = _get_post(&query);

        assert_eq!(post.title, "My First Post");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }

    #[test]
    fn test_get_post_upper_true() {
        let query = GetPostQuery { upper: Some(true) };
        let post = _get_post(&query);

        assert_eq!(post.title, "MY FIRST POST");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }
}
