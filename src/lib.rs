//! # hda-nostd
//!
//! A `no_std` Intel High Definition Audio (HDA) driver for bare-metal Rust.
//!
//! Implements the Intel HDA specification 1.0a for bare-metal audio playback.
//! Supports codec discovery, widget enumeration, output path finding, and
//! PCM audio streaming via DMA.
//!
//! This crate is `#![no_std]` and uses `extern crate alloc` for DMA buffer
//! allocation. All hardware access is volatile MMIO through BAR0.
//!
//! ## Features
//!
//! - Controller initialization and reset (GCTL, GCAP)
//! - CORB/RIRB ring buffer communication with codecs
//! - Full codec enumeration: vendor ID, widget tree, pin configs
//! - Automatic output path discovery (DAC -> Mixer -> Pin)
//! - PCM stream setup with BDL (Buffer Descriptor List) DMA
//! - Common stream formats: 48kHz/44.1kHz/96kHz, 16/24-bit, mono/stereo
//! - Amp gain/mute control and EAPD support
//! - Built-in sine wave tone generator (no libm needed)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use hda_nostd::{HdaController, StreamFormat};
//!
//! // BAR0 address from PCI enumeration (must be identity-mapped)
//! let bar0: usize = 0xFE_0000_0000;
//!
//! unsafe {
//!     let mut ctrl = HdaController::init(bar0).expect("HDA init failed");
//!
//!     if let Some(mut out) = ctrl.output() {
//!         // Play a 440 Hz beep for 500 ms
//!         out.beep(440, 500).ok();
//!     }
//! }
//! ```

#![no_std]

extern crate alloc;

pub mod registers;
pub mod codec;
pub mod widget;
pub mod stream;
pub mod driver;

pub use driver::{HdaController, HdaOutput};
pub use widget::WidgetType;
pub use stream::StreamFormat;
