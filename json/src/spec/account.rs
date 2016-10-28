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

//! Spec account deserialization.

use uint::Uint;
use bytes::Bytes;
use spec::builtin::Builtin;

/// Spec account.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Account {
	/// Builtin contract.
	pub builtin: Option<Builtin>,
	/// Balance.
	pub balance: Option<Uint>,
	/// Nonce.
	pub nonce: Option<Uint>,
	/// Code.
	pub code: Option<Bytes>
}

impl Account {
	/// Returns true if account does not have nonce and balance.
	pub fn is_empty(&self) -> bool {
		self.balance.is_none() && self.nonce.is_none()
	}
}

#[cfg(test)]
mod tests {
	use serde_json;
	use spec::account::Account;
	use util::U256;
	use uint::Uint;
	use bytes::Bytes;

	#[test]
	fn account_deserialization() {
		let s = r#"{
			"balance": "1",
			"nonce": "0",
			"builtin": { "name": "ecrecover", "pricing": { "linear": { "base": 3000, "word": 0 } } },
			"code": "1234"
		}"#;
		let deserialized: Account = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized.balance.unwrap(), Uint(U256::from(1)));
		assert_eq!(deserialized.nonce.unwrap(), Uint(U256::from(0)));
		assert_eq!(deserialized.code.unwrap(), Bytes::new(vec![0x12, 0x34]));
		assert!(deserialized.builtin.is_some()); // Further tested in builtin.rs
	}
}
