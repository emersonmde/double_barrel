use crate::lcd::LiquidCrystalDisplay;

pub struct InputOutputModule {
    port_a: u8,
    port_b: u8,
    ddr_a: u8,
    ddr_b: u8,

    lcd: LiquidCrystalDisplay,
}

impl InputOutputModule {
    pub fn new() -> Self {
        Self { ddr_a: 0, ddr_b: 0, port_a: 0, port_b: 0, lcd: LiquidCrystalDisplay::new() }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x6000 => {
                self.port_b = value;
                self.lcd.set_data_bus(self.port_b & self.ddr_b);
            }
            0x6001 => {
                self.port_a = value;
                self.lcd.set_control_bus((self.port_a & self.ddr_a) >> 5);
            }
            0x6002 => self.ddr_b = value,
            0x6003 => self.ddr_a = value,
            _ => println!("Not a valid IO Address"),
        }
    }
}