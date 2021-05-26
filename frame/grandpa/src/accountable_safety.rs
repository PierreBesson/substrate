// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sp_std::collections::btree_map::BTreeMap;

use sp_finality_grandpa::acc_safety::StoredAccountableSafetyState;
use sp_runtime::traits::Saturating;

use crate::AccountableSafetyState;

use super::{Pallet, Config, BlockNotIncluded};

pub trait AccountableSafety<T: Config> {
	/// Update the accountable safety state machine(s), if there are any active.
	fn update();

	/// Initiate the accountable safety protocol. This will be called when mutually inconsistent
	/// finalized blocks are detected.
	fn start_accountable_safety_protocol();

	/// Get the current state of the accountable safety protocol instance(s). This is used by the
	/// accountable safety worker to determine e.g if it needs to submit any query replies.
	fn state() -> Option<()>;

	/// Submit a response to a query where the reply can be either prevotes or precommits
	fn add_response();

	/// Submit a response to a query which specifically calls for prevotes.
	fn add_prevote_response();
}

impl<T: Config> AccountableSafety<T> for () {
	fn update() {}
	fn start_accountable_safety_protocol() {}
	fn state() -> Option<()> { None }
	fn add_response() {}
	fn add_prevote_response() {}
}

pub struct AccountableSafetyHandler;

impl<T: Config> AccountableSafety<T> for AccountableSafetyHandler {
	fn update() {
		let a = Pallet::<T>::block_not_included();
		log::info!("JON: a: {:?}", a);

		let a = a.unwrap_or(1u32.into());
		<BlockNotIncluded<T>>::put(a.saturating_add(1u32.into()));
	}

	fn start_accountable_safety_protocol() {
		// TODO: setup data structures
		let commit_for_block_not_included = sp_finality_grandpa::acc_safety::Commit::<T::BlockNumber> {
			target_number: 0u32.into(),
			precommits: Vec::new(),
		};

		let querying_rounds = BTreeMap::new();
		let prevote_queries = BTreeMap::new();

		let acc_state = StoredAccountableSafetyState {
			block_not_included: 0u32.into(),
			round_for_block_not_included: 0u32.into(),
			commit_for_block_not_included,
			querying_rounds,
			prevote_queries,
		};

		<AccountableSafetyState<T>>::put(acc_state);
	}

	fn state() -> Option<()> {
		todo!();
	}

	fn add_response() {
		todo!();
	}

	fn add_prevote_response() {
		todo!();
	}
}