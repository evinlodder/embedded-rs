#![no_std]
#![no_main]

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use panic_halt as _; // panicking behavior

use tm4c123x_hal::{self as hal, delay::Delay, prelude::*, tm4c123x};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let peripherals = tm4c123x::Peripherals::take().unwrap();
    let cortex_peripherals = Peripherals::take().unwrap();

    // Take ownership over the raw peripheral and convert them into the corresponding HAL structs
    let mut sysctl = peripherals.SYSCTL.constrain();
    let mut portf = peripherals.GPIO_PORTF.split(&mut sysctl.power_control);
    // Configuring the on-board LED (connected to pin PF3) as output
    let mut green_led = portf.pf3.into_push_pull_output();
    let mut blue_led = portf.pf2.into_push_pull_output();

    // Getting access to the systick counter
    let mut delay = Delay::new(cortex_peripherals.SYST, &mut sysctl.clock_setup.freeze());

    green_led.set_high();
    let mut blue_on = false;

    loop {
        // Wait for the systick counter to reach 0
        delay.delay_ms(1000_u16);

        // Toggle the LED manually
        if blue_on {
            blue_led.set_low();
            green_led.set_high();
        } else {
            green_led.set_low();
            blue_led.set_high();
        }
        blue_on = !blue_on;
    }
}
