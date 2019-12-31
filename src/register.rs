use enum_repr::EnumRepr;

/// Register addresses
///
/// From TPS65185 data sheet: <http://www.ti.com/lit/ds/symlink/tps65185.pdf>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
// TODO: More extensive documentation for each register
pub enum Register {
    /// Thermistor Readout
    TMST_VALUE = 0x00,

    /// Enable
    ENABLE = 0x01,

    /// Voltage Adjustment
    VADJ = 0x02,

    /// VCOM1
    VCOM1 = 0x03,

    /// VCOM2
    VCOM2 = 0x04,

    /// Interrupt Enable 1
    INT_EN1 = 0x05,

    /// Interrupt Enable 2
    INT_EN2 = 0x06,

    /// Interrupt 1
    INT1 = 0x07,

    /// Interrupt 2
    INT2 = 0x08,

    /// Power-Up Sequence 0
    UPSEQ0 = 0x09,

    /// Power-Up Sequence 1
    UPSEQ1 = 0x0a,

    /// Power-Down Sequence 0
    DWNSEQ0 = 0x0b,

    /// Power-Down Sequence 1
    DWNSEQ1 = 0x0c,

    /// Thermistor 1
    TMST1 = 0x0d,

    /// Thermistor 2
    TMST2 = 0x0e,

    /// Power Good Status
    PG = 0x0f,

    /// Revision and Version Control
    REVID = 0x10,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[EnumRepr(type = "u8")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceVersion {
    Tps65185_1p0 = 0x45,
    Tps65185_1p1 = 0x55,
    Tps65185_1p2 = 0x65,
    Tps651851_1p0 = 0x66,
}
