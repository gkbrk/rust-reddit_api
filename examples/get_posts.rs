extern crate reddit_api;

use reddit_api::Reddit;

fn main() {
    let r = Reddit::new("Reddit bot by /u/gkbrk");

    for comment in r.get_subreddit("rust").get_comments() {
        println!("Comment by {}: {}", comment.author.unwrap_or("[DELETED]".to_owned()), comment.text);
    }
}
