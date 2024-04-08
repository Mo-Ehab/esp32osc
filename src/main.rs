#![no_std]
#![no_main]


use esp_backtrace as _;
use esp_hal::{adc::{AdcConfig, Attenuation, ADC}, clock::ClockControl, peripherals::{Peripherals, ADC1}, prelude::*, uart::{config::{Config, DataBits, Parity, StopBits}, TxRxPins}, Uart, IO};
use ufmt::uwrite;

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let analog_pin = io.pins.gpio36.into_analog();

    let mut adc1_config = AdcConfig::new();
    let mut adc1_pin = adc1_config.enable_pin(analog_pin, Attenuation::Attenuation0dB);

    let mut adc1 = ADC::<ADC1>::new(peripherals.ADC1, adc1_config);


    let config = Config {
        baudrate: 921600 ,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityEven,
        stop_bits: StopBits::STOP1,
    };
    
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio1.into_push_pull_output(),
        io.pins.gpio2.into_floating_input(),
    );
    
    let mut serial1 = Uart::new_with_config(peripherals.UART1, config, Some(pins), &clocks);

    loop {
        uwrite!(serial1, "{}, ", nb::block!(adc1.read(&mut adc1_pin)).unwrap()).unwrap();
    }
}
