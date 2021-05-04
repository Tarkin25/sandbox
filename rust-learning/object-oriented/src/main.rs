use object_oriented::{Screen, SelectBox, Button};
use object_oriented::blog::{Post};
use object_oriented::user::User;

fn main() {

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    let mut user = User::new(1, None, None);

    println!("{:?}", user);

    user = user.with_email(String::from("user@example.com"));

    println!("{:?}", user);

    user = user.with_name(String::from("user"));

    println!("{:?}", user);


    user = user.with_email(String::from("another@email.com")).with_name(String::from("another name"));

    println!("{:?}", user);
}
