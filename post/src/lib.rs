#![allow(unused, unused_assignments)]

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    
 }

 pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
    }

    fn request_review(&mut self){
        if let Some(st) = self.state.take() {
            self.state = Some(st.request_review())
        }
    }

    fn publish(&mut self) {
        if let Some(st) = self.state.take() {
            self.state = Some(st.approve())
        }
    }
    
}

struct Draft { }
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(
            Pending {}
        )
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
}

struct Pending{ }
impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(
            Published {}
        ) 
    }
    
}

struct Published { }
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    
}

pub fn runner() {       
    let mut post = Post::new();
    
    post.add_text("\n\n\nI am happy to share that my Rust skills are boosting day by day. \n\nThanks to Goldy.");

    println!("Content: {}-", post.content());

    post.request_review();

    println!("Content: {}-", post.content());
    
    post.publish();

    println!("Content: {}", post.content());
}
