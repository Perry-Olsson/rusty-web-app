use serde::{Deserialize, Serialize};

use crate::models::{id::Id, post::Post};

pub struct PostService {
}

impl PostService {
    pub fn new() -> PostService {
        PostService {}
    }

    pub fn create_post(&self, new_post: NewPost) -> Result<Post, String> {
        if new_post.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        if new_post.content.trim().is_empty() {
            return Err("Content cannot be empty".to_string());
        }

        Ok(Post {
            id: Id::new(5),
            title: new_post.title,
            content: new_post.content,
        })
    }

    pub fn get_post(&self, query: GetPostRequest) -> Option<Post> {
        if query.id != 5 {
            return None;
        }
        let mut post = Post {
            id: Id::new(5),
            title: "My First Post".to_string(),
            content: "This is some dummy content for the post.".to_string(),
        };

        if query.upper.unwrap_or(false) {
            post.title = post.title.to_uppercase();
        }

        Some(post)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetPostRequest {
    pub id: Id,
    pub upper: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_post_default() {
        let query = GetPostRequest { 
            id: Id::new(5),
            upper: None 
        };
        let post = PostService::new().get_post(query).unwrap();

        assert_eq!(post.title, "My First Post");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }

    #[test]
    fn test_get_post_upper_false() {
        let query = GetPostRequest { 
            id: Id::new(5),
            upper: Some(false) 
        };
        let post = PostService::new().get_post(query).unwrap();

        assert_eq!(post.title, "My First Post");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }

    #[test]
    fn test_get_post_upper_true() {
        let query = GetPostRequest { 
            id: Id::new(5),
            upper: Some(true) 
        };
        let post = PostService::new().get_post(query).unwrap();

        assert_eq!(post.title, "MY FIRST POST");
        assert_eq!(post.content, "This is some dummy content for the post.");
    }

    #[test]
    fn test_get_non_existent_post() {
        let query = GetPostRequest { 
            id: Id::new(6),
            upper: None 
        };
        let post = PostService::new().get_post(query);
        assert!(post.is_none())
    }

    #[test]
    fn test_create_post_valid() {
        let new_post = NewPost {
            title: "Test Post".to_string(),
            content: "Test content".to_string(),
        };
        let result =PostService::new().create_post(new_post);

        assert!(result.is_ok());
        let post = result.unwrap();
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.content, "Test content");
    }

    #[test]
    fn test_create_post_empty_title() {
        let new_post = NewPost {
            title: "".to_string(),
            content: "Test content".to_string(),
        };
        let result =PostService::new().create_post(new_post);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Title cannot be empty");
    }

    #[test]
    fn test_create_post_empty_content() {
        let new_post = NewPost {
            title: "Test Post".to_string(),
            content: "".to_string(),
        };
        let result =PostService::new().create_post(new_post);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Content cannot be empty");
    }

    #[test]
    fn test_create_post_whitespace_title() {
        let new_post = NewPost {
            title: "   ".to_string(),
            content: "Test content".to_string(),
        };
        let result =PostService::new().create_post(new_post);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Title cannot be empty");
    }

    #[test]
    fn test_create_post_whitespace_content() {
        let new_post = NewPost {
            title: "Test Post".to_string(),
            content: "   ".to_string(),
        };
        let result =PostService::new().create_post(new_post);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Content cannot be empty");
    }
}
