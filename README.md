# hda-nostd

[![Crates.io](https://img.shields.io/crates/v/hda-nostd.svg)](https://crates.io/crates/hda-nostd)
[![docs.rs](https://docs.rs/hda-nostd/badge.svg)](https://docs.rs/hda-nostd)
[![License](https://img.shields.io/crates/l/hda-nostd.svg)](https://github.com/suhteevah/hda-nostd#license)

A `#![no_std]` Intel High Definition Audio (HDA) driver for bare-metal Rust.

Implements the Intel HDA specification 1.0a for bare-metal audio playback. No OS
required -- just give it a BAR0 address from PCI enumeration and it handles the rest.

## Features

- **Controller init & reset** -- GCTL, GCAP, version check, full reset sequence
- **CORB/RIRB communication** -- ring buffer setup, verb encoding, response polling
- **Codec discovery** -- vendor/device ID, function group enumeration
- **Widget tree enumeration** -- audio outputs, inputs, mixers, selectors, pin complexes
- **Pin configuration decoding** -- device type, connectivity, association, location
- **Automatic output path finding** -- DAC -> Mixer/Selector -> Pin traversal
- **PCM stream playback** -- BDL (Buffer Descriptor List) DMA, stream format config
- **Amp control** -- gain/mute on output and input amplifiers, EAPD support
- **Preset formats** -- 48kHz/44.1kHz/96kHz, 16/24-bit, mono/stereo
- **Tone generator** -- built-in sine wave via fixed-point math (no libm)

## Requirements

- `#![no_std]` environment with a global allocator (`extern crate alloc`)
- Identity-mapped memory (heap virtual addresses == physical addresses)
- x86_64 target (MMIO via volatile reads/writes)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hda-nostd = "0.1"
```

Basic example:

```rust,no_run
use hda_nostd::{HdaController, StreamFormat};

// BAR0 from PCI enumeration (must be identity-mapped)
let bar0: usize = 0xFE_0000_0000;

unsafe {
    // Initialize controller, discover codecs, find output path
    let mut ctrl = HdaController::init(bar0).expect("HDA init failed");

    if let Some(mut out) = ctrl.output() {
        // Play a 440 Hz beep for 500 ms
        out.beep(440, 500).ok();

        // Or play raw PCM samples
        let samples: Vec<i16> = vec![0i16; 48000 * 2]; // 1 second stereo silence
        out.play_pcm(&samples, 48000, 2).ok();

        // Adjust volume (0-127)
        out.set_volume(100);
    }
}
```

## Modules

| Module | Description |
|--------|-------------|
| `registers` | HDA controller MMIO register offsets, bit fields, volatile accessors |
| `codec` | CORB/RIRB ring buffers, verb encoding, codec communication |
| `widget` | Widget types, capabilities, pin config, codec/widget tree structures |
| `stream` | Stream format, BDL entries, DMA stream management, tone generator |
| `driver` | High-level `HdaController` and `HdaOutput` API |

## Specification Reference

All register offsets, bit fields, and protocol details follow the
[Intel High Definition Audio Specification, Revision 1.0a (June 2010)](https://www.intel.com/content/dam/www/public/us/en/documents/product-specifications/high-definition-audio-specification.pdf).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
