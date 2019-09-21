# drv2605l

This module implements a driver for the drv2605l, a Haptic Driver device addressable via I2C from Texas Instruments.

Its rather fully featured implementation for both LRA and ERM motors including:

- Internal rom library sequencing and playback
- Pwm playback at the in/trig pin
- Real time playback over i2c commands
- Analog voltage input at the in/trig pin

A few things are not supported at the time of this writing:

- bidirectional input
- choosing loop operation. The prefered and far more common closed loop operation is hardcoded except for the case of ERM rom libraries which require open loop.
- audio to vibe mode
- external trigger mode

This work originated in the [drv2065 driver](https://github.com/wez/drv2605) and may be able to unify someday.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
