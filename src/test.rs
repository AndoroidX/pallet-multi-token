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