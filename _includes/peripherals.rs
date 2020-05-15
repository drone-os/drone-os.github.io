use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    periph_gpio_a3,
    pin::{GpioPinMap, GpioPinPeriph},
};

/// TPS22917 Load Switch.
pub struct LoadSwitch<T: GpioPinMap> {
    pin: GpioPinPeriph<T>,
}

impl<T: GpioPinMap> LoadSwitch<T> {
    /// Sets up a new [`LoadSwitch`].
    pub fn init(pin: GpioPinPeriph<T>) -> Self {
        pin.gpio_moder_moder.write_bits(0b01); // general purpose output
        Self { pin }
    }

    /// Turns the switch on.
    pub fn on(&self) {
        self.pin.gpio_bsrr_bs.set_bit(); // tie the pin to +3.3v
    }

    /// Turns the switch off.
    pub fn off(&self) {
        self.pin.gpio_bsrr_br.set_bit(); // tie the pin to ground
    }
}

pub fn handler(reg: Regs) {
    let sd_card_switch = LoadSwitch::init(periph_gpio_a3!(reg));
}
