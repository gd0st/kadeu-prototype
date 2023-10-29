// Now that I have figured out lifetimes, my code is going off the rails a bit...
//TODO place in own module
pub struct Score {
    hit: u32,
    total: u32,
}

pub enum Tally {
    Hit,
    Miss,
}
// This concept doesn't really work.
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

enum Message {
    Message(String),
    End,
}
