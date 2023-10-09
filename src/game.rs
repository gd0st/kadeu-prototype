pub trait KCard {
    fn front(&self) -> String;
    fn back(&self) -> String;
    fn score(&self, answer: String) -> Score;
}

pub trait KDeck {
    type Front;
    type Back;
}

pub struct Score {
    hit: u32,
    total: u32,
}

pub enum Tally {
    Hit,
    Miss,
}
impl Into<Score> for Vec<Tally> {
    fn into(self) -> Score {
        let (mut hit, mut total) = (0, 0);
        self.iter().for_each(|tally| {
            if let Tally::Hit = tally {
                hit += 1;
                total += 1;
            } else {
                total += 1;
            }
        });
        Score { hit, total }
    }
}
impl Score {
    pub fn new() -> Score {
        Score { hit: 0, total: 0 }
    }
}
#[derive(Debug, Clone)]
pub enum Mode {
    Practice,
    Test,
    Hardcore,
}
pub struct Game {
    cards: Vec<Box<dyn KCard>>,
    mode: Mode,
}

impl Game {
    pub fn new(cards: Vec<Box<dyn KCard>>, mode: Mode) -> Game {
        Game { cards, mode }
    }
    pub fn answer(&mut self, answer: String) -> bool {
        let card = self.cards.pop();
        if let Some(card) = card {
            //let score = card.score(answer);
            // FIXME -- Should be reconfigured when score is working properly.
            let score = true;
            self.cards.push(card);
            return score;
        }
        false
    }
    pub fn cards(&self) -> &Vec<Box<dyn KCard>> {
        &self.cards
    }
}
