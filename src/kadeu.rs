pub trait Kadeu {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> Result<Score, String>;
}

pub trait KadeuDeck {
    fn title(&self) -> &String;
    fn cards(&self) -> Vec<dyn Kadeu>;
}
