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
#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking

use crate::{AccountIdOf, Call, Config, ConnectedDids, CurrencyOf, Pallet};

use codec::Encode;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::traits::{Currency, Get};
use frame_system::RawOrigin;
use kilt_support::traits::GenerateBenchmarkOrigin;
use sp_io::crypto::sr25519_generate;
use sp_runtime::{app_crypto::sr25519, KeyTypeId};

const SEED: u32 = 0;

fn make_free_for_did<T: Config>(account: &AccountIdOf<T>) {
	let balance = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::minimum_balance() + <T as Config>::Deposit::get();
	<CurrencyOf<T> as Currency<AccountIdOf<T>>>::make_free_balance_be(account, balance);
}

benchmarks! {
	where_clause {
		where
		T::AccountId: From<sr25519::Public>,
		T::DidIdentifier: From<T::AccountId>,
		T::Signature: From<sr25519::Signature>,
		T::Signer: Default,
		T::EnsureOrigin: GenerateBenchmarkOrigin<T::Origin, T::AccountId, T::DidIdentifier>,
	}

	associate_account {
		let caller: T::AccountId = account("caller", 0, SEED);
		let did: T::DidIdentifier = account("did", 0, SEED);
		let connected_acc = sr25519_generate(KeyTypeId(*b"aura"), None);
		let connected_acc_id: T::AccountId = connected_acc.into();
		let bn: <T as frame_system::Config>::BlockNumber = 500_u32.into();

		let sig: T::Signature = sp_io::crypto::sr25519_sign(KeyTypeId(*b"aura"), &connected_acc, &Encode::encode(&(&did, bn))[..])
			.ok_or("Error while building signature.")?
			.into();

		make_free_for_did::<T>(&caller);
		let origin = T::EnsureOrigin::generate_origin(caller, did);
	}: _<T::Origin>(origin, connected_acc_id, bn, sig)
	verify {
		assert!(ConnectedDids::<T>::get(T::AccountId::from(connected_acc)).is_some());
	}

	associate_sender {
		let caller: T::AccountId = account("caller", 0, SEED);
		let did: T::DidIdentifier = account("did", 0, SEED);

		make_free_for_did::<T>(&caller);
		let origin = T::EnsureOrigin::generate_origin(caller.clone(), did);
	}: _<T::Origin>(origin)
	verify {
		assert!(ConnectedDids::<T>::get(caller).is_some());
	}

	remove_sender_association {
		let caller: T::AccountId = account("caller", 0, SEED);
		let did: T::DidIdentifier = account("did", 0, SEED);

		make_free_for_did::<T>(&caller);
		Pallet::<T>::add_association(caller.clone(), did, caller.clone()).expect("should create association");

		let origin = RawOrigin::Signed(caller.clone());
	}: _(origin)
	verify {
		assert!(ConnectedDids::<T>::get(caller).is_none());
	}

	remove_account_association {
		let caller: T::AccountId = account("caller", 0, SEED);
		let did: T::DidIdentifier = account("did", 0, SEED);
		make_free_for_did::<T>(&caller);

		Pallet::<T>::add_association(caller.clone(), did.clone(), caller.clone()).expect("should create association");

		let origin = T::EnsureOrigin::generate_origin(caller.clone(), did);
	}: _<T::Origin>(origin, caller.clone())
	verify {
		assert!(ConnectedDids::<T>::get(caller).is_none());
	}
}

#[cfg(test)]
use crate::Pallet as DidLookup;

impl_benchmark_test_suite!(
	DidLookup,
	crate::mock::ExtBuilder::default().build_with_keystore(),
	crate::mock::Test
);
