const LCD_ENABLE: u8 = 0b100;
const LCD_RW: u8 = 0b010;
const LCD_RS: u8 = 0b001;

pub struct LiquidCrystalDisplay {
    instruction_register: u8,
    data_bus: u8,
    control_bus: u8,
}

impl LiquidCrystalDisplay {
    pub fn new() -> Self {
        Self { instruction_register: 0, data_bus: 0, control_bus: 0 }
    }

    pub fn set_data_bus(&mut self, data: u8) {
        println!("SET DATA BUS {:?}", data as char);
        self.data_bus = data;
    }

    pub fn set_control_bus(&mut self, data: u8) {
        println!("SET CONTROL BUS {:#05b}", data);
        let enable_was_low: bool = (self.control_bus & LCD_ENABLE) == 0;
        self.control_bus = data;
        let enable = data & LCD_ENABLE;
        let rs = data & LCD_RS;
        let rw = data & LCD_RW;
        if enable_was_low && enable == 1 {
            // Rising enable
            // TODO: match?
            if rs == 1 {
                if rw == 1 {
                    // read from display
                } else {
                    // write to display
                }
            } else {
                if rw == 1 {
                    // read busy flag
                } else {
                    // write instruction
                }
            }
        }
    }
}
