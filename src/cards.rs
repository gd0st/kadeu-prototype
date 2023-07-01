pub struct Card<T, U> {
	key: T,
	value: U
}

pub trait CardFactory {
	type KEY;
	type VALUE;
	fn new(key: Self::KEY, value: Self::VALUE) -> Self;
}

impl<T,U>  CardFactory for Card<T, U> {
	type KEY = T;
	type VALUE = U;
	fn new(key: Self::KEY, value: Self::VALUE) -> Self {
		Card { key, value }
	}
}

impl Kadeu for Card<String, String> {
	fn front(&self) -> &String {
		&self.key
	}
	fn back(&self) -> String {
		self.value.to_string()
	}
}

impl Kadeu for Card<String, usize> {
	fn front(&self) -> &String {
		&self.key
	}
	fn back(&self) -> String {
		self.value.to_string()
	}
}

impl Kadeu for Card<String, isize> {
	fn front(&self) -> &String {
		&self.key
	}
	fn back(&self) -> String {
		self.value.to_string()
	}
}

pub enum KCard {
	Simple(String, T),
	Complex(String, Vec<T>)
}

impl KCard {
	pub fn make(self) -> Box<dyn Kadeu> {
		match self {
			KCard::Simple(front, back) => Box::new(Card::new(front, back)),
			KCard::Complex(front, back) => Box::new(Card::new(front, back))
		}
	} 
}



pub enum Score {
	Miss,
	Accurate
}

pub trait Kadeu {
	fn front(&self) -> &String;
	fn back(&self) -> String;
}


#[cfg(test)]
mod test {

	use super::*;
	#[test]
	fn str_card(){
		let front = "foo".to_string();
		let back = "bar";
		let card = Card::new(front, back);
	}
	fn many_cards() {
		let front = "foo".to_string();
		let back = "bar".to_string();
		let cards: Vec<Box<dyn Kadeu>> = vec![
			Box::new(Card::new(front, back))
		];
	}
}
	