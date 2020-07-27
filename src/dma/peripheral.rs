use crate::{init_state, pac, syscon};

use super::Channels;

/// Entry point to the DMA API
pub struct DMA {
    /// Handle to the DMA controller
    pub handle: Handle<init_state::Disabled>,

    /// The DMA channels
    pub channels: Channels,
}

impl DMA {
    pub(crate) fn new(dma: pac::DMA0) -> Self {
        let descriptors = unsafe { &mut super::descriptors::DESCRIPTORS };
        let srambase = descriptors as *mut _ as u32;

        Self {
            handle: Handle::new(dma, srambase),
            channels: Channels::new(descriptors),
        }
    }

    /// Return the raw peripheral
    ///
    /// This method serves as an escape hatch from the HAL API. It returns the
    /// raw peripheral, allowing you to do whatever you want with it, without
    /// limitations imposed by the API.
    ///
    /// If you are using this method because a feature you need is missing from
    /// the HAL API, please [open an issue] or, if an issue for your feature
    /// request already exists, comment on the existing issue, so we can
    /// prioritize it accordingly.
    ///
    /// [open an issue]: https://github.com/lpc-rs/lpc8xx-hal/issues
    pub fn free(self) -> pac::DMA0 {
        self.handle.dma
    }
}

/// Handle to the DMA controller
pub struct Handle<State = init_state::Enabled> {
    _state: State,
    dma: pac::DMA0,
    srambase: u32,
}

impl Handle<init_state::Disabled> {
    pub(crate) fn new(dma: pac::DMA0, srambase: u32) -> Self {
        Handle {
            _state: init_state::Disabled,
            dma,
            srambase,
        }
    }
}

impl<'dma> Handle<init_state::Disabled> {
    /// Enable the DMA controller
    pub fn enable(
        self,
        syscon: &mut syscon::Handle,
    ) -> Handle<init_state::Enabled> {
        syscon.enable_clock(&self.dma);

        // Set descriptor table address
        //
        // See user manual, section 12.6.3.
        self.dma
            .srambase
            .write(|w| unsafe { w.bits(self.srambase) });

        // Enable the DMA controller
        //
        // See user manual, section 12.6.1.
        self.dma.ctrl.write(|w| w.enable().enabled());

        Handle {
            _state: init_state::Enabled(()),
            dma: self.dma,
            srambase: self.srambase,
        }
    }
}

impl Handle<init_state::Enabled> {
    /// Disable the DMA controller
    pub fn disable(
        self,
        syscon: &mut syscon::Handle,
    ) -> Handle<init_state::Disabled> {
        syscon.disable_clock(&self.dma);

        Handle {
            _state: init_state::Disabled,
            dma: self.dma,
            srambase: self.srambase,
        }
    }
}
