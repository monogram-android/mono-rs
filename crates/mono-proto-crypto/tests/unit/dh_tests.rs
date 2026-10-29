use super::*;

#[test]
fn public_value_rejects_invalid_modulus_without_panicking() {
    for prime in [BigUint::from(0_u8), BigUint::from(1_u8)] {
        assert!(validate_dh_public_value(&BigUint::from(2_u8), &prime).is_err());
    }
}

#[test]
fn public_value_rejects_values_at_group_safety_margin() {
    let prime = (BigUint::one() << 2048_usize) - BigUint::from(159_u8);
    let margin = BigUint::one() << 1984_usize;
    assert!(validate_dh_public_value(&margin, &prime).is_err());
    assert!(validate_dh_public_value(&(&prime - &margin), &prime).is_err());
    assert!(validate_dh_public_value(&(&margin + BigUint::one()), &prime).is_ok());
}
