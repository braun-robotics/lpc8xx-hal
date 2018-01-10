//! APIs for the switch matrix (SWM)
//!
//! See user manual, chapter 7.


use lpc82x;
use lpc82x::swm::pinenable0;

use gpio::PinName;
use init_state::{
    self,
    InitState,
};
use syscon;


/// Interface to the switch matrix (SWM)
///
/// This API expects to be the sole owner of the SWM peripheral. Don't use
/// [`lpc82x::SWM`] directly, unless you know what you're doing.
///
/// [`lpc82x::SWM`]: ../../lpc82x/struct.SWM.html
pub struct SWM<'swm> {
    /// Main SWM API
    pub api: Api<'swm, init_state::Unknown>,

    /// Movable functions
    pub movable_functions: MovableFunctions,
}

impl<'swm> SWM<'swm> {
    pub(crate) fn new(swm: &'swm lpc82x::SWM) -> Self {
        SWM {
            api              : Api::new(swm),
            movable_functions: MovableFunctions::new(),
        }
    }
}


/// Main API of the SWM peripheral
pub struct Api<'swm, State: InitState = init_state::Initialized> {
    swm   : &'swm lpc82x::SWM,
    _state: State,
}

impl<'swm> Api<'swm, init_state::Unknown> {
    pub(crate) fn new(swm: &'swm lpc82x::SWM) -> Self {
        Api {
            swm   : swm,
            _state: init_state::Unknown,
        }
    }

    /// Initialize the switch matrix
    pub fn init(mut self, syscon: &mut syscon::Api)
        -> Api<'swm, init_state::Initialized>
    {
        syscon.enable_clock(&mut self.swm);

        Api {
            swm   : self.swm,
            _state: init_state::Initialized,
        }
    }
}

impl<'swm> Api<'swm> {
    /// Enables a fixed function
    ///
    /// # Limitations
    ///
    /// The fixed function can be enabled on a pin that is currently used for
    /// something else. The HAL user needs to make sure that this assignment
    /// doesn't conflict with any other uses of the pin.
    pub fn enable_fixed_function<F: FixedFunction>(&mut self) {
        self.swm.pinenable0.modify(|_, w| F::enable(w));
    }

    /// Disables a fixed function
    pub fn disable_fixed_function<F: FixedFunction>(&mut self) {
        self.swm.pinenable0.modify(|_, w| F::disable(w));
    }
}


/// Implemented for types that represent movable functions
pub trait MovableFunction {
    /// Assigns the movable function to a pin
    ///
    /// # Limitations
    ///
    /// This method can be used to assign the movable function to pins that are
    /// currently used for something else. The HAL user needs to make sure that
    /// this assignment doesn't conflict with any other uses of the pin.
    fn assign<P: PinName>(&mut self, swm: &mut Api);
}

macro_rules! movable_functions {
    ($($field:ident, $type:ident, $register:ident, $reg_field:ident;)*) => {
        /// Provides access to all movable functions
        #[allow(missing_docs)]
        pub struct MovableFunctions {
            $(pub $field: $type,)*
        }

        impl MovableFunctions {
            fn new() -> Self {
                MovableFunctions {
                    $($field: $type,)*
                }
            }
        }


        $(
            /// Represents a movable function
            #[allow(non_camel_case_types)]
            pub struct $type;

            impl MovableFunction for $type {
                fn assign<P: PinName>(&mut self, swm: &mut Api) {
                    swm.swm.$register.modify(|_, w|
                        unsafe { w.$reg_field().bits(P::ID)
                    })
                }
            }
        )*
    }
}

movable_functions!(
    u0_txd       , U0_TXD       , pinassign0 , u0_txd_o;
    u0_rxd       , U0_RXD       , pinassign0 , u0_rxd_i;
    u0_rts       , U0_RTS       , pinassign0 , u0_rts_o;
    u0_cts       , U0_CTS       , pinassign0 , u0_cts_i;
    u0_sclk      , U0_SCLK      , pinassign1 , u0_sclk_io;
    u1_txd       , U1_TXD       , pinassign1 , u1_txd_o;
    u1_rxd       , U1_RXD       , pinassign1 , u1_rxd_i;
    u1_rts       , U1_RTS       , pinassign1 , u1_rts_o;
    u1_cts       , U1_CTS       , pinassign2 , u1_cts_i;
    u1_sclk      , U1_SCLK      , pinassign2 , u1_sclk_io;
    u2_txd       , U2_TXD       , pinassign2 , u2_txd_o;
    u2_rxd       , U2_RXD       , pinassign2 , u2_rxd_i;
    u2_rts       , U2_RTS       , pinassign3 , u2_rts_o;
    u2_cts       , U2_CTS       , pinassign3 , u2_cts_i;
    u2_sclk      , U2_SCLK      , pinassign3 , u2_sclk_io;
    spi0_sck     , SPI0_SCK     , pinassign3 , spi0_sck_io;
    spi0_mosi    , SPI0_MOSI    , pinassign4 , spi0_mosi_io;
    spi0_miso    , SPI0_MISO    , pinassign4 , spi0_miso_io;
    spi0_ssel0   , SPI0_SSEL0   , pinassign4 , spi0_ssel0_io;
    spi0_ssek1   , SPI0_SSEL1   , pinassign4 , spi0_ssel1_io;
    spi0_ssel2   , SPI0_SSEL2   , pinassign5 , spi0_ssel2_io;
    spi0_ssel3   , SPI0_SSEL3   , pinassign5 , spi0_ssel3_io;
    spi1_sck     , SPI1_SCK     , pinassign5 , spi1_sck_io;
    spi1_mosi    , SPI1_MOSI    , pinassign5 , spi1_mosi_io;
    spi1_miso    , SPI1_MISO    , pinassign6 , spi1_miso_io;
    spi1_ssel0   , SPI1_SSEL0   , pinassign6 , spi1_ssel0_io;
    spi1_ssel1   , SPI1_SSEL1   , pinassign6 , spi1_ssel1_io;
    sct_pin0     , SCT_PIN0     , pinassign6 , sct_in0_i;
    sct_pin1     , SCT_PIN1     , pinassign7 , sct_in1_i;
    sct_pin2     , SCT_PIN2     , pinassign7 , sct_in2_i;
    sct_pin3     , SCT_PIN3     , pinassign7 , sct_in3_i;
    sct_out0     , SCT_OUT0     , pinassign7 , sct_out0_o;
    sct_out1     , SCT_OUT1     , pinassign8 , sct_out1_o;
    sct_out2     , SCT_OUT2     , pinassign8 , sct_out2_o;
    sct_out3     , SCT_OUT3     , pinassign8 , sct_out3_o;
    sct_out4     , SCT_OUT4     , pinassign8 , sct_out4_o;
    sct_out5     , SCT_OUT5     , pinassign9 , sct_out5_o;
    i2c1_sda     , I2C1_SDA     , pinassign9 , i2c1_sda_io;
    i2c1_scl     , I2C1_SCL     , pinassign9 , i2c1_scl_io;
    i2c2_sda     , I2C2_SDA     , pinassign9 , i2c2_sda_io;
    i2c2_scl     , I2C2_SCL     , pinassign10, i2c2_scl_io;
    i2c3_sda     , I2C3_SDA     , pinassign10, i2c3_sda_io;
    i2c3_scl     , I2C3_SCL     , pinassign10, i2c3_scl_io;
    adc_pintrig0 , ADC_PINTRIG0 , pinassign10, adc_pintrig0_i;
    acd_pintrig1 , ADC_PINTRIG1 , pinassign11, adc_pintrig1_i;
    acmp_o       , ACMP_O       , pinassign11, acmp_o_o;
    clkout       , CLKOUT       , pinassign11, clkout_o;
    gpio_int_bmat, GPIO_INT_BMAT, pinassign11, gpio_int_bmat_o;
);


/// Implemented for types that represent movable functions
///
/// This trait is an internal implementation detail and should neither be
/// implemented nor used outside of LPC82x HAL. Any incompatible changes to this
/// trait won't be considered breaking changes.
pub trait FixedFunction {
    /// Internal method to enable a fixed function
    fn enable(w: &mut pinenable0::W) -> &mut pinenable0::W;

    /// Internal method to disable a fixed function
    fn disable(w: &mut pinenable0::W) -> &mut pinenable0::W;
}

macro_rules! impl_fixed_function {
    ($fixed_function:ident, $field:ident) => {
        /// Represents a fixed function
        ///
        /// Can be used with [`SWM::enable_fixed_function`] and
        /// [`SWM::disable_fixed_function`].
        ///
        /// [`SWM::enable_fixed_function`]: struct.SWM.html#method.enable_fixed_function
        /// [`SWM::disable_fixed_function`]: struct.SWM.html#method.disable_fixed_function
        #[allow(non_camel_case_types)]
        pub struct $fixed_function;

        impl FixedFunction for $fixed_function {
            fn enable(w: &mut pinenable0::W) -> &mut pinenable0::W {
                w.$field().clear_bit()
            }

            fn disable(w: &mut pinenable0::W) -> &mut pinenable0::W {
                w.$field().set_bit()
            }
        }
    }
}

impl_fixed_function!(ACMP_I1 , acmp_i1 );
impl_fixed_function!(ACMP_I2 , acmp_i2 );
impl_fixed_function!(ACMP_I3 , acmp_i3 );
impl_fixed_function!(ACMP_I4 , acmp_i4 );
impl_fixed_function!(SWCLK   , swclk   );
impl_fixed_function!(SWDIO   , swdio   );
impl_fixed_function!(XTALIN  , xtalin  );
impl_fixed_function!(XTALOUT , xtalout );
impl_fixed_function!(RESETN  , resetn  );
impl_fixed_function!(CLKIN   , clkin   );
impl_fixed_function!(VDDCMP  , vddcmp  );
impl_fixed_function!(I2C0_SDA, i2c0_sda);
impl_fixed_function!(I2C0_SCL, i2c0_scl);
impl_fixed_function!(ADC_0   , adc_0   );
impl_fixed_function!(ADC_1   , adc_1   );
impl_fixed_function!(ADC_2   , adc_2   );
impl_fixed_function!(ADC_3   , adc_3   );
impl_fixed_function!(ADC_4   , adc_4   );
impl_fixed_function!(ADC_5   , adc_5   );
impl_fixed_function!(ADC_6   , adc_6   );
impl_fixed_function!(ADC_7   , adc_7   );
impl_fixed_function!(ADC_8   , adc_8   );
impl_fixed_function!(ADC_9   , adc_9   );
impl_fixed_function!(ADC_10  , adc_10  );
impl_fixed_function!(ADC_11  , adc_11  );
