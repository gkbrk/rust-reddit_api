extern crate hyper;
extern crate serde_json;

use hyper::client::Client;
use hyper::header::UserAgent;
use serde_json::Value;

pub struct Reddit {
    user_agent: UserAgent
}

impl Reddit {
    pub fn new(user_agent: &str) -> Reddit {
        Reddit {
            user_agent: UserAgent(user_agent.to_string())
        }
    }

    pub fn get_subreddit(&self, name: &str) -> Subreddit {
        Subreddit {
            name: name.to_string(),
            user_agent: self.user_agent.clone()
        }
    }
}

pub struct Subreddit {
    name: String,
    user_agent: UserAgent
}

impl Subreddit {
    pub fn get_comments(&self) -> Vec<Comment> {
        let mut comments = Vec::new();

        let client = Client::new();

        let res = client.get(&format!("https://reddit.com/r/{}/comments.json", self.name))
            .header(self.user_agent.clone())
            .send().unwrap();

        let api_data: Value = serde_json::from_reader(res).unwrap();
        for api_comment in api_data.search("children").unwrap().as_array().unwrap() {
            comments.push(Comment::from_serde_value(api_comment));
        }

        return comments;
    }

    pub fn get_hot(&self) -> Vec<Submission> {
        let mut submissions = Vec::new();
        let client = Client::new();

        let res = client.get(&format!("https://reddit.com/r/{}/hot.json", self.name))
            .header(self.user_agent.clone())
            .send().unwrap();

        let api_data: Value = serde_json::from_reader(res).unwrap();
        for api_submission in api_data.search("children").unwrap().as_array().unwrap() {
            submissions.push(Submission::from_serde_value(api_submission));
        }

        return submissions;
    }
}

pub struct Comment {
    pub author: Option<String>,
    pub text: String,
    pub html: String
}

impl Comment {
    fn from_serde_value(value: &Value) -> Comment {
        let api_comment = value.as_object().unwrap().get("data").unwrap().as_object().unwrap();
        
        Comment {
            author: match api_comment.get("author").unwrap().as_string() {
                Some(author) => Some(author.to_string()),
                None => None
            },
            text: api_comment.get("body").unwrap().as_string().unwrap().to_string(),
            html: api_comment.get("body_html").unwrap().as_string().unwrap().to_string()
        }
    }
}

pub struct Submission {
    pub domain: String,
    pub title: String,
    pub url: String,
    pub text: String
}

impl Submission {
    fn from_serde_value(value: &Value) -> Submission {
        let api_submission = value.as_object().unwrap().get("data").unwrap().as_object().unwrap();

        Submission {
            domain: api_submission.get("domain").unwrap().as_string().unwrap().to_string(),
            title: api_submission.get("title").unwrap().as_string().unwrap().to_string(),
            url: api_submission.get("url").unwrap().as_string().unwrap().to_string(),
            text: api_submission.get("text").unwrap().as_string().unwrap().to_string()
        }
    }
}
