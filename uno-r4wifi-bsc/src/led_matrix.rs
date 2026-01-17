use embassy_hal_internal::Peri;
use embassy_time::Timer;
use ra4_hal::{
    gpio::{AnyPin, Flex, Pin},
    peripherals::{P003, P004, P011, P012, P013, P015, P204, P205, P206, P212, P213},
};

pub struct LedMatrix {
    pins: [Peri<'static, AnyPin>; 11],
}

const PIN_MAP: [[u8; 2]; 96] = [
    [7, 3], // 0
    [3, 7],
    [7, 4],
    [4, 7],
    [3, 4],
    [4, 3],
    [7, 8],
    [8, 7],
    [3, 8],
    [8, 3],
    [4, 8], // 10
    [8, 4],
    [7, 0],
    [0, 7],
    [3, 0],
    [0, 3],
    [4, 0],
    [0, 4],
    [8, 0],
    [0, 8],
    [7, 6], // 20
    [6, 7],
    [3, 6],
    [6, 3],
    [4, 6],
    [6, 4],
    [8, 6],
    [6, 8],
    [0, 6],
    [6, 0],
    [7, 5], // 30
    [5, 7],
    [3, 5],
    [5, 3],
    [4, 5],
    [5, 4],
    [8, 5],
    [5, 8],
    [0, 5],
    [5, 0],
    [6, 5], // 40
    [5, 6],
    [7, 1],
    [1, 7],
    [3, 1],
    [1, 3],
    [4, 1],
    [1, 4],
    [8, 1],
    [1, 8],
    [0, 1], // 50
    [1, 0],
    [6, 1],
    [1, 6],
    [5, 1],
    [1, 5],
    [7, 2],
    [2, 7],
    [3, 2],
    [2, 3], // 60
    [4, 2],
    [2, 4],
    [8, 2],
    [2, 8],
    [0, 2],
    [2, 0],
    [6, 2],
    [2, 6],
    [5, 2],
    [2, 5], // 70
    [1, 2],
    [2, 1],
    [7, 10],
    [10, 7],
    [3, 10],
    [10, 3],
    [4, 10],
    [10, 4],
    [8, 10],
    [10, 8], // 80
    [0, 10],
    [10, 0],
    [6, 10],
    [10, 6],
    [5, 10],
    [10, 5],
    [1, 10],
    [10, 1],
    [2, 10],
    [10, 2], // 90
    [7, 9],
    [9, 7],
    [3, 9],
    [9, 3],
    [4, 9],
    [9, 4], // 96
];

impl<'d> LedMatrix {
    pub fn new(
        d0: Peri<'d, P003>,
        d1: Peri<'d, P004>,
        d2: Peri<'d, P011>,
        d3: Peri<'d, P012>,
        d4: Peri<'d, P013>,
        d5: Peri<'d, P015>,
        d6: Peri<'d, P204>,
        d7: Peri<'d, P205>,
        d8: Peri<'d, P206>,
        d9: Peri<'d, P212>,
        d10: Peri<'d, P213>,
    ) -> Self {
        let pins = unsafe {
            [
                AnyPin::steal((d0.port() as u16) * 100 + d0.pin() as u16),
                AnyPin::steal((d1.port() as u16) * 100 + d1.pin() as u16),
                AnyPin::steal((d2.port() as u16) * 100 + d2.pin() as u16),
                AnyPin::steal((d3.port() as u16) * 100 + d3.pin() as u16),
                AnyPin::steal((d4.port() as u16) * 100 + d4.pin() as u16),
                AnyPin::steal((d5.port() as u16) * 100 + d5.pin() as u16),
                AnyPin::steal((d6.port() as u16) * 100 + d6.pin() as u16),
                AnyPin::steal((d7.port() as u16) * 100 + d7.pin() as u16),
                AnyPin::steal((d8.port() as u16) * 100 + d8.pin() as u16),
                AnyPin::steal((d9.port() as u16) * 100 + d9.pin() as u16),
                AnyPin::steal((d10.port() as u16) * 100 + d10.pin() as u16),
            ]
        };

        Self { pins }
    }

    pub async fn set_pixel(&self, idx: u8, state: bool) {
        let port0 = ra4_hal::pac::PORT0;
        let port2 = ra4_hal::pac::PORT2;

        port0.pcntr1().write(|w| {
            for pin in [3, 4, 11, 12, 13, 15] {
                w.set_pdr(pin, false);
            }
        });
        port2.pcntr1().write(|w| {
            for pin in [4, 5, 6, 12, 13] {
                w.set_pdr(pin, false);
            }
        });

        if state {
            let [high, low] = PIN_MAP[idx as usize];
            let high = &self.pins[high as usize];
            let low = &self.pins[low as usize];

            let mut high = Flex::new(unsafe { high.clone_unchecked() });
            high.set_as_output();
            high.set_high();

            let mut low = Flex::new(unsafe { low.clone_unchecked() });
            low.set_as_output();
            low.set_low();
        }

        Timer::after_micros(350).await;
    }
}

#[macro_export]
/// FOo
macro_rules! led_matrix_init {
    ($p:ident) => {
        $crate::led_matrix::LedMatrix::new(
            $p.P003, $p.P004, $p.P011, $p.P012, $p.P013, $p.P015, $p.P204, $p.P205, $p.P206,
            $p.P212, $p.P213,
        )
    };
}
