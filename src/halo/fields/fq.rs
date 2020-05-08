/// Native, base field of the curve.

use pairing::ff::{Field, PrimeField, PrimeFieldRepr, SqrtField};

#[derive(PrimeField)]
#[PrimeFieldModulus = "28948022309329048855892746252171976963322203655954433126947083963168578338817"]
#[PrimeFieldGenerator = "5"]
pub struct Fq(FqRepr);

#[cfg(test)]
mod tests {
    use rand::{Rand, SeedableRng, XorShiftRng};
    use super::*;

    #[test]
    fn fq_is_valid() {
        print!("modulus = {}\n", MODULUS);
        print!("R = {}\n", R);
        let mut a = Fq(MODULUS);
        assert!(!a.is_valid());
        a.0.sub_noborrow(&FqRepr::from(1));
        assert!(a.is_valid());
        assert!(Fq(FqRepr::from(0)).is_valid());
        assert!(Fq(FqRepr([
            0xdf4671abd14dab3e,
            0xe2dc0c9f534fbd33,
            0x31ca6c880cc444a6,
            0x257a67e70ef33359
        ]))
        .is_valid());
        assert!(!Fq(FqRepr([
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
            0xffffffffffffffff,
        ]))
        .is_valid());

        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        for _ in 0..1000 {
            let a = Fq::rand(&mut rng);
            assert!(a.is_valid());
        }
    }

    #[test]
    fn fq_roots_of_unity() {
        assert_eq!(Fq::S, 34);
    }
}
