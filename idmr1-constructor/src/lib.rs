use derive_new::new;
use rand::prelude::*;

#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin
}

#[derive(Debug)]
pub struct User {
    id: u32,
    pub username: String,
    pub role: Role,
}

impl User {
    // 1. fn new act as a user defined constructor
    // 2. because fn new is defined as associated fn, it can access its private fields
    // 3. we can set a default value for all or some fields, using this constructor.
    // 4. we do some prior work before constructing instance
    pub fn new(username: String) -> Result<Self, String> {
        // check if username already exists
        if username == "defaultuser" {
            return Err("username already exists!".to_owned());
        }
        Ok(Self {
            id: thread_rng().gen_range(0..99999999),
            username,
            role: Role::Creator,
        })
    }
}

impl Default for User {
    // 1. default constructor
    // 2. default trait has one fn default, takes no args, returns Self
    // 3. still we can do prior work
    fn default() -> Self {
        let id = thread_rng().gen_range(0..99999999);
        Self {
            id,
            username: format!("guest{id}"),
            role: Role::Guest,
        }
    }
}

// all struct fields have its own default trait implementation
#[derive(Debug, Default)]
pub struct Post {
    content: String,
    tags: Vec<String>,
    likes: u32,
}

// on creating new instance
// you need to give value for fields that has not have #new annotation, 
#[derive(Debug, Default, new)]
pub struct PostStatus {
    status: String,
    #[new(value="vec![\"no comments\".to_owned()]")]
    comments: Vec<String>,
    #[new(default)]
    likes: u32,
}