use std::thread;
use std::time::Duration;

// These are Broadcom pins (BCM), they correspond to physical pins 15 and 16 respectively
// https://electronicsmith.com/raspberry-pi-pinout-for-all-models/
//const GPIO_BUZZER: u8 = 22;
const GPIO_BUZZER: u8 = 27;
const GPIO_LED: u8 = 23;

pub fn beep_freq(times: u8, voltage: u8, millis: u64) {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
    let mut pin = rppal::gpio::Gpio::new().unwrap().get(GPIO_BUZZER).unwrap().into_output();
    }

    for _ in 0..times {
        // Blink the LED by setting the pin's logic level high for 500 ms.
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            println!("beep");
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            pin.write(voltage.into());
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
