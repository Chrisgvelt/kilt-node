// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

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

//! Autogenerated weights for pallet_web3_names
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-02, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-web3-names
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/pallet-web3-names/src/default_weights.rs
// --template=.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_web3_names.
pub trait WeightInfo {
	fn claim(n: u32, ) -> Weight;
	fn release_by_owner() -> Weight;
	fn reclaim_deposit(n: u32, ) -> Weight;
	fn ban(n: u32, ) -> Weight;
	fn unban(n: u32, ) -> Weight;
	fn transfer_deposit() -> Weight;
}

/// Weights for pallet_web3_names using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: Web3Names Banned (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		Weight::from_ref_time(51_814_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(42_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn release_by_owner() -> Weight {
		Weight::from_ref_time(44_267_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn reclaim_deposit(n: u32, ) -> Weight {
		Weight::from_ref_time(43_834_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(19_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn ban(n: u32, ) -> Weight {
		Weight::from_ref_time(47_315_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(43_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	fn unban(n: u32, ) -> Weight {
		Weight::from_ref_time(20_297_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(32_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn transfer_deposit() -> Weight {
		Weight::from_ref_time(20_297_000 as u64)
			// Standard Error: 1_000
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: Web3Names Banned (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		Weight::from_ref_time(51_814_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(42_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Names (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn release_by_owner() -> Weight {
		Weight::from_ref_time(44_267_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn reclaim_deposit(n: u32, ) -> Weight {
		Weight::from_ref_time(43_834_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(19_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	// Storage: Web3Names Owner (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Web3Names Names (r:0 w:1)
	fn ban(n: u32, ) -> Weight {
		Weight::from_ref_time(47_315_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(43_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Web3Names Banned (r:1 w:1)
	fn unban(n: u32, ) -> Weight {
		Weight::from_ref_time(20_297_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(32_000 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn transfer_deposit() -> Weight {
		Weight::from_ref_time(20_297_000 as u64)
			// Standard Error: 1_000
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
