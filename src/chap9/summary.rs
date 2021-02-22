pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Click here for summary")
    }
}