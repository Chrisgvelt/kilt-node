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
//! DATE: 2021-06-11, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./kilt-parachain
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// did
// --extrinsic
// *
// --steps
// 1
// --repeat
// 10
// --heap-pages=4096
// --output=../../pallets/did/src/default_weights.rs
// --template=../../.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for did.
pub trait WeightInfo {
	fn submit_did_create_operation_ed25519_keys(n: u32, u: u32, ) -> Weight;
	fn submit_did_create_operation_sr25519_keys(n: u32, u: u32, ) -> Weight;
	fn submit_did_update_operation_ed25519_keys(n: u32, m: u32, u: u32, ) -> Weight;
	fn submit_did_update_operation_sr25519_keys(n: u32, m: u32, u: u32, ) -> Weight;
	fn submit_did_delete_operation() -> Weight;
	fn submit_did_call_ed25519_key() -> Weight;
	fn submit_did_call_sr25519_key() -> Weight;
}

/// Weights for did using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn submit_did_create_operation_ed25519_keys(n: u32, _u: u32, ) -> Weight {
		(84_486_000_u64)
			// Standard Error: 64_000
			.saturating_add((3_433_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_create_operation_sr25519_keys(n: u32, u: u32, ) -> Weight {
		(89_550_000_u64)
			// Standard Error: 53_000
			.saturating_add((2_249_000_u64).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((6_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_ed25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		(78_788_000_u64)
			// Standard Error: 171_000
			.saturating_add((2_961_000_u64).saturating_mul(n as Weight))
			// Standard Error: 171_000
			.saturating_add((3_032_000_u64).saturating_mul(m as Weight))
			// Standard Error: 8_000
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_sr25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		(62_987_000_u64)
			// Standard Error: 133_000
			.saturating_add((4_353_000_u64).saturating_mul(n as Weight))
			// Standard Error: 133_000
			.saturating_add((3_464_000_u64).saturating_mul(m as Weight))
			// Standard Error: 6_000
			.saturating_add((30_000_u64).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_delete_operation() -> Weight {
		(74_610_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(77_355_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(79_730_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn submit_did_create_operation_ed25519_keys(n: u32, _u: u32, ) -> Weight {
		(84_486_000_u64)
			// Standard Error: 64_000
			.saturating_add((3_433_000_u64).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_create_operation_sr25519_keys(n: u32, u: u32, ) -> Weight {
		(89_550_000_u64)
			// Standard Error: 53_000
			.saturating_add((2_249_000_u64).saturating_mul(n as Weight))
			// Standard Error: 2_000
			.saturating_add((6_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_ed25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		(78_788_000_u64)
			// Standard Error: 171_000
			.saturating_add((2_961_000_u64).saturating_mul(n as Weight))
			// Standard Error: 171_000
			.saturating_add((3_032_000_u64).saturating_mul(m as Weight))
			// Standard Error: 8_000
			.saturating_add((24_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_sr25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		(62_987_000_u64)
			// Standard Error: 133_000
			.saturating_add((4_353_000_u64).saturating_mul(n as Weight))
			// Standard Error: 133_000
			.saturating_add((3_464_000_u64).saturating_mul(m as Weight))
			// Standard Error: 6_000
			.saturating_add((30_000_u64).saturating_mul(u as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_delete_operation() -> Weight {
		(74_610_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		(77_355_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		(79_730_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
