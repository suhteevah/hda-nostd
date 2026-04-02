# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Initial release.
- Controller initialization and reset (Intel HDA spec 1.0a).
- CORB/RIRB ring buffer communication with codecs.
- Codec discovery with vendor/device ID and function group enumeration.
- Full widget tree enumeration: audio outputs, inputs, mixers, selectors, pin complexes.
- Pin configuration default decoding (device type, connectivity, association).
- Automatic output path discovery (DAC -> Mixer/Selector -> Pin).
- PCM stream playback via Buffer Descriptor List (BDL) DMA.
- Preset stream formats: 48kHz/44.1kHz/96kHz, 16/24-bit, mono/stereo.
- Amplifier gain/mute control on output and input paths.
- EAPD (External Amplifier Power Down) support.
- Built-in sine wave tone generator using fixed-point math (no libm).
- High-level `HdaController` and `HdaOutput` API.
- All MMIO register constants and volatile accessors.
