use super::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(RuntimeOrigin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(RuntimeOrigin::signed(ALICE), false));
		assert_noop!(
			Flipper::set_value(RuntimeOrigin::signed(ALICE), false),
			crate::Error::<Test>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		let _ = Flipper::set_value(RuntimeOrigin::signed(ALICE), false);
		assert_ok!(Flipper::flip_value(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Flipper::value(), Some(true));
	});
}

#[test]
fn flip_value_ko() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Flipper::flip_value(RuntimeOrigin::signed(ALICE)),
			crate::Error::<Test>::NoneValue
		);
	});
}
