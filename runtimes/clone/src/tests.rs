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

use super::{Runtime, RuntimeCall};
use pallet_xcm::BalanceOf;
use parity_scale_codec::MaxEncodedLen;
use runtime_common::constants::MAX_INDICES_BYTE_LENGTH;

#[test]
fn call_size() {
	assert!(
		core::mem::size_of::<RuntimeCall>() <= 240,
		"size of Call is more than 240 bytes: some calls have too big arguments, use Box to reduce \
		the size of Call.
		If the limit is too strong, maybe consider increase the limit to 300.",
	);
}

#[test]
fn indices_storage_sizes() {
	type Indices = (<Runtime as frame_system::Config>::AccountId, BalanceOf<Runtime>, bool);

	let size = Indices::max_encoded_len();
	assert_eq!(size, MAX_INDICES_BYTE_LENGTH as usize)
}
