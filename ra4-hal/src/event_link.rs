use crate::pac;

pub(crate) trait IcuEventer {
    const ICU_INDEX: u8;

    #[inline]
    fn iel_disable() {
        let icu = pac::ICU;

        icu.ielsr(Self::ICU_INDEX as _).modify(|w| {
            w.set_iels(ra4m1_ctpac::icu::vals::Iels::_0X000);
        });
    }

    #[inline]
    fn iel_unpend() {
        let icu = pac::ICU;

        icu.ielsr(Self::ICU_INDEX as _).modify(|w| {
            w.set_ir(false);
        });
    }

    #[inline]
    fn iel_enable(mask: InterruptEvent) {
        let icu = pac::ICU;

        icu.ielsr(Self::ICU_INDEX as _).write(|w| {
            w.set_iels(ra4m1_ctpac::icu::vals::Iels::from_bits(mask as u8));
        });
    }
}

#[allow(unused)]
#[repr(u8)]
pub enum InterruptEvent {
    Iic0Rxi = 0x35,
    Iic0Txi = 0x36,
    Iic0Tei = 0x37,
    Iic0Eei = 0x38,
    Iic0Wui = 0x39,

    Iic1Rxi = 0x3A,
    Iic1Txi = 0x3B,
    Iic1Tei = 0x3C,
    Iic1Eei = 0x3D,

    Kint = 0x45,

    CacFerri = 0x47,
    CacMendi = 0x48,
    CacOvfi = 0x49,

    Can0Ers = 0x4A,
    Can0Rxf = 0x4B,
    Can0Txf = 0x4C,
    Can0Rxm = 0x4D,
    Can0Txm = 0x4E,

    Gpt0CcmpA = 0x57,
    Gpt0CcmpB = 0x58,
    Gpt0CmpC = 0x59,
    Gpt0CmpD = 0x5A,
    Gpt0CmpE = 0x5B,
    Gpt0CmpF = 0x5C,
    Gpt0Ovf = 0x5D,
    Gpt0Udf = 0x5E,

    GptUvwEdge = 0x97,

    Sci0Rxi = 0x98,
    Sci0Txi = 0x99,
    Sci0Tei = 0x9A,
    Sci0Eri = 0x9B,
    Sci0Am = 0x9C,
    Sci0RxiOrEri = 0x9D,

    Sci1Rxi = 0x9E,
    Sci1Txi = 0x9F,
    Sci1Tei = 0xA0,
    Sci1Eri = 0xA1,
    Sci1Am = 0xA2,

    Sci2Rxi = 0xA3,
    Sci2Txi = 0xA4,
    Sci2Tei = 0xA5,
    Sci2Eri = 0xA6,
    Sci2Am = 0xA7,

    Sci9Rxi = 0xA8,
    Sci9Txi = 0xA9,
    Sci9Tei = 0xAA,
    Sci9Eri = 0xAB,
    Sci9Am = 0xAC,

    Spi0SpRi = 0xAD,
    Spi0SpTi = 0xAE,
    Spi0SpIi = 0xAF,
    Spi0SpEi = 0xB0,
    Spi0SpTend = 0xB1,

    Spi1SpRi = 0xB2,
    Spi1SpTi = 0xB3,
    Spi1SpIi = 0xB4,
    Spi1SpEi = 0xB5,
    Spi1SpTend = 0xB6,
}

macro_rules! add_iel_index {
    ($index:literal) => {
        paste::paste! {
            impl IcuEventer for crate::interrupt::typelevel::[< IEL $index >] {
                const ICU_INDEX: u8 = $index;
            }
        }
    };
}

add_iel_index!(0);
add_iel_index!(1);
add_iel_index!(2);
add_iel_index!(3);
add_iel_index!(4);
add_iel_index!(5);
add_iel_index!(6);
add_iel_index!(7);
add_iel_index!(8);
add_iel_index!(9);
add_iel_index!(10);
add_iel_index!(11);
add_iel_index!(12);
add_iel_index!(13);
add_iel_index!(14);
add_iel_index!(15);
add_iel_index!(16);
add_iel_index!(17);
add_iel_index!(18);
add_iel_index!(19);
add_iel_index!(20);
add_iel_index!(21);
add_iel_index!(22);
add_iel_index!(23);
add_iel_index!(24);
add_iel_index!(25);
add_iel_index!(26);
add_iel_index!(27);
add_iel_index!(28);
add_iel_index!(29);
add_iel_index!(30);
add_iel_index!(31);
