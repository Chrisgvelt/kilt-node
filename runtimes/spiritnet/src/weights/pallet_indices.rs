// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2023 BOTLabs GmbH

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

//! Autogenerated weights for pallet_indices
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// pallet
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=pallet-indices
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/pallet_indices.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_indices`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_indices::WeightInfo for WeightInfo<T> {
	// Storage: Indices Accounts (r:1 w:1)
	// Proof: Indices Accounts (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn claim() -> Weight {
		Weight::from_parts(23_245_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Proof: Indices Accounts (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		Weight::from_parts(30_412_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Proof: Indices Accounts (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn free() -> Weight {
		Weight::from_parts(25_174_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Proof: Indices Accounts (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(132), added: 2607, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		Weight::from_parts(29_725_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Indices Accounts (r:1 w:1)
	// Proof: Indices Accounts (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		Weight::from_parts(27_373_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
