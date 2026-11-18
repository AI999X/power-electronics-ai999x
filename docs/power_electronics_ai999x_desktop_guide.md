# Power Electronics: the desktop app, user guide

**Power Electronics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Power Electronics covers DC-DC converters in continuous and discontinuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, bridge), rectifiers and phase control, inverters and PWM, motor drives, device losses, snubbers, thermal and magnetic design and resonant converters. Its desktop app holds all 79 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-power-electronics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-power-electronics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-power-electronics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-power-electronics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-power-electronics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-power-electronics-1.6.5-linux-x86_64.tar.gz
./ai999x-power-electronics-1.6.5-linux-x86_64/ai999x-power-electronics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Power Electronics app, opened on Buck converter in continuous conduction with its worked example.](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/hero.webp)

*The Power Electronics app, opened on Buck converter in continuous conduction with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open DC-DC converters › Buck converter in continuous conduction (or type part of its name in the search box). It opens with a worked example filled in: “Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A” The answer: D = 0.2083333333, dIL = 0.8996212121 A, IL_max = 3.449810606 A, IL_min = 2.550189394 A.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Buck converter in continuous conduction: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-1.webp)

*Buck converter in continuous conduction: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Buck converter in continuous conduction
D = 0.2083333333
dIL = 0.8996212121 A
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Three-phase fully controlled thyristor bridge](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-2.webp)

*Three-phase fully controlled thyristor bridge*

![Space-vector modulation dwell times](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-3.webp)

*Space-vector modulation dwell times*

![Flyback converter in continuous conduction](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-4.webp)

*Flyback converter in continuous conduction*

![Core loss by the Steinmetz equation](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-5.webp)

*Core loss by the Steinmetz equation*

![Control-to-output transfer function of a DC-DC converter](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-6.webp)

*Control-to-output transfer function of a DC-DC converter*

![Bootstrap capacitor for a high-side gate driver](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-7.webp)

*Bootstrap capacitor for a high-side gate driver*

![Boost power-factor-correction stage](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/scene-8.webp)

*Boost power-factor-correction stage*

![Buck converter in continuous conduction: a chart, enlarged.](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-1.webp)

*Buck converter in continuous conduction: a chart, enlarged.*

![Three-phase fully controlled thyristor bridge: a chart, enlarged.](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/chart-2.webp)

*Three-phase fully controlled thyristor bridge: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Power Electronics app in the dark theme.](https://www.ai999x.com/?ax_shot=power-electronics-ai999x/hero-dark.webp)

*The Power Electronics app in the dark theme.*

## 7. Every calculation

79 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/power-electronics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Control | Control-to-output transfer function of a DC-DC converter, Peak current mode: subharmonic stability and slope compensation |
| DC-DC converters | Boost converter in continuous conduction, Boost converter in discontinuous conduction, Boost converter output voltage ripple, Buck converter in continuous conduction, Buck converter in discontinuous conduction, Buck converter inductor and capacitor design, Buck converter output voltage ripple, Buck converter switch, diode, inductor and input-capacitor currents, Buck-boost converter in continuous conduction, Buck-boost converter in discontinuous conduction, Critical inductance at the CCM/DCM boundary, Cuk converter in continuous conduction, Ideal DC-DC conversion ratio from the duty cycle, Interleaved multiphase buck: output current ripple cancellation, SEPIC converter in continuous conduction |
| Devices | Bootstrap capacitor for a high-side gate driver, Current sharing between paralleled MOSFETs, Diode reverse-recovery charge and losses, Gate-drive power and currents, Hard-switching loss of a power transistor, MOSFET conduction loss at junction temperature, RC snubber from measured ringing frequencies, RCD turn-off snubber design, Zero-voltage switching: resonant transition of a switch node |
| Front ends | Boost power-factor-correction stage, Hold-up (bulk) capacitor |
| Inverters | Inverter dead-time voltage error, Quasi-square-wave (phase-shifted) full-bridge inverter, Sinusoidal PWM: fundamental output voltage, Space-vector modulation dwell times, Square-wave inverter output and harmonics, Square-wave inverter with an RL load: current, Three-phase six-step (square-wave) inverter |
| Isolated converters | Flyback converter in continuous conduction, Flyback magnetizing inductance for discontinuous conduction, Forward converter with reset winding, Push-pull, half-bridge and full-bridge converters |
| Losses | Boost converter with inductor resistance: gain and efficiency, Buck converter efficiency with conduction and switching losses, IGBT and diode conduction losses in a sinusoidal PWM inverter |
| Magnetics | Core loss by the Steinmetz equation, Core loss under triangular flux (iGSE), Gapped-core inductor: turns and air gap, Inductor core area product, Skin effect in a round wire, Transformer core area product, Transformer turns from Faraday's law, Winding AC resistance factor (Dowell) |
| Motor drives | Armature current ripple of a chopper-fed DC motor, BLDC motor with six-step (120 degree) drive, Chopper-fed separately excited DC motor: speed and torque, Induction motor constant V/f speed control, Maximum torque per ampere (MTPA) of an interior PMSM, Phase-controlled rectifier DC motor drive, PMSM steady state in the dq frame: voltages and torque, Regenerative braking of a DC motor with a chopper, Surface PMSM field weakening: base speed, maximum speed and torque |
| Power quality | Average and RMS of a rectangular pulse train, Power factor with harmonic distortion, Total harmonic distortion from harmonic values |
| Rectifiers | Capacitor-input rectifier filter: exact ripple and peak diode current, Commutation overlap in a single-phase thyristor bridge, Commutation overlap in a three-phase thyristor bridge, Half-wave diode rectifier with an RL load, Rectifier line current: THD, displacement and power factor, Rectifier line-current harmonic spectrum, Single-phase AC voltage controller (resistive load), Single-phase phase-controlled thyristor rectifier, Three-phase diode bridge rectifier with resistive load, Three-phase fully controlled thyristor bridge, Three-phase half-wave (three-pulse) diode rectifier |
| Resonant converters | Dual active bridge: power transfer and inductor current, LLC resonant converter: tank gain (first harmonic), Series resonant converter: tank gain (first harmonic) |
| Thermal | Junction temperature under periodic power pulses, Several power devices on one heat sink, Transient thermal impedance of a Foster network |

## 8. Other ways to use it

The same engine runs on the command line (`power-electronics`, its own guide), in Python (`pip install power-electronics-ai999x`, `import power_electronics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/power-electronics-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/power-electronics-ai999x/
