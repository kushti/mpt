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

use std::env;
use std::path::PathBuf;
use ethkey::Address;
use {SafeAccount, Error};
use super::{KeyDirectory, DiskDirectory, DirectoryType};

fn parity_dir_path() -> PathBuf {
	let mut home = env::home_dir().expect("Failed to get home dir");
	home.push(".parity");
	home
}

fn parity_keystore(t: DirectoryType) -> PathBuf {
	let mut dir = parity_dir_path();
	match t {
		DirectoryType::Testnet => {
			dir.push("testnet_keys");
		},
		DirectoryType::Main => {
			dir.push("keys");
		}
	}
	dir
}

pub struct ParityDirectory {
	dir: DiskDirectory,
}

impl ParityDirectory {
	pub fn create(t: DirectoryType) -> Result<Self, Error> {
		let result = ParityDirectory {
			dir: try!(DiskDirectory::create(parity_keystore(t))),
		};

		Ok(result)
	}

	pub fn open(t: DirectoryType) -> Self {
		ParityDirectory {
			dir: DiskDirectory::at(parity_keystore(t)),
		}
	}
}

impl KeyDirectory for ParityDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error> {
		self.dir.load()
	}

	fn insert(&self, account: SafeAccount) -> Result<SafeAccount, Error> {
		self.dir.insert(account)
	}

	fn remove(&self, address: &Address) -> Result<(), Error> {
		self.dir.remove(address)
	}
}
