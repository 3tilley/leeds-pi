use std::thread;
use std::time::Duration;

// These are Broadcom pins (BCM), they correspond to physical pins 15 and 16 respectively
// https://electronicsmith.com/raspberry-pi-pinout-for-all-models/
const GPIO_LED: u8 = 23;
const GPIO_ACTIVE_BUZZER: u8 = 4;
const GPIO_PASSIVE_BUZZER: u8 = 27;

pub fn beep_freq(times: u8, millis: u64) {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_ACTIVE_BUZZER).unwrap().into_output();
    }

    let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_ACTIVE_BUZZER).unwrap().into_output();
    for _ in 0..times {
        // Blink the LED by setting the pin's logic level high for 500 ms.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("beep");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.set_high();
        }
        thread::sleep(Duration::from_millis(millis));
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("beep");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.set_low();
        }
        thread::sleep(Duration::from_millis(millis));
    }
}

pub fn blink(times: u8) {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
    }

    let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_LED).unwrap().into_output();
    for _ in 0..times {
        // Blink the LED by setting the pin's logic level high for 500 ms.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("blink");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.set_high();
        }
        thread::sleep(Duration::from_millis(500));
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("blink");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
                pin.set_low();
        }
        thread::sleep(Duration::from_millis(500));
    }
}

pub fn play_tone(note: f64, sixteenths: u8, bpm: u64) {
    let period = Duration::from_millis((1000.0 / note) as u64);
    // t = (1 / 16) * sixs * 60 / bpm
    let length = Duration::from_millis((sixteenths as u64 * 60 * 1000) / (4 * bpm));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_PASSIVE_BUZZER).unwrap().into_output();
        pin.set_pwm_frequency(note, 0.5);
        thread::sleep(length);
        pin.clear_pwm();
    }
}

pub fn play_tune(tune: Vec<(f64, u8)>, bpm: u64) {
    for (note, length) in tune {
        play_tone(note, length, bpm);
    }
}
