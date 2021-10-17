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

//! Autogenerated weights for kilt_launch
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=kilt_launch
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtimes/peregrine/src/weights/kilt_launch.rs
// --template=.maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for kilt_launch using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> kilt_launch::WeightInfo for WeightInfo<T> {
	fn change_transfer_account() -> Weight {
		(2_974_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn force_unlock(n: u32, ) -> Weight {
		(26_197_000_u64)
			// Standard Error: 21_000
			.saturating_add((28_402_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn locked_transfer() -> Weight {
		(137_035_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_vesting() -> Weight {
		(148_358_000_u64)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_locking() -> Weight {
		(155_535_000_u64)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	fn migrate_multiple_genesis_accounts_vesting(n: u32, ) -> Weight {
		(49_807_000_u64)
			// Standard Error: 71_000
			.saturating_add((96_386_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn migrate_multiple_genesis_accounts_locking(n: u32, ) -> Weight {
		(60_941_000_u64)
			// Standard Error: 53_000
			.saturating_add((97_982_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
}