# `uno-r4wifi-bsc`

Board support crate for the Arduino Uno R4 WiFi.  Currently just a little driver for the onboard LED matrix.  E.g.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = ra4_hal::init();

    // This will claim ownership of the pins we need to prevent accidental misuse.
    let matrix = led_matrix_init!(p);

    // The async version of set_pixel adds a small delay at the end
    // to slow things down enough to prevent side effects.
    matrix.set_pixel(1, true).await;
}
```

Note that as the LEDs are multiplexed you'll have to invoke this in a loop to keep multiple pixels "on".
