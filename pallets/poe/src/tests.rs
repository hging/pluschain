use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn it_can_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
    let file = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), file.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(Proofs::<Test>::get(&file), (1, frame_system::Module::<Test>::block_number()));
	});
}

#[test]
fn correct_error_for_already_claim() {
  new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
    let file = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), file.clone());

    assert_noop!(
      PoeModule::create_claim(Origin::signed(1), file.clone()),
      Error::<Test>::ProofAlreadyClaimed
    );

    assert_noop!(
      PoeModule::create_claim(Origin::signed(2), file.clone()),
      Error::<Test>::ProofAlreadyClaimed
    );
	}); 
}

#[test]
fn correct_error_for_too_long() {
  new_test_ext().execute_with(|| {
    let file = vec![0u8; 513];
    assert_noop!(
      PoeModule::create_claim(Origin::signed(1), file.clone()),
      Error::<Test>::ProofTooLong
    );
  })
}

#[test]
fn it_can_revoke_claim() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    let _ = PoeModule::create_claim(Origin::signed(1), file.clone());
    assert_ok!(PoeModule::revoke_claim(Origin::signed(1), file.clone()));
    assert_eq!(Proofs::<Test>::get(&file), (0, 0));
  })
}

#[test]
fn correct_error_on_revoke_for_not_owner() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    let _ = PoeModule::create_claim(Origin::signed(1), file.clone());
    assert_noop!(
      PoeModule::revoke_claim(Origin::signed(2), file.clone()),
      Error::<Test>::NotProofOwner
    );
  })
}

#[test]
fn correct_error_on_revoke_for_file_not_exist() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    assert_noop!(
      PoeModule::revoke_claim(Origin::signed(1), file.clone()),
      Error::<Test>::NoSuchProof
    );
  })
}

#[test]
fn it_can_transfer_claim() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    let _ = PoeModule::create_claim(Origin::signed(1), file.clone());
    assert_ok!(PoeModule::transfer_claim(Origin::signed(1), 2, file.clone()));
		assert_eq!(Proofs::<Test>::get(&file), (2, frame_system::Module::<Test>::block_number()));
  })
}

#[test]
fn correct_error_on_transfer_for_not_owner() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    let _ = PoeModule::create_claim(Origin::signed(1), file.clone());
    assert_noop!(
      PoeModule::transfer_claim(Origin::signed(2), 3, file.clone()),
      Error::<Test>::NotProofOwner
    );
  })
}

#[test]
fn correct_error_on_transfer_for_file_not_exist() {
  new_test_ext().execute_with(|| {
    let file = vec![0, 1];
    assert_noop!(
      PoeModule::transfer_claim(Origin::signed(1), 2, file.clone()),
      Error::<Test>::NoSuchProof
    );
  })
}