# Power Electronics (power-electronics-ai999x)

[![tests](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/power-electronics-ai999x)](https://pypi.org/project/power-electronics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/power-electronics-ai999x)

Power Electronics covers DC-DC converters in continuous and discontinuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, bridge), rectifiers and phase control, inverters and PWM, motor drives, device losses, snubbers, thermal and magnetic design and resonant converters: 79 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Power Electronics desktop app: Buck converter in continuous conduction, with its worked example and live scene](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/hero.webp)](https://www.ai999x.com/models/power-electronics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-power-electronics-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-power-electronics-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `power-electronics` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
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

## Command line: 15 examples

Each command below was run with `power-electronics` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ power-electronics solve pe_converter_small_signal topology=boost "Vin=12 V" D=0.5 "L=50 uH" "C=100 uF" "R=10 ohm"
Gvd0 = 48 V
f0 = 1125.395395 Hz
Q = 7.071067812
f_rhpz = 7957.747155 Hz
rhp_zero = present

$ power-electronics solve pe_boost_ccm "Vin=12 V" "Vout=48 V" "L=100 uH" "fs=100 kHz" "Io=1 A"
D = 0.75
IL = 4 A
dIL = 0.9 A
IL_max = 4.45 A
IL_min = 3.55 A
V_switch = 48 V

$ power-electronics solve pe_bootstrap_capacitor "Qg=60 nC" "Ileak=200 uA" "fs=100 kHz" Dmax=0.9 "dV=0.5 V"
Q_total = 6.18e-08 C
C_min = 1.236e-07 F
C_recommended = 1.236e-06 F

$ power-electronics solve pe_boost_pfc "Vac=230 V" "f_line=50 Hz" "Vo=400 V" "Po=500 W" eff=0.95 "L=500 uH" "fs=65 kHz" "C=330 uF"
I_in_pk = 3.236186642 A
D_peak = 0.1868272016
dI_peak = 1.869819057 A
dI_max = 3.076923077 A
IL_at_peak = 4.17109617 A
dVo_pp = 12.05719266 V

$ power-electronics solve pe_dead_time_error "Vdc=600 V" "td=2000 ns" "fs=10 kHz" "V1=250 V"
dV_avg = 12 V
dV1_peak = 15.27887454 V
dV1_pct = 6.111549815

$ power-electronics solve pe_flyback_ccm "Vin=48 V" "Vout=12 V" n=2 "Lm=200 uH" "fs=100 kHz" "Io=2 A"
D = 0.3333333333
Im = 1.5 A
dIm = 0.8 A
Ipk_primary = 1.9 A
Ipk_secondary = 3.8 A
V_switch = 72 V

$ power-electronics solve pe_boost_nonideal_gain "Vin=12 V" D=0.75 "R=50 ohm" "RL=0.5 ohm"
M = 3.448275862
Vout = 41.37931034 V
efficiency = 0.8620689655
IL = 3.310344828 A
D_peak = 0.9
M_max = 5

$ power-electronics solve pe_steinmetz_core_loss k=3.2 alpha=1.4 beta=2.5 "f=100 kHz" "Bpk=100 mT" "Vc=5 cm^3"
Pv = 101192.8851 W/m^3
Pv_mW_cm3 = 101.1928851
P = 0.5059644256 W

$ power-electronics solve pe_dc_motor_armature_ripple "Vs=220 V" D=0.5 "fs=1 kHz" "Ra=1 ohm" "La=10 mH" "Ea=100 V"
I_max = 12.74942723 A
I_min = 7.250572773 A
dI = 5.498854453 A
I_avg = 10 A

$ power-electronics solve pe_pulse_waveform_rms A=12 D=0.25
avg = 3
rms = 6
ac_rms = 5.196152423
form_factor = 2
ripple_factor = 1.732050808

$ power-electronics solve pe_capacitor_filter_exact circuit=half-wave "Vp=169.7 V" "f=0.06 kHz" "R=500 ohm" "C=100 uF"
theta_off_deg = 93.03678865
alpha_deg = 48.23082389
dV = 43.13189012 V
V_avg = 148.1028626 V
I_D_peak = 4.514737529 A
conduction_deg = 44.80596476

$ power-electronics solve pe_dual_active_bridge_power "V1=400 V" "V2=48 V" n=8 "L=50 uH" "fs=100 kHz" "phi=30 deg"
P = 2133.333333 W
P_max = 3840 W
i0 = -7.2 A
i_phi = 5.866666667 A
I_rms = 6.175699091 A

$ power-electronics solve pe_foster_periodic_pulses "R=[0.05, 0.1, 0.15]" "tau=[0.0001, 0.003, 0.05]" "P=500 W" "tp=2 ms" "T=10 ms"
dT_peak = 66.45250612 K
dT_min = 15.57762553 K
dT_avg = 30 K
ripple = 50.87488059 K

$ power-electronics solve pe_peak_current_mode_slope "Vin=12 V" "Vout=8 V" "L=10 uH" "fs=300 kHz" "ma=0.3 A/us"
D = 0.6666666667
m1 = 400000 A/s
m2 = 800000 A/s
alpha = -0.7142857143
stable = stable
ma_min = 200000 A/s

$ power-electronics solve pe_boost_dcm "Vin=12 V" "L=10 uH" "fs=100 kHz" "R=100 ohm" D=0.4
K = 0.02
M = 3.372281323
Vout = 40.46737588 V
D2 = 0.1686140662
I_pk = 4.8 A
Io = 0.4046737588 A
```

## Rust library: 15 examples

The crate `power-electronics-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `power_electronics_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/power-electronics/examples/readme.rs`: `cd rust && cargo run --release --example readme -p power-electronics-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&power_electronics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "pe_converter_small_signal", "inputs": {"topology": "boost", "Vin": "12 V", "D": "0.5", "L": "50 uH", "C": "100 uF", "R": "10 ohm"}}));
// SOLVED
// Gvd0 = 48.0 V
// f0 = 1125.3953952 Hz
// Q = 7.07106781187
// f_rhpz = 7957.74715459 Hz
// rhp_zero = "present"

show(json!({"key": "pe_boost_ccm", "inputs": {"Vin": "12 V", "Vout": "48 V", "L": "100 uH", "fs": "100 kHz", "Io": "1 A"}}));
// SOLVED
// D = 0.75
// IL = 4.0 A
// dIL = 0.9 A
// IL_max = 4.45 A
// IL_min = 3.55 A
// V_switch = 48.0 V

show(json!({"key": "pe_bootstrap_capacitor", "inputs": {"Qg": "60 nC", "Ileak": "200 uA", "fs": "100 kHz", "Dmax": "0.9", "dV": "0.5 V"}}));
// SOLVED
// Q_total = 6.18e-8 C
// C_min = 1.236e-7 F
// C_recommended = 1.236e-6 F

show(json!({"key": "pe_boost_pfc", "inputs": {"Vac": "230 V", "f_line": "50 Hz", "Vo": "400 V", "Po": "500 W", "eff": "0.95", "L": "500 uH", "fs": "65 kHz", "C": "330 uF"}}));
// SOLVED
// I_in_pk = 3.23618664159 A
// D_peak = 0.186827201635
// dI_peak = 1.86981905679 A
// dI_max = 3.07692307692 A
// IL_at_peak = 4.17109616998 A
// dVo_pp = 12.0571926585 V

show(json!({"key": "pe_dead_time_error", "inputs": {"Vdc": "600 V", "td": "2000 ns", "fs": "10 kHz", "V1": "250 V"}}));
// SOLVED
// dV_avg = 12.0 V
// dV1_peak = 15.2788745368 V
// dV1_pct = 6.11154981473

show(json!({"key": "pe_flyback_ccm", "inputs": {"Vin": "48 V", "Vout": "12 V", "n": "2", "Lm": "200 uH", "fs": "100 kHz", "Io": "2 A"}}));
// SOLVED
// D = 0.333333333333
// Im = 1.5 A
// dIm = 0.8 A
// Ipk_primary = 1.9 A
// Ipk_secondary = 3.8 A
// V_switch = 72.0 V

show(json!({"key": "pe_boost_nonideal_gain", "inputs": {"Vin": "12 V", "D": "0.75", "R": "50 ohm", "RL": "0.5 ohm"}}));
// SOLVED
// M = 3.44827586207
// Vout = 41.3793103448 V
// efficiency = 0.862068965517
// IL = 3.31034482759 A
// D_peak = 0.9
// M_max = 5.0

show(json!({"key": "pe_steinmetz_core_loss", "inputs": {"k": "3.2", "alpha": "1.4", "beta": "2.5", "f": "100 kHz", "Bpk": "100 mT", "Vc": "5 cm^3"}}));
// SOLVED
// Pv = 101192.885125 W/m^3
// Pv_mW_cm3 = 101.192885125
// P = 0.505964425627 W

show(json!({"key": "pe_dc_motor_armature_ripple", "inputs": {"Vs": "220 V", "D": "0.5", "fs": "1 kHz", "Ra": "1 ohm", "La": "10 mH", "Ea": "100 V"}}));
// SOLVED
// I_max = 12.7494272265 A
// I_min = 7.25057277347 A
// dI = 5.49885445305 A
// I_avg = 10.0 A

show(json!({"key": "pe_pulse_waveform_rms", "inputs": {"A": "12", "D": "0.25"}}));
// SOLVED
// avg = 3.0
// rms = 6.0
// ac_rms = 5.19615242271
// form_factor = 2.0
// ripple_factor = 1.73205080757

show(json!({"key": "pe_capacitor_filter_exact", "inputs": {"circuit": "half-wave", "Vp": "169.7 V", "f": "0.06 kHz", "R": "500 ohm", "C": "100 uF"}}));
// SOLVED
// theta_off_deg = 93.0367886534
// alpha_deg = 48.2308238926
// dV = 43.1318901238 V
// V_avg = 148.102862563 V
// I_D_peak = 4.51473752936 A
// conduction_deg = 44.8059647608

show(json!({"key": "pe_dual_active_bridge_power", "inputs": {"V1": "400 V", "V2": "48 V", "n": "8", "L": "50 uH", "fs": "100 kHz", "phi": "30 deg"}}));
// SOLVED
// P = 2133.33333333 W
// P_max = 3840.0 W
// i0 = -7.2 A
// i_phi = 5.86666666667 A
// I_rms = 6.17569909073 A

show(json!({"key": "pe_foster_periodic_pulses", "inputs": {"R": [0.05, 0.1, 0.15], "tau": [0.0001, 0.003, 0.05], "P": "500 W", "tp": "2 ms", "T": "10 ms"}}));
// SOLVED
// dT_peak = 66.4525061197 K
// dT_min = 15.5776255331 K
// dT_avg = 30.0 K
// ripple = 50.8748805866 K

show(json!({"key": "pe_peak_current_mode_slope", "inputs": {"Vin": "12 V", "Vout": "8 V", "L": "10 uH", "fs": "300 kHz", "ma": "0.3 A/us"}}));
// SOLVED
// D = 0.666666666667
// m1 = 400000.0 A/s
// m2 = 800000.0 A/s
// alpha = -0.714285714286
// stable = "stable"
// ma_min = 200000.0 A/s

show(json!({"key": "pe_boost_dcm", "inputs": {"Vin": "12 V", "L": "10 uH", "fs": "100 kHz", "R": "100 ohm", "D": "0.4"}}));
// SOLVED
// K = 0.02
// M = 3.37228132327
// Vout = 40.4673758792 V
// D2 = 0.168614066163
// I_pk = 4.8 A
// Io = 0.404673758792 A
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Power Electronics on ai999x.com](https://www.ai999x.com/models/power-electronics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/power-electronics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/power-electronics-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `power-electronics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/power-electronics`) and its data (`rust/models/power_electronics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p power-electronics-ai999x
./target/release/power-electronics --help
cargo test -p power-electronics-ai999x
```
