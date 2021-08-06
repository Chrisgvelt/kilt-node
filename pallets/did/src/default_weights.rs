// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-05, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --extrinsic=*
// --pallet=did
// --steps=1
// --repeat=20
// --template
// .maintain/weight-template.hbs
// --output
// pallets/did/src/default_weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight;
	fn update_ed25519_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight;
	fn update_sr25519_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight;
	fn update_ecdsa_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight;
	fn delete() -> Weight;
	fn submit_did_call_ed25519_key() -> Weight;
	fn submit_did_call_sr25519_key() -> Weight;
	fn submit_did_call_ecdsa_key() -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(71_462_000_u64)
			// Standard Error: 105_000
			.saturating_add((2_176_000_u64).saturating_mul(n as Weight))
			// Standard Error: 4_000
			.saturating_add((19_000_u64).saturating_mul(u as Weight))
			// Standard Error: 475_000
			.saturating_add((2_897_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(80_034_000_u64)
			// Standard Error: 111_000
			.saturating_add((1_580_000_u64).saturating_mul(n as Weight))
			// Standard Error: 5_000
			.saturating_add((20_000_u64).saturating_mul(u as Weight))
			// Standard Error: 502_000
			.saturating_add((1_458_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(163_338_000_u64)
			// Standard Error: 70_000
			.saturating_add((2_028_000_u64).saturating_mul(n as Weight))
			// Standard Error: 3_000
			.saturating_add((22_000_u64).saturating_mul(u as Weight))
			// Standard Error: 315_000
			.saturating_add((3_977_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn update_ed25519_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight {
		(19_548_000_u64)
			// Standard Error: 30_000
			.saturating_add((2_735_000_u64).saturating_mul(n as Weight))
			// Standard Error: 30_000
			.saturating_add((1_923_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((8_000_u64).saturating_mul(u as Weight))
			// Standard Error: 138_000
			.saturating_add((1_276_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn update_sr25519_keys(n: u32, m: u32, _u: u32, c: u32, ) -> Weight {
		(22_526_000_u64)
			// Standard Error: 73_000
			.saturating_add((3_146_000_u64).saturating_mul(n as Weight))
			// Standard Error: 73_000
			.saturating_add((2_083_000_u64).saturating_mul(m as Weight))
			// Standard Error: 331_000
			.saturating_add((1_177_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn update_ecdsa_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight {
		(22_988_000_u64)
			// Standard Error: 14_000
			.saturating_add((2_823_000_u64).saturating_mul(n as Weight))
			// Standard Error: 14_000
			.saturating_add((1_793_000_u64).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((4_000_u64).saturating_mul(u as Weight))
			// Standard Error: 64_000
			.saturating_add((319_000_u64).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn delete() -> Weight {
		(17_904_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(67_156_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(70_993_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(170_751_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_ed25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(71_462_000_u64)
			// Standard Error: 105_000
			.saturating_add((2_176_000_u64).saturating_mul(n as Weight))
			// Standard Error: 4_000
			.saturating_add((19_000_u64).saturating_mul(u as Weight))
			// Standard Error: 475_000
			.saturating_add((2_897_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn create_sr25519_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(80_034_000_u64)
			// Standard Error: 111_000
			.saturating_add((1_580_000_u64).saturating_mul(n as Weight))
			// Standard Error: 5_000
			.saturating_add((20_000_u64).saturating_mul(u as Weight))
			// Standard Error: 502_000
			.saturating_add((1_458_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn create_ecdsa_keys(n: u32, u: u32, c: u32, ) -> Weight {
		(163_338_000_u64)
			// Standard Error: 70_000
			.saturating_add((2_028_000_u64).saturating_mul(n as Weight))
			// Standard Error: 3_000
			.saturating_add((22_000_u64).saturating_mul(u as Weight))
			// Standard Error: 315_000
			.saturating_add((3_977_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn update_ed25519_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight {
		(19_548_000_u64)
			// Standard Error: 30_000
			.saturating_add((2_735_000_u64).saturating_mul(n as Weight))
			// Standard Error: 30_000
			.saturating_add((1_923_000_u64).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((8_000_u64).saturating_mul(u as Weight))
			// Standard Error: 138_000
			.saturating_add((1_276_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn update_sr25519_keys(n: u32, m: u32, _u: u32, c: u32, ) -> Weight {
		(22_526_000_u64)
			// Standard Error: 73_000
			.saturating_add((3_146_000_u64).saturating_mul(n as Weight))
			// Standard Error: 73_000
			.saturating_add((2_083_000_u64).saturating_mul(m as Weight))
			// Standard Error: 331_000
			.saturating_add((1_177_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn update_ecdsa_keys(n: u32, m: u32, u: u32, c: u32, ) -> Weight {
		(22_988_000_u64)
			// Standard Error: 14_000
			.saturating_add((2_823_000_u64).saturating_mul(n as Weight))
			// Standard Error: 14_000
			.saturating_add((1_793_000_u64).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((4_000_u64).saturating_mul(u as Weight))
			// Standard Error: 64_000
			.saturating_add((319_000_u64).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn delete() -> Weight {
		(17_904_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(67_156_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(70_993_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ecdsa_key() -> Weight {
		(170_751_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
