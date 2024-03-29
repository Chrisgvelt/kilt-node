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

//! Autogenerated weights for frame_system
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
// --pallet=frame-system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/spiritnet/src/weights/frame_system.rs
// --template=.maintain/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	fn set_code() -> Weight {
		Weight::from_parts(87_586_619_000, 1485)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remark(b: u32, ) -> Weight {
		Weight::from_parts(9_320_004 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(541 as u64, 0).saturating_mul(b as u64))
	}
	fn remark_with_event(b: u32, ) -> Weight {
		Weight::from_parts(8_660_000 as u64, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(2_041 as u64, 0).saturating_mul(b as u64))
	}
	// Storage: System Digest (r:1 w:1)
	// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: unknown `0x3a686561707061676573` (r:0 w:1)
	// Proof Skipped: unknown `0x3a686561707061676573` (r:0 w:1)
	fn set_heap_pages() -> Weight {
		Weight::from_parts(4_806_000 as u64, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	fn set_storage(i: u32, ) -> Weight {
		Weight::from_parts(3_953_000 as u64, 0)
			// Standard Error: 2_457
			.saturating_add(Weight::from_parts(642_798 as u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	fn kill_storage(i: u32, ) -> Weight {
		Weight::from_parts(2_542_000 as u64, 0)
			// Standard Error: 890
			.saturating_add(Weight::from_parts(478_434 as u64, 0).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
	fn kill_prefix(p: u32, ) -> Weight {
		Weight::from_parts(7_173_000 as u64, 0)
			// Standard Error: 1_138
			.saturating_add(Weight::from_parts(1_035_139 as u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}
