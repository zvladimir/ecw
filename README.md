<p align="center"><img src="ui/img/logo.svg" width="96" height="96" alt="ECW logo"></p>

# ECW — Engineers' Circuit Workbench

**English** · [Русский](README.ru.md) · [Esperanto](README.eo.md)

*Fast calculations. Practical engineering.*

**Small, focused calculators for an electronics engineer's everyday work.**

You enter the values you know, and the program tries to work out all the other quantities by itself: resistor values, gains, signal ranges, RMS currents. Values can be entered with a tolerance (`10k 1%`), and it is taken into account in the calculations. The built-in help not only explains how to use the program, but also describes in detail the order of the calculations, with all the formulas.

![ECW in action: pick a standard resistor, then design an inverting amplifier by typing the values you know](docs/img/demo.gif)

## Features

- **Fill in what you know.** No modes and no *Calculate* button: enter any sufficient set of values and the remaining ones are computed.
- **Tolerances everywhere they matter.** Worst-case results by exact corner analysis, not just an optimistic nominal value.
- **Schematics as forms.** On the op-amp and level shifter pages you type the values right into the circuit; a **Plot** tab shows the transfer line and its tolerance band.
- **Help as a textbook (F1).** Every calculator has theory, the order of the calculation with all the formulas, worked examples, and links to the documents and standards it relies on.
- **Engineering input.** SI prefixes (`4.7k`, `100n`, `2.2M`), ranges (`-5; -1`), asymmetric tolerances (`10k +5% -1%`).
- **English, Russian and Esperanto**: the interface and the help.
- **Offline desktop app** for Windows, macOS and Linux.

## Calculators

| Calculator | What it does | Based on |
|---|---|---|
| **Ohm's Law** | any two of U, I, R, P give all the others | |
| **Voltage Divider** | two or three resistors; any three (or four) quantities give all the others | |
| **Resistor Solver** | the nearest standard value and the best series and parallel pairs, from E6…E96 or your own set of resistors | IEC 60063 |
| **AWG Table** | resistance, voltage drop, power and temperature rise of copper wire, 40 AWG to 4/0 | ASTM B258 |
| **Current Shape** | RMS and average current, duty cycle for six current waveforms | |
| **Op-Amp (inverting, non-inverting, differential)** | gain, resistors and signal ranges; tolerances are taken into account | differential: TI [SBOA274](https://www.ti.com/lit/SBOA274) |
| **Level Shifter (inverting, non-inverting)** | calculates the gain and the level shift of the input signal, suggests the required reference voltage | TI [SBOA261](https://www.ti.com/lit/SBOA261), [SBOA262](https://www.ti.com/lit/SBOA262) |

| Difference amplifier | Plot with the tolerance band |
|---|---|
| ![Difference amplifier](docs/img/difference-amplifier.png) | ![Output against input with the worst-case band](docs/img/plot.png) |

![A page of the built-in help: derivation and design procedure of the non-inverting level shifter](docs/img/help.png)

## Install

Download the file for your system from the [Releases](https://github.com/zvladimir/ecw/releases) page.

Nothing to install: each release has a Windows `.exe` and a Linux `.AppImage` that run as they are (and a much smaller plain Linux binary `ecw-<version>-linux-x86_64` for systems that already have WebKitGTK 4.1 and GTK 3: `chmod +x` and run), `SHA256SUMS`, and `ecw-<version>-web.html`: the same calculators as ONE html file with WebAssembly inside, for computers where programs cannot be started. Open it in any current browser by a double click: no server and no internet needed.

## Build from source

You need [Rust](https://www.rust-lang.org/tools/install) 1.88 or newer and the system libraries of Tauri 2 (on Linux: WebKitGTK and friends, see the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)).

```sh
git clone https://github.com/zvladimir/ecw.git
cd ecw
cargo run -p ecw                    # run the app
cargo test -p ecw-core              # unit, golden and contract tests
cargo build -p ecw --release        # an optimised binary in target/release
```

The release files are made by the Tauri CLI: `cargo install tauri-cli`, then `cargo tauri build --no-bundle` (a bare executable) or `cargo tauri build --bundles appimage`.

The browser version: `rustup target add wasm32-unknown-unknown`, `cargo install wasm-bindgen-cli --locked` (the same version as `wasm-bindgen` in `Cargo.lock`), then `./scripts/build-web.sh` makes `target/web/ecw.html` (Python 3 packs the single file).

## Contributing

Bug reports with the values you entered, the result you expected and the source of the formula are very welcome. To add a calculator, see [CONTRIBUTING.md](.github/CONTRIBUTING.md): it walks through the physics, the API, the texts, the page and the tests.

## History

Version 0.2 is a rewrite as a Tauri app. The first version, *Electrical Calculations Wizard* 0.1a (Rust + iced, Ohm's law and the voltage divider), is in the history of this repository.

## License and support

ECW is free software under the [MIT license](LICENSE). If it saves you time, you can support the development on [Boosty](https://boosty.to/desialsi).
