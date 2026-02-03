#![no_std]
#![allow(dead_code)]

// Based on https://github.com/tuupola/axp192
// and https://github.com/m5stack/M5CoreS3/blob/main/src/AXP2101.cpp
const AXP2101_ADDRESS: u8 = 0x34;
/* Power control registers */
const AXP2101_POWER_STATUS: u8 = 0x00;
const AXP2101_CHARGE_STATUS: u8 = 0x01;
const AXP2101_OTG_VBUS_STATUS: u8 = 0x04;
const AXP2101_DATA_BUFFER0: u8 = 0x06;
const AXP2101_DATA_BUFFER1: u8 = 0x07;
const AXP2101_DATA_BUFFER2: u8 = 0x08;
const AXP2101_DATA_BUFFER3: u8 = 0x09;
const AXP2101_DATA_BUFFER4: u8 = 0x0a;
const AXP2101_DATA_BUFFER5: u8 = 0x0b;
/* Output control: 2 EXTEN, 0 DCDC2 */
const AXP2101_EXTEN_DCDC2_CONTROL: u8 = 0x10;
/* Power output control: 6 EXTEN, 4 DCDC2, 3 LDO3, 2 LDO2, 1 DCDC3, 0 DCDC1 */
const AXP2101_DCDC13_LDO23_CONTROL: u8 = 0x12;
const AXP2101_DCDC2_VOLTAGE: u8 = 0x23;
const AXP2101_DCDC2_SLOPE: u8 = 0x25;
const AXP2101_DCDC1_VOLTAGE: u8 = 0x26;
const AXP2101_DCDC3_VOLTAGE: u8 = 0x27;
/* Output voltage control: 7-4 LDO2, 3-0 LDO3 */
const AXP2101_LDO23_VOLTAGE: u8 = 0x28;
const AXP2101_VBUS_IPSOUT_CHANNEL: u8 = 0x30;
const AXP2101_SHUTDOWN_VOLTAGE: u8 = 0x31;
const AXP2101_SHUTDOWN_BATTERY_CHGLED_CONTROL: u8 = 0x32;
const AXP2101_CHARGE_CONTROL_1: u8 = 0x33;
const AXP2101_CHARGE_CONTROL_2: u8 = 0x34;
const AXP2101_BATTERY_CHARGE_CONTROL: u8 = 0x35;
const AXP2101_PEK: u8 = 0x36;
const AXP2101_DCDC_FREQUENCY: u8 = 0x37;
const AXP2101_BATTERY_CHARGE_LOW_TEMP: u8 = 0x38;
const AXP2101_BATTERY_CHARGE_HIGH_TEMP: u8 = 0x39;
const AXP2101_APS_LOW_POWER1: u8 = 0x3A;
const AXP2101_APS_LOW_POWER2: u8 = 0x3B;
const AXP2101_BATTERY_DISCHARGE_LOW_TEMP: u8 = 0x3c;
const AXP2101_BATTERY_DISCHARGE_HIGH_TEMP: u8 = 0x3d;
const AXP2101_DCDC_MODE: u8 = 0x80;
const AXP2101_ADC_ENABLE_1: u8 = 0x82;
const AXP2101_ADC_ENABLE_2: u8 = 0x83;
const AXP2101_ADC_RATE_TS_PIN: u8 = 0x84;
const AXP2101_GPIO30_INPUT_RANGE: u8 = 0x85;
const AXP2101_GPIO0_ADC_IRQ_RISING: u8 = 0x86;
const AXP2101_GPIO0_ADC_IRQ_FALLING: u8 = 0x87;
const AXP2101_TIMER_CONTROL: u8 = 0x8a;
const AXP2101_VBUS_MONITOR: u8 = 0x8b;
const AXP2101_TEMP_SHUTDOWN_CONTROL: u8 = 0x8f;

/* GPIO control registers */

const AXP2101_GPIO0_LDOIO0_VOLTAGE: u8 = 0x91;
const AXP2101_GPIO1_CONTROL: u8 = 0x92;
const AXP2101_GPIO2_CONTROL: u8 = 0x93;
const AXP2101_GPIO20_SIGNAL_STATUS: u8 = 0x94;
const AXP2101_GPIO43_FUNCTION_CONTROL: u8 = 0x95;
const AXP2101_GPIO43_SIGNAL_STATUS: u8 = 0x96;
const AXP2101_GPIO20_PULLDOWN_CONTROL: u8 = 0x97;
const AXP2101_PWM1_FREQUENCY: u8 = 0x98;
const AXP2101_PWM1_DUTY_CYCLE_1: u8 = 0x99;
const AXP2101_PWM1_DUTY_CYCLE_2: u8 = 0x9a;
const AXP2101_PWM2_FREQUENCY: u8 = 0x9b;
const AXP2101_PWM2_DUTY_CYCLE_1: u8 = 0x9c;
const AXP2101_PWM2_DUTY_CYCLE_2: u8 = 0x9d;
const AXP2101_N_RSTO_GPIO5_CONTROL: u8 = 0x9e;

/* Interrupt control registers */
const AXP2101_ENABLE_CONTROL_1: u8 = 0x40;
const AXP2101_ENABLE_CONTROL_2: u8 = 0x41;
const AXP2101_ENABLE_CONTROL_3: u8 = 0x42;
const AXP2101_ENABLE_CONTROL_4: u8 = 0x43;
const AXP2101_ENABLE_CONTROL_5: u8 = 0x4a;
const AXP2101_IRQ_STATUS_1: u8 = 0x44;
const AXP2101_IRQ_STATUS_2: u8 = 0x45;
const AXP2101_IRQ_STATUS_3: u8 = 0x46;
const AXP2101_IRQ_STATUS_4: u8 = 0x47;
const AXP2101_IRQ_STATUS_5: u8 = 0x4d;

/* ADC data registers */
const AXP2101_ACIN_VOLTAGE: u8 = 0x56;
const AXP2101_ACIN_CURRENT: u8 = 0x58;
const AXP2101_VBUS_VOLTAGE: u8 = 0x5a;
const AXP2101_VBUS_CURRENT: u8 = 0x5c;
const AXP2101_TEMP: u8 = 0x5e;
const AXP2101_TS_INPUT: u8 = 0x62;
const AXP2101_GPIO0_VOLTAGE: u8 = 0x64;
const AXP2101_GPIO1_VOLTAGE: u8 = 0x66;
const AXP2101_GPIO2_VOLTAGE: u8 = 0x68;
const AXP2101_GPIO3_VOLTAGE: u8 = 0x6a;
const AXP2101_BATTERY_POWER: u8 = 0x70;
const AXP2101_BATTERY_VOLTAGE: u8 = 0x78;
const AXP2101_CHARGE_CURRENT: u8 = 0x7a;
const AXP2101_DISCHARGE_CURRENT: u8 = 0x7c;
const AXP2101_APS_VOLTAGE: u8 = 0x7e;
const AXP2101_CHARGE_COULOMB: u8 = 0xb0;
const AXP2101_DISCHARGE_COULOMB: u8 = 0xb4;
const AXP2101_COULOMB_COUNTER_CONTROL: u8 = 0xb8;

/* Computed ADC */
const AXP2101_COULOMB_COUNTER: u8 = 0xff;

// AXP2101 datasheet: https://www.x-powers.com/en.php/Info/download/id/34.html
const AXP2101_CHG_LED: u8 = 0x69;
const AXP2101_ALDO_ENABLE: u8 = 0x90;
const AXP2101_ALDO4: u8 = 0x95;

/// Commands for AXP2101 power management IC
///
/// Register 0x12 (DCDC13_LDO23_CONTROL) bits:
/// - Bit 0: DCDC1 enable
/// - Bit 1: DCDC3 enable
/// - Bit 2: LDO2 enable
/// - Bit 3: LDO3 enable
///
/// For M5Stack CoreS3, DCDC3 (bit 1) powers the BMI270 IMU and should be enabled.
#[derive(Debug, Clone, Copy)]
pub enum Command {
    /// Enable/disable charging LED
    ChgLed(bool),

    /// Enable/disable all ALDO outputs (writes to ALDO_ENABLE register at 0x90)
    AldoEnable(bool),

    /// Enable/disable ALDO4 output (for LCD/Touch/TF-card on CoreS3)
    Aldo4(bool),

    /// Enable DCDC1, DCDC3, LDO2, LDO3 regulators
    /// Value is a bitmask where bit 0=DCDC1, bit 1=DCDC3, bit 2=LDO2, bit 3=LDO3
    /// For M5Stack CoreS3 with BMI270 IMU, use 0b00000010 to enable DCDC3
    Dcdc13Ldo23Control(u8),
}

pub enum DataFormat<'a> {
    /// Slice of unsigned bytes
    U8(&'a [u8]),
}

impl Command {
    /// Send command to AXP2101 via I2C
    pub fn send<I>(self, iface: &mut I) -> Result<(), Axp2101Error>
    where
        I: Axp2101ReadWrite,
    {
        let (data, len) = match self {
            // Command structure: [register_address, value_to_write]

            // Charging LED control (register 0x69)
            Command::ChgLed(_on) => ([AXP2101_CHG_LED, 0b00110101], 2),

            // ALDO enable register (0x90) - 0xBF enables multiple ALDO outputs
            Command::AldoEnable(_on) => ([AXP2101_ALDO_ENABLE, 0xBF], 2),

            // ALDO4 control (register 0x95 = GPIO43_FUNCTION_CONTROL)
            // 0b00011100 enables ALDO4
            Command::Aldo4(_on) => ([AXP2101_ALDO4, 0b00011100], 2),

            // DCDC1/DCDC3/LDO2/LDO3 control register (0x12)
            // Bitmask: bit 0=DCDC1, bit 1=DCDC3, bit 2=LDO2, bit 3=LDO3
            // For CoreS3 BMI270 IMU: enable DCDC3 (0b00000010)
            // For all regulators: 0b00001111 (DCDC1+DCDC3+LDO2+LDO3)
            Command::Dcdc13Ldo23Control(mask) => ([AXP2101_DCDC13_LDO23_CONTROL, mask], 2),
        };
        iface.send_commands(DataFormat::U8(&data[0..len]))
    }
}

#[derive(Debug)]
pub enum Axp2101Error {
    NotSupported,
    InvalidArgument,
    ReadError,
    WriteError,
}

pub trait Axp2101ReadWrite {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), Axp2101Error>;
}

pub struct Axp2101<I> {
    interface: I,
}

// Implement Axp2101ReadWrite for I2CInterface
impl<I> Axp2101ReadWrite for I2CInterface<I>
where
    I: embedded_hal::i2c::I2c,
{
    // Send commands over I2C to AXP2101
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), Axp2101Error> {
        let mut data_buf = [0];

        match cmd {
            DataFormat::U8(data) => {
                // Read current register value (optional, for verification)
                let _ = self.i2c.write_read(self.addr, &[data[0]], &mut data_buf);

                // Write new value
                self.i2c
                    .write(self.addr, data)
                    .map_err(|_| Axp2101Error::WriteError)
            }
        }
    }
}

impl<I> Axp2101<I>
where
    I: Axp2101ReadWrite,
{
    /// Create a new AXP2101 driver instance
    pub fn new(interface: I) -> Self {
        Self { interface }
    }

    /// Initialize AXP2101 with default settings for M5Stack CoreS3
    ///
    /// This enables:
    /// - Charging LED
    /// - ALDO outputs (for LCD/Touch/TF-card)
    /// - ALDO4 (specific output)
    /// - DCDC3 (powers BMI270 IMU and other peripherals)
    pub fn init(&mut self) -> Result<(), Axp2101Error> {
        // Enable charging LED
        Command::ChgLed(true).send(&mut self.interface)?;

        // Enable ALDO outputs (LCD, Touch, etc.)
        Command::AldoEnable(true).send(&mut self.interface)?;

        // Enable ALDO4
        Command::Aldo4(true).send(&mut self.interface)?;

        // Enable DCDC3 - CRITICAL for M5Stack CoreS3!
        // DCDC3 powers the BMI270 IMU and other 3.3V peripherals
        // Bit 1 of register 0x12 = DCDC3 enable
        Command::Dcdc13Ldo23Control(0b00000010).send(&mut self.interface)?;

        Ok(())
    }

    /// Enable DCDC1, DCDC3, LDO2, and/or LDO3 regulators
    ///
    /// # Arguments
    /// * `mask` - Bitmask where:
    ///   - Bit 0: DCDC1 enable
    ///   - Bit 1: DCDC3 enable (IMU power on M5Stack CoreS3)
    ///   - Bit 2: LDO2 enable
    ///   - Bit 3: LDO3 enable
    ///
    /// # Example
    /// ```ignore
    /// // Enable DCDC3 only (for BMI270 IMU)
    /// axp.set_dcdc13_ldo23_control(0b00000010)?;
    ///
    /// // Enable all DCDC and LDO outputs
    /// axp.set_dcdc13_ldo23_control(0b00001111)?;
    /// ```
    pub fn set_dcdc13_ldo23_control(&mut self, mask: u8) -> Result<(), Axp2101Error> {
        Command::Dcdc13Ldo23Control(mask).send(&mut self.interface)
    }

    /// Release the I2C interface, returning ownership of the underlying I2C peripheral
    pub fn release_i2c<I2C>(self) -> I2C
    where
        I: Into<I2CPowerManagementInterface<I2C>>,
        I2C: embedded_hal::i2c::I2c,
    {
        let interface: I2CPowerManagementInterface<I2C> = self.interface.into();
        interface.release()
    }
}

// Implement Axp2101ReadWrite for I2CPowerManagementInterface
impl<I2C> Axp2101ReadWrite for I2CPowerManagementInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), Axp2101Error> {
        let mut data_buf = [0];

        match cmd {
            DataFormat::U8(data) => {
                // Read current register value (optional, for verification)
                let _ = self
                    .i2c
                    .write_read(AXP2101_ADDRESS, &[data[0]], &mut data_buf);

                // Write new value
                self.i2c
                    .write(AXP2101_ADDRESS, data)
                    .map_err(|_| Axp2101Error::WriteError)
            }
        }
    }
}

pub struct I2CInterface<I2C> {
    i2c: I2C,
    addr: u8,
    data_byte: u8,
}

impl<I2C> I2CInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create new I2C interface for communication
    pub fn new(i2c: I2C, addr: u8, data_byte: u8) -> Self {
        Self {
            i2c,
            addr,
            data_byte,
        }
    }

    /// Consume the interface and return the underlying I2C peripheral
    pub fn release(self) -> I2C {
        self.i2c
    }
}

#[derive(Debug, Copy, Clone)]
pub struct I2CPowerManagementInterface<I2C> {
    i2c: I2C,
}

impl<I2C> I2CPowerManagementInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create new power management interface
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Consume the interface and return the underlying I2C peripheral
    pub fn release(self) -> I2C {
        self.i2c
    }
}
