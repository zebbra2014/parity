// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use keccak::Keccak256;
use super::{KeyPair, Error, Generator, Secret};

/// Simple brainwallet.
pub struct Brain(String);

impl Brain {
	pub fn new(s: String) -> Self {
		Brain(s)
	}
}

impl Generator for Brain {
	fn generate(self) -> Result<KeyPair, Error> {
		let seed = self.0;
		let mut secret = seed.bytes().collect::<Vec<u8>>().keccak256();

		let mut i = 0;
		loop {
			secret = secret.keccak256();
			
			match i > 16384 {
				false => i += 1,
				true => {
					let result = KeyPair::from_secret(Secret::from(secret.clone()));
					if result.is_ok() {
						return result
					}
				},
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use {Brain, Generator};

	#[test]
	fn test_brain() {
		let words = "this is sparta!".to_owned();
		let first_keypair = Brain(words.clone()).generate().unwrap();
		let second_keypair = Brain(words.clone()).generate().unwrap();
		assert_eq!(first_keypair.secret(), second_keypair.secret());
	}
}