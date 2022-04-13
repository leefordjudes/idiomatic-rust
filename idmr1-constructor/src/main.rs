use idmr1_constructor::{User, Post, PostStatus};


fn main() {
    // if ok, new returns its instance
    // if err, default instance will return
    // unwrap_or_default only works on type that has default implementation
    let user1 = User::new("defaultuser".to_owned())
                        .unwrap_or_default();
    println!("{:#?}", user1);
    let user2 = User::new("newuser".to_owned())
                        .unwrap_or_default();
    println!("{:#?}", user2);

    let post1 = Post::default();
    println!("{:#?}", post1);

    let post_status = PostStatus::new("pending".to_owned());
    println!("{:#?}", post_status);
}


