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


extern crate ethcore_util;
#[macro_use] extern crate log;

extern crate time;

use time::PreciseTime;
use ethcore_util::hash::*;
use ethcore_util::bytes::*;
use ethcore_util::trie::*;

use ethcore_util::memorydb::*;
use ethcore_util::overlaydb::*;

use ethcore_util::triehash::*;
use ethcore_util::sha3::*;


fn random_value(seed: &mut H256) -> Bytes {
    *seed = seed.sha3();
    match seed[0] % 2 {
        1 => vec![seed[31];1],
        _ => seed.to_vec(),
    }
}

fn random_8_bytes(seed: &mut H256) -> Bytes {
    *seed = seed.sha3();
    seed[0..8].to_vec()
}

fn trie() {
    let initSize = 5000000;
    let blocks = 100000;
    let additions = 500;
    let modifications = 1500;

    const keyCacheSize: usize = 10000;


    let mut key_cache = vec![vec![0; 32]; keyCacheSize];

    let mut db = OverlayDB::new_temp();
    //let mut db = MemoryDB::new();

    let mut root = H256::new();
    let mut t = TrieDBMut::new(&mut db, &mut root);
    let mut seed = H256::new();

    for icnt in 0..initSize {
        let k = random_value(&mut seed);
        let v = random_8_bytes(&mut seed);
        t.insert(&k, &v).unwrap();

        if icnt < keyCacheSize {
            key_cache[icnt] = k;
        }

        if icnt % 10000 == 0 {
            println!("Initialized {}", icnt);
            t.commit();
        }
    }

    for bcnt in 0..blocks {
        for kcc in 0..500 {
            let sk = key_cache.pop().unwrap();
            key_cache.push(sk);
        }

        let start = PreciseTime::now();
        for cnt in 0..additions {
            let k = random_value(&mut seed);
            let v = random_8_bytes(&mut seed);
            t.insert(&k, &v).unwrap();

            if cnt % 50 == 0 {
                key_cache.pop();
                key_cache.push(k);
            }
        }

        for cnt in 0..modifications {
            let k = &key_cache[cnt];
            let v = random_8_bytes(&mut seed);

            t.insert(&k, &v).unwrap();
        }
        t.commit();
        let end = PreciseTime::now();
        println!("block: {}, time: {} ms", bcnt, start.to(end) * 1000);
    }
}


fn main() {
    trie();
}

/*
#[bench]
fn trie_insertions_32_mir_1k(b: &mut Bencher) {
	let st = StandardMap {
		alphabet: Alphabet::All,
		min_key: 32,
		journal_key: 0,
		value_mode: ValueMode::Mirror,
		count: 1000,
	};
	let d = st.make();
	let mut hash_count = 0usize;
	b.iter(&mut ||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
		hash_count = t.hash_count;
	});
//	println!("hash_count: {}", hash_count);
}
#[bench]
fn trie_iter(b: &mut Bencher) {
	let st = StandardMap {
		alphabet: Alphabet::All,
		min_key: 32,
		journal_key: 0,
		value_mode: ValueMode::Mirror,
		count: 1000,
	};
	let d = st.make();
	let mut memdb = MemoryDB::new();
	let mut root = H256::new();
	{
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
	}

	b.iter(&mut ||{
		let t = TrieDB::new(&memdb, &root).unwrap();
		for n in t.iter().unwrap() {
			black_box(n).unwrap();
		}
	});
}

#[bench]
fn triehash_insertions_32_mir_1k(b: &mut Bencher) {
	let st = StandardMap {
		alphabet: Alphabet::All,
		min_key: 32,
		journal_key: 0,
		value_mode: ValueMode::Mirror,
		count: 1000,
	};
	let d = st.make();
	b.iter(&mut ||{
		trie_root(d.clone()).clone();
	});
}

#[bench]
fn trie_insertions_32_ran_1k(b: &mut Bencher) {
	let st = StandardMap {
		alphabet: Alphabet::All,
		min_key: 32,
		journal_key: 0,
		value_mode: ValueMode::Random,
		count: 1000,
	};
	let d = st.make();
	let mut hash_count = 0usize;
	let mut r = H256::new();
	b.iter(&mut ||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
		hash_count = t.hash_count;
		r = t.root().clone();
	});
//	println!("result: {}", hash_count);
}

#[bench]
fn triehash_insertions_32_ran_1k(b: &mut Bencher) {
	let st = StandardMap {
		alphabet: Alphabet::All,
		min_key: 32,
		journal_key: 0,
		value_mode: ValueMode::Random,
		count: 1000,
	};
	let d = st.make();
	b.iter(&mut ||{
		trie_root(d.clone()).clone();
	});
}

#[bench]
fn trie_insertions_six_high(b: &mut Bencher) {
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_bytes(6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
	})
}

#[bench]
fn triehash_insertions_six_high(b: &mut Bencher) {
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_bytes(6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(&||{
		trie_root(d.clone());
	})
}

#[bench]
fn trie_insertions_six_mid(b: &mut Bencher) {
	let alphabet = b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}
	b.iter(||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
		debug!("hash_count={:?}", t.hash_count);
	})
}

#[bench]
fn triehash_insertions_six_mid(b: &mut Bencher) {
	let alphabet = b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}
	b.iter(||{
		trie_root(d.clone());
	})
}

#[bench]
fn trie_insertions_random_mid(b: &mut Bencher) {
	let alphabet = b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 1, 5, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
	})
}

#[bench]
fn triehash_insertions_random_mid(b: &mut Bencher) {
	let alphabet = b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 1, 5, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(||{
		trie_root(d.clone());
	})
}

#[bench]
fn trie_insertions_six_low(b: &mut Bencher) {
	let alphabet = b"abcdef";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(||{
		let mut memdb = MemoryDB::new();
		let mut root = H256::new();
		let mut t = TrieDBMut::new(&mut memdb, &mut root);
		for i in d.iter() {
			t.insert(&i.0, &i.1).unwrap();
		}
	})
}

#[bench]
fn triehash_insertions_six_low(b: &mut Bencher) {
	let alphabet = b"abcdef";
	let mut d: Vec<(Bytes, Bytes)> = Vec::new();
	let mut seed = H256::new();
	for _ in 0..1000 {
		let k = random_word(alphabet, 6, 0, &mut seed);
		let v = random_value(&mut seed);
		d.push((k, v))
	}

	b.iter(||{
		trie_root(d.clone());
	})
}

#[bench]
fn sha3x10000(b: &mut Bencher) {
	b.iter(||{
		let mut seed = H256::new();
		for _ in 0..10000 {
			seed = seed.sha3()
		}
	})
}
*/
