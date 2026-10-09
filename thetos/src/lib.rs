//! Each `fail_*` module is a mistake that must NOT compile. Each `ok_*` module is the correct
//! form of the same operation and must compile. Selected with `--features`; see tools/run_thetos.sh.
#![no_std]

// ---- wrong-mode access: writing to a pin configured as an input --------------------------------
#[cfg(feature = "fail_wrong_mode")]
mod fail_wrong_mode {
    use nucleo_l152re::{GpioLevel, InputPin, OutputPin, PullState, UninitPin, PA6};
    pub fn task() {
        let mut pin = PA6.into_input(PullState::HighZ);
        pin.set(GpioLevel::High);
    }
}

#[cfg(feature = "ok_wrong_mode")]
mod ok_wrong_mode {
    use nucleo_l152re::{GpioLevel, OutputPin, OutputStyle, UninitPin, PA6};
    pub fn task() {
        let mut pin = PA6.into_output(OutputStyle::PushPull);
        pin.set(GpioLevel::High);
    }
}

// ---- stale handle: reusing a pin handle after it changed state ---------------------------------
#[cfg(feature = "fail_stale_handle")]
mod fail_stale_handle {
    use nucleo_l152re::{OutputStyle, PullState, UninitPin, PA6};
    pub fn task() {
        let pin = PA6;
        let _out = pin.into_output(OutputStyle::PushPull);
        let _again = pin.into_input(PullState::HighZ);
    }
}

#[cfg(feature = "ok_stale_handle")]
mod ok_stale_handle {
    use nucleo_l152re::{OutputStyle, UninitPin, PA6};
    pub fn task() {
        let pin = PA6;
        let _out = pin.into_output(OutputStyle::PushPull);
    }
}

// ---- UART used before it is initialised --------------------------------------------------------
#[cfg(feature = "fail_uart_before_init")]
mod fail_uart_before_init {
    use nucleo_l152re::{Serial, Uart, PA2, PA3};
    pub fn task() {
        let mut serial = Serial::usart2(PA2, PA3);
        serial.write_byte(b'x');
    }
}

#[cfg(feature = "ok_uart_before_init")]
mod ok_uart_before_init {
    use nucleo_l152re::{Serial, Uart, UartConfig, UninitUart, PA2, PA3};
    pub fn task() {
        let mut serial = Serial::usart2(PA2, PA3).into_active(UartConfig::default());
        serial.write_byte(b'x');
    }
}

// ---- PWM channel used before the timer is activated --------------------------------------------
#[cfg(feature = "fail_pwm_before_active")]
mod fail_pwm_before_active {
    use nucleo_l152re::{Pwm, Tim3, PA6};
    pub fn task() {
        let pwm = Pwm::<Tim3, _>::new();
        let _channel = pwm.channel1(PA6);
    }
}

#[cfg(feature = "ok_pwm_before_active")]
mod ok_pwm_before_active {
    use nucleo_l152re::{Pwm, PwmConfig, Tim3, UninitPwm, PA6};
    pub fn task() {
        let pwm = Pwm::<Tim3, _>::new().into_active(PwmConfig::default());
        let _channel = pwm.channel1(PA6);
    }
}

// ---- shared state between tasks (a Rust language guarantee, not a ThetOS design feature) -------
#[cfg(feature = "fail_shared_state")]
mod fail_shared_state {
    use core::cell::Cell;
    static DUTY: Cell<u16> = Cell::new(0);
    pub fn task() {
        DUTY.set(100);
    }
}

#[cfg(feature = "ok_shared_state")]
mod ok_shared_state {
    use core::sync::atomic::{AtomicU16, Ordering};
    static DUTY: AtomicU16 = AtomicU16::new(0);
    pub fn task() {
        DUTY.store(100, Ordering::Relaxed);
    }
}
