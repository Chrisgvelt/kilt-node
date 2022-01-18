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

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/pallet_vesting.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_vesting using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(54_335_000_u64)
			// Standard Error: 1_000
			.saturating_add((143_000_u64).saturating_mul(l as Weight))
			// Standard Error: 2_000
			.saturating_add((215_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(53_721_000_u64)
			// Standard Error: 1_000
			.saturating_add((130_000_u64).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((130_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(56_678_000_u64)
			// Standard Error: 2_000
			.saturating_add((130_000_u64).saturating_mul(l as Weight))
			// Standard Error: 5_000
			.saturating_add((191_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		(55_882_000_u64)
			// Standard Error: 1_000
			.saturating_add((133_000_u64).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((108_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		(95_991_000_u64)
			// Standard Error: 5_000
			.saturating_add((118_000_u64).saturating_mul(l as Weight))
			// Standard Error: 11_000
			.saturating_add((133_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		(95_180_000_u64)
			// Standard Error: 5_000
			.saturating_add((117_000_u64).saturating_mul(l as Weight))
			// Standard Error: 11_000
			.saturating_add((141_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(57_069_000_u64)
			// Standard Error: 1_000
			.saturating_add((154_000_u64).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((203_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(57_544_000_u64)
			// Standard Error: 1_000
			.saturating_add((145_000_u64).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((185_000_u64).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
