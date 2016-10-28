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

use std::sync::Arc;
use std::str::FromStr;
use jsonrpc_core::IoHandler;
use util::{U256, Uint, Address};
use ethcore::account_provider::AccountProvider;
use v1::{PersonalClient, Personal};
use v1::tests::helpers::TestMinerService;
use ethcore::client::TestBlockChainClient;
use ethcore::transaction::{Action, Transaction};

struct PersonalTester {
	accounts: Arc<AccountProvider>,
	io: IoHandler,
	miner: Arc<TestMinerService>,
	// these unused fields are necessary to keep the data alive
	// as the handler has only weak pointers.
	_client: Arc<TestBlockChainClient>,
}

fn blockchain_client() -> Arc<TestBlockChainClient> {
	let client = TestBlockChainClient::new();
	Arc::new(client)
}

fn accounts_provider() -> Arc<AccountProvider> {
	Arc::new(AccountProvider::transient_provider())
}

fn miner_service() -> Arc<TestMinerService> {
	Arc::new(TestMinerService::default())
}

fn setup() -> PersonalTester {
	let accounts = accounts_provider();
	let client = blockchain_client();
	let miner = miner_service();
	let personal = PersonalClient::new(&accounts, &client, &miner, false);

	let io = IoHandler::new();
	io.add_delegate(personal.to_delegate());

	let tester = PersonalTester {
		accounts: accounts,
		io: io,
		miner: miner,
		_client: client,
	};

	tester
}

#[test]
fn accounts() {
	let tester = setup();
	let address = tester.accounts.new_account("").unwrap();
	let request = r#"{"jsonrpc": "2.0", "method": "personal_listAccounts", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":[""#.to_owned() + &format!("0x{:?}", address) + r#""],"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn new_account() {
	let tester = setup();
	let request = r#"{"jsonrpc": "2.0", "method": "personal_newAccount", "params": ["pass"], "id": 1}"#;

	let res = tester.io.handle_request_sync(request);

	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];
	let response = r#"{"jsonrpc":"2.0","result":""#.to_owned() + format!("0x{:?}", address).as_ref() + r#"","id":1}"#;

	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_get_account_info() {
	let tester = setup();
	tester.accounts.new_account("").unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();
	tester.accounts.set_account_name(address.clone(), "Test".to_owned()).unwrap();
	tester.accounts.set_account_meta(address.clone(), "{foo: 69}".to_owned()).unwrap();

	let request = r#"{"jsonrpc": "2.0", "method": "personal_accountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{}\":{{\"meta\":\"{{foo: 69}}\",\"name\":\"Test\",\"uuid\":\"{}\"}}}},\"id\":1}}", address.hex(), uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_set_name() {
	let tester = setup();
	tester.accounts.new_account("").unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = format!(r#"{{"jsonrpc": "2.0", "method": "personal_setAccountName", "params": ["0x{}", "Test"], "id": 1}}"#, address.hex());
	let response = r#"{"jsonrpc":"2.0","result":null,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();

	let request = r#"{"jsonrpc": "2.0", "method": "personal_accountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{}\":{{\"meta\":\"{{}}\",\"name\":\"Test\",\"uuid\":\"{}\"}}}},\"id\":1}}", address.hex(), uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_set_meta() {
	let tester = setup();
	tester.accounts.new_account("").unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = format!(r#"{{"jsonrpc": "2.0", "method": "personal_setAccountMeta", "params": ["0x{}", "{{foo: 69}}"], "id": 1}}"#, address.hex());
	let response = r#"{"jsonrpc":"2.0","result":null,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();

	let request = r#"{"jsonrpc": "2.0", "method": "personal_accountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{}\":{{\"meta\":\"{{foo: 69}}\",\"name\":\"{}\",\"uuid\":\"{}\"}}}},\"id\":1}}", address.hex(), uuid, uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn sign_and_send_transaction_with_invalid_password() {
	let tester = setup();
	let address = tester.accounts.new_account("password123").unwrap();
	let request = r#"{
		"jsonrpc": "2.0",
		"method": "personal_signAndSendTransaction",
		"params": [{
			"from": ""#.to_owned() + format!("0x{:?}", address).as_ref() + r#"",
			"to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
			"gas": "0x76c0",
			"gasPrice": "0x9184e72a000",
			"value": "0x9184e72a"
		}, "password321"],
		"id": 1
	}"#;

	let response = r#"{"jsonrpc":"2.0","error":{"code":-32021,"message":"Account password is invalid or account does not exist.","data":"SStore(InvalidPassword)"},"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request.as_ref()), Some(response.into()));
}

#[test]
fn sign_and_send_transaction() {
	let tester = setup();
	let address = tester.accounts.new_account("password123").unwrap();

	let request = r#"{
		"jsonrpc": "2.0",
		"method": "personal_signAndSendTransaction",
		"params": [{
			"from": ""#.to_owned() + format!("0x{:?}", address).as_ref() + r#"",
			"to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
			"gas": "0x76c0",
			"gasPrice": "0x9184e72a000",
			"value": "0x9184e72a"
		}, "password123"],
		"id": 1
	}"#;

	let t = Transaction {
		nonce: U256::zero(),
		gas_price: U256::from(0x9184e72a000u64),
		gas: U256::from(0x76c0),
		action: Action::Call(Address::from_str("d46e8dd67c5d32be8058bb8eb970870f07244567").unwrap()),
		value: U256::from(0x9184e72au64),
		data: vec![]
	};
	tester.accounts.unlock_account_temporarily(address, "password123".into()).unwrap();
	let signature = tester.accounts.sign(address, None, t.hash()).unwrap();
	let t = t.with_signature(signature);

	let response = r#"{"jsonrpc":"2.0","result":""#.to_owned() + format!("0x{:?}", t.hash()).as_ref() + r#"","id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request.as_ref()), Some(response));

	tester.miner.last_nonces.write().insert(address.clone(), U256::zero());

	let t = Transaction {
		nonce: U256::one(),
		gas_price: U256::from(0x9184e72a000u64),
		gas: U256::from(0x76c0),
		action: Action::Call(Address::from_str("d46e8dd67c5d32be8058bb8eb970870f07244567").unwrap()),
		value: U256::from(0x9184e72au64),
		data: vec![]
	};
	tester.accounts.unlock_account_temporarily(address, "password123".into()).unwrap();
	let signature = tester.accounts.sign(address, None, t.hash()).unwrap();
	let t = t.with_signature(signature);

	let response = r#"{"jsonrpc":"2.0","result":""#.to_owned() + format!("0x{:?}", t.hash()).as_ref() + r#"","id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request.as_ref()), Some(response));
}