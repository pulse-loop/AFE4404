use embedded_hal::{delay::blocking::DelayUs, digital::blocking::OutputPin};
use esp_idf_hal::{
    i2c::{config::MasterConfig, Master, MasterPins},
    peripherals::Peripherals,
    prelude::*,
};

use afe4404::{
    afe4404::ThreeLedsMode,
    high_level::{
        clock::ClockConfiguration,
        led_current::{LedCurrentConfiguration, OffsetCurrentConfiguration},
        tia::{CapacitorConfiguration, ResistorConfiguration},
        timing_window::{ActiveTiming, LedTiming, MeasurementWindowConfiguration, AmbientTiming, PowerDownTiming},
    },
    uom::si::{
        capacitance::picofarad,
        electric_current::{microampere, milliampere},
        electrical_resistance::kiloohm,
        f32::{Capacitance, ElectricCurrent, ElectricalResistance, Frequency, Time},
        frequency::megahertz,
        time::microsecond,
    },
    AFE4404,
};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let config = MasterConfig::new().baudrate(100.kHz().into());

    let i2c = Master::new(
        peripherals.i2c0,
        MasterPins {
            sda: peripherals.pins.gpio3,
            scl: peripherals.pins.gpio2,
        },
        config,
    )
    .expect("Failed to initialize I2C bus.");

    let mut frontend = AFE4404::with_three_leds(i2c, 0x58u8, Frequency::new::<megahertz>(4.0));

    frontend.sw_reset().expect("Cannot reset the afe");

    frontend
        .set_leds_current(&LedCurrentConfiguration::<ThreeLedsMode>::new(
            ElectricCurrent::new::<milliampere>(30.0), // Green led.
            ElectricCurrent::new::<milliampere>(30.0), // Red led.
            ElectricCurrent::new::<milliampere>(30.0), // Infrared led.
        ))
        .expect("Cannot set leds current");

    frontend
        .set_offset_current(&OffsetCurrentConfiguration::<ThreeLedsMode>::new(
            ElectricCurrent::new::<microampere>(-1.5),
            ElectricCurrent::new::<microampere>(-3.0),
            ElectricCurrent::new::<microampere>(-3.0),
            ElectricCurrent::new::<microampere>(0.0),
        ))
        .expect("Cannot set offset current");

    frontend
        .set_tia_resistors(&ResistorConfiguration {
            resistor1: ElectricalResistance::new::<kiloohm>(50.0),
            resistor2: ElectricalResistance::new::<kiloohm>(50.0),
        })
        .expect("Cannot set tia resistors");

    frontend
        .set_tia_capacitors(&CapacitorConfiguration {
            capacitor1: Capacitance::new::<picofarad>(5.0),
            capacitor2: Capacitance::new::<picofarad>(5.0),
        })
        .expect("Cannot set tia capacitors");

    frontend
        .set_timing_window(&MeasurementWindowConfiguration::<ThreeLedsMode>::new(
            Time::new::<microsecond>(10_000.0),
            ActiveTiming::<ThreeLedsMode>::new(
                LedTiming {
                    led_st: Time::new::<microsecond>(200.5),
                    led_end: Time::new::<microsecond>(300.25),
                    sample_st: Time::new::<microsecond>(225.5),
                    sample_end: Time::new::<microsecond>(300.25),
                    reset_st: Time::new::<microsecond>(634.75),
                    reset_end: Time::new::<microsecond>(636.25),
                    conv_st: Time::new::<microsecond>(636.75),
                    conv_end: Time::new::<microsecond>(901.5),
                },
                LedTiming {
                    led_st: Time::new::<microsecond>(0.0),
                    led_end: Time::new::<microsecond>(99.75),
                    sample_st: Time::new::<microsecond>(25.0),
                    sample_end: Time::new::<microsecond>(99.75),
                    reset_st: Time::new::<microsecond>(100.25),
                    reset_end: Time::new::<microsecond>(101.75),
                    conv_st: Time::new::<microsecond>(102.25),
                    conv_end: Time::new::<microsecond>(367.0),
                },
                LedTiming {
                    led_st: Time::new::<microsecond>(100.25),
                    led_end: Time::new::<microsecond>(200.0),
                    sample_st: Time::new::<microsecond>(125.25),
                    sample_end: Time::new::<microsecond>(200.0),
                    reset_st: Time::new::<microsecond>(367.5),
                    reset_end: Time::new::<microsecond>(369.0),
                    conv_st: Time::new::<microsecond>(369.5),
                    conv_end: Time::new::<microsecond>(634.25),
                },
                AmbientTiming {
                    sample_st: Time::new::<microsecond>(325.75),
                    sample_end: Time::new::<microsecond>(400.5),
                    reset_st: Time::new::<microsecond>(902.0),
                    reset_end: Time::new::<microsecond>(903.5),
                    conv_st: Time::new::<microsecond>(904.0),
                    conv_end: Time::new::<microsecond>(1168.75),
                },
            ),
            PowerDownTiming {
                power_down_st: Time::new::<microsecond>(1368.75),
                power_down_end: Time::new::<microsecond>(9799.75),
            },
        ))
        .expect("Cannot set timing window");

    frontend
        .set_clock_source(&ClockConfiguration::Internal)
        .expect("Cannot set clock source");

    loop {
        let readings = frontend.read();
        match readings {
            Ok(readings) => {
                println!("Green: {}", readings.led1().value);
                println!("Red: {}", readings.led2().value);
                println!("Infrared: {}", readings.led3().value);
                println!("Ambient: {}", readings.ambient().value);
            }
            Err(e) => println!("Error: {:?}", e),
        }

        let mut delay = esp_idf_hal::delay::Ets;
        delay.delay_ms(100).unwrap();

        // TODO: Check ready pin.
    }
}
