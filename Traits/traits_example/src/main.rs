use aggregator::{Summary,Operations ,Tweet,NewsArticle};


fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn special_notify(item: &(impl Summary + Operations)) {
    println!("special notifay on {} {}",item.summarize(),item.counter());
}
fn returns_summarizable() -> impl Summary {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle{
        headline: String::from("Blabla"),
        location: String::from("Blabla"),
        author: String::from("Blabla"),
        content: String::from("Blabla"),
    };
    println!(" article is in {}",article.summarize());

    notify(&article);
    special_notify(&article);

    let item=returns_summarizable();
    notify(&item);

    
}