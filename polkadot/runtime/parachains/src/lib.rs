// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Runtime modules for parachains code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(feature = "runtime-benchmarks", recursion_limit = "256")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod assigner;
pub mod assigner_on_demand;
pub mod assigner_parachains;
pub mod configuration;
pub mod disputes;
pub mod dmp;
pub mod hrmp;
pub mod inclusion;
pub mod initializer;
pub mod metrics;
pub mod origin;
pub mod paras;
pub mod paras_inherent;
pub mod reward_points;
pub mod scheduler;
pub mod session_info;
pub mod shared;

pub mod runtime_api_impl;

mod util;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod builder;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod ump_tests;

pub use origin::{ensure_parachain, Origin};
pub use paras::{ParaLifecycle, SetGoAhead};
use primitives::{HeadData, Id as ParaId, ValidationCode};
use sp_runtime::{DispatchResult, FixedU128};

/// Trait for tracking message delivery fees on a transport protocol.
pub trait FeeTracker {
	fn get_fee_factor(para: ParaId) -> FixedU128;
}

/// Schedule a para to be initialized at the start of the next session with the given genesis data.
pub fn schedule_para_initialize<T: paras::Config>(
	id: ParaId,
	genesis: paras::ParaGenesisArgs,
) -> Result<(), ()> {
	<paras::Pallet<T>>::schedule_para_initialize(id, genesis).map_err(|_| ())
}

/// Schedule a para to be cleaned up at the start of the next session.
pub fn schedule_para_cleanup<T: paras::Config>(id: primitives::Id) -> Result<(), ()> {
	<paras::Pallet<T>>::schedule_para_cleanup(id).map_err(|_| ())
}

/// Schedule a parathread (on-demand parachain) to be upgraded to a lease holding parachain.
pub fn schedule_parathread_upgrade<T: paras::Config>(id: ParaId) -> Result<(), ()> {
	paras::Pallet::<T>::schedule_parathread_upgrade(id).map_err(|_| ())
}

/// Schedule a lease holding parachain to be downgraded to an on-demand parachain.
pub fn schedule_parachain_downgrade<T: paras::Config>(id: ParaId) -> Result<(), ()> {
	paras::Pallet::<T>::schedule_parachain_downgrade(id).map_err(|_| ())
}

/// Schedules a validation code upgrade to a parachain with the given id.
pub fn schedule_code_upgrade<T: paras::Config>(
	id: ParaId,
	new_code: ValidationCode,
	set_go_ahead: SetGoAhead,
) -> DispatchResult {
	paras::Pallet::<T>::schedule_code_upgrade_external(id, new_code, set_go_ahead)
}

/// Sets the current parachain head with the given id.
pub fn set_current_head<T: paras::Config>(id: ParaId, new_head: HeadData) {
	paras::Pallet::<T>::set_current_head(id, new_head)
}
