# esp32osc

Rust based esp-32 oscilliscope.

---


Uses esp32 ADC to read voltage level and send it via UART to computer. The data can be read by this (rust based viewer)[https://github.com/Mo-Ehab/egui_viewer_for_esp32osc/]. You will need to install (espflash)[https://github.com/esp-rs/espflash/tree/main/espflash].

it can currently read up to 2000 sample per second.

---

### How to run
Can be run by `cargo run --release`

