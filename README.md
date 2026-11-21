# Power Electronics (power-electronics-ai999x)

[![tests](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/power-electronics-ai999x)](https://pypi.org/project/power-electronics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/power-electronics-ai999x)

Power Electronics covers DC-DC converters in continuous and discontinuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, bridge), rectifiers and phase control, inverters and PWM, motor drives, device losses, snubbers, thermal and magnetic design and resonant converters: 79 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Power Electronics desktop app: Buck converter in continuous conduction, with its worked example and live scene](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/hero.webp)](https://www.ai999x.com/models/power-electronics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-power-electronics-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-power-electronics-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `power-electronics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install power-electronics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Power Electronics desktop app: every scene and chart is drawn from the computed answer.

[![Power Electronics desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/clip.mp4)*

| | |
|---|---|
| [![Buck converter in continuous conduction: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-1.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_buck_ccm)<br>Buck converter in continuous conduction | [![Three-phase fully controlled thyristor bridge: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-2.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_three_phase_controlled_rectifier)<br>Three-phase fully controlled thyristor bridge |
| [![Space-vector modulation dwell times: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-3.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_space_vector_times)<br>Space-vector modulation dwell times | [![Flyback converter in continuous conduction: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-4.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_flyback_ccm)<br>Flyback converter in continuous conduction |
| [![Core loss by the Steinmetz equation: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-5.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_steinmetz_core_loss)<br>Core loss by the Steinmetz equation | [![Control-to-output transfer function of a DC-DC converter: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-6.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_converter_small_signal)<br>Control-to-output transfer function of a DC-DC converter |
| [![Bootstrap capacitor for a high-side gate driver: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-7.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_bootstrap_capacitor)<br>Bootstrap capacitor for a high-side gate driver | [![Boost power-factor-correction stage: the live scene in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-8.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_boost_pfc)<br>Boost power-factor-correction stage |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Buck converter in continuous conduction: a chart in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-1.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_buck_ccm)<br>Buck converter in continuous conduction | [![Three-phase fully controlled thyristor bridge: a chart in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-2.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_three_phase_controlled_rectifier)<br>Three-phase fully controlled thyristor bridge | [![Space-vector modulation dwell times: a chart in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-3.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_space_vector_times)<br>Space-vector modulation dwell times | [![Flyback converter in continuous conduction: a chart in the Power Electronics desktop app](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-4.webp)](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_flyback_ccm)<br>Flyback converter in continuous conduction |

## Ask in words: 8 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import power_electronics_ai999x as power_electronics

a = power_electronics.ask(
    "Single-phase commutation overlap: Vs = 230 V, f = 0.05 kHz, Ls = "
    "2 mH, Id = 20 A, alpha = 30 deg"
)
print(a.outcome, a.values)
# SOLVED {'u_deg': 7.93041601856, 'Vdc': 171.330264284, 'dV': 8.0}

a = power_electronics.ask(
    "Boost converter output ripple: Vin = 12 V, Vout = 48 V, Io = 1 A, "
    "C = 47 uF, fs = 100 kHz"
)
print(a.outcome, a.values)
# SOLVED {'D': 0.75, 'dVo': 0.159574468085, 'dVo_pct': 0.332446808511}

a = power_electronics.ask(
    "Three-phase commutation overlap: VLL = 400 V, f = 0.05 kHz, Ls = "
    "0.5 mH, Id = 100 A, alpha = 30 deg"
)
print(a.outcome, a.values)
# SOLVED {'u_deg': 5.85622851056, 'Vdc': 452.81808074, 'dV': 15.0}

a = power_electronics.ask(
    "Inductor area product: L = 100 uH, Ipk = 12 A, Irms = 10 A, Bmax "
    "= 300 mT, J = 4 A/mm^2, Ku = 0.4"
)
print(a.outcome, a.values)
# SOLVED {'Ap': 2.5e-08, 'Ap_cm4': 2.5}

a = power_electronics.ask(
    "MOSFET conduction loss: Rds25 = 0.01 ohm, alpha = 0.007, Tj = 125 "
    "degC, Irms = 20 A"
)
print(a.outcome, a.values)
# SOLVED {'Rds_T': 0.017, 'P': 6.8, 'P_25': 4.0, 'ratio': 1.7}

a = power_electronics.ask(
    "Transformer turns: waveform = square, V = 48 V, f = 100 kHz, Bmax "
    "= 150 mT, Ac = 1.25 cm^2, Vs = 12 V"
)
print(a.outcome, a.values)
# SOLVED {'Np': 6.4, 'Ns': 1.6, 'n': 4.0, 'volts_per_turn': 7.5}

a = power_electronics.ask(
    "Critical inductance: topology = buck, Vin = 24 V, Vout = 12 V, fs "
    "= 100 kHz, R = 10 ohm"
)
print(a.outcome, a.values)
# SOLVED {'D': 0.5, 'L_crit': 2.5e-05, 'Io': 1.2}

a = power_electronics.ask(
    "DC-DC conversion ratio: topology = boost, Vin = 12 V, D = 0.6"
)
print(a.outcome, a.values)
# SOLVED {'M': 2.5, 'Vout': 30.0, 'polarity': 'same'}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Power Electronics on ai999x.com](https://www.ai999x.com/models/power-electronics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/power-electronics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/power-electronics-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `power-electronics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/power-electronics`) and its data (`rust/models/power_electronics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p power-electronics-ai999x
./target/release/power-electronics --help
cargo test -p power-electronics-ai999x
```
