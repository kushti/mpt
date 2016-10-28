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

#[cfg(target_os = "macos")]
fn geth_dir_path() -> PathBuf {
	let mut home = env::home_dir().expect("Failed to get home dir");
	home.push("Library");
	home.push("Ethereum");
	home
}

#[cfg(windows)]
/// Default path for ethereum installation on Windows
pub fn geth_dir_path() -> PathBuf {
	let mut home = env::home_dir().expect("Failed to get home dir");
	home.push("AppData");
	home.push("Roaming");
	home.push("Ethereum");
	home
}

#[cfg(not(any(target_os = "macos", windows)))]
/// Default path for ethereum installation on posix system which is not Mac OS
pub fn geth_dir_path() -> PathBuf {
	let mut home = env::home_dir().expect("Failed to get home dir");
	home.push(".ethereum");
	home
}

fn geth_keystore(t: DirectoryType) -> PathBuf {
	let mut dir = geth_dir_path();
	match t {
		DirectoryType::Testnet => {
			dir.push("testnet");
			dir.push("keystore");
		},
		DirectoryType::Main => {
			dir.push("keystore");
		}
	}
	dir
}

pub struct GethDirectory {
	dir: DiskDirectory,
}

impl GethDirectory {
	pub fn create(t: DirectoryType) -> Result<Self, Error> {
		let result = GethDirectory {
			dir: try!(DiskDirectory::create(geth_keystore(t))),
		};

		Ok(result)
	}

	pub fn open(t: DirectoryType) -> Self {
		GethDirectory {
			dir: DiskDirectory::at(geth_keystore(t)),
		}
	}
}

impl KeyDirectory for GethDirectory {
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
