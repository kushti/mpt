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

use serde::{Deserialize, Deserializer, Serialize, Serializer, Error};
use serde::de::{Visitor, MapVisitor};
use super::{Cipher, CipherSer, CipherSerParams, Kdf, KdfSer, KdfSerParams, H256, Bytes};

pub type CipherText = Bytes;

#[derive(Debug, PartialEq)]
pub struct Crypto {
	pub cipher: Cipher,
	pub ciphertext: CipherText,
	pub kdf: Kdf,
	pub mac: H256,
}

enum CryptoField {
	Cipher,
	CipherParams,
	CipherText,
	Kdf,
	KdfParams,
	Mac,
}

impl Deserialize for CryptoField {
	fn deserialize<D>(deserializer: &mut D) -> Result<CryptoField, D::Error>
		where D: Deserializer
	{
		deserializer.deserialize(CryptoFieldVisitor)
	}
}

struct CryptoFieldVisitor;

impl Visitor for CryptoFieldVisitor {
	type Value = CryptoField;

	fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
		where E: Error
	{
		match value {
			"cipher" => Ok(CryptoField::Cipher),
			"cipherparams" => Ok(CryptoField::CipherParams),
			"ciphertext" => Ok(CryptoField::CipherText),
			"kdf" => Ok(CryptoField::Kdf),
			"kdfparams" => Ok(CryptoField::KdfParams),
			"mac" => Ok(CryptoField::Mac),
			_ => Err(Error::custom(format!("Unknown field: '{}'", value))),
		}
	}
}

impl Deserialize for Crypto {
	fn deserialize<D>(deserializer: &mut D) -> Result<Crypto, D::Error>
		where D: Deserializer
	{
		static FIELDS: &'static [&'static str] = &["id", "version", "crypto", "Crypto", "address"];
		deserializer.deserialize_struct("Crypto", FIELDS, CryptoVisitor)
	}
}

struct CryptoVisitor;

impl Visitor for CryptoVisitor {
	type Value = Crypto;

	fn visit_map<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
		where V: MapVisitor
	{
		let mut cipher = None;
		let mut cipherparams = None;
		let mut ciphertext = None;
		let mut kdf = None;
		let mut kdfparams = None;
		let mut mac = None;

		loop {
			match try!(visitor.visit_key()) {
				Some(CryptoField::Cipher) => { cipher = Some(try!(visitor.visit_value())); }
				Some(CryptoField::CipherParams) => { cipherparams = Some(try!(visitor.visit_value())); }
				Some(CryptoField::CipherText) => { ciphertext = Some(try!(visitor.visit_value())); }
				Some(CryptoField::Kdf) => { kdf = Some(try!(visitor.visit_value())); }
				Some(CryptoField::KdfParams) => { kdfparams = Some(try!(visitor.visit_value())); }
				Some(CryptoField::Mac) => { mac = Some(try!(visitor.visit_value())); }
				None => { break; }
			}
		}

		let cipher = match (cipher, cipherparams) {
			(Some(CipherSer::Aes128Ctr), Some(CipherSerParams::Aes128Ctr(params))) => Cipher::Aes128Ctr(params),
			(None, _) => return Err(Error::missing_field("cipher")),
			(Some(_), None) => return Err(Error::missing_field("cipherparams")),
		};

		let ciphertext = match ciphertext {
			Some(ciphertext) => ciphertext,
			None => try!(visitor.missing_field("ciphertext")),
		};

		let kdf = match (kdf, kdfparams) {
			(Some(KdfSer::Pbkdf2), Some(KdfSerParams::Pbkdf2(params))) => Kdf::Pbkdf2(params),
			(Some(KdfSer::Scrypt), Some(KdfSerParams::Scrypt(params))) => Kdf::Scrypt(params),
			(Some(_), Some(_)) => return Err(Error::custom("Invalid cipherparams")),
			(None, _) => return Err(Error::missing_field("kdf")),
			(Some(_), None) => return Err(Error::missing_field("kdfparams")),
		};

		let mac = match mac {
			Some(mac) => mac,
			None => try!(visitor.missing_field("mac")),
		};

		try!(visitor.end());

		let result = Crypto {
			cipher: cipher,
			ciphertext: ciphertext,
			kdf: kdf,
			mac: mac,
		};

		Ok(result)
	}
}

impl Serialize for Crypto {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: Serializer
	{
		let mut state = try!(serializer.serialize_struct("Crypto", 6));
		match self.cipher {
			Cipher::Aes128Ctr(ref params) => {
				try!(serializer.serialize_struct_elt(&mut state, "cipher", &CipherSer::Aes128Ctr));
				try!(serializer.serialize_struct_elt(&mut state, "cipherparams", params));
			},
		}
		try!(serializer.serialize_struct_elt(&mut state, "ciphertext", &self.ciphertext));
		match self.kdf {
			Kdf::Pbkdf2(ref params) => {
				try!(serializer.serialize_struct_elt(&mut state, "kdf", &KdfSer::Pbkdf2));
				try!(serializer.serialize_struct_elt(&mut state, "kdfparams", params));
			},
			Kdf::Scrypt(ref params) => {
				try!(serializer.serialize_struct_elt(&mut state, "kdf", &KdfSer::Scrypt));
				try!(serializer.serialize_struct_elt(&mut state, "kdfparams", params));
			},
		}

		try!(serializer.serialize_struct_elt(&mut state, "mac", &self.mac));
		serializer.serialize_struct_end(state)
	}
}
