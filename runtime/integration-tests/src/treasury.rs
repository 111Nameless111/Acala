// This file is part of Acala.

// Copyright (C) 2020-2025 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::setup::*;

#[test]
fn treasury_should_take_xcm_execution_revenue() {
	ExtBuilder::default().build().execute_with(|| {
		let dot_amount = 1000 * dollar(RELAY_CHAIN_CURRENCY);

		#[cfg(feature = "with-mandala-runtime")]
		let shallow_weight = 3_000_000;
		#[cfg(feature = "with-karura-runtime")]
		let shallow_weight = 600_000_000;
		#[cfg(feature = "with-acala-runtime")]
		let shallow_weight = 600_000_000;
		let origin = Location::parent();

		// receive relay chain token
		let asset: Asset = (Location::parent(), dot_amount).into();
		let mut msg = Xcm(vec![
			ReserveAssetDeposited(asset.clone().into()),
			BuyExecution {
				fees: asset,
				weight_limit: Limited(Weight::from_parts(shallow_weight, 0)),
			},
			DepositAsset {
				assets: AllCounted(u32::max_value()).into(),
				beneficiary: Junction::AccountId32 {
					network: None,
					id: ALICE,
				}
				.into(),
			},
		]);
		use xcm_executor::traits::WeightBounds;
		let debt = <XcmConfig as xcm_executor::Config>::Weigher::weight(&mut msg).unwrap_or_default();
		assert_eq!(debt, Weight::from_parts(shallow_weight, 0));

		assert_eq!(Tokens::free_balance(RELAY_CHAIN_CURRENCY, &ALICE.into()), 0);
		assert_eq!(Tokens::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()), 0);

		let weight_limit = debt;
		let mut hash = msg.using_encoded(sp_io::hashing::blake2_256);
		assert_eq!(
			XcmExecutor::<XcmConfig>::prepare_and_execute(origin, msg, &mut hash, weight_limit, Weight::zero()),
			Outcome::Complete {
				used: Weight::from_parts(shallow_weight, 0)
			}
		);

		let actual_amount = Tokens::free_balance(RELAY_CHAIN_CURRENCY, &ALICE.into());
		#[cfg(feature = "with-mandala-runtime")]
		assert_debug_snapshot!(actual_amount, @"9999999719830");
		#[cfg(feature = "with-karura-runtime")]
		assert_debug_snapshot!(actual_amount, @"999999887932000");
		#[cfg(feature = "with-acala-runtime")]
		assert_debug_snapshot!(actual_amount, @"9999998879320");

		assert_eq!(
			Tokens::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
			dot_amount - actual_amount
		);
	});
}

#[test]
fn treasury_handles_dust_correctly() {
	ExtBuilder::default()
		.balances(vec![
			(
				AccountId::from(BOB),
				RELAY_CHAIN_CURRENCY,
				ExistentialDeposits::get(&RELAY_CHAIN_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				RELAY_CHAIN_CURRENCY,
				ExistentialDeposits::get(&RELAY_CHAIN_CURRENCY),
			),
			(
				AccountId::from(BOB),
				LIQUID_CURRENCY,
				ExistentialDeposits::get(&LIQUID_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				LIQUID_CURRENCY,
				ExistentialDeposits::get(&LIQUID_CURRENCY),
			),
			(
				AccountId::from(BOB),
				USD_CURRENCY,
				ExistentialDeposits::get(&USD_CURRENCY),
			),
			(
				AccountId::from(ALICE),
				USD_CURRENCY,
				ExistentialDeposits::get(&USD_CURRENCY),
			),
		])
		.build()
		.execute_with(|| {
			let relay_ed = ExistentialDeposits::get(&RELAY_CHAIN_CURRENCY);
			let liquid_ed = ExistentialDeposits::get(&LIQUID_CURRENCY);
			let usd_ed = ExistentialDeposits::get(&USD_CURRENCY);

			// Test empty treasury receives dust tokens of relay
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				0
			);
			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(AccountId::from(ALICE)),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				RELAY_CHAIN_CURRENCY,
				1
			));
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &AccountId::from(BOB)),
				relay_ed + 1
			);

			// ALICE account is reaped and treasury receives dust tokens
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &AccountId::from(ALICE)),
				0
			);
			// Treasury can have under the existential deposit
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				relay_ed - 1
			);

			// treasury can send funds when under existential deposit
			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(TreasuryAccount::get()),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				RELAY_CHAIN_CURRENCY,
				relay_ed - 2
			));
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				1
			);

			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(AccountId::from(BOB)),
				sp_runtime::MultiAddress::Id(AccountId::from(ALICE)),
				RELAY_CHAIN_CURRENCY,
				relay_ed
			));
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &AccountId::from(ALICE)),
				relay_ed
			);
			assert_eq!(Currencies::free_balance(RELAY_CHAIN_CURRENCY, &AccountId::from(BOB)), 0);
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				relay_ed
			);
			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(AccountId::from(ALICE)),
				sp_runtime::MultiAddress::Id(TreasuryAccount::get()),
				RELAY_CHAIN_CURRENCY,
				relay_ed
			));

			// Treasury is not reaped when going from over existential deposit to back under it
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				2 * relay_ed
			);
			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(TreasuryAccount::get()),
				sp_runtime::MultiAddress::Id(AccountId::from(ALICE)),
				RELAY_CHAIN_CURRENCY,
				relay_ed + 1
			));
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &AccountId::from(ALICE)),
				relay_ed + 1
			);
			assert_eq!(
				Currencies::free_balance(RELAY_CHAIN_CURRENCY, &TreasuryAccount::get()),
				relay_ed - 1
			);

			// Test empty treasury receives dust tokens of Liquid Currency
			assert_eq!(Currencies::free_balance(LIQUID_CURRENCY, &TreasuryAccount::get()), 0);
			assert_ok!(Currencies::transfer(
				RuntimeOrigin::signed(AccountId::from(ALICE)),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				LIQUID_CURRENCY,
				1
			));
			assert_eq!(
				Currencies::free_balance(LIQUID_CURRENCY, &AccountId::from(BOB)),
				liquid_ed + 1
			);
			assert_eq!(Currencies::free_balance(LIQUID_CURRENCY, &AccountId::from(ALICE)), 0);
			assert_eq!(
				Currencies::free_balance(LIQUID_CURRENCY, &TreasuryAccount::get()),
				liquid_ed - 1
			);

			// Test empty treasury receives dust tokens of USD Currency using Tokens pallet
			assert_eq!(Tokens::free_balance(USD_CURRENCY, &TreasuryAccount::get()), 0);
			assert_ok!(Tokens::transfer(
				RuntimeOrigin::signed(AccountId::from(ALICE)),
				sp_runtime::MultiAddress::Id(AccountId::from(BOB)),
				USD_CURRENCY,
				1
			));
			assert_eq!(Tokens::free_balance(USD_CURRENCY, &AccountId::from(BOB)), usd_ed + 1);
			assert_eq!(Tokens::free_balance(USD_CURRENCY, &AccountId::from(ALICE)), 0);
			assert_eq!(Tokens::free_balance(USD_CURRENCY, &TreasuryAccount::get()), usd_ed - 1);
		});
}

#[cfg(feature = "with-mandala-runtime")]
mod mandala_only_tests {
	use super::*;
	type NegativeImbalance = <Balances as PalletCurrency<AccountId>>::NegativeImbalance;
	use frame_support::{pallet_prelude::Decode, traits::OnUnbalanced};
	use pallet_authorship::EventHandler;

	#[test]
	fn treasury_handles_collator_rewards_correctly() {
		ExtBuilder::default()
			.balances(vec![(AccountId::from(ALICE), NATIVE_CURRENCY, dollar(NATIVE_CURRENCY))])
			.build()
			.execute_with(|| {
				let keys: SessionKeys = Decode::decode(&mut &[0u8; 128][..]).unwrap();
				assert_ok!(Session::set_keys(
					RuntimeOrigin::signed(AccountId::from(ALICE)),
					keys,
					vec![]
				));
				assert_ok!(CollatorSelection::set_desired_candidates(RuntimeOrigin::root(), 1));
				assert_ok!(CollatorSelection::register_as_candidate(RuntimeOrigin::signed(
					AccountId::from(ALICE)
				)));

				let pot_account_id = CollatorSelection::account_id();
				// Currently pot has ExistentialDeposits
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &pot_account_id),
					10 * cent(NATIVE_CURRENCY)
				);
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &AccountId::from(ALICE)),
					dollar(NATIVE_CURRENCY)
				);

				let min_reward = MinRewardDistributeAmount::get();

				// Only 20% of the fee went into the pot
				let tip = NegativeImbalance::new((min_reward - 1) * 10);
				let fee = NegativeImbalance::new(0);
				DealWithFees::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));

				// The amount above existential is below the `MinRewardDistributeAmount`.
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &pot_account_id),
					299_999_999_998
				);

				CollatorSelection::note_author(AccountId::from(BOB));
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &pot_account_id),
					299_999_999_998
				);
				assert_eq!(Currencies::free_balance(NATIVE_CURRENCY, &AccountId::from(BOB)), 0);

				// Put a little more money into the pot
				let tip = NegativeImbalance::new(10);
				let fee = NegativeImbalance::new(0);

				DealWithFees::on_unbalanceds(Some(fee).into_iter().chain(Some(tip)));

				// Now the above existential is above the `MinRewardDistributeAmount`.
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &pot_account_id),
					300_000_000_000
				);

				// Splits half of 300_000_000_000 to BOB
				CollatorSelection::note_author(AccountId::from(BOB));

				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &pot_account_id),
					200_000_000_000
				);
				assert_eq!(
					Currencies::free_balance(NATIVE_CURRENCY, &AccountId::from(BOB)),
					100_000_000_000
				);
			});
	}
}
