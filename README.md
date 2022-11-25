# AFE4404 Rust driver

This is a Rust Embedded HAL driver for the AFE4404, an integrated analog front end for wearable, optical heart-monitoring and biosensing, from Texas Instruments.
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
