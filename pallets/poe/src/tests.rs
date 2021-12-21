use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// create claim test case
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim), 
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	});
}


// revoke claim test case
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_none() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn revoke_claim_failed_when_owner_is_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}


// transfer claim test case
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_none() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn transfer_claim_failed_when_owner_is_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim: Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	});
}