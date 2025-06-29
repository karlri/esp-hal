//! # GPIO configuration module (ESP32)
//!
//! ## Overview
//!
//! The `GPIO` module provides functions and configurations for controlling the
//! `General Purpose Input/Output` pins on the `ESP32` chip. It allows you to
//! configure pins as inputs or outputs, set their state and read their state.
//!
//! Let's get through the functionality and configurations provided by this GPIO
//! module:
//!   - `io_mux_reg(gpio_num: u8) -> &'static io_mux::GPIO0:`:
//!       * This function returns a reference to the GPIO register associated
//!         with the given GPIO number. It uses unsafe code and transmutation to
//!         access the GPIO registers based on the provided GPIO number.
//!   - `gpio_intr_enable(int_enable: bool, nmi_enable: bool) -> u8`:
//!       * This function enables or disables GPIO interrupts and Non-Maskable
//!         Interrupts (NMI). It takes two boolean arguments int_enable and
//!         nmi_enable to control the interrupt and NMI enable settings. The
//!         function returns an u8 value representing the interrupt enable
//!         settings.
//!   - `errata36(pin_num: u8, pull_up: bool, pull_down: bool)`:
//!       * Handles the configuration of pull-up and pull-down resistors for
//!         specific GPIO pins
//!   - `gpio` block:
//!       * Defines the pin configurations for various GPIO pins. Each line
//!         represents a pin and its associated options such as input/output
//!         mode, analog capability, and corresponding functions.
//!   - `analog` block:
//!       * Block defines the analog capabilities of various GPIO pins. Each
//!         line represents a pin and its associated options such as mux
//!         selection, function selection, and input enable.
//!   - `enum InputSignal`:
//!       * This enumeration defines input signals for the GPIO mux. Each input
//!         signal is assigned a specific value.
//!   - `enum OutputSignal`:
//!       * This enumeration defines output signals for the GPIO mux. Each
//!         output signal is assigned a specific value.
//!
//! This trait provides functions to read the interrupt status and NMI status
//! registers for both the `PRO CPU` and `APP CPU`. The implementation uses the
//! `gpio` peripheral to access the appropriate registers.

use core::mem::transmute;

use crate::{
    gpio::AlternateFunction,
    pac::io_mux,
    peripherals::{GPIO, IO_MUX},
    system::Cpu,
};

pub(crate) const FUNC_IN_SEL_OFFSET: usize = 0;

pub(crate) type InputSignalType = u16;
pub(crate) type OutputSignalType = u16;
pub(crate) const OUTPUT_SIGNAL_MAX: u16 = 548;
pub(crate) const INPUT_SIGNAL_MAX: u16 = 539;

pub(crate) const ONE_INPUT: u8 = 0x38;
pub(crate) const ZERO_INPUT: u8 = 0x30;

pub(crate) const GPIO_FUNCTION: AlternateFunction = AlternateFunction::_2;

pub(crate) fn io_mux_reg(gpio_num: u8) -> &'static io_mux::GPIO0 {
    let iomux = IO_MUX::regs();

    unsafe {
        match gpio_num {
            0 => transmute::<&'static io_mux::GPIO0, &'static io_mux::GPIO0>(iomux.gpio0()),
            1 => transmute::<&'static io_mux::GPIO1, &'static io_mux::GPIO0>(iomux.gpio1()),
            2 => transmute::<&'static io_mux::GPIO2, &'static io_mux::GPIO0>(iomux.gpio2()),
            3 => transmute::<&'static io_mux::GPIO3, &'static io_mux::GPIO0>(iomux.gpio3()),
            4 => transmute::<&'static io_mux::GPIO4, &'static io_mux::GPIO0>(iomux.gpio4()),
            5 => transmute::<&'static io_mux::GPIO5, &'static io_mux::GPIO0>(iomux.gpio5()),
            6 => transmute::<&'static io_mux::GPIO6, &'static io_mux::GPIO0>(iomux.gpio6()),
            7 => transmute::<&'static io_mux::GPIO7, &'static io_mux::GPIO0>(iomux.gpio7()),
            8 => transmute::<&'static io_mux::GPIO8, &'static io_mux::GPIO0>(iomux.gpio8()),
            9 => transmute::<&'static io_mux::GPIO9, &'static io_mux::GPIO0>(iomux.gpio9()),
            10 => transmute::<&'static io_mux::GPIO10, &'static io_mux::GPIO0>(iomux.gpio10()),
            11 => transmute::<&'static io_mux::GPIO11, &'static io_mux::GPIO0>(iomux.gpio11()),
            12 => transmute::<&'static io_mux::GPIO12, &'static io_mux::GPIO0>(iomux.gpio12()),
            13 => transmute::<&'static io_mux::GPIO13, &'static io_mux::GPIO0>(iomux.gpio13()),
            14 => transmute::<&'static io_mux::GPIO14, &'static io_mux::GPIO0>(iomux.gpio14()),
            15 => transmute::<&'static io_mux::GPIO15, &'static io_mux::GPIO0>(iomux.gpio15()),
            16 => transmute::<&'static io_mux::GPIO16, &'static io_mux::GPIO0>(iomux.gpio16()),
            17 => transmute::<&'static io_mux::GPIO17, &'static io_mux::GPIO0>(iomux.gpio17()),
            18 => transmute::<&'static io_mux::GPIO18, &'static io_mux::GPIO0>(iomux.gpio18()),
            19 => transmute::<&'static io_mux::GPIO19, &'static io_mux::GPIO0>(iomux.gpio19()),
            20 => transmute::<&'static io_mux::GPIO20, &'static io_mux::GPIO0>(iomux.gpio20()),
            21 => transmute::<&'static io_mux::GPIO21, &'static io_mux::GPIO0>(iomux.gpio21()),
            22 => transmute::<&'static io_mux::GPIO22, &'static io_mux::GPIO0>(iomux.gpio22()),
            23 => transmute::<&'static io_mux::GPIO23, &'static io_mux::GPIO0>(iomux.gpio23()),
            24 => transmute::<&'static io_mux::GPIO24, &'static io_mux::GPIO0>(iomux.gpio24()),
            25 => transmute::<&'static io_mux::GPIO25, &'static io_mux::GPIO0>(iomux.gpio25()),
            26 => transmute::<&'static io_mux::GPIO26, &'static io_mux::GPIO0>(iomux.gpio26()),
            27 => transmute::<&'static io_mux::GPIO27, &'static io_mux::GPIO0>(iomux.gpio27()),
            32 => transmute::<&'static io_mux::GPIO32, &'static io_mux::GPIO0>(iomux.gpio32()),
            33 => transmute::<&'static io_mux::GPIO33, &'static io_mux::GPIO0>(iomux.gpio33()),
            34 => transmute::<&'static io_mux::GPIO34, &'static io_mux::GPIO0>(iomux.gpio34()),
            35 => transmute::<&'static io_mux::GPIO35, &'static io_mux::GPIO0>(iomux.gpio35()),
            36 => transmute::<&'static io_mux::GPIO36, &'static io_mux::GPIO0>(iomux.gpio36()),
            37 => transmute::<&'static io_mux::GPIO37, &'static io_mux::GPIO0>(iomux.gpio37()),
            38 => transmute::<&'static io_mux::GPIO38, &'static io_mux::GPIO0>(iomux.gpio38()),
            39 => transmute::<&'static io_mux::GPIO39, &'static io_mux::GPIO0>(iomux.gpio39()),
            other => panic!("GPIO {} does not exist", other),
        }
    }
}

pub(crate) fn gpio_intr_enable(int_enable: bool, nmi_enable: bool) -> u8 {
    match Cpu::current() {
        Cpu::AppCpu => int_enable as u8 | ((nmi_enable as u8) << 1),
        Cpu::ProCpu => ((int_enable as u8) << 2) | ((nmi_enable as u8) << 3),
    }
}

/// Peripheral input signals for the GPIO mux
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[doc(hidden)]
pub enum InputSignal {
    SPICLK                = 0,
    SPIQ                  = 1,
    SPID                  = 2,
    SPIHD                 = 3,
    SPIWP                 = 4,
    SPICS0                = 5,
    SPICS1                = 6,
    SPICS2                = 7,
    HSPICLK               = 8,
    HSPIQ                 = 9,
    HSPID                 = 10,
    HSPICS0               = 11,
    HSPIHD                = 12,
    HSPIWP                = 13,
    U0RXD                 = 14,
    U0CTS                 = 15,
    U0DSR                 = 16,
    U1RXD                 = 17,
    U1CTS                 = 18,
    I2S0O_BCK             = 23,
    I2S1O_BCK             = 24,
    I2S0O_WS              = 25,
    I2S1O_WS              = 26,
    I2S0I_BCK             = 27,
    I2S0I_WS              = 28,
    I2CEXT0_SCL           = 29,
    I2CEXT0_SDA           = 30,
    PWM0_SYNC0            = 31,
    PWM0_SYNC1            = 32,
    PWM0_SYNC2            = 33,
    PWM0_F0               = 34,
    PWM0_F1               = 35,
    PWM0_F2               = 36,
    PCNT0_SIG_CH0         = 39,
    PCNT0_SIG_CH1         = 40,
    PCNT0_CTRL_CH0        = 41,
    PCNT0_CTRL_CH1        = 42,
    PCNT1_SIG_CH0         = 43,
    PCNT1_SIG_CH1         = 44,
    PCNT1_CTRL_CH0        = 45,
    PCNT1_CTRL_CH1        = 46,
    PCNT2_SIG_CH0         = 47,
    PCNT2_SIG_CH1         = 48,
    PCNT2_CTRL_CH0        = 49,
    PCNT2_CTRL_CH1        = 50,
    PCNT3_SIG_CH0         = 51,
    PCNT3_SIG_CH1         = 52,
    PCNT3_CTRL_CH0        = 53,
    PCNT3_CTRL_CH1        = 54,
    PCNT4_SIG_CH0         = 55,
    PCNT4_SIG_CH1         = 56,
    PCNT4_CTRL_CH0        = 57,
    PCNT4_CTRL_CH1        = 58,
    HSPICS1               = 61,
    HSPICS2               = 62,
    VSPICLK               = 63,
    VSPIQ                 = 64,
    VSPID                 = 65,
    VSPIHD                = 66,
    VSPIWP                = 67,
    VSPICS0               = 68,
    VSPICS1               = 69,
    VSPICS2               = 70,
    PCNT5_SIG_CH0         = 71,
    PCNT5_SIG_CH1         = 72,
    PCNT5_CTRL_CH0        = 73,
    PCNT5_CTRL_CH1        = 74,
    PCNT6_SIG_CH0         = 75,
    PCNT6_SIG_CH1         = 76,
    PCNT6_CTRL_CH0        = 77,
    PCNT6_CTRL_CH1        = 78,
    PCNT7_SIG_CH0         = 79,
    PCNT7_SIG_CH1         = 80,
    PCNT7_CTRL_CH0        = 81,
    PCNT7_CTRL_CH1        = 82,
    RMT_SIG_0             = 83,
    RMT_SIG_1             = 84,
    RMT_SIG_2             = 85,
    RMT_SIG_3             = 86,
    RMT_SIG_4             = 87,
    RMT_SIG_5             = 88,
    RMT_SIG_6             = 89,
    RMT_SIG_7             = 90,
    TWAI_RX               = 94,
    I2CEXT1_SCL           = 95,
    I2CEXT1_SDA           = 96,
    HOST_CARD_DETECT_N_1  = 97,
    HOST_CARD_DETECT_N_2  = 98,
    HOST_CARD_WRITE_PRT_1 = 99,
    HOST_CARD_WRITE_PRT_2 = 100,
    HOST_CARD_INT_N_1     = 101,
    HOST_CARD_INT_N_2     = 102,
    PWM1_SYNC0            = 103,
    PWM1_SYNC1            = 104,
    PWM1_SYNC2            = 105,
    PWM1_F0               = 106,
    PWM1_F1               = 107,
    PWM1_F2               = 108,
    PWM0_CAP0             = 109,
    PWM0_CAP1             = 110,
    PWM0_CAP2             = 111,
    PWM1_CAP0             = 112,
    PWM1_CAP1             = 113,
    PWM1_CAP2             = 114,
    I2S0I_DATA_0          = 140,
    I2S0I_DATA_1          = 141,
    I2S0I_DATA_2          = 142,
    I2S0I_DATA_3          = 143,
    I2S0I_DATA_4          = 144,
    I2S0I_DATA_5          = 145,
    I2S0I_DATA_6          = 146,
    I2S0I_DATA_7          = 147,
    I2S0I_DATA_8          = 148,
    I2S0I_DATA_9          = 149,
    I2S0I_DATA_10         = 150,
    I2S0I_DATA_11         = 151,
    I2S0I_DATA_12         = 152,
    I2S0I_DATA_13         = 153,
    I2S0I_DATA_14         = 154,
    I2S0I_DATA_15         = 155,
    I2S1I_BCK             = 164,
    I2S1I_WS              = 165,
    I2S1I_DATA_0          = 166,
    I2S1I_DATA_1          = 167,
    I2S1I_DATA_2          = 168,
    I2S1I_DATA_3          = 169,
    I2S1I_DATA_4          = 170,
    I2S1I_DATA_5          = 171,
    I2S1I_DATA_6          = 172,
    I2S1I_DATA_7          = 173,
    I2S1I_DATA_8          = 174,
    I2S1I_DATA_9          = 175,
    I2S1I_DATA_10         = 176,
    I2S1I_DATA_11         = 177,
    I2S1I_DATA_12         = 178,
    I2S1I_DATA_13         = 179,
    I2S1I_DATA_14         = 180,
    I2S1I_DATA_15         = 181,
    I2S0I_H_SYNC          = 190,
    I2S0I_V_SYNC          = 191,
    I2S0I_H_ENABLE        = 192,
    I2S1I_H_SYNC          = 193,
    I2S1I_V_SYNC          = 194,
    I2S1I_H_ENABLE        = 195,
    U2RXD                 = 198,
    U2CTS                 = 199,
    EMAC_MDC              = 200,
    EMAC_MDI              = 201,
    EMAC_CRS              = 202,
    EMAC_COL              = 203,
    PCMFSYNC              = 204,
    PCMCLK                = 205,
    PCMDIN                = 206,

    SD_DATA0              = 512,
    SD_DATA1,
    SD_DATA2,
    SD_DATA3,
    HS1_DATA0,
    HS1_DATA1,
    HS1_DATA2,
    HS1_DATA3,
    HS1_DATA4,
    HS1_DATA5,
    HS1_DATA6,
    HS1_DATA7,
    HS2_DATA0,
    HS2_DATA1,
    HS2_DATA2,
    HS2_DATA3,

    EMAC_TX_CLK,
    EMAC_RXD2,
    EMAC_TX_ER,
    EMAC_RX_CLK,
    EMAC_RX_ER,
    EMAC_RXD3,
    EMAC_RXD0,
    EMAC_RXD1,
    EMAC_RX_DV,

    MTDI,
    MTCK,
    MTMS,
}

/// Peripheral output signals for the GPIO mux
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[doc(hidden)]
pub enum OutputSignal {
    SPICLK                   = 0,
    SPIQ                     = 1,
    SPID                     = 2,
    SPIHD                    = 3,
    SPIWP                    = 4,
    SPICS0                   = 5,
    SPICS1                   = 6,
    SPICS2                   = 7,
    HSPICLK                  = 8,
    HSPIQ                    = 9,
    HSPID                    = 10,
    HSPICS0                  = 11,
    HSPIHD                   = 12,
    HSPIWP                   = 13,
    U0TXD                    = 14,
    U0RTS                    = 15,
    U0DTR                    = 16,
    U1TXD                    = 17,
    U1RTS                    = 18,
    I2S0O_BCK                = 23,
    I2S1O_BCK                = 24,
    I2S0O_WS                 = 25,
    I2S1O_WS                 = 26,
    I2S0I_BCK                = 27,
    I2S0I_WS                 = 28,
    I2CEXT0_SCL              = 29,
    I2CEXT0_SDA              = 30,
    SDIO_TOHOSTT             = 31,
    PWM0_0A                  = 32,
    PWM0_0B                  = 33,
    PWM0_1A                  = 34,
    PWM0_1B                  = 35,
    PWM0_2A                  = 36,
    PWM0_2B                  = 37,
    HSPICS1                  = 61,
    HSPICS2                  = 62,
    VSPICLK                  = 63,
    VSPIQ                    = 64,
    VSPID                    = 65,
    VSPIHD                   = 66,
    VSPIWP                   = 67,
    VSPICS0                  = 68,
    VSPICS1                  = 69,
    VSPICS2                  = 70,
    LEDC_HS_SIG0             = 71,
    LEDC_HS_SIG1             = 72,
    LEDC_HS_SIG2             = 73,
    LEDC_HS_SIG3             = 74,
    LEDC_HS_SIG4             = 75,
    LEDC_HS_SIG5             = 76,
    LEDC_HS_SIG6             = 77,
    LEDC_HS_SIG7             = 78,
    LEDC_LS_SIG0             = 79,
    LEDC_LS_SIG1             = 80,
    LEDC_LS_SIG2             = 81,
    LEDC_LS_SIG3             = 82,
    LEDC_LS_SIG4             = 83,
    LEDC_LS_SIG5             = 84,
    LEDC_LS_SIG6             = 85,
    LEDC_LS_SIG7             = 86,
    RMT_SIG_0                = 87,
    RMT_SIG_1                = 88,
    RMT_SIG_2                = 89,
    RMT_SIG_3                = 90,
    RMT_SIG_4                = 91,
    RMT_SIG_5                = 92,
    RMT_SIG_6                = 93,
    RMT_SIG_7                = 94,
    I2CEXT1_SCL              = 95,
    I2CEXT1_SDA              = 96,
    HOST_CCMD_OD_PULLUP_EN_N = 97,
    HOST_RST_N_1             = 98,
    HOST_RST_N_2             = 99,
    GPIO_SD0                 = 100,
    GPIO_SD1                 = 101,
    GPIO_SD2                 = 102,
    GPIO_SD3                 = 103,
    GPIO_SD4                 = 104,
    GPIO_SD5                 = 105,
    GPIO_SD6                 = 106,
    GPIO_SD7                 = 107,
    PWM1_0A                  = 108,
    PWM1_0B                  = 109,
    PWM1_1A                  = 110,
    PWM1_1B                  = 111,
    PWM1_2A                  = 112,
    PWM1_2B                  = 113,
    TWAI_TX                  = 123,
    TWAI_BUS_OFF_ON          = 124,
    TWAI_CLKOUT              = 125,
    I2S0O_DATA_0             = 140,
    I2S0O_DATA_1             = 141,
    I2S0O_DATA_2             = 142,
    I2S0O_DATA_3             = 143,
    I2S0O_DATA_4             = 144,
    I2S0O_DATA_5             = 145,
    I2S0O_DATA_6             = 146,
    I2S0O_DATA_7             = 147,
    I2S0O_DATA_8             = 148,
    I2S0O_DATA_9             = 149,
    I2S0O_DATA_10            = 150,
    I2S0O_DATA_11            = 151,
    I2S0O_DATA_12            = 152,
    I2S0O_DATA_13            = 153,
    I2S0O_DATA_14            = 154,
    I2S0O_DATA_15            = 155,
    I2S0O_DATA_16            = 156,
    I2S0O_DATA_17            = 157,
    I2S0O_DATA_18            = 158,
    I2S0O_DATA_19            = 159,
    I2S0O_DATA_20            = 160,
    I2S0O_DATA_21            = 161,
    I2S0O_DATA_22            = 162,
    I2S0O_DATA_23            = 163,
    I2S1I_BCK                = 164,
    I2S1I_WS                 = 165,
    I2S1O_DATA_0             = 166,
    I2S1O_DATA_1             = 167,
    I2S1O_DATA_2             = 168,
    I2S1O_DATA_3             = 169,
    I2S1O_DATA_4             = 170,
    I2S1O_DATA_5             = 171,
    I2S1O_DATA_6             = 172,
    I2S1O_DATA_7             = 173,
    I2S1O_DATA_8             = 174,
    I2S1O_DATA_9             = 175,
    I2S1O_DATA_10            = 176,
    I2S1O_DATA_11            = 177,
    I2S1O_DATA_12            = 178,
    I2S1O_DATA_13            = 179,
    I2S1O_DATA_14            = 180,
    I2S1O_DATA_15            = 181,
    I2S1O_DATA_16            = 182,
    I2S1O_DATA_17            = 183,
    I2S1O_DATA_18            = 184,
    I2S1O_DATA_19            = 185,
    I2S1O_DATA_20            = 186,
    I2S1O_DATA_21            = 187,
    I2S1O_DATA_22            = 188,
    I2S1O_DATA_23            = 189,
    U2TXD                    = 198,
    U2RTS                    = 199,
    EMAC_MDC                 = 200,
    EMAC_MDO                 = 201,
    EMAC_CRS                 = 202,
    EMAC_COL                 = 203,
    BT_AUDIO0RQ              = 204,
    BT_AUDIO1RQ              = 205,
    BT_AUDIO2RQ              = 206,
    BLE_AUDIO0RQ             = 207,
    BLE_AUDIO1RQ             = 208,
    BLE_AUDIO2RQ             = 209,
    PCMFSYNC                 = 210,
    PCMCLK                   = 211,
    PCMDOUT                  = 212,
    BLE_AUDIO_SYNC0_P        = 213,
    BLE_AUDIO_SYNC1_P        = 214,
    BLE_AUDIO_SYNC2_P        = 215,
    ANT_SEL0                 = 216,
    ANT_SEL1                 = 217,
    ANT_SEL2                 = 218,
    ANT_SEL3                 = 219,
    ANT_SEL4                 = 220,
    ANT_SEL5                 = 221,
    ANT_SEL6                 = 222,
    ANT_SEL7                 = 223,
    SIGNAL_224               = 224,
    SIGNAL_225               = 225,
    SIGNAL_226               = 226,
    SIGNAL_227               = 227,
    SIGNAL_228               = 228,
    GPIO                     = 256,

    CLK_OUT1                 = 512,
    CLK_OUT2,
    CLK_OUT3,
    SD_CLK,
    SD_CMD,
    SD_DATA0,
    SD_DATA1,
    SD_DATA2,
    SD_DATA3,
    HS1_CLK,
    HS1_CMD,
    HS1_DATA0,
    HS1_DATA1,
    HS1_DATA2,
    HS1_DATA3,
    HS1_DATA4,
    HS1_DATA5,
    HS1_DATA6,
    HS1_DATA7,
    HS1_STROBE,
    HS2_CLK,
    HS2_CMD,
    HS2_DATA0,
    HS2_DATA1,
    HS2_DATA2,
    HS2_DATA3,

    EMAC_TX_CLK,
    EMAC_TX_ER,
    EMAC_TXD3,
    EMAC_RX_ER,
    EMAC_TXD2,
    EMAC_CLK_OUT,
    EMAC_CLK_180,
    EMAC_TXD0,
    EMAC_TX_EN,
    EMAC_TXD1,

    MTDO,
}

macro_rules! rtcio_analog {
    (
        $pin_num:expr, $rtc_pin:expr, $pin_reg:expr, $prefix:pat, $hold:ident $(, $rue:literal)?
    ) => {
        paste::paste! {
            impl $crate::gpio::RtcPin for $crate::peripherals::[<GPIO $pin_num>]<'_> {
                fn rtc_number(&self) -> u8 {
                    $rtc_pin
                }

                /// Set the RTC properties of the pin. If `mux` is true then then pin is
                /// routed to RTC, when false it is routed to IO_MUX.
                fn rtc_set_config(&self, input_enable: bool, mux: bool, func: $crate::gpio::RtcFunction) {
                    // disable input
                    $crate::peripherals::RTC_IO::regs()
                        .$pin_reg.modify(|_,w| unsafe {
                            w.[<$prefix fun_ie>]().bit(input_enable);
                            w.[<$prefix mux_sel>]().bit(mux);
                            w.[<$prefix fun_sel>]().bits(func as u8)
                        });
                }

                fn rtcio_pad_hold(&self, enable: bool) {
                    $crate::peripherals::LPWR::regs()
                        .hold_force()
                        .modify(|_, w| w.$hold().bit(enable));
                }
            }

            $(
                // FIXME: replace with $(ignore($rue)) once stable
                $crate::ignore!($rue);
                impl $crate::gpio::RtcPinWithResistors for $crate::peripherals::[<GPIO $pin_num>]<'_> {
                    fn rtcio_pullup(&self, enable: bool) {
                        $crate::peripherals::RTC_IO::regs()
                            .$pin_reg.modify(|_, w| w.[< $prefix rue >]().bit(enable));
                    }

                    fn rtcio_pulldown(&self, enable: bool) {
                        $crate::peripherals::RTC_IO::regs()
                            .$pin_reg.modify(|_, w| w.[< $prefix rde >]().bit(enable));
                    }
                }
            )?

            impl $crate::gpio::AnalogPin for $crate::peripherals::[<GPIO $pin_num>]<'_> {
                /// Configures the pin for analog mode.
                fn set_analog(&self, _: $crate::private::Internal) {
                    use $crate::gpio::RtcPin;
                    let rtcio = $crate::peripherals::RTC_IO::regs();

                    // disable output
                    rtcio.enable_w1tc().write(|w| unsafe { w.enable_w1tc().bits(1 << self.rtc_number()) });

                    // disable open drain
                    rtcio.pin(self.rtc_number() as usize).modify(|_,w| w.pad_driver().bit(false));

                    rtcio.$pin_reg.modify(|_,w| {
                        w.[<$prefix fun_ie>]().clear_bit();

                        // Connect pin to analog / RTC module instead of standard GPIO
                        w.[<$prefix mux_sel>]().set_bit();

                        // Select function "RTC function 1" (GPIO) for analog use
                        unsafe { w.[<$prefix fun_sel>]().bits(0b00) };

                        // Disable pull-up and pull-down resistors on the pin, if it has them
                        $(
                            // FIXME: replace with $(ignore($rue)) once stable
                            $crate::ignore!($rue);
                            w.[<$prefix rue>]().bit(false);
                            w.[<$prefix rde>]().bit(false);
                        )?

                        w
                    });
                }
            }
        }
    };

    (
        $( ( $pin_num:expr, $rtc_pin:expr, $pin_reg:expr, $prefix:pat, $hold:ident $(, $rue:literal )? ) )+
    ) => {
        $(
            rtcio_analog!($pin_num, $rtc_pin, $pin_reg, $prefix, $hold $(, $rue )?);
        )+

        pub(crate) fn errata36(pin: $crate::gpio::AnyPin<'_>, pull_up: bool, pull_down: bool) {
            use $crate::gpio::{Pin, RtcPinWithResistors};

            let has_pullups = match pin.number() {
                $(
                    $( $pin_num => $rue, )?
                )+
                _ => false,
            };

            if has_pullups {
                pin.rtcio_pullup(pull_up);
                pin.rtcio_pulldown(pull_down);
            }
        }
    };
}

/// Common functionality for all touch pads
macro_rules! touch {
    (@pin_specific $touch_num:expr, true) => {
        paste::paste! {
            RTC_IO::regs().[< touch_pad $touch_num >]().write(|w| unsafe {
                w.xpd().set_bit();
                // clear input_enable
                w.fun_ie().clear_bit();
                // Connect pin to analog / RTC module instead of standard GPIO
                w.mux_sel().set_bit();
                // Disable pull-up and pull-down resistors on the pin
                w.rue().clear_bit();
                w.rde().clear_bit();
                w.tie_opt().clear_bit();
                // Select function "RTC function 1" (GPIO) for analog use
                w.fun_sel().bits(0b00)
            });
        }
    };

    (@pin_specific $touch_num:expr, false) => {
        paste::paste! {
            RTC_IO::regs().[< touch_pad $touch_num >]().write(|w| {
                w.xpd().set_bit();
                w.tie_opt().clear_bit()
            });
        }
    };

    (
        $(
            (
                $touch_num:literal, $pin_num:literal, $touch_out_reg:expr, $touch_thres_reg:expr, $normal_pin:literal
            )
        )+
    ) => {
        $(
        impl $crate::gpio::TouchPin for paste::paste!($crate::peripherals::[<GPIO $pin_num>]<'_>) {
            fn set_touch(&self, _: $crate::private::Internal) {
                use $crate::peripherals::{GPIO, RTC_IO, SENS};
                use $crate::gpio::RtcPin;

                let gpio = GPIO::regs();
                let rtcio = RTC_IO::regs();
                let sens = SENS::regs();

                // Pad to normal mode (not open-drain)
                gpio.pin(self.rtc_number() as usize).write(|w| w.pad_driver().clear_bit());

                // clear output
                rtcio
                    .enable_w1tc()
                    .write(|w| unsafe { w.enable_w1tc().bits(1 << self.rtc_number()) });
                paste::paste! {
                    sens . $touch_thres_reg ()
                        .write(|w| unsafe {
                            w. [<touch_out_th $touch_num>] ().bits(
                                0b0 // Default: 0 for esp32 gets overridden later anyway.
                            )
                        });

                    touch!( @pin_specific $touch_num, $normal_pin );

                    // enable the pin
                    sens.sar_touch_enable().modify(|r, w| unsafe {
                        w.touch_pad_worken().bits(
                            r.touch_pad_worken().bits() | ( 1 << $touch_num )
                        )
                    });
                }
            }

            fn touch_measurement(&self, _: $crate::private::Internal) -> u16 {
                paste::paste! {
                    $crate::peripherals::SENS::regs() . $touch_out_reg ().read()
                        . [<touch_meas_out $touch_num>] ().bits()
                }
            }

            fn touch_nr(&self, _: $crate::private::Internal) -> u8 {
                $touch_num
            }

            fn set_threshold(&self, threshold: u16, _: $crate::private::Internal) {
                paste::paste! {
                    $crate::peripherals::SENS::regs() . $touch_thres_reg ()
                        .write(|w| unsafe {
                            w. [<touch_out_th $touch_num>] ().bits(threshold)
                        });
                }
            }
        })+
    };
}

rtcio_analog! {
    (36, 0,  sensor_pads(),    sense1_, sense1          )
    (37, 1,  sensor_pads(),    sense2_, sense2          )
    (38, 2,  sensor_pads(),    sense3_, sense3          )
    (39, 3,  sensor_pads(),    sense4_, sense4          )
    (34, 4,  adc_pad(),        adc1_,   adc1            )
    (35, 5,  adc_pad(),        adc2_,   adc2            )
    (25, 6,  pad_dac1(),       "",      pdac1,      true)
    (26, 7,  pad_dac2(),       "",      pdac2,      true)
    (33, 8,  xtal_32k_pad(),   x32n_,   x32n,       true)
    (32, 9,  xtal_32k_pad(),   x32p_,   x32p,       true)
    (4,  10, touch_pad0(),     "",      touch_pad0, true)
    (0,  11, touch_pad1(),     "",      touch_pad1, true)
    (2,  12, touch_pad2(),     "",      touch_pad2, true)
    (15, 13, touch_pad3(),     "",      touch_pad3, true)
    (13, 14, touch_pad4(),     "",      touch_pad4, true)
    (12, 15, touch_pad5(),     "",      touch_pad5, true)
    (14, 16, touch_pad6(),     "",      touch_pad6, true)
    (27, 17, touch_pad7(),     "",      touch_pad7, true)
}

touch! {
    // touch_nr, pin_nr, touch_out_reg, touch_thres_reg, normal_pin
    (0, 4,  sar_touch_out1, sar_touch_thres1, true)
    (1, 0,  sar_touch_out1, sar_touch_thres1, true)
    (2, 2,  sar_touch_out2, sar_touch_thres2, true)
    (3, 15, sar_touch_out2, sar_touch_thres2, true)
    (4, 13, sar_touch_out3, sar_touch_thres3, true)
    (5, 12, sar_touch_out3, sar_touch_thres3, true)
    (6, 14, sar_touch_out4, sar_touch_thres4, true)
    (7, 27, sar_touch_out4, sar_touch_thres4, true)
    // ---
    (8, 33, sar_touch_out5, sar_touch_thres5, false)
    (9, 32, sar_touch_out5, sar_touch_thres5, false)
}

#[derive(Clone, Copy)]
pub(crate) enum InterruptStatusRegisterAccess {
    Bank0,
    Bank1,
}

impl InterruptStatusRegisterAccess {
    pub(crate) fn interrupt_status_read(self) -> u32 {
        match self {
            Self::Bank0 => GPIO::regs().status().read().bits(),
            Self::Bank1 => GPIO::regs().status1().read().bits(),
        }
    }
}
