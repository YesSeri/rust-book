mod chap9;
use chap9::summary::Summary;
fn main() {
    chap9::largest::run();
    // let t = chap9::tweet::Tweet{
    //     username: String::from("BigWarrior"),
    //     content: String::from("Some cool content"),
    //     reply: false,
    //     retweet: false,
    // };

    // notify(&t);
}


fn notify<T: Summary>(item: &T){
    println!("A summary: {}", item.summarize());
}