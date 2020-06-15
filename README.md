# 味噌 - Miso

A cross-platform ambient soundscape generator. No browser included.

![Screenshot of Miso](screenshot.png)

Miso is built using [`rodio`](https://github.com/RustAudio/rodio) and [`iced`](https://github.com/hecrj/iced).

Use `cargo run --release` to build & run.

## Troubleshooting
- The `--release` flag is needed for clear sound playback.

- If you run into issues with the `alsa-sys` crate while building, ex:

  `error: failed to run custom build command for 'alsa-sys v0.1.2'`

  You may need to install the `libsound2-dev` library. On Ubuntu: 

  `sudo apt install -y libasound2-dev`

