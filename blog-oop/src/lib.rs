use std::fmt;

fn type_of<T>(_: T) -> String {
    // format!("hello {}", "world!");
    format!("{}", std::any::type_name::<T>())
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

pub struct Post {
    pub state: Option<Box<dyn State>>,
    content: String,
}

/// 打印变量
/// https://lolimay.cn/2021/01/18/Rust/rust%E4%B8%AD%E6%80%8E%E4%B9%88%E5%88%A4%E6%96%AD%E4%B8%80%E4%B8%AA%E5%8F%98%E9%87%8F%E7%9A%84%E7%B1%BB%E5%9E%8B/
impl fmt::Debug for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Post")
            .field("state", &type_of(self.state.as_ref()))
            .field("content", &self.content)
            .finish()
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

struct Draft {}

impl State for Draft {
    /// 草稿可以请求审批
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    /// 已经通过审批的文章，申请审批不会有任何反应。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    /// 已经被审批的文章可以使用 approve 方法发布。
    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("{}", "Publishing");
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        println!("{}", &post.content);
        &post.content
    }
}
