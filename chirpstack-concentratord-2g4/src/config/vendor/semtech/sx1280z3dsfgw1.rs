use super::super::super::super::config;
use super::super::Configuration;

pub fn new(conf: &config::Configuration) -> Configuration {
    Configuration {
        tty_path: conf
            .gateway
            .com_dev_path
            .clone()
            .unwrap_or("/dev/ttyACM0".to_string()),
        min_max_tx_freq: (2400000000, 2483500000),
        reset_pin: Some((
            "/dev/gpiochip0".to_string(),
            conf.gateway.mcu_reset_pin.unwrap_or(32),
        )),
        boot0_pin: Some((
            "/dev/gpiochip0".to_string(),
            conf.gateway.mcu_boot0_pin.unwrap_or(18),
        )),
    }
}
