use lambdaworks_math::{
    cyclic_group::IsGroup, 
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, 
        traits::IsEllipticCurve,
    }, 
    field::element::FieldElement,
    field::fields::u64_prime_field::U64PrimeField,
};

fn find_bls12_381_pk() {
    let generator = BLS12381Curve::generator();
    let secret_key: u64 = 0x6C616D6264617370;
    let public_key = generator.operate_with_self(secret_key);

    println!("Public key: {:?}", public_key);
}

fn find_multiplicative_inverse() {
    const MODULUS: u64 = (2u128.pow(64) - 2u128.pow(32) + 1) as u64;
    type F = U64PrimeField<MODULUS>;
    type FE = FieldElement<F>;

    let two = FE::from(2);
    let inverse = two.inv().unwrap();

    println!("Multiplicative inverse of 2: {:?}", inverse);
}

fn main() {
    find_bls12_381_pk();
    // find_multiplicative_inverse();
}
