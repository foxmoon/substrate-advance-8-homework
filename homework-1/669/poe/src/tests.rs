use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn test_claim_creation_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();
        assert_ok!(PoeModule::create_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
        assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
    })
}

#[test]
fn test_claim_creation_failed_on_existed_claim() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn test_claim_revocation_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(
            RuntimeOrigin::signed(1),
            claim.clone()
        ));
    })
}

#[test]
fn test_claim_revocation_failed_when_claim_is_not_existed() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn test_claim_revocation_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn test_claim_transfer_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(
            RuntimeOrigin::signed(1),
            claim.clone(),
            2
        ));

        let bounded_claim =
            BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn test_claim_transfer_failed_on_claim_not_existed() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn test_claim_transfer_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1, 5, 10]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
    })
}
