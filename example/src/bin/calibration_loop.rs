use std::sync::atomic::AtomicBool;

use embedded_hal::delay::blocking::DelayUs;
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
        timing_window::{
            ActiveTiming, AmbientTiming, LedTiming, MeasurementWindowConfiguration, PowerDownTiming,
        },
        value_reading::Readings,
    },
    uom::si::{
        capacitance::picofarad,
        electric_current::milliampere,
        electric_potential::volt,
        electrical_resistance::kiloohm,
        f32::{
            Capacitance, ElectricCurrent, ElectricPotential, ElectricalResistance, Frequency, Time,
        },
        frequency::megahertz,
        time::microsecond,
    },
    AFE4404,
};

static DATA_READY: AtomicBool = AtomicBool::new(false);

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
        .unwrap();

    frontend
        .set_clock_source(&ClockConfiguration::Internal)
        .unwrap();

    struct Parameters {
        resistors: [ElectricalResistance; 8],
        capacitors: [Capacitance; 8],
        current_max_value: ElectricCurrent,
        voltage_max_value: ElectricPotential,
    }
    let parameters: Parameters = Parameters {
        resistors: [
            ElectricalResistance::new::<kiloohm>(10.0),
            ElectricalResistance::new::<kiloohm>(25.0),
            ElectricalResistance::new::<kiloohm>(50.0),
            ElectricalResistance::new::<kiloohm>(100.0),
            ElectricalResistance::new::<kiloohm>(250.0),
            ElectricalResistance::new::<kiloohm>(500.0),
            ElectricalResistance::new::<kiloohm>(1000.0),
            ElectricalResistance::new::<kiloohm>(2000.0),
        ],
        capacitors: [
            Capacitance::new::<picofarad>(2.5),
            Capacitance::new::<picofarad>(5.0),
            Capacitance::new::<picofarad>(7.5),
            Capacitance::new::<picofarad>(10.0),
            Capacitance::new::<picofarad>(17.5),
            Capacitance::new::<picofarad>(20.0),
            Capacitance::new::<picofarad>(22.5),
            Capacitance::new::<picofarad>(25.0),
        ],
        current_max_value: ElectricCurrent::new::<milliampere>(60.0),
        voltage_max_value: ElectricPotential::new::<volt>(1.0),
    };

    // Starting values.
    let mut best_resistors = ResistorConfiguration {
        resistor1: parameters.resistors[0],
        resistor2: parameters.resistors[0],
    };
    // TODO: Estimate best capacitor starting value.
    let mut best_capacitors = CapacitorConfiguration {
        capacitor1: parameters.capacitors[0],
        capacitor2: parameters.capacitors[0],
    };
    let mut best_currents = LedCurrentConfiguration::<ThreeLedsMode>::new(
        ElectricCurrent::new::<milliampere>(0.0),
        ElectricCurrent::new::<milliampere>(0.0),
        ElectricCurrent::new::<milliampere>(0.0),
    );

    // TODO: Manage the errors.
    frontend.set_tia_resistors(&best_resistors).unwrap();
    frontend.set_tia_capacitors(&best_capacitors).unwrap();

    unsafe {
        peripherals
            .pins
            .gpio4
            .into_subscribed(
                || {
                    DATA_READY.store(true, std::sync::atomic::Ordering::Relaxed);
                },
                esp_idf_hal::gpio::InterruptType::PosEdge,
            )
            .unwrap();
    }

    let mut delay = esp_idf_hal::delay::Ets;

    // Gain calibration loop.
    let voltage_threshold = parameters.voltage_max_value * 0.8;
    *best_currents.led1_mut() = parameters.current_max_value;
    *best_currents.led2_mut() = parameters.current_max_value;
    *best_currents.led3_mut() = parameters.current_max_value;
    frontend.set_leds_current(&best_currents).unwrap();

    for resistor in parameters.resistors.iter() {
        delay.delay_ms(200).unwrap();

        // TODO: Manage infinite readings.
        // TODO: Change in function: get_sample_blocking().
        let sample;
        loop {
            if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
                let current_readings = frontend.read();
                if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                    // TODO: Manage reading error.
                    sample = current_readings.unwrap();
                    break;
                }
            }
        }

        // Select greater resistors in case of current saturation.
        if sample.led1() < &voltage_threshold {
            best_resistors.resistor1 = *resistor;
        }
        if sample.led2() < &voltage_threshold || sample.led3() < &voltage_threshold {
            best_resistors.resistor2 = *resistor;
        }
        if sample.led1() >= &voltage_threshold
            && sample.led2() >= &voltage_threshold
            && sample.led3() >= &voltage_threshold
        {
            break;
        }
    }
    println!("Best resistors: reistor1 {}kOhm, resistor2 {}kOhm", best_resistors.resistor1.value / 1000.0, best_resistors.resistor2.value / 1000.0);

    // Current calibration loop.
    let mut lower_current_bound = [ElectricCurrent::new::<milliampere>(0.0); 3];
    let mut upper_current_bound = [parameters.current_max_value; 3];
    let mut mid_current_bound = [parameters.current_max_value / 2.0; 3];
    *best_currents.led1_mut() = mid_current_bound[0];
    *best_currents.led2_mut() = mid_current_bound[1];
    *best_currents.led3_mut() = mid_current_bound[2];
    frontend.set_leds_current(&best_currents).unwrap();

    while upper_current_bound[0] - lower_current_bound[0] > ElectricCurrent::new::<milliampere>(0.8)
        || upper_current_bound[1] - lower_current_bound[1]
            > ElectricCurrent::new::<milliampere>(0.8)
        || upper_current_bound[2] - lower_current_bound[2]
            > ElectricCurrent::new::<milliampere>(0.8)
    {
        delay.delay_ms(200).unwrap();

        // TODO: Manage infinite readings.
        // TODO: Change in function: get_sample_blocking().
        let sample;
        loop {
            if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
                let current_readings = frontend.read();
                if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                    // TODO: Manage reading error.
                    sample = current_readings.unwrap();
                    break;
                }
            }
        }

        if upper_current_bound[0] - lower_current_bound[0]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            // Find closest current value using bisection method.
            // TODO: Change in function: bisection<T>(a: &mut T, b: &mut T, c: &mut T, greater_interval: bool).
            if sample.led1() > &voltage_threshold {
                upper_current_bound[0] = mid_current_bound[0] // Decrease current.
            } else {
                lower_current_bound[0] = mid_current_bound[0] // Increase current.
            }
            mid_current_bound[0] = (upper_current_bound[0] + lower_current_bound[0]) / 2.0;
            // Set new best value.
            *best_currents.led1_mut() = mid_current_bound[0];
        }
        if upper_current_bound[1] - lower_current_bound[1]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            if sample.led2() > &voltage_threshold {
                upper_current_bound[1] = mid_current_bound[1]
            } else {
                lower_current_bound[1] = mid_current_bound[1]
            }
            mid_current_bound[1] = (upper_current_bound[1] + lower_current_bound[1]) / 2.0;
            *best_currents.led2_mut() = mid_current_bound[1];
        }
        if upper_current_bound[2] - lower_current_bound[2]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            if sample.led3() > &voltage_threshold {
                upper_current_bound[2] = mid_current_bound[2]
            } else {
                lower_current_bound[2] = mid_current_bound[2]
            }
            mid_current_bound[2] = (upper_current_bound[2] + lower_current_bound[2]) / 2.0;
            *best_currents.led3_mut() = mid_current_bound[2];
        }

        best_currents = frontend.set_leds_current(&best_currents).unwrap();
    }
    println!("Best currents: led1 {}A, led2 {}A, led3 {}A", best_currents.led1().value, best_currents.led2().value, best_currents.led3().value);

    loop {
        if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
            DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
            let readings = frontend.read();
            if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                match readings {
                    Ok(readings) => {
                        // println!("Green: {}", readings.led1().value);
                        // println!("Red: {}", readings.led2().value);
                        // println!("Infrared: {}", readings.led3().value);
                        // println!("Ambient: {}", readings.ambient().value);
                        println!(
                            "{} {} {} {}",
                            readings.led1().value,
                            readings.led2().value,
                            readings.led3().value,
                            readings.ambient().value
                        );
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
        delay.delay_ms(1000).unwrap();
    }
}
