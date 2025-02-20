//! General Purpose Input/Output
use core::marker::PhantomData;

use crate::pac;

/// Extension trait to split GLB peripheral into independent pins, registers and other modules
pub trait GlbExt {
    /// Splits the register block into independent pins and modules
    fn split(self) -> Parts;
}

#[derive(Copy, Clone)]
pub enum Event {
    /// Trigger on the falling edge
    NegativePulse = 0,
    /// Trigger on the rising edge
    PositivePulse = 1,
    /// Trigger while low level
    NegativeLevel = 2,
    /// Trigger while high level
    HighLevel = 3,
}

/// Extension trait to setup/enable/disable/clear/check input pins
pub trait InterruptPin {
    // Is make_interrupt_source redundant?
    //fn make_interrupt_source(&mut self, afio: &mut afio::Parts);
    fn trigger_on_event(&mut self, event: Event);
    fn control_asynchronous(&mut self);
    fn control_synchronous(&mut self);
    fn enable_interrupt(&mut self);
    fn disable_interrupt(&mut self);
    fn clear_interrupt_pending_bit(&mut self);
    fn check_interrupt(&self) -> bool;
}

pub use uart_sig::*;

/// UART signals
pub mod uart_sig {
    use core::marker::PhantomData;

    use crate::pac;

    /// UART0 RTS (type state)
    pub struct Uart0Rts;

    /// UART0 CTS (type state)
    pub struct Uart0Cts;

    /// UART0 TXD (type state)
    pub struct Uart0Tx;

    /// UART0 RXD (type state)
    pub struct Uart0Rx;

    /// UART1 RXD (type state)
    pub struct Uart1Rx;

    /// UART1 RTS (type state)
    pub struct Uart1Rts;

    /// UART1 CTS (type state)
    pub struct Uart1Cts;

    /// UART1 TXD (type state)
    pub struct Uart1Tx;

    macro_rules! impl_uart_sig {
        ($UartSigi: ident, $doc1: expr, $sigi:ident, $UartMuxi: ident, $doc2: expr) => {
            #[doc = $doc1]
            pub struct $UartSigi;

            #[doc = $doc2]
            pub struct $UartMuxi<MODE> {
                pub(crate) _mode: PhantomData<MODE>,
            }

            impl<MODE> $UartMuxi<MODE> {
                /// Configure the internal UART signal to UART0-RTS
                pub fn into_uart0_rts(self) -> $UartMuxi<Uart0Rts> {
                    self.into_uart_mode(0)
                }

                /// Configure the internal UART signal to UART0-CTS
                pub fn into_uart0_cts(self) -> $UartMuxi<Uart0Cts> {
                    self.into_uart_mode(1)
                }

                /// Configure the internal UART signal to UART0-TX
                pub fn into_uart0_tx(self) -> $UartMuxi<Uart0Tx> {
                    self.into_uart_mode(2)
                }

                /// Configure the internal UART signal to UART0-RX
                pub fn into_uart0_rx(self) -> $UartMuxi<Uart0Rx> {
                    self.into_uart_mode(3)
                }

                /// Configure the internal UART signal to UART1-RTS
                pub fn into_uart1_rts(self) -> $UartMuxi<Uart1Rts> {
                    self.into_uart_mode(4)
                }

                /// Configure the internal UART signal to UART1-CTS
                pub fn into_uart1_cts(self) -> $UartMuxi<Uart1Cts> {
                    self.into_uart_mode(5)
                }

                /// Configure the internal UART signal to UART1-TX
                pub fn into_uart1_tx(self) -> $UartMuxi<Uart1Tx> {
                    self.into_uart_mode(6)
                }

                /// Configure the internal UART signal to UART1-RX
                pub fn into_uart1_rx(self) -> $UartMuxi<Uart1Rx> {
                    self.into_uart_mode(7)
                }

                paste::paste! {
                    #[inline]
                    fn into_uart_mode<T>(self, mode: u8) -> $UartMuxi<T> {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.uart_sig_sel_0.modify(|_r, w| unsafe { w
                            .[<uart_ $sigi _sel>]().bits(mode)
                        });

                        $UartMuxi { _mode: PhantomData }
                    }
                }
            }
        };
    }

    impl_uart_sig!(
        UartSig0,
        "UART signal 0 (type state)",
        sig_0,
        UartMux0,
        "UART multiplexer peripherals for signal 0"
    );

    impl_uart_sig!(
        UartSig1,
        "UART signal 1 (type state)",
        sig_1,
        UartMux1,
        "UART multiplexer peripherals for signal 1"
    );

    impl_uart_sig!(
        UartSig2,
        "UART signal 2 (type state)",
        sig_2,
        UartMux2,
        "UART multiplexer peripherals for signal 2"
    );

    impl_uart_sig!(
        UartSig3,
        "UART signal 3 (type state)",
        sig_3,
        UartMux3,
        "UART multiplexer peripherals for signal 3"
    );

    impl_uart_sig!(
        UartSig4,
        "UART signal 4 (type state)",
        sig_4,
        UartMux4,
        "UART multiplexer peripherals for signal 4"
    );

    impl_uart_sig!(
        UartSig5,
        "UART signal 5 (type state)",
        sig_5,
        UartMux5,
        "UART multiplexer peripherals for signal 5"
    );

    impl_uart_sig!(
        UartSig6,
        "UART signal 6 (type state)",
        sig_6,
        UartMux6,
        "UART multiplexer peripherals for signal 6"
    );

    impl_uart_sig!(
        UartSig7,
        "UART signal 7 (type state)",
        sig_7,
        UartMux7,
        "UART multiplexer peripherals for signal 7"
    );
}

/// Clock configurator registers
pub struct ClkCfg {
    pub(crate) _ownership: (),
}

/*
// todo: english
    在GPIO模式下，可以设置内部上下拉，以类型状态机模式设计
    SPI、UART、I2C等数字功能下，可以设置内部上下拉，但不会影响返回类型的状态
    ADC、DAC下，软件禁止设置内部上下拉。HAL库不会生成此类函数，以免出错。
*/

/// Hi-Z Floating pin (type state)
pub struct Floating;
/// Pulled down pin (type state)
pub struct PullDown;
/// Pulled up pin (type state)
pub struct PullUp;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// PWM mode (type state)
pub struct Pwm<MODE> {
    _mode: PhantomData<MODE>,
}

/// UART pin mode (type state)
pub struct Uart;

/// SPI pin mode (type state)
pub struct Spi;

/// I2C pin mode (type state)
pub struct I2c;

#[doc(hidden)]
pub trait UartPin<SIG> {}

// There are Pin0 to Pin22, totally 23 pins

pub use self::pin::*;

macro_rules! impl_glb {
    ($($Pini: ident: ($pini: ident, $gpio_cfgctli: ident, $UartSigi: ident, $sigi: ident, $spi_kind: ident, $i2c_kind: ident, $gpio_i: ident, $gpio_int_mode_seti: ident) ,)+) => {
        impl GlbExt for pac::GLB {
            fn split(self) -> Parts {
                Parts {
                    $( $pini: $Pini { _mode: PhantomData }, )+
                    uart_mux0: UartMux0 { _mode: PhantomData },
                    uart_mux1: UartMux1 { _mode: PhantomData },
                    uart_mux2: UartMux2 { _mode: PhantomData },
                    uart_mux3: UartMux3 { _mode: PhantomData },
                    uart_mux4: UartMux4 { _mode: PhantomData },
                    uart_mux5: UartMux5 { _mode: PhantomData },
                    uart_mux6: UartMux6 { _mode: PhantomData },
                    uart_mux7: UartMux7 { _mode: PhantomData },
                    clk_cfg: ClkCfg { _ownership: () },
                }
            }
        }

        /// GPIO parts
        pub struct Parts {
            $( pub $pini: $Pini<Input<Floating>>, )+
            pub uart_mux0: UartMux0<Uart0Cts>,
            pub uart_mux1: UartMux1<Uart0Cts>,
            pub uart_mux2: UartMux2<Uart0Cts>,
            pub uart_mux3: UartMux3<Uart0Cts>,
            pub uart_mux4: UartMux4<Uart0Cts>,
            pub uart_mux5: UartMux5<Uart0Cts>,
            pub uart_mux6: UartMux6<Uart0Cts>,
            pub uart_mux7: UartMux7<Uart0Cts>,
            pub clk_cfg: ClkCfg,
        }

        /// GPIO pins
        pub mod pin {
            use core::marker::PhantomData;
            use core::convert::Infallible;
            use embedded_hal::digital::blocking::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
            use embedded_hal_zero::digital::v2::{
                InputPin as InputPinZero,
                OutputPin as OutputPinZero,
                StatefulOutputPin as StatefulOutputPinZero,
                ToggleableOutputPin as ToggleableOutputPinZero
            };
            use crate::pac;
            use super::*;

            /// Simple implementation of InputPin trait to use within EH0 and EH1 impls without name conflicts
            trait InternalInputPinImpl {
                fn is_high_inner(&self) -> bool;
                fn is_low_inner(&self) -> bool;
            }

            /// Simple implementation of OutputPin trait to use within EH0 and EH1 impls without name conflicts
            trait InternalOutputPinImp {
                fn set_high_inner(&self);
                fn set_low_inner(&self);
            }

            /// Simple implementation of StatefulOutputPin trait to use within EH0 and EH1 impls without name conflicts
            trait InternalStatefulOutputImp {
                fn is_output_high_inner(&self) -> bool;
                fn is_output_low_inner(&self) -> bool;
            }

            $(
            /// Pin
            pub struct $Pini<MODE> {
                pub(crate) _mode: PhantomData<MODE>,
            }

            impl<MODE> $Pini<MODE> {
                // 11 -> GPIO_FUN_SWGPIO
                /// Configures the pin to operate as a Hi-Z floating output pin.
                pub fn into_floating_output(self) -> $Pini<Output<Floating>> {
                    self.into_pin_with_mode(11, false, false, false)
                }

                /// Configures the pin to operate as a pull-up output pin.
                pub fn into_pull_up_output(self) -> $Pini<Output<PullUp>> {
                    self.into_pin_with_mode(11, true, false, false)
                }

                /// Configures the pin to operate as a pull-down output pin.
                pub fn into_pull_down_output(self) -> $Pini<Output<PullDown>> {
                    self.into_pin_with_mode(11, false, true, false)
                }

                /// Configures the pin to operate as a Hi-Z floating input pin.
                pub fn into_floating_input(self) -> $Pini<Input<Floating>> {
                    self.into_pin_with_mode(11, false, false, true)
                }

                /// Configures the pin to operate as a pull-up input pin.
                pub fn into_pull_up_input(self) -> $Pini<Input<PullUp>> {
                    self.into_pin_with_mode(11, true, false, true)
                }

                /// Configures the pin to operate as a pull-down input pin.
                pub fn into_pull_down_input(self) -> $Pini<Input<PullDown>> {
                    self.into_pin_with_mode(11, false, true, true)
                }

                // Do pull-up/pull-down/hi-z actually do anything for PWM?

                /// Configures the pin to operate as a pull-down PWM pin.
                pub fn into_pull_down_pwm(self) -> $Pini<Pwm<PullDown>> {
                    // `bl602_pac::glb::gpio_cfgctln::REG_GPIO_n_FUNC_SEL_A::PWM_CHn`
                    self.into_pin_with_mode(8, false, true, true)
                }

                /// Configures the pin to operate as a pull-up PWM pin.
                pub fn into_pull_up_pwm(self) -> $Pini<Pwm<PullUp>> {
                    self.into_pin_with_mode(8, false, true, true)
                }

                /// Configures the pin to operate as a Hi-Z floating PWM pin.
                pub fn into_floating_pwm(self) -> $Pini<Pwm<Floating>> {
                    self.into_pin_with_mode(8, false, false, true)
                }

                paste::paste! {
                    #[inline]
                    fn into_pin_with_mode<T>(self, mode: u8, pu: bool, pd: bool, ie: bool) -> $Pini<T> {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_cfgctli.modify(|_r, w| unsafe { w
                            .[<reg_ $gpio_i _func_sel>]().bits(mode)
                            .[<reg_ $gpio_i _ie>]().bit(ie) // output
                            .[<reg_ $gpio_i _pu>]().bit(pu)
                            .[<reg_ $gpio_i _pd>]().bit(pd)
                            .[<reg_ $gpio_i _drv>]().bits(0) // disabled
                            .[<reg_ $gpio_i _smt>]().clear_bit()
                        });

                        // If we're an input clear the Output Enable bit as well, else set it.
                        glb.gpio_cfgctl34.modify(|_, w| w.[<reg_ $gpio_i _oe>]().bit(!ie));

                        $Pini { _mode: PhantomData }
                    }
                }
            }

            impl<MODE> $Pini<Input<MODE>> {
                paste::paste! {
                    /// Enable smitter GPIO input filter
                    pub fn enable_smitter(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_cfgctli.modify(|_, w| w.[<reg_ $gpio_i _smt>]().set_bit());
                    }

                    /// Enable smitter GPIO output filter
                    pub fn disable_smitter(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_cfgctli.modify(|_, w| w.[<reg_ $gpio_i _smt>]().clear_bit());
                    }
                }
            }

            impl<MODE> $Pini<MODE> {
                paste::paste! {
                    /// Configures the pin to UART alternate mode
                    pub fn [<into_uart_ $sigi>](self) -> $Pini<Uart> {
                        // 7 -> GPIO_FUN_UART
                        self.into_pin_with_mode(7, true, false, true)
                    }

                    /// Configures the pin to SPI alternate mode
                    pub fn [<into_spi_ $spi_kind>](self) -> $Pini<Spi> {
                        // 4 -> GPIO0_FUN_SPI_x
                        self.into_pin_with_mode(4, true, false, true)
                    }

                    /// Configures the pin to I2C alternate mode
                    pub fn [<into_i2c_ $i2c_kind>](self) -> $Pini<I2c> {
                        // 6 -> GPIO_FUN_I2C_x
                        self.into_pin_with_mode(6, true, false, true)
                    }
                }
            }

            impl UartPin<$UartSigi> for $Pini<Uart> {}

            impl<MODE> InternalInputPinImpl for $Pini<Input<MODE>> {
                paste::paste! {
                    fn is_high_inner(&self) -> bool {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl30.read().[<reg_ $gpio_i _i>]().bit_is_set()
                    }
                }
                paste::paste! {
                    fn is_low_inner(&self) -> bool {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl30.read().[<reg_ $gpio_i _i>]().bit_is_clear()
                    }
                }
            }

            impl<MODE> InternalOutputPinImp for $Pini<Output<MODE>> {
                paste::paste! {
                    fn set_high_inner(&self) {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl32.modify(|_, w| w.[<reg_ $gpio_i _o>]().set_bit())
                    }
                }
                paste::paste! {
                    fn set_low_inner(&self)  {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl32.modify(|_, w| w.[<reg_ $gpio_i _o>]().clear_bit())
                    }
                }
            }

            impl<MODE> InternalStatefulOutputImp for $Pini<Output<MODE>> {
                paste::paste! {
                    fn is_output_high_inner(&self) -> bool {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl32.read().[<reg_ $gpio_i _o>]().bit_is_set()
                    }

                    fn is_output_low_inner(& self) -> bool {
                        let glb = unsafe { &*pac::GLB::ptr() };
                        glb.gpio_cfgctl32.read().[<reg_ $gpio_i _o>]().bit_is_clear()
                    }
                }
            }


            impl<MODE> InputPin for $Pini<Input<MODE>> {
                type Error = Infallible;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_high_inner())
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_low_inner())
                }
            }

            impl<MODE> InputPinZero for $Pini<Input<MODE>> {
                type Error = Infallible;

                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_high_inner())
                }

                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_low_inner())
                }
            }

            impl<MODE> InterruptPin for $Pini<Input<MODE>> {

                paste::paste! {
                    fn trigger_on_event(&mut self, event: Event) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_int_mode_seti.modify(|_, w| { w
                                                                .[<reg_ $gpio_i _interrupt_trigger_mode>]().bits(event as u8)
                        });
                    }

                    fn control_asynchronous(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_int_mode_seti.modify(|_, w| { w
                                                                .[<reg_ $gpio_i _interrupt_control_mode>]().asynchronous()
                        });
                    }

                    fn control_synchronous(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.$gpio_int_mode_seti.modify(|_, w| { w
                                                                .[<reg_ $gpio_i _interrupt_control_mode>]().synchronous()
                        });
                    }

                    fn enable_interrupt(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.gpio_int_mask1.modify(|_, w| { w
                                                           .[<reg_ $gpio_i _mask>]().unmasked()
                        });
                    }

                    fn disable_interrupt(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.gpio_int_mask1.modify(|_, w| { w
                                                           .[<reg_ $gpio_i _mask>]().masked()
                        });
                    }

                    fn clear_interrupt_pending_bit(&mut self) {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.gpio_int_clr1.modify(|_, w| { w
                                                          .[<reg_ $gpio_i _interrupt_clear>]().clear_bit()
                        });
                    }

                    fn check_interrupt(&self) -> bool {
                        let glb = unsafe { &*pac::GLB::ptr() };

                        glb.gpio_int_stat1.read().[<reg_ $gpio_i _interrupt_status>]().is_set()
                    }
                }
            }


            impl<MODE> OutputPin for $Pini<Output<MODE>> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    self.set_high_inner();
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    self.set_low_inner();
                    Ok(())
                }
            }

            impl<MODE> OutputPinZero for $Pini<Output<MODE>> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    self.set_high_inner();
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    self.set_low_inner();
                    Ok(())
                }
            }

            impl<MODE> StatefulOutputPin for $Pini<Output<MODE>> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_output_high_inner())
                }

                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_output_low_inner())
                }
            }

            impl<MODE> StatefulOutputPinZero for $Pini<Output<MODE>> {
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_output_high_inner())
                }

                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    Ok(self.is_output_low_inner())
                }
            }


            impl<MODE> ToggleableOutputPin for $Pini<Output<MODE>> {
                type Error = Infallible;

                fn toggle(&mut self) -> Result<(), Self::Error> {
                    if self.is_output_high_inner() {
                        self.set_low_inner()
                    } else {
                        self.set_high_inner()
                    }
                    Ok(())
                }
            }

            impl<MODE> ToggleableOutputPinZero for $Pini<Output<MODE>> {
                type Error = Infallible;

                fn toggle(&mut self) -> Result<(), Self::Error> {
                    if self.is_output_high_inner() {
                        self.set_low_inner()
                    } else {
                        self.set_high_inner()
                    }
                    Ok(())
                }
            }

            )+
        }
    };
}

// There are Pin0 to Pin22, totally 23 pins
// todo: generate macros
impl_glb! {
    Pin0: (pin0, gpio_cfgctl0, UartSig0, sig0, miso, scl, gpio_0, gpio_int_mode_set1),
    Pin1: (pin1, gpio_cfgctl0, UartSig1, sig1, mosi, sda, gpio_1, gpio_int_mode_set1),
    Pin2: (pin2, gpio_cfgctl1, UartSig2, sig2, ss, scl, gpio_2, gpio_int_mode_set1),
    Pin3: (pin3, gpio_cfgctl1, UartSig3, sig3, sclk, sda, gpio_3, gpio_int_mode_set1),
    Pin4: (pin4, gpio_cfgctl2, UartSig4, sig4, miso, scl, gpio_4, gpio_int_mode_set1),
    Pin5: (pin5, gpio_cfgctl2, UartSig5, sig5, mosi, sda, gpio_5, gpio_int_mode_set1),
    Pin6: (pin6, gpio_cfgctl3, UartSig6, sig6, ss, scl, gpio_6, gpio_int_mode_set1),
    Pin7: (pin7, gpio_cfgctl3, UartSig7, sig7, sclk, sda, gpio_7, gpio_int_mode_set1),
    Pin8: (pin8, gpio_cfgctl4, UartSig0, sig0, miso, scl, gpio_8, gpio_int_mode_set1),
    Pin9: (pin9, gpio_cfgctl4, UartSig1, sig1, mosi, sda, gpio_9, gpio_int_mode_set1),
    Pin10: (pin10, gpio_cfgctl5, UartSig2, sig2, ss, scl, gpio_10, gpio_int_mode_set2),
    Pin11: (pin11, gpio_cfgctl5, UartSig3, sig3, sclk, sda, gpio_11, gpio_int_mode_set2),
    Pin12: (pin12, gpio_cfgctl6, UartSig4, sig4, miso, scl, gpio_12, gpio_int_mode_set2),
    Pin13: (pin13, gpio_cfgctl6, UartSig5, sig5, mosi, sda, gpio_13, gpio_int_mode_set2),
    Pin14: (pin14, gpio_cfgctl7, UartSig6, sig6, ss, scl, gpio_14, gpio_int_mode_set2),
    Pin15: (pin15, gpio_cfgctl7, UartSig7, sig7, sclk, sda, gpio_15, gpio_int_mode_set2),
    Pin16: (pin16, gpio_cfgctl8, UartSig0, sig0, miso, scl, gpio_16, gpio_int_mode_set2),
    Pin17: (pin17, gpio_cfgctl8, UartSig1, sig1, mosi, sda, gpio_17, gpio_int_mode_set2),
    Pin18: (pin18, gpio_cfgctl9, UartSig2, sig2, ss, scl, gpio_18, gpio_int_mode_set2),
    Pin19: (pin19, gpio_cfgctl9, UartSig3, sig3, sclk, sda, gpio_19, gpio_int_mode_set2),
    Pin20: (pin20, gpio_cfgctl10, UartSig4, sig4, miso, scl, gpio_20, gpio_int_mode_set3),
    Pin21: (pin21, gpio_cfgctl10, UartSig5, sig5, mosi, sda, gpio_21, gpio_int_mode_set3),
    Pin22: (pin22, gpio_cfgctl11, UartSig6, sig6, ss, scl, gpio_22, gpio_int_mode_set3),
}
