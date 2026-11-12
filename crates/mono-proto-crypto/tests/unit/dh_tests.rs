use super::*;

#[test]
fn factor_pq_rejects_prime_without_search_and_accepts_semiprimes() {
    assert!(matches!(
        factor_pq(0x7fff_ffff_ffff_ffe7),
        Err(Error::InvalidPq("value is prime"))
    ));
    assert_eq!(factor_pq(17 * 19).unwrap(), (17, 19));
    assert_eq!(factor_pq(1009 * 1013).unwrap(), (1009, 1013));
    assert!(factor_pq(41 * 41).is_err());
}

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
