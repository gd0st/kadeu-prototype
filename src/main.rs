use clap::Parser;
use kadeu::de_csv;
use kadeu::kadeu::{KCard, Kadeu};
use kadeu::util::{read_filepath, read_to_buff};
use std::io::{self, Write};

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    // Filepath of the deck that is to be parsed.
    #[arg(short, long)]
    flashcards: String,
    // TODO allow for flashcard csv file.
    //#[arg(long)]
    //flashcards: String,
    //#[arg(long)]
    //shuffle: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.flashcards);
    let contents = read_filepath(args.flashcards).expect("--filepath contents");
    // TODO json parser, yaml parser etc. A general deserializer implementation would be nice.
    let cards = de_csv::parse_csv_data(contents.as_str());
    for card in cards {
        let mut answer = String::new();
        println!("!> {}", card.front());
        print!("?: ");
        io::stdout().flush().expect("Flushed stdout");
        read_to_buff(&mut answer);
        let _ = card.score(answer).expect("Score response from Kadeu");
        println!("=> {}", card.back());
        // game state will allow for practice, test, etc.
        // Scheduler will determine how the cards are organized.
        // ignore_case etc will also be introduced for json cards.
    }
}

#[cfg(test)]
mod test {
    use kadeu::Score;

    use super::{KCard, Kadeu};

    #[test]
    fn make_string_kadeu() {
        let front = "What is the deepest man made hole?".to_string();
        let back = "Kola Superdeep Borehold".to_string();
        let card: Box<dyn Kadeu> = KCard::Card(front, back.to_owned()).make();
        let score = card.score(back.to_owned()).unwrap();
        assert_eq!(back.to_owned(), card.back());
        assert_eq!(score.to_string(), "Hit".to_string());
    }

    #[test]
    fn make_usize_kadeu() {
        let front = "How many feet deep is the Kola Superdeep Borehole?".to_string();
        let back = 40226;
        let card: Box<dyn Kadeu> = KCard::Card(front, back).make();
        let score = card.score(back.to_string()).unwrap();
        assert_eq!(back.to_string(), card.back());
        assert_eq!(score.to_string(), "Hit".to_string());
    }

    #[test]
    fn mixed_kadeu() {
        let cards = vec![
            KCard::Card(
                "Who is the third highest paid federal employee".to_string(),
                "The Postmaster General".to_string(),
            )
            .make(),
            KCard::Card(
                "How many K is the starting salary for the Postmaster General".to_string(),
                275,
            )
            .make(),
        ];

        cards.iter().for_each(|card| {
            // TODO need to engineer a better test.
            println!("{}", card.front());
            println!("{}", card.back());
        });
    }
}
