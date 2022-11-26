# AFE4404 Rust driver

[![build](https://github.com/pulse-loop/AFE4404/actions/workflows/build.yml/badge.svg)](https://github.com/pulse-loop/AFE4404/actions/workflows/build.yml)

This is a AFE4404 driver for Rust Embedded HAL.
It allows a high level interaction with the AFE.

## Usage

Initialise the AFE:

```rust
let mut frontend = AFE4404::with_three_leds(i2c, 0x58u8, Frequency::new::<megahertz>(4.0));
```

Set the LEDs current:

```rust
frontend
    .set_leds_current(&LedCurrentConfiguration::<ThreeLedsMode>::new(
        ElectricCurrent::new::<milliampere>(60.0),
        ElectricCurrent::new::<milliampere>(10.0),
        ElectricCurrent::new::<milliampere>(10.0),
    ))
    .expect("Cannot set leds current");
```

Set the TIA resistors:

```rust
frontend
    .set_tia_resistors(&ResistorConfiguration {
        resistor1: ElectricalResistance::new::<kiloohm>(250.0),
        resistor2: ElectricalResistance::new::<kiloohm>(100.0),
    })
    .expect("Cannot set tia resistors");
```

Set the clock source:

```rust
frontend
    .set_clock_source(&ClockConfiguration::Internal)
    .expect("Cannot set clock source");
```

Read the sampled values:

```rust
let sample = frontend.read();
```
