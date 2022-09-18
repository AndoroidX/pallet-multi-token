use crate::mock::*;

use frame_support::{assert_ok};

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::credit(&1, &0, 100));
        assert_ok!(MultiToken::transfer(Origin::signed(1), 1, 2, 0, 5));
        assert_eq!(MultiToken::get_account(0, 2).unwrap(), 5);
        assert_ok!(MultiToken::transfer(Origin::signed(2), 2, 1, 0, 5));
        assert_eq!(MultiToken::get_account(0, 1).unwrap(), 100);
    });
}

#[test]
fn approve() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::approve(Origin::signed(1), 2, true));
        assert!(MultiToken::get_approval(1, 2).unwrap());
        assert!(!MultiToken::get_approval(2,1).unwrap_or(false));
        assert_ok!(MultiToken::approve(Origin::signed(1), 2, false));
        assert!(!MultiToken::get_approval(1, 2).unwrap());
    });
}

#[test]
fn transfer_with_delegate() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::credit(&1, &0, 100));
        assert_ok!(MultiToken::approve(Origin::signed(1), 2, true));
        assert_ok!(MultiToken::transfer(Origin::signed(2), 1, 3, 0, 5));
        assert_eq!(MultiToken::get_account(0, 3).unwrap(), 5);
    });
}

#[test]
#[should_panic]
fn transfer_with_unapproved_delegate() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::credit(&1, &0, 100));
        assert_ok!(MultiToken::transfer(Origin::signed(2), 1, 3, 0, 5));
        assert_eq!(MultiToken::get_account(0, 3).unwrap(), 5);
    });
}

#[test]
#[should_panic]
fn transfer_from_nonexisting_account() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::transfer(Origin::signed(1), 1, 2, 0, 5));
    });
}

#[test]
#[should_panic]
fn transfer_with_overflow() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::credit(&1, &0, u128::MAX-1));
        assert_ok!(MultiToken::credit(&2, &0, u128::MAX-1));
        assert_ok!(MultiToken::transfer(Origin::signed(1), 1, 2, 0, 5));
    });
}

#[test]
#[should_panic]
fn transfer_more_than_on_balance() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::credit(&1, &0, 100));
        assert_ok!(MultiToken::transfer(Origin::signed(1), 1, 2, 0, 105));
    });
}

#[test]
fn create_and_mint() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::create(Origin::signed(1)));
        assert_ok!(MultiToken::mint(Origin::signed(1), 0, 100));
        assert_eq!(MultiToken::get_asset_id_nonce().unwrap(), 0);
        assert_eq!(MultiToken::get_asset(0).unwrap(), 1);
        assert_eq!(MultiToken::get_account(0, 1).unwrap(), 100);
    });
}

#[test]
fn create_nonce_increment() {
    new_test_ext().execute_with(|| {
        assert_ok!(MultiToken::create(Origin::signed(1)));
        assert_eq!(MultiToken::get_asset_id_nonce().unwrap(), 0);
        assert_ok!(MultiToken::create(Origin::signed(1)));
        assert_eq!(MultiToken::get_asset_id_nonce().unwrap(), 1);
        assert_ok!(MultiToken::create(Origin::signed(2)));
        assert_eq!(MultiToken::get_asset_id_nonce().unwrap(), 2);
    });
}