use embedded_hal::delay::blocking::DelayUs;
use esp_idf_hal::{
    i2c::{config::MasterConfig, Master, MasterPins},
    peripherals::Peripherals,
    prelude::*,
};

use AFE4404::{
    high_level::{
        clock::ClockConfiguration,
        led_current::LedConfiguration,
        tia::{CapacitorConfiguration, ResistorConfiguration},
        timing_window::*,
        timing_window::{ActiveTimingConfiguration, LedTiming, MeasurementWindowConfiguration},
        value_reading::ReadingMode,
    },
    uom::si::{
        capacitance::picofarad,
        electric_current::milliampere,
        electrical_resistance::kiloohm,
        f32::{Capacitance, ElectricCurrent, ElectricalResistance, Frequency},
        frequency::megahertz,
        time::microsecond,
    },
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

    let mut frontend = AFE4404::AFE4404::new(i2c, 0x58u8, Frequency::new::<megahertz>(4.0));

    frontend.sw_reset().expect("Cannot reset the afe");

    println!(
        "Setting: {:?}\nGetting: {:?}",
        frontend
            .set_leds_current(&LedConfiguration {
                led1_current: ElectricCurrent::new::<milliampere>(30.0), // Green led.
                led2_current: ElectricCurrent::new::<milliampere>(30.0), // Red led.
                led3_current: ElectricCurrent::new::<milliampere>(30.0), // Infrared led.
            })
            .expect("Cannot set leds current"),
        frontend
            .get_leds_current()
            .expect("Cannot get leds current")
    );

    println!(
        "Setting: {:?}\nGetting: {:?}",
        frontend
            .set_tia_resistors(&ResistorConfiguration {
                resistor1: ElectricalResistance::new::<kiloohm>(50.0),
                resistor2: ElectricalResistance::new::<kiloohm>(100.0),
            })
            .expect("Cannot set tia resistors"),
        frontend
            .get_tia_resistors()
            .expect("Cannot get tia resistors")
    );

    println!(
        "Setting: {:?}\nGetting: {:?}",
        frontend
            .set_tia_capacitors(&CapacitorConfiguration {
                capacitor1: Capacitance::new::<picofarad>(5.0),
                capacitor2: Capacitance::new::<picofarad>(12.0),
            })
            .expect("Cannot set tia capacitors"),
        frontend
            .get_tia_capacitors()
            .expect("Cannot get tia capacitors")
    );

    println!(
        "Setting: {:?}\nGetting: {:?}",
        frontend
            .set_timing_window(&MeasurementWindowConfiguration {
                period: AFE4404::uom::si::f32::Time::new::<microsecond>(10_000.0),
                active_timing_configuration: ActiveTimingConfiguration::ThreeLeds {
                    led2: LedTiming {
                        led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(0.0),
                        led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(99.75),
                        sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(25.0),
                        sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(99.75),
                        reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(100.25),
                        reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(101.75),
                        conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(102.25),
                        conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(367.0),
                    },
                    led3: LedTiming {
                        led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(100.25),
                        led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(200.0),
                        sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(125.25),
                        sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(200.0),
                        reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(367.5),
                        reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(369.0),
                        conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(369.5),
                        conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(634.25),
                    },
                    led1: LedTiming {
                        led_st: AFE4404::uom::si::f32::Time::new::<microsecond>(200.5),
                        led_end: AFE4404::uom::si::f32::Time::new::<microsecond>(300.25),
                        sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(225.5),
                        sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(300.25),
                        reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(634.75),
                        reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(636.25),
                        conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(636.75),
                        conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(901.5),
                    },
                    ambient: AmbientTiming {
                        sample_st: AFE4404::uom::si::f32::Time::new::<microsecond>(325.75),
                        sample_end: AFE4404::uom::si::f32::Time::new::<microsecond>(400.5),
                        reset_st: AFE4404::uom::si::f32::Time::new::<microsecond>(902.0),
                        reset_end: AFE4404::uom::si::f32::Time::new::<microsecond>(903.5),
                        conv_st: AFE4404::uom::si::f32::Time::new::<microsecond>(904.0),
                        conv_end: AFE4404::uom::si::f32::Time::new::<microsecond>(1168.75),
                    },
                },
                inactive_timing: PowerDownTiming {
                    power_down_st: AFE4404::uom::si::f32::Time::new::<microsecond>(1368.75),
                    power_down_end: AFE4404::uom::si::f32::Time::new::<microsecond>(9799.75),
                },
            })
            .expect("Cannot set timing window"),
        frontend
            .get_timing_window()
            .expect("Cannot get timing window")
    );

    println!(
        "Setting: {:?}\nGetting: {:?}",
        frontend
            .set_clock_source(&ClockConfiguration::Internal)
            .expect("Cannot set clock source"),
        frontend
            .get_clock_source()
            .expect("Cannot get clock source")
    );

    loop {
        let readings = frontend.read(&ReadingMode::ThreeLeds).expect("Cannot read.");
        println!("Readings: {:?}", readings);
        let mut delay = esp_idf_hal::delay::Ets;
        delay.delay_ms(100).unwrap();

        // TODO: Check ready pin.
    }
}
