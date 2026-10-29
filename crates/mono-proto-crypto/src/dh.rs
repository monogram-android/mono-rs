//! Bounded factorization and Diffie–Hellman validation.

use crate::Error;
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn is_probable_prime(value: &BigUint) -> bool {
    let two = BigUint::from(2_u8);
    if value < &two {
        return false;
    }
    for prime in [2_u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let prime = BigUint::from(prime);
        if value == &prime {
            return true;
        }
        if value % &prime == BigUint::zero() {
            return false;
        }
    }
    let one = BigUint::one();
    let n_minus_one = value - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0_u32;
    while (&d & &one) == BigUint::zero() {
        d >>= 1;
        s += 1;
    }
    'witness: for base in [2_u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = BigUint::from(base).modpow(&d, value);
        if x == one || x == n_minus_one {
            continue;
        }
        for _ in 1..s {
            x = x.modpow(&two, value);
            if x == n_minus_one {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn gcd_u64(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn rho_step(value: u64, constant: u64, modulus: u64) -> u64 {
    u64::try_from(
        (u128::from(value) * u128::from(value) + u128::from(constant)) % u128::from(modulus),
    )
    .expect("value reduced modulo a u64")
}

/// Factor Telegram's authorization `pq` proof-of-work value into `p < q`.
///
/// Telegram constrains `pq` to a positive signed 64-bit semiprime. The bounded
/// deterministic Pollard-rho search avoids attacker-controlled unbounded work.
pub fn factor_pq(pq: u64) -> Result<(u64, u64), Error> {
    if pq < 6 || pq > i64::MAX as u64 {
        return Err(Error::InvalidPq(
            "value is outside Telegram's signed 64-bit range",
        ));
    }
    for divisor in [2_u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if pq % divisor == 0 {
            let other = pq / divisor;
            if other == divisor || !is_probable_prime(&BigUint::from(other)) {
                return Err(Error::InvalidPq("factors are not distinct primes"));
            }
            return Ok((divisor.min(other), divisor.max(other)));
        }
    }
    for constant in 1_u64..=64 {
        let mut slow = 2_u64;
        let mut fast = 2_u64;
        for _ in 0..2_000_000 {
            slow = rho_step(slow, constant, pq);
            fast = rho_step(rho_step(fast, constant, pq), constant, pq);
            let divisor = gcd_u64(slow.abs_diff(fast), pq);
            if divisor == 1 {
                continue;
            }
            if divisor == pq {
                break;
            }
            let other = pq / divisor;
            if divisor == other
                || !is_probable_prime(&BigUint::from(divisor))
                || !is_probable_prime(&BigUint::from(other))
            {
                return Err(Error::InvalidPq("factors are not distinct primes"));
            }
            return Ok((divisor.min(other), divisor.max(other)));
        }
    }
    Err(Error::InvalidPq("bounded factorization search failed"))
}

/// Validate a 2048-bit MTProto safe prime and approved generator.
pub fn validate_dh_parameters(prime: &BigUint, generator: u32) -> Result<(), Error> {
    if prime.bits() != 2048 {
        return Err(Error::InvalidDh("prime is not exactly 2048 bits"));
    }
    if !(2..=7).contains(&generator) {
        return Err(Error::InvalidDh("generator is outside 2..=7"));
    }
    let residue = |modulus: u32| {
        (prime % BigUint::from(modulus))
            .to_u32_digits()
            .first()
            .copied()
            .unwrap_or(0)
    };
    let generator_ok = match generator {
        2 => residue(8) == 7,
        3 => residue(3) == 2,
        4 => true,
        5 => matches!(residue(5), 1 | 4),
        6 => matches!(residue(24), 19 | 23),
        7 => matches!(residue(7), 3 | 5 | 6),
        _ => false,
    };
    if !generator_ok {
        return Err(Error::InvalidDh("generator residue check failed"));
    }
    if !is_probable_prime(prime) {
        return Err(Error::InvalidDh("prime failed primality checks"));
    }
    let q = (prime - BigUint::one()) >> 1;
    if !is_probable_prime(&q) {
        return Err(Error::InvalidDh("(p - 1) / 2 failed primality checks"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "../tests/unit/dh_tests.rs"]
mod tests;

/// Validate a remote DH public value is safely inside the 2048-bit group.
pub fn validate_dh_public_value(value: &BigUint, prime: &BigUint) -> Result<(), Error> {
    if prime <= &BigUint::one() {
        return Err(Error::InvalidDh("invalid group modulus"));
    }
    if value <= &BigUint::one() || value >= &(prime - BigUint::one()) {
        return Err(Error::InvalidDh("public value outside group"));
    }
    let margin_shift = usize::try_from(prime.bits().saturating_sub(64))
        .map_err(|_| Error::InvalidDh("prime bit length exceeds platform limits"))?;
    let margin = BigUint::one() << margin_shift;
    if value <= &margin || value >= &(prime - &margin) {
        return Err(Error::InvalidDh("public value too close to group boundary"));
    }
    Ok(())
}

/// Derive and left-pad a shared authorization key after validating the peer value.
pub fn derive_auth_key(
    peer_public: &BigUint,
    secret: &BigUint,
    prime: &BigUint,
) -> Result<Vec<u8>, Error> {
    validate_dh_public_value(peer_public, prime)?;
    if secret.is_zero() {
        return Err(Error::InvalidDh("private exponent is zero"));
    }
    let shared = peer_public.modpow(secret, prime).to_bytes_be();
    let size = usize::try_from(prime.bits().div_ceil(8))
        .map_err(|_| Error::InvalidDh("prime size overflow"))?;
    let mut output = vec![0_u8; size];
    let start = size
        .checked_sub(shared.len())
        .ok_or(Error::InvalidDh("shared key exceeds modulus"))?;
    output[start..].copy_from_slice(&shared);
    Ok(output)
}
