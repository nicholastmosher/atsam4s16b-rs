#![doc = "Peripheral access API for ATSAM4S16B microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn PMC();
    fn EFC0();
    fn UART0();
    fn UART1();
    fn PIOA();
    fn PIOB();
    fn PIOC();
    fn USART0();
    fn USART1();
    fn HSMCI();
    fn TWI0();
    fn TWI1();
    fn SPI();
    fn SSC();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn ADC();
    fn DACC();
    fn PWM();
    fn CRCCU();
    fn ACC();
    fn UDP();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 35] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMC },
    Vector { _handler: EFC0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _handler: PIOC },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HSMCI },
    Vector { _handler: TWI0 },
    Vector { _handler: TWI1 },
    Vector { _handler: SPI },
    Vector { _handler: SSC },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: ADC },
    Vector { _handler: DACC },
    Vector { _handler: PWM },
    Vector { _handler: CRCCU },
    Vector { _handler: ACC },
    Vector { _handler: UDP },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "5 - PMC"]
    PMC,
    #[doc = "6 - EFC0"]
    EFC0,
    #[doc = "8 - UART0"]
    UART0,
    #[doc = "9 - UART1"]
    UART1,
    #[doc = "11 - PIOA"]
    PIOA,
    #[doc = "12 - PIOB"]
    PIOB,
    #[doc = "13 - PIOC"]
    PIOC,
    #[doc = "14 - USART0"]
    USART0,
    #[doc = "15 - USART1"]
    USART1,
    #[doc = "18 - HSMCI"]
    HSMCI,
    #[doc = "19 - TWI0"]
    TWI0,
    #[doc = "20 - TWI1"]
    TWI1,
    #[doc = "21 - SPI"]
    SPI,
    #[doc = "22 - SSC"]
    SSC,
    #[doc = "23 - TC0"]
    TC0,
    #[doc = "24 - TC1"]
    TC1,
    #[doc = "25 - TC2"]
    TC2,
    #[doc = "26 - TC3"]
    TC3,
    #[doc = "27 - TC4"]
    TC4,
    #[doc = "28 - TC5"]
    TC5,
    #[doc = "29 - ADC"]
    ADC,
    #[doc = "30 - DACC"]
    DACC,
    #[doc = "31 - PWM"]
    PWM,
    #[doc = "32 - CRCCU"]
    CRCCU,
    #[doc = "33 - ACC"]
    ACC,
    #[doc = "34 - UDP"]
    UDP,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PMC => 5,
            Interrupt::EFC0 => 6,
            Interrupt::UART0 => 8,
            Interrupt::UART1 => 9,
            Interrupt::PIOA => 11,
            Interrupt::PIOB => 12,
            Interrupt::PIOC => 13,
            Interrupt::USART0 => 14,
            Interrupt::USART1 => 15,
            Interrupt::HSMCI => 18,
            Interrupt::TWI0 => 19,
            Interrupt::TWI1 => 20,
            Interrupt::SPI => 21,
            Interrupt::SSC => 22,
            Interrupt::TC0 => 23,
            Interrupt::TC1 => 24,
            Interrupt::TC2 => 25,
            Interrupt::TC3 => 26,
            Interrupt::TC4 => 27,
            Interrupt::TC5 => 28,
            Interrupt::ADC => 29,
            Interrupt::DACC => 30,
            Interrupt::PWM => 31,
            Interrupt::CRCCU => 32,
            Interrupt::ACC => 33,
            Interrupt::UDP => 34,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hsmci::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    fn deref(&self) -> &hsmci::RegisterBlock {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssc::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    fn deref(&self) -> &ssc::RegisterBlock {
        unsafe { &*SSC::ptr() }
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &spi::RegisterBlock {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc1::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    fn deref(&self) -> &tc1::RegisterBlock {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire Interface 0"]
pub mod twi0;
#[doc = "Two-wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi1::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    fn deref(&self) -> &twi1::RegisterBlock {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "Two-wire Interface 1"]
pub mod twi1;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    fn deref(&self) -> &pwm::RegisterBlock {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "USB Device Port"]
pub struct UDP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDP {}
impl UDP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const udp::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for UDP {
    type Target = udp::RegisterBlock;
    fn deref(&self) -> &udp::RegisterBlock {
        unsafe { &*UDP::ptr() }
    }
}
#[doc = "USB Device Port"]
pub mod udp;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dacc::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    fn deref(&self) -> &dacc::RegisterBlock {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Analog Comparator Controller"]
pub struct ACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACC {}
impl ACC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acc::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for ACC {
    type Target = acc::RegisterBlock;
    fn deref(&self) -> &acc::RegisterBlock {
        unsafe { &*ACC::ptr() }
    }
}
#[doc = "Analog Comparator Controller"]
pub mod acc;
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub struct CRCCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCCU {}
impl CRCCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crccu::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for CRCCU {
    type Target = crccu::RegisterBlock;
    fn deref(&self) -> &crccu::RegisterBlock {
        unsafe { &*CRCCU::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub mod crccu;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const matrix::RegisterBlock {
        1074659840 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    fn deref(&self) -> &matrix::RegisterBlock {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074660352 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074660864 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub mod uart0;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const chipid::RegisterBlock {
        1074661184 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    fn deref(&self) -> &chipid::RegisterBlock {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1074661376 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub mod uart1;
#[doc = "Embedded Flash Controller 0"]
pub struct EFC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC0 {}
impl EFC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const efc0::RegisterBlock {
        1074661888 as *const _
    }
}
impl Deref for EFC0 {
    type Target = efc0::RegisterBlock;
    fn deref(&self) -> &efc0::RegisterBlock {
        unsafe { &*EFC0::ptr() }
    }
}
#[doc = "Embedded Flash Controller 0"]
pub mod efc0;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pioa::RegisterBlock {
        1074662912 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &pioa::RegisterBlock {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const piob::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    fn deref(&self) -> &piob::RegisterBlock {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pioc::RegisterBlock {
        1074663936 as *const _
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    fn deref(&self) -> &pioc::RegisterBlock {
        unsafe { &*PIOC::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rstc::RegisterBlock {
        1074664448 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &rstc::RegisterBlock {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const supc::RegisterBlock {
        1074664464 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &supc::RegisterBlock {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtt::RegisterBlock {
        1074664496 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    fn deref(&self) -> &rtt::RegisterBlock {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1074664528 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074664544 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Register"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpbr::RegisterBlock {
        1074664592 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    fn deref(&self) -> &gpbr::RegisterBlock {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "UDP"]
    pub UDP: UDP,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "ACC"]
    pub ACC: ACC,
    #[doc = "CRCCU"]
    pub CRCCU: CRCCU,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "EFC0"]
    pub EFC0: EFC0,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            UDP: UDP {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            ACC: ACC {
                _marker: PhantomData,
            },
            CRCCU: CRCCU {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            EFC0: EFC0 {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
        }
    }
}
