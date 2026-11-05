# power-electronics: the Power Electronics program — user guide

**power-electronics-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Power Electronics covers DC-DC converters in continuous and discontinuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, bridge), rectifiers and phase control, inverters and PWM, motor drives, device losses, snubbers, thermal and magnetic design and resonant converters. This guide is about `power-electronics`, the model as a self-contained program for Windows and Linux: 79 capabilities in 13 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/power-electronics-ai999x/); the Python package (`pip install power-electronics-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Power Electronics | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-power-electronics-1.4.0-windows-x86_64.zip |
| Desktop app: Power Electronics | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-power-electronics-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Power Electronics in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-power-electronics-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-power-electronics.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-power-electronics-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-power-electronics-1.4.0-linux-x86_64.tar.gz
./ai999x-power-electronics-1.4.0-linux-x86_64/ai999x-power-electronics
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `power-electronics` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
power-electronics --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
power-electronics --version
```

### In your browser

Open https://www.ai999x.com/models/power-electronics-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install power-electronics-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ power-electronics --version
power-electronics-ai999x 1.0.0 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ power-electronics search "buck converter in continuous conduction"
pe_buck_ccm                          Buck converter in continuous conduction  (DC-DC converters / buck)
pe_buck_boost_ccm                    Buck-boost converter in continuous conduction  (DC-DC converters / buck-boost)
pe_boost_ccm                         Boost converter in continuous conduction  (DC-DC converters / boost)
pe_cuk_ccm                           Cuk converter in continuous conduction  (DC-DC converters / Cuk)
pe_flyback_ccm                       Flyback converter in continuous conduction  (isolated converters / flyback)
pe_sepic_ccm                         SEPIC converter in continuous conduction  (DC-DC converters / SEPIC)
pe_buck_dcm                          Buck converter in discontinuous conduction  (DC-DC converters / discontinuous conduction)
pe_buck_boost_dcm                    Buck-boost converter in discontinuous conduction  (DC-DC converters / discontinuous conduction)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ power-electronics list
pe_ac_voltage_controller                 Single-phase AC voltage controller (resistive load)
pe_bldc_six_step                         BLDC motor with six-step (120 degree) drive
pe_boost_ccm                             Boost converter in continuous conduction
pe_boost_dcm                             Boost converter in discontinuous conduction
pe_boost_nonideal_gain                   Boost converter with inductor resistance: gain and efficiency
pe_boost_output_ripple                   Boost converter output voltage ripple
pe_boost_pfc                             Boost power-factor-correction stage
pe_bootstrap_capacitor                   Bootstrap capacitor for a high-side gate driver
… 71 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ power-electronics solve pe_buck_ccm "Vin=24 V" "Vout=5 V" "L=22 uH" "fs=200 kHz" "Io=3 A" --json
{"outcome":"SOLVED","message":"","value":0.899621212121,"unit":"A","solved_for":"dIL","si":[["Vin",24.0],["Vout",5.0],["L",0.000022],["fs",200000.0],["Io",3.0]],"values":[["D",0.208333333333,""],["dIL",0.899621212121,"A"],["IL_max",3.44981060606,"A"],["IL_min",2.55018939394,"A"],["ripple_ratio",0.299873737374,""],["Iin",0.625,"A"]],"target":"dIL","phrase":"duty cycle","assumptions":["0 < Vout < Vin","continuous conduction (IL_min > 0)"],"name":"Buck converter in continuous conduction","equation":"D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ power-electronics solve pe_buck_ccm "Vin=24 V" "Vout=5 V" "L=22 uH" "fs=200 kHz"
NEEDS_INPUT: pe_buck_ccm needs Io (load (output) current). Example: solve('pe_buck_ccm', Vin='24 V', Vout='5 V', L='2.2e-05 H', fs='200000 Hz', Io='3 A').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ power-electronics figures pe_buck_ccm "Vin=24 V" "Vout=5 V" "L=22 uH" "fs=200 kHz" "Io=3 A" --out .
[
  {
    "slug": "pe-buck-ccm-dil-vs-vin",
    "title": "dIL against Vin: Buck converter in continuous conduction",
    "caption": "Peak-to-peak inductor ripple (dIL) against input voltage (Vin) from 12 V to 36 V, the other inputs as given: 31 points, each computed by the model (inputs it refuses are left out). Marked: the inputs given, Vin = 24 V, dIL = 899.6 mA. In SI units: dIL = 0.8996 A. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-dil-vs-vin.svg"
  },
  {
    "slug": "pe-buck-ccm-live-1",
    "title": "Valley inductor current as load (output) current varies",
    "caption": "Valley inductor current as Io runs from 0.75 to 5.25 A, every other input as you entered it (41 solutions). Marked: your Io = 3 A gives IL_min = 2.55. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-live-1.svg"
  },
  {
    "slug": "pe-buck-ccm-live-2",
    "title": "Peak-to-peak inductor ripple as input voltage varies",
    "caption": "Peak-to-peak inductor ripple as Vin runs from 6 to 42 V, every other input as you entered it (29 solutions). Marked: your Vin = 24 V gives dIL = 0.8996. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | power-electronics batch

`cases.jsonl`:

```json
{"key": "pe_buck_ccm", "inputs": {"Vin": "24 V", "Vout": "5 V", "L": "22 uH", "fs": "200 kHz", "Io": "3 A"}}
{"key": "pe_three_phase_controlled_rectifier", "inputs": {"VLL": "415 V", "alpha": "45 deg", "Id": "50 A"}}
```

```shell
$ power-electronics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.899621212121,"unit":"A","solved_for":"dIL","si":[["Vin",24.0],["Vout",5.0],["L",0.000022],["fs",200000.0],["Io",3.0]],"values":[["D",0.208333333333,""],["dIL",0.899621212121,"A"],["IL_max",3.44981060606,"A"],["IL_min",2.55018939394,"A"],["ripple_ratio",0.299873737374,""],["Iin",0.625,"A"]],"target":"dIL","phrase":"duty cycle","assumptions":["0 < Vout < Vin","continuous conduction (IL_min > 0)"],"name":"Buck converter in continuous conduction","equation":"D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io"}
{"outcome":"SOLVED","message":"","value":396.295808299,"unit":"V","solved_for":"Vdc","si":[["VLL",415.0],["alpha",0.7853981633974483],["Id",50.0]],"values":[["Vdc",396.295808299,"V"],["Vrms",415.0,"V"],["P",19814.7904149,"W"],["I_thy_avg",16.6666666667,"A"],["I_thy_rms",28.8675134595,"A"],["Is_rms",40.8248290464,"A"],["mode","rectifying",""]],"target":"Vdc","phrase":"average output voltage","assumptions":["0 <= alpha < 180 degrees","continuous constant current, no source inductance"],"name":"Three-phase fully controlled thyristor bridge","equation":"Vdc = 3 sqrt(2) VLL cos(alpha)/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3) cos(2 alpha)/(4 pi)); I_thy = Id/3 (avg), Id/sqrt(3) (rms)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `power-electronics info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`power-electronics solve ... --json` prints one answer as one line of JSON. `power-electronics batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | power-electronics batch`. `power-electronics catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["power-electronics", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`power-electronics figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ power-electronics figures pe_buck_ccm "Vin=24 V" "Vout=5 V" "L=22 uH" "fs=200 kHz" "Io=3 A" --out .
[
  {
    "slug": "pe-buck-ccm-dil-vs-vin",
    "title": "dIL against Vin: Buck converter in continuous conduction",
    "caption": "Peak-to-peak inductor ripple (dIL) against input voltage (Vin) from 12 V to 36 V, the other inputs as given: 31 points, each computed by the model (inputs it refuses are left out). Marked: the inputs given, Vin = 24 V, dIL = 899.6 mA. In SI units: dIL = 0.8996 A. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-dil-vs-vin.svg"
  },
  {
    "slug": "pe-buck-ccm-live-1",
    "title": "Valley inductor current as load (output) current varies",
    "caption": "Valley inductor current as Io runs from 0.75 to 5.25 A, every other input as you entered it (41 solutions). Marked: your Io = 3 A gives IL_min = 2.55. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-live-1.svg"
  },
  {
    "slug": "pe-buck-ccm-live-2",
    "title": "Peak-to-peak inductor ripple as input voltage varies",
    "caption": "Peak-to-peak inductor ripple as Vin runs from 6 to 42 V, every other input as you entered it (29 solutions). Marked: your Vin = 24 V gives dIL = 0.8996. Your inputs: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A.",
    "file": ".\\pe-buck-ccm-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `power_electronics_ai999x.wasm` (https://www.ai999x.com/compute/rust/power_electronics_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("power_electronics_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `power-electronics info <id>` shows the same inputs and outputs in the terminal.

### Control (2)

<a id="pe_converter_small_signal"></a>

#### Control-to-output transfer function of a DC-DC converter — `pe_converter_small_signal`

The control-to-output (duty-to-voltage) transfer function features of an ideal buck, boost or buck-boost converter in continuous conduction: dc gain, double-pole frequency, quality factor and right-half-plane zero.

$$
G_{vd}(s) = G_{d0}\frac{1 - s/\omega_z}{1 + s/(Q\omega_0) + (s/\omega_0)^2},\quad \omega_{z,boost} = \frac{D'^2R}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `topology` | in | buck, boost or buck-boost | — |
| `Vin` | in | input voltage | V |
| `D` | in | duty cycle (a fraction or %) | — |
| `L` | in | inductance | H |
| `C` | in | output capacitance | F |
| `R` | in | load resistance | ohm |
| `Gvd0` | out | dc gain of vo/d (volts per unit duty) | V |
| `f0` | out | double-pole (corner) frequency | Hz |
| `Q` | out | quality factor of the double pole | — |
| `f_rhpz` | out | right-half-plane zero frequency | Hz |
| `rhp_zero` | out | whether a right-half-plane zero is present | — |

**Applies when:** ideal converter in continuous conduction, no ESR.

```shell
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

In the worked example, topology = boost, Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm go through the equation above and give Gvd0 = 48 V, f0 = 1125.395395 Hz, Q = 7.071067812, f_rhpz = 7957.747155 Hz. It applies only when ideal converter in continuous conduction, no ESR: outside that it says what is wrong instead of returning a number.

<a id="pe_peak_current_mode_slope"></a>

#### Peak current mode: subharmonic stability and slope compensation — `pe_peak_current_mode_slope`

The inductor current slopes, the period-to-period perturbation factor, the stability verdict and the smallest and the deadbeat compensation ramp slopes of a peak-current-mode buck converter.

$$
\alpha = -\frac{m_2 - m_a}{m_1 + m_a},\quad |\alpha| < 1 \Leftarrow m_a > \frac{m_2-m_1}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `ma` | in | compensation ramp slope referred to the inductor current | A/s |
| `D` | out | duty cycle | — |
| `m1` | out | rising inductor current slope | A/s |
| `m2` | out | falling inductor current slope (magnitude) | A/s |
| `alpha` | out | perturbation factor per switching period | — |
| `stable` | out | stable or unstable | — |
| `ma_min` | out | smallest ramp slope for stability at this duty cycle | A/s |
| `ma_deadbeat` | out | deadbeat compensation ramp slope | A/s |

**Applies when:** 0 < Vout < Vin; continuous conduction.

```shell
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

In the worked example, Vin = 12 V, Vout = 8 V, L = 10 uH, fs = 300 kHz, ma = 0.3 A/us go through the equation above and give D = 0.6666666667, m1 = 400000 A/s, m2 = 800000 A/s, alpha = -0.7142857143. It applies only when 0 < Vout < Vin; continuous conduction: outside that it says what is wrong instead of returning a number.

### DC-DC converters (15)

<a id="pe_boost_ccm"></a>

#### Boost converter in continuous conduction — `pe_boost_ccm`

The duty cycle, average inductor (input) current, inductor ripple, peak and valley currents and switch voltage stress of an ideal boost (step-up) converter in continuous conduction.

$$
D = 1-\frac{V_{in}}{V_o},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load (output) current | A |
| `D` | out | duty cycle | — |
| `IL` | out | average inductor current (input current) | A |
| `dIL` | out | peak-to-peak inductor ripple | A |
| `IL_max` | out | peak inductor current | A |
| `IL_min` | out | valley inductor current | A |
| `V_switch` | out | switch and diode voltage stress | V |

**Applies when:** Vout > Vin > 0; continuous conduction (IL_min > 0).

```shell
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

In the worked example, Vin = 12 V, Vout = 48 V, L = 100 uH, fs = 100 kHz, Io = 1 A go through the equation above and give D = 0.75, IL = 4 A, dIL = 0.9 A, IL_max = 4.45 A. It applies only when Vout > Vin > 0; continuous conduction (IL_min > 0): outside that it says what is wrong instead of returning a number.

<a id="pe_boost_dcm"></a>

#### Boost converter in discontinuous conduction — `pe_boost_dcm`

The conversion ratio, output voltage, diode conduction interval D2, peak inductor current and load current of an ideal boost converter in discontinuous conduction.

$$
M = \frac{1+\sqrt{1+4D^2/K}}{2},\quad D_2 = \frac{D}{M-1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `R` | in | load resistance | ohm |
| `D` | in | duty cycle (a fraction or %) | — |
| `K` | out | dimensionless conduction parameter K | — |
| `M` | out | conversion ratio Vout/Vin | — |
| `Vout` | out | output voltage | V |
| `D2` | out | diode conduction interval as a fraction of the period | — |
| `I_pk` | out | peak inductor current | A |
| `Io` | out | load current | A |

**Applies when:** K < D (1 - D)^2 (discontinuous conduction).

```shell
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

In the worked example, Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 100 ohm, D = 0.4 go through the equation above and give K = 0.02, M = 3.372281323, Vout = 40.46737588 V, D2 = 0.1686140662. It applies only when K < D (1 - D)^2 (discontinuous conduction): outside that it says what is wrong instead of returning a number.

<a id="pe_boost_output_ripple"></a>

#### Boost converter output voltage ripple — `pe_boost_output_ripple`

The peak-to-peak output voltage ripple of a boost converter: the capacitor alone supplies the load current during the switch on-time (ideal capacitor, no ESR).

$$
\Delta v_o = \frac{I_o D}{C f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `Io` | in | load current | A |
| `C` | in | output capacitance | F |
| `fs` | in | switching frequency | Hz |
| `D` | out | duty cycle | — |
| `dVo` | out | peak-to-peak output ripple | V |
| `dVo_pct` | out | ripple in percent of Vout | — |

**Applies when:** Vout > Vin > 0; continuous conduction, ideal capacitor.

```shell
$ power-electronics solve pe_boost_output_ripple "Vin=12 V" "Vout=48 V" "Io=1 A" "C=47 uF" "fs=100 kHz"
D = 0.75
dVo = 0.1595744681 V
dVo_pct = 0.3324468085
  by: Boost converter output voltage ripple  (D = 1 - Vin/Vout; dVo = Io D/(C fs))
  Vin = 12 V
  Vout = 48 V
  Io = 1 A
  C = 4.7e-05 F
  fs = 100000 Hz
  assumes: Vout > Vin > 0; continuous conduction, ideal capacitor
```

In the worked example, Vin = 12 V, Vout = 48 V, Io = 1 A, C = 47 uF, fs = 100 kHz go through the equation above and give D = 0.75, dVo = 0.1595744681 V, dVo_pct = 0.3324468085. It applies only when Vout > Vin > 0; continuous conduction, ideal capacitor: outside that it says what is wrong instead of returning a number.

<a id="pe_buck_ccm"></a>

#### Buck converter in continuous conduction — `pe_buck_ccm`

The duty cycle, inductor ripple, peak and valley inductor currents, ripple ratio and average input current of an ideal buck (step-down) converter in continuous conduction.

$$
D = \frac{V_o}{V_{in}},\quad \Delta i_L = \frac{(V_{in}-V_o)D}{L f_s},\quad I_{L,\max/\min} = I_o \pm \tfrac{\Delta i_L}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load (output) current | A |
| `D` | out | duty cycle | — |
| `dIL` | out | peak-to-peak inductor ripple | A |
| `IL_max` | out | peak inductor current | A |
| `IL_min` | out | valley inductor current | A |
| `ripple_ratio` | out | ripple ratio dIL/Io | — |
| `Iin` | out | average input current | A |

**Applies when:** 0 < Vout < Vin; continuous conduction (IL_min > 0).

```shell
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

In the worked example, Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A go through the equation above and give D = 0.2083333333, dIL = 0.8996212121 A, IL_max = 3.449810606 A, IL_min = 2.550189394 A. It applies only when 0 < Vout < Vin; continuous conduction (IL_min > 0): outside that it says what is wrong instead of returning a number.

<a id="pe_buck_dcm"></a>

#### Buck converter in discontinuous conduction — `pe_buck_dcm`

The conversion ratio, output voltage, second-interval duty D2 (diode conduction), peak inductor current and load current of an ideal buck converter in discontinuous conduction.

$$
M = \frac{2}{1+\sqrt{1+4K/D^2}},\quad K = \frac{2L}{R T_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `R` | in | load resistance | ohm |
| `D` | in | duty cycle (a fraction or %) | — |
| `K` | out | dimensionless conduction parameter K | — |
| `M` | out | conversion ratio Vout/Vin | — |
| `Vout` | out | output voltage | V |
| `D2` | out | diode conduction interval as a fraction of the period | — |
| `I_pk` | out | peak inductor current | A |
| `Io` | out | load current | A |

**Applies when:** K < 1 - D (discontinuous conduction).

```shell
$ power-electronics solve pe_buck_dcm "Vin=24 V" "L=10 uH" "fs=100 kHz" "R=20 ohm" D=0.3
K = 0.1
M = 0.6
Vout = 14.4 V
D2 = 0.2
I_pk = 2.88 A
Io = 0.72 A
  by: Buck converter in discontinuous conduction  (K = 2L/(R Ts); M = 2/(1 + sqrt(1 + 4K/D^2)); D2 = D (1 - M)/M; I_pk = (Vin - Vo) D Ts/L)
  Vin = 24 V
  L = 1e-05 H
  fs = 100000 Hz
  R = 20 ohm
  D = 0.3
  assumes: K < 1 - D (discontinuous conduction)
```

In the worked example, Vin = 24 V, L = 10 uH, fs = 100 kHz, R = 20 ohm, D = 0.3 go through the equation above and give K = 0.1, M = 0.6, Vout = 14.4 V, D2 = 0.2. It applies only when K < 1 - D (discontinuous conduction): outside that it says what is wrong instead of returning a number.

<a id="pe_buck_lc_design"></a>

#### Buck converter inductor and capacitor design — `pe_buck_lc_design`

The inductance that gives a chosen ripple ratio and the output capacitance that limits the output ripple to a chosen value in a buck converter (continuous conduction, ideal capacitor).

$$
L = \frac{(V_{in}-V_o)D}{r I_o f_s},\quad C = \frac{r I_o}{8 f_s \Delta v_o}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `Io` | in | load current | A |
| `fs` | in | switching frequency | Hz |
| `r` | in | ripple ratio dIL/Io (a fraction or %) | — |
| `dV` | in | allowed peak-to-peak output ripple | V |
| `D` | out | duty cycle | — |
| `L` | out | required inductance | H |
| `C` | out | required output capacitance | F |
| `IL_max` | out | peak inductor current | A |
| `dIL` | out | inductor ripple | A |

**Applies when:** 0 < Vout < Vin; 0 < r < 2.

```shell
$ power-electronics solve pe_buck_lc_design "Vin=12 V" "Vout=3.3 V" "Io=5 A" "fs=500 kHz" r=0.3 "dV=0.01 V"
D = 0.275
L = 3.19e-06 H
C = 3.75e-05 F
IL_max = 5.75 A
dIL = 1.5 A
  by: Buck converter inductor and capacitor design  (L = (Vin - Vo) D/(r Io fs); C = r Io/(8 fs dV))
  Vin = 12 V
  Vout = 3.3 V
  Io = 5 A
  fs = 500000 Hz
  r = 0.3
  dV = 0.01 V
  assumes: 0 < Vout < Vin; 0 < r < 2
```

In the worked example, Vin = 12 V, Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 0.3, dV = 0.01 V go through the equation above and give D = 0.275, L = 3.19e-06 H, C = 3.75e-05 F, IL_max = 5.75 A. It applies only when 0 < Vout < Vin; 0 < r < 2: outside that it says what is wrong instead of returning a number.

<a id="pe_buck_output_ripple"></a>

#### Buck converter output voltage ripple — `pe_buck_output_ripple`

The peak-to-peak output voltage ripple of a buck converter in continuous conduction, with the inductor ripple current flowing entirely into the output capacitor (no ESR).

$$
\Delta v_o = \frac{\Delta i_L}{8 C f_s} = \frac{(1-D)V_o}{8 L C f_s^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L` | in | inductance | H |
| `C` | in | output capacitance | F |
| `fs` | in | switching frequency | Hz |
| `D` | out | duty cycle | — |
| `dIL` | out | inductor ripple | A |
| `dVo` | out | peak-to-peak output ripple | V |
| `dVo_pct` | out | output ripple in percent of Vout | — |

**Applies when:** 0 < Vout < Vin; continuous conduction; ripple current into C, ideal capacitor.

```shell
$ power-electronics solve pe_buck_output_ripple "Vin=24 V" "Vout=5 V" "L=22 uH" "C=100 uF" "fs=200 kHz"
D = 0.2083333333
dIL = 0.8996212121 A
dVo = 0.005622632576 V
dVo_pct = 0.1124526515
  by: Buck converter output voltage ripple  (dIL = (Vin - Vout) D/(L fs); dVo = dIL/(8 C fs) = (1 - D) Vout/(8 L C fs^2))
  Vin = 24 V
  Vout = 5 V
  L = 2.2e-05 H
  C = 0.0001 F
  fs = 200000 Hz
  assumes: 0 < Vout < Vin; continuous conduction; ripple current into C, ideal capacitor
```

In the worked example, Vin = 24 V, Vout = 5 V, L = 22 uH, C = 100 uF, fs = 200 kHz go through the equation above and give D = 0.2083333333, dIL = 0.8996212121 A, dVo = 0.005622632576 V, dVo_pct = 0.1124526515. It applies only when 0 < Vout < Vin; continuous conduction; ripple current into C, ideal capacitor: outside that it says what is wrong instead of returning a number.

<a id="pe_buck_component_currents"></a>

#### Buck converter switch, diode, inductor and input-capacitor currents — `pe_buck_component_currents`

The average and RMS currents of the switch and the diode, the RMS inductor current, the peak current and the RMS input-capacitor current of a buck converter in continuous conduction (trapezoidal waveforms with ripple).

$$
I_{sw,rms} = \sqrt{D\left(I_o^2 + \tfrac{\Delta i_L^2}{12}\right)},\quad I_{C_{in},rms} = \sqrt{I_{sw,rms}^2 - (D I_o)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `Io` | in | load current | A |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `D` | out | duty cycle | — |
| `I_sw_avg` | out | average switch current | A |
| `I_sw_rms` | out | RMS switch current | A |
| `I_d_avg` | out | average diode current | A |
| `I_d_rms` | out | RMS diode current | A |
| `I_L_rms` | out | RMS inductor current | A |
| `I_pk` | out | peak switch and inductor current | A |
| `I_cin_rms` | out | RMS input-capacitor ripple current | A |

**Applies when:** 0 < Vout < Vin; continuous conduction.

```shell
$ power-electronics solve pe_buck_component_currents "Vin=24 V" "Vout=5 V" "Io=3 A" "L=22 uH" "fs=200 kHz"
D = 0.2083333333
I_sw_avg = 0.625 A
I_sw_rms = 1.374427395 A
I_d_avg = 2.375 A
I_d_rms = 2.679252233 A
I_L_rms = 3.011219553 A
I_pk = 3.449810606 A
I_cin_rms = 1.224101983 A
  by: Buck converter switch, diode, inductor and input-capacitor currents  (I_sw,rms = sqrt(D (Io^2 + dI^2/12)); I_d,rms = sqrt((1 - D)(Io^2 + dI^2/12)); I_cin,rms = sqrt(I_sw,rms^2 - (D Io)^2))
  Vin = 24 V
  Vout = 5 V
  Io = 3 A
  L = 2.2e-05 H
  fs = 200000 Hz
  assumes: 0 < Vout < Vin; continuous conduction
```

In the worked example, Vin = 24 V, Vout = 5 V, Io = 3 A, L = 22 uH, fs = 200 kHz go through the equation above and give D = 0.2083333333, I_sw_avg = 0.625 A, I_sw_rms = 1.374427395 A, I_d_avg = 2.375 A. It applies only when 0 < Vout < Vin; continuous conduction: outside that it says what is wrong instead of returning a number.

<a id="pe_buck_boost_ccm"></a>

#### Buck-boost converter in continuous conduction — `pe_buck_boost_ccm`

The duty cycle, average inductor current, ripple, peak and valley currents and switch voltage stress of an ideal inverting buck-boost converter in continuous conduction (output voltage as a magnitude).

$$
D = \frac{|V_o|}{V_{in}+|V_o|},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage magnitude (the output is inverted) | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load (output) current | A |
| `D` | out | duty cycle | — |
| `IL` | out | average inductor current | A |
| `dIL` | out | peak-to-peak inductor ripple | A |
| `IL_max` | out | peak inductor current | A |
| `IL_min` | out | valley inductor current | A |
| `V_switch` | out | switch voltage stress Vin + Vout | V |

**Applies when:** Vin, Vout > 0; continuous conduction (IL_min > 0).

```shell
$ power-electronics solve pe_buck_boost_ccm "Vin=12 V" "Vout=15 V" "L=47 uH" "fs=100 kHz" "Io=1 A"
D = 0.5555555556
IL = 2.25 A
dIL = 1.418439716 A
IL_max = 2.959219858 A
IL_min = 1.540780142 A
V_switch = 27 V
  by: Buck-boost converter in continuous conduction  (D = Vout/(Vin + Vout); IL = Io/(1 - D); dIL = Vin D/(L fs))
  Vin = 12 V
  Vout = 15 V
  L = 4.7e-05 H
  fs = 100000 Hz
  Io = 1 A
  assumes: Vin, Vout > 0; continuous conduction (IL_min > 0)
```

In the worked example, Vin = 12 V, Vout = 15 V, L = 47 uH, fs = 100 kHz, Io = 1 A go through the equation above and give D = 0.5555555556, IL = 2.25 A, dIL = 1.418439716 A, IL_max = 2.959219858 A. It applies only when Vin, Vout > 0; continuous conduction (IL_min > 0): outside that it says what is wrong instead of returning a number.

<a id="pe_buck_boost_dcm"></a>

#### Buck-boost converter in discontinuous conduction — `pe_buck_boost_dcm`

The conversion ratio (magnitude), output voltage, diode conduction interval, peak inductor current and load current of an ideal inverting buck-boost converter in discontinuous conduction.

$$
M = \frac{D}{\sqrt{K}},\quad D_2 = \sqrt{K}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `L` | in | inductance | H |
| `fs` | in | switching frequency | Hz |
| `R` | in | load resistance | ohm |
| `D` | in | duty cycle (a fraction or %) | — |
| `K` | out | dimensionless conduction parameter K | — |
| `M` | out | conversion ratio \|Vout\|/Vin | — |
| `Vout` | out | output voltage magnitude | V |
| `D2` | out | diode conduction interval (= sqrt K) | — |
| `I_pk` | out | peak inductor current | A |
| `Io` | out | load current | A |

**Applies when:** K < (1 - D)^2 (discontinuous conduction).

```shell
$ power-electronics solve pe_buck_boost_dcm "Vin=12 V" "L=10 uH" "fs=100 kHz" "R=50 ohm" D=0.4
K = 0.04
M = 2
Vout = 24 V
D2 = 0.2
I_pk = 4.8 A
Io = 0.48 A
  by: Buck-boost converter in discontinuous conduction  (M = D/sqrt(K); D2 = sqrt(K); I_pk = Vin D Ts/L)
  Vin = 12 V
  L = 1e-05 H
  fs = 100000 Hz
  R = 50 ohm
  D = 0.4
  assumes: K < (1 - D)^2 (discontinuous conduction)
```

In the worked example, Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 50 ohm, D = 0.4 go through the equation above and give K = 0.04, M = 2, Vout = 24 V, D2 = 0.2. It applies only when K < (1 - D)^2 (discontinuous conduction): outside that it says what is wrong instead of returning a number.

<a id="pe_critical_inductance"></a>

#### Critical inductance at the CCM/DCM boundary — `pe_critical_inductance`

The smallest inductance that keeps a buck, boost or buck-boost converter in continuous conduction at a load resistance (the inductor current just reaches zero at the end of each period).

$$
L_{c,buck} = \frac{(1-D)R}{2f_s},\ L_{c,boost} = \frac{D(1-D)^2R}{2f_s},\ L_{c,buck\text{-}boost} = \frac{(1-D)^2R}{2f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `topology` | in | buck, boost or buck-boost | — |
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage (magnitude) | V |
| `fs` | in | switching frequency | Hz |
| `R` | in | load resistance | ohm |
| `D` | out | duty cycle | — |
| `L_crit` | out | critical (boundary) inductance | H |
| `Io` | out | load current | A |

**Applies when:** ideal converter at the boundary of continuous conduction.

```shell
$ power-electronics solve pe_critical_inductance topology=buck "Vin=24 V" "Vout=12 V" "fs=100 kHz" "R=10 ohm"
D = 0.5
L_crit = 2.5e-05 H
Io = 1.2 A
  by: Critical inductance at the CCM/DCM boundary  (buck Lc = (1 - D) R/(2 fs); boost Lc = D (1 - D)^2 R/(2 fs); buck-boost Lc = (1 - D)^2 R/(2 fs))
  Vin = 24 V
  Vout = 12 V
  fs = 100000 Hz
  R = 10 ohm
  topology = buck
  assumes: ideal converter at the boundary of continuous conduction
```

In the worked example, topology = buck, Vin = 24 V, Vout = 12 V, fs = 100 kHz, R = 10 ohm go through the equation above and give D = 0.5, L_crit = 2.5e-05 H, Io = 1.2 A. It applies only when ideal converter at the boundary of continuous conduction: outside that it says what is wrong instead of returning a number.

<a id="pe_cuk_ccm"></a>

#### Cuk converter in continuous conduction — `pe_cuk_ccm`

The duty cycle, inductor currents and ripples, coupling-capacitor voltage and its ripple of an ideal Cuk converter (inverted output, given as a magnitude) in continuous conduction.

$$
D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in}+V_o,\ \Delta i_{L1} = \frac{V_{in}D}{L_1 f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage magnitude | V |
| `L1` | in | input inductance | H |
| `L2` | in | output inductance | H |
| `C1` | in | coupling (energy-transfer) capacitance | F |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load current | A |
| `D` | out | duty cycle | — |
| `IL1` | out | average input inductor current | A |
| `IL2` | out | average output inductor current | A |
| `VC1` | out | average coupling-capacitor voltage Vin + Vout | V |
| `dIL1` | out | input inductor ripple | A |
| `dIL2` | out | output inductor ripple | A |
| `dVC1` | out | coupling-capacitor ripple | V |

**Applies when:** continuous conduction in both inductors.

```shell
$ power-electronics solve pe_cuk_ccm "Vin=12 V" "Vout=15 V" "L1=100 uH" "L2=100 uH" "C1=10 uF" "fs=100 kHz" "Io=1 A"
D = 0.5555555556
IL1 = 1.25 A
IL2 = 1 A
VC1 = 27 V
dIL1 = 0.6666666667 A
dIL2 = 0.6666666667 A
dVC1 = 0.5555555556 V
  by: Cuk converter in continuous conduction  (D = Vo/(Vin + Vo); IL1 = Io Vo/Vin; VC1 = Vin + Vo; dIL1 = Vin D/(L1 fs); dIL2 = Vo (1 - D)/(L2 fs); dVC1 = Io D/(C1 fs))
  Vin = 12 V
  Vout = 15 V
  L1 = 0.0001 H
  L2 = 0.0001 H
  C1 = 1e-05 F
  fs = 100000 Hz
  Io = 1 A
  assumes: continuous conduction in both inductors
```

In the worked example, Vin = 12 V, Vout = 15 V, L1 = 100 uH, L2 = 100 uH, C1 = 10 uF, fs = 100 kHz, Io = 1 A go through the equation above and give D = 0.5555555556, IL1 = 1.25 A, IL2 = 1 A, VC1 = 27 V. It applies only when continuous conduction in both inductors: outside that it says what is wrong instead of returning a number.

<a id="pe_conversion_ratio"></a>

#### Ideal DC-DC conversion ratio from the duty cycle — `pe_conversion_ratio`

The ideal continuous-conduction voltage conversion ratio M = Vout/Vin and output voltage of a buck, boost, buck-boost, Cuk or SEPIC converter at a duty cycle, with the output polarity.

$$
M_{buck} = D,\quad M_{boost} = \frac{1}{1-D},\quad M_{buck\text{-}boost,\,Cuk,\,SEPIC} = \frac{D}{1-D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `topology` | in | converter topology: buck, boost, buck-boost, Cuk or SEPIC | — |
| `Vin` | in | input voltage | V |
| `D` | in | duty cycle (a fraction or %) | — |
| `M` | out | conversion ratio Vout/Vin (magnitude) | — |
| `Vout` | out | output voltage magnitude | V |
| `polarity` | out | output polarity relative to the input (same or inverted) | — |

**Applies when:** 0 < D < 1; ideal, continuous conduction.

```shell
$ power-electronics solve pe_conversion_ratio topology=boost "Vin=12 V" D=0.6
M = 2.5
Vout = 30 V
polarity = same
  by: Ideal DC-DC conversion ratio from the duty cycle  (buck M = D; boost M = 1/(1 - D); buck-boost, Cuk, SEPIC M = D/(1 - D))
  Vin = 12 V
  D = 0.6
  topology = boost
  assumes: 0 < D < 1; ideal, continuous conduction
```

In the worked example, topology = boost, Vin = 12 V, D = 0.6 go through the equation above and give M = 2.5, Vout = 30 V, polarity = same. It applies only when 0 < D < 1; ideal, continuous conduction: outside that it says what is wrong instead of returning a number.

<a id="pe_interleaved_buck_ripple"></a>

#### Interleaved multiphase buck: output current ripple cancellation — `pe_interleaved_buck_ripple`

The per-phase and total output current ripple of an N-phase interleaved buck converter (phases shifted by T/N, equal inductors), the cancellation factor and the ripple frequency N fs.

$$
\frac{\Delta i_{tot}}{\Delta i_{ph}} = \frac{N\left(D-\frac{m}{N}\right)\left(\frac{m+1}{N}-D\right)}{D(1-D)},\ m = \lfloor ND\rfloor
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of phases | — |
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L` | in | inductance of each phase | H |
| `fs` | in | switching frequency of each phase | Hz |
| `D` | out | duty cycle | — |
| `dI_phase` | out | ripple of one phase | A |
| `dI_total` | out | ripple of the summed output current | A |
| `cancellation` | out | ratio of total to per-phase ripple | — |
| `f_ripple` | out | output ripple frequency N fs | Hz |

**Applies when:** 2 <= N <= 12; 0 < Vout < Vin; continuous conduction in each phase.

```shell
$ power-electronics solve pe_interleaved_buck_ripple N=2 "Vin=12 V" "Vout=1.2 V" "L=0.47 uH" "fs=500 kHz"
D = 0.1
dI_phase = 4.595744681 A
dI_total = 4.085106383 A
cancellation = 0.8888888889
f_ripple = 1000000 Hz
  by: Interleaved multiphase buck: output current ripple cancellation  (m = floor(N D); dI_total/dI_phase = N (D - m/N)((m + 1)/N - D)/(D (1 - D)); zero when N D is a whole number)
  Vin = 12 V
  Vout = 1.2 V
  L = 4.7e-07 H
  fs = 500000 Hz
  assumes: 2 <= N <= 12; 0 < Vout < Vin; continuous conduction in each phase
```

In the worked example, N = 2, Vin = 12 V, Vout = 1.2 V, L = 0.47 uH, fs = 500 kHz go through the equation above and give D = 0.1, dI_phase = 4.595744681 A, dI_total = 4.085106383 A, cancellation = 0.8888888889. It applies only when 2 <= N <= 12; 0 < Vout < Vin; continuous conduction in each phase: outside that it says what is wrong instead of returning a number.

<a id="pe_sepic_ccm"></a>

#### SEPIC converter in continuous conduction — `pe_sepic_ccm`

The duty cycle, inductor currents and ripples, coupling-capacitor voltage and ripple and switch voltage stress of an ideal SEPIC (non-inverting buck-boost) converter in continuous conduction.

$$
D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in},\ \Delta i_{L1,2} = \frac{V_{in}D}{L_{1,2} f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `L1` | in | input inductance | H |
| `L2` | in | second inductance | H |
| `C1` | in | coupling capacitance | F |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load current | A |
| `D` | out | duty cycle | — |
| `IL1` | out | average input inductor current | A |
| `IL2` | out | average second inductor current | A |
| `VC1` | out | average coupling-capacitor voltage (= Vin) | V |
| `dIL1` | out | input inductor ripple | A |
| `dIL2` | out | second inductor ripple | A |
| `dVC1` | out | coupling-capacitor ripple | V |
| `V_switch` | out | switch voltage stress Vin + Vout | V |

**Applies when:** continuous conduction in both inductors.

```shell
$ power-electronics solve pe_sepic_ccm "Vin=9 V" "Vout=12 V" "L1=47 uH" "L2=47 uH" "C1=10 uF" "fs=200 kHz" "Io=1 A"
D = 0.5714285714
IL1 = 1.333333333 A
IL2 = 1 A
VC1 = 9 V
dIL1 = 0.547112462 A
dIL2 = 0.547112462 A
dVC1 = 0.2857142857 V
V_switch = 21 V
  by: SEPIC converter in continuous conduction  (D = Vo/(Vin + Vo); IL1 = Io Vo/Vin; IL2 = Io; VC1 = Vin; dIL1 = Vin D/(L1 fs); dIL2 = Vin D/(L2 fs); dVC1 = Io D/(C1 fs))
  Vin = 9 V
  Vout = 12 V
  L1 = 4.7e-05 H
  L2 = 4.7e-05 H
  C1 = 1e-05 F
  fs = 200000 Hz
  Io = 1 A
  assumes: continuous conduction in both inductors
```

In the worked example, Vin = 9 V, Vout = 12 V, L1 = 47 uH, L2 = 47 uH, C1 = 10 uF, fs = 200 kHz, Io = 1 A go through the equation above and give D = 0.5714285714, IL1 = 1.333333333 A, IL2 = 1 A, VC1 = 9 V. It applies only when continuous conduction in both inductors: outside that it says what is wrong instead of returning a number.

### Devices (9)

<a id="pe_bootstrap_capacitor"></a>

#### Bootstrap capacitor for a high-side gate driver — `pe_bootstrap_capacitor`

The charge drawn from a bootstrap capacitor during the longest high-side on-time and the smallest capacitance that keeps its droop within a limit, with the usual tenfold design margin.

$$
C_{boot} \ge \frac{Q_g + I_{lk}D_{max}/f_s}{\Delta V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qg` | in | high-side gate charge | C |
| `Ileak` | in | quiescent plus leakage current of the high side | A |
| `fs` | in | switching frequency | Hz |
| `Dmax` | in | maximum high-side duty cycle (a fraction or %) | — |
| `dV` | in | allowed bootstrap droop | V |
| `Q_total` | out | charge drawn per cycle | C |
| `C_min` | out | smallest bootstrap capacitance | F |
| `C_recommended` | out | tenfold margin | F |

**Applies when:** the capacitor recharges fully while the low side is on.

```shell
$ power-electronics solve pe_bootstrap_capacitor "Qg=60 nC" "Ileak=200 uA" "fs=100 kHz" Dmax=0.9 "dV=0.5 V"
Q_total = 6.18e-08 C
C_min = 1.236e-07 F
C_recommended = 1.236e-06 F
  by: Bootstrap capacitor for a high-side gate driver  (Q = Qg + I_leak Dmax/fs; C_min = Q/dV)
  Qg = 6e-08 C
  Ileak = 0.0002 A
  fs = 100000 Hz
  Dmax = 0.9
  dV = 0.5 V
  assumes: the capacitor recharges fully while the low side is on
```

In the worked example, Qg = 60 nC, Ileak = 200 uA, fs = 100 kHz, Dmax = 0.9, dV = 0.5 V go through the equation above and give Q_total = 6.18e-08 C, C_min = 1.236e-07 F, C_recommended = 1.236e-06 F. It applies only when the capacitor recharges fully while the low side is on: outside that it says what is wrong instead of returning a number.

<a id="pe_parallel_mosfet_sharing"></a>

#### Current sharing between paralleled MOSFETs — `pe_parallel_mosfet_sharing`

The current and conduction loss of each of several paralleled MOSFETs (or any resistive devices) sharing a total current in proportion to their on-state conductances, the total loss and the worst-case current imbalance.

$$
I_k = I\frac{1/R_k}{\sum_j 1/R_j},\quad P_k = I_k^2R_k
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | on-resistance of each device in milliohms | — |
| `I` | in | total current | A |
| `currents` | out | current in each device, amperes | — |
| `losses` | out | conduction loss of each device, watts | — |
| `P_total` | out | total conduction loss | W |
| `imbalance` | out | largest current divided by the equal share | — |

**Applies when:** static sharing (on-resistance only).

```shell
$ power-electronics solve pe_parallel_mosfet_sharing "R=[10, 12]" "I=40 A"
currents = [21.8181818182,18.1818181818]
losses = [4.76033057851,3.96694214876]
P_total = 8.727272727 W
imbalance = 1.090909091
  by: Current sharing between paralleled MOSFETs  (I_k = I (1/R_k)/sum(1/R_j); P_k = I_k^2 R_k)
  I = 40 A
  assumes: static sharing (on-resistance only)
```

In the worked example, R = [10, 12], I = 40 A go through the equation above and give currents = [21.81818182, 18.18181818], losses = [4.760330579, 3.966942149], P_total = 8.727272727 W, imbalance = 1.090909091. It applies only when static sharing (on-resistance only): outside that it says what is wrong instead of returning a number.

<a id="pe_diode_reverse_recovery"></a>

#### Diode reverse-recovery charge and losses — `pe_diode_reverse_recovery`

The recovered charge, the two recovery intervals and the reverse-recovery energy dissipated in the diode and the extra turn-on energy in the switch (and the powers) for a triangular recovery current of peak Irm, time trr and softness S = tb/ta.

$$
Q_{rr} = \tfrac12I_{rm}t_{rr},\quad E_D = \frac{V_rI_{rm}t_b}{6},\quad E_{sw} = V_rI_{rm}\left(\frac{t_a}{2}+\frac{t_b}{3}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Irm` | in | peak reverse-recovery current | A |
| `trr` | in | reverse-recovery time | s |
| `S` | in | softness factor tb/ta | — |
| `Vr` | in | reverse (bus) voltage | V |
| `fs` | in | switching frequency | Hz |
| `Qrr` | out | recovered charge | C |
| `ta` | out | current rise interval | s |
| `tb` | out | current decay interval | s |
| `E_diode` | out | recovery energy dissipated in the diode | J |
| `E_switch` | out | extra switch turn-on energy | J |
| `P_diode` | out | diode recovery loss | W |
| `P_switch` | out | switch loss caused by recovery | W |
| `P_total` | out | total recovery loss (= Vr Qrr fs) | W |

**Applies when:** triangular recovery current; diode voltage rises linearly during tb.

```shell
$ power-electronics solve pe_diode_reverse_recovery "Irm=10 A" "trr=100 ns" S=1 "Vr=400 V" "fs=100 kHz"
Qrr = 5e-07 C
ta = 5e-08 s
tb = 5e-08 s
E_diode = 3.333333333e-05 J
E_switch = 0.0001666666667 J
P_diode = 3.333333333 W
P_switch = 16.66666667 W
P_total = 20 W
  by: Diode reverse-recovery charge and losses  (ta = trr/(1 + S), tb = S trr/(1 + S); Qrr = Irm trr/2; E_D = Vr Irm tb/6; E_sw = Vr Irm (ta/2 + tb/3); together Vr Qrr)
  Irm = 10 A
  trr = 1e-07 s
  S = 1
  Vr = 400 V
  fs = 100000 Hz
  assumes: triangular recovery current; diode voltage rises linearly during tb
```

In the worked example, Irm = 10 A, trr = 100 ns, S = 1, Vr = 400 V, fs = 100 kHz go through the equation above and give Qrr = 5e-07 C, ta = 5e-08 s, tb = 5e-08 s, E_diode = 3.333333333e-05 J. It applies only when triangular recovery current; diode voltage rises linearly during tb: outside that it says what is wrong instead of returning a number.

<a id="pe_gate_drive_power"></a>

#### Gate-drive power and currents — `pe_gate_drive_power`

The gate-driver power, energy per switching cycle, average gate current and peak gate current of a MOSFET or IGBT from its total gate charge, drive voltage, switching frequency and total gate resistance.

$$
P_{G} = Q_gV_{drv}f_s,\quad I_{G,pk} = \frac{V_{drv}}{R_g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Qg` | in | total gate charge at the drive voltage | C |
| `Vdrv` | in | gate-drive voltage swing | V |
| `fs` | in | switching frequency | Hz |
| `Rg` | in | total gate resistance (driver, external and internal) | ohm |
| `P_drive` | out | gate-drive power Qg V fs | W |
| `Ig_avg` | out | average gate current Qg fs | A |
| `Ig_peak` | out | peak gate current V/Rg | A |
| `E_cycle` | out | energy per switching cycle Qg V | J |

**Applies when:** the gate fully charges and discharges every cycle.

```shell
$ power-electronics solve pe_gate_drive_power "Qg=50 nC" "Vdrv=12 V" "fs=200 kHz" "Rg=5 ohm"
P_drive = 0.12 W
Ig_avg = 0.01 A
Ig_peak = 2.4 A
E_cycle = 6e-07 J
  by: Gate-drive power and currents  (P = Qg Vdrv fs; Ig,avg = Qg fs; Ig,peak = Vdrv/Rg)
  Qg = 5e-08 C
  Vdrv = 12 V
  fs = 200000 Hz
  Rg = 5 ohm
  assumes: the gate fully charges and discharges every cycle
```

In the worked example, Qg = 50 nC, Vdrv = 12 V, fs = 200 kHz, Rg = 5 ohm go through the equation above and give P_drive = 0.12 W, Ig_avg = 0.01 A, Ig_peak = 2.4 A, E_cycle = 6e-07 J. It applies only when the gate fully charges and discharges every cycle: outside that it says what is wrong instead of returning a number.

<a id="pe_switching_loss"></a>

#### Hard-switching loss of a power transistor — `pe_switching_loss`

The turn-on and turn-off energies and the switching and output-capacitance losses of a MOSFET or IGBT switching an inductive (clamped) load with linear voltage and current transitions.

$$
P_{sw} = \tfrac12 V I\,(t_r + t_f)\,f_s,\quad P_{oss} = \tfrac12C_{oss}V^2f_s
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | blocking (dc bus) voltage | V |
| `I` | in | switched current | A |
| `tr` | in | turn-on crossover time | s |
| `tf` | in | turn-off crossover time | s |
| `fs` | in | switching frequency | Hz |
| `Coss` | in | output capacitance | F |
| `E_on` | out | turn-on energy | J |
| `E_off` | out | turn-off energy | J |
| `P_sw` | out | switching loss | W |
| `P_oss` | out | output-capacitance loss | W |
| `P_total` | out | total switching-related loss | W |

**Applies when:** clamped inductive load, linear transitions.

```shell
$ power-electronics solve pe_switching_loss "V=400 V" "I=10 A" "tr=30 ns" "tf=50 ns" "fs=100 kHz" "Coss=100 pF"
E_on = 6e-05 J
E_off = 0.0001 J
P_sw = 16 W
P_oss = 0.8 W
P_total = 16.8 W
  by: Hard-switching loss of a power transistor  (E_on = V I tr/2; E_off = V I tf/2; P_sw = (E_on + E_off) fs; P_oss = Coss V^2 fs/2)
  V = 400 V
  I = 10 A
  tr = 3e-08 s
  tf = 5e-08 s
  fs = 100000 Hz
  Coss = 1e-10 F
  assumes: clamped inductive load, linear transitions
```

In the worked example, V = 400 V, I = 10 A, tr = 30 ns, tf = 50 ns, fs = 100 kHz, Coss = 100 pF go through the equation above and give E_on = 6e-05 J, E_off = 0.0001 J, P_sw = 16 W, P_oss = 0.8 W. It applies only when clamped inductive load, linear transitions: outside that it says what is wrong instead of returning a number.

<a id="pe_mosfet_conduction_loss"></a>

#### MOSFET conduction loss at junction temperature — `pe_mosfet_conduction_loss`

The on-resistance of a MOSFET at its junction temperature (linear temperature coefficient from the 25 degC value) and the conduction loss for an RMS drain current, with the loss at 25 degC for comparison.

$$
R_{ds}(T_j) = R_{ds,25}\left(1+\alpha(T_j-25^\circ\mathrm{C})\right),\quad P = I_{rms}^2R_{ds}(T_j)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rds25` | in | on-resistance at room temperature | ohm |
| `alpha` | in | temperature coefficient of the on-resistance per kelvin | — |
| `Tj` | in | junction temperature | K |
| `Irms` | in | RMS drain current | A |
| `Rds_T` | out | on-resistance at Tj | ohm |
| `P` | out | conduction loss at the junction temperature | W |
| `P_25` | out | conduction loss at room temperature | W |
| `ratio` | out | ratio of hot to room-temperature on-resistance | — |

**Applies when:** linear temperature model.

```shell
$ power-electronics solve pe_mosfet_conduction_loss "Rds25=0.01 ohm" alpha=0.007 "Tj=125 degC" "Irms=20 A"
Rds_T = 0.017 ohm
P = 6.8 W
P_25 = 4 W
ratio = 1.7
  by: MOSFET conduction loss at junction temperature  (Rds(Tj) = Rds25 (1 + alpha (Tj - 25 degC)); P = Irms^2 Rds(Tj))
  Rds25 = 0.01 ohm
  alpha = 0.007
  Tj = 398.15 K
  Irms = 20 A
  assumes: linear temperature model
```

In the worked example, Rds25 = 0.01 ohm, alpha = 0.007, Tj = 125 degC, Irms = 20 A go through the equation above and give Rds_T = 0.017 ohm, P = 6.8 W, P_25 = 4 W, ratio = 1.7. It applies only when linear temperature model: outside that it says what is wrong instead of returning a number.

<a id="pe_rc_snubber_ringing"></a>

#### RC snubber from measured ringing frequencies — `pe_rc_snubber_ringing`

The parasitic capacitance and inductance of a ringing switch node found by measuring the ringing frequency with and without an added capacitor, the damping resistor (characteristic impedance), the snubber capacitor (k times the parasitic) and the snubber resistor loss.

$$
C_p = \frac{C_{add}}{(f_0/f_1)^2-1},\quad L_p = \frac{1}{(2\pi f_0)^2C_p},\quad R_s = \sqrt{L_p/C_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f0` | in | ringing frequency without the added capacitor | Hz |
| `f1` | in | ringing frequency with the added capacitor | Hz |
| `Cadd` | in | added test capacitance | F |
| `k` | in | snubber capacitance as a multiple of the parasitic capacitance | — |
| `V` | in | voltage the node swings to | V |
| `fs` | in | switching frequency | Hz |
| `Cp` | out | parasitic capacitance | F |
| `Lp` | out | parasitic inductance | H |
| `Rs` | out | snubber resistance sqrt(Lp/Cp) | ohm |
| `Cs` | out | snubber capacitance k Cp | F |
| `P_R` | out | snubber resistor loss | W |

**Applies when:** f1 < f0.

```shell
$ power-electronics solve pe_rc_snubber_ringing "f0=50000 kHz" "f1=25000 kHz" "Cadd=300 pF" k=4 "V=48 V" "fs=500 kHz"
Cp = 1e-10 F
Lp = 1.013211836e-07 H
Rs = 31.83098862 ohm
Cs = 4e-10 F
P_R = 0.4608 W
  by: RC snubber from measured ringing frequencies  (Cp = Cadd/((f0/f1)^2 - 1); Lp = 1/((2 pi f0)^2 Cp); Rs = sqrt(Lp/Cp); Cs = k Cp; P = Cs V^2 fs)
  f0 = 50000000 Hz
  f1 = 25000000 Hz
  Cadd = 3e-10 F
  k = 4
  V = 48 V
  fs = 500000 Hz
  assumes: f1 < f0
```

In the worked example, f0 = 50000 kHz, f1 = 25000 kHz, Cadd = 300 pF, k = 4, V = 48 V, fs = 500 kHz go through the equation above and give Cp = 1e-10 F, Lp = 1.013211836e-07 H, Rs = 31.83098862 ohm, Cs = 4e-10 F. It applies only when f1 < f0: outside that it says what is wrong instead of returning a number.

<a id="pe_turnoff_snubber"></a>

#### RCD turn-off snubber design — `pe_turnoff_snubber`

The turn-off snubber capacitance that lets the switch voltage reach the bus voltage just as the current finishes falling, the switch turn-off energy with and without the snubber, and the snubber (resistor) loss per cycle and in watts.

$$
C_s = \frac{I t_f}{2V},\quad E_{sw} = \frac{I^2t_f^2}{24C_s} = \frac{VIt_f}{12},\quad E_R = \tfrac12C_sV^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I` | in | switched (load) current | A |
| `V` | in | bus voltage | V |
| `tf` | in | current fall time | s |
| `fs` | in | switching frequency | Hz |
| `Cs` | out | snubber capacitance | F |
| `E_off_with` | out | switch turn-off energy with the snubber | J |
| `E_off_without` | out | switch turn-off energy without a snubber | J |
| `E_snubber` | out | energy dumped in the snubber resistor | J |
| `P_switch` | out | switch turn-off loss with the snubber | W |
| `P_snubber` | out | snubber resistor loss | W |
| `P_total` | out | switch plus snubber loss | W |

**Applies when:** linear current fall; the snubber capacitor discharges fully during the on-time.

```shell
$ power-electronics solve pe_turnoff_snubber "I=10 A" "V=300 V" "tf=100 ns" "fs=50 kHz"
Cs = 1.666666667e-09 F
E_off_with = 2.5e-05 J
E_off_without = 0.00015 J
E_snubber = 7.5e-05 J
P_switch = 1.25 W
P_snubber = 3.75 W
P_total = 5 W
  by: RCD turn-off snubber design  (Cs = I tf/(2V); E_sw = I^2 tf^2/(24 Cs); E_R = Cs V^2/2)
  I = 10 A
  V = 300 V
  tf = 1e-07 s
  fs = 50000 Hz
  assumes: linear current fall; the snubber capacitor discharges fully during the on-time
```

In the worked example, I = 10 A, V = 300 V, tf = 100 ns, fs = 50 kHz go through the equation above and give Cs = 1.666666667e-09 F, E_off_with = 2.5e-05 J, E_off_without = 0.00015 J, E_snubber = 7.5e-05 J. It applies only when linear current fall; the snubber capacitor discharges fully during the on-time: outside that it says what is wrong instead of returning a number.

<a id="pe_zvs_transition"></a>

#### Zero-voltage switching: resonant transition of a switch node — `pe_zvs_transition`

Whether an inductor current can swing a switch node's capacitance from 0 to the input voltage (zero-voltage switching), the minimum current needed, the energies, and the transition time (or the time to the voltage peak when it cannot), for a resonant LC transition.

$$
v(t) = I_0Z\sin\omega t,\quad Z = \sqrt{L/C},\quad t_{zvs} = \frac{1}{\omega}\sin^{-1}\frac{V_{in}}{I_0Z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | voltage the node must swing | V |
| `Ceq` | in | total node capacitance | F |
| `L` | in | resonant (leakage) inductance | H |
| `I0` | in | inductor current at the start of the transition | A |
| `Z` | out | characteristic impedance sqrt(L/C) | ohm |
| `f_res` | out | resonant frequency | Hz |
| `I_min` | out | minimum current for ZVS Vin/Z | A |
| `E_L` | out | inductor energy | J |
| `E_C` | out | capacitor energy | J |
| `zvs` | out | achieved or not achieved | — |
| `t_transition` | out | time to reach Vin (or to the voltage peak when ZVS fails) | s |
| `v_reached` | out | highest node voltage reached (capped at Vin) | V |

**Applies when:** lossless resonant transition starting at 0 V.

```shell
$ power-electronics solve pe_zvs_transition "Vin=400 V" "Ceq=400 pF" "L=20 uH" "I0=3 A"
Z = 223.6067977 ohm
f_res = 1779406.359 Hz
I_min = 1.788854382 A
E_L = 9e-05 J
E_C = 3.2e-05 J
zvs = achieved
t_transition = 5.714183564e-08 s
v_reached = 400 V
  by: Zero-voltage switching: resonant transition of a switch node  (v(t) = I0 Z sin(w t), Z = sqrt(L/C); ZVS when I0 Z >= Vin; t = asin(Vin/(I0 Z))/w)
  Vin = 400 V
  Ceq = 4e-10 F
  L = 2e-05 H
  I0 = 3 A
  assumes: lossless resonant transition starting at 0 V
```

In the worked example, Vin = 400 V, Ceq = 400 pF, L = 20 uH, I0 = 3 A go through the equation above and give Z = 223.6067977 ohm, f_res = 1779406.359 Hz, I_min = 1.788854382 A, E_L = 9e-05 J. It applies only when lossless resonant transition starting at 0 V: outside that it says what is wrong instead of returning a number.

### Front ends (2)

<a id="pe_boost_pfc"></a>

#### Boost power-factor-correction stage — `pe_boost_pfc`

The peak line current, duty cycle and inductor ripple at the line peak, the largest ripple over the line cycle, the inductor current at the line peak and the second-harmonic output voltage ripple of a continuous-conduction boost PFC stage with unity power factor.

$$
\Delta i_L(v) = \frac{v\,(1-v/V_o)}{Lf_s},\quad \Delta i_{L,max} = \frac{V_o}{4Lf_s},\quad \Delta V_o = \frac{P_o}{2\pi f_{line}CV_o}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vac` | in | RMS line voltage | V |
| `f_line` | in | line frequency | Hz |
| `Vo` | in | dc output voltage | V |
| `Po` | in | output power | W |
| `eff` | in | efficiency (a fraction or %) | — |
| `L` | in | boost inductance | H |
| `fs` | in | switching frequency | Hz |
| `C` | in | output (bulk) capacitance | F |
| `I_in_pk` | out | peak line current | A |
| `D_peak` | out | duty cycle at the line peak | — |
| `dI_peak` | out | inductor ripple at the line peak | A |
| `dI_max` | out | largest inductor ripple over the line cycle | A |
| `IL_at_peak` | out | peak inductor current at the line peak | A |
| `dVo_pp` | out | peak-to-peak second-harmonic output ripple | V |

**Applies when:** unity power factor, continuous conduction; Vo > sqrt(2) Vac; small output ripple.

```shell
$ power-electronics solve pe_boost_pfc "Vac=230 V" "f_line=50 Hz" "Vo=400 V" "Po=500 W" eff=0.95 "L=500 uH" "fs=65 kHz" "C=330 uF"
I_in_pk = 3.236186642 A
D_peak = 0.1868272016
dI_peak = 1.869819057 A
dI_max = 3.076923077 A
IL_at_peak = 4.17109617 A
dVo_pp = 12.05719266 V
  by: Boost power-factor-correction stage  (Ipk = 2 Po/(eff Vpk); dI(v) = v (1 - v/Vo)/(L fs), largest at v = Vo/2; dVo = Po/(2 pi f_line C Vo))
  Vac = 230 V
  f_line = 50 Hz
  Vo = 400 V
  Po = 500 W
  eff = 0.95
  L = 0.0005 H
  fs = 65000 Hz
  C = 0.00033 F
  assumes: unity power factor, continuous conduction; Vo > sqrt(2) Vac; small output ripple
```

In the worked example, Vac = 230 V, f_line = 50 Hz, Vo = 400 V, Po = 500 W, eff = 0.95, L = 500 uH, fs = 65 kHz, C = 330 uF go through the equation above and give I_in_pk = 3.236186642 A, D_peak = 0.1868272016, dI_peak = 1.869819057 A, dI_max = 3.076923077 A. It applies only when unity power factor, continuous conduction; Vo > sqrt(2) Vac; small output ripple: outside that it says what is wrong instead of returning a number.

<a id="pe_hold_up_capacitor"></a>

#### Hold-up (bulk) capacitor — `pe_hold_up_capacitor`

The bulk capacitance that keeps a constant-power load running for a hold-up time while the capacitor voltage falls from V1 to the minimum operating voltage V2, with the energies involved.

$$
C = \frac{2P_ot_{hold}}{V_1^2 - V_2^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Po` | in | load power drawn from the capacitor | W |
| `t_hold` | in | hold-up time | s |
| `V1` | in | starting capacitor voltage | V |
| `V2` | in | minimum operating voltage | V |
| `C` | out | required capacitance | F |
| `E_used` | out | energy delivered Po t | J |
| `E_initial` | out | energy stored at the starting voltage | J |
| `E_remaining` | out | energy left at the minimum voltage | J |

**Applies when:** constant-power load; V2 < V1.

```shell
$ power-electronics solve pe_hold_up_capacitor "Po=300 W" "t_hold=20 ms" "V1=390 V" "V2=300 V"
C = 0.000193236715 F
E_used = 6 J
E_initial = 14.69565217 J
E_remaining = 8.695652174 J
  by: Hold-up (bulk) capacitor  (C = 2 Po t_hold/(V1^2 - V2^2))
  Po = 300 W
  t_hold = 0.02 s
  V1 = 390 V
  V2 = 300 V
  assumes: constant-power load; V2 < V1
```

In the worked example, Po = 300 W, t_hold = 20 ms, V1 = 390 V, V2 = 300 V go through the equation above and give C = 0.000193236715 F, E_used = 6 J, E_initial = 14.69565217 J, E_remaining = 8.695652174 J. It applies only when constant-power load; V2 < V1: outside that it says what is wrong instead of returning a number.

### Inverters (7)

<a id="pe_dead_time_error"></a>

#### Inverter dead-time voltage error — `pe_dead_time_error`

The average voltage error per switching period caused by the dead time of an inverter leg, the peak of its fundamental (the error is a square wave in phase with the current) and that as a percentage of the wanted fundamental.

$$
\Delta V = t_d f_s V_{dc},\quad \Delta\hat V_1 = \frac{4}{\pi}t_d f_s V_{dc}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc-link voltage | V |
| `td` | in | dead time | s |
| `fs` | in | switching frequency | Hz |
| `V1` | in | wanted fundamental peak voltage | V |
| `dV_avg` | out | average voltage error td fs Vdc | V |
| `dV1_peak` | out | peak fundamental of the error | V |
| `dV1_pct` | out | error in percent of the wanted fundamental | — |

**Applies when:** td fs < 0.5; the current does not change sign within a switching period.

```shell
$ power-electronics solve pe_dead_time_error "Vdc=600 V" "td=2000 ns" "fs=10 kHz" "V1=250 V"
dV_avg = 12 V
dV1_peak = 15.27887454 V
dV1_pct = 6.111549815
  by: Inverter dead-time voltage error  (dV = td fs Vdc; its fundamental 4 dV/pi)
  Vdc = 600 V
  td = 2e-06 s
  fs = 10000 Hz
  V1 = 250 V
  assumes: td fs < 0.5; the current does not change sign within a switching period
```

In the worked example, Vdc = 600 V, td = 2000 ns, fs = 10 kHz, V1 = 250 V go through the equation above and give dV_avg = 12 V, dV1_peak = 15.27887454 V, dV1_pct = 6.111549815. It applies only when td fs < 0.5; the current does not change sign within a switching period: outside that it says what is wrong instead of returning a number.

<a id="pe_quasi_square_inverter"></a>

#### Quasi-square-wave (phase-shifted) full-bridge inverter — `pe_quasi_square_inverter`

The fundamental, the RMS value of an odd harmonic n, the RMS output and the THD of a full-bridge inverter with a zero-voltage interval alpha at each end of every half-cycle, and the alpha that eliminates harmonic n.

$$
V_{n,rms} = \frac{4V_{dc}}{n\pi\sqrt2}\cos(n\alpha),\quad V_{rms} = V_{dc}\sqrt{1-\frac{2\alpha}{\pi}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc input voltage | V |
| `alpha` | in | zero-voltage interval at each end of a half-cycle | rad |
| `n` | in | odd harmonic order | — |
| `V1_rms` | out | RMS fundamental | V |
| `Vn_rms` | out | RMS value of harmonic n | V |
| `Vrms` | out | RMS output voltage | V |
| `THD` | out | total harmonic distortion | — |
| `alpha_eliminate_n_deg` | out | zero interval in degrees that removes harmonic n | — |

**Applies when:** 0 <= alpha < 90 degrees; n odd, 3..99.

```shell
$ power-electronics solve pe_quasi_square_inverter "Vdc=100 V" "alpha=30 deg" n=3
V1_rms = 77.96968012 V
Vn_rms = 0 V
Vrms = 81.64965809 V
THD = 0.3108419393
alpha_eliminate_n_deg = 30
  by: Quasi-square-wave (phase-shifted) full-bridge inverter  (Vn = 4 Vdc cos(n alpha)/(n pi sqrt 2); Vrms = Vdc sqrt(1 - 2 alpha/pi); harmonic n vanishes at alpha = 90/n degrees)
  Vdc = 100 V
  alpha = 0.5235987756 rad
  assumes: 0 <= alpha < 90 degrees; n odd, 3..99
```

In the worked example, Vdc = 100 V, alpha = 30 deg, n = 3 go through the equation above and give V1_rms = 77.96968012 V, Vn_rms = 0 V, Vrms = 81.64965809 V, THD = 0.3108419393. It applies only when 0 <= alpha < 90 degrees; n odd, 3..99: outside that it says what is wrong instead of returning a number.

<a id="pe_spwm_fundamental"></a>

#### Sinusoidal PWM: fundamental output voltage — `pe_spwm_fundamental`

The fundamental output voltage of sinusoidal (sine-triangle) PWM in the linear range: the peak and RMS phase-leg voltage, the RMS line voltage of a three-phase inverter, the RMS output of a bipolar full bridge, and the carrier frequency.

$$
\hat V_{1,leg} = m_a\frac{V_{dc}}{2},\quad V_{LL1,rms} = \frac{\sqrt3}{2\sqrt2}m_aV_{dc},\quad f_c = m_f f_1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc-link voltage | V |
| `ma` | in | amplitude modulation index | — |
| `mf` | in | frequency modulation ratio (odd whole number) | — |
| `f1` | in | fundamental output frequency | Hz |
| `V1_leg_peak` | out | peak fundamental of a leg against the dc midpoint | V |
| `V1_leg_rms` | out | RMS fundamental of a leg | V |
| `V1_LL_rms` | out | RMS fundamental line voltage of a three-phase inverter | V |
| `V1_fullbridge_rms` | out | RMS fundamental of a bipolar full bridge | V |
| `f_carrier` | out | carrier (switching) frequency | Hz |

**Applies when:** 0 < ma <= 1; mf odd, 15..201 (natural sampling).

```shell
$ power-electronics solve pe_spwm_fundamental "Vdc=600 V" ma=0.8 mf=21 "f1=50 Hz"
V1_leg_peak = 240 V
V1_leg_rms = 169.7056275 V
V1_LL_rms = 293.9387691 V
V1_fullbridge_rms = 339.411255 V
f_carrier = 1050 Hz
  by: Sinusoidal PWM: fundamental output voltage  (V1,leg = ma Vdc/2 (peak); V1,LL = sqrt(3) ma Vdc/(2 sqrt 2); bipolar full bridge V1 = ma Vdc/sqrt 2; fc = mf f1)
  Vdc = 600 V
  ma = 0.8
  f1 = 50 Hz
  assumes: 0 < ma <= 1; mf odd, 15..201 (natural sampling)
```

In the worked example, Vdc = 600 V, ma = 0.8, mf = 21, f1 = 50 Hz go through the equation above and give V1_leg_peak = 240 V, V1_leg_rms = 169.7056275 V, V1_LL_rms = 293.9387691 V, V1_fullbridge_rms = 339.411255 V. It applies only when 0 < ma <= 1; mf odd, 15..201 (natural sampling): outside that it says what is wrong instead of returning a number.

<a id="pe_space_vector_times"></a>

#### Space-vector modulation dwell times — `pe_space_vector_times`

The sector and the dwell times T1, T2 of the two adjacent active vectors and T0 of the zero vectors that synthesise a reference space vector (peak phase voltage Vref at angle theta) in one switching period of a two-level three-phase inverter.

$$
T_1 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\!\left(\tfrac{\pi}{3}-\theta'\right),\quad T_2 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\theta',\quad T_0 = T_s - T_1 - T_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vref` | in | reference vector magnitude (peak phase voltage) | V |
| `theta` | in | reference vector angle | rad |
| `Vdc` | in | dc-link voltage | V |
| `fs` | in | switching (sampling) frequency | Hz |
| `sector` | out | sector number | — |
| `T1` | out | dwell time of the first active vector | s |
| `T2` | out | dwell time of the second active vector | s |
| `T0` | out | total zero-vector time | s |
| `m` | out | modulation index | — |

**Applies when:** inside the hexagon (T1 + T2 <= Ts).

```shell
$ power-electronics solve pe_space_vector_times "Vref=200 V" "theta=25 deg" "Vdc=600 V" "fs=10 kHz"
sector = 1
T1 = 3.311545099e-05 s
T2 = 2.439987672e-05 s
T0 = 4.248467229e-05 s
m = 0.5773502692
  by: Space-vector modulation dwell times  (T1 = sqrt(3) Ts Vref/Vdc sin(60 deg - theta'), T2 = sqrt(3) Ts Vref/Vdc sin(theta'), T0 = Ts - T1 - T2, theta' = angle within the sector)
  Vref = 200 V
  theta = 0.436332313 rad
  Vdc = 600 V
  fs = 10000 Hz
  assumes: inside the hexagon (T1 + T2 <= Ts)
```

In the worked example, Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz go through the equation above and give sector = 1, T1 = 3.311545099e-05 s, T2 = 2.439987672e-05 s, T0 = 4.248467229e-05 s. It applies only when inside the hexagon (T1 + T2 <= Ts): outside that it says what is wrong instead of returning a number.

<a id="pe_square_wave_inverter"></a>

#### Square-wave inverter output and harmonics — `pe_square_wave_inverter`

The RMS output voltage, the fundamental (RMS and peak), the RMS value of an odd harmonic n and the THD of a single-phase half-bridge or full-bridge inverter operated with square-wave (180 degree) switching.

$$
V_{n,rms} = \frac{4A}{n\pi\sqrt2},\quad THD = \sqrt{\frac{\pi^2}{8}-1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc input voltage | V |
| `bridge` | in | half-bridge or full-bridge | — |
| `n` | in | odd harmonic order | — |
| `Vrms` | out | RMS output voltage | V |
| `V1_rms` | out | RMS fundamental | V |
| `V1_peak` | out | peak fundamental | V |
| `Vn_rms` | out | RMS value of harmonic n | V |
| `THD` | out | total harmonic distortion | — |

**Applies when:** n odd, 3..99; ideal switching, no dead time.

```shell
$ power-electronics solve pe_square_wave_inverter "Vdc=100 V" bridge=full-bridge n=3
Vrms = 100 V
V1_rms = 90.03163162 V
V1_peak = 127.3239545 V
Vn_rms = 30.01054387 V
THD = 0.4834258476
  by: Square-wave inverter output and harmonics  (A = Vdc (full bridge) or Vdc/2 (half bridge); V1 = 4 A/(pi sqrt 2); Vn = V1/n; THD = sqrt(pi^2/8 - 1))
  Vdc = 100 V
  bridge = full-bridge
  assumes: n odd, 3..99; ideal switching, no dead time
```

In the worked example, Vdc = 100 V, bridge = full-bridge, n = 3 go through the equation above and give Vrms = 100 V, V1_rms = 90.03163162 V, V1_peak = 127.3239545 V, Vn_rms = 30.01054387 V. It applies only when n odd, 3..99; ideal switching, no dead time: outside that it says what is wrong instead of returning a number.

<a id="pe_inverter_rl_load_current"></a>

#### Square-wave inverter with an RL load: current — `pe_inverter_rl_load_current`

The steady-state peak current, RMS current, load power and average dc source current of a full-bridge square-wave inverter driving a series RL load (exact exponential current, not a harmonic approximation).

$$
I_{max} = \frac{V_{dc}}{R}\,\frac{1-e^{-T/2\tau}}{1+e^{-T/2\tau}},\quad \tau = \frac{L}{R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc input voltage | V |
| `f` | in | output frequency | Hz |
| `R` | in | load resistance | ohm |
| `L` | in | load inductance | H |
| `I_max` | out | peak load current | A |
| `I_rms` | out | RMS load current | A |
| `P` | out | load power | W |
| `Is_avg` | out | average dc source current | A |

**Applies when:** steady state, ideal switches with antiparallel diodes.

```shell
$ power-electronics solve pe_inverter_rl_load_current "Vdc=100 V" "f=60 Hz" "R=10 ohm" "L=25 mH"
I_max = 9.311096087 A
I_rms = 6.643299141 A
P = 441.3342348 W
Is_avg = 4.413342348 A
  by: Square-wave inverter with an RL load: current  (I_max = (Vdc/R)(1 - e^(-T/2tau))/(1 + e^(-T/2tau)); i(t) = Vdc/R + (-I_max - Vdc/R) e^(-t/tau) over a half-cycle; P = I_rms^2 R)
  Vdc = 100 V
  f = 60 Hz
  R = 10 ohm
  L = 0.025 H
  assumes: steady state, ideal switches with antiparallel diodes
```

In the worked example, Vdc = 100 V, f = 60 Hz, R = 10 ohm, L = 25 mH go through the equation above and give I_max = 9.311096087 A, I_rms = 6.643299141 A, P = 441.3342348 W, Is_avg = 4.413342348 A. It applies only when steady state, ideal switches with antiparallel diodes: outside that it says what is wrong instead of returning a number.

<a id="pe_six_step_inverter"></a>

#### Three-phase six-step (square-wave) inverter — `pe_six_step_inverter`

The RMS line and phase voltages, their fundamentals, the THD and the 5th and 7th harmonics of a three-phase voltage-source inverter in six-step (180 degree) operation.

$$
V_{LL1} = \frac{\sqrt6}{\pi}V_{dc},\quad V_{LL} = \sqrt{\tfrac23}V_{dc},\quad V_{LL,h} = \frac{V_{LL1}}{h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc-link voltage | V |
| `f` | in | output (fundamental) frequency | Hz |
| `VLL_rms` | out | RMS line-to-line voltage | V |
| `VLL1_rms` | out | RMS fundamental line voltage | V |
| `Vph_rms` | out | RMS phase voltage | V |
| `Vph1_rms` | out | RMS fundamental phase voltage | V |
| `THD` | out | THD of the line voltage | — |
| `VLL5_rms` | out | RMS fifth-harmonic line voltage | V |
| `VLL7_rms` | out | RMS seventh-harmonic line voltage | V |
| `f5` | out | fifth-harmonic frequency | Hz |
| `f7` | out | seventh-harmonic frequency | Hz |

**Applies when:** balanced star load, ideal switching.

```shell
$ power-electronics solve pe_six_step_inverter "Vdc=600 V" "f=50 Hz"
VLL_rms = 489.8979486 V
VLL1_rms = 467.8180807 V
Vph_rms = 282.8427125 V
Vph1_rms = 270.0948948 V
THD = 0.3108419393
VLL5_rms = 93.56361615 V
VLL7_rms = 66.83115439 V
f5 = 250 Hz
f7 = 350 Hz
  by: Three-phase six-step (square-wave) inverter  (V_LL = sqrt(2/3) Vdc; V_LL1 = sqrt(6) Vdc/pi = 0.78 Vdc; harmonics 6k +- 1 at V_LL1/h)
  Vdc = 600 V
  f = 50 Hz
  assumes: balanced star load, ideal switching
```

In the worked example, Vdc = 600 V, f = 50 Hz go through the equation above and give VLL_rms = 489.8979486 V, VLL1_rms = 467.8180807 V, Vph_rms = 282.8427125 V, Vph1_rms = 270.0948948 V. It applies only when balanced star load, ideal switching: outside that it says what is wrong instead of returning a number.

### Isolated converters (4)

<a id="pe_flyback_ccm"></a>

#### Flyback converter in continuous conduction — `pe_flyback_ccm`

The duty cycle, magnetizing current (primary-referred) and ripple, peak primary and secondary currents and the switch and diode voltage stresses of an ideal flyback converter in continuous conduction.

$$
\frac{V_o}{V_{in}} = \frac{1}{n}\frac{D}{1-D},\quad I_m = \frac{I_o}{n(1-D)},\quad \Delta i_m = \frac{V_{in}D}{L_m f_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `n` | in | turns ratio Np/Ns | — |
| `Lm` | in | magnetizing inductance (primary) | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load current | A |
| `D` | out | duty cycle | — |
| `Im` | out | average magnetizing current referred to the primary | A |
| `dIm` | out | magnetizing current ripple | A |
| `Ipk_primary` | out | peak primary (switch) current | A |
| `Ipk_secondary` | out | peak secondary (diode) current | A |
| `V_switch` | out | switch voltage stress Vin + n Vo | V |
| `V_diode` | out | diode reverse voltage Vo + Vin/n | V |

**Applies when:** continuous conduction of the magnetizing current; no leakage inductance.

```shell
$ power-electronics solve pe_flyback_ccm "Vin=48 V" "Vout=12 V" n=2 "Lm=200 uH" "fs=100 kHz" "Io=2 A"
D = 0.3333333333
Im = 1.5 A
dIm = 0.8 A
Ipk_primary = 1.9 A
Ipk_secondary = 3.8 A
V_switch = 72 V
V_diode = 36 V
  by: Flyback converter in continuous conduction  (D = n Vo/(Vin + n Vo); Im = Io/(n (1 - D)); dIm = Vin D/(Lm fs))
  Vin = 48 V
  Vout = 12 V
  n = 2
  Lm = 0.0002 H
  fs = 100000 Hz
  Io = 2 A
  assumes: continuous conduction of the magnetizing current; no leakage inductance
```

In the worked example, Vin = 48 V, Vout = 12 V, n = 2, Lm = 200 uH, fs = 100 kHz, Io = 2 A go through the equation above and give D = 0.3333333333, Im = 1.5 A, dIm = 0.8 A, Ipk_primary = 1.9 A. It applies only when continuous conduction of the magnetizing current; no leakage inductance: outside that it says what is wrong instead of returning a number.

<a id="pe_flyback_dcm_design"></a>

#### Flyback magnetizing inductance for discontinuous conduction — `pe_flyback_dcm_design`

The magnetizing inductance at which a flyback converter delivers a power at a duty cycle in discontinuous conduction (the primary current ramps from zero every period), with the peak, RMS and average primary currents.

$$
L_m = \frac{(V_{in}D)^2}{2 P_{in} f_s},\quad I_{pk} = \frac{V_{in}D}{L_m f_s},\quad I_{rms} = I_{pk}\sqrt{D/3}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage (the minimum, for design) | V |
| `Po` | in | output power | W |
| `eff` | in | efficiency (a fraction or %) | — |
| `fs` | in | switching frequency | Hz |
| `D` | in | duty cycle at this input voltage (a fraction or %) | — |
| `Pin` | out | input power | W |
| `Lm` | out | magnetizing inductance | H |
| `I_pk` | out | peak primary current | A |
| `I_rms` | out | RMS primary current | A |
| `I_avg` | out | average input current | A |
| `E_cycle` | out | energy stored per cycle | J |

**Applies when:** discontinuous conduction: the stored energy is fully released every period.

```shell
$ power-electronics solve pe_flyback_dcm_design "Vin=100 V" "Po=30 W" eff=0.85 "fs=100 kHz" D=0.45
Pin = 35.29411765 W
Lm = 0.000286875 H
I_pk = 1.568627451 A
I_rms = 0.6075267994 A
I_avg = 0.3529411765 A
E_cycle = 0.0003529411765 J
  by: Flyback magnetizing inductance for discontinuous conduction  (Pin = Po/eff = Lm Ipk^2 fs/2 with Ipk = Vin D/(Lm fs) -> Lm = (Vin D)^2/(2 Pin fs); I_rms = Ipk sqrt(D/3))
  Vin = 100 V
  Po = 30 W
  eff = 0.85
  fs = 100000 Hz
  D = 0.45
  assumes: discontinuous conduction: the stored energy is fully released every period
```

In the worked example, Vin = 100 V, Po = 30 W, eff = 0.85, fs = 100 kHz, D = 0.45 go through the equation above and give Pin = 35.29411765 W, Lm = 0.000286875 H, I_pk = 1.568627451 A, I_rms = 0.6075267994 A. It applies only when discontinuous conduction: the stored energy is fully released every period: outside that it says what is wrong instead of returning a number.

<a id="pe_forward_ccm"></a>

#### Forward converter with reset winding — `pe_forward_ccm`

The duty cycle, the largest duty cycle the reset winding allows, the switch voltage stress and the output inductor ripple of a single-switch forward converter in continuous conduction.

$$
V_o = \frac{V_{in} D}{n},\quad D_{max} = \frac{1}{1+N_r/N_p},\quad V_{sw} = V_{in}\left(1+\frac{N_p}{N_r}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `n` | in | turns ratio Np/Ns | — |
| `Nr` | in | reset-winding to primary turns ratio Nr/Np | — |
| `L` | in | output inductance | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load current | A |
| `D` | out | duty cycle | — |
| `D_max` | out | largest duty cycle that resets the core | — |
| `V_switch` | out | switch voltage stress | V |
| `dIL` | out | output inductor ripple | A |
| `IL_max` | out | peak output inductor current | A |
| `IL_min` | out | valley output inductor current | A |

**Applies when:** D < D_max; continuous conduction of the output inductor.

```shell
$ power-electronics solve pe_forward_ccm "Vin=48 V" "Vout=5 V" n=3 Nr=1 "L=22 uH" "fs=200 kHz" "Io=5 A"
D = 0.3125
D_max = 0.5
V_switch = 96 V
dIL = 0.78125 A
IL_max = 5.390625 A
IL_min = 4.609375 A
  by: Forward converter with reset winding  (D = n Vo/Vin; D_max = 1/(1 + Nr/Np); Vsw = Vin (1 + Np/Nr); dIL = (Vin/n - Vo) D/(L fs))
  Vin = 48 V
  Vout = 5 V
  n = 3
  Nr = 1
  L = 2.2e-05 H
  fs = 200000 Hz
  Io = 5 A
  assumes: D < D_max; continuous conduction of the output inductor
```

In the worked example, Vin = 48 V, Vout = 5 V, n = 3, Nr = 1, L = 22 uH, fs = 200 kHz, Io = 5 A go through the equation above and give D = 0.3125, D_max = 0.5, V_switch = 96 V, dIL = 0.78125 A. It applies only when D < D_max; continuous conduction of the output inductor: outside that it says what is wrong instead of returning a number.

<a id="pe_bridge_converter"></a>

#### Push-pull, half-bridge and full-bridge converters — `pe_bridge_converter`

The duty cycle per switch, switch voltage stress, output ripple frequency and output inductor ripple of an ideal push-pull, half-bridge or full-bridge DC-DC converter with a centre-tapped (full-wave) rectifier, in continuous conduction.

$$
V_o = \frac{2 D V_{in}}{n}\ (\text{push-pull, full-bridge}),\quad V_o = \frac{D V_{in}}{n}\ (\text{half-bridge})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `topology` | in | push-pull, half-bridge or full-bridge | — |
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `n` | in | turns ratio Np/Ns (primary to each secondary half) | — |
| `L` | in | output inductance | H |
| `fs` | in | switching frequency | Hz |
| `Io` | in | load current | A |
| `D` | out | duty cycle of each switch | — |
| `V_switch` | out | switch voltage stress | V |
| `f_ripple` | out | output ripple frequency | Hz |
| `dIL` | out | output inductor ripple | A |
| `IL_min` | out | valley output inductor current | A |

**Applies when:** D < 0.5; continuous conduction of the output inductor.

```shell
$ power-electronics solve pe_bridge_converter topology=full-bridge "Vin=400 V" "Vout=48 V" n=5 "L=50 uH" "fs=100 kHz" "Io=10 A"
D = 0.3
V_switch = 400 V
f_ripple = 200000 Hz
dIL = 1.92 A
IL_min = 9.04 A
  by: Push-pull, half-bridge and full-bridge converters  (Vo = 2 D Vin/n (push-pull, full-bridge), Vo = D Vin/n (half-bridge); dIL = (Vsec - Vo) D/(L fs))
  Vin = 400 V
  Vout = 48 V
  n = 5
  L = 5e-05 H
  fs = 100000 Hz
  Io = 10 A
  topology = full-bridge
  assumes: D < 0.5; continuous conduction of the output inductor
```

In the worked example, topology = full-bridge, Vin = 400 V, Vout = 48 V, n = 5, L = 50 uH, fs = 100 kHz, Io = 10 A go through the equation above and give D = 0.3, V_switch = 400 V, f_ripple = 200000 Hz, dIL = 1.92 A. It applies only when D < 0.5; continuous conduction of the output inductor: outside that it says what is wrong instead of returning a number.

### Losses (3)

<a id="pe_boost_nonideal_gain"></a>

#### Boost converter with inductor resistance: gain and efficiency — `pe_boost_nonideal_gain`

The conversion ratio, output voltage, efficiency and inductor current of a boost converter whose inductor has a series resistance, with the duty cycle of maximum gain and that maximum gain.

$$
M = \frac{1}{1-D}\,\frac{1}{1+\frac{R_L}{(1-D)^2R}},\quad \eta = \frac{1}{1+\frac{R_L}{(1-D)^2R}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `D` | in | duty cycle (a fraction or %) | — |
| `R` | in | load resistance | ohm |
| `RL` | in | inductor series resistance | ohm |
| `M` | out | conversion ratio | — |
| `Vout` | out | output voltage | V |
| `efficiency` | out | efficiency | — |
| `IL` | out | average inductor current | A |
| `D_peak` | out | duty cycle of maximum gain | — |
| `M_max` | out | maximum gain | — |

**Applies when:** 0 < RL < R; continuous conduction, ideal switches.

```shell
$ power-electronics solve pe_boost_nonideal_gain "Vin=12 V" D=0.75 "R=50 ohm" "RL=0.5 ohm"
M = 3.448275862
Vout = 41.37931034 V
efficiency = 0.8620689655
IL = 3.310344828 A
D_peak = 0.9
M_max = 5
  by: Boost converter with inductor resistance: gain and efficiency  (M = (1/(1 - D)) / (1 + RL/((1 - D)^2 R)); efficiency = 1/(1 + RL/((1 - D)^2 R)); D_peak = 1 - sqrt(RL/R); M_max = sqrt(R/RL)/2)
  Vin = 12 V
  D = 0.75
  R = 50 ohm
  RL = 0.5 ohm
  assumes: 0 < RL < R; continuous conduction, ideal switches
```

In the worked example, Vin = 12 V, D = 0.75, R = 50 ohm, RL = 0.5 ohm go through the equation above and give M = 3.448275862, Vout = 41.37931034 V, efficiency = 0.8620689655, IL = 3.310344828 A. It applies only when 0 < RL < R; continuous conduction, ideal switches: outside that it says what is wrong instead of returning a number.

<a id="pe_buck_with_losses"></a>

#### Buck converter efficiency with conduction and switching losses — `pe_buck_with_losses`

The duty cycle needed and the losses of a buck converter with a switch on-resistance, inductor resistance, diode forward drop and hard switching: conduction, diode, inductor and switching losses and the efficiency (ripple neglected).

$$
D = \frac{V_o+V_F+I_oR_L}{V_{in}-I_oR_{ds}+V_F},\quad P_{sw} = \tfrac12 V_{in}I_o t_{sw} f_s,\quad \eta = \frac{P_o}{P_o+\sum P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vin` | in | input voltage | V |
| `Vout` | in | output voltage | V |
| `Io` | in | load current | A |
| `fs` | in | switching frequency | Hz |
| `Rds` | in | switch on-resistance | ohm |
| `RL` | in | inductor resistance | ohm |
| `VF` | in | diode forward voltage | V |
| `tsw` | in | total switching (crossover) time per period, turn-on plus turn-off | s |
| `D` | out | duty cycle | — |
| `P_switch_cond` | out | switch conduction loss | W |
| `P_diode` | out | diode conduction loss | W |
| `P_inductor` | out | inductor copper loss | W |
| `P_switching` | out | switching loss | W |
| `P_loss` | out | total loss | W |
| `efficiency` | out | efficiency | — |

**Applies when:** continuous conduction, ripple neglected; hard switching with linear crossover at Vin and Io.

```shell
$ power-electronics solve pe_buck_with_losses "Vin=24 V" "Vout=5 V" "Io=4 A" "fs=200 kHz" "Rds=0.02 ohm" "RL=0.01 ohm" "VF=0.5 V" "tsw=40 ns"
D = 0.2268632269
P_switch_cond = 0.0725962326 W
P_diode = 1.546273546 W
P_inductor = 0.16 W
P_switching = 0.384 W
P_loss = 2.162869779 W
efficiency = 0.9024102113
  by: Buck converter efficiency with conduction and switching losses  (D = (Vo + VF + Io RL)/(Vin - Io Rds + VF); P_sw,cond = Io^2 Rds D; P_D = VF Io (1 - D); P_L = Io^2 RL; P_sw = Vin Io tsw fs/2)
  Vin = 24 V
  Vout = 5 V
  Io = 4 A
  fs = 200000 Hz
  Rds = 0.02 ohm
  RL = 0.01 ohm
  VF = 0.5 V
  tsw = 4e-08 s
  assumes: continuous conduction, ripple neglected; hard switching with linear crossover at Vin and Io
```

In the worked example, Vin = 24 V, Vout = 5 V, Io = 4 A, fs = 200 kHz, Rds = 0.02 ohm, RL = 0.01 ohm, VF = 0.5 V, tsw = 40 ns go through the equation above and give D = 0.2268632269, P_switch_cond = 0.0725962326 W, P_diode = 1.546273546 W, P_inductor = 0.16 W. It applies only when continuous conduction, ripple neglected; hard switching with linear crossover at Vin and Io: outside that it says what is wrong instead of returning a number.

<a id="pe_igbt_inverter_conduction_losses"></a>

#### IGBT and diode conduction losses in a sinusoidal PWM inverter — `pe_igbt_inverter_conduction_losses`

The conduction losses of one IGBT and one freewheeling diode (and of a whole leg) in a sinusoidal PWM inverter, from the peak phase current, modulation index, power-factor angle and the devices' threshold-voltage plus slope-resistance models.

$$
P_T = V_{ce0}\hat I\left(\frac{1}{2\pi}+\frac{M\cos\phi}{8}\right) + r_{ce}\hat I^2\left(\frac18+\frac{M\cos\phi}{3\pi}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ip` | in | peak phase current | A |
| `M` | in | modulation index | — |
| `phi` | in | power-factor angle (current lagging positive) | rad |
| `Vce0` | in | IGBT threshold voltage | V |
| `rce` | in | IGBT slope resistance | ohm |
| `VF0` | in | diode threshold voltage | V |
| `rF` | in | diode slope resistance | ohm |
| `P_igbt` | out | conduction loss of one IGBT | W |
| `P_diode` | out | conduction loss of one diode | W |
| `P_switch_pair` | out | one IGBT plus one diode | W |
| `P_leg` | out | conduction loss of a leg (two of each) | W |

**Applies when:** 0 < M <= 1, sinusoidal current, switching frequency far above the output frequency.

```shell
$ power-electronics solve pe_igbt_inverter_conduction_losses "Ip=100 A" M=0.9 "phi=30 deg" "Vce0=0.9 V" "rce=0.01 ohm" "VF0=0.8 V" "rF=0.008 ohm"
P_igbt = 43.86238552 W
P_diode = 8.322220068 W
P_switch_pair = 52.18460559 W
P_leg = 104.3692112 W
  by: IGBT and diode conduction losses in a sinusoidal PWM inverter  (P_T = Vce0 Ip (1/(2pi) + M cos(phi)/8) + rce Ip^2 (1/8 + M cos(phi)/(3pi)); P_D = VF0 Ip (1/(2pi) - M cos(phi)/8) + rF Ip^2 (1/8 - M cos(phi)/(3pi)))
  Ip = 100 A
  M = 0.9
  phi = 0.5235987756 rad
  Vce0 = 0.9 V
  rce = 0.01 ohm
  VF0 = 0.8 V
  rF = 0.008 ohm
  assumes: 0 < M <= 1, sinusoidal current, switching frequency far above the output frequency
```

In the worked example, Ip = 100 A, M = 0.9, phi = 30 deg, Vce0 = 0.9 V, rce = 0.01 ohm, VF0 = 0.8 V, rF = 0.008 ohm go through the equation above and give P_igbt = 43.86238552 W, P_diode = 8.322220068 W, P_switch_pair = 52.18460559 W, P_leg = 104.3692112 W. It applies only when 0 < M <= 1, sinusoidal current, switching frequency far above the output frequency: outside that it says what is wrong instead of returning a number.

### Magnetics (8)

<a id="pe_steinmetz_core_loss"></a>

#### Core loss by the Steinmetz equation — `pe_steinmetz_core_loss`

The core loss density and total core loss of a magnetic core under sinusoidal flux from the Steinmetz parameters k, alpha, beta (SI: f in Hz, B in T, loss density in W/m^3), the frequency, the peak flux density and the core volume.

$$
P_v = k f^{\alpha} \hat B^{\beta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | Steinmetz coefficient (SI units) | — |
| `alpha` | in | frequency exponent | — |
| `beta` | in | flux-density exponent | — |
| `f` | in | frequency | Hz |
| `Bpk` | in | peak flux density | T |
| `Vc` | in | core volume | m^3 |
| `Pv` | out | core loss density | W/m^3 |
| `Pv_mW_cm3` | out | core loss density in milliwatts per cubic centimetre | — |
| `P` | out | core loss | W |

**Applies when:** sinusoidal flux, within the parameters' fitted range.

```shell
$ power-electronics solve pe_steinmetz_core_loss k=3.2 alpha=1.4 beta=2.5 "f=100 kHz" "Bpk=100 mT" "Vc=5 cm^3"
Pv = 101192.8851 W/m^3
Pv_mW_cm3 = 101.1928851
P = 0.5059644256 W
  by: Core loss by the Steinmetz equation  (Pv = k f^alpha Bpk^beta; P = Pv Vc)
  k = 3.2
  alpha = 1.4
  beta = 2.5
  f = 100000 Hz
  Bpk = 0.1 T
  Vc = 5e-06 m^3
  assumes: sinusoidal flux, within the parameters' fitted range
```

In the worked example, k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3 go through the equation above and give Pv = 101192.8851 W/m^3, Pv_mW_cm3 = 101.1928851, P = 0.5059644256 W. It applies only when sinusoidal flux, within the parameters' fitted range: outside that it says what is wrong instead of returning a number.

<a id="pe_igse_triangular_core_loss"></a>

#### Core loss under triangular flux (iGSE) — `pe_igse_triangular_core_loss`

The core loss of a magnetic component whose flux is triangular (a converter inductor or transformer with duty cycle D) by the improved generalized Steinmetz equation, from the sinusoidal Steinmetz parameters; also its ratio to the sinusoidal loss at the same peak flux.

$$
P_v = k_i(\Delta B)^{\beta}f^{\alpha}\left(D^{1-\alpha}+(1-D)^{1-\alpha}\right),\ k_i = \frac{k}{(2\pi)^{\alpha-1}\int_0^{2\pi}|\cos\theta|^{\alpha}2^{\beta-\alpha}d\theta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | Steinmetz coefficient (SI units) | — |
| `alpha` | in | frequency exponent | — |
| `beta` | in | flux-density exponent | — |
| `f` | in | switching frequency | Hz |
| `dB` | in | peak-to-peak flux density swing | T |
| `D` | in | duty cycle of the rising flux (a fraction or %) | — |
| `Vc` | in | core volume | m^3 |
| `ki` | out | iGSE coefficient | — |
| `Pv` | out | core loss density | W/m^3 |
| `P` | out | core loss | W |
| `ratio_to_sine` | out | loss relative to a sinusoid of the same peak flux | — |

**Applies when:** piecewise-linear flux, no relaxation effects.

```shell
$ power-electronics solve pe_igse_triangular_core_loss k=3.2 alpha=1.4 beta=2.5 "f=100 kHz" "dB=200 mT" D=0.5 "Vc=5 cm^3"
ki = 0.1998061472
Pv = 94324.78603 W/m^3
P = 0.4716239301 W
ratio_to_sine = 0.9321286364
  by: Core loss under triangular flux (iGSE)  (ki = k/((2 pi)^(alpha-1) int|cos|^alpha 2^(beta-alpha)); Pv = ki dB^beta f^alpha (D^(1-alpha) + (1-D)^(1-alpha)))
  k = 3.2
  alpha = 1.4
  beta = 2.5
  f = 100000 Hz
  dB = 0.2 T
  D = 0.5
  Vc = 5e-06 m^3
  assumes: piecewise-linear flux, no relaxation effects
```

In the worked example, k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, dB = 200 mT, D = 0.5, Vc = 5 cm^3 go through the equation above and give ki = 0.1998061472, Pv = 94324.78603 W/m^3, P = 0.4716239301 W, ratio_to_sine = 0.9321286364. It applies only when piecewise-linear flux, no relaxation effects: outside that it says what is wrong instead of returning a number.

<a id="pe_gapped_inductor_design"></a>

#### Gapped-core inductor: turns and air gap — `pe_gapped_inductor_design`

The number of turns and the air-gap length of a gapped-core inductor that gives an inductance L with the peak flux density Bmax at the peak current (core reluctance and fringing neglected), the stored energy, the inductance factor AL and the gap volume.

$$
N = \frac{LI_{pk}}{B_{max}A_c},\quad l_g = \frac{\mu_0N^2A_c}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | inductance | H |
| `Ipk` | in | peak current | A |
| `Bmax` | in | peak flux density allowed | T |
| `Ac` | in | core cross-section area | m^2 |
| `N` | out | turns (before rounding) | — |
| `l_gap` | out | air-gap length | m |
| `E` | out | peak stored energy | J |
| `AL` | out | inductance per turn squared | H |
| `V_gap` | out | gap volume | m^3 |

**Applies when:** gap reluctance dominates; no fringing.

```shell
$ power-electronics solve pe_gapped_inductor_design "L=100 uH" "Ipk=10 A" "Bmax=300 mT" "Ac=1 cm^2"
N = 33.33333333
l_gap = 0.001396263401 m
E = 0.005 J
AL = 9e-08 H
V_gap = 1.396263401e-07 m^3
  by: Gapped-core inductor: turns and air gap  (N = L Ipk/(Bmax Ac); l_g = mu0 N^2 Ac/L; E = L Ipk^2/2)
  L = 0.0001 H
  Ipk = 10 A
  Bmax = 0.3 T
  Ac = 0.0001 m^2
  assumes: gap reluctance dominates; no fringing
```

In the worked example, L = 100 uH, Ipk = 10 A, Bmax = 300 mT, Ac = 1 cm^2 go through the equation above and give N = 33.33333333, l_gap = 0.001396263401 m, E = 0.005 J, AL = 9e-08 H. It applies only when gap reluctance dominates; no fringing: outside that it says what is wrong instead of returning a number.

<a id="pe_inductor_area_product"></a>

#### Inductor core area product — `pe_inductor_area_product`

The core area product Ap = Ac Wa an inductor needs to store its energy at the peak flux density Bmax with a current density J and window utilisation Ku (core selection by the area-product method).

$$
A_p = A_cW_a = \frac{L\,I_{pk}I_{rms}}{B_{max}JK_u}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | inductance | H |
| `Ipk` | in | peak current | A |
| `Irms` | in | RMS current | A |
| `Bmax` | in | peak flux density | T |
| `J` | in | winding current density | A/m^2 |
| `Ku` | in | window utilisation factor (a fraction or %) | — |
| `Ap` | out | area product Ac Wa | m^4 |
| `Ap_cm4` | out | area product in centimetres to the fourth | — |

**Applies when:** 0 < Ku < 1.

```shell
$ power-electronics solve pe_inductor_area_product "L=100 uH" "Ipk=12 A" "Irms=10 A" "Bmax=300 mT" "J=4 A/mm^2" Ku=0.4
Ap = 2.5e-08 m^4
Ap_cm4 = 2.5
  by: Inductor core area product  (Ap = L Ipk Irms/(Bmax J Ku))
  L = 0.0001 H
  Ipk = 12 A
  Irms = 10 A
  Bmax = 0.3 T
  J = 4000000 A/m^2
  Ku = 0.4
  assumes: 0 < Ku < 1
```

In the worked example, L = 100 uH, Ipk = 12 A, Irms = 10 A, Bmax = 300 mT, J = 4 A/mm^2, Ku = 0.4 go through the equation above and give Ap = 2.5e-08 m^4, Ap_cm4 = 2.5. It applies only when 0 < Ku < 1: outside that it says what is wrong instead of returning a number.

<a id="pe_skin_effect_round_wire"></a>

#### Skin effect in a round wire — `pe_skin_effect_round_wire`

The skin depth and the exact AC-to-DC resistance ratio (Bessel-function solution) of an isolated round conductor at a frequency, with its DC and AC resistance for a length.

$$
\delta = \sqrt{\frac{\rho}{\pi f\mu_0}},\quad \frac{R_{ac}}{R_{dc}} = \mathrm{Re}\left[\frac{kr}{2}\frac{J_0(kr)}{J_1(kr)}\right],\ k = \frac{1-j}{\delta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | wire diameter | m |
| `f` | in | frequency | Hz |
| `rho` | in | conductor resistivity | ohm*m |
| `length` | in | conductor length | m |
| `delta` | out | skin depth | m |
| `d_over_delta` | out | diameter in skin depths | — |
| `Rdc` | out | DC resistance | ohm |
| `Rac` | out | AC resistance | ohm |
| `ratio` | out | Rac/Rdc | — |

**Applies when:** isolated straight wire (no proximity effect); radius at most 10 skin depths.

```shell
$ power-electronics solve pe_skin_effect_round_wire "d=1 mm" "f=100 kHz" "rho=1.72e-08 ohm*m" "length=1 m"
delta = 0.000208729751 m
d_over_delta = 4.790883882
Rdc = 0.02189972017 ohm
Rac = 0.03178226064 ohm
ratio = 1.451263322
  by: Skin effect in a round wire  (delta = sqrt(rho/(pi f mu0)); Rac/Rdc = Re[(k r/2) J0(k r)/J1(k r)], k = (1 - j)/delta)
  d = 0.001 m
  f = 100000 Hz
  rho = 1.72e-08 ohm*m
  length = 1 m
  assumes: isolated straight wire (no proximity effect); radius at most 10 skin depths
```

In the worked example, d = 1 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, length = 1 m go through the equation above and give delta = 0.000208729751 m, d_over_delta = 4.790883882, Rdc = 0.02189972017 ohm, Rac = 0.03178226064 ohm. It applies only when isolated straight wire (no proximity effect); radius at most 10 skin depths: outside that it says what is wrong instead of returning a number.

<a id="pe_transformer_area_product"></a>

#### Transformer core area product — `pe_transformer_area_product`

The apparent power the windings handle and the core area product Ap = Ac Wa a two-winding transformer needs for an output power at an efficiency, flux density, frequency, current density and window utilisation (sine or square-wave excitation).

$$
A_p = \frac{P_t}{K_fK_uB_mfJ},\quad P_t = P_o\left(1+\frac{1}{\eta}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Po` | in | output power | W |
| `eff` | in | efficiency (a fraction or %) | — |
| `waveform` | in | sine or square | — |
| `Bm` | in | peak flux density | T |
| `f` | in | frequency | Hz |
| `J` | in | current density | A/m^2 |
| `Ku` | in | window utilisation (a fraction or %) | — |
| `Pt` | out | apparent power of both windings | W |
| `Ap` | out | area product | m^4 |
| `Ap_cm4` | out | area product in centimetres to the fourth | — |

**Applies when:** two windings at the same current density.

```shell
$ power-electronics solve pe_transformer_area_product "Po=250 W" eff=0.95 waveform=square "Bm=100 mT" "f=100 kHz" "J=4 A/mm^2" Ku=0.4
Pt = 513.1578947 W
Ap = 8.018092105e-09 m^4
Ap_cm4 = 0.8018092105
  by: Transformer core area product  (Pt = Po (1 + 1/eff); Ap = Pt/(Kf Ku Bm f J), Kf = 4.44 (sine) or 4 (square))
  Po = 250 W
  eff = 0.95
  Bm = 0.1 T
  f = 100000 Hz
  J = 4000000 A/m^2
  Ku = 0.4
  waveform = square
  assumes: two windings at the same current density
```

In the worked example, Po = 250 W, eff = 0.95, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 0.4 go through the equation above and give Pt = 513.1578947 W, Ap = 8.018092105e-09 m^4, Ap_cm4 = 0.8018092105. It applies only when two windings at the same current density: outside that it says what is wrong instead of returning a number.

<a id="pe_transformer_turns"></a>

#### Transformer turns from Faraday's law — `pe_transformer_turns`

The primary and secondary turns of a transformer that keep the peak flux density at Bmax in a core of cross-section Ac (sine or square-wave excitation), the turns ratio and the volts per turn.

$$
N_p = \frac{V}{K_f f B_{max}A_c},\quad K_f = \sqrt2\pi\ (\text{sine}),\ 4\ (\text{square})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `waveform` | in | sine or square | — |
| `V` | in | primary voltage (RMS for sine, amplitude for square) | V |
| `f` | in | frequency | Hz |
| `Bmax` | in | peak flux density | T |
| `Ac` | in | core cross-section area | m^2 |
| `Vs` | in | secondary voltage (same measure as V) | V |
| `Np` | out | primary turns (before rounding up) | — |
| `Ns` | out | secondary turns (before rounding) | — |
| `n` | out | turns ratio Np/Ns | — |
| `volts_per_turn` | out | volts per turn | V |

**Applies when:** the core does not saturate; no dc bias.

```shell
$ power-electronics solve pe_transformer_turns waveform=square "V=48 V" "f=100 kHz" "Bmax=150 mT" "Ac=1.25 cm^2" "Vs=12 V"
Np = 6.4
Ns = 1.6
n = 4
volts_per_turn = 7.5 V
  by: Transformer turns from Faraday's law  (Np = V/(Kf f Bmax Ac), Kf = 4.44 (sine, RMS) or 4 (square))
  V = 48 V
  f = 100000 Hz
  Bmax = 0.15 T
  Ac = 0.000125 m^2
  Vs = 12 V
  waveform = square
  assumes: the core does not saturate; no dc bias
```

In the worked example, waveform = square, V = 48 V, f = 100 kHz, Bmax = 150 mT, Ac = 1.25 cm^2, Vs = 12 V go through the equation above and give Np = 6.4, Ns = 1.6, n = 4, volts_per_turn = 7.5 V. It applies only when the core does not saturate; no dc bias: outside that it says what is wrong instead of returning a number.

<a id="pe_dowell_proximity_factor"></a>

#### Winding AC resistance factor (Dowell) — `pe_dowell_proximity_factor`

The ratio of AC to DC resistance of a winding of m layers of foil (or equivalent layer) thickness h at a frequency, including skin and proximity effects by Dowell's one-dimensional solution.

$$
F_R = \Delta\left[\frac{\sinh2\Delta+\sin2\Delta}{\cosh2\Delta-\cos2\Delta} + \frac{2(m^2-1)}{3}\frac{\sinh\Delta-\sin\Delta}{\cosh\Delta+\cos\Delta}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | layer (conductor) thickness | m |
| `f` | in | frequency | Hz |
| `rho` | in | conductor resistivity | ohm*m |
| `m` | in | number of layers | — |
| `delta` | out | skin depth | m |
| `Delta` | out | layer thickness in skin depths h/delta | — |
| `FR` | out | AC-to-DC resistance factor | — |

**Applies when:** full-width layers (foil or porosity-corrected); sinusoidal current.

```shell
$ power-electronics solve pe_dowell_proximity_factor "h=0.2 mm" "f=100 kHz" "rho=1.72e-08 ohm*m" m=4
delta = 0.000208729751 m
Delta = 0.9581767764
FR = 2.431126826
  by: Winding AC resistance factor (Dowell)  (F_R = D [ (sinh 2D + sin 2D)/(cosh 2D - cos 2D) + 2 (m^2 - 1)/3 (sinh D - sin D)/(cosh D + cos D) ], D = h/delta)
  h = 0.0002 m
  f = 100000 Hz
  rho = 1.72e-08 ohm*m
  assumes: full-width layers (foil or porosity-corrected); sinusoidal current
```

In the worked example, h = 0.2 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, m = 4 go through the equation above and give delta = 0.000208729751 m, Delta = 0.9581767764, FR = 2.431126826. It applies only when full-width layers (foil or porosity-corrected); sinusoidal current: outside that it says what is wrong instead of returning a number.

### Motor drives (9)

<a id="pe_dc_motor_armature_ripple"></a>

#### Armature current ripple of a chopper-fed DC motor — `pe_dc_motor_armature_ripple`

The maximum, minimum, peak-to-peak and average armature current of a DC motor fed by a step-down chopper, from the exact exponential (RL plus back-EMF) waveform in continuous conduction.

$$
I_{max} = \frac{V_s}{R_a}\frac{1-e^{-DT/\tau}}{1-e^{-T/\tau}} - \frac{E_a}{R_a},\ I_{min} = \frac{V_s}{R_a}\frac{e^{DT/\tau}-1}{e^{T/\tau}-1} - \frac{E_a}{R_a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | dc supply voltage | V |
| `D` | in | duty cycle (a fraction or %) | — |
| `fs` | in | chopping frequency | Hz |
| `Ra` | in | armature resistance | ohm |
| `La` | in | armature inductance | H |
| `Ea` | in | back EMF | V |
| `I_max` | out | maximum armature current | A |
| `I_min` | out | minimum armature current | A |
| `dI` | out | peak-to-peak ripple | A |
| `I_avg` | out | average armature current | A |

**Applies when:** continuous conduction (I_min > 0); constant speed (constant back EMF) over a period.

```shell
$ power-electronics solve pe_dc_motor_armature_ripple "Vs=220 V" D=0.5 "fs=1 kHz" "Ra=1 ohm" "La=10 mH" "Ea=100 V"
I_max = 12.74942723 A
I_min = 7.250572773 A
dI = 5.498854453 A
I_avg = 10 A
  by: Armature current ripple of a chopper-fed DC motor  (I_max = (Vs/Ra)(1 - e^(-DT/tau))/(1 - e^(-T/tau)) - Ea/Ra; I_min = (Vs/Ra)(e^(DT/tau) - 1)/(e^(T/tau) - 1) - Ea/Ra; I_avg = (D Vs - Ea)/Ra)
  Vs = 220 V
  D = 0.5
  fs = 1000 Hz
  Ra = 1 ohm
  La = 0.01 H
  Ea = 100 V
  assumes: continuous conduction (I_min > 0); constant speed (constant back EMF) over a period
```

In the worked example, Vs = 220 V, D = 0.5, fs = 1 kHz, Ra = 1 ohm, La = 10 mH, Ea = 100 V go through the equation above and give I_max = 12.74942723 A, I_min = 7.250572773 A, dI = 5.498854453 A, I_avg = 10 A. It applies only when continuous conduction (I_min > 0); constant speed (constant back EMF) over a period: outside that it says what is wrong instead of returning a number.

<a id="pe_bldc_six_step"></a>

#### BLDC motor with six-step (120 degree) drive — `pe_bldc_six_step`

The speed, torque, no-load speed, stall current, electrical frequency and commutation interval of a trapezoidal brushless DC motor driven six-step from a PWM-chopped dc bus (two phases conduct in series).

$$
DV_{dc} = 2R_{ph}I + 2K_{e,ph}\omega,\quad T = 2K_{e,ph}I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vdc` | in | dc bus voltage | V |
| `D` | in | PWM duty cycle (a fraction or %) | — |
| `Rph` | in | phase resistance | ohm |
| `Keph` | in | phase back-EMF constant (flat top) | V*s/rad |
| `I` | in | dc link (phase) current | A |
| `p` | in | pole pairs | — |
| `omega` | out | speed | rad/s |
| `rpm` | out | speed in rpm | — |
| `T` | out | torque | N*m |
| `omega_no_load` | out | no-load speed | rad/s |
| `I_stall` | out | stall current | A |
| `f_e` | out | electrical frequency | Hz |
| `t_commutation` | out | time between commutations | s |

**Applies when:** ideal trapezoidal EMF and block currents; winding inductance and commutation neglected.

```shell
$ power-electronics solve pe_bldc_six_step "Vdc=48 V" D=0.8 "Rph=0.1 ohm" "Keph=0.05 V*s/rad" "I=10 A" p=4
omega = 364 rad/s
rpm = 3475.943957
T = 1 N*m
omega_no_load = 384 rad/s
I_stall = 192 A
f_e = 231.7295971 Hz
t_commutation = 0.0007192290874 s
  by: BLDC motor with six-step (120 degree) drive  (D Vdc = 2 Rph I + 2 Keph w; T = 2 Keph I; f_e = p w/(2 pi))
  Vdc = 48 V
  D = 0.8
  Rph = 0.1 ohm
  Keph = 0.05 V*s/rad
  I = 10 A
  assumes: ideal trapezoidal EMF and block currents; winding inductance and commutation neglected
```

In the worked example, Vdc = 48 V, D = 0.8, Rph = 0.1 ohm, Keph = 0.05 V*s/rad, I = 10 A, p = 4 go through the equation above and give omega = 364 rad/s, rpm = 3475.943957, T = 1 N*m, omega_no_load = 384 rad/s. It applies only when ideal trapezoidal EMF and block currents; winding inductance and commutation neglected: outside that it says what is wrong instead of returning a number.

<a id="pe_dc_motor_chopper_drive"></a>

#### Chopper-fed separately excited DC motor: speed and torque — `pe_dc_motor_chopper_drive`

The armature current, average armature voltage, back EMF, speed (rad/s and rpm), output power and armature efficiency of a separately excited DC motor fed by a step-down chopper at a duty cycle and carrying a load torque (continuous current, ripple neglected).

$$
V_a = DV_s,\quad I_a = \frac{T_L}{K},\quad \omega = \frac{V_a - I_aR_a}{K}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | dc supply voltage | V |
| `D` | in | chopper duty cycle (a fraction or %) | — |
| `Ra` | in | armature resistance | ohm |
| `K` | in | motor constant K = Ke = Kt | V*s/rad |
| `TL` | in | load torque | N*m |
| `Ia` | out | armature current TL/K | A |
| `Va` | out | average armature voltage D Vs | V |
| `Ea` | out | back EMF | V |
| `omega` | out | speed | rad/s |
| `rpm` | out | speed in rpm | — |
| `P_out` | out | developed (output) power | W |
| `efficiency` | out | armature efficiency Ea/Va | — |

**Applies when:** continuous armature current; constant field.

```shell
$ power-electronics solve pe_dc_motor_chopper_drive "Vs=220 V" D=0.6 "Ra=0.5 ohm" "K=1.2 V*s/rad" "TL=24 N*m"
Ia = 20 A
Va = 132 V
Ea = 122 V
omega = 101.6666667 rad/s
rpm = 970.8451529
P_out = 2440 W
efficiency = 0.9242424242
  by: Chopper-fed separately excited DC motor: speed and torque  (Va = D Vs; Ia = TL/K; Ea = Va - Ia Ra; w = Ea/K)
  Vs = 220 V
  D = 0.6
  Ra = 0.5 ohm
  K = 1.2 V*s/rad
  TL = 24 N*m
  assumes: continuous armature current; constant field
```

In the worked example, Vs = 220 V, D = 0.6, Ra = 0.5 ohm, K = 1.2 V*s/rad, TL = 24 N*m go through the equation above and give Ia = 20 A, Va = 132 V, Ea = 122 V, omega = 101.6666667 rad/s. It applies only when continuous armature current; constant field: outside that it says what is wrong instead of returning a number.

<a id="pe_induction_motor_vf"></a>

#### Induction motor constant V/f speed control — `pe_induction_motor_vf`

The supply voltage, synchronous speed, rotor speed, slip and slip frequency of an induction motor under constant V/f control below base frequency, from its rated voltage, frequency and speed (linear torque-slip region, torque as a fraction of rated).

$$
\frac{V}{f} = \text{const},\quad n = \frac{120f}{P} - \left(n_{s,r} - n_r\right)\frac{T}{T_r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V_rated` | in | rated line voltage | V |
| `f_rated` | in | rated (base) frequency | Hz |
| `poles` | in | number of poles | — |
| `n_rated` | in | rated speed in rpm | — |
| `f` | in | operating frequency | Hz |
| `T_ratio` | in | load torque as a fraction of rated torque (or %) | — |
| `V` | out | supply voltage at f (constant V/f) | V |
| `ns` | out | synchronous speed in rpm | — |
| `n` | out | rotor speed in rpm | — |
| `slip` | out | slip | — |
| `f_slip` | out | slip (rotor) frequency | Hz |

**Applies when:** f <= f_rated; linear torque-slip region (T_ratio <= 1.5); stator resistance drop neglected.

```shell
$ power-electronics solve pe_induction_motor_vf "V_rated=400 V" "f_rated=50 Hz" poles=4 n_rated=1450 "f=30 Hz" T_ratio=1
V = 240 V
ns = 900
n = 850
slip = 0.05555555556
f_slip = 1.666666667 Hz
  by: Induction motor constant V/f speed control  (V = V_rated f/f_rated; ns = 120 f/P; the slip speed at constant V/f is proportional to torque: n = ns - (ns,rated - n_rated) T/T_rated)
  V_rated = 400 V
  f_rated = 50 Hz
  n_rated = 1450
  f = 30 Hz
  T_ratio = 1
  assumes: f <= f_rated; linear torque-slip region (T_ratio <= 1.5); stator resistance drop neglected
```

In the worked example, V_rated = 400 V, f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 1 go through the equation above and give V = 240 V, ns = 900, n = 850, slip = 0.05555555556. It applies only when f <= f_rated; linear torque-slip region (T_ratio <= 1.5); stator resistance drop neglected: outside that it says what is wrong instead of returning a number.

<a id="pe_pmsm_mtpa"></a>

#### Maximum torque per ampere (MTPA) of an interior PMSM — `pe_pmsm_mtpa`

The d- and q-axis currents, current advance angle and torque that give the most torque for a given current magnitude in a salient (interior) permanent-magnet synchronous motor, and the torque with id = 0 for comparison.

$$
i_d = \frac{\psi_f - \sqrt{\psi_f^2 + 8(L_q-L_d)^2 I_s^2}}{4(L_q-L_d)},\quad i_q = \sqrt{I_s^2 - i_d^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ld` | in | d-axis inductance | H |
| `Lq` | in | q-axis inductance (Lq > Ld) | H |
| `psi_f` | in | magnet flux linkage | Wb |
| `p` | in | pole pairs | — |
| `Is` | in | current magnitude (peak) | A |
| `id` | out | d-axis current (negative) | A |
| `iq` | out | q-axis current | A |
| `beta_deg` | out | current advance angle from the q axis, degrees | — |
| `Te` | out | maximum torque at this current | N*m |
| `Te_id0` | out | torque with zero d-axis current | N*m |

**Applies when:** Lq > Ld; magnetic saturation neglected.

```shell
$ power-electronics solve pe_pmsm_mtpa "Ld=0.4 mH" "Lq=1 mH" "psi_f=80 mWb" p=4 "Is=100 A"
id = -44.84026266 A
iq = 89.38316869 A
beta_deg = 26.64124423
Te = 57.33259411 N*m
Te_id0 = 48 N*m
  by: Maximum torque per ampere (MTPA) of an interior PMSM  (id = (psi_f - sqrt(psi_f^2 + 8 (Lq - Ld)^2 Is^2)) / (4 (Lq - Ld)); iq = sqrt(Is^2 - id^2))
  Ld = 0.0004 H
  Lq = 0.001 H
  psi_f = 0.08 Wb
  Is = 100 A
  assumes: Lq > Ld; magnetic saturation neglected
```

In the worked example, Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, Is = 100 A go through the equation above and give id = -44.84026266 A, iq = 89.38316869 A, beta_deg = 26.64124423, Te = 57.33259411 N*m. It applies only when Lq > Ld; magnetic saturation neglected: outside that it says what is wrong instead of returning a number.

<a id="pe_dc_drive_controlled_rectifier"></a>

#### Phase-controlled rectifier DC motor drive — `pe_dc_drive_controlled_rectifier`

The armature voltage, back EMF, speed, torque and developed power of a separately excited DC motor fed by a single-phase or three-phase fully controlled thyristor bridge at a firing angle, with continuous armature current.

$$
V_a = \frac{2\sqrt2V_s}{\pi}\cos\alpha\ (1\phi),\ \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha\ (3\phi),\quad \omega = \frac{V_a - I_aR_a}{K}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `converter` | in | single-phase or three-phase full converter | — |
| `Vs` | in | RMS supply voltage (line-to-line for three-phase) | V |
| `alpha` | in | firing angle | rad |
| `Ra` | in | armature resistance | ohm |
| `K` | in | motor constant | V*s/rad |
| `Ia` | in | armature current | A |
| `Va` | out | average armature voltage | V |
| `Ea` | out | back EMF | V |
| `omega` | out | speed | rad/s |
| `rpm` | out | speed in rpm | — |
| `T` | out | developed torque K Ia | N*m |
| `P` | out | developed power Ea Ia | W |
| `mode` | out | motoring or reverse | — |

**Applies when:** continuous armature current.

```shell
$ power-electronics solve pe_dc_drive_controlled_rectifier converter=single-phase "Vs=230 V" "alpha=45 deg" "Ra=1 ohm" "K=1 V*s/rad" "Ia=10 A"
Va = 146.4225476 V
Ea = 136.4225476 V
omega = 136.4225476 rad/s
rpm = 1302.739368
T = 10 N*m
P = 1364.225476 W
mode = motoring
  by: Phase-controlled rectifier DC motor drive  (Va = 2 sqrt(2) Vs cos(alpha)/pi (single-phase) or 3 sqrt(2) VLL cos(alpha)/pi (three-phase); Ea = Va - Ia Ra; w = Ea/K)
  Vs = 230 V
  alpha = 0.7853981634 rad
  Ra = 1 ohm
  K = 1 V*s/rad
  Ia = 10 A
  converter = single-phase
  assumes: continuous armature current
```

In the worked example, converter = single-phase, Vs = 230 V, alpha = 45 deg, Ra = 1 ohm, K = 1 V*s/rad, Ia = 10 A go through the equation above and give Va = 146.4225476 V, Ea = 136.4225476 V, omega = 136.4225476 rad/s, rpm = 1302.739368. It applies only when continuous armature current: outside that it says what is wrong instead of returning a number.

<a id="pe_pmsm_dq_model"></a>

#### PMSM steady state in the dq frame: voltages and torque — `pe_pmsm_dq_model`

The electrical speed, d- and q-axis voltages, voltage magnitude, electromagnetic torque, mechanical, input and copper power and power factor of a permanent-magnet synchronous motor in steady state from its dq currents (amplitude-invariant Park transform).

$$
v_d = R_si_d - \omega_eL_qi_q,\ v_q = R_si_q + \omega_e(L_di_d+\psi_f),\ T_e = \tfrac32p\left(\psi_fi_q + (L_d-L_q)i_di_q\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rs` | in | stator resistance per phase | ohm |
| `Ld` | in | d-axis inductance | H |
| `Lq` | in | q-axis inductance | H |
| `psi_f` | in | permanent-magnet flux linkage | Wb |
| `p` | in | pole pairs | — |
| `id` | in | d-axis current (negative for field weakening) | A |
| `iq` | in | q-axis current | A |
| `omega_m` | in | mechanical speed | rad/s |
| `omega_e` | out | electrical speed p w_m | rad/s |
| `vd` | out | d-axis voltage | V |
| `vq` | out | q-axis voltage | V |
| `V_peak` | out | peak phase voltage magnitude | V |
| `Te` | out | electromagnetic torque | N*m |
| `P_mech` | out | mechanical power | W |
| `P_in` | out | electrical input power | W |
| `P_cu` | out | stator copper loss | W |
| `PF` | out | power factor | — |

**Applies when:** steady state, sinusoidal machine, iron loss neglected; amplitude-invariant transform.

```shell
$ power-electronics solve pe_pmsm_dq_model "Rs=0.05 ohm" "Ld=0.4 mH" "Lq=1 mH" "psi_f=80 mWb" p=4 "id=-20 A" "iq=60 A" "omega_m=300 rad/s"
omega_e = 1200 rad/s
vd = -73 V
vq = 89.4 V
V_peak = 115.4181961 V
Te = 33.12 N*m
P_mech = 9936 W
P_in = 10236 W
P_cu = 300 W
PF = 0.9348345181
  by: PMSM steady state in the dq frame: voltages and torque  (vd = Rs id - omega_e Lq iq; vq = Rs iq + omega_e (Ld id + psi_f); Te = 1.5 p (psi_f iq + (Ld - Lq) id iq); P_in = 1.5 (vd id + vq iq))
  Rs = 0.05 ohm
  Ld = 0.0004 H
  Lq = 0.001 H
  psi_f = 0.08 Wb
  id = -20 A
  iq = 60 A
  omega_m = 300 rad/s
  assumes: steady state, sinusoidal machine, iron loss neglected; amplitude-invariant transform
```

In the worked example, Rs = 0.05 ohm, Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, id = -20 A, iq = 60 A, omega_m = 300 rad/s go through the equation above and give omega_e = 1200 rad/s, vd = -73 V, vq = 89.4 V, V_peak = 115.4181961 V. It applies only when steady state, sinusoidal machine, iron loss neglected; amplitude-invariant transform: outside that it says what is wrong instead of returning a number.

<a id="pe_regenerative_braking_chopper"></a>

#### Regenerative braking of a DC motor with a chopper — `pe_regenerative_braking_chopper`

The back EMF, average armature voltage, armature current, braking torque, mechanical power absorbed, power returned to the supply and braking efficiency of a DC motor braked regeneratively by a step-up chopper.

$$
I_a = \frac{K\omega - (1-D)V_s}{R_a},\quad P_{regen} = (1-D)V_sI_a
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | dc supply voltage | V |
| `D` | in | chopper duty cycle (a fraction or %) | — |
| `Ra` | in | armature resistance | ohm |
| `K` | in | motor constant | V*s/rad |
| `omega` | in | motor speed | rad/s |
| `Ea` | out | back EMF K w | V |
| `Va` | out | average armature voltage | V |
| `Ia` | out | armature (braking) current | A |
| `T_brake` | out | braking torque | N*m |
| `P_mech` | out | mechanical power absorbed Ea Ia | W |
| `P_regen` | out | power returned to the supply | W |
| `efficiency` | out | braking efficiency P_regen/P_mech | — |

**Applies when:** Ea > (1 - D) Vs; continuous armature current.

```shell
$ power-electronics solve pe_regenerative_braking_chopper "Vs=220 V" D=0.5 "Ra=0.5 ohm" "K=1 V*s/rad" "omega=150 rad/s"
Ea = 150 V
Va = 110 V
Ia = 80 A
T_brake = 80 N*m
P_mech = 12000 W
P_regen = 8800 W
efficiency = 0.7333333333
  by: Regenerative braking of a DC motor with a chopper  (Ea = K w; Va = (1 - D) Vs; Ia = (Ea - Va)/Ra; P_regen = Va Ia)
  Vs = 220 V
  D = 0.5
  Ra = 0.5 ohm
  K = 1 V*s/rad
  omega = 150 rad/s
  assumes: Ea > (1 - D) Vs; continuous armature current
```

In the worked example, Vs = 220 V, D = 0.5, Ra = 0.5 ohm, K = 1 V*s/rad, omega = 150 rad/s go through the equation above and give Ea = 150 V, Va = 110 V, Ia = 80 A, T_brake = 80 N*m. It applies only when Ea > (1 - D) Vs; continuous armature current: outside that it says what is wrong instead of returning a number.

<a id="pe_pmsm_field_weakening"></a>

#### Surface PMSM field weakening: base speed, maximum speed and torque — `pe_pmsm_field_weakening`

The base speed, the maximum speed, and at a given speed the d- and q-axis currents, torque and power of a surface (non-salient) permanent-magnet motor at its current and voltage limits (field weakening above base speed; stator resistance neglected).

$$
\omega_b = \frac{V_{max}}{p\sqrt{\psi_f^2 + (L_sI_{max})^2}},\ \omega_{max} = \frac{V_{max}}{p(\psi_f - L_sI_{max})},\ i_d = \frac{(V_{max}/p\omega)^2 - \psi_f^2 - L_s^2I_{max}^2}{2\psi_fL_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ls` | in | synchronous inductance (Ld = Lq) | H |
| `psi_f` | in | magnet flux linkage | Wb |
| `p` | in | pole pairs | — |
| `Imax` | in | current limit (peak) | A |
| `Vmax` | in | voltage limit (peak phase) | V |
| `omega` | in | mechanical speed of interest | rad/s |
| `omega_base` | out | base speed (mechanical) | rad/s |
| `omega_max` | out | maximum speed (mechanical) | rad/s |
| `rpm_base` | out | base speed in rpm | — |
| `id` | out | d-axis current at omega | A |
| `iq` | out | q-axis current at omega | A |
| `T` | out | available torque at omega | N*m |
| `P` | out | available power at omega | W |

**Applies when:** psi_f > Ls Imax (finite maximum speed); omega < omega_max; Rs = 0.

```shell
$ power-electronics solve pe_pmsm_field_weakening "Ls=1 mH" "psi_f=100 mWb" p=4 "Imax=60 A" "Vmax=200 V" "omega=600 rad/s"
omega_base = 428.7464629 rad/s
omega_max = 1250 rad/s
rpm_base = 4094.227134
id = -33.27777778 A
iq = 49.92584006 A
T = 29.95550404 N*m
P = 17973.30242 W
  by: Surface PMSM field weakening: base speed, maximum speed and torque  (w_b = Vmax/(p sqrt(psi_f^2 + (Ls Imax)^2)); w_max = Vmax/(p (psi_f - Ls Imax)); above w_b: id = ((Vmax/(p w))^2 - psi_f^2 - Ls^2 Imax^2)/(2 psi_f Ls))
  Ls = 0.001 H
  psi_f = 0.1 Wb
  Imax = 60 A
  Vmax = 200 V
  omega = 600 rad/s
  assumes: psi_f > Ls Imax (finite maximum speed); omega < omega_max; Rs = 0
```

In the worked example, Ls = 1 mH, psi_f = 100 mWb, p = 4, Imax = 60 A, Vmax = 200 V, omega = 600 rad/s go through the equation above and give omega_base = 428.7464629 rad/s, omega_max = 1250 rad/s, rpm_base = 4094.227134, id = -33.27777778 A. It applies only when psi_f > Ls Imax (finite maximum speed); omega < omega_max; Rs = 0: outside that it says what is wrong instead of returning a number.

### Power quality (3)

<a id="pe_pulse_waveform_rms"></a>

#### Average and RMS of a rectangular pulse train — `pe_pulse_waveform_rms`

The average, RMS and AC-RMS values, form factor and ripple factor of a unipolar rectangular pulse train of amplitude A and duty cycle D (a PWM switch voltage or current).

$$
V_{avg} = DA,\quad V_{rms} = \sqrt{D}\,A,\quad RF = \sqrt{\tfrac1D - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | pulse amplitude (any unit) | — |
| `D` | in | duty cycle (a fraction or %) | — |
| `avg` | out | average value D A | — |
| `rms` | out | RMS value sqrt(D) A | — |
| `ac_rms` | out | RMS of the AC component | — |
| `form_factor` | out | form factor rms/avg | — |
| `ripple_factor` | out | ripple factor ac_rms/avg | — |

**Applies when:** 0 < D < 1.

```shell
$ power-electronics solve pe_pulse_waveform_rms A=12 D=0.25
avg = 3
rms = 6
ac_rms = 5.196152423
form_factor = 2
ripple_factor = 1.732050808
  by: Average and RMS of a rectangular pulse train  (avg = D A; rms = sqrt(D) A; ac = A sqrt(D (1 - D)); FF = 1/sqrt(D); RF = sqrt(1/D - 1))
  A = 12
  D = 0.25
  assumes: 0 < D < 1
```

In the worked example, A = 12, D = 0.25 go through the equation above and give avg = 3, rms = 6, ac_rms = 5.196152423, form_factor = 2. It applies only when 0 < D < 1: outside that it says what is wrong instead of returning a number.

<a id="pe_power_factor_distortion"></a>

#### Power factor with harmonic distortion — `pe_power_factor_distortion`

The displacement power factor, distortion factor and true power factor of a load drawing a distorted current from a sinusoidal voltage, from the current THD and the displacement angle of its fundamental.

$$
PF = \frac{\cos\phi_1}{\sqrt{1+THD_i^2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `THD` | in | current total harmonic distortion (a fraction or %) | — |
| `phi` | in | displacement angle of the fundamental current (lagging positive) | rad |
| `DPF` | out | displacement power factor cos(phi) | — |
| `distortion_factor` | out | distortion factor | — |
| `PF` | out | true power factor | — |

**Applies when:** sinusoidal voltage; -90 <= phi <= 90 degrees.

```shell
$ power-electronics solve pe_power_factor_distortion THD=0.483 "phi=0 deg"
DPF = 1
distortion_factor = 0.900466522
PF = 0.900466522
  by: Power factor with harmonic distortion  (PF = cos(phi) / sqrt(1 + THD^2))
  THD = 0.483
  phi = 0 rad
  assumes: sinusoidal voltage; -90 <= phi <= 90 degrees
```

In the worked example, THD = 0.483, phi = 0 deg go through the equation above and give DPF = 1, distortion_factor = 0.900466522, PF = 0.900466522. It applies only when sinusoidal voltage; -90 <= phi <= 90 degrees: outside that it says what is wrong instead of returning a number.

<a id="pe_thd_from_harmonics"></a>

#### Total harmonic distortion from harmonic values — `pe_thd_from_harmonics`

The total harmonic distortion, RMS value and distortion factor of a waveform from the RMS value of its fundamental and of its harmonics 2, 3, 4, ... in order.

$$
THD = \frac{\sqrt{\sum_{h\ge2} V_h^2}}{V_1},\quad V_{rms} = V_1\sqrt{1+THD^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V1` | in | RMS value of the fundamental (any unit, the same as the harmonics) | — |
| `Vh` | in | RMS values of the harmonics in order of harmonic number, starting with the second | — |
| `THD` | out | total harmonic distortion | — |
| `THD_pct` | out | THD in percent | — |
| `Vrms` | out | RMS value of the waveform | — |
| `distortion_factor` | out | distortion factor | — |

**Applies when:** V1 > 0, harmonics >= 0.

```shell
$ power-electronics solve pe_thd_from_harmonics V1=230 "Vh=[0, 11.5, 0, 6.9, 0, 4.6]"
THD = 0.06164414003
THD_pct = 6.164414003
Vrms = 230.4365856
distortion_factor = 0.9981053979
  by: Total harmonic distortion from harmonic values  (THD = sqrt(sum Vh^2)/V1; Vrms = sqrt(V1^2 + sum Vh^2); DF = V1/Vrms = 1/sqrt(1 + THD^2))
  V1 = 230
  assumes: V1 > 0, harmonics >= 0
```

In the worked example, V1 = 230, Vh = [0, 11.5, 0, 6.9, 0, 4.6] go through the equation above and give THD = 0.06164414003, THD_pct = 6.164414003, Vrms = 230.4365856, distortion_factor = 0.9981053979. It applies only when V1 > 0, harmonics >= 0: outside that it says what is wrong instead of returning a number.

### Rectifiers (11)

<a id="pe_capacitor_filter_exact"></a>

#### Capacitor-input rectifier filter: exact ripple and peak diode current — `pe_capacitor_filter_exact`

The exact steady state of a half-wave or full-wave rectifier with a smoothing capacitor and resistive load: diode turn-off angle, recharge angle, peak-to-peak ripple, average output voltage, peak diode current and conduction angle (no small-ripple approximation).

$$
\theta = \pi - \tan^{-1}(\omega RC),\ \sin\alpha = \sin\theta\,e^{-(P+\alpha-\theta)/\omega RC},\ I_{D,pk} = V_p\left(\omega C\cos\alpha+\frac{\sin\alpha}{R}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `circuit` | in | half-wave or full-wave | — |
| `Vp` | in | peak source voltage (after any diode drop) | V |
| `f` | in | line frequency | Hz |
| `R` | in | load resistance | ohm |
| `C` | in | filter capacitance | F |
| `theta_off_deg` | out | angle at which the diode turns off, degrees | — |
| `alpha_deg` | out | angle at which the diode turns on again, degrees | — |
| `dV` | out | peak-to-peak ripple | V |
| `V_avg` | out | average output voltage | V |
| `I_D_peak` | out | peak diode current | A |
| `conduction_deg` | out | diode conduction angle, degrees | — |

**Applies when:** ideal diode and source, steady state.

```shell
$ power-electronics solve pe_capacitor_filter_exact circuit=half-wave "Vp=169.7 V" "f=0.06 kHz" "R=500 ohm" "C=100 uF"
theta_off_deg = 93.03678865
alpha_deg = 48.23082389
dV = 43.13189012 V
V_avg = 148.1028626 V
I_D_peak = 4.514737529 A
conduction_deg = 44.80596476
  by: Capacitor-input rectifier filter: exact ripple and peak diode current  (theta = pi - atan(w R C); sin(alpha) = sin(theta) exp(-(P + alpha - theta)/(w R C)) (bisection), P = 2 pi or pi; dV = Vp (1 - sin alpha); I_D,peak = Vp (w C cos alpha + sin alpha/R))
  Vp = 169.7 V
  f = 60 Hz
  R = 500 ohm
  C = 0.0001 F
  circuit = half-wave
  assumes: ideal diode and source, steady state
```

In the worked example, circuit = half-wave, Vp = 169.7 V, f = 0.06 kHz, R = 500 ohm, C = 100 uF go through the equation above and give theta_off_deg = 93.03678865, alpha_deg = 48.23082389, dV = 43.13189012 V, V_avg = 148.1028626 V. It applies only when ideal diode and source, steady state: outside that it says what is wrong instead of returning a number.

<a id="pe_single_phase_overlap"></a>

#### Commutation overlap in a single-phase thyristor bridge — `pe_single_phase_overlap`

The overlap (commutation) angle and the reduced average output voltage of a single-phase fully controlled bridge with source inductance Ls and a constant dc current.

$$
\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_s I_d}{V_m},\quad V_{dc} = \frac{2V_m}{\pi}\cos\alpha - \frac{2\omega L_sI_d}{\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | RMS source voltage | V |
| `f` | in | line frequency | Hz |
| `Ls` | in | source (commutating) inductance | H |
| `Id` | in | constant dc current | A |
| `alpha` | in | firing angle | rad |
| `u_deg` | out | overlap angle in degrees | — |
| `Vdc` | out | average output voltage with overlap | V |
| `dV` | out | voltage lost to commutation | V |

**Applies when:** commutation finishes within the half-cycle.

```shell
$ power-electronics solve pe_single_phase_overlap "Vs=230 V" "f=0.05 kHz" "Ls=2 mH" "Id=20 A" "alpha=30 deg"
u_deg = 7.930416019
Vdc = 171.3302643 V
dV = 8 V
  by: Commutation overlap in a single-phase thyristor bridge  (cos(alpha + u) = cos(alpha) - 2 w Ls Id/Vm; Vdc = 2 Vm cos(alpha)/pi - 2 w Ls Id/pi)
  Vs = 230 V
  f = 50 Hz
  Ls = 0.002 H
  Id = 20 A
  alpha = 0.5235987756 rad
  assumes: commutation finishes within the half-cycle
```

In the worked example, Vs = 230 V, f = 0.05 kHz, Ls = 2 mH, Id = 20 A, alpha = 30 deg go through the equation above and give u_deg = 7.930416019, Vdc = 171.3302643 V, dV = 8 V. It applies only when commutation finishes within the half-cycle: outside that it says what is wrong instead of returning a number.

<a id="pe_three_phase_overlap"></a>

#### Commutation overlap in a three-phase thyristor bridge — `pe_three_phase_overlap`

The overlap angle, the average output voltage and the voltage lost to commutation of a six-pulse thyristor (or diode, alpha = 0) bridge with source inductance Ls per phase and a constant dc current.

$$
\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_sI_d}{\sqrt2 V_{LL}},\quad V_{dc} = \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha - \frac{3\omega L_sI_d}{\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VLL` | in | RMS line-to-line voltage | V |
| `f` | in | line frequency | Hz |
| `Ls` | in | source inductance per phase | H |
| `Id` | in | constant dc current | A |
| `alpha` | in | firing angle | rad |
| `u_deg` | out | overlap angle in degrees | — |
| `Vdc` | out | average output voltage | V |
| `dV` | out | voltage lost to commutation | V |

**Applies when:** commutation finishes before the next one starts.

```shell
$ power-electronics solve pe_three_phase_overlap "VLL=400 V" "f=0.05 kHz" "Ls=0.5 mH" "Id=100 A" "alpha=30 deg"
u_deg = 5.856228511
Vdc = 452.8180807 V
dV = 15 V
  by: Commutation overlap in a three-phase thyristor bridge  (cos(alpha + u) = cos(alpha) - 2 w Ls Id/(sqrt(2) VLL); Vdc = 3 sqrt(2) VLL cos(alpha)/pi - 3 w Ls Id/pi)
  VLL = 400 V
  f = 50 Hz
  Ls = 0.0005 H
  Id = 100 A
  alpha = 0.5235987756 rad
  assumes: commutation finishes before the next one starts
```

In the worked example, VLL = 400 V, f = 0.05 kHz, Ls = 0.5 mH, Id = 100 A, alpha = 30 deg go through the equation above and give u_deg = 5.856228511, Vdc = 452.8180807 V, dV = 15 V. It applies only when commutation finishes before the next one starts: outside that it says what is wrong instead of returning a number.

<a id="pe_half_wave_rl_rectifier"></a>

#### Half-wave diode rectifier with an RL load — `pe_half_wave_rl_rectifier`

The extinction angle (where the load current returns to zero), the load impedance angle and magnitude, and the average output voltage and current of a half-wave diode rectifier feeding a series RL load.

$$
\sin(\beta-\theta)+\sin\theta\,e^{-\beta/\omega\tau} = 0,\quad V_{avg} = \frac{V_m(1-\cos\beta)}{2\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | RMS source voltage | V |
| `f` | in | line frequency | Hz |
| `R` | in | load resistance | ohm |
| `L` | in | load inductance | H |
| `beta_deg` | out | extinction angle in degrees | — |
| `theta_deg` | out | load impedance angle atan(wL/R) in degrees | — |
| `Z` | out | load impedance magnitude | ohm |
| `V_avg` | out | average output voltage | V |
| `I_avg` | out | average load current | A |

**Applies when:** ideal diode, no freewheeling diode.

```shell
$ power-electronics solve pe_half_wave_rl_rectifier "Vs=120 V" "f=0.06 kHz" "R=100 ohm" "L=100 mH"
beta_deg = 200.6578643
theta_deg = 20.65599738
Z = 106.8701223 ohm
V_avg = 52.28236942 V
I_avg = 0.5228236942 A
  by: Half-wave diode rectifier with an RL load  (sin(beta - theta) + sin(theta) exp(-beta/(wL/R)) = 0 (bisection on pi < beta < 2 pi); Vavg = Vm (1 - cos beta)/(2 pi); Iavg = Vavg/R)
  Vs = 120 V
  f = 60 Hz
  R = 100 ohm
  L = 0.1 H
  assumes: ideal diode, no freewheeling diode
```

In the worked example, Vs = 120 V, f = 0.06 kHz, R = 100 ohm, L = 100 mH go through the equation above and give beta_deg = 200.6578643, theta_deg = 20.65599738, Z = 106.8701223 ohm, V_avg = 52.28236942 V. It applies only when ideal diode, no freewheeling diode: outside that it says what is wrong instead of returning a number.

<a id="pe_rectifier_line_current_pf"></a>

#### Rectifier line current: THD, displacement and power factor — `pe_rectifier_line_current_pf`

The RMS and fundamental line current, current THD, displacement power factor, power factor, real power and apparent power of a single-phase or three-phase (six-pulse) bridge rectifier with a constant dc current at a firing angle (0 for diodes).

$$
PF = \frac{I_{s1}}{I_s}\cos\alpha,\quad \frac{I_{s1}}{I_s} = \frac{2\sqrt2}{\pi}\ (1\phi),\ \frac{3}{\pi}\ (3\phi)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `circuit` | in | single-phase or three-phase bridge | — |
| `Id` | in | constant dc output current | A |
| `Vs` | in | RMS source voltage (line-to-line for three-phase) | V |
| `alpha` | in | firing angle | rad |
| `Is_rms` | out | RMS line current | A |
| `Is1_rms` | out | RMS fundamental line current | A |
| `THD` | out | current THD | — |
| `DPF` | out | displacement power factor cos(alpha) | — |
| `PF` | out | power factor | — |
| `P` | out | real power | W |
| `S` | out | apparent power | VA |

**Applies when:** constant (ripple-free) dc current, no commutation overlap.

```shell
$ power-electronics solve pe_rectifier_line_current_pf circuit=single-phase "Id=10 A" "Vs=230 V" "alpha=0 deg"
Is_rms = 10 A
Is1_rms = 9.003163162 A
THD = 0.4834258476
DPF = 1
PF = 0.9003163162
P = 2070.727527 W
S = 2300 VA
  by: Rectifier line current: THD, displacement and power factor  (single-phase: Is = Id, Is1 = 2 sqrt(2) Id/pi (PF = 0.9 cos a); three-phase: Is = sqrt(2/3) Id, Is1 = sqrt(6) Id/pi (PF = 0.955 cos a))
  Id = 10 A
  Vs = 230 V
  alpha = 0 rad
  circuit = single-phase
  assumes: constant (ripple-free) dc current, no commutation overlap
```

In the worked example, circuit = single-phase, Id = 10 A, Vs = 230 V, alpha = 0 deg go through the equation above and give Is_rms = 10 A, Is1_rms = 9.003163162 A, THD = 0.4834258476, DPF = 1. It applies only when constant (ripple-free) dc current, no commutation overlap: outside that it says what is wrong instead of returning a number.

<a id="pe_rectifier_line_harmonics"></a>

#### Rectifier line-current harmonic spectrum — `pe_rectifier_line_harmonics`

The RMS value of harmonic h of the line current of a single-phase or six-pulse three-phase bridge with a constant dc current, its ratio to the fundamental (1/h for characteristic harmonics), and the THD of harmonics up to h.

$$
I_h = \frac{I_1}{h},\quad h = 6k\pm1\ (3\phi),\ h\ \text{odd}\ (1\phi)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `circuit` | in | single-phase or three-phase bridge | — |
| `Id` | in | constant dc output current | A |
| `h` | in | harmonic order | — |
| `Ih` | out | RMS current of harmonic h | A |
| `Ih_over_I1` | out | ratio to the fundamental | — |
| `THD_to_h` | out | THD counting harmonics up to order h | — |
| `I1` | out | RMS fundamental current | A |

**Applies when:** 1 <= h <= 49; constant dc current, no overlap.

```shell
$ power-electronics solve pe_rectifier_line_harmonics circuit=three-phase "Id=100 A" h=5
Ih = 15.59393602 A
Ih_over_I1 = 0.2
THD_to_h = 0.2
I1 = 77.96968012 A
  by: Rectifier line-current harmonic spectrum  (single-phase: odd h, Ih = 2 sqrt(2) Id/(h pi); three-phase: h = 6k +- 1, Ih = sqrt(6) Id/(h pi); others 0)
  Id = 100 A
  circuit = three-phase
  assumes: 1 <= h <= 49; constant dc current, no overlap
```

In the worked example, circuit = three-phase, Id = 100 A, h = 5 go through the equation above and give Ih = 15.59393602 A, Ih_over_I1 = 0.2, THD_to_h = 0.2, I1 = 77.96968012 A. It applies only when 1 <= h <= 49; constant dc current, no overlap: outside that it says what is wrong instead of returning a number.

<a id="pe_ac_voltage_controller"></a>

#### Single-phase AC voltage controller (resistive load) — `pe_ac_voltage_controller`

The RMS output voltage and current, load power, input power factor and the average and RMS thyristor currents of a single-phase AC voltage controller (antiparallel thyristors or a triac) with a resistive load at a firing angle.

$$
V_{o} = V_s\sqrt{1-\frac{\alpha}{\pi}+\frac{\sin2\alpha}{2\pi}},\quad PF = \frac{V_o}{V_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vs` | in | RMS source voltage | V |
| `alpha` | in | firing angle | rad |
| `R` | in | load resistance | ohm |
| `Vo_rms` | out | RMS output voltage | V |
| `Io_rms` | out | RMS load current | A |
| `P` | out | load power | W |
| `PF` | out | input power factor | — |
| `I_scr_avg` | out | average thyristor current | A |
| `I_scr_rms` | out | RMS thyristor current | A |

**Applies when:** 0 <= alpha < 180 degrees; resistive load.

```shell
$ power-electronics solve pe_ac_voltage_controller "Vs=230 V" "alpha=60 deg" "R=10 ohm"
Vo_rms = 206.2958829 V
Io_rms = 20.62958829 A
P = 4255.799131 W
PF = 0.8969386214
I_scr_avg = 7.765228227 A
I_scr_rms = 14.58732177 A
  by: Single-phase AC voltage controller (resistive load)  (Vo = Vs sqrt(1 - alpha/pi + sin(2 alpha)/(2 pi)); PF = Vo/Vs; I_SCR,avg = sqrt(2) Vs (1 + cos alpha)/(2 pi R))
  Vs = 230 V
  alpha = 1.047197551 rad
  R = 10 ohm
  assumes: 0 <= alpha < 180 degrees; resistive load
```

In the worked example, Vs = 230 V, alpha = 60 deg, R = 10 ohm go through the equation above and give Vo_rms = 206.2958829 V, Io_rms = 20.62958829 A, P = 4255.799131 W, PF = 0.8969386214. It applies only when 0 <= alpha < 180 degrees; resistive load: outside that it says what is wrong instead of returning a number.

<a id="pe_single_phase_controlled_rectifier"></a>

#### Single-phase phase-controlled thyristor rectifier — `pe_single_phase_controlled_rectifier`

The average, RMS and AC (ripple) output voltage of a single-phase thyristor rectifier at a firing angle: half-wave with a resistive load, fully controlled bridge with continuous load current, or semi-converter (half-controlled) with continuous current.

$$
V_{dc,HW} = \frac{V_m(1+\cos\alpha)}{2\pi},\ V_{dc,FB} = \frac{2V_m\cos\alpha}{\pi},\ V_{dc,semi} = \frac{V_m(1+\cos\alpha)}{\pi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `circuit` | in | half-wave, full-bridge or semi-converter | — |
| `Vs` | in | RMS source voltage | V |
| `alpha` | in | firing angle (delay from the natural commutation point) | rad |
| `Vdc` | out | average output voltage | V |
| `Vrms` | out | RMS output voltage | V |
| `Vac_rms` | out | RMS of the AC component | V |
| `mode` | out | rectifying or inverting (negative average voltage) | — |

**Applies when:** 0 <= alpha < 180 degrees; ideal thyristors, no source inductance.

```shell
$ power-electronics solve pe_single_phase_controlled_rectifier circuit=full-bridge "Vs=230 V" "alpha=30 deg"
Vdc = 179.3302643 V
Vrms = 230 V
Vac_rms = 144.0161668 V
mode = rectifying
  by: Single-phase phase-controlled thyristor rectifier  (half-wave: Vdc = Vm (1 + cos a)/(2 pi); full bridge: Vdc = 2 Vm cos a/pi; semi-converter: Vdc = Vm (1 + cos a)/pi)
  Vs = 230 V
  alpha = 0.5235987756 rad
  circuit = full-bridge
  assumes: 0 <= alpha < 180 degrees; ideal thyristors, no source inductance
```

In the worked example, circuit = full-bridge, Vs = 230 V, alpha = 30 deg go through the equation above and give Vdc = 179.3302643 V, Vrms = 230 V, Vac_rms = 144.0161668 V, mode = rectifying. It applies only when 0 <= alpha < 180 degrees; ideal thyristors, no source inductance: outside that it says what is wrong instead of returning a number.

<a id="pe_three_phase_diode_bridge"></a>

#### Three-phase diode bridge rectifier with resistive load — `pe_three_phase_diode_bridge`

The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage, and the load, diode and line currents of a six-pulse three-phase diode bridge feeding a resistor.

$$
V_{dc} = \frac{3\sqrt2\,V_{LL}}{\pi},\quad V_{rms} = \sqrt2 V_{LL}\sqrt{\tfrac12+\tfrac{3\sqrt3}{4\pi}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VLL` | in | RMS line-to-line source voltage | V |
| `f` | in | line frequency | Hz |
| `R` | in | load resistance | ohm |
| `Vdc` | out | average output voltage | V |
| `Vrms` | out | RMS output voltage | V |
| `ripple_factor` | out | ripple factor | — |
| `form_factor` | out | form factor | — |
| `f_ripple` | out | ripple frequency | Hz |
| `PIV` | out | diode peak inverse voltage | V |
| `Idc` | out | average load current | A |
| `I_diode_avg` | out | average diode current | A |
| `I_diode_rms` | out | RMS diode current | A |
| `Is_rms` | out | RMS line current | A |

**Applies when:** ideal diodes, no source inductance, resistive load.

```shell
$ power-electronics solve pe_three_phase_diode_bridge "VLL=400 V" "f=0.05 kHz" "R=50 ohm"
Vdc = 540.1897897 V
Vrms = 540.6652707 V
ripple_factor = 0.04196661381
form_factor = 1.000880211
f_ripple = 300 Hz
PIV = 565.6854249 V
Idc = 10.80379579 A
I_diode_avg = 3.601265265 A
I_diode_rms = 6.243064791 A
Is_rms = 8.829026898 A
  by: Three-phase diode bridge rectifier with resistive load  (Vm = sqrt(2) VLL; Vdc = 3 Vm/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3)/(4 pi)); diode avg = Idc/3)
  VLL = 400 V
  f = 50 Hz
  R = 50 ohm
  assumes: ideal diodes, no source inductance, resistive load
```

In the worked example, VLL = 400 V, f = 0.05 kHz, R = 50 ohm go through the equation above and give Vdc = 540.1897897 V, Vrms = 540.6652707 V, ripple_factor = 0.04196661381, form_factor = 1.000880211. It applies only when ideal diodes, no source inductance, resistive load: outside that it says what is wrong instead of returning a number.

<a id="pe_three_phase_controlled_rectifier"></a>

#### Three-phase fully controlled thyristor bridge — `pe_three_phase_controlled_rectifier`

The average and RMS output voltage, the power, and the thyristor and line currents of a six-pulse fully controlled thyristor bridge with continuous (constant) load current at a firing angle, rectifying or inverting.

$$
V_{dc} = \frac{3\sqrt2 V_{LL}}{\pi}\cos\alpha,\quad I_{s,rms} = \sqrt{\tfrac23}\,I_d
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `VLL` | in | RMS line-to-line source voltage | V |
| `alpha` | in | firing angle | rad |
| `Id` | in | constant dc output current | A |
| `Vdc` | out | average output voltage | V |
| `Vrms` | out | RMS output voltage | V |
| `P` | out | dc output power Vdc Id | W |
| `I_thy_avg` | out | average thyristor current | A |
| `I_thy_rms` | out | RMS thyristor current | A |
| `Is_rms` | out | RMS line current | A |
| `mode` | out | rectifying or inverting | — |

**Applies when:** 0 <= alpha < 180 degrees; continuous constant current, no source inductance.

```shell
$ power-electronics solve pe_three_phase_controlled_rectifier "VLL=415 V" "alpha=45 deg" "Id=50 A"
Vdc = 396.2958083 V
Vrms = 415 V
P = 19814.79041 W
I_thy_avg = 16.66666667 A
I_thy_rms = 28.86751346 A
Is_rms = 40.82482905 A
mode = rectifying
  by: Three-phase fully controlled thyristor bridge  (Vdc = 3 sqrt(2) VLL cos(alpha)/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3) cos(2 alpha)/(4 pi)); I_thy = Id/3 (avg), Id/sqrt(3) (rms))
  VLL = 415 V
  alpha = 0.7853981634 rad
  Id = 50 A
  assumes: 0 <= alpha < 180 degrees; continuous constant current, no source inductance
```

In the worked example, VLL = 415 V, alpha = 45 deg, Id = 50 A go through the equation above and give Vdc = 396.2958083 V, Vrms = 415 V, P = 19814.79041 W, I_thy_avg = 16.66666667 A. It applies only when 0 <= alpha < 180 degrees; continuous constant current, no source inductance: outside that it says what is wrong instead of returning a number.

<a id="pe_three_phase_half_wave_rectifier"></a>

#### Three-phase half-wave (three-pulse) diode rectifier — `pe_three_phase_half_wave_rectifier`

The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage and the load and diode currents of a three-pulse (star, half-wave) diode rectifier feeding a resistor.

$$
V_{dc} = \frac{3\sqrt3\,V_m}{2\pi},\quad V_{rms} = V_m\sqrt{\tfrac12+\tfrac{3\sqrt3}{8\pi}},\quad PIV = \sqrt3 V_m
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vph` | in | RMS phase (line-to-neutral) voltage | V |
| `f` | in | line frequency | Hz |
| `R` | in | load resistance | ohm |
| `Vdc` | out | average output voltage | V |
| `Vrms` | out | RMS output voltage | V |
| `ripple_factor` | out | ripple factor | — |
| `form_factor` | out | form factor | — |
| `f_ripple` | out | ripple frequency | Hz |
| `PIV` | out | diode peak inverse voltage | V |
| `Idc` | out | average load current | A |
| `I_diode_avg` | out | average diode current | A |
| `I_diode_rms` | out | RMS diode current | A |

**Applies when:** ideal diodes, resistive load.

```shell
$ power-electronics solve pe_three_phase_half_wave_rectifier "Vph=230 V" "f=0.05 kHz" "R=20 ohm"
Vdc = 268.9953964 V
Vrms = 273.4483021 V
ripple_factor = 0.1827065996
form_factor = 1.016553836
f_ripple = 150 Hz
PIV = 563.3826408 V
Idc = 13.44976982 A
I_diode_avg = 4.483256607 A
I_diode_rms = 7.893772542 A
  by: Three-phase half-wave (three-pulse) diode rectifier  (Vm = sqrt(2) Vph; Vdc = 3 sqrt(3) Vm/(2 pi); Vrms = Vm sqrt(1/2 + 3 sqrt(3)/(8 pi)); PIV = sqrt(3) Vm)
  Vph = 230 V
  f = 50 Hz
  R = 20 ohm
  assumes: ideal diodes, resistive load
```

In the worked example, Vph = 230 V, f = 0.05 kHz, R = 20 ohm go through the equation above and give Vdc = 268.9953964 V, Vrms = 273.4483021 V, ripple_factor = 0.1827065996, form_factor = 1.016553836. It applies only when ideal diodes, resistive load: outside that it says what is wrong instead of returning a number.

### Resonant converters (3)

<a id="pe_dual_active_bridge_power"></a>

#### Dual active bridge: power transfer and inductor current — `pe_dual_active_bridge_power`

The power transferred by a dual-active-bridge converter with single phase-shift modulation, the maximum transferable power, the inductor current at the switching instants of both bridges and its RMS value.

$$
P = \frac{nV_1V_2\,\phi(\pi-\phi)}{2\pi^2f_sL},\quad P_{max} = \frac{nV_1V_2}{8f_sL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V1` | in | primary dc voltage | V |
| `V2` | in | secondary dc voltage | V |
| `n` | in | turns ratio Np/Ns | — |
| `L` | in | series (leakage) inductance referred to the primary | H |
| `fs` | in | switching frequency | Hz |
| `phi` | in | phase shift of the secondary bridge | rad |
| `P` | out | transferred power | W |
| `P_max` | out | maximum transferable power | W |
| `i0` | out | inductor current when the primary bridge switches | A |
| `i_phi` | out | inductor current when the secondary bridge switches | A |
| `I_rms` | out | RMS inductor current | A |

**Applies when:** square-wave bridges, single phase shift 0 < phi <= 90 degrees.

```shell
$ power-electronics solve pe_dual_active_bridge_power "V1=400 V" "V2=48 V" n=8 "L=50 uH" "fs=100 kHz" "phi=30 deg"
P = 2133.333333 W
P_max = 3840 W
i0 = -7.2 A
i_phi = 5.866666667 A
I_rms = 6.175699091 A
  by: Dual active bridge: power transfer and inductor current  (P = n V1 V2 phi (pi - phi)/(2 pi^2 fs L); i(0) = -(V1 pi + nV2 (2 phi - pi))/(2 w L); i(phi) = i(0) + (V1 + nV2) phi/(w L))
  V1 = 400 V
  V2 = 48 V
  n = 8
  L = 5e-05 H
  fs = 100000 Hz
  phi = 0.5235987756 rad
  assumes: square-wave bridges, single phase shift 0 < phi <= 90 degrees
```

In the worked example, V1 = 400 V, V2 = 48 V, n = 8, L = 50 uH, fs = 100 kHz, phi = 30 deg go through the equation above and give P = 2133.333333 W, P_max = 3840 W, i0 = -7.2 A, i_phi = 5.866666667 A. It applies only when square-wave bridges, single phase shift 0 < phi <= 90 degrees: outside that it says what is wrong instead of returning a number.

<a id="pe_llc_resonant_gain"></a>

#### LLC resonant converter: tank gain (first harmonic) — `pe_llc_resonant_gain`

The two resonant frequencies, the reflected AC load resistance, quality factor, inductance ratio and voltage gain of an LLC resonant tank at a switching frequency (first-harmonic approximation, full-wave rectified output).

$$
M = \left|\frac{j\omega L_m\parallel R_{ac}}{j\omega L_r + \frac{1}{j\omega C_r} + j\omega L_m\parallel R_{ac}}\right|,\quad R_{ac} = \frac{8n^2R}{\pi^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lr` | in | series resonant inductance | H |
| `Cr` | in | resonant capacitance | F |
| `Lm` | in | magnetizing inductance | H |
| `n` | in | transformer turns ratio Np/Ns | — |
| `R` | in | dc load resistance | ohm |
| `f` | in | switching frequency | Hz |
| `fr1` | out | series resonant frequency | Hz |
| `fr2` | out | lower resonant frequency | Hz |
| `Rac` | out | reflected AC load resistance | ohm |
| `Q` | out | quality factor sqrt(Lr/Cr)/Rac | — |
| `k` | out | inductance ratio Lm/Lr | — |
| `M` | out | tank voltage gain | — |

**Applies when:** first-harmonic approximation.

```shell
$ power-electronics solve pe_llc_resonant_gain "Lr=60 uH" "Cr=27 nF" "Lm=300 uH" n=8 "R=2 ohm" "f=100 kHz"
fr1 = 125043.9328 Hz
fr2 = 51048.9718 Hz
Rac = 103.752892 ohm
Q = 0.454353138
k = 5
M = 1.0981695
  by: LLC resonant converter: tank gain (first harmonic)  (Rac = 8 n^2 R/pi^2; M = |Zp/(Zs + Zp)|, Zp = jwLm || Rac, Zs = jwLr + 1/(jwCr))
  Lr = 6e-05 H
  Cr = 2.7e-08 F
  Lm = 0.0003 H
  n = 8
  R = 2 ohm
  f = 100000 Hz
  assumes: first-harmonic approximation
```

In the worked example, Lr = 60 uH, Cr = 27 nF, Lm = 300 uH, n = 8, R = 2 ohm, f = 100 kHz go through the equation above and give fr1 = 125043.9328 Hz, fr2 = 51048.9718 Hz, Rac = 103.752892 ohm, Q = 0.454353138. It applies only when first-harmonic approximation: outside that it says what is wrong instead of returning a number.

<a id="pe_series_resonant_gain"></a>

#### Series resonant converter: tank gain (first harmonic) — `pe_series_resonant_gain`

The resonant frequency, characteristic impedance, quality factor, normalised frequency and voltage gain of a series resonant tank driving an equivalent AC load resistance (first-harmonic approximation).

$$
M = \frac{1}{\sqrt{1+Q^2\left(f_n-\frac{1}{f_n}\right)^2}},\quad Q = \frac{\sqrt{L_r/C_r}}{R}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Lr` | in | resonant inductance | H |
| `Cr` | in | resonant capacitance | F |
| `R` | in | equivalent AC load resistance | ohm |
| `f` | in | switching frequency | Hz |
| `fr` | out | resonant frequency | Hz |
| `Z0` | out | characteristic impedance sqrt(Lr/Cr) | ohm |
| `Q` | out | quality factor | — |
| `fn` | out | normalised frequency f/fr | — |
| `M` | out | tank voltage gain | — |

**Applies when:** first-harmonic approximation.

```shell
$ power-electronics solve pe_series_resonant_gain "Lr=100 uH" "Cr=25 nF" "R=30 ohm" "f=120 kHz"
fr = 100658.4242 Hz
Z0 = 63.2455532 ohm
Q = 2.108185107
fn = 1.192150592
M = 0.8019643432
  by: Series resonant converter: tank gain (first harmonic)  (fr = 1/(2 pi sqrt(Lr Cr)); Q = sqrt(Lr/Cr)/R; M = 1/sqrt(1 + Q^2 (fn - 1/fn)^2))
  Lr = 0.0001 H
  Cr = 2.5e-08 F
  R = 30 ohm
  f = 120000 Hz
  assumes: first-harmonic approximation
```

In the worked example, Lr = 100 uH, Cr = 25 nF, R = 30 ohm, f = 120 kHz go through the equation above and give fr = 100658.4242 Hz, Z0 = 63.2455532 ohm, Q = 2.108185107, fn = 1.192150592. It applies only when first-harmonic approximation: outside that it says what is wrong instead of returning a number.

### Thermal (3)

<a id="pe_foster_periodic_pulses"></a>

#### Junction temperature under periodic power pulses — `pe_foster_periodic_pulses`

The peak, minimum and average junction temperature rise and the temperature ripple of a device described by a Foster network under a periodic rectangular power pulse train (exact periodic steady state, no single-pulse approximation).

$$
\Delta T_{pk} = \sum_iPR_i\frac{1-e^{-t_p/\tau_i}}{1-e^{-T/\tau_i}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | Foster thermal resistances in kelvin per watt | — |
| `tau` | in | Foster time constants in seconds | — |
| `P` | in | pulse power | W |
| `tp` | in | pulse duration | s |
| `T` | in | pulse period | s |
| `dT_peak` | out | peak temperature rise (end of each pulse) | K |
| `dT_min` | out | minimum temperature rise (start of each pulse) | K |
| `dT_avg` | out | average rise P Rth tp/T | K |
| `ripple` | out | peak minus minimum | K |

**Applies when:** periodic steady state; tp < T.

```shell
$ power-electronics solve pe_foster_periodic_pulses "R=[0.05, 0.1, 0.15]" "tau=[0.0001, 0.003, 0.05]" "P=500 W" "tp=2 ms" "T=10 ms"
dT_peak = 66.45250612 K
dT_min = 15.57762553 K
dT_avg = 30 K
ripple = 50.87488059 K
  by: Junction temperature under periodic power pulses  (per element: peak = P R (1 - e^(-tp/tau))/(1 - e^(-T/tau)); minimum = peak e^(-(T - tp)/tau); summed over the network)
  P = 500 W
  tp = 0.002 s
  T = 0.01 s
  assumes: periodic steady state; tp < T
```

In the worked example, R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 2 ms, T = 10 ms go through the equation above and give dT_peak = 66.45250612 K, dT_min = 15.57762553 K, dT_avg = 30 K, ripple = 50.87488059 K. It applies only when periodic steady state; tp < T: outside that it says what is wrong instead of returning a number.

<a id="pe_thermal_shared_heatsink"></a>

#### Several power devices on one heat sink — `pe_thermal_shared_heatsink`

The heat-sink temperature and every device's junction temperature when several devices, each with its own junction-to-case and case-to-sink resistance, share one heat sink to ambient.

$$
T_s = T_a + R_{sa}\sum_kP_k,\quad T_{j,k} = T_s + P_k(R_{jc}+R_{cs})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | loss of each device in watts | — |
| `Rjc` | in | junction-to-case thermal resistance of each device | K/W |
| `Rcs` | in | case-to-sink (interface) resistance of each device | K/W |
| `Rsa` | in | sink-to-ambient resistance | K/W |
| `Ta` | in | ambient temperature | K |
| `P_total` | out | total loss | W |
| `Ts` | out | heat-sink temperature | K |
| `Tj` | out | junction temperature of each device, kelvin | — |
| `Tj_max` | out | hottest junction | K |
| `Tj_max_C` | out | hottest junction in degC | — |

**Applies when:** steady state; the sink is isothermal.

```shell
$ power-electronics solve pe_thermal_shared_heatsink "P=[20, 20, 15]" "Rjc=0.8 K/W" "Rcs=0.2 K/W" "Rsa=0.5 K/W" "Ta=40 degC"
P_total = 55 W
Ts = 340.65 K
Tj = [360.65,360.65,355.65]
Tj_max = 360.65 K
Tj_max_C = 87.5
  by: Several power devices on one heat sink  (Ts = Ta + Rsa sum(P); Tj,k = Ts + P_k (Rjc + Rcs))
  Rjc = 0.8 K/W
  Rcs = 0.2 K/W
  Rsa = 0.5 K/W
  Ta = 313.15 K
  assumes: steady state; the sink is isothermal
```

In the worked example, P = [20, 20, 15], Rjc = 0.8 K/W, Rcs = 0.2 K/W, Rsa = 0.5 K/W, Ta = 40 degC go through the equation above and give P_total = 55 W, Ts = 340.65 K, Tj = [360.65, 360.65, 355.65], Tj_max = 360.65 K. It applies only when steady state; the sink is isothermal: outside that it says what is wrong instead of returning a number.

<a id="pe_foster_transient_thermal"></a>

#### Transient thermal impedance of a Foster network — `pe_foster_transient_thermal`

The transient thermal impedance Zth(tp) of a device described by a Foster RC network (datasheet R_i, tau_i pairs) and the junction temperature rise for a single power pulse of duration tp, with the steady-state values.

$$
Z_{th}(t) = \sum_i R_i\left(1-e^{-t/\tau_i}\right),\quad \Delta T_j = P\,Z_{th}(t_p)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | Foster thermal resistances in kelvin per watt | — |
| `tau` | in | Foster time constants in seconds | — |
| `P` | in | pulse power | W |
| `tp` | in | pulse duration | s |
| `Zth` | out | transient thermal impedance at tp | K/W |
| `dTj` | out | junction temperature rise at the end of the pulse | K |
| `Rth_total` | out | steady-state thermal resistance | K/W |
| `dTj_steady` | out | steady-state rise for continuous power | K |

**Applies when:** linear network, starting from thermal equilibrium.

```shell
$ power-electronics solve pe_foster_transient_thermal "R=[0.05, 0.1, 0.15]" "tau=[0.0001, 0.003, 0.05]" "P=500 W" "tp=10 ms"
Zth = 0.1736229877 K/W
dTj = 86.81149385 K
Rth_total = 0.3 K/W
dTj_steady = 150 K
  by: Transient thermal impedance of a Foster network  (Zth(t) = sum R_i (1 - exp(-t/tau_i)); dTj = P Zth(tp))
  P = 500 W
  tp = 0.01 s
  assumes: linear network, starting from thermal equilibrium
```

In the worked example, R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 10 ms go through the equation above and give Zth = 0.1736229877 K/W, dTj = 86.81149385 K, Rth_total = 0.3 K/W, dTj_steady = 150 K. It applies only when linear network, starting from thermal equilibrium: outside that it says what is wrong instead of returning a number.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
