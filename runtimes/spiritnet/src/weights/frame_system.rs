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

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("spiritnet-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain=spiritnet-dev
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/frame_system.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for frame_system using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	fn remark(b: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 0
			.saturating_add((1_000_u64).saturating_mul(b as Weight))
	}
	fn remark_with_event(b: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 0
			.saturating_add((2_000_u64).saturating_mul(b as Weight))
	}
	fn set_heap_pages() -> Weight {
		(2_062_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn set_changes_trie_config() -> Weight {
		(7_869_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn set_storage(i: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 0
			.saturating_add((915_000_u64).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as Weight)))
	}
	fn kill_storage(i: u32, ) -> Weight {
		(0_u64)
			// Standard Error: 1_000
			.saturating_add((612_000_u64).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i as Weight)))
	}
	fn kill_prefix(p: u32, ) -> Weight {
		(7_328_000_u64)
			// Standard Error: 1_000
			.saturating_add((778_000_u64).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as Weight)))
	}
}