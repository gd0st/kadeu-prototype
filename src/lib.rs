pub mod config;
pub mod de;
pub mod errors;
pub mod interface;
pub mod model;

use interface::{KCard, controller::{Controller, Strategy}, Linear};

pub enum RepoSource {
    // TODO Will require network and async etc.
    Net,
    Local,
}

impl RepoSource {
    fn net() -> Self {
        Self::Net
    }

    fn local() -> Self {
        Self::Local
    }
}

pub struct KadeuRepo {
    source: RepoSource,
    path: String,

}

//impl KadeuRepo {
//pub fn new(path: String, source: RepoSource) -> Self {
//KadeuRepo { path, source }
//}
//
//pub fn list(&self) -> Vec<String> {
//match self.source {
//RepoSource::Local => {
//let filenames = dbg!(fs::read_dir(&self.path).expect("Local Kadeu Repo was read"));
//}
//_ => todo!(),
//};
//
//vec![]
//}
//
//// TODO make this &str
//pub fn get(&self, name: String) -> Result<Box<dyn KDeck>, KadeuError> {
//todo!()
//}
//}

enum Message {
    Answer(String),
    Feedback(String),
    Info(String),
}

enum State {
    Waiting,
    Ready,
    Done,
}

//impl App {
//pub fn new(deck: Box<dyn KDeck>, engine: Box<dyn Engine>) -> Self {
//App { deck, engine }
//}
//fn send_message(&self, message: Message) {
//match message {
//Message::Answer(answer) => self.engine.input(answer),
//_ => (),
//};
//}
//}

//TODO integrate this?
