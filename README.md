# axp2101-rs

**Archived**

This project was archived, please use https://crates.io/crates/axp2101-embedded

Rust driver for the AXP2101 power management IC (PMIC) used in M5Stack CoreS3 and similar ESP32-S3 development boards.


## Features

- **`embedded-hal 1.0` compatible** - Works with the latest embedded HAL traits
- **Register-level control** - Direct access to AXP2101 power management registers
- **DCDC/LDO regulator control** - Enable/disable individual power outputs
- **No_std supported** - Suitable for embedded systems

## Tested Hardware

This driver has been tested and is actively used in:

- **M5Stack CoreS3** - ESP32-S3 based development kit with:
  - AXP2101 PMIC for power management
  - 320x240 ILI9342C display
  - BMI270 6-axis IMU (I2C address: **0x69**)

## Overview

The AXP2101 is a single-cell NVDC PMU (Power Management Unit) with e-gauge capability, commonly used in M5Stack devices. It provides:

- **Multiple DCDC converters** - Efficient step-down regulators for core power
- **LDO regulators** - Low dropout regulators for peripheral power
- **Battery charging management** - Controls battery charging and power path
- **GPIO control** - Configurable GPIO pins for power sequencing

## Power Output Registers

### DCDC1/DCDC3/LDO2/LDO3 Control (Register 0x12)

| Bit | Regulator | Description |
|-----|-----------|-------------|
| 0   | DCDC1     | Main ESP32 power (typically 3.3V) |
| 1   | **DCDC3**   | **Peripheral power (IMU, sensors, etc.)** |
| 2   | LDO2      | LCD/Touch enable |
| 3   | LDO3      | Speaker enable |

**Note:** DCDC3 must be enabled for the BMI270 IMU to function on M5Stack CoreS3.

## Usage

### Basic Initialization

```rust,ignore
use axp2101_rs::{Axp2101, I2CPowerManagementInterface};

// Initialize with M5Stack CoreS3 defaults
let i2c = /* your I2C implementation */;
let axp_interface = I2CPowerManagementInterface::new(i2c);
let mut axp = Axp2101::new(axp_interface);
axp.init()?;
```

The `init()` method enables:
- Charging LED
- ALDO outputs (LCD, Touch, etc.)
- ALDO4 (specific peripheral power)
- **DCDC3** (critical for IMU and sensors)

### Manual Regulator Control

```rust,ignore
use axp2101::Command;

// Enable DCDC3 only (for BMI270 IMU on CoreS3)
axp.set_dcdc13_ldo23_control(0b00000010)?;

// Enable all DCDC and LDO outputs
axp.set_dcdc13_ldo23_control(0b00001111)?;
```

## Register Reference

| Register | Address | Description |
|----------|----------|-------------|
| `0x00` | POWER_STATUS | Power status |
| `0x01` | CHARGE_STATUS | Battery charging status |
| `0x12` | DCDC13_LDO23_CONTROL | DCDC1/DCDC3/LDO2/LDO3 enable |
| `0x69` | CHG_LED | Charging LED control |
| `0x90` | ALDO_ENABLE | ALDO output enable |
| `0x95` | GPIO43_FUNCTION_CONTROL | ALDO4 control |

## I2C Address

- **AXP2101**: `0x34` (7-bit)

## M5Stack CoreS3 Specifics

### Known Power Configuration

Based on M5Stack CoreS3 hardware analysis:

| Regulator | Enabled | Voltage | Purpose |
|-----------|---------|---------|---------|
| DCDC1     | Yes     | 3.3V   | ESP32 main power |
| **DCDC3** | **Yes** | **3.3V** | **BMI270 IMU, MBUS, sensors** |
| DCDC2     | No      | -       | Not used |
| DCDC4     | No      | -       | Not used |
| ALDO1     | Yes     | 1.8V   | Not used |
| ALDO2     | Yes     | 3.3V   | LCD/Touch enable |
| ALDO3     | No      | 3.3V   | Speaker enable |
| **ALDO4** | **Yes** | **3.3V** | **LCD/Touch/TF-card power** |

### BMI270 IMU Configuration

The BMI270 6-axis IMU on the CoreS3 uses:
- **I2C Address**: `0x69` (alternative address - SDO pin pulled high)
- **Power Source**: DCDC3 output
- **I2C Bus**: Shared with AXP2101 and AW9523 on I2C0 (GPIO12=SDA, GPIO11=SCL)

## References

- [AXP2101 Datasheet](https://www.x-powers.com/en.php/Info/download/id/34.html) - Official datasheet
- [M5Stack CoreS3](https://docs.m5stack.com/en/core/CoreS3) - Official documentation
- [M5Stack Core2 Reference](https://github.com/hyx0329/axp2101-rs#m5stack-core2-11-reference) - Power configuration reference

## License

MIT License

## Contributing

Contributions are welcome! This driver was developed to support the M5Stack CoreS3 but may work with other boards using the AXP2101 PMIC.

## Changelog

### 0.3.0
- Added DCDC13_LDO23_CONTROL register support (DCDC1/DCDC3/LDO2/LDO3)
- Fixed `init()` to enable DCDC3 for BMI270 IMU power
- Added `set_dcdc13_ldo23_control()` method for manual regulator control
- Updated documentation with M5Stack CoreS3 specific information

### 0.2.0
- Initial release
- Basic AXP2101 power management
