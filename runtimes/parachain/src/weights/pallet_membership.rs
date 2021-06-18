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

//! Autogenerated weights for pallet_membership
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-17, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain
// dev
// --heap-pages
// 4096
// --extrinsic
// *
// --pallet
// pallet_membership
// --steps
// 50
// --repeat
// 20
// --execution
// wasm
// --wasm-execution
// Compiled
// --output
// runtimes/parachain/src/weights/pallet_membership.rs
// --template
// .maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_membership using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	fn add_member(m: u32, ) -> Weight {
		36_406_000_u64
			// Standard Error: 3_000
			.saturating_add(187_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn remove_member(m: u32, ) -> Weight {
		45_048_000_u64
			// Standard Error: 1_000
			.saturating_add(143_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn swap_member(m: u32, ) -> Weight {
		44_310_000_u64
			// Standard Error: 2_000
			.saturating_add(190_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn reset_member(m: u32, ) -> Weight {
		46_904_000_u64
			// Standard Error: 2_000
			.saturating_add(357_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn change_key(m: u32, ) -> Weight {
		46_549_000_u64
			// Standard Error: 1_000
			.saturating_add(179_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn set_prime(m: u32, ) -> Weight {
		12_132_000_u64
			// Standard Error: 0
			.saturating_add(125_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn clear_prime(m: u32, ) -> Weight {
		4_955_000_u64
			// Standard Error: 0
			.saturating_add(5_000_u64.saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}