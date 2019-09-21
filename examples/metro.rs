// Example for the metro_m0
// cargo build --release --example metro
#![no_std]
#![no_main]

use bsp::hal;
use bsp::pac;
use metro_m0 as bsp;
use panic_rtt as _;

use drv2605l::{Calibration, CalibrationParams, Drv2605l, Effect, Library, Mode, RomParams};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut stdout = jlink_rtt::Output::new();
            writeln!(stdout, $($arg)*).ok();
        }
    };
}

#[bsp::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let i2c = bsp::i2c_master(
        &mut clocks,
        400.khz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    // An ERM motor with default configuration and Auto calibration
    //
    // Note secure motor to a mass or calibration will fail! Your motor may or
    // may not calibrate with defaults, ideally these should be computed from
    // the datasheet
    let calib = CalibrationParams::default();
    let mut haptic = Drv2605l::new(i2c, Calibration::Auto(calib), false).unwrap();
    dbgprint!("sucessfully calibrated device");

    // An LRA motor with configuration and Auto calibration
    // let mut calib = CalibrationParams::default();
    // calib.rated_voltage = 0x3E;
    // calib.overdrive_voltage_clamp = 0x8C;
    // calib.drive_time = 0x13;
    // let mut haptic = Drv2605l::new(i2c, Calibration::Auto(calib), true).unwrap();

    // print the sucessful calibration values so you can hardcode them later
    // let params = haptic.calibration().unwrap();
    // dbgprint!(
    //     "compenstation:{} back_emf:{} back_emf_gain:{}",
    //     params.compenstation,
    //     params.back_emf,
    //     params.back_emf_gain
    // );

    // and use hardcoded ones them instead of auto calibration like this
    // let mut haptic = Drv2605l::new(
    //     i2c,
    //     //from the
    //     Calibration::Load(drv2605::LoadParams {
    //         compenstation: 0x3E,
    //         back_emf: 0x89,
    //         back_emf_gain: 0x25,
    //     }),
    //     false,
    // )
    // .unwrap();
    // dbgprint!("device successfully init");

    // Now lets play some built in effects. Each library has all the same
    // vibrations, but is tuned to work for certain motor characteristics so its
    // important to choose Library for for your motor characteristics
    haptic
        .set_mode(Mode::Rom(Library::B, RomParams::default()))
        .unwrap();

    // a sequence of
    let roms = [
        Effect::StrongClick100,
        Effect::Delays(10), // 10 * 10ms delay or 100ms
        Effect::ShortDoubleClickStrongOne100,
        Effect::Delays(100), //100 * 10ms or 1000ms
        Effect::StrongClick100,
        Effect::Stop, //stop early
        Effect::Stop, //stop early
        Effect::Stop, //stop early
    ];
    haptic.set_rom(&roms).unwrap();

    // device starts in standby, so lets wake it up for motor operation
    haptic.set_standby(false).unwrap();
    loop {
        // fire
        haptic.set_go().unwrap();
        // you dont to, but we can poll the device until the sequence finishes
        while haptic.go().unwrap() {}

        // wait another second after that before we start again
        delay.delay_ms(255u8);
        delay.delay_ms(255u8);
        delay.delay_ms(255u8);
        delay.delay_ms(255u8);
    }

    // or rtp mode, or software pwm over i2c, might look like this instead
    // haptic.set_standby(false).unwrap();
    // haptic.set_mode(Mode::RealTimePlayback).unwrap();
    // loop {
    //     haptic.set_standby(false).unwrap();

    //     for i in 180..255 {
    //         haptic.set_rtp(i).unwrap();
    //         delay.delay_ms(100u8);
    //     }
    //     for i in (180..255).rev() {
    //         haptic.set_rtp(i).unwrap();
    //         delay.delay_ms(100u8);
    //     }
    //     haptic.set_standby(true).unwrap();
    //     delay.delay_ms(255u8);
    //     delay.delay_ms(255u8);
    //     delay.delay_ms(255u8);
    //     delay.delay_ms(255u8);
    // }

    // or pwm mode, assuming pwm had been configured and was outputting to the
    // in/trig pin
    // haptic.set_mode(Mode::Pwm).unwrap();
    // haptic.set_standby(false).unwrap();
    // loop{
    //       delay.delay_ms(255u8);
    // }
}
