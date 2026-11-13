# power-electronics-ai999x: offline, verified power electronics solver with step-by-step answers

[![tests](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/power-electronics-ai999x)](https://pypi.org/project/power-electronics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/power-electronics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/power-electronics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/power-electronics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/power-electronics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/power-electronics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

power-electronics-ai999x is an offline, deterministic power electronics solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers power electronics questions with verified results from 79 certified capabilities, gives 482 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**power-electronics-ai999x** reads a question in plain English (or a typed call), picks a certified power electronics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** power-electronics-ai999x comes as the AI999X desktop app and as the command-line program `power-electronics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Power Electronics desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-power-electronics-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-power-electronics-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`power-electronics`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install power-electronics-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Power Electronics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Power Electronics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-power-electronics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-power-electronics-1.5.0-linux-x86_64.tar.gz
./ai999x-power-electronics-1.5.0-linux-x86_64/ai999x-power-electronics
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
power-electronics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
power-electronics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ power-electronics ask "Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A"
Read as: Buck converter in continuous conduction
D = 0.2083333333
dIL = 0.8996212121 A
IL_max = 3.449810606 A
IL_min = 2.550189394 A
ripple_ratio = 0.2998737374
Iin = 0.625 A
  by: Buck converter in continuous conduction  (D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io)
  Io = 3 A
  L = 2.2e-05 H
  Vin = 24 V
  Vout = 5 V
  fs = 200000 Hz
  assumes: 0 < Vout < Vin; continuous conduction (IL_min > 0)
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ power-electronics info pe_buck_ccm
Buck converter in continuous conduction
  D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io
  in   Vin      input voltage [V]
  in   Vout     output voltage [V]
  in   L        inductance [H]
  in   fs       switching frequency [Hz]
  in   Io       load (output) current [A]
  out  D        duty cycle [1]
  out  dIL      peak-to-peak inductor ripple [A]
  out  IL_max   peak inductor current [A]
  out  IL_min   valley inductor current [A]
  out  ripple_ratio ripple ratio dIL/Io [1]
  out  Iin      average input current [A]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ power-electronics solve pe_buck_ccm "Vin=24 V" "Vout=5 V" "L=22 uH" "fs=200 kHz" "Io=3 A"
D = 0.2083333333
dIL = 0.8996212121 A
IL_max = 3.449810606 A
IL_min = 2.550189394 A
ripple_ratio = 0.2998737374
Iin = 0.625 A
  by: Buck converter in continuous conduction  (D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io)
  Vin = 24 V
  Vout = 5 V
  L = 2.2e-05 H
  fs = 200000 Hz
  Io = 3 A
  assumes: 0 < Vout < Vin; continuous conduction (IL_min > 0)
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | power-electronics batch

`cases.jsonl`:

```json
{"key": "pe_buck_ccm", "inputs": {"Vin": "24 V", "Vout": "5 V", "L": "22 uH", "fs": "200 kHz", "Io": "3 A"}}
{"key": "pe_three_phase_controlled_rectifier", "inputs": {"VLL": "415 V", "alpha": "45 deg", "Id": "50 A"}}
```

```console
$ power-electronics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.899621212121,"unit":"A","solved_for":"dIL","si":[["Vin",24.0],["Vout",5.0],["L",0.000022],["fs",200000.0],["Io",3.0]],"values":[["D",0.208333333333,""],["dIL",0.899621212121,"A"],["IL_max",3.44981060606,"A"],["IL_min",2.55018939394,"A"],["ripple_ratio",0.299873737374,""],["Iin",0.625,"A"]],"target":"dIL","phrase":"duty cycle","assumptions":["0 < Vout < Vin","continuous conduction (IL_min > 0)"],"name":"Buck converter in continuous conduction","equation":"D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io"}
{"outcome":"SOLVED","message":"","value":396.295808299,"unit":"V","solved_for":"Vdc","si":[["VLL",415.0],["alpha",0.7853981633974483],["Id",50.0]],"values":[["Vdc",396.295808299,"V"],["Vrms",415.0,"V"],["P",19814.7904149,"W"],["I_thy_avg",16.6666666667,"A"],["I_thy_rms",28.8675134595,"A"],["Is_rms",40.8248290464,"A"],["mode","rectifying",""]],"target":"Vdc","phrase":"average output voltage","assumptions":["0 <= alpha < 180 degrees","continuous constant current, no source inductance"],"name":"Three-phase fully controlled thyristor bridge","equation":"Vdc = 3 sqrt(2) VLL cos(alpha)/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3) cos(2 alpha)/(4 pi)); I_thy = Id/3 (avg), Id/sqrt(3) (rms)"}
```

### In your browser

Open [Power Electronics on ai999x.com](https://www.ai999x.com/models/power-electronics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `power_electronics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install power-electronics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Control-to-output transfer function of a DC-DC converter

The control-to-output (duty-to-voltage) transfer function features of an ideal buck, boost or buck-boost converter in continuous conduction: dc gain, double-pole frequency, quality factor and right-half-plane zero.

Command line:

```console
$ power-electronics solve pe_converter_small_signal topology=boost "Vin=12 V" D=0.5 "L=50 uH" "C=100 uF" "R=10 ohm"
Gvd0 = 48 V
f0 = 1125.395395 Hz
Q = 7.071067812
f_rhpz = 7957.747155 Hz
rhp_zero = present
  by: Control-to-output transfer function of a DC-DC converter  (buck: Gvd0 = Vin, w0 = 1/sqrt(LC), Q = R sqrt(C/L); boost: Gvd0 = Vin/D'^2, w0 = D'/sqrt(LC), Q = D' R sqrt(C/L), wz = D'^2 R/L; buck-boost: wz = D'^2 R/(D L))
  Vin = 12 V
  D = 0.5
  L = 5e-05 H
  C = 0.0001 F
  R = 10 ohm
  topology = boost
  assumes: ideal converter in continuous conduction, no ESR
```

Python:

```python
import power_electronics_ai999x as m

a = m.solve("pe_converter_small_signal", topology="boost", Vin="12 V", D="0.5", L="50 uH", C="100 uF", R="10 ohm")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_converter_small_signal) and press **Compute in your browser**.

_What this means:_ This calculation determines the dynamic response characteristics of an ideal DC-DC converter operating in continuous conduction mode. Using the input voltage 12 V, duty cycle 0.5, inductance 50 uH, capacitance 100 uF, and load resistance 10 ohm, it computes the small-signal transfer function parameters. The result includes the DC gain 48 V, which represents the steady-state change in output voltage per unit change in duty cycle. The double-pole frequency 1125.4 Hz indicates where the magnitude begins to roll off at forty decibels per decade, while the quality factor 7.07107 describes the sharpness of the resonance peak near this frequency. If applicable, the right-half-plane zero frequency 7957.75 Hz is also calculated, marking a critical point for stability analysis.

### Peak current mode: subharmonic stability and slope compensation

The inductor current slopes, the period-to-period perturbation factor, the stability verdict and the smallest and the deadbeat compensation ramp slopes of a peak-current-mode buck converter.

Command line:

```console
$ power-electronics solve pe_peak_current_mode_slope "Vin=12 V" "Vout=8 V" "L=10 uH" "fs=300 kHz" "ma=0.3 A/us"
D = 0.6666666667
m1 = 400000 A/s
m2 = 800000 A/s
alpha = -0.7142857143
stable = stable
ma_min = 200000 A/s
ma_deadbeat = 800000 A/s
  by: Peak current mode: subharmonic stability and slope compensation  (m1 = (Vin - Vo)/L, m2 = Vo/L; alpha = -(m2 - ma)/(m1 + ma); stable when |alpha| < 1; ma,min = (m2 - m1)/2)
  Vin = 12 V
  Vout = 8 V
  L = 1e-05 H
  fs = 300000 Hz
  ma = 300000 A/s
  assumes: 0 < Vout < Vin; continuous conduction
```

Python:

```python
import power_electronics_ai999x as m

a = m.solve("pe_peak_current_mode_slope", Vin="12 V", Vout="8 V", L="10 uH", fs="300 kHz", ma="0.3 A/us")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_peak_current_mode_slope) and press **Compute in your browser**.

_What this means:_ This calculation determines the stability of a peak-current-mode buck converter by analyzing the inductor current slopes. The rising slope 400000 A/s and falling slope 800000 A/s are derived from the input voltage, output voltage, and inductance. These values feed into the perturbation factor formula to yield -0.714286. If the magnitude of this factor is less than one, the loop is stable; otherwise, subharmonic oscillations occur. The minimum compensation ramp required for stability at this operating point is 200000 A/s, while the deadbeat slope that eliminates ringing entirely is 800000 A/s.

### Boost converter in continuous conduction

The duty cycle, average inductor (input) current, inductor ripple, peak and valley currents and switch voltage stress of an ideal boost (step-up) converter in continuous conduction.

Command line:

```console
$ power-electronics solve pe_boost_ccm "Vin=12 V" "Vout=48 V" "L=100 uH" "fs=100 kHz" "Io=1 A"
D = 0.75
IL = 4 A
dIL = 0.9 A
IL_max = 4.45 A
IL_min = 3.55 A
V_switch = 48 V
  by: Boost converter in continuous conduction  (D = 1 - Vin/Vout; IL = Io/(1 - D); dIL = Vin D/(L fs))
  Vin = 12 V
  Vout = 48 V
  L = 0.0001 H
  fs = 100000 Hz
  Io = 1 A
  assumes: Vout > Vin > 0; continuous conduction (IL_min > 0)
```

Python:

```python
import power_electronics_ai999x as m

a = m.solve("pe_boost_ccm", Vin="12 V", Vout="48 V", L="100 uH", fs="100 kHz", Io="1 A")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_boost_ccm) and press **Compute in your browser**.

_What this means:_ This calculation determines the operating parameters for an ideal boost converter in continuous conduction mode. The duty cycle, computed as 0.75, dictates the voltage conversion ratio based on the input and output voltages. From this, the average inductor current, which equals the input current, is found to be 4 A. The peak-to-peak inductor ripple current is calculated as 0.9 A, derived from the input voltage, duty cycle, inductance, and switching frequency. These values define the steady-state behavior of the converter.

### Boost converter in discontinuous conduction

The conversion ratio, output voltage, diode conduction interval D2, peak inductor current and load current of an ideal boost converter in discontinuous conduction.

Command line:

```console
$ power-electronics solve pe_boost_dcm "Vin=12 V" "L=10 uH" "fs=100 kHz" "R=100 ohm" D=0.4
K = 0.02
M = 3.372281323
Vout = 40.46737588 V
D2 = 0.1686140662
I_pk = 4.8 A
Io = 0.4046737588 A
  by: Boost converter in discontinuous conduction  (M = (1 + sqrt(1 + 4 D^2/K))/2; D2 = D/(M - 1); I_pk = Vin D Ts/L)
  Vin = 12 V
  L = 1e-05 H
  fs = 100000 Hz
  R = 100 ohm
  D = 0.4
  assumes: K < D (1 - D)^2 (discontinuous conduction)
```

Python:

```python
import power_electronics_ai999x as m

a = m.solve("pe_boost_dcm", Vin="12 V", L="10 uH", fs="100 kHz", R="100 ohm", D="0.4")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/power-electronics-ai999x/#cap-pe_boost_dcm) and press **Compute in your browser**.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/power-electronics-ai999x/blob/main/docs/power_electronics_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/power-electronics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified power electronics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 482 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `power-electronics` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** DC-DC converters, rectifiers, inverters and PWM, motor drives, device losses, thermal and magnetic design, resonant converters.

## Model overview

| Property | Value |
|---|---|
| Package | `power-electronics-ai999x` 1.6.0 (import `power_electronics_ai999x`) |
| Type | Deterministic neuro-symbolic power electronics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 79 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 482 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install power-electronics-ai999x
```

```python
import power_electronics_ai999x as power_electronics

power_electronics.ask("Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A").show()
# -> SOLVED — D = 0.208333, dIL = 0.899621, IL_max = 3.44981, IL_min = 2.55019  (pe_buck_ccm)
power_electronics.ask("Three-phase controlled rectifier: VLL = 415 V, alpha = 45 deg, Id = 50 A").show()
# -> SOLVED — Vdc = 396.296, Vrms = 415, P = 19814.8, I_thy_avg = 16.6667  (pe_three_phase_controlled_rectifier)
power_electronics.ask("Space vector dwell times: Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz").show()
# -> SOLVED — sector = 1, T1 = 3.31155e-05, T2 = 2.43999e-05, T0 = 4.24847e-05  (pe_space_vector_times)
power_electronics.solve("pe_flyback_ccm", Vin=48.0, Vout=12.0, n=2.0, Lm=0.0002, fs=100000.0, Io=2.0).show()
# -> SOLVED — D = 0.333333, Im = 1.5, dIm = 0.8, Ipk_primary = 1.9  (pe_flyback_ccm)
power_electronics.ask("Steinmetz core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3").show()
# -> SOLVED — Pv = 101193, Pv_mW_cm3 = 101.193, P = 0.505964  (pe_steinmetz_core_loss)
power_electronics.ask("Who created you?").message
# -> power-electronics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `power-electronics-ai999x "<question>"`; for the native program use `power-electronics` (above). The outputs above were produced by running this exact code on power-electronics-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `power-electronics-ai999x`; the answer is the one it returned, with the steps it took.

**Buck converter in continuous conduction** · _DC-DC converters_

> Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A

→ **D = 0.2083333333, dIL = 0.8996212121 A, IL_max = 3.449810606 A, IL_min = 2.550189394 A, ripple_ratio = 0.2998737374, Iin = 0.625 A**

<details><summary>Steps</summary>

- Buck converter in continuous conduction: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A → D = 0.2083333333 A, dIL = 0.8996212121 A, IL_max = 3.449810606 A, IL_min = 2.550189394 A, ripple_ratio = 0.2998737374 A, Iin = 0.625 A

</details>

**Three-phase fully controlled thyristor bridge** · _Rectifiers_

> Three-phase controlled rectifier: VLL = 415 V, alpha = 45 deg, Id = 50 A

→ **Vdc = 396.2958083 V, Vrms = 415 V, P = 19814.79041 W, I_thy_avg = 16.66666667 A, I_thy_rms = 28.86751346 A, Is_rms = 40.82482905 A, mode = rectifying**

<details><summary>Steps</summary>

- Three-phase fully controlled thyristor bridge: VLL = 415 V, alpha = 45 deg, Id = 50 A → Vdc = 396.2958083 V, Vrms = 415 V, P = 19814.79041 W, I_thy_avg = 16.66666667 A, I_thy_rms = 28.86751346 A, Is_rms = 40.82482905 A, mode = rectifying V

</details>

**Space-vector modulation dwell times** · _Inverters_

> Space vector dwell times: Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz

→ **sector = 1, T1 = 3.311545099e-05 s, T2 = 2.439987672e-05 s, T0 = 4.248467229e-05 s, m = 0.5773502692**

<details><summary>Steps</summary>

- Space-vector modulation dwell times: Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz → sector = 1 s, T1 = 3.311545099e-05 s, T2 = 2.439987672e-05 s, T0 = 4.248467229e-05 s, m = 0.5773502692 s

</details>

**Flyback converter in continuous conduction** · _Isolated converters_

`m.solve("pe_flyback_ccm", Vin=48.0, Vout=12.0, n=2.0, Lm=0.0002, fs=100000.0, Io=2.0)`

→ **D = 0.3333333333, Im = 1.5 A, dIm = 0.8 A, Ipk_primary = 1.9 A, Ipk_secondary = 3.8 A, V_switch = 72 V, V_diode = 36 V**

<details><summary>Steps</summary>

- Flyback converter in continuous conduction: Vin = 48 V, Vout = 12 V, n = 2, Lm = 0.0002 H, fs = 100000 Hz, Io = 2 A → D = 0.3333333333, Im = 1.5 A, dIm = 0.8 A, Ipk_primary = 1.9 A, Ipk_secondary = 3.8 A, V_switch = 72 V, V_diode = 36 V

</details>

**Core loss by the Steinmetz equation** · _Magnetics_

> Steinmetz core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3

→ **Pv = 101192.8851 W/m^3, Pv_mW_cm3 = 101.1928851, P = 0.5059644256 W**

<details><summary>Steps</summary>

- Core loss by the Steinmetz equation: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3 → Pv = 101192.8851 W/m^3, Pv_mW_cm3 = 101.1928851 W/m^3, P = 0.5059644256 W

</details>

**Control-to-output transfer function of a DC-DC converter** · _Control_

> Converter small-signal model: topology = boost, Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm

→ **Gvd0 = 48 V, f0 = 1125.395395 Hz, Q = 7.071067812, f_rhpz = 7957.747155 Hz, rhp_zero = present**

<details><summary>Steps</summary>

- Control-to-output transfer function of a DC-DC converter: topology = boost, Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm → Gvd0 = 48 V, f0 = 1125.395395 Hz, Q = 7.071067812 Hz, f_rhpz = 7957.747155 Hz, rhp_zero = present Hz

</details>

## Figures

Every point was computed by `power-electronics-ai999x`.

**Buck converter in continuous conduction**

![Buck converter in continuous conduction, computed by power-electronics-ai999x](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-buck-ccm-dil-vs-vin.svg)

_dIL (peak-to-peak inductor ripple) against Vin (input voltage), with Io = 3 A, L = 2.20e-5 H, Vout = 5 V, fs = 2.00e5 Hz. 31 points computed by power-electronics-ai999x; marked: Vin = 24, dIL = 0.899621 A._

**Boost power-factor-correction stage**

![Boost power-factor-correction stage, computed by power-electronics-ai999x](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-boost-pfc-di-max-vs-vac.svg)

_dI_max (largest inductor ripple over the line cycle) against Vac (RMS line voltage), with C = 3.30e-4 F, L = 5.00e-4 H, Po = 500 W, Vo = 400 V, eff = 0.95, f_line = 50 Hz, fs = 65000 Hz. 22 points computed by power-electronics-ai999x; marked: Vac = 230, dI_max = 3.07692 A._

**Flyback converter in continuous conduction**

![Flyback converter in continuous conduction, computed by power-electronics-ai999x](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-flyback-ccm-d-vs-vin.svg)

_D (duty cycle) against Vin (input voltage), with Io = 2 A, Lm = 2.00e-4 H, Vout = 12 V, fs = 1.00e5 Hz, n = 2. 31 points computed by power-electronics-ai999x; marked: Vin = 48, D = 0.333333._

**Peak current mode: subharmonic stability and slope compensation**

![Peak current mode: subharmonic stability and slope compensation, computed by power-electronics-ai999x](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-peak-current-mode-slope-alpha-vs-vin.svg)

_alpha (perturbation factor per switching period) against Vin (input voltage), with L = 1.00e-5 H, Vout = 8 V, fs = 3.00e5 Hz, ma = 3.00e5 A/s. 25 points computed by power-electronics-ai999x; marked: Vin = 12, alpha = -0.714286._

## Capabilities

79 certified procedures. A few of them:

- **Procedures:** Armature current ripple of a chopper-fed DC motor; Average and RMS of a rectangular pulse train; BLDC motor with six-step (120 degree) drive; Boost converter in continuous conduction; Boost converter in discontinuous conduction; Boost converter output voltage ripple; Boost converter with inductor resistance: gain and efficiency; Boost power-factor-correction stage; Bootstrap capacitor for a high-side gate driver; Buck converter efficiency with conduction and switching losses; Buck converter in continuous conduction; Buck converter in discontinuous conduction; Buck converter inductor and capacitor design; Buck converter output voltage ripple; Buck converter switch, diode, inductor and input-capacitor currents; Buck-boost converter in continuous conduction; Buck-boost converter in discontinuous conduction; Capacitor-input rectifier filter: exact ripple and peak diode current; Chopper-fed separately excited DC motor: speed and torque; Commutation overlap in a single-phase thyristor bridge; Commutation overlap in a three-phase thyristor bridge; Control-to-output transfer function of a DC-DC converter; Core loss by the Steinmetz equation; Core loss under triangular flux (iGSE) …

`power_electronics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified power electronics capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (DC-DC converters, rectifiers, inverters and PWM, motor drives, device losses, thermal and magnetic design, resonant converters); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{power_electronics_ai999x_1_6_0,
  title  = {power-electronics-ai999x: an offline, verified power electronics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/power-electronics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/power-electronics-ai999x/discussions or visit https://ai999x.com.

## Support the project

power-electronics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/power-electronics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/power-electronics-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `power-electronics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/power-electronics`) and its data (`rust/models/power_electronics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p power-electronics-ai999x
./target/release/power-electronics --help
cargo test -p power-electronics-ai999x
```
