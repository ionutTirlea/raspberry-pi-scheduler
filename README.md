# raspberry-pi-scheduler

A lightweight Rust scheduler for running recurring tasks on a Raspberry Pi.

## How it works

1. Place a `tasks.toml` file next to the binary
2. Run the binary
3. It runs your commands on the defined schedules

## tasks.toml

```toml
[[task]]
name = "check_temperature"
schedule = "0 */5 * * * *"
command = "/opt/plugins/check_temperature"

[[task]]
name = "water_plants"
schedule = "0 0 8 * * *"
command = "/opt/plugins/water_plants"
```

- `name` — a label for logging
- `schedule` — a cron expression (seconds minutes hours day month weekday)
- `command` — path to the executable to run

## Build

```sh
cargo build --release
```

The binary will be at `target/release/raspberry-pi-scheduler`.

## Run

```sh
./raspberry-pi-scheduler
```

## Cross-compile for Raspberry Pi

```sh
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
```

Then copy the binary and `tasks.toml` to the Pi.

## License

This project is licensed under the MIT License.
See the [LICENSE](LICENSE) file for details.
