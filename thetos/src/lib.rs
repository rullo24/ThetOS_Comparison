//! Each `fail_*` module is a mistake that must NOT compile. Each `ok_*` module is the correct
//! form of the same operation and must compile. Selected with `--features`; see tools/run_thetos.sh.
#![no_std]

// =================================== KERNEL LEVEL ===============================================

// ---- a task function that returns ---------------------------------------------------------------
#[cfg(feature = "fail_task_returns")]
mod fail_task_returns {
    use core::ptr::null_mut;
    use nucleo_l152re::{System, TaskId, TaskPriority};
    extern "C" fn task(_arg: *mut ()) -> ! {
        // falls off the end
    }
    pub fn setup(sys: &mut System) {
        let _ = sys.spawn_task(TaskId(1), TaskPriority::DEFAULT, 512, task, null_mut());
    }
}

#[cfg(feature = "ok_task_returns")]
mod ok_task_returns {
    use core::ptr::null_mut;
    use nucleo_l152re::{System, TaskId, TaskPriority};
    extern "C" fn task(_arg: *mut ()) -> ! {
        loop {}
    }
    pub fn setup(sys: &mut System) {
        let _ = sys.spawn_task(TaskId(1), TaskPriority::DEFAULT, 512, task, null_mut());
    }
}

// ---- priority and stack size swapped ------------------------------------------------------------
#[cfg(feature = "fail_swapped_args")]
mod fail_swapped_args {
    use core::ptr::null_mut;
    use nucleo_l152re::{System, TaskId, TaskPriority};
    extern "C" fn task(_arg: *mut ()) -> ! {
        loop {}
    }
    pub fn setup(sys: &mut System) {
        let _ = sys.spawn_task(TaskId(1), 512, TaskPriority::DEFAULT, task, null_mut());
    }
}

#[cfg(feature = "ok_swapped_args")]
mod ok_swapped_args {
    use core::ptr::null_mut;
    use nucleo_l152re::{System, TaskId, TaskPriority};
    extern "C" fn task(_arg: *mut ()) -> ! {
        loop {}
    }
    pub fn setup(sys: &mut System) {
        let _ = sys.spawn_task(TaskId(1), TaskPriority::DEFAULT, 512, task, null_mut());
    }
}

// ---- critical section never exited: not expressible, so there is no fail_ fixture ---------------
// ThetOS offers only the closure form; the section ends when the closure returns, on every path.
#[cfg(feature = "ok_critical_section")]
mod ok_critical_section {
    use cortex_m::CortexMCriticalSection;
    use specs::kernel::CriticalSection;
    pub fn task(early: bool) -> u32 {
        CortexMCriticalSection.with_execute(|| {
            if early {
                return 1;
            }
            2
        })
    }
}

// =================================== DRIVER LEVEL ===============================================

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

// ---- stale handle: reusing a UART handle after it changed state --------------------------------
#[cfg(feature = "fail_stale_handle")]
mod fail_stale_handle {
    use nucleo_l152re::{Serial, UartConfig, UninitUart, PA2, PA3};
    pub fn task() {
        let serial = Serial::usart2(PA2, PA3);
        let _active = serial.into_active(UartConfig::default());
        let _again = serial.into_active(UartConfig::default());
    }
}

#[cfg(feature = "ok_stale_handle")]
mod ok_stale_handle {
    use nucleo_l152re::{Serial, UartConfig, UninitUart, PA2, PA3};
    pub fn task() {
        let serial = Serial::usart2(PA2, PA3);
        let _active = serial.into_active(UartConfig::default());
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

// ---- shared state between tasks (kernel level; see KERNEL LEVEL group in the report) (a Rust language guarantee, not a ThetOS design feature) -------
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
