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

//! Transaction execution format module.

use util::{Bytes, U256, Address, U512};
use rlp::*;
use trace::{VMTrace, FlatTrace};
use types::log_entry::LogEntry;
use types::state_diff::StateDiff;
use std::fmt;

/// The type of the call-like instruction.
#[derive(Debug, PartialEq, Clone, Binary)]
pub enum CallType {
	/// Not a CALL.
	None,
	/// CALL.
	Call,
	/// CALLCODE.
	CallCode,
	/// DELEGATECALL.
	DelegateCall,
}

impl Encodable for CallType {
	fn rlp_append(&self, s: &mut RlpStream) {
		let v = match *self {
			CallType::None => 0u32,
			CallType::Call => 1,
			CallType::CallCode => 2,
			CallType::DelegateCall => 3,
		};
		s.append(&v);
	}
}

impl Decodable for CallType {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		decoder.as_rlp().as_val().and_then(|v| Ok(match v {
			0u32 => CallType::None,
			1 => CallType::Call,
			2 => CallType::CallCode,
			3 => CallType::DelegateCall,
			_ => return Err(DecoderError::Custom("Invalid value of CallType item")),
		}))
	}
}

/// Transaction execution receipt.
#[derive(Debug, PartialEq, Clone, Binary)]
pub struct Executed {
	/// Gas paid up front for execution of transaction.
	pub gas: U256,

	/// Gas used during execution of transaction.
	pub gas_used: U256,

	/// Gas refunded after the execution of transaction.
	/// To get gas that was required up front, add `refunded` and `gas_used`.
	pub refunded: U256,

	/// Cumulative gas used in current block so far.
	///
	/// `cumulative_gas_used = gas_used(t0) + gas_used(t1) + ... gas_used(tn)`
	///
	/// where `tn` is current transaction.
	pub cumulative_gas_used: U256,

	/// Vector of logs generated by transaction.
	pub logs: Vec<LogEntry>,

	/// Addresses of contracts created during execution of transaction.
	/// Ordered from earliest creation.
	///
	/// eg. sender creates contract A and A in constructor creates contract B
	///
	/// B creation ends first, and it will be the first element of the vector.
	pub contracts_created: Vec<Address>,
	/// Transaction output.
	pub output: Bytes,
	/// The trace of this transaction.
	pub trace: Vec<FlatTrace>,
	/// The VM trace of this transaction.
	pub vm_trace: Option<VMTrace>,
	/// The state diff, if we traced it.
	pub state_diff: Option<StateDiff>,
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone, Binary)]
pub enum ExecutionError {
	/// Returned when there gas paid for transaction execution is
	/// lower than base gas required.
	NotEnoughBaseGas {
		/// Absolute minimum gas required.
		required: U256,
		/// Gas provided.
		got: U256
	},
	/// Returned when block (gas_used + gas) > gas_limit.
	///
	/// If gas =< gas_limit, upstream may try to execute the transaction
	/// in next block.
	BlockGasLimitReached {
		/// Gas limit of block for transaction.
		gas_limit: U256,
		/// Gas used in block prior to transaction.
		gas_used: U256,
		/// Amount of gas in block.
		gas: U256
	},
	/// Returned when transaction nonce does not match state nonce.
	InvalidNonce {
		/// Nonce expected.
		expected: U256,
		/// Nonce found.
		got: U256
	},
	/// Returned when cost of transaction (value + gas_price * gas) exceeds
	/// current sender balance.
	NotEnoughCash {
		/// Minimum required balance.
		required: U512,
		/// Actual balance.
		got: U512
	},
	/// Returned when internal evm error occurs.
	Internal,
	/// Returned when generic transaction occurs
	TransactionMalformed(String),
}

impl fmt::Display for ExecutionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::ExecutionError::*;

		let msg = match *self {
			NotEnoughBaseGas { required, got } =>
				format!("Not enough base gas. {} is required, but only {} paid", required, got),
			BlockGasLimitReached { gas_limit, gas_used, gas } =>
				format!("Block gas limit reached. The limit is {}, {} has \
					already been used, and {} more is required", gas_limit, gas_used, gas),
			InvalidNonce { expected, got } =>
				format!("Invalid transaction nonce: expected {}, found {}", expected, got),
			NotEnoughCash { required, got } =>
				format!("Cost of transaction exceeds sender balance. {} is required \
					but the sender only has {}", required, got),
			Internal => "Internal evm error".into(),
			TransactionMalformed(ref err) => format!("Malformed transaction: {}", err),
		};

		f.write_fmt(format_args!("Transaction execution error ({}).", msg))
	}
}

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone, Binary)]
pub enum CallError {
	/// Couldn't find the transaction in the chain.
	TransactionNotFound,
	/// Couldn't find requested block's state in the chain.
	StatePruned,
	/// Error executing.
	Execution(ExecutionError),
}

impl From<ExecutionError> for CallError {
	fn from(error: ExecutionError) -> Self {
		CallError::Execution(error)
	}
}

impl fmt::Display for CallError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::CallError::*;

		let msg = match *self {
			TransactionNotFound => "Transaction couldn't be found in the chain".into(),
			StatePruned => "Couldn't find the transaction block's state in the chain".into(),
			Execution(ref e) => format!("{}", e),
		};

		f.write_fmt(format_args!("Transaction execution error ({}).", msg))
	}
}

/// Transaction execution result.
pub type ExecutionResult = Result<Executed, ExecutionError>;

#[test]
fn should_encode_and_decode_call_type() {
	use rlp;

	let original = CallType::Call;
	let encoded = rlp::encode(&original);
	let decoded = rlp::decode(&encoded);
	assert_eq!(original, decoded);
}