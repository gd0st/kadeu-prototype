use crate::{Card, CardModifier, Kadeu};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug)]
pub enum DeError {
    ParsingError(String),
}

impl fmt::Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Error for DeError {}

impl Kadeu for Card<String, String> {
    fn front(&self) -> String {
        self.front.to_owned()
    }

    fn back(&self) -> String {
        self.back.to_owned()
    }

    fn score(&self, answer: String) -> bool {
        let mut target = self.back.to_owned();
        // TODO make a generic Modifier (&T) -> T
        if let Some(modifiers) = &self.modifiers {
            for modfier in modifiers {
                match modfier {
                    CardModifier::IgnoreCase => target = target.to_lowercase(),
                }
            }
        }
        if target == answer {
            true
        } else {
            false
        }
    }
}
