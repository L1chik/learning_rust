use traits_2::Tweet;
use traits_2::NewsArticle;
use traits_2::Summary;
use traits_2::largest;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("sadasdasd"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };



    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largwst number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
