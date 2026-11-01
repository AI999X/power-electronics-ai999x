# power-electronics-ai999x — offline, verified power electronics solver with step-by-step answers

[![tests](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/power-electronics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/power-electronics-ai999x)](https://pypi.org/project/power-electronics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/power-electronics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/power-electronics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/power-electronics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/power-electronics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/power-electronics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

power-electronics-ai999x is an offline, deterministic Python solver that answers power electronics questions with verified results from 79 certified capabilities, gives 482 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**power-electronics-ai999x** is a compact, deterministic power electronics engine for Python. It reads a question in plain English (or a typed call), picks a certified power electronics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified power electronics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 482 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** DC-DC converters, rectifiers, inverters and PWM, motor drives, device losses, thermal and magnetic design, resonant converters.

## Model overview

| Property | Value |
|---|---|
| Package | `power-electronics-ai999x` 1.0.0 (import `power_electronics_ai999x`) |
| Type | Deterministic neuro-symbolic power electronics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 79 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 482 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

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

The Python package also installs its own command: `power-electronics-ai999x "<question>"`, `power-electronics-ai999x --capabilities`. The outputs above were produced by running this exact code on power-electronics-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

79 certified procedures. A few of them:

- **Procedures:** Armature current ripple of a chopper-fed DC motor; Average and RMS of a rectangular pulse train; BLDC motor with six-step (120 degree) drive; Boost converter in continuous conduction; Boost converter in discontinuous conduction; Boost converter output voltage ripple; Boost converter with inductor resistance: gain and efficiency; Boost power-factor-correction stage; Bootstrap capacitor for a high-side gate driver; Buck converter efficiency with conduction and switching losses; Buck converter in continuous conduction; Buck converter in discontinuous conduction; Buck converter inductor and capacitor design; Buck converter output voltage ripple; Buck converter switch, diode, inductor and input-capacitor currents; Buck-boost converter in continuous conduction; Buck-boost converter in discontinuous conduction; Capacitor-input rectifier filter: exact ripple and peak diode current; Chopper-fed separately excited DC motor: speed and torque; Commutation overlap in a single-phase thyristor bridge; Commutation overlap in a three-phase thyristor bridge; Control-to-output transfer function of a DC-DC converter; Core loss by the Steinmetz equation; Core loss under triangular flux (iGSE) …

`power_electronics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified power electronics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (DC-DC converters, rectifiers, inverters and PWM, motor drives, device losses, thermal and magnetic design, resonant converters); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{power_electronics_ai999x_1_0_0,
  title  = {power-electronics-ai999x: an offline, verified power electronics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/power-electronics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/power-electronics-ai999x/discussions or visit https://ai999x.com.

## Support the project

power-electronics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/power-electronics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/power-electronics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/power-electronics-ai999x/tree/main/onnx).
