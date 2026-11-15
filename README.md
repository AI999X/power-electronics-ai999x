# Power Electronics (power-electronics-ai999x)

[![tests](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/power-electronics-ai999x)](https://pypi.org/project/power-electronics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/power-electronics-ai999x)

Power Electronics covers DC-DC converters in continuous and discontinuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, bridge), rectifiers and phase control, inverters and PWM, motor drives, device losses, snubbers, thermal and magnetic design and resonant converters: 79 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Power Electronics desktop app: Buck converter switch, diode, inductor and input-capacitor currents, with its worked example and live scene](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/hero.webp)](https://www.ai999x.com/models/power-electronics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `power-electronics` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#try) |
| **Python** 3.10+ | `pip install power-electronics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import power_electronics_ai999x as power_electronics
a = power_electronics.ask("Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 0.899621212121 A
```

Every calculation, with its formula, a worked example and its figures: **[Power Electronics on ai999x.com](https://www.ai999x.com/models/power-electronics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/power-electronics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/power-electronics-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `power-electronics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/power-electronics`) and its data (`rust/models/power_electronics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p power-electronics-ai999x
./target/release/power-electronics --help
cargo test -p power-electronics-ai999x
```
