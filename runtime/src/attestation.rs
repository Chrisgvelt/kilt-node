
use traits::{Verify,Member};
use sr_primitives::verify_encoded_lazy;
use runtime_primitives::codec::{Codec};
use srml_support::{StorageMap, dispatch::Result};
use {balances, system::ensure_signed};
use rstd::prelude::*;

pub trait Trait: balances::Trait {
	type Signature: Verify<Signer=Self::AccountId> + Member + Codec + Default;
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn add(origin, claim_hash: T::Hash, signature: T::Signature) -> Result {
			let sender = ensure_signed(origin)?;
			if !verify_encoded_lazy(&signature, &claim_hash, &sender) {
				return Err("bad signature")
			}

			let mut existing_attestations_for_claim = <Attestations<T>>::get(claim_hash.clone());
			let mut last_attested : Option<(T::Hash,T::AccountId,T::Signature,bool)> = None;
			for v in existing_attestations_for_claim.clone() {
				if v.1.eq(&sender) {
					last_attested = Some(v);
					break;
				}
			}
			match last_attested {
				Some(_v)	=> return Err("already attested"),
				None	=> {
					existing_attestations_for_claim.push((claim_hash.clone(), sender.clone(), signature.clone(), false));
					<Attestations<T>>::insert(claim_hash.clone(), existing_attestations_for_claim);
					Ok(())
				},
			}
		}

		fn revoke(origin, claim_hash: T::Hash, signature: T::Signature) -> Result {
			let sender = ensure_signed(origin)?;
			if !verify_encoded_lazy(&signature, &claim_hash, &sender) {
				return Err("bad signature")
			}

			let existing_attestations_for_claim = <Attestations<T>>::get(claim_hash.clone());
			let mut last_attested : Option<(T::Hash,T::AccountId,T::Signature,bool)> = None;
			for v in existing_attestations_for_claim.clone() {
				if v.1.eq(&sender) {
					last_attested = Some(v);
				}
			}
			match last_attested {
				None	=> return Err("not attested"),
				Some(mut v)	=> {
					v.3 = true;
					Ok(())
				},
			}
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Attestation {
		Attestations get(attestations): map T::Hash => Vec<(T::Hash,T::AccountId,T::Signature,bool)>;
	}
}
