use esp_idf_hal::{prelude::*, peripherals::Peripherals, i2c::{Master, MasterPins, config::MasterConfig}};
use embedded_hal::delay::{blocking::DelayUs};

use AFE4404::{
    high_level::timing_window::*,
    uom::{
        si::{
            f32::{
                Capacitance,
                ElectricCurrent,
                ElectricalResistance,
            },
            electrical_resistance::kiloohm,
            electric_current::milliampere,
            time::microsecond,
            capacitance::picofarad,
        },
    },
    high_level::{timing_window::{ActiveTimingConfiguration, LedTiming, MeasurementWindowConfiguration}, value_reading::ReadingMode}
};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let config = MasterConfig::new().baudrate(400.kHz().into());

    let i2c = Master::new(
        peripherals.i2c0,
        MasterPins {
            sda: peripherals.pins.gpio3,
            scl: peripherals.pins.gpio2,
        },
        config,
    )
    .expect("Failed to initialize I2C bus.");

    let mut frontend = AFE4404::AFE4404::new(i2c, 0x58u8);

    frontend.reset().expect("Cannot reset the afe");
    frontend.set_clock_source(true).expect("Cannot set clock source.");
    frontend.enable_clock_out().expect("Cannot enable clock out");

    frontend.set_leds_current(
        ElectricCurrent::new::<milliampere>(25.0),
        ElectricCurrent::new::<milliampere>(25.0),
        // TODO: Make function accept 50mA.
        ElectricCurrent::new::<milliampere>(25.0),
    ).expect("Cannot set leds current");

    frontend.set_tia_resistors(
        ElectricalResistance::new::<kiloohm>(100.0),
        ElectricalResistance::new::<kiloohm>(100.0),
    ).expect("Cannot set tia resistors");
    
    frontend.set_tia_capacitors(
        Capacitance::new::<picofarad>(5.0),
        Capacitance::new::<picofarad>(5.0),
    ).expect("Cannot set tia capacitors");
    
    frontend.set_timing_window(
        MeasurementWindowConfiguration {
            period: AFE4404::uom::si::f32::Time::new::<microsecond>(10_000.0),
            active_timing_configuration: ActiveTimingConfiguration::ThreeLeds {
                led2: LedTiming {
                    led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(0.0),
                    led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(100.0),
                    sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(25.0),
                    sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(100.0),
                    reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(100.25),
                    reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(102.0),
                    conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(102.25),
                    conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(367.25),
                },
                led3: LedTiming {
                    led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(100.25),
                    led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(200.25),
                    sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(125.25),
                    sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(200.25),
                    reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(367.5),
                    reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(369.25),
                    conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(369.5),
                    conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(634.5),
                },
                led1: LedTiming {
                    led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(200.5),
                    led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(300.5),
                    sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(225.5),
                    sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(300.5),
                    reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(634.75),
                    reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(636.5),
                    conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(636.75),
                    conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(901.75),
                },
                ambient: AmbientTiming {
                    sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(325.75),
                    sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(400.75),
                    reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(902.0),
                    reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(903.75),
                    conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(904.0),
                    conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(1169.0),
                },
            },
            inactive_timing: PowerDownTiming {
                power_down_st: AFE4404::uom::si::f32::Time::new::<microsecond>(1368.75),
                power_down_end: AFE4404::uom::si::f32::Time::new::<microsecond>(9800.0),
            },
        }
    ).expect("Cannot set timing window");



    frontend.start_sampling().expect("Cannot start sampling.");
    
    loop {
        let readings = frontend.read(ReadingMode::ThreeLeds).expect("Cannot read.");
        println!("Readings: {:?}", readings);

        let mut delay = esp_idf_hal::delay::FreeRtos;
        delay.delay_ms(100).unwrap();

        // TODO: Check ready pin.
    }
}
