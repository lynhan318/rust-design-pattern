// INTENTS:
// Flyweight let you fit objects into the available amount of RAM by sharing common parts of state
// between multiple objects instead of keeping all of data in each object
//                                        ┌─────────────────────────────┐
//Client  ────────────────────────────────►                             │
//                                        │ FlyweightFactory            │
//                                        │                             │
//                                        │ + make_object()             │
//                                        │                             │
//                                        └─────────────┬───────────────┘
//                                                      │
//                                                      │
//                                                      │ cache/memory
//                                                      │
//                                                      │
//                                                      │
//                                         ┌────────────▼────────────────┐
//                                         │  Flyweight                  │
//                                         │  +shareableState            │
//                                         │  +doSomething()             │
//                                         │                             │
//                                         │                             │
//                                         └─────────────────────────────┘
//
use std::collections::HashMap;

#[derive(Clone)]
struct Article {
    pub id: u64,
    pub author: u64,
    pub content: String,
    pub title: String,
}

struct ArticleProvider {
    pub articles: HashMap<u64, Article>,
}
impl ArticleProvider {
    pub fn new_article(&mut self, id: u64, author: u64, title: String, content: String) {
        self.articles.insert(
            author,
            Article {
                id,
                author,
                content,
                title,
            },
        );
    }
    pub fn get(&self, id: u64) -> Option<&Article> {
        self.articles.get(&id)
    }
}

#[derive(Clone)]
struct Author {
    pub id: u64,
    pub name: String,
    pub age: u64,
}

struct UserProvider {
    pub users: HashMap<u64, Author>,
}
impl UserProvider {
    pub fn new_user(&mut self, id: u64, name: String, age: u64) {
        self.users.insert(id, Author { id, name, age });
    }
    pub fn get(&self, id: u64) -> Option<&Author> {
        self.users.get(&id)
    }
}

pub struct CommentResponse {
    author: Author,
    content: String,
    article: Article,
}
pub fn demo_flyweight() {
    // create and save all users in a hashmap, serve via a user's provider
    let mut users = UserProvider {
        users: HashMap::new(),
    };
    users.new_user(1, "Kevin".into(), 20);
    users.new_user(2, "Annie".into(), 19);

    // create and save all articles in a hashmap, serve via a article's provider
    let mut articles = ArticleProvider {
        articles: HashMap::new(),
    };
    articles.new_article(1, 1, "Title".into(), "Content".into());
    articles.new_article(2, 2, "Title".into(), "Content".into());

    let resp_comments = vec![
        CommentResponse {
            author: users.get(1).unwrap().clone(),
            content: "comment1".into(),
            article: articles.get(1).unwrap().clone(),
        },
        CommentResponse {
            author: users.get(1).unwrap().clone(),
            content: "comment2".into(),
            article: articles.get(1).unwrap().clone(),
        },
    ];
}
