unsafe extern "C" {
    fn IEL0();
    fn IEL1();
    fn IEL2();
    fn IEL3();
    fn IEL4();
    fn IEL5();
    fn IEL6();
    fn IEL7();
    fn IEL8();
    fn IEL9();
    fn IEL10();
    fn IEL11();
    fn IEL12();
    fn IEL13();
    fn IEL14();
    fn IEL15();
    fn IEL16();
    fn IEL17();
    fn IEL18();
    fn IEL19();
    fn IEL20();
    fn IEL21();
    fn IEL22();
    fn IEL23();
    fn IEL24();
    fn IEL25();
    fn IEL26();
    fn IEL27();
    fn IEL28();
    fn IEL29();
    fn IEL30();
    fn IEL31();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: IEL0 },
    Vector { _handler: IEL1 },
    Vector { _handler: IEL2 },
    Vector { _handler: IEL3 },
    Vector { _handler: IEL4 },
    Vector { _handler: IEL5 },
    Vector { _handler: IEL6 },
    Vector { _handler: IEL7 },
    Vector { _handler: IEL8 },
    Vector { _handler: IEL9 },
    Vector { _handler: IEL10 },
    Vector { _handler: IEL11 },
    Vector { _handler: IEL12 },
    Vector { _handler: IEL13 },
    Vector { _handler: IEL14 },
    Vector { _handler: IEL15 },
    Vector { _handler: IEL16 },
    Vector { _handler: IEL17 },
    Vector { _handler: IEL18 },
    Vector { _handler: IEL19 },
    Vector { _handler: IEL20 },
    Vector { _handler: IEL21 },
    Vector { _handler: IEL22 },
    Vector { _handler: IEL23 },
    Vector { _handler: IEL24 },
    Vector { _handler: IEL25 },
    Vector { _handler: IEL26 },
    Vector { _handler: IEL27 },
    Vector { _handler: IEL28 },
    Vector { _handler: IEL29 },
    Vector { _handler: IEL30 },
    Vector { _handler: IEL31 },
];
