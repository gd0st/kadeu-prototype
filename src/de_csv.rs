use crate::kadeu::{KCard, Kadeu};
use csv;
use serde::Deserialize;
use std::{error::Error, io, process};

#[derive(Deserialize, Debug)]
struct CsvCard {
    front: String,
    back: String,
}

pub fn parse_csv_data(file_data: &str) -> Vec<Box<dyn Kadeu>> {
    let mut record = csv::Reader::from_reader(file_data.as_bytes());

    let mut kcards: Vec<Box<dyn Kadeu>> = vec![];
    for record in record.deserialize() {
        let card: CsvCard = record.unwrap();
        kcards.push(KCard::Card(card.front, card.back).make());
    }

    kcards
}

#[cfg(test)]
mod test {
    use super::parse_csv_data;
    #[test]
    fn csv_parse_str_for_kadeu() {
        let csv_data = "\
front,back
foo,bar";
        let mut cards = parse_csv_data(csv_data);
        if let Some(card) = cards.pop() {
            assert_eq!(card.front(), "foo".to_string());
        } else {
            assert!(false)
        }
    }
}
