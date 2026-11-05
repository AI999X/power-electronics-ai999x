"""Power electronics procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded
by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: DC-DC converters in continuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback, forward, push-pull,
half- and full-bridge: duty cycle, inductor and capacitor ripple, component currents, critical inductance, filter
design, interleaving) and in discontinuous conduction (conversion ratio, second-interval duty, peak current), losses and
efficiency (non-ideal boost gain, a buck with conduction and switching losses); rectifiers (three-phase half-wave and
bridge, phase-controlled single- and three-phase, half-wave RL load, the exact capacitor-input filter, commutation
overlap, line-current harmonics, THD and power factor, AC voltage controllers); inverters and PWM (square-wave and
quasi-square-wave, six-step, sinusoidal PWM, space-vector duty times, dead-time error, RL load current, IGBT and diode
losses over a fundamental period); motor drives (chopper- and rectifier-fed DC motors, regenerative braking, armature
current ripple, BLDC six-step, induction motor V/f, PMSM dq torque, MTPA and field weakening); devices (MOSFET
conduction and switching loss, diode reverse recovery, gate drive, bootstrap capacitor, current sharing, snubbers,
zero-voltage switching); thermal design (several devices on one heat sink, Foster transient impedance, periodic pulses);
magnetics (transformer turns, gapped inductors, area product, Steinmetz and iGSE core loss, skin effect, Dowell
proximity factor); resonant and isolated converters (series resonant and LLC gain, dual active bridge power); control
(averaged small-signal model, slope compensation of peak current mode).

Electronics Engineering (model 36) holds the single-phase rectifier averages, the small-ripple capacitor filter, linear
regulators and heat-sink series paths; they are not repeated here.

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, cmath, re).

How the references differ from the shipped code:
- switched waveforms are SIMULATED: inductor and capacitor currents are built interval by interval from the switch
  states by integrating v/L and i/C with scipy quad, the steady state is found from the periodicity condition, and
  averages, ripples and RMS values are integrated numerically; RL and RC circuits are propagated with scipy's matrix
  exponential (expm) or integrated with scipy solve_ivp (DOP853) to their periodic steady state;
- duty cycles and steady-state operating points come from sympy solving the volt-second and charge-balance equations
  of each topology; discontinuous-mode ratios from scipy brentq on the simulated average output current;
- rectifier and inverter averages, RMS values and Fourier coefficients by scipy quad of the actual output or line
  waveform; sinusoidal PWM by computing every naturally sampled switching instant (brentq) and Fourier-analysing the
  pulse train; space-vector times by solving the volt-second balance of the two active vectors (scipy.linalg.solve);
- machines: PMSM voltages and torque from abc flux linkages differentiated by sympy (power balance), MTPA and field
  weakening by brentq on the torque derivative and the voltage limit; DC drives by scipy fsolve on the steady state of
  their dynamic equations;
- magnetics: core loss through the iGSE integral (scipy quad), round-wire skin effect by scipy's complex Bessel
  functions, Dowell's factor through its complex-hyperbolic form, gapped inductors by sympy solving the magnetic circuit;
- small-signal transfer functions by sympy linearisation of the averaged state equations; resonant gains by
  scipy.signal.freqs; peak current mode stability by simulating the perturbed current map.
- weaker references (the same published formula evaluated a different way) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- SI internally; companion outputs give customary units (degrees, percent, rpm, degC, cm^4).
- Ideal switches and diodes unless losses are inputs; converters in steady state with constant output voltage for the
  inductor waveforms (small ripple), so ripples follow from volt-seconds and charge from the ripple current.
- A continuous-conduction capability refuses an operating point whose inductor current reaches zero (it is
  discontinuous); a discontinuous-mode capability refuses a continuous one. Vout of inverting topologies is a magnitude.
- K = 2L/(R Ts) for discontinuous-mode ratios.
- Turns ratio n = Np/Ns (primary to secondary) for every isolated topology.
- Rectifier voltages: Vs is an RMS value (line-to-line for three-phase circuits, phase for the three-phase half-wave);
  firing and overlap angles in degrees from the natural commutation point; output current Id constant (highly inductive
  load) where stated.
- Harmonics are RMS values; THD = sqrt(sum of harmonic squares) / fundamental.
- Square-wave, quasi-square and six-step inverters: harmonic n has amplitude (4 Vdc / n pi) cos(n alpha) for a full
  bridge with zero intervals alpha at each end of a half-cycle; sinusoidal PWM is naturally sampled, bipolar, with a
  modulation index ma <= 1 and an odd frequency ratio mf >= 15 so that no sideband falls on the fundamental.
- Park transform amplitude-invariant (torque 1.5 p ...); speeds in rad/s with rpm companions.
- Temperatures: degC inputs are converted to K; Rds(T) = Rds,25 (1 + alpha (T - 25 degC)), linear.
- Steinmetz parameters k, alpha, beta in SI (f in Hz, B in T, loss density in W/m^3).
- Lists carry no unit: their unit is stated in the input (watts, kelvin per watt, seconds, milliohms, volts).
- Results built from exp, log or roots are rounded to 12 significant digits and exact zeros are written as 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A`). Physical values carry a unit;
fractions may be written with %; lists in brackets; choices as words. A thermal resistance may be written in degC/W
(read as K/W). Any other number, an unknown name or a name given twice is refused, never guessed.
"""
from __future__ import annotations

import cmath
import math
import re

# ================================================================================================ constants
MU0 = 1.25663706127e-6                # H/m (CODATA 2022)
T0C = 273.15                          # K
TWO_PI = 2.0 * math.pi
SQ2 = math.sqrt(2.0)
SQ3 = math.sqrt(3.0)


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(*vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _frac(name, x):
    _need(isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x) and 0 < x < 1,
          f"{name} must lie strictly between 0 and 1")


def _snap(x, scale):
    """A value that is zero by physics written as exactly 0 (rounding residue below 1e-12 of its scale)."""
    return 0.0 if abs(x) <= 1e-12 * abs(scale) else x


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


# ---------------------------------------------------------------------------------------------- reference helpers
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _quad(f, a, b, points=None):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400, points=points)
    return float(v)


def _pwl(segs, L, avg):
    """Reference helper: simulate an inductor current through switch intervals segs = [(duration, voltage across L)]
    by integrating v/L (scipy quad), then fix the starting current so that the period average equals avg. Returns the
    currents at every interval boundary (first = start of period) and the period."""
    rel = [0.0]
    for dt, v in segs:
        rel.append(rel[-1] + _quad(lambda t, v=v: v / L, 0.0, dt))
    T = sum(dt for dt, _ in segs)
    area = 0.0
    for k, (dt, v) in enumerate(segs):
        area += _quad(lambda t, k=k, v=v: rel[k] + v * t / L, 0.0, dt)
    i0 = avg - area / T
    return [i0 + x for x in rel], T


def _pwl_rms(segs, L, avg):
    """Reference helper: the RMS value of the simulated inductor current (quad of the square, interval by interval)."""
    cur, T = _pwl(segs, L, avg)
    s = 0.0
    for k, (dt, v) in enumerate(segs):
        s += _quad(lambda t, k=k, v=v: (cur[k] + v * t / L) ** 2, 0.0, dt)
    return math.sqrt(s / T)


def _sym_solve(eqs, unknowns):
    """Reference helper: sympy solves the steady-state (volt-second, charge-balance) equations; one real solution."""
    import sympy as sp
    sol = sp.solve(eqs, unknowns, dict=True)
    _need(len(sol) >= 1, "no steady-state solution")
    return {str(k): float(v) for k, v in sol[0].items()}


def _sq(x):
    """Reference helper: a float as an exact sympy rational from its shortest decimal spelling (fast, unlike nsimplify)."""
    import sympy as sp
    return sp.Rational(repr(float(x)))


# ================================================================================================ units
_KU = {"temperature": "K", "voltage": "V", "current": "A", "resistance": "ohm", "capacitance": "F", "inductance": "H",
       "frequency": "Hz", "angular_frequency": "rad/s", "time": "s", "power": "W", "length": "m", "area": "m^2",
       "volume": "m^3", "energy": "J", "angle": "rad", "magnetic_field": "T", "magnetic_flux": "Wb", "charge": "C",
       "torque": "N*m", "resistivity": "ohm*m", "current_density": "A/m^2", "thermal_resistance": "K/W",
       "temperature_difference": "K", "apparent_power": "VA",
       "motor_constant": "V*s/rad", "area_product": "m^4", "core_loss_density": "W/m^3", "current_slope": "A/s",
       "number": ""}

# how a value of a kind is written in templates and questions (unit, factor from SI); SI when absent
_SHOW = {"inductance": ("uH", 1e6), "capacitance": ("uF", 1e6), "frequency": ("kHz", 1e-3), "angle": ("deg", 180.0 / math.pi),
         "temperature": ("degC", 1.0), "current_density": ("A/mm^2", 1e-6), "area_product": ("cm^4", 1e8),
         "area": ("cm^2", 1e4), "current_slope": ("A/us", 1e-6)}

_THERMAL = re.compile(r"^(?:°C|degC|deg C|C)\s*/\s*W$")


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime: a thermal resistance in degC/W is the same number in K/W."""
    u = unit.strip()
    if kind == "thermal_resistance" and _THERMAL.match(u):
        u = "K/W"
    return f"{num} {u}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _squash(text):
    """A choice word without case, spaces, hyphens or underscores ('Buck-boost' and 'buck boost' agree)."""
    s = ""
    for ch in text.lower():
        if ch not in " -_":
            s += ch
    return s


def _choice_value(text, choices):
    """The documented choice a written word names, compared without case, spaces, hyphens or underscores; a word
    that names no choice is passed on as written (the runtime then refuses it as an invalid choice)."""
    w = _squash(text.strip())
    for c in choices:
        if _squash(c) == w:
            return c
    return text.strip()


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard is built in: every stated number is bound to a named input (any other text or number refuses
    the whole question); the capability phrase must open the question and be followed by the colon; a physical value
    without its unit, or a unit on a pure number, is refused; values are passed with their units so the runtime
    converts each exactly once."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") == "choice":
                    if key in out:
                        return None
                    out[key] = _choice_value(mw.group(2), by[key]["choices"])
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "choice", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                out[key] = _written_unit(num, unit, kind)
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    return f"{x:.10g}"


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        unit, f = spec.get("show") or _SHOW.get(kind) or (_KU[kind], 1.0)
        if unit == "degC":
            return f"{_fmt(float(v) - T0C)} degC"
        return f"{_fmt(float(v) * f)} {unit}"
    return _fmt(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, show=None):
    _KU[kind]                                   # every physical kind has its SI unit here
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if show:
        d["show"] = show
    return d


def _numin(sym, meaning, required=True, pct=False):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=12, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": dict(outputs or {})}


_DEG = ("deg", 180.0 / math.pi)
_NS = ("ns", 1e9)
_US = ("us", 1e6)
_MS = ("ms", 1e3)
_MOHM = ("mohm", 1e3)
_MH = ("mH", 1e3)
_NF = ("nF", 1e9)
_PF = ("pF", 1e12)
_NC = ("nC", 1e9)
_MM = ("mm", 1e3)
_HZ = ("Hz", 1.0)
_MWB = ("mWb", 1e3)
_MT = ("mT", 1e3)


# ================================================================================================ DC-DC: CONTINUOUS CONDUCTION
_V = lambda s, m: _phys(s, "voltage", m)          # noqa: E731
_I = lambda s, m: _phys(s, "current", m)          # noqa: E731
_L = lambda s, m: _phys(s, "inductance", m)       # noqa: E731
_C = lambda s, m: _phys(s, "capacitance", m)      # noqa: E731
_F = lambda s, m: _phys(s, "frequency", m)        # noqa: E731
_R = lambda s, m: _phys(s, "resistance", m)       # noqa: E731


def _ccm_ok(imin, scale):
    _need(imin > 1e-12 * scale, "the inductor current would reach zero: the converter is in discontinuous conduction "
          "(use the discontinuous-mode capability)")


def buck_ccm(Vin, Vout, L, fs, Io):
    _pos("Vin, Vout, L, fs and Io", Vin, Vout, L, fs, Io)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    D = Vout / Vin
    dI = (Vin - Vout) * D / (L * fs)
    _ccm_ok(Io - dI / 2.0, Io)
    return _out(D=D, dIL=dI, IL_max=Io + dI / 2.0, IL_min=Io - dI / 2.0, ripple_ratio=dI / Io, Iin=D * Io)


def _ref_buck_ccm(Vin, Vout, L, fs, Io):
    import sympy as sp
    d = sp.Symbol("D")
    T = 1.0 / fs
    D = _sym_solve([_sq(Vin - Vout) * d - _sq(Vout) * (1 - d)], [d])["D"]   # volt-seconds on L
    cur, _ = _pwl([(D * T, Vin - Vout), ((1 - D) * T, -Vout)], L, Io)                       # C carries no dc current
    return {"D": D, "dIL": max(cur) - min(cur), "IL_max": max(cur), "IL_min": min(cur), "ripple_ratio": (max(cur) - min(cur)) / Io,
            "Iin": _quad(lambda t: cur[0] + (Vin - Vout) * t / L, 0.0, D * T) / T}


def _rand_buck(rng):
    Vin = _sig(rng.uniform(8.0, 400.0), 3)
    Vout = _sig(Vin * rng.uniform(0.1, 0.9), 3)
    fs = _sig(_lu(rng, 2e4, 1e6), 3)
    Io = _sig(_lu(rng, 0.5, 40.0), 3)
    r = rng.uniform(0.1, 1.5)
    L = _sig((Vin - Vout) * (Vout / Vin) / (r * Io * fs), 3)
    return {"Vin": Vin, "Vout": Vout, "L": L, "fs": fs, "Io": Io}


_add("pe_buck_ccm",
     name="Buck converter in continuous conduction", topic="DC-DC converters / buck",
     purpose="The duty cycle, inductor ripple, peak and valley inductor currents, ripple ratio and average input current of an ideal buck (step-down) converter in continuous conduction.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L", "inductance"), _F("fs", "switching frequency"),
             _I("Io", "load (output) current")],
     outputs=[_o("D", "number", "duty cycle"), _o("dIL", "current", "peak-to-peak inductor ripple"), _o("IL_max", "current", "peak inductor current"),
              _o("IL_min", "current", "valley inductor current"), _o("ripple_ratio", "number", "ripple ratio dIL/Io"),
              _o("Iin", "current", "average input current")],
     primary="dIL", conditions=["0 < Vout < Vin", "continuous conduction (IL_min > 0)"],
     method="D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); IL = Io; IL_max,min = Io +- dIL/2; Iin = D Io",
     latex=r"D = \frac{V_o}{V_{in}},\quad \Delta i_L = \frac{(V_{in}-V_o)D}{L f_s},\quad I_{L,\max/\min} = I_o \pm \tfrac{\Delta i_L}{2}",
     run=buck_ccm, reference=_ref_buck_ccm,
     wrong=lambda Vin, Vout, L, fs, Io: (lambda D, dI: _out(D=D, dIL=dI, IL_max=Io + dI / 2, IL_min=Io - dI / 2, ripple_ratio=dI / Io, Iin=D * Io))(Vout / Vin, Vin * Vout / Vin / (L * fs)),
     rand=_rand_buck,
     phrases=("Buck converter in CCM", "Buck converter continuous conduction", "Buck converter"),
     examples=[_ex({"Vin": 24.0, "Vout": 5.0, "L": 22e-6, "fs": 200e3, "Io": 3.0}),
               _ex({"Vin": 48.0, "Vout": 12.0, "L": 47e-6, "fs": 100e3, "Io": 5.0})],
     level="beginner")


def boost_ccm(Vin, Vout, L, fs, Io):
    _pos("Vin, Vout, L, fs and Io", Vin, Vout, L, fs, Io)
    _need(Vout > Vin, "a boost converter needs Vout > Vin")
    D = 1.0 - Vin / Vout
    IL = Io * Vout / Vin
    dI = Vin * D / (L * fs)
    _ccm_ok(IL - dI / 2.0, IL)
    return _out(D=D, IL=IL, dIL=dI, IL_max=IL + dI / 2.0, IL_min=IL - dI / 2.0, V_switch=Vout)


def _ref_boost_ccm(Vin, Vout, L, fs, Io):
    import sympy as sp
    d, il = sp.symbols("D IL")
    T = 1.0 / fs
    s = _sym_solve([_sq(Vin) * d + _sq(Vin - Vout) * (1 - d),          # volt-seconds on L
                    il * (1 - d) - _sq(Io)], [d, il])                          # charge balance on C
    D, IL = s["D"], s["IL"]
    cur, _ = _pwl([(D * T, Vin), ((1 - D) * T, Vin - Vout)], L, IL)
    return {"D": D, "IL": IL, "dIL": max(cur) - min(cur), "IL_max": max(cur), "IL_min": min(cur), "V_switch": Vout}


def _rand_boost(rng):
    Vin = _sig(rng.uniform(3.0, 200.0), 3)
    Vout = _sig(Vin / rng.uniform(0.15, 0.9), 3)
    fs = _sig(_lu(rng, 2e4, 1e6), 3)
    Io = _sig(_lu(rng, 0.1, 20.0), 3)
    D = 1 - Vin / Vout
    r = rng.uniform(0.1, 1.5)
    L = _sig(Vin * D / (r * Io * Vout / Vin * fs), 3)
    return {"Vin": Vin, "Vout": Vout, "L": L, "fs": fs, "Io": Io}


_add("pe_boost_ccm",
     name="Boost converter in continuous conduction", topic="DC-DC converters / boost",
     purpose="The duty cycle, average inductor (input) current, inductor ripple, peak and valley currents and switch voltage stress of an ideal boost (step-up) converter in continuous conduction.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L", "inductance"), _F("fs", "switching frequency"),
             _I("Io", "load (output) current")],
     outputs=[_o("D", "number", "duty cycle"), _o("IL", "current", "average inductor current (input current)"),
              _o("dIL", "current", "peak-to-peak inductor ripple"), _o("IL_max", "current", "peak inductor current"),
              _o("IL_min", "current", "valley inductor current"), _o("V_switch", "voltage", "switch and diode voltage stress")],
     primary="dIL", conditions=["Vout > Vin > 0", "continuous conduction (IL_min > 0)"],
     method="D = 1 - Vin/Vout; IL = Io/(1 - D); dIL = Vin D/(L fs)",
     latex=r"D = 1-\frac{V_{in}}{V_o},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}",
     run=boost_ccm, reference=_ref_boost_ccm,
     wrong=lambda Vin, Vout, L, fs, Io: (lambda D: (lambda IL, dI: _out(D=D, IL=IL, dIL=dI, IL_max=IL + dI / 2, IL_min=IL - dI / 2, V_switch=Vout))(Io / (1 - D), Vin * D / (L * fs)))(Vin / Vout),
     rand=_rand_boost,
     phrases=("Boost converter in CCM", "Boost converter continuous conduction", "Boost converter"),
     examples=[_ex({"Vin": 12.0, "Vout": 48.0, "L": 100e-6, "fs": 100e3, "Io": 1.0}),
               _ex({"Vin": 5.0, "Vout": 12.0, "L": 10e-6, "fs": 500e3, "Io": 0.8})],
     level="beginner")


def buck_boost_ccm(Vin, Vout, L, fs, Io):
    _pos("Vin, Vout (magnitude), L, fs and Io", Vin, Vout, L, fs, Io)
    D = Vout / (Vin + Vout)
    IL = Io / (1.0 - D)
    dI = Vin * D / (L * fs)
    _ccm_ok(IL - dI / 2.0, IL)
    return _out(D=D, IL=IL, dIL=dI, IL_max=IL + dI / 2.0, IL_min=IL - dI / 2.0, V_switch=Vin + Vout)


def _ref_buck_boost_ccm(Vin, Vout, L, fs, Io):
    import sympy as sp
    d, il = sp.symbols("D IL")
    T = 1.0 / fs
    s = _sym_solve([_sq(Vin) * d - _sq(Vout) * (1 - d), il * (1 - d) - _sq(Io)], [d, il])
    D, IL = s["D"], s["IL"]
    cur, _ = _pwl([(D * T, Vin), ((1 - D) * T, -Vout)], L, IL)
    return {"D": D, "IL": IL, "dIL": max(cur) - min(cur), "IL_max": max(cur), "IL_min": min(cur), "V_switch": Vin + Vout}


def _rand_buck_boost(rng):
    Vin = _sig(rng.uniform(5.0, 200.0), 3)
    Vout = _sig(Vin * _lu(rng, 0.2, 5.0), 3)
    fs = _sig(_lu(rng, 2e4, 1e6), 3)
    Io = _sig(_lu(rng, 0.1, 20.0), 3)
    D = Vout / (Vin + Vout)
    r = rng.uniform(0.1, 1.5)
    L = _sig(Vin * D / (r * Io / (1 - D) * fs), 3)
    return {"Vin": Vin, "Vout": Vout, "L": L, "fs": fs, "Io": Io}


_add("pe_buck_boost_ccm",
     name="Buck-boost converter in continuous conduction", topic="DC-DC converters / buck-boost",
     purpose="The duty cycle, average inductor current, ripple, peak and valley currents and switch voltage stress of an ideal inverting buck-boost converter in continuous conduction (output voltage as a magnitude).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage magnitude (the output is inverted)"), _L("L", "inductance"),
             _F("fs", "switching frequency"), _I("Io", "load (output) current")],
     outputs=[_o("D", "number", "duty cycle"), _o("IL", "current", "average inductor current"),
              _o("dIL", "current", "peak-to-peak inductor ripple"), _o("IL_max", "current", "peak inductor current"),
              _o("IL_min", "current", "valley inductor current"), _o("V_switch", "voltage", "switch voltage stress Vin + Vout")],
     primary="dIL", conditions=["Vin, Vout > 0", "continuous conduction (IL_min > 0)"],
     method="D = Vout/(Vin + Vout); IL = Io/(1 - D); dIL = Vin D/(L fs)",
     latex=r"D = \frac{|V_o|}{V_{in}+|V_o|},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}",
     run=buck_boost_ccm, reference=_ref_buck_boost_ccm,
     wrong=lambda Vin, Vout, L, fs, Io: (lambda D: (lambda IL, dI: _out(D=D, IL=IL, dIL=dI, IL_max=IL + dI / 2, IL_min=IL - dI / 2, V_switch=Vin + Vout))(Io, Vin * D / (L * fs)))(Vout / (Vin + Vout)),
     rand=_rand_buck_boost,
     phrases=("Buck-boost converter in CCM", "Buck-boost converter continuous conduction", "Buck-boost converter"),
     examples=[_ex({"Vin": 12.0, "Vout": 15.0, "L": 47e-6, "fs": 100e3, "Io": 1.0}),
               _ex({"Vin": 24.0, "Vout": 12.0, "L": 33e-6, "fs": 250e3, "Io": 2.0})],
     level="beginner")


_TOPO5 = ("buck", "boost", "buck-boost", "Cuk", "SEPIC")


def conversion_ratio(topology, Vin, D):
    _pos("Vin", Vin)
    _frac("the duty cycle D", D)
    _need(topology in _TOPO5, "topology: buck, boost, buck-boost, Cuk or SEPIC")
    if topology == "buck":
        M = D
    elif topology == "boost":
        M = 1.0 / (1.0 - D)
    else:
        M = D / (1.0 - D)
    pol = "inverted" if topology in ("buck-boost", "Cuk") else "same"
    return _out(M=M, Vout=M * Vin, polarity=pol)


def _ref_conversion_ratio(topology, Vin, D):
    import sympy as sp
    vo = sp.Symbol("Vo")
    d, vi = _sq(D), _sq(Vin)
    # volt-second balance of the (first) inductor over one period, ON for D, OFF for 1 - D; Vo as a magnitude
    eq = {"buck": (vi - vo) * d - vo * (1 - d),
          "boost": vi * d + (vi - vo) * (1 - d),
          "buck-boost": vi * d - vo * (1 - d),
          "Cuk": vi * d + (vi - (vi + vo)) * (1 - d),                 # C1 holds Vin + Vo
          "SEPIC": vi * d + (vi - vi - vo) * (1 - d)}[topology]       # C1 holds Vin
    Vo = _sym_solve([eq], [vo])["Vo"]
    return {"M": Vo / Vin, "Vout": Vo, "polarity": "inverted" if topology in ("buck-boost", "Cuk") else "same"}


_add("pe_conversion_ratio",
     name="Ideal DC-DC conversion ratio from the duty cycle", topic="DC-DC converters / conversion ratio",
     purpose="The ideal continuous-conduction voltage conversion ratio M = Vout/Vin and output voltage of a buck, boost, buck-boost, Cuk or SEPIC converter at a duty cycle, with the output polarity.",
     inputs=[_choice("topology", _TOPO5, "converter topology: buck, boost, buck-boost, Cuk or SEPIC"), _V("Vin", "input voltage"),
             _numin("D", "duty cycle (a fraction or %)", pct=True)],
     outputs=[_o("M", "number", "conversion ratio Vout/Vin (magnitude)"), _o("Vout", "voltage", "output voltage magnitude"),
              _o("polarity", "number", "output polarity relative to the input (same or inverted)")],
     primary="M", conditions=["0 < D < 1", "ideal, continuous conduction"],
     method="buck M = D; boost M = 1/(1 - D); buck-boost, Cuk, SEPIC M = D/(1 - D)",
     latex=r"M_{buck} = D,\quad M_{boost} = \frac{1}{1-D},\quad M_{buck\text{-}boost,\,Cuk,\,SEPIC} = \frac{D}{1-D}",
     run=conversion_ratio, reference=_ref_conversion_ratio,
     wrong=lambda topology, Vin, D: (lambda M: _out(M=M, Vout=M * Vin, polarity="inverted" if topology in ("buck-boost", "Cuk") else "same"))(
         {"buck": 1 - D, "boost": 1 / D, "buck-boost": (1 - D) / D, "Cuk": (1 - D) / D, "SEPIC": (1 - D) / D}[topology]),
     rand=lambda rng: {"topology": rng.choice(_TOPO5), "Vin": _sig(rng.uniform(3.0, 400.0), 3), "D": round(rng.uniform(0.05, 0.9), 3)},
     phrases=("DC-DC conversion ratio", "Converter conversion ratio", "Ideal conversion ratio"),
     examples=[_ex({"topology": "boost", "Vin": 12.0, "D": 0.6}), _ex({"topology": "Cuk", "Vin": 24.0, "D": 0.4})],
     level="beginner",
     extra_q=[{"q": "DC-DC conversion ratio: topology = Boost, Vin = 12 V, D = 60%", "like": 0}])


def buck_output_ripple(Vin, Vout, L, C, fs):
    _pos("Vin, Vout, L, C and fs", Vin, Vout, L, C, fs)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    D = Vout / Vin
    dI = (Vin - Vout) * D / (L * fs)
    dV = dI / (8.0 * C * fs)
    return _out(D=D, dIL=dI, dVo=dV, dVo_pct=100.0 * dV / Vout)


def _ref_buck_output_ripple(Vin, Vout, L, C, fs):
    T = 1.0 / fs
    D = Vout / Vin
    cur, _ = _pwl([(D * T, Vin - Vout), ((1 - D) * T, -Vout)], L, 0.0)     # the ripple current that flows in C
    # the charge delivered while the capacitor current is positive (it rises through zero mid-ON, falls mid-OFF)
    a, b = cur[0], cur[1]
    t_up = -a / ((Vin - Vout) / L)
    t_dn = b / (Vout / L)
    Q = _quad(lambda t: a + (Vin - Vout) * t / L, t_up, D * T) + _quad(lambda t: b - Vout * t / L, 0.0, t_dn)
    dV = Q / C
    return {"D": D, "dIL": b - a, "dVo": dV, "dVo_pct": 100.0 * dV / Vout}


_add("pe_buck_output_ripple",
     name="Buck converter output voltage ripple", topic="DC-DC converters / buck",
     purpose="The peak-to-peak output voltage ripple of a buck converter in continuous conduction, with the inductor ripple current flowing entirely into the output capacitor (no ESR).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L", "inductance"), _C("C", "output capacitance"),
             _F("fs", "switching frequency")],
     outputs=[_o("D", "number", "duty cycle"), _o("dIL", "current", "inductor ripple"), _o("dVo", "voltage", "peak-to-peak output ripple"),
              _o("dVo_pct", "number", "output ripple in percent of Vout")],
     primary="dVo", conditions=["0 < Vout < Vin", "continuous conduction; ripple current into C, ideal capacitor"],
     method="dIL = (Vin - Vout) D/(L fs); dVo = dIL/(8 C fs) = (1 - D) Vout/(8 L C fs^2)",
     latex=r"\Delta v_o = \frac{\Delta i_L}{8 C f_s} = \frac{(1-D)V_o}{8 L C f_s^2}",
     run=buck_output_ripple, reference=_ref_buck_output_ripple,
     wrong=lambda Vin, Vout, L, C, fs: (lambda D: (lambda dI: _out(D=D, dIL=dI, dVo=dI / (2 * C * fs), dVo_pct=100 * dI / (2 * C * fs) / Vout))((Vin - Vout) * D / (L * fs)))(Vout / Vin),
     rand=lambda rng: (lambda Vin, x, fs, L: {"Vin": Vin, "Vout": _sig(Vin * x, 3), "L": L, "C": _sig(_lu(rng, 1e-6, 1e-3), 3), "fs": fs})(
         _sig(rng.uniform(8.0, 400.0), 3), rng.uniform(0.1, 0.9), _sig(_lu(rng, 2e4, 1e6), 3), _sig(_lu(rng, 1e-6, 1e-3), 3)),
     phrases=("Buck converter output ripple", "Buck output voltage ripple"),
     examples=[_ex({"Vin": 24.0, "Vout": 5.0, "L": 22e-6, "C": 100e-6, "fs": 200e3}),
               _ex({"Vin": 12.0, "Vout": 3.3, "L": 4.7e-6, "C": 47e-6, "fs": 500e3})],
     level="intermediate")


def boost_output_ripple(Vin, Vout, Io, C, fs):
    _pos("Vin, Vout, Io, C and fs", Vin, Vout, Io, C, fs)
    _need(Vout > Vin, "a boost converter needs Vout > Vin")
    D = 1.0 - Vin / Vout
    dV = Io * D / (C * fs)
    return _out(D=D, dVo=dV, dVo_pct=100.0 * dV / Vout)


def _ref_boost_output_ripple(Vin, Vout, Io, C, fs):
    import sympy as sp
    d = sp.Symbol("D")
    D = _sym_solve([_sq(Vin) * d + _sq(Vin - Vout) * (1 - d)], [d])["D"]
    # while the switch is ON the diode is off and C alone feeds the load: the charge it loses is the ripple
    Q = _quad(lambda t: Io, 0.0, D / fs)
    return {"D": D, "dVo": Q / C, "dVo_pct": 100.0 * Q / C / Vout}


_add("pe_boost_output_ripple",
     name="Boost converter output voltage ripple", topic="DC-DC converters / boost",
     purpose="The peak-to-peak output voltage ripple of a boost converter: the capacitor alone supplies the load current during the switch on-time (ideal capacitor, no ESR).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _I("Io", "load current"), _C("C", "output capacitance"),
             _F("fs", "switching frequency")],
     outputs=[_o("D", "number", "duty cycle"), _o("dVo", "voltage", "peak-to-peak output ripple"), _o("dVo_pct", "number", "ripple in percent of Vout")],
     primary="dVo", conditions=["Vout > Vin > 0", "continuous conduction, ideal capacitor"],
     method="D = 1 - Vin/Vout; dVo = Io D/(C fs)",
     latex=r"\Delta v_o = \frac{I_o D}{C f_s}",
     run=boost_output_ripple, reference=_ref_boost_output_ripple,
     wrong=lambda Vin, Vout, Io, C, fs: (lambda D: _out(D=D, dVo=Io * (1 - D) / (C * fs), dVo_pct=100 * Io * (1 - D) / (C * fs) / Vout))(1 - Vin / Vout),
     rand=lambda rng: (lambda Vin: {"Vin": Vin, "Vout": _sig(Vin / rng.uniform(0.15, 0.85), 3), "Io": _sig(_lu(rng, 0.1, 20.0), 3),
                                    "C": _sig(_lu(rng, 1e-6, 1e-3), 3), "fs": _sig(_lu(rng, 2e4, 1e6), 3)})(_sig(rng.uniform(3.0, 200.0), 3)),
     phrases=("Boost converter output ripple", "Boost output voltage ripple"),
     examples=[_ex({"Vin": 12.0, "Vout": 48.0, "Io": 1.0, "C": 47e-6, "fs": 100e3}),
               _ex({"Vin": 5.0, "Vout": 12.0, "Io": 0.5, "C": 22e-6, "fs": 500e3})],
     level="intermediate")


def cuk_ccm(Vin, Vout, L1, L2, C1, fs, Io):
    _pos("Vin, Vout (magnitude), L1, L2, C1, fs and Io", Vin, Vout, L1, L2, C1, fs, Io)
    D = Vout / (Vin + Vout)
    T = 1.0 / fs
    IL1 = Io * Vout / Vin
    d1, d2 = Vin * D * T / L1, Vout * (1.0 - D) * T / L2
    _ccm_ok(IL1 - d1 / 2.0, IL1)
    _ccm_ok(Io - d2 / 2.0, Io)
    return _out(D=D, IL1=IL1, IL2=Io, VC1=Vin + Vout, dIL1=d1, dIL2=d2, dVC1=Io * D * T / C1)


def _ref_cuk_ccm(Vin, Vout, L1, L2, C1, fs, Io):
    import sympy as sp
    d, vc, i1 = sp.symbols("D VC1 IL1")
    vi, vo, io = _sq(Vin), _sq(Vout), _sq(Io)
    # ON: L1 sees Vin, L2 sees VC1 - Vo, C1 carries -IL2; OFF: L1 sees Vin - VC1, L2 sees -Vo, C1 carries IL1
    s = _sym_solve([vi * d + (vi - vc) * (1 - d), (vc - vo) * d - vo * (1 - d), i1 * (1 - d) - io * d], [d, vc, i1])
    D, VC1, IL1 = s["D"], s["VC1"], s["IL1"]
    T = 1.0 / fs
    c1, _ = _pwl([(D * T, Vin), ((1 - D) * T, Vin - VC1)], L1, IL1)
    c2, _ = _pwl([(D * T, VC1 - Vout), ((1 - D) * T, -Vout)], L2, Io)
    return {"D": D, "IL1": IL1, "IL2": Io, "VC1": VC1, "dIL1": max(c1) - min(c1), "dIL2": max(c2) - min(c2),
            "dVC1": _quad(lambda t: Io, 0.0, D * T) / C1}


def _rand_two_l(rng):
    Vin = _sig(rng.uniform(5.0, 100.0), 3)
    Vout = _sig(Vin * _lu(rng, 0.3, 3.0), 3)
    fs = _sig(_lu(rng, 5e4, 5e5), 3)
    Io = _sig(_lu(rng, 0.2, 10.0), 3)
    D = Vout / (Vin + Vout)
    L1 = _sig(Vin * D / (rng.uniform(0.1, 1.2) * Io * Vout / Vin * fs), 3)
    L2 = _sig(Vin * D / (rng.uniform(0.1, 1.2) * Io * fs), 3)
    return {"Vin": Vin, "Vout": Vout, "L1": L1, "L2": L2, "C1": _sig(_lu(rng, 1e-6, 1e-4), 3), "fs": fs, "Io": Io}


_TWO_L_IN = [_V("Vin", "input voltage"), _V("Vout", "output voltage magnitude"), _L("L1", "input inductance"), _L("L2", "output inductance"),
             _C("C1", "coupling (energy-transfer) capacitance"), _F("fs", "switching frequency"), _I("Io", "load current")]

_add("pe_cuk_ccm",
     name="Cuk converter in continuous conduction", topic="DC-DC converters / Cuk",
     purpose="The duty cycle, inductor currents and ripples, coupling-capacitor voltage and its ripple of an ideal Cuk converter (inverted output, given as a magnitude) in continuous conduction.",
     inputs=_TWO_L_IN,
     outputs=[_o("D", "number", "duty cycle"), _o("IL1", "current", "average input inductor current"), _o("IL2", "current", "average output inductor current"),
              _o("VC1", "voltage", "average coupling-capacitor voltage Vin + Vout"), _o("dIL1", "current", "input inductor ripple"),
              _o("dIL2", "current", "output inductor ripple"), _o("dVC1", "voltage", "coupling-capacitor ripple")],
     primary="dIL1", conditions=["continuous conduction in both inductors"],
     method="D = Vo/(Vin + Vo); IL1 = Io Vo/Vin; VC1 = Vin + Vo; dIL1 = Vin D/(L1 fs); dIL2 = Vo (1 - D)/(L2 fs); dVC1 = Io D/(C1 fs)",
     latex=r"D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in}+V_o,\ \Delta i_{L1} = \frac{V_{in}D}{L_1 f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}",
     run=cuk_ccm, reference=_ref_cuk_ccm,
     wrong=lambda Vin, Vout, L1, L2, C1, fs, Io: dict(cuk_ccm(Vin, Vout, L1, L2, C1, fs, Io), VC1=_r(Vin), dIL2=_r((Vin - Vout) * Vout / (Vin + Vout) / (L2 * fs))),
     rand=_rand_two_l,
     phrases=("Cuk converter", "Cuk converter in CCM"),
     examples=[_ex({"Vin": 12.0, "Vout": 15.0, "L1": 100e-6, "L2": 100e-6, "C1": 10e-6, "fs": 100e3, "Io": 1.0}),
               _ex({"Vin": 24.0, "Vout": 12.0, "L1": 220e-6, "L2": 150e-6, "C1": 22e-6, "fs": 50e3, "Io": 2.0})],
     level="intermediate")


def sepic_ccm(Vin, Vout, L1, L2, C1, fs, Io):
    _pos("Vin, Vout, L1, L2, C1, fs and Io", Vin, Vout, L1, L2, C1, fs, Io)
    D = Vout / (Vin + Vout)
    T = 1.0 / fs
    IL1 = Io * Vout / Vin
    d1, d2 = Vin * D * T / L1, Vin * D * T / L2
    _ccm_ok(IL1 - d1 / 2.0, IL1)
    _ccm_ok(Io - d2 / 2.0, Io)
    return _out(D=D, IL1=IL1, IL2=Io, VC1=Vin, dIL1=d1, dIL2=d2, dVC1=Io * D * T / C1, V_switch=Vin + Vout)


def _ref_sepic_ccm(Vin, Vout, L1, L2, C1, fs, Io):
    import sympy as sp
    d, vc, i1 = sp.symbols("D VC1 IL1")
    vi, vo, io = _sq(Vin), _sq(Vout), _sq(Io)
    # ON: L1 sees Vin, L2 sees VC1 (in reverse: the switch puts C1 across it), C1 carries -IL2;
    # OFF: L1 sees Vin - VC1 - Vo, L2 sees -Vo, C1 carries IL1
    s = _sym_solve([vi * d + (vi - vc - vo) * (1 - d), vc * d - vo * (1 - d), i1 * (1 - d) - io * d], [d, vc, i1])
    D, VC1, IL1 = s["D"], s["VC1"], s["IL1"]
    T = 1.0 / fs
    c1, _ = _pwl([(D * T, Vin), ((1 - D) * T, Vin - VC1 - Vout)], L1, IL1)
    c2, _ = _pwl([(D * T, VC1), ((1 - D) * T, -Vout)], L2, Io)
    return {"D": D, "IL1": IL1, "IL2": Io, "VC1": VC1, "dIL1": max(c1) - min(c1), "dIL2": max(c2) - min(c2),
            "dVC1": _quad(lambda t: Io, 0.0, D * T) / C1, "V_switch": VC1 + Vout}


_add("pe_sepic_ccm",
     name="SEPIC converter in continuous conduction", topic="DC-DC converters / SEPIC",
     purpose="The duty cycle, inductor currents and ripples, coupling-capacitor voltage and ripple and switch voltage stress of an ideal SEPIC (non-inverting buck-boost) converter in continuous conduction.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L1", "input inductance"), _L("L2", "second inductance"),
             _C("C1", "coupling capacitance"), _F("fs", "switching frequency"), _I("Io", "load current")],
     outputs=[_o("D", "number", "duty cycle"), _o("IL1", "current", "average input inductor current"), _o("IL2", "current", "average second inductor current"),
              _o("VC1", "voltage", "average coupling-capacitor voltage (= Vin)"), _o("dIL1", "current", "input inductor ripple"),
              _o("dIL2", "current", "second inductor ripple"), _o("dVC1", "voltage", "coupling-capacitor ripple"),
              _o("V_switch", "voltage", "switch voltage stress Vin + Vout")],
     primary="dIL1", conditions=["continuous conduction in both inductors"],
     method="D = Vo/(Vin + Vo); IL1 = Io Vo/Vin; IL2 = Io; VC1 = Vin; dIL1 = Vin D/(L1 fs); dIL2 = Vin D/(L2 fs); dVC1 = Io D/(C1 fs)",
     latex=r"D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in},\ \Delta i_{L1,2} = \frac{V_{in}D}{L_{1,2} f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}",
     run=sepic_ccm, reference=_ref_sepic_ccm,
     wrong=lambda Vin, Vout, L1, L2, C1, fs, Io: dict(sepic_ccm(Vin, Vout, L1, L2, C1, fs, Io), VC1=_r(Vin + Vout), dIL1=_r(Vin * Vout / (Vin + Vout) / (2 * L1 * fs))),
     rand=lambda rng: (lambda x: {"Vin": x["Vin"], "Vout": x["Vout"], "L1": x["L1"], "L2": x["L2"], "C1": x["C1"], "fs": x["fs"], "Io": x["Io"]})(_rand_two_l(rng)),
     phrases=("SEPIC converter", "SEPIC converter in CCM"),
     examples=[_ex({"Vin": 9.0, "Vout": 12.0, "L1": 47e-6, "L2": 47e-6, "C1": 10e-6, "fs": 200e3, "Io": 1.0}),
               _ex({"Vin": 24.0, "Vout": 12.0, "L1": 100e-6, "L2": 100e-6, "C1": 4.7e-6, "fs": 100e3, "Io": 1.5})],
     level="intermediate")


def flyback_ccm(Vin, Vout, n, Lm, fs, Io):
    _pos("Vin, Vout, n, Lm, fs and Io", Vin, Vout, n, Lm, fs, Io)
    D = n * Vout / (Vin + n * Vout)
    T = 1.0 / fs
    Im = Io / (n * (1.0 - D))
    dI = Vin * D * T / Lm
    _ccm_ok(Im - dI / 2.0, Im)
    return _out(D=D, Im=Im, dIm=dI, Ipk_primary=Im + dI / 2.0, Ipk_secondary=n * (Im + dI / 2.0),
                V_switch=Vin + n * Vout, V_diode=Vout + Vin / n)


def _ref_flyback_ccm(Vin, Vout, n, Lm, fs, Io):
    import sympy as sp
    d, im = sp.symbols("D Im")
    vi, vo, nn, io = _sq(Vin), _sq(Vout), _sq(n), _sq(Io)
    # magnetizing inductance on the primary: ON it sees Vin; OFF the reflected output -n Vo; the secondary carries
    # n Im during OFF, whose average is the load current
    s = _sym_solve([vi * d - nn * vo * (1 - d), nn * im * (1 - d) - io], [d, im])
    D, Im = s["D"], s["Im"]
    T = 1.0 / fs
    c, _ = _pwl([(D * T, Vin), ((1 - D) * T, -n * Vout)], Lm, Im)
    return {"D": D, "Im": Im, "dIm": max(c) - min(c), "Ipk_primary": max(c), "Ipk_secondary": n * max(c),
            "V_switch": Vin + n * Vout, "V_diode": Vout + Vin / n}


_add("pe_flyback_ccm",
     name="Flyback converter in continuous conduction", topic="isolated converters / flyback",
     purpose="The duty cycle, magnetizing current (primary-referred) and ripple, peak primary and secondary currents and the switch and diode voltage stresses of an ideal flyback converter in continuous conduction.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _numin("n", "turns ratio Np/Ns"), _L("Lm", "magnetizing inductance (primary)"),
             _F("fs", "switching frequency"), _I("Io", "load current")],
     outputs=[_o("D", "number", "duty cycle"), _o("Im", "current", "average magnetizing current referred to the primary"),
              _o("dIm", "current", "magnetizing current ripple"), _o("Ipk_primary", "current", "peak primary (switch) current"),
              _o("Ipk_secondary", "current", "peak secondary (diode) current"), _o("V_switch", "voltage", "switch voltage stress Vin + n Vo"),
              _o("V_diode", "voltage", "diode reverse voltage Vo + Vin/n")],
     primary="D", conditions=["continuous conduction of the magnetizing current", "no leakage inductance"],
     method="D = n Vo/(Vin + n Vo); Im = Io/(n (1 - D)); dIm = Vin D/(Lm fs)",
     latex=r"\frac{V_o}{V_{in}} = \frac{1}{n}\frac{D}{1-D},\quad I_m = \frac{I_o}{n(1-D)},\quad \Delta i_m = \frac{V_{in}D}{L_m f_s}",
     run=flyback_ccm, reference=_ref_flyback_ccm,
     wrong=lambda Vin, Vout, n, Lm, fs, Io: (lambda D: (lambda Im, dI: _out(D=D, Im=Im, dIm=dI, Ipk_primary=Im + dI / 2, Ipk_secondary=n * (Im + dI / 2), V_switch=Vin + n * Vout, V_diode=Vout + Vin / n))(Io / (n * (1 - D)), Vin * D / (Lm * fs)))(Vout / (Vin + Vout)),
     rand=lambda rng: (lambda Vin, Vout, n, fs, Io: (lambda D: {"Vin": Vin, "Vout": Vout, "n": n, "Lm": _sig(Vin * D / (rng.uniform(0.1, 1.2) * Io / (n * (1 - D)) * fs), 3), "fs": fs, "Io": Io})(n * Vout / (Vin + n * Vout)))(
         _sig(rng.uniform(12.0, 400.0), 3), _sig(rng.uniform(3.0, 48.0), 3), _sig(rng.uniform(0.5, 12.0), 3), _sig(_lu(rng, 5e4, 3e5), 3), _sig(_lu(rng, 0.2, 10.0), 3)),
     phrases=("Flyback converter", "Flyback converter in CCM"),
     examples=[_ex({"Vin": 48.0, "Vout": 12.0, "n": 2.0, "Lm": 200e-6, "fs": 100e3, "Io": 2.0}),
               _ex({"Vin": 325.0, "Vout": 5.0, "n": 20.0, "Lm": 6e-3, "fs": 65e3, "Io": 3.0})],
     level="intermediate")


def forward_ccm(Vin, Vout, n, Nr, L, fs, Io):
    _pos("Vin, Vout, n, Nr, L, fs and Io", Vin, Vout, n, Nr, L, fs, Io)
    D = n * Vout / Vin
    Dmax = 1.0 / (1.0 + Nr)
    _need(D < Dmax, "the duty cycle needed exceeds the maximum the reset winding allows (the core would not reset)")
    T = 1.0 / fs
    dI = (Vin / n - Vout) * D * T / L
    _ccm_ok(Io - dI / 2.0, Io)
    return _out(D=D, D_max=Dmax, V_switch=Vin * (1.0 + 1.0 / Nr), dIL=dI, IL_max=Io + dI / 2.0, IL_min=Io - dI / 2.0)


def _ref_forward_ccm(Vin, Vout, n, Nr, L, fs, Io):
    import sympy as sp
    d, dm = sp.symbols("D Dm")
    vi, vo, nn, nr = _sq(Vin), _sq(Vout), _sq(n), _sq(Nr)
    # output inductor volt-seconds; core reset: Vin D T on the primary equals (Vin Np/Nr) t_reset, D + t_reset/T = 1
    D = _sym_solve([(vi / nn - vo) * d - vo * (1 - d)], [d])["D"]
    Dmax = _sym_solve([vi * dm - (vi / nr) * (1 - dm)], [dm])["Dm"]
    T = 1.0 / fs
    c, _ = _pwl([(D * T, Vin / n - Vout), ((1 - D) * T, -Vout)], L, Io)
    return {"D": D, "D_max": Dmax, "V_switch": Vin + Vin / Nr, "dIL": max(c) - min(c), "IL_max": max(c), "IL_min": min(c)}


_add("pe_forward_ccm",
     name="Forward converter with reset winding", topic="isolated converters / forward",
     purpose="The duty cycle, the largest duty cycle the reset winding allows, the switch voltage stress and the output inductor ripple of a single-switch forward converter in continuous conduction.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _numin("n", "turns ratio Np/Ns"),
             _numin("Nr", "reset-winding to primary turns ratio Nr/Np"), _L("L", "output inductance"),
             _F("fs", "switching frequency"), _I("Io", "load current")],
     outputs=[_o("D", "number", "duty cycle"), _o("D_max", "number", "largest duty cycle that resets the core"),
              _o("V_switch", "voltage", "switch voltage stress"), _o("dIL", "current", "output inductor ripple"),
              _o("IL_max", "current", "peak output inductor current"), _o("IL_min", "current", "valley output inductor current")],
     primary="D", conditions=["D < D_max", "continuous conduction of the output inductor"],
     method="D = n Vo/Vin; D_max = 1/(1 + Nr/Np); Vsw = Vin (1 + Np/Nr); dIL = (Vin/n - Vo) D/(L fs)",
     latex=r"V_o = \frac{V_{in} D}{n},\quad D_{max} = \frac{1}{1+N_r/N_p},\quad V_{sw} = V_{in}\left(1+\frac{N_p}{N_r}\right)",
     run=forward_ccm, reference=_ref_forward_ccm,
     wrong=lambda Vin, Vout, n, Nr, L, fs, Io: (lambda D: (lambda dI: _out(D=D, D_max=1 / (1 + Nr), V_switch=Vin * (1 + 1 / Nr), dIL=dI, IL_max=Io + dI / 2, IL_min=Io - dI / 2))((Vin * n - Vout) * D / (L * fs)))(Vout / (n * Vin)),
     rand=lambda rng: (lambda Vin, n, Nr, fs, Io: (lambda Vout: (lambda D: {"Vin": Vin, "Vout": Vout, "n": n, "Nr": Nr, "L": _sig((Vin / n - Vout) * D / (rng.uniform(0.1, 1.2) * Io * fs), 3), "fs": fs, "Io": Io})(n * Vout / Vin))(
         _sig(Vin / n * rng.uniform(0.1, 0.95) / (1 + Nr), 3)))(_sig(rng.uniform(24.0, 400.0), 3), _sig(rng.uniform(0.5, 10.0), 3), rng.choice([0.5, 1.0, 1.0, 1.5, 2.0]),
                                                            _sig(_lu(rng, 5e4, 5e5), 3), _sig(_lu(rng, 0.5, 20.0), 3)),
     phrases=("Forward converter", "Forward converter in CCM"),
     examples=[_ex({"Vin": 48.0, "Vout": 5.0, "n": 3.0, "Nr": 1.0, "L": 22e-6, "fs": 200e3, "Io": 5.0}),
               _ex({"Vin": 300.0, "Vout": 12.0, "n": 8.0, "Nr": 1.0, "L": 100e-6, "fs": 100e3, "Io": 4.0})],
     level="intermediate")


_BRIDGE3 = ("push-pull", "half-bridge", "full-bridge")


def bridge_converter(topology, Vin, Vout, n, L, fs, Io):
    _pos("Vin, Vout, n, L, fs and Io", Vin, Vout, n, L, fs, Io)
    _need(topology in _BRIDGE3, "topology: push-pull, half-bridge or full-bridge")
    Vsec = Vin / n if topology != "half-bridge" else Vin / (2.0 * n)
    D = Vout / (2.0 * Vsec)                     # each switch (pair) is on for D T in every period, twice per period
    _need(D < 0.5, "the output voltage needs a duty cycle of 50 % or more per switch, which is not possible")
    dI = (Vsec - Vout) * D / (L * fs)
    _ccm_ok(Io - dI / 2.0, Io)
    vsw = 2.0 * Vin if topology == "push-pull" else Vin
    return _out(D=D, V_switch=vsw, f_ripple=2.0 * fs, dIL=dI, IL_min=Io - dI / 2.0)


def _ref_bridge_converter(topology, Vin, Vout, n, L, fs, Io):
    import sympy as sp
    d = sp.Symbol("D")
    vp = Vin if topology != "half-bridge" else Vin / 2.0          # the primary sees Vin (half-bridge: the capacitor midpoint)
    vs = vp / n
    T = 1.0 / fs
    # over HALF a period the rectified secondary is vs for D T, then the inductor freewheels
    D = _sym_solve([(_sq(vs) - _sq(Vout)) * d - _sq(Vout) * (sp.Rational(1, 2) - d)], [d])["D"]
    c, Th = _pwl([(D * T, vs - Vout), ((0.5 - D) * T, -Vout)], L, Io)
    return {"D": D, "V_switch": 2.0 * Vin if topology == "push-pull" else Vin, "f_ripple": 1.0 / Th, "dIL": max(c) - min(c), "IL_min": min(c)}


_add("pe_bridge_converter",
     name="Push-pull, half-bridge and full-bridge converters", topic="isolated converters / bridge",
     purpose="The duty cycle per switch, switch voltage stress, output ripple frequency and output inductor ripple of an ideal push-pull, half-bridge or full-bridge DC-DC converter with a centre-tapped (full-wave) rectifier, in continuous conduction.",
     inputs=[_choice("topology", _BRIDGE3, "push-pull, half-bridge or full-bridge"), _V("Vin", "input voltage"), _V("Vout", "output voltage"),
             _numin("n", "turns ratio Np/Ns (primary to each secondary half)"), _L("L", "output inductance"), _F("fs", "switching frequency"),
             _I("Io", "load current")],
     outputs=[_o("D", "number", "duty cycle of each switch"), _o("V_switch", "voltage", "switch voltage stress"),
              _o("f_ripple", "frequency", "output ripple frequency"), _o("dIL", "current", "output inductor ripple"),
              _o("IL_min", "current", "valley output inductor current")],
     primary="D", conditions=["D < 0.5", "continuous conduction of the output inductor"],
     method="Vo = 2 D Vin/n (push-pull, full-bridge), Vo = D Vin/n (half-bridge); dIL = (Vsec - Vo) D/(L fs)",
     latex=r"V_o = \frac{2 D V_{in}}{n}\ (\text{push-pull, full-bridge}),\quad V_o = \frac{D V_{in}}{n}\ (\text{half-bridge})",
     run=bridge_converter, reference=_ref_bridge_converter,
     wrong=lambda topology, Vin, Vout, n, L, fs, Io: (lambda r: dict(r, dIL=_r(r["dIL"] / 2), IL_min=_r(Io - r["dIL"] / 4)))(bridge_converter(topology, Vin, Vout, n, L, fs, Io)),
     rand=lambda rng: (lambda top, Vin, n, fs, Io: (lambda vs: (lambda Vout: {"topology": top, "Vin": Vin, "Vout": Vout, "n": n,
                                                                            "L": _sig((vs - Vout) * (Vout / (2 * vs)) / (rng.uniform(0.1, 1.0) * Io * fs), 3), "fs": fs, "Io": Io})(_sig(vs * rng.uniform(0.1, 0.9), 3)))(
         Vin / n if top != "half-bridge" else Vin / (2 * n)))(rng.choice(_BRIDGE3), _sig(rng.uniform(24.0, 400.0), 3), _sig(rng.uniform(0.5, 10.0), 3), _sig(_lu(rng, 5e4, 3e5), 3), _sig(_lu(rng, 1.0, 30.0), 3)),
     phrases=("Isolated bridge converter", "Push-pull or bridge converter"),
     examples=[_ex({"topology": "full-bridge", "Vin": 400.0, "Vout": 48.0, "n": 5.0, "L": 50e-6, "fs": 100e3, "Io": 10.0}),
               _ex({"topology": "push-pull", "Vin": 24.0, "Vout": 12.0, "n": 1.0, "L": 22e-6, "fs": 100e3, "Io": 3.0})],
     level="intermediate")


_TOPO3 = ("buck", "boost", "buck-boost")


def critical_inductance(topology, Vin, Vout, fs, R):
    _pos("Vin, Vout, fs and R", Vin, Vout, fs, R)
    _need(topology in _TOPO3, "topology: buck, boost or buck-boost")
    if topology == "buck":
        _need(Vout < Vin, "a buck converter needs Vout < Vin")
        D = Vout / Vin
        Lc = (1.0 - D) * R / (2.0 * fs)
    elif topology == "boost":
        _need(Vout > Vin, "a boost converter needs Vout > Vin")
        D = 1.0 - Vin / Vout
        Lc = D * (1.0 - D) ** 2 * R / (2.0 * fs)
    else:
        D = Vout / (Vin + Vout)
        Lc = (1.0 - D) ** 2 * R / (2.0 * fs)
    return _out(D=D, L_crit=Lc, Io=Vout / R)


def _ref_critical_inductance(topology, Vin, Vout, fs, R):
    T, Io = 1.0 / fs, Vout / R
    if topology == "buck":
        D = Vout / Vin
        segs, avg = [(D * T, Vin - Vout), ((1 - D) * T, -Vout)], Io
    elif topology == "boost":
        D = 1 - Vin / Vout
        segs, avg = [(D * T, Vin), ((1 - D) * T, Vin - Vout)], Io / (1 - D)
    else:
        D = Vout / (Vin + Vout)
        segs, avg = [(D * T, Vin), ((1 - D) * T, -Vout)], Io / (1 - D)
    # the inductance at which the simulated valley current just touches zero
    Lc = _brentq(lambda L: min(_pwl(segs, L, avg)[0]), 1e-12, 1.0)
    return {"D": D, "L_crit": Lc, "Io": Io}


_add("pe_critical_inductance",
     name="Critical inductance at the CCM/DCM boundary", topic="DC-DC converters / conduction boundary",
     purpose="The smallest inductance that keeps a buck, boost or buck-boost converter in continuous conduction at a load resistance (the inductor current just reaches zero at the end of each period).",
     inputs=[_choice("topology", _TOPO3, "buck, boost or buck-boost"), _V("Vin", "input voltage"), _V("Vout", "output voltage (magnitude)"),
             _F("fs", "switching frequency"), _R("R", "load resistance")],
     outputs=[_o("D", "number", "duty cycle"), _o("L_crit", "inductance", "critical (boundary) inductance"), _o("Io", "current", "load current")],
     primary="L_crit", conditions=["ideal converter at the boundary of continuous conduction"],
     method="buck Lc = (1 - D) R/(2 fs); boost Lc = D (1 - D)^2 R/(2 fs); buck-boost Lc = (1 - D)^2 R/(2 fs)",
     latex=r"L_{c,buck} = \frac{(1-D)R}{2f_s},\ L_{c,boost} = \frac{D(1-D)^2R}{2f_s},\ L_{c,buck\text{-}boost} = \frac{(1-D)^2R}{2f_s}",
     run=critical_inductance, reference=_ref_critical_inductance,
     wrong=lambda topology, Vin, Vout, fs, R: (lambda r: dict(r, L_crit=_r(r["L_crit"] * 2)))(critical_inductance(topology, Vin, Vout, fs, R)),
     rand=lambda rng: (lambda top, Vin: {"topology": top, "Vin": Vin, "Vout": _sig(Vin * (rng.uniform(0.1, 0.9) if top == "buck" else rng.uniform(1.2, 6.0) if top == "boost" else _lu(rng, 0.2, 5.0)), 3),
                                         "fs": _sig(_lu(rng, 2e4, 1e6), 3), "R": _sig(_lu(rng, 0.5, 500.0), 3)})(rng.choice(_TOPO3), _sig(rng.uniform(5.0, 300.0), 3)),
     phrases=("Critical inductance", "CCM DCM boundary inductance"),
     examples=[_ex({"topology": "buck", "Vin": 24.0, "Vout": 12.0, "fs": 100e3, "R": 10.0}),
               _ex({"topology": "boost", "Vin": 12.0, "Vout": 36.0, "fs": 50e3, "R": 50.0})],
     level="intermediate")


def buck_lc_design(Vin, Vout, Io, fs, r, dV):
    _pos("Vin, Vout, Io, fs and dV", Vin, Vout, Io, fs, dV)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    _need(0 < r < 2, "the ripple ratio r must lie between 0 and 2 (continuous conduction)")
    D = Vout / Vin
    dI = r * Io
    L = (Vin - Vout) * D / (dI * fs)
    C = dI / (8.0 * fs * dV)
    return _out(D=D, L=L, C=C, IL_max=Io + dI / 2.0, dIL=dI)


def _ref_buck_lc_design(Vin, Vout, Io, fs, r, dV):
    T = 1.0 / fs
    D = Vout / Vin
    segs = [(D * T, Vin - Vout), ((1 - D) * T, -Vout)]
    L = _brentq(lambda L: (lambda c: max(c) - min(c))(_pwl(segs, L, Io)[0]) - r * Io, 1e-12, 10.0)
    C = _brentq(lambda C: _ref_buck_output_ripple(Vin, Vout, L, C, fs)["dVo"] - dV, 1e-15, 10.0)
    c, _ = _pwl(segs, L, Io)
    return {"D": D, "L": L, "C": C, "IL_max": max(c), "dIL": max(c) - min(c)}


_add("pe_buck_lc_design",
     name="Buck converter inductor and capacitor design", topic="DC-DC converters / filter design",
     purpose="The inductance that gives a chosen ripple ratio and the output capacitance that limits the output ripple to a chosen value in a buck converter (continuous conduction, ideal capacitor).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _I("Io", "load current"), _F("fs", "switching frequency"),
             _numin("r", "ripple ratio dIL/Io (a fraction or %)", pct=True), _V("dV", "allowed peak-to-peak output ripple")],
     outputs=[_o("D", "number", "duty cycle"), _o("L", "inductance", "required inductance"), _o("C", "capacitance", "required output capacitance"),
              _o("IL_max", "current", "peak inductor current"), _o("dIL", "current", "inductor ripple")],
     primary="L", conditions=["0 < Vout < Vin", "0 < r < 2"],
     method="L = (Vin - Vo) D/(r Io fs); C = r Io/(8 fs dV)",
     latex=r"L = \frac{(V_{in}-V_o)D}{r I_o f_s},\quad C = \frac{r I_o}{8 f_s \Delta v_o}",
     run=buck_lc_design, reference=_ref_buck_lc_design,
     wrong=lambda Vin, Vout, Io, fs, r, dV: (lambda D: _out(D=D, L=Vin * D / (r * Io * fs), C=r * Io / (8 * fs * dV), IL_max=Io * (1 + r / 2), dIL=r * Io))(Vout / Vin),
     rand=lambda rng: (lambda Vin: {"Vin": Vin, "Vout": _sig(Vin * rng.uniform(0.1, 0.9), 3), "Io": _sig(_lu(rng, 0.5, 30.0), 3), "fs": _sig(_lu(rng, 2e4, 1e6), 3),
                                    "r": round(rng.uniform(0.1, 0.8), 3), "dV": _sig(_lu(rng, 1e-3, 0.2), 3)})(_sig(rng.uniform(8.0, 100.0), 3)),
     phrases=("Buck LC filter design", "Buck converter filter design"),
     examples=[_ex({"Vin": 12.0, "Vout": 3.3, "Io": 5.0, "fs": 500e3, "r": 0.3, "dV": 0.01}),
               _ex({"Vin": 48.0, "Vout": 12.0, "Io": 10.0, "fs": 100e3, "r": 0.4, "dV": 0.05})],
     level="intermediate",
     extra_q=[{"q": "Buck LC filter design: Vin = 12 V, Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 30%, dV = 10 mV", "like": 0}])


def buck_currents(Vin, Vout, Io, L, fs):
    _pos("Vin, Vout, Io, L and fs", Vin, Vout, Io, L, fs)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    D = Vout / Vin
    dI = (Vin - Vout) * D / (L * fs)
    _ccm_ok(Io - dI / 2.0, Io)
    m2 = Io * Io + dI * dI / 12.0
    sw = math.sqrt(D * m2)
    return _out(D=D, I_sw_avg=D * Io, I_sw_rms=sw, I_d_avg=(1.0 - D) * Io, I_d_rms=math.sqrt((1.0 - D) * m2),
                I_L_rms=math.sqrt(m2), I_pk=Io + dI / 2.0, I_cin_rms=math.sqrt(D * m2 - (D * Io) ** 2))


def _ref_buck_currents(Vin, Vout, Io, L, fs):
    T = 1.0 / fs
    D = Vout / Vin
    c, _ = _pwl([(D * T, Vin - Vout), ((1 - D) * T, -Vout)], L, Io)
    on = lambda t: c[0] + (Vin - Vout) * t / L        # noqa: E731
    off = lambda t: c[1] - Vout * t / L               # noqa: E731
    s_avg = _quad(on, 0, D * T) / T
    s_ms = _quad(lambda t: on(t) ** 2, 0, D * T) / T
    d_avg = _quad(off, 0, (1 - D) * T) / T
    d_ms = _quad(lambda t: off(t) ** 2, 0, (1 - D) * T) / T
    # input capacitor: the switch current minus its average (the source supplies the dc part)
    cin = (_quad(lambda t: (on(t) - s_avg) ** 2, 0, D * T) + _quad(lambda t: s_avg ** 2, 0, (1 - D) * T)) / T
    return {"D": D, "I_sw_avg": s_avg, "I_sw_rms": math.sqrt(s_ms), "I_d_avg": d_avg, "I_d_rms": math.sqrt(d_ms),
            "I_L_rms": math.sqrt(s_ms + d_ms), "I_pk": max(c), "I_cin_rms": math.sqrt(cin)}


_add("pe_buck_component_currents",
     name="Buck converter switch, diode, inductor and input-capacitor currents", topic="DC-DC converters / component stresses",
     purpose="The average and RMS currents of the switch and the diode, the RMS inductor current, the peak current and the RMS input-capacitor current of a buck converter in continuous conduction (trapezoidal waveforms with ripple).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _I("Io", "load current"), _L("L", "inductance"), _F("fs", "switching frequency")],
     outputs=[_o("D", "number", "duty cycle"), _o("I_sw_avg", "current", "average switch current"), _o("I_sw_rms", "current", "RMS switch current"),
              _o("I_d_avg", "current", "average diode current"), _o("I_d_rms", "current", "RMS diode current"), _o("I_L_rms", "current", "RMS inductor current"),
              _o("I_pk", "current", "peak switch and inductor current"), _o("I_cin_rms", "current", "RMS input-capacitor ripple current")],
     primary="I_sw_rms", conditions=["0 < Vout < Vin", "continuous conduction"],
     method="I_sw,rms = sqrt(D (Io^2 + dI^2/12)); I_d,rms = sqrt((1 - D)(Io^2 + dI^2/12)); I_cin,rms = sqrt(I_sw,rms^2 - (D Io)^2)",
     latex=r"I_{sw,rms} = \sqrt{D\left(I_o^2 + \tfrac{\Delta i_L^2}{12}\right)},\quad I_{C_{in},rms} = \sqrt{I_{sw,rms}^2 - (D I_o)^2}",
     run=buck_currents, reference=_ref_buck_currents,
     wrong=lambda Vin, Vout, Io, L, fs: (lambda r, D: dict(r, I_sw_rms=_r(Io * math.sqrt(D)), I_cin_rms=_r(Io * math.sqrt(D * (1 - D)))))(buck_currents(Vin, Vout, Io, L, fs), Vout / Vin),
     rand=lambda rng: (lambda b: {"Vin": b["Vin"], "Vout": b["Vout"], "Io": b["Io"], "L": b["L"], "fs": b["fs"]})(_rand_buck(rng)),
     phrases=("Buck converter component currents", "Buck converter switch and diode currents"),
     examples=[_ex({"Vin": 24.0, "Vout": 5.0, "Io": 3.0, "L": 22e-6, "fs": 200e3}), _ex({"Vin": 48.0, "Vout": 12.0, "Io": 10.0, "L": 22e-6, "fs": 100e3})],
     level="intermediate")


def interleaved_buck(N, Vin, Vout, L, fs):
    _need(isinstance(N, int) and 2 <= N <= 12, "N must be a whole number of phases from 2 to 12")
    _pos("Vin, Vout, L and fs", Vin, Vout, L, fs)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    D = Vout / Vin
    dI = (Vin - Vout) * D / (L * fs)
    m = math.floor(N * D)
    K = N * (D - m / N) * ((m + 1) / N - D) / (D * (1.0 - D))
    K = _snap(K, 1.0)
    return _out(D=D, dI_phase=dI, dI_total=K * dI, cancellation=K, f_ripple=N * fs)


def _ref_interleaved_buck(N, Vin, Vout, L, fs):
    T = 1.0 / fs
    D = Vout / Vin
    c, _ = _pwl([(D * T, Vin - Vout), ((1 - D) * T, -Vout)], L, 0.0)

    def phase(t):                      # one phase's ripple current (zero mean), periodic in T
        t = t % T
        return c[0] + (Vin - Vout) * t / L if t < D * T else c[1] - Vout * (t - D * T) / L
    # the summed current is piecewise linear: sample every breakpoint of every phase
    pts = sorted({((k * T / N) + s) % T for k in range(N) for s in (0.0, D * T)} | {0.0})
    tot = [sum(phase(t - k * T / N) for k in range(N)) for t in pts]
    rip = max(tot) - min(tot)
    rip = 0.0 if rip <= 1e-12 * (max(c) - min(c)) else rip
    return {"D": D, "dI_phase": max(c) - min(c), "dI_total": rip, "cancellation": rip / (max(c) - min(c)), "f_ripple": N / T}


_add("pe_interleaved_buck_ripple",
     name="Interleaved multiphase buck: output current ripple cancellation", topic="DC-DC converters / interleaving",
     purpose="The per-phase and total output current ripple of an N-phase interleaved buck converter (phases shifted by T/N, equal inductors), the cancellation factor and the ripple frequency N fs.",
     inputs=[_intin("N", "number of phases", 2, 12), _V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L", "inductance of each phase"),
             _F("fs", "switching frequency of each phase")],
     outputs=[_o("D", "number", "duty cycle"), _o("dI_phase", "current", "ripple of one phase"), _o("dI_total", "current", "ripple of the summed output current"),
              _o("cancellation", "number", "ratio of total to per-phase ripple"), _o("f_ripple", "frequency", "output ripple frequency N fs")],
     primary="dI_total", conditions=["2 <= N <= 12", "0 < Vout < Vin", "continuous conduction in each phase"],
     method="m = floor(N D); dI_total/dI_phase = N (D - m/N)((m + 1)/N - D)/(D (1 - D)); zero when N D is a whole number",
     latex=r"\frac{\Delta i_{tot}}{\Delta i_{ph}} = \frac{N\left(D-\frac{m}{N}\right)\left(\frac{m+1}{N}-D\right)}{D(1-D)},\ m = \lfloor ND\rfloor",
     run=interleaved_buck, reference=_ref_interleaved_buck,
     wrong=lambda N, Vin, Vout, L, fs: (lambda r: dict(r, dI_total=_r(r["dI_phase"] / N), cancellation=_r(1.0 / N)))(interleaved_buck(N, Vin, Vout, L, fs)),
     rand=lambda rng: (lambda Vin: {"N": rng.randint(2, 6), "Vin": Vin, "Vout": _sig(Vin * rng.uniform(0.05, 0.95), 3), "L": _sig(_lu(rng, 1e-7, 1e-4), 3), "fs": _sig(_lu(rng, 1e5, 2e6), 3)})(_sig(rng.uniform(5.0, 48.0), 3)),
     phrases=("Interleaved buck ripple", "Multiphase buck ripple"),
     examples=[_ex({"N": 2, "Vin": 12.0, "Vout": 1.2, "L": 0.47e-6, "fs": 500e3}), _ex({"N": 4, "Vin": 12.0, "Vout": 1.8, "L": 0.33e-6, "fs": 600e3})],
     level="expert")


# ================================================================================================ DC-DC: DISCONTINUOUS CONDUCTION
def _dcm_common(Vin, L, fs, R, D):
    _pos("Vin, L, fs and R", Vin, L, fs, R)
    _frac("the duty cycle D", D)
    return 2.0 * L * fs / R


def buck_dcm(Vin, L, fs, R, D):
    K = _dcm_common(Vin, L, fs, R, D)
    _need(K < 1.0 - D, "K = 2L/(R Ts) >= 1 - D: the converter is in continuous conduction (use the CCM capability)")
    M = 2.0 / (1.0 + math.sqrt(1.0 + 4.0 * K / (D * D)))
    Vo = M * Vin
    return _out(K=K, M=M, Vout=Vo, D2=D * (1.0 - M) / M, I_pk=(Vin - Vo) * D / (L * fs), Io=Vo / R)


def _ref_buck_dcm(Vin, L, fs, R, D):
    T = 1.0 / fs

    def mismatch(Vo):                  # simulate one DCM period at a trial Vo; compare the average current with Vo/R
        ipk = _quad(lambda t: (Vin - Vo) / L, 0.0, D * T)
        t2 = ipk / (Vo / L)
        q = _quad(lambda t: (Vin - Vo) * t / L, 0.0, D * T) + _quad(lambda t: ipk - Vo * t / L, 0.0, t2)
        return q / T - Vo / R
    Vo = _brentq(mismatch, 1e-9 * Vin, Vin * (1 - 1e-12))
    ipk = (Vin - Vo) * D * T / L
    return {"K": 2 * L / (R * T), "M": Vo / Vin, "Vout": Vo, "D2": ipk * L / (Vo * T), "I_pk": ipk, "Io": Vo / R}


_add("pe_buck_dcm",
     name="Buck converter in discontinuous conduction", topic="DC-DC converters / discontinuous conduction",
     purpose="The conversion ratio, output voltage, second-interval duty D2 (diode conduction), peak inductor current and load current of an ideal buck converter in discontinuous conduction.",
     inputs=[_V("Vin", "input voltage"), _L("L", "inductance"), _F("fs", "switching frequency"), _R("R", "load resistance"),
             _numin("D", "duty cycle (a fraction or %)", pct=True)],
     outputs=[_o("K", "number", "dimensionless conduction parameter K"), _o("M", "number", "conversion ratio Vout/Vin"), _o("Vout", "voltage", "output voltage"),
              _o("D2", "number", "diode conduction interval as a fraction of the period"), _o("I_pk", "current", "peak inductor current"),
              _o("Io", "current", "load current")],
     primary="M", conditions=["K < 1 - D (discontinuous conduction)"],
     method="K = 2L/(R Ts); M = 2/(1 + sqrt(1 + 4K/D^2)); D2 = D (1 - M)/M; I_pk = (Vin - Vo) D Ts/L",
     latex=r"M = \frac{2}{1+\sqrt{1+4K/D^2}},\quad K = \frac{2L}{R T_s}",
     run=buck_dcm, reference=_ref_buck_dcm,
     wrong=lambda Vin, L, fs, R, D: (lambda r: dict(r, M=_r(D), Vout=_r(D * Vin)))(buck_dcm(Vin, L, fs, R, D)),
     rand=lambda rng: (lambda D, R, fs: {"Vin": _sig(rng.uniform(5.0, 200.0), 3), "L": _sig((1 - D) * rng.uniform(0.05, 0.9) * R / (2 * fs), 3), "fs": fs, "R": R, "D": D})(
         round(rng.uniform(0.1, 0.8), 3), _sig(_lu(rng, 2.0, 500.0), 3), _sig(_lu(rng, 2e4, 5e5), 3)),
     phrases=("Buck converter in DCM", "Buck converter discontinuous conduction"),
     examples=[_ex({"Vin": 24.0, "L": 10e-6, "fs": 100e3, "R": 20.0, "D": 0.3}), _ex({"Vin": 12.0, "L": 4.7e-6, "fs": 200e3, "R": 50.0, "D": 0.2})],
     level="advanced")


def boost_dcm(Vin, L, fs, R, D):
    K = _dcm_common(Vin, L, fs, R, D)
    _need(K < D * (1.0 - D) ** 2, "K = 2L/(R Ts) >= D (1 - D)^2: the converter is in continuous conduction (use the CCM capability)")
    M = (1.0 + math.sqrt(1.0 + 4.0 * D * D / K)) / 2.0
    Vo = M * Vin
    return _out(K=K, M=M, Vout=Vo, D2=D / (M - 1.0), I_pk=Vin * D / (L * fs), Io=Vo / R)


def _ref_boost_dcm(Vin, L, fs, R, D):
    T = 1.0 / fs
    ipk = _quad(lambda t: Vin / L, 0.0, D * T)

    def mismatch(Vo):                  # the diode carries the inductor current while it falls to zero
        t2 = ipk / ((Vo - Vin) / L)
        q = _quad(lambda t: ipk - (Vo - Vin) * t / L, 0.0, t2)
        return q / T - Vo / R
    Vo = _brentq(mismatch, Vin * (1 + 1e-12), Vin * 1e6)
    return {"K": 2 * L / (R * T), "M": Vo / Vin, "Vout": Vo, "D2": ipk * L / ((Vo - Vin) * T), "I_pk": ipk, "Io": Vo / R}


_add("pe_boost_dcm",
     name="Boost converter in discontinuous conduction", topic="DC-DC converters / discontinuous conduction",
     purpose="The conversion ratio, output voltage, diode conduction interval D2, peak inductor current and load current of an ideal boost converter in discontinuous conduction.",
     inputs=[_V("Vin", "input voltage"), _L("L", "inductance"), _F("fs", "switching frequency"), _R("R", "load resistance"),
             _numin("D", "duty cycle (a fraction or %)", pct=True)],
     outputs=[_o("K", "number", "dimensionless conduction parameter K"), _o("M", "number", "conversion ratio Vout/Vin"), _o("Vout", "voltage", "output voltage"),
              _o("D2", "number", "diode conduction interval as a fraction of the period"), _o("I_pk", "current", "peak inductor current"),
              _o("Io", "current", "load current")],
     primary="M", conditions=["K < D (1 - D)^2 (discontinuous conduction)"],
     method="M = (1 + sqrt(1 + 4 D^2/K))/2; D2 = D/(M - 1); I_pk = Vin D Ts/L",
     latex=r"M = \frac{1+\sqrt{1+4D^2/K}}{2},\quad D_2 = \frac{D}{M-1}",
     run=boost_dcm, reference=_ref_boost_dcm,
     wrong=lambda Vin, L, fs, R, D: (lambda r: dict(r, M=_r(1 / (1 - D)), Vout=_r(Vin / (1 - D))))(boost_dcm(Vin, L, fs, R, D)),
     rand=lambda rng: (lambda D, R, fs: {"Vin": _sig(rng.uniform(3.0, 100.0), 3), "L": _sig(D * (1 - D) ** 2 * rng.uniform(0.05, 0.9) * R / (2 * fs), 3), "fs": fs, "R": R, "D": D})(
         round(rng.uniform(0.1, 0.8), 3), _sig(_lu(rng, 10.0, 2000.0), 3), _sig(_lu(rng, 2e4, 5e5), 3)),
     phrases=("Boost converter in DCM", "Boost converter discontinuous conduction"),
     examples=[_ex({"Vin": 12.0, "L": 10e-6, "fs": 100e3, "R": 100.0, "D": 0.4}), _ex({"Vin": 5.0, "L": 2.2e-6, "fs": 500e3, "R": 200.0, "D": 0.3})],
     level="advanced")


def buck_boost_dcm(Vin, L, fs, R, D):
    K = _dcm_common(Vin, L, fs, R, D)
    _need(K < (1.0 - D) ** 2, "K = 2L/(R Ts) >= (1 - D)^2: the converter is in continuous conduction (use the CCM capability)")
    M = D / math.sqrt(K)
    Vo = M * Vin
    return _out(K=K, M=M, Vout=Vo, D2=math.sqrt(K), I_pk=Vin * D / (L * fs), Io=Vo / R)


def _ref_buck_boost_dcm(Vin, L, fs, R, D):
    T = 1.0 / fs
    ipk = _quad(lambda t: Vin / L, 0.0, D * T)

    def mismatch(Vo):
        t2 = ipk / (Vo / L)
        return _quad(lambda t: ipk - Vo * t / L, 0.0, t2) / T - Vo / R
    Vo = _brentq(mismatch, Vin * 1e-9, Vin * 1e6)
    return {"K": 2 * L / (R * T), "M": Vo / Vin, "Vout": Vo, "D2": ipk * L / (Vo * T), "I_pk": ipk, "Io": Vo / R}


_add("pe_buck_boost_dcm",
     name="Buck-boost converter in discontinuous conduction", topic="DC-DC converters / discontinuous conduction",
     purpose="The conversion ratio (magnitude), output voltage, diode conduction interval, peak inductor current and load current of an ideal inverting buck-boost converter in discontinuous conduction.",
     inputs=[_V("Vin", "input voltage"), _L("L", "inductance"), _F("fs", "switching frequency"), _R("R", "load resistance"),
             _numin("D", "duty cycle (a fraction or %)", pct=True)],
     outputs=[_o("K", "number", "dimensionless conduction parameter K"), _o("M", "number", "conversion ratio |Vout|/Vin"), _o("Vout", "voltage", "output voltage magnitude"),
              _o("D2", "number", "diode conduction interval (= sqrt K)"), _o("I_pk", "current", "peak inductor current"), _o("Io", "current", "load current")],
     primary="M", conditions=["K < (1 - D)^2 (discontinuous conduction)"],
     method="M = D/sqrt(K); D2 = sqrt(K); I_pk = Vin D Ts/L",
     latex=r"M = \frac{D}{\sqrt{K}},\quad D_2 = \sqrt{K}",
     run=buck_boost_dcm, reference=_ref_buck_boost_dcm,
     wrong=lambda Vin, L, fs, R, D: (lambda r: dict(r, M=_r(D / (1 - D)), Vout=_r(Vin * D / (1 - D))))(buck_boost_dcm(Vin, L, fs, R, D)),
     rand=lambda rng: (lambda D, R, fs: {"Vin": _sig(rng.uniform(3.0, 100.0), 3), "L": _sig((1 - D) ** 2 * rng.uniform(0.05, 0.9) * R / (2 * fs), 3), "fs": fs, "R": R, "D": D})(
         round(rng.uniform(0.1, 0.8), 3), _sig(_lu(rng, 5.0, 1000.0), 3), _sig(_lu(rng, 2e4, 5e5), 3)),
     phrases=("Buck-boost converter in DCM", "Buck-boost converter discontinuous conduction"),
     examples=[_ex({"Vin": 12.0, "L": 10e-6, "fs": 100e3, "R": 50.0, "D": 0.4}), _ex({"Vin": 24.0, "L": 20e-6, "fs": 50e3, "R": 100.0, "D": 0.25})],
     level="advanced")


def flyback_dcm_design(Vin, Po, eff, fs, D):
    _pos("Vin, Po and fs", Vin, Po, fs)
    _frac("the efficiency", eff)
    _frac("the duty cycle D", D)
    Pin = Po / eff
    Lm = (Vin * D) ** 2 / (2.0 * Pin * fs)
    Ipk = Vin * D / (Lm * fs)
    return _out(Pin=Pin, Lm=Lm, I_pk=Ipk, I_rms=Ipk * math.sqrt(D / 3.0), I_avg=Pin / Vin, E_cycle=Pin / fs)


def _ref_flyback_dcm_design(Vin, Po, eff, fs, D):
    T = 1.0 / fs
    Pin = Po / eff

    def power(Lm):                     # the input power when the primary current ramps from zero during D T
        return _quad(lambda t: Vin * Vin * t / Lm, 0.0, D * T) / T
    Lm = _brentq(lambda L: power(L) - Pin, 1e-12, 100.0)
    ipk = _quad(lambda t: Vin / Lm, 0.0, D * T)
    rms = math.sqrt(_quad(lambda t: (Vin * t / Lm) ** 2, 0.0, D * T) / T)
    return {"Pin": Pin, "Lm": Lm, "I_pk": ipk, "I_rms": rms, "I_avg": _quad(lambda t: Vin * t / Lm, 0.0, D * T) / T, "E_cycle": Pin * T}


_add("pe_flyback_dcm_design",
     name="Flyback magnetizing inductance for discontinuous conduction", topic="isolated converters / flyback design",
     purpose="The magnetizing inductance at which a flyback converter delivers a power at a duty cycle in discontinuous conduction (the primary current ramps from zero every period), with the peak, RMS and average primary currents.",
     inputs=[_V("Vin", "input voltage (the minimum, for design)"), _phys("Po", "power", "output power"), _numin("eff", "efficiency (a fraction or %)", pct=True),
             _F("fs", "switching frequency"), _numin("D", "duty cycle at this input voltage (a fraction or %)", pct=True)],
     outputs=[_o("Pin", "power", "input power"), _o("Lm", "inductance", "magnetizing inductance"), _o("I_pk", "current", "peak primary current"),
              _o("I_rms", "current", "RMS primary current"), _o("I_avg", "current", "average input current"), _o("E_cycle", "energy", "energy stored per cycle")],
     primary="Lm", conditions=["discontinuous conduction: the stored energy is fully released every period"],
     method="Pin = Po/eff = Lm Ipk^2 fs/2 with Ipk = Vin D/(Lm fs) -> Lm = (Vin D)^2/(2 Pin fs); I_rms = Ipk sqrt(D/3)",
     latex=r"L_m = \frac{(V_{in}D)^2}{2 P_{in} f_s},\quad I_{pk} = \frac{V_{in}D}{L_m f_s},\quad I_{rms} = I_{pk}\sqrt{D/3}",
     run=flyback_dcm_design, reference=_ref_flyback_dcm_design,
     wrong=lambda Vin, Po, eff, fs, D: (lambda r: dict(r, Lm=_r((Vin * D) ** 2 / (2 * Po * fs))))(flyback_dcm_design(Vin, Po, eff, fs, D)),
     rand=lambda rng: {"Vin": _sig(rng.uniform(12.0, 400.0), 3), "Po": _sig(_lu(rng, 1.0, 150.0), 3), "eff": round(rng.uniform(0.7, 0.95), 3),
                       "fs": _sig(_lu(rng, 3e4, 3e5), 3), "D": round(rng.uniform(0.2, 0.5), 3)},
     phrases=("Flyback DCM design", "Flyback magnetizing inductance"),
     examples=[_ex({"Vin": 100.0, "Po": 30.0, "eff": 0.85, "fs": 100e3, "D": 0.45}), _ex({"Vin": 18.0, "Po": 10.0, "eff": 0.8, "fs": 200e3, "D": 0.4})],
     level="advanced",
     extra_q=[{"q": "Flyback DCM design: Vin = 100 V, Po = 30 W, eff = 85%, fs = 100 kHz, D = 45%", "like": 0}])


# ================================================================================================ LOSSES IN CONVERTERS
def boost_nonideal(Vin, D, R, RL):
    _pos("Vin, R and RL", Vin, R, RL)
    _frac("the duty cycle D", D)
    _need(RL < R, "the inductor resistance must be below the load resistance")
    a = RL / ((1.0 - D) ** 2 * R)
    eta = 1.0 / (1.0 + a)
    M = eta / (1.0 - D)
    Vo = M * Vin
    return _out(M=M, Vout=Vo, efficiency=eta, IL=Vo / (R * (1.0 - D)), D_peak=1.0 - math.sqrt(RL / R), M_max=0.5 * math.sqrt(R / RL))


def _ref_boost_nonideal(Vin, D, R, RL):
    import sympy as sp
    vo, il, d = sp.symbols("Vo IL d")
    # averaged equations: inductor volt-seconds with its resistance, capacitor charge balance
    eqs = [_sq(Vin) - il * _sq(RL) - (1 - d) * vo, (1 - d) * il - vo / _sq(R)]
    s = sp.solve(eqs, [vo, il], dict=True)[0]
    Vo = float(s[vo].subs(d, _sq(D)))
    IL = float(s[il].subs(d, _sq(D)))
    Mexpr = s[vo] / _sq(Vin)
    dM = sp.lambdify(d, sp.diff(Mexpr, d))
    Dp = _brentq(lambda x: float(dM(x)), 1e-9, 1 - 1e-9)
    return {"M": Vo / Vin, "Vout": Vo, "efficiency": (Vo * Vo / R) / (Vin * IL), "IL": IL, "D_peak": Dp,
            "M_max": float(Mexpr.subs(d, sp.Float(Dp, 30)))}


_add("pe_boost_nonideal_gain",
     name="Boost converter with inductor resistance: gain and efficiency", topic="losses / non-ideal converters",
     purpose="The conversion ratio, output voltage, efficiency and inductor current of a boost converter whose inductor has a series resistance, with the duty cycle of maximum gain and that maximum gain.",
     inputs=[_V("Vin", "input voltage"), _numin("D", "duty cycle (a fraction or %)", pct=True), _R("R", "load resistance"), _R("RL", "inductor series resistance")],
     outputs=[_o("M", "number", "conversion ratio"), _o("Vout", "voltage", "output voltage"), _o("efficiency", "number", "efficiency"),
              _o("IL", "current", "average inductor current"), _o("D_peak", "number", "duty cycle of maximum gain"), _o("M_max", "number", "maximum gain")],
     primary="M", conditions=["0 < RL < R", "continuous conduction, ideal switches"],
     method="M = (1/(1 - D)) / (1 + RL/((1 - D)^2 R)); efficiency = 1/(1 + RL/((1 - D)^2 R)); D_peak = 1 - sqrt(RL/R); M_max = sqrt(R/RL)/2",
     latex=r"M = \frac{1}{1-D}\,\frac{1}{1+\frac{R_L}{(1-D)^2R}},\quad \eta = \frac{1}{1+\frac{R_L}{(1-D)^2R}}",
     run=boost_nonideal, reference=_ref_boost_nonideal,
     wrong=lambda Vin, D, R, RL: (lambda r: dict(r, M=_r((1 / (1 - D)) / (1 + RL / ((1 - D) * R))), Vout=_r(Vin * (1 / (1 - D)) / (1 + RL / ((1 - D) * R)))))(boost_nonideal(Vin, D, R, RL)),
     rand=lambda rng: {"Vin": _sig(rng.uniform(3.0, 100.0), 3), "D": round(rng.uniform(0.1, 0.9), 3), "R": _sig(_lu(rng, 5.0, 500.0), 3), "RL": _sig(_lu(rng, 0.01, 2.0), 3)},
     phrases=("Non-ideal boost converter", "Boost converter with inductor resistance"),
     examples=[_ex({"Vin": 12.0, "D": 0.75, "R": 50.0, "RL": 0.5}), _ex({"Vin": 24.0, "D": 0.5, "R": 20.0, "RL": 0.1})],
     level="advanced")


def buck_losses(Vin, Vout, Io, fs, Rds, RL, VF, tsw):
    _pos("Vin, Vout, Io and fs", Vin, Vout, Io, fs)
    _nonneg("Rds, RL, VF and tsw", Rds, RL, VF, tsw)
    _need(Io * Rds < Vin, "the switch drop exceeds the input voltage")
    D = (Vout + VF + Io * RL) / (Vin - Io * Rds + VF)
    _need(0 < D < 1, "no duty cycle below 1 reaches this output voltage with these losses")
    Pc = Io * Io * Rds * D
    Pd = VF * Io * (1.0 - D)
    Pl = Io * Io * RL
    Psw = 0.5 * Vin * Io * tsw * fs
    Po = Vout * Io
    Pin = Po + Pc + Pd + Pl + Psw
    return _out(D=D, P_switch_cond=Pc, P_diode=Pd, P_inductor=Pl, P_switching=Psw, P_loss=Pin - Po, efficiency=Po / Pin)


def _ref_buck_losses(Vin, Vout, Io, fs, Rds, RL, VF, tsw):
    import sympy as sp
    d = sp.Symbol("D")
    # averaged switch-node voltage minus the inductor drop equals Vout
    D = _sym_solve([(_sq(Vin) - _sq(Io) * _sq(Rds)) * d - _sq(VF) * (1 - d)
                    - _sq(Io) * _sq(RL) - _sq(Vout)], [d])["D"]
    T = 1.0 / fs
    Pc = _quad(lambda t: Io * Io * Rds, 0.0, D * T) / T
    Pd = _quad(lambda t: VF * Io, 0.0, (1 - D) * T) / T
    Pl = Io * Io * RL
    # linear voltage-current crossover at turn-on and turn-off, tsw/2 each, at full Vin and Io
    Psw = 2 * _quad(lambda t: Vin * Io * (t / (tsw / 2)), 0.0, tsw / 2) * fs if tsw > 0 else 0.0
    Po = Vout * Io
    return {"D": D, "P_switch_cond": Pc, "P_diode": Pd, "P_inductor": Pl, "P_switching": Psw, "P_loss": Pc + Pd + Pl + Psw,
            "efficiency": Po / (Po + Pc + Pd + Pl + Psw)}


_add("pe_buck_with_losses",
     name="Buck converter efficiency with conduction and switching losses", topic="losses / efficiency",
     purpose="The duty cycle needed and the losses of a buck converter with a switch on-resistance, inductor resistance, diode forward drop and hard switching: conduction, diode, inductor and switching losses and the efficiency (ripple neglected).",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _I("Io", "load current"), _F("fs", "switching frequency"),
             _R("Rds", "switch on-resistance", ), _R("RL", "inductor resistance"), _V("VF", "diode forward voltage"),
             _phys("tsw", "time", "total switching (crossover) time per period, turn-on plus turn-off", show=_NS)],
     outputs=[_o("D", "number", "duty cycle"), _o("P_switch_cond", "power", "switch conduction loss"), _o("P_diode", "power", "diode conduction loss"),
              _o("P_inductor", "power", "inductor copper loss"), _o("P_switching", "power", "switching loss"), _o("P_loss", "power", "total loss"),
              _o("efficiency", "number", "efficiency")],
     primary="efficiency", conditions=["continuous conduction, ripple neglected", "hard switching with linear crossover at Vin and Io"],
     method="D = (Vo + VF + Io RL)/(Vin - Io Rds + VF); P_sw,cond = Io^2 Rds D; P_D = VF Io (1 - D); P_L = Io^2 RL; P_sw = Vin Io tsw fs/2",
     latex=r"D = \frac{V_o+V_F+I_oR_L}{V_{in}-I_oR_{ds}+V_F},\quad P_{sw} = \tfrac12 V_{in}I_o t_{sw} f_s,\quad \eta = \frac{P_o}{P_o+\sum P}",
     run=buck_losses, reference=_ref_buck_losses,
     wrong=lambda Vin, Vout, Io, fs, Rds, RL, VF, tsw: (lambda r: dict(r, P_switch_cond=_r(Io * Io * Rds), P_loss=_r(r["P_loss"] + Io * Io * Rds * (1 - r["D"])),
                                                                       efficiency=_r(Vout * Io / (Vout * Io + r["P_loss"] + Io * Io * Rds * (1 - r["D"])))))(buck_losses(Vin, Vout, Io, fs, Rds, RL, VF, tsw)),
     rand=lambda rng: (lambda Vin: {"Vin": Vin, "Vout": _sig(Vin * rng.uniform(0.1, 0.8), 3), "Io": _sig(_lu(rng, 0.5, 20.0), 3), "fs": _sig(_lu(rng, 5e4, 1e6), 3),
                                    "Rds": _sig(_lu(rng, 0.002, 0.2), 3), "RL": _sig(_lu(rng, 0.002, 0.1), 3), "VF": _sig(rng.uniform(0.3, 0.9), 3),
                                    "tsw": _sig(_lu(rng, 5e-9, 1e-7), 3)})(_sig(rng.uniform(8.0, 60.0), 3)),
     phrases=("Buck converter losses", "Buck converter efficiency with losses"),
     examples=[_ex({"Vin": 24.0, "Vout": 5.0, "Io": 4.0, "fs": 200e3, "Rds": 0.02, "RL": 0.01, "VF": 0.5, "tsw": 40e-9}),
               _ex({"Vin": 48.0, "Vout": 12.0, "Io": 10.0, "fs": 100e3, "Rds": 0.01, "RL": 0.005, "VF": 0.7, "tsw": 60e-9})],
     level="advanced")


# ================================================================================================ CONTROL OF CONVERTERS
def small_signal(topology, Vin, D, L, C, R):
    _pos("Vin, L, C and R", Vin, L, C, R)
    _frac("the duty cycle D", D)
    _need(topology in _TOPO3, "topology: buck, boost or buck-boost")
    Dp = 1.0 - D
    if topology == "buck":
        g0, w0, Q, wz = Vin, 1.0 / math.sqrt(L * C), R * math.sqrt(C / L), 0.0
    elif topology == "boost":
        g0, w0, Q, wz = Vin / (Dp * Dp), Dp / math.sqrt(L * C), Dp * R * math.sqrt(C / L), Dp * Dp * R / L
    else:
        g0, w0, Q, wz = Vin / (Dp * Dp), Dp / math.sqrt(L * C), Dp * R * math.sqrt(C / L), Dp * Dp * R / (D * L)
    return _out(Gvd0=g0, f0=w0 / TWO_PI, Q=Q, f_rhpz=wz / TWO_PI, rhp_zero="none" if topology == "buck" else "present")


def _ref_small_signal(topology, Vin, D, L, C, R):
    import numpy as np
    import sympy as sp
    il, vc, d, s = sp.symbols("iL vC d s")
    vin, LL, CC, RR = _sq(Vin), _sq(L), _sq(C), _sq(R)
    # averaged state equations (vC = output magnitude): L diL/dt, C dvC/dt
    if topology == "buck":
        f = [(d * vin - vc) / LL, (il - vc / RR) / CC]
    elif topology == "boost":
        f = [(vin - (1 - d) * vc) / LL, ((1 - d) * il - vc / RR) / CC]
    else:
        f = [(d * vin - (1 - d) * vc) / LL, ((1 - d) * il - vc / RR) / CC]
    ss = sp.solve([fi.subs(d, _sq(D)) for fi in f], [il, vc], dict=True)[0]
    op = {il: ss[il], vc: ss[vc], d: _sq(D)}
    A = sp.Matrix([[sp.diff(fi, x).subs(op) for x in (il, vc)] for fi in f])
    B = sp.Matrix([sp.diff(fi, d).subs(op) for fi in f])
    G = sp.cancel((sp.Matrix([[0, 1]]) * (s * sp.eye(2) - A).adjugate() * B)[0, 0] / (s * sp.eye(2) - A).det())
    num, den = sp.fraction(sp.together(G))
    nc = [float(c) for c in sp.Poly(sp.expand(num), s).all_coeffs()]
    dc = [float(c) for c in sp.Poly(sp.expand(den), s).all_coeffs()]
    a2, a1, a0 = dc[0], dc[1], dc[2]
    w0 = math.sqrt(a0 / a2)
    zeros = [complex(z) for z in np.roots(nc)] if len(nc) > 1 else []
    rhp = [z.real for z in zeros if z.real > 0]
    return {"Gvd0": nc[-1] / a0, "f0": w0 / TWO_PI, "Q": w0 * a2 / a1, "f_rhpz": (rhp[0] / TWO_PI) if rhp else 0.0,
            "rhp_zero": "present" if rhp else "none"}


_add("pe_converter_small_signal",
     name="Control-to-output transfer function of a DC-DC converter", topic="control / averaged small-signal model",
     purpose="The control-to-output (duty-to-voltage) transfer function features of an ideal buck, boost or buck-boost converter in continuous conduction: dc gain, double-pole frequency, quality factor and right-half-plane zero.",
     inputs=[_choice("topology", _TOPO3, "buck, boost or buck-boost"), _V("Vin", "input voltage"), _numin("D", "duty cycle (a fraction or %)", pct=True),
             _L("L", "inductance"), _C("C", "output capacitance"), _R("R", "load resistance")],
     outputs=[_o("Gvd0", "voltage", "dc gain of vo/d (volts per unit duty)"), _o("f0", "frequency", "double-pole (corner) frequency"),
              _o("Q", "number", "quality factor of the double pole"), _o("f_rhpz", "frequency", "right-half-plane zero frequency"),
              _o("rhp_zero", "number", "whether a right-half-plane zero is present")],
     primary="f0", conditions=["ideal converter in continuous conduction, no ESR"],
     method="buck: Gvd0 = Vin, w0 = 1/sqrt(LC), Q = R sqrt(C/L); boost: Gvd0 = Vin/D'^2, w0 = D'/sqrt(LC), Q = D' R sqrt(C/L), wz = D'^2 R/L; buck-boost: wz = D'^2 R/(D L)",
     latex=r"G_{vd}(s) = G_{d0}\frac{1 - s/\omega_z}{1 + s/(Q\omega_0) + (s/\omega_0)^2},\quad \omega_{z,boost} = \frac{D'^2R}{L}",
     run=small_signal, reference=_ref_small_signal,
     wrong=lambda topology, Vin, D, L, C, R: (lambda r: dict(r, f0=_r(1 / (TWO_PI * math.sqrt(L * C))), f_rhpz=_r(r["f_rhpz"] / (1 - D)) if r["f_rhpz"] else 0.0))(small_signal(topology, Vin, D, L, C, R)),
     rand=lambda rng: {"topology": rng.choice(("boost", "buck-boost", "boost", "buck-boost", "buck")), "Vin": _sig(rng.uniform(5.0, 100.0), 3), "D": round(rng.uniform(0.15, 0.8), 3),
                       "L": _sig(_lu(rng, 1e-6, 1e-3), 3), "C": _sig(_lu(rng, 1e-6, 1e-3), 3), "R": _sig(_lu(rng, 1.0, 200.0), 3)},
     phrases=("Converter small-signal model", "Control-to-output transfer function"),
     examples=[_ex({"topology": "boost", "Vin": 12.0, "D": 0.5, "L": 50e-6, "C": 100e-6, "R": 10.0}),
               _ex({"topology": "buck", "Vin": 24.0, "D": 0.4, "L": 22e-6, "C": 220e-6, "R": 2.0})],
     level="expert")


def peak_current_mode(Vin, Vout, L, fs, ma):
    _pos("Vin, Vout, L and fs", Vin, Vout, L, fs)
    _nonneg("the compensation ramp slope ma", ma)
    _need(Vout < Vin, "a buck converter needs Vout < Vin")
    m1 = (Vin - Vout) / L
    m2 = Vout / L
    alpha = -(m2 - ma) / (m1 + ma)
    ma_min = max(0.0, (m2 - m1) / 2.0)
    return _out(D=Vout / Vin, m1=m1, m2=m2, alpha=alpha, stable="stable" if abs(alpha) < 1 else "unstable",
                ma_min=_snap(ma_min, m2), ma_deadbeat=m2)


def _ref_peak_current_mode(Vin, Vout, L, fs, ma):
    T = 1.0 / fs
    m1, m2 = (Vin - Vout) / L, Vout / L
    D = Vout / Vin
    ic = 1.0 + (m1 + ma) * D * T                # control level chosen so the steady-state valley current is 1 A

    def next_valley(i0):                        # simulate one period of the peak-current-mode modulator
        t_on = _brentq(lambda t: i0 + m1 * t - (ic - ma * t), 0.0, T)
        ipk = i0 + _quad(lambda t: m1, 0.0, t_on)
        return ipk - _quad(lambda t: m2, 0.0, T - t_on)
    i_ss = 1.0
    eps = 1e-3 * m1 * D * T
    alpha = (next_valley(i_ss + eps) - next_valley(i_ss - eps)) / (2 * eps)
    return {"D": D, "m1": m1, "m2": m2, "alpha": alpha, "stable": "stable" if abs(alpha) < 1 else "unstable",
            "ma_min": max(0.0, (m2 - m1) / 2.0), "ma_deadbeat": m2}


_add("pe_peak_current_mode_slope",
     name="Peak current mode: subharmonic stability and slope compensation", topic="control / current mode",
     purpose="The inductor current slopes, the period-to-period perturbation factor, the stability verdict and the smallest and the deadbeat compensation ramp slopes of a peak-current-mode buck converter.",
     inputs=[_V("Vin", "input voltage"), _V("Vout", "output voltage"), _L("L", "inductance"), _F("fs", "switching frequency"),
             _phys("ma", "current_slope", "compensation ramp slope referred to the inductor current")],
     outputs=[_o("D", "number", "duty cycle"), _o("m1", "current_slope", "rising inductor current slope"), _o("m2", "current_slope", "falling inductor current slope (magnitude)"),
              _o("alpha", "number", "perturbation factor per switching period"), _o("stable", "number", "stable or unstable"),
              _o("ma_min", "current_slope", "smallest ramp slope for stability at this duty cycle"), _o("ma_deadbeat", "current_slope", "deadbeat compensation ramp slope")],
     primary="alpha", conditions=["0 < Vout < Vin", "continuous conduction"],
     method="m1 = (Vin - Vo)/L, m2 = Vo/L; alpha = -(m2 - ma)/(m1 + ma); stable when |alpha| < 1; ma,min = (m2 - m1)/2",
     latex=r"\alpha = -\frac{m_2 - m_a}{m_1 + m_a},\quad |\alpha| < 1 \Leftarrow m_a > \frac{m_2-m_1}{2}",
     run=peak_current_mode, reference=_ref_peak_current_mode,
     wrong=lambda Vin, Vout, L, fs, ma: (lambda r: dict(r, alpha=_r(-(r["m2"] - ma) / r["m1"])))(peak_current_mode(Vin, Vout, L, fs, ma)),
     rand=lambda rng: (lambda Vin, L: (lambda Vout: {"Vin": Vin, "Vout": Vout, "L": L, "fs": _sig(_lu(rng, 5e4, 1e6), 3),
                                                     "ma": _sig(Vout / L * rng.uniform(0.05, 1.2), 3)})(_sig(Vin * rng.uniform(0.15, 0.85), 3)))(_sig(rng.uniform(5.0, 60.0), 3), _sig(_lu(rng, 1e-6, 1e-4), 3)),
     phrases=("Peak current mode slope compensation", "Current mode slope compensation"),
     examples=[_ex({"Vin": 12.0, "Vout": 8.0, "L": 10e-6, "fs": 300e3, "ma": 0.3e6}), _ex({"Vin": 24.0, "Vout": 5.0, "L": 22e-6, "fs": 200e3, "ma": 0.1e6})],
     level="expert")


# ================================================================================================ RECTIFIERS
_ANG = lambda s, m: _phys(s, "angle", m, show=_DEG)        # noqa: E731


def _ll(theta, k, Vm):
    """Line-to-line voltage k (0..5: ab, ac, bc, ba, ca, cb) of a three-phase source with line peak Vm; v_ab leads
    v_a by 30 degrees (v_a = (Vm/sqrt 3) sin theta)."""
    return Vm * math.sin(theta + math.pi / 6.0 - k * math.pi / 3.0)


def three_phase_bridge(VLL, f, R):
    _pos("VLL, f and R", VLL, f, R)
    Vm = SQ2 * VLL
    Vdc = 3.0 * Vm / math.pi
    Vrms = Vm * math.sqrt(0.5 + 3.0 * SQ3 / (4.0 * math.pi))
    ff = Vrms / Vdc
    return _out(Vdc=Vdc, Vrms=Vrms, ripple_factor=math.sqrt(ff * ff - 1.0), form_factor=ff, f_ripple=6.0 * f, PIV=Vm,
                Idc=Vdc / R, I_diode_avg=Vdc / (3.0 * R), I_diode_rms=Vrms / (R * SQ3), Is_rms=Vrms / R * math.sqrt(2.0 / 3.0))


def _ref_three_phase_bridge(VLL, f, R):
    Vm = SQ2 * VLL
    out = lambda th: max(_ll(th, k, Vm) for k in range(6))                      # noqa: E731  the highest line voltage
    va = lambda th: math.sin(th)                                                # noqa: E731
    vb = lambda th: math.sin(th - 2 * math.pi / 3)                              # noqa: E731
    vc = lambda th: math.sin(th + 2 * math.pi / 3)                              # noqa: E731
    pts = [k * math.pi / 6 for k in range(13)]
    seg = lambda g: sum(_quad(g, pts[i], pts[i + 1]) for i in range(12)) / TWO_PI   # noqa: E731
    Vdc = seg(out)
    Vrms = math.sqrt(seg(lambda th: out(th) ** 2))
    # the upper diode of phase a conducts while v_a is the highest phase voltage; line current = upper - lower diode
    da = lambda th: (out(th) / R) if va(th) >= max(vb(th), vc(th)) else 0.0     # noqa: E731
    dal = lambda th: (out(th) / R) if va(th) <= min(vb(th), vc(th)) else 0.0    # noqa: E731
    return {"Vdc": Vdc, "Vrms": Vrms, "ripple_factor": math.sqrt(Vrms ** 2 - Vdc ** 2) / Vdc, "form_factor": Vrms / Vdc,
            "f_ripple": 6 * f, "PIV": max(_ll(math.pi / 3, k, Vm) for k in range(6)), "Idc": Vdc / R, "I_diode_avg": seg(da),
            "I_diode_rms": math.sqrt(seg(lambda th: da(th) ** 2)), "Is_rms": math.sqrt(seg(lambda th: (da(th) - dal(th)) ** 2))}


_add("pe_three_phase_diode_bridge",
     name="Three-phase diode bridge rectifier with resistive load", topic="rectifiers / three-phase uncontrolled",
     purpose="The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage, and the load, diode and line currents of a six-pulse three-phase diode bridge feeding a resistor.",
     inputs=[_V("VLL", "RMS line-to-line source voltage"), _F("f", "line frequency"), _R("R", "load resistance")],
     outputs=[_o("Vdc", "voltage", "average output voltage"), _o("Vrms", "voltage", "RMS output voltage"),
              _o("ripple_factor", "number", "ripple factor"), _o("form_factor", "number", "form factor"), _o("f_ripple", "frequency", "ripple frequency"),
              _o("PIV", "voltage", "diode peak inverse voltage"), _o("Idc", "current", "average load current"), _o("I_diode_avg", "current", "average diode current"),
              _o("I_diode_rms", "current", "RMS diode current"), _o("Is_rms", "current", "RMS line current")],
     primary="Vdc", conditions=["ideal diodes, no source inductance, resistive load"],
     method="Vm = sqrt(2) VLL; Vdc = 3 Vm/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3)/(4 pi)); diode avg = Idc/3",
     latex=r"V_{dc} = \frac{3\sqrt2\,V_{LL}}{\pi},\quad V_{rms} = \sqrt2 V_{LL}\sqrt{\tfrac12+\tfrac{3\sqrt3}{4\pi}}",
     run=three_phase_bridge, reference=_ref_three_phase_bridge,
     wrong=lambda VLL, f, R: (lambda r: dict(r, Vdc=_r(2 * SQ2 * VLL / math.pi), Idc=_r(2 * SQ2 * VLL / math.pi / R)))(three_phase_bridge(VLL, f, R)),
     rand=lambda rng: {"VLL": _sig(rng.uniform(100.0, 690.0), 3), "f": rng.choice([50.0, 60.0, 400.0]), "R": _sig(_lu(rng, 1.0, 500.0), 3)},
     phrases=("Three-phase diode bridge", "Three-phase bridge rectifier"),
     examples=[_ex({"VLL": 400.0, "f": 50.0, "R": 50.0}), _ex({"VLL": 480.0, "f": 60.0, "R": 20.0})],
     level="beginner")


def three_phase_half_wave(Vph, f, R):
    _pos("Vph, f and R", Vph, f, R)
    Vm = SQ2 * Vph
    Vdc = 3.0 * SQ3 * Vm / TWO_PI
    Vrms = Vm * math.sqrt(0.5 + 3.0 * SQ3 / (8.0 * math.pi))
    ff = Vrms / Vdc
    return _out(Vdc=Vdc, Vrms=Vrms, ripple_factor=math.sqrt(ff * ff - 1.0), form_factor=ff, f_ripple=3.0 * f, PIV=SQ3 * Vm,
                Idc=Vdc / R, I_diode_avg=Vdc / (3.0 * R), I_diode_rms=Vrms / (R * SQ3))


def _ref_three_phase_half_wave(Vph, f, R):
    Vm = SQ2 * Vph
    ph = [lambda th, k=k: Vm * math.sin(th - k * TWO_PI / 3) for k in range(3)]
    out = lambda th: max(p(th) for p in ph)                                      # noqa: E731
    pts = [k * math.pi / 6 for k in range(13)]
    seg = lambda g: sum(_quad(g, pts[i], pts[i + 1]) for i in range(12)) / TWO_PI   # noqa: E731
    Vdc = seg(out)
    Vrms = math.sqrt(seg(lambda th: out(th) ** 2))
    da = lambda th: out(th) / R if ph[0](th) >= max(ph[1](th), ph[2](th)) else 0.0   # noqa: E731
    # a blocking diode sees its own phase minus the output: the largest reverse voltage over a period
    piv = max(out(th) - ph[0](th) for th in [k * math.pi / 3600 for k in range(7201)])
    return {"Vdc": Vdc, "Vrms": Vrms, "ripple_factor": math.sqrt(Vrms ** 2 - Vdc ** 2) / Vdc, "form_factor": Vrms / Vdc, "f_ripple": 3 * f,
            "PIV": piv, "Idc": Vdc / R, "I_diode_avg": seg(da), "I_diode_rms": math.sqrt(seg(lambda th: da(th) ** 2))}


_add("pe_three_phase_half_wave_rectifier",
     name="Three-phase half-wave (three-pulse) diode rectifier", topic="rectifiers / three-phase uncontrolled",
     purpose="The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage and the load and diode currents of a three-pulse (star, half-wave) diode rectifier feeding a resistor.",
     inputs=[_V("Vph", "RMS phase (line-to-neutral) voltage"), _F("f", "line frequency"), _R("R", "load resistance")],
     outputs=[_o("Vdc", "voltage", "average output voltage"), _o("Vrms", "voltage", "RMS output voltage"), _o("ripple_factor", "number", "ripple factor"),
              _o("form_factor", "number", "form factor"), _o("f_ripple", "frequency", "ripple frequency"), _o("PIV", "voltage", "diode peak inverse voltage"),
              _o("Idc", "current", "average load current"), _o("I_diode_avg", "current", "average diode current"), _o("I_diode_rms", "current", "RMS diode current")],
     primary="Vdc", conditions=["ideal diodes, resistive load"],
     method="Vm = sqrt(2) Vph; Vdc = 3 sqrt(3) Vm/(2 pi); Vrms = Vm sqrt(1/2 + 3 sqrt(3)/(8 pi)); PIV = sqrt(3) Vm",
     latex=r"V_{dc} = \frac{3\sqrt3\,V_m}{2\pi},\quad V_{rms} = V_m\sqrt{\tfrac12+\tfrac{3\sqrt3}{8\pi}},\quad PIV = \sqrt3 V_m",
     run=three_phase_half_wave, reference=_ref_three_phase_half_wave,
     wrong=lambda Vph, f, R: (lambda r: dict(r, Vdc=_r(SQ2 * Vph / math.pi * 3 / 2), Idc=_r(SQ2 * Vph / math.pi * 3 / 2 / R)))(three_phase_half_wave(Vph, f, R)),
     rand=lambda rng: {"Vph": _sig(rng.uniform(50.0, 400.0), 3), "f": rng.choice([50.0, 60.0]), "R": _sig(_lu(rng, 1.0, 500.0), 3)},
     phrases=("Three-phase half-wave rectifier", "Three-pulse rectifier"),
     examples=[_ex({"Vph": 230.0, "f": 50.0, "R": 20.0}), _ex({"Vph": 120.0, "f": 60.0, "R": 10.0})],
     level="beginner")


_SPC = ("half-wave", "full-bridge", "semi-converter")


def single_phase_controlled(circuit, Vs, alpha):
    _pos("Vs", Vs)
    _need(circuit in _SPC, "circuit: half-wave, full-bridge or semi-converter")
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Vm = SQ2 * Vs
    s2 = math.sin(2.0 * alpha) / 2.0
    if circuit == "half-wave":
        Vdc = Vm * (1.0 + math.cos(alpha)) / TWO_PI
        Vrms = Vm * math.sqrt((math.pi - alpha + s2) / (4.0 * math.pi))
    elif circuit == "full-bridge":
        Vdc = _snap(2.0 * Vm * math.cos(alpha) / math.pi, Vm)
        Vrms = Vs
    else:
        Vdc = Vm * (1.0 + math.cos(alpha)) / math.pi
        Vrms = Vm * math.sqrt((math.pi - alpha + s2) / TWO_PI)
    ac = math.sqrt(max(Vrms * Vrms - Vdc * Vdc, 0.0))
    return _out(Vdc=Vdc, Vrms=Vrms, Vac_rms=ac, mode="inverting" if Vdc < 0 else "rectifying")


def _ref_single_phase_controlled(circuit, Vs, alpha):
    Vm = SQ2 * Vs
    v = lambda th: Vm * math.sin(th)                       # noqa: E731
    if circuit == "half-wave":                              # resistive load: conducts from alpha to pi, period 2 pi
        P, parts = TWO_PI, [(alpha, math.pi)]
    elif circuit == "full-bridge":                          # continuous current: each pair conducts for pi from alpha
        P, parts = math.pi, [(alpha, alpha + math.pi)]
    else:                                                   # freewheeling diode clamps the output to 0 after pi
        P, parts = math.pi, [(alpha, math.pi)]
    Vdc = sum(_quad(v, a, b) for a, b in parts if b > a) / P
    ms = sum(_quad(lambda th: v(th) ** 2, a, b) for a, b in parts if b > a) / P
    Vdc = 0.0 if abs(Vdc) <= 1e-12 * Vm else Vdc
    return {"Vdc": Vdc, "Vrms": math.sqrt(ms), "Vac_rms": math.sqrt(max(ms - Vdc ** 2, 0.0)), "mode": "inverting" if Vdc < 0 else "rectifying"}


_add("pe_single_phase_controlled_rectifier",
     name="Single-phase phase-controlled thyristor rectifier", topic="rectifiers / phase-controlled",
     purpose="The average, RMS and AC (ripple) output voltage of a single-phase thyristor rectifier at a firing angle: half-wave with a resistive load, fully controlled bridge with continuous load current, or semi-converter (half-controlled) with continuous current.",
     inputs=[_choice("circuit", _SPC, "half-wave, full-bridge or semi-converter"),
             _V("Vs", "RMS source voltage"), _ANG("alpha", "firing angle (delay from the natural commutation point)")],
     outputs=[_o("Vdc", "voltage", "average output voltage"), _o("Vrms", "voltage", "RMS output voltage"), _o("Vac_rms", "voltage", "RMS of the AC component"),
              _o("mode", "number", "rectifying or inverting (negative average voltage)")],
     primary="Vdc", conditions=["0 <= alpha < 180 degrees", "ideal thyristors, no source inductance"],
     method="half-wave: Vdc = Vm (1 + cos a)/(2 pi); full bridge: Vdc = 2 Vm cos a/pi; semi-converter: Vdc = Vm (1 + cos a)/pi",
     latex=r"V_{dc,HW} = \frac{V_m(1+\cos\alpha)}{2\pi},\ V_{dc,FB} = \frac{2V_m\cos\alpha}{\pi},\ V_{dc,semi} = \frac{V_m(1+\cos\alpha)}{\pi}",
     run=single_phase_controlled, reference=_ref_single_phase_controlled,
     wrong=lambda circuit, Vs, alpha: single_phase_controlled(circuit, Vs / SQ2, alpha),
     rand=lambda rng: {"circuit": rng.choice(_SPC), "Vs": _sig(rng.uniform(100.0, 480.0), 3), "alpha": math.radians(round(rng.uniform(0.0, 170.0), 1))},
     phrases=("Single-phase controlled rectifier", "Single-phase thyristor rectifier"),
     examples=[_ex({"circuit": "full-bridge", "Vs": 230.0, "alpha": math.radians(30.0)}), _ex({"circuit": "half-wave", "Vs": 120.0, "alpha": math.radians(60.0)})],
     level="beginner")


def three_phase_controlled(VLL, alpha, Id):
    _pos("VLL and Id", VLL, Id)
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Vm = SQ2 * VLL
    Vdc = _snap(3.0 * Vm * math.cos(alpha) / math.pi, Vm)
    Vrms = Vm * math.sqrt(0.5 + 3.0 * SQ3 * math.cos(2.0 * alpha) / (4.0 * math.pi))
    return _out(Vdc=Vdc, Vrms=Vrms, P=Vdc * Id, I_thy_avg=Id / 3.0, I_thy_rms=Id / SQ3, Is_rms=Id * math.sqrt(2.0 / 3.0),
                mode="inverting" if Vdc < 0 else "rectifying")


def _ref_three_phase_controlled(VLL, alpha, Id):
    Vm = SQ2 * VLL

    def out(th):                       # the pair that would conduct naturally at th - alpha, still conducting at th
        k = max(range(6), key=lambda j: _ll(th - alpha, j, Vm))
        return _ll(th, k, Vm)
    pts = [alpha + k * math.pi / 3 for k in range(7)]
    Vdc = sum(_quad(out, pts[i], pts[i + 1]) for i in range(6)) / TWO_PI
    ms = sum(_quad(lambda th: out(th) ** 2, pts[i], pts[i + 1]) for i in range(6)) / TWO_PI
    Vdc = 0.0 if abs(Vdc) <= 1e-12 * Vm else Vdc
    # each thyristor carries Id for one third of the period; a line carries +Id and -Id for a third each
    return {"Vdc": Vdc, "Vrms": math.sqrt(ms), "P": Vdc * Id, "I_thy_avg": _quad(lambda t: Id, 0.0, TWO_PI / 3) / TWO_PI,
            "I_thy_rms": math.sqrt(_quad(lambda t: Id * Id, 0.0, TWO_PI / 3) / TWO_PI),
            "Is_rms": math.sqrt(2 * _quad(lambda t: Id * Id, 0.0, TWO_PI / 3) / TWO_PI), "mode": "inverting" if Vdc < 0 else "rectifying"}


_add("pe_three_phase_controlled_rectifier",
     name="Three-phase fully controlled thyristor bridge", topic="rectifiers / phase-controlled",
     purpose="The average and RMS output voltage, the power, and the thyristor and line currents of a six-pulse fully controlled thyristor bridge with continuous (constant) load current at a firing angle, rectifying or inverting.",
     inputs=[_V("VLL", "RMS line-to-line source voltage"), _ANG("alpha", "firing angle"), _I("Id", "constant dc output current")],
     outputs=[_o("Vdc", "voltage", "average output voltage"), _o("Vrms", "voltage", "RMS output voltage"), _o("P", "power", "dc output power Vdc Id"),
              _o("I_thy_avg", "current", "average thyristor current"), _o("I_thy_rms", "current", "RMS thyristor current"),
              _o("Is_rms", "current", "RMS line current"), _o("mode", "number", "rectifying or inverting")],
     primary="Vdc", conditions=["0 <= alpha < 180 degrees", "continuous constant current, no source inductance"],
     method="Vdc = 3 sqrt(2) VLL cos(alpha)/pi; Vrms = Vm sqrt(1/2 + 3 sqrt(3) cos(2 alpha)/(4 pi)); I_thy = Id/3 (avg), Id/sqrt(3) (rms)",
     latex=r"V_{dc} = \frac{3\sqrt2 V_{LL}}{\pi}\cos\alpha,\quad I_{s,rms} = \sqrt{\tfrac23}\,I_d",
     run=three_phase_controlled, reference=_ref_three_phase_controlled,
     wrong=lambda VLL, alpha, Id: (lambda r: dict(r, Vdc=_r(3 * VLL * math.cos(alpha) / math.pi), P=_r(3 * VLL * math.cos(alpha) / math.pi * Id)))(three_phase_controlled(VLL, alpha, Id)),
     rand=lambda rng: {"VLL": _sig(rng.uniform(100.0, 690.0), 3), "alpha": math.radians(round(rng.uniform(0.0, 170.0), 1)), "Id": _sig(_lu(rng, 1.0, 500.0), 3)},
     phrases=("Three-phase controlled rectifier", "Three-phase thyristor bridge"),
     examples=[_ex({"VLL": 415.0, "alpha": math.radians(45.0), "Id": 50.0}), _ex({"VLL": 400.0, "alpha": math.radians(120.0), "Id": 20.0})],
     level="intermediate")


def ac_voltage_controller(Vs, alpha, R):
    _pos("Vs and R", Vs, R)
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Vo = Vs * math.sqrt(1.0 - alpha / math.pi + math.sin(2.0 * alpha) / TWO_PI)
    Io = Vo / R
    return _out(Vo_rms=Vo, Io_rms=Io, P=Vo * Vo / R, PF=Vo / Vs, I_scr_avg=SQ2 * Vs * (1.0 + math.cos(alpha)) / (TWO_PI * R), I_scr_rms=Io / SQ2)


def _ref_ac_voltage_controller(Vs, alpha, R):
    Vm = SQ2 * Vs
    v = lambda th: Vm * math.sin(th)                                    # noqa: E731
    ms = (_quad(lambda th: v(th) ** 2, alpha, math.pi) + _quad(lambda th: v(th) ** 2, math.pi + alpha, TWO_PI)) / TWO_PI
    Vo = math.sqrt(ms)
    P = (_quad(lambda th: v(th) * v(th) / R, alpha, math.pi) * 2) / TWO_PI
    return {"Vo_rms": Vo, "Io_rms": Vo / R, "P": P, "PF": P / (Vs * Vo / R), "I_scr_avg": _quad(lambda th: v(th) / R, alpha, math.pi) / TWO_PI,
            "I_scr_rms": math.sqrt(_quad(lambda th: (v(th) / R) ** 2, alpha, math.pi) / TWO_PI)}


_add("pe_ac_voltage_controller",
     name="Single-phase AC voltage controller (resistive load)", topic="rectifiers / AC voltage control",
     purpose="The RMS output voltage and current, load power, input power factor and the average and RMS thyristor currents of a single-phase AC voltage controller (antiparallel thyristors or a triac) with a resistive load at a firing angle.",
     inputs=[_V("Vs", "RMS source voltage"), _ANG("alpha", "firing angle"), _R("R", "load resistance")],
     outputs=[_o("Vo_rms", "voltage", "RMS output voltage"), _o("Io_rms", "current", "RMS load current"), _o("P", "power", "load power"),
              _o("PF", "number", "input power factor"), _o("I_scr_avg", "current", "average thyristor current"), _o("I_scr_rms", "current", "RMS thyristor current")],
     primary="Vo_rms", conditions=["0 <= alpha < 180 degrees", "resistive load"],
     method="Vo = Vs sqrt(1 - alpha/pi + sin(2 alpha)/(2 pi)); PF = Vo/Vs; I_SCR,avg = sqrt(2) Vs (1 + cos alpha)/(2 pi R)",
     latex=r"V_{o} = V_s\sqrt{1-\frac{\alpha}{\pi}+\frac{\sin2\alpha}{2\pi}},\quad PF = \frac{V_o}{V_s}",
     run=ac_voltage_controller, reference=_ref_ac_voltage_controller,
     wrong=lambda Vs, alpha, R: (lambda r: dict(r, Vo_rms=_r(Vs * math.sqrt(1 - alpha / math.pi)), Io_rms=_r(Vs * math.sqrt(1 - alpha / math.pi) / R)))(ac_voltage_controller(Vs, alpha, R)),
     rand=lambda rng: {"Vs": _sig(rng.uniform(100.0, 480.0), 3), "alpha": math.radians(round(rng.uniform(5.0, 170.0), 1)), "R": _sig(_lu(rng, 1.0, 200.0), 3)},
     phrases=("AC voltage controller", "Single-phase AC voltage controller"),
     examples=[_ex({"Vs": 230.0, "alpha": math.radians(60.0), "R": 10.0}), _ex({"Vs": 120.0, "alpha": math.radians(90.0), "R": 20.0})],
     level="intermediate")


_PH2 = ("single-phase", "three-phase")


def line_current_pf(circuit, Id, Vs, alpha):
    _pos("Id and Vs", Id, Vs)
    _need(circuit in _PH2, "circuit: single-phase or three-phase")
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees (0 for diodes)")
    c = _snap(math.cos(alpha), 1.0)
    if circuit == "single-phase":
        Is, Is1 = Id, 2.0 * SQ2 * Id / math.pi
        P, S = 2.0 * SQ2 * Vs * c * Id / math.pi, Vs * Id
    else:
        Is, Is1 = Id * math.sqrt(2.0 / 3.0), math.sqrt(6.0) * Id / math.pi
        P, S = 3.0 * SQ2 * Vs * c * Id / math.pi, SQ3 * Vs * Id * math.sqrt(2.0 / 3.0)
    return _out(Is_rms=Is, Is1_rms=Is1, THD=math.sqrt(Is * Is - Is1 * Is1) / Is1, DPF=c, PF=P / S, P=P, S=S)


def _ref_line_current_pf(circuit, Id, Vs, alpha):
    if circuit == "single-phase":
        Vm = SQ2 * Vs
        cur = lambda th: Id if alpha <= (th % TWO_PI) < alpha + math.pi or (th % TWO_PI) < alpha + math.pi - TWO_PI else -Id   # noqa: E731
        vph, nph = (lambda th: Vm * math.sin(th)), 1
        S = Vs * math.sqrt(_quad(lambda th: cur(th) ** 2, 0, TWO_PI, points=[alpha, alpha + math.pi]) / TWO_PI)
    else:
        Vp = SQ2 * Vs / SQ3                                     # phase a: v_a = Vp sin(th)
        a0 = math.pi / 6 + alpha                                # phase a's upper thyristor conducts 120 deg from here

        def cur(th):
            t = (th - a0) % TWO_PI
            return Id if t < TWO_PI / 3 else (-Id if math.pi <= t < math.pi + TWO_PI / 3 else 0.0)
        vph, nph = (lambda th: Vp * math.sin(th)), 3
        S = SQ3 * Vs * math.sqrt(_quad(lambda th: cur(th) ** 2, 0, TWO_PI, points=[(a0 + k * math.pi / 3) % TWO_PI for k in range(6)]) / TWO_PI)
    pts = sorted({(alpha + k * math.pi / 6) % TWO_PI for k in range(12)} | {(math.pi / 6 + alpha + k * math.pi / 3) % TWO_PI for k in range(6)})
    q = lambda g: sum(_quad(g, a, b) for a, b in zip([0.0] + pts, pts + [TWO_PI]) if b > a)   # noqa: E731
    a1 = q(lambda th: cur(th) * math.cos(th)) / math.pi
    b1 = q(lambda th: cur(th) * math.sin(th)) / math.pi
    Is = math.sqrt(q(lambda th: cur(th) ** 2) / TWO_PI)
    Is1 = math.hypot(a1, b1) / SQ2
    P = nph * q(lambda th: vph(th) * cur(th)) / TWO_PI
    P = 0.0 if abs(P) <= 1e-12 * S else P
    dpf = b1 / math.hypot(a1, b1)
    return {"Is_rms": Is, "Is1_rms": Is1, "THD": math.sqrt(Is * Is - Is1 * Is1) / Is1, "DPF": 0.0 if abs(dpf) < 1e-12 else dpf,
            "PF": P / S, "P": P, "S": S}


_add("pe_rectifier_line_current_pf",
     name="Rectifier line current: THD, displacement and power factor", topic="rectifiers / power quality",
     purpose="The RMS and fundamental line current, current THD, displacement power factor, power factor, real power and apparent power of a single-phase or three-phase (six-pulse) bridge rectifier with a constant dc current at a firing angle (0 for diodes).",
     inputs=[_choice("circuit", _PH2, "single-phase or three-phase bridge"), _I("Id", "constant dc output current"),
             _V("Vs", "RMS source voltage (line-to-line for three-phase)"), _ANG("alpha", "firing angle")],
     outputs=[_o("Is_rms", "current", "RMS line current"), _o("Is1_rms", "current", "RMS fundamental line current"), _o("THD", "number", "current THD"),
              _o("DPF", "number", "displacement power factor cos(alpha)"), _o("PF", "number", "power factor"), _o("P", "power", "real power"),
              _o("S", "apparent_power", "apparent power")],
     primary="PF", conditions=["constant (ripple-free) dc current, no commutation overlap"],
     method="single-phase: Is = Id, Is1 = 2 sqrt(2) Id/pi (PF = 0.9 cos a); three-phase: Is = sqrt(2/3) Id, Is1 = sqrt(6) Id/pi (PF = 0.955 cos a)",
     latex=r"PF = \frac{I_{s1}}{I_s}\cos\alpha,\quad \frac{I_{s1}}{I_s} = \frac{2\sqrt2}{\pi}\ (1\phi),\ \frac{3}{\pi}\ (3\phi)",
     run=line_current_pf, reference=_ref_line_current_pf,
     wrong=lambda circuit, Id, Vs, alpha: (lambda r: dict(r, PF=_r(math.cos(alpha)), THD=0.0))(line_current_pf(circuit, Id, Vs, alpha)),
     rand=lambda rng: {"circuit": rng.choice(_PH2), "Id": _sig(_lu(rng, 1.0, 500.0), 3), "Vs": _sig(rng.uniform(100.0, 690.0), 3), "alpha": math.radians(round(rng.uniform(0.0, 80.0), 1))},
     phrases=("Rectifier line current power factor", "Rectifier power factor and THD"),
     examples=[_ex({"circuit": "single-phase", "Id": 10.0, "Vs": 230.0, "alpha": 0.0}), _ex({"circuit": "three-phase", "Id": 100.0, "Vs": 400.0, "alpha": math.radians(30.0)})],
     level="intermediate")


def line_current_harmonics(circuit, Id, h):
    _pos("Id", Id)
    _need(circuit in _PH2, "circuit: single-phase or three-phase")
    _need(isinstance(h, int) and 1 <= h <= 49, "the harmonic order h must be a whole number from 1 to 49")

    def amp(k):
        if circuit == "single-phase":
            return 2.0 * SQ2 * Id / (k * math.pi) if k % 2 == 1 else 0.0
        return math.sqrt(6.0) * Id / (k * math.pi) if (k % 6 == 1 or k % 6 == 5) else 0.0
    I1 = amp(1)
    s = 0.0
    for k in range(2, h + 1):
        s += amp(k) ** 2
    return _out(Ih=amp(h), Ih_over_I1=amp(h) / I1, THD_to_h=math.sqrt(s) / I1, I1=I1)


def _ref_line_current_harmonics(circuit, Id, h):
    if circuit == "single-phase":
        cur, pts = (lambda th: Id if th < math.pi else -Id), [math.pi]
    else:                                     # diode bridge: phase a's current +Id from 30 to 150 deg, -Id from 210 to 330
        cur = lambda th: Id if math.pi / 6 <= th < 5 * math.pi / 6 else (-Id if 7 * math.pi / 6 <= th < 11 * math.pi / 6 else 0.0)  # noqa: E731
        pts = [k * math.pi / 6 for k in (1, 5, 7, 11)]
    edges = [0.0] + pts + [TWO_PI]

    def harm(k):
        a = sum(_quad(lambda th: cur(th) * math.cos(k * th), edges[i], edges[i + 1]) for i in range(len(edges) - 1)) / math.pi
        b = sum(_quad(lambda th: cur(th) * math.sin(k * th), edges[i], edges[i + 1]) for i in range(len(edges) - 1)) / math.pi
        v = math.hypot(a, b) / SQ2
        return 0.0 if v <= 1e-12 * Id else v
    hs = [harm(k) for k in range(1, h + 1)]
    return {"Ih": hs[-1], "Ih_over_I1": hs[-1] / hs[0], "THD_to_h": math.sqrt(sum(x * x for x in hs[1:])) / hs[0], "I1": hs[0]}


_add("pe_rectifier_line_harmonics",
     name="Rectifier line-current harmonic spectrum", topic="rectifiers / power quality",
     purpose="The RMS value of harmonic h of the line current of a single-phase or six-pulse three-phase bridge with a constant dc current, its ratio to the fundamental (1/h for characteristic harmonics), and the THD of harmonics up to h.",
     inputs=[_choice("circuit", _PH2, "single-phase or three-phase bridge"), _I("Id", "constant dc output current"), _intin("h", "harmonic order", 1, 49)],
     outputs=[_o("Ih", "current", "RMS current of harmonic h"), _o("Ih_over_I1", "number", "ratio to the fundamental"),
              _o("THD_to_h", "number", "THD counting harmonics up to order h"), _o("I1", "current", "RMS fundamental current")],
     primary="Ih", conditions=["1 <= h <= 49", "constant dc current, no overlap"],
     method="single-phase: odd h, Ih = 2 sqrt(2) Id/(h pi); three-phase: h = 6k +- 1, Ih = sqrt(6) Id/(h pi); others 0",
     latex=r"I_h = \frac{I_1}{h},\quad h = 6k\pm1\ (3\phi),\ h\ \text{odd}\ (1\phi)",
     run=line_current_harmonics, reference=_ref_line_current_harmonics,
     wrong=lambda circuit, Id, h: (lambda r: dict(r, Ih=_r(r["I1"] / (h * h)), Ih_over_I1=_r(1.0 / (h * h))))(line_current_harmonics(circuit, Id, h)),
     rand=lambda rng: (lambda c: {"circuit": c, "Id": _sig(_lu(rng, 1.0, 500.0), 3),
                                  "h": rng.choice([3, 5, 7, 9, 11, 13, 15, 17, 19]) if c == "single-phase" else rng.choice([5, 7, 11, 13, 17, 19, 23, 25, 29, 31])})(rng.choice(_PH2)),
     phrases=("Rectifier line harmonics", "Rectifier current harmonic"),
     examples=[_ex({"circuit": "three-phase", "Id": 100.0, "h": 5}), _ex({"circuit": "single-phase", "Id": 20.0, "h": 7})],
     level="advanced")


def half_wave_rl(Vs, f, R, L):
    _pos("Vs, f, R and L", Vs, f, R, L)
    Vm = SQ2 * Vs
    w = TWO_PI * f
    th = math.atan2(w * L, R)
    wt = w * L / R
    g = lambda b: math.sin(b - th) + math.sin(th) * math.exp(-b / wt)    # noqa: E731
    lo, hi = math.pi, TWO_PI
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi:
            break
        if g(mid) > 0:
            lo = mid
        else:
            hi = mid
    beta = 0.5 * (lo + hi)
    Vavg = Vm * (1.0 - math.cos(beta)) / TWO_PI
    return _out(beta_deg=math.degrees(beta), theta_deg=math.degrees(th), Z=math.hypot(R, w * L), V_avg=Vavg, I_avg=Vavg / R)


def _ref_half_wave_rl(Vs, f, R, L):
    from scipy.integrate import solve_ivp
    Vm = SQ2 * Vs
    w = TWO_PI * f

    def rhs(t, y):                     # L di/dt = vs - R i while the diode conducts; q accumulates the charge
        return [(Vm * math.sin(w * t) - R * y[0]) / L, y[0]]
    ev = lambda t, y: y[0] if t > 0.25 / f else 1.0         # noqa: E731
    ev.terminal, ev.direction = True, -1
    sol = solve_ivp(rhs, (0.0, 1.0 / f), [0.0, 0.0], method="DOP853", rtol=1e-13, atol=1e-16 * Vm / R + 1e-300, events=ev, dense_output=True)
    tb = float(sol.t_events[0][0])
    q = float(sol.y_events[0][0][1])
    T = 1.0 / f
    Vavg = _quad(lambda t: Vm * math.sin(w * t), 0.0, tb) / T
    return {"beta_deg": math.degrees(w * tb), "theta_deg": math.degrees(math.atan2(w * L, R)), "Z": abs(complex(R, w * L)), "V_avg": Vavg, "I_avg": q / T}


_add("pe_half_wave_rl_rectifier",
     name="Half-wave diode rectifier with an RL load", topic="rectifiers / RL load",
     purpose="The extinction angle (where the load current returns to zero), the load impedance angle and magnitude, and the average output voltage and current of a half-wave diode rectifier feeding a series RL load.",
     inputs=[_V("Vs", "RMS source voltage"), _F("f", "line frequency"), _R("R", "load resistance"), _phys("L", "inductance", "load inductance", show=_MH)],
     outputs=[_o("beta_deg", "number", "extinction angle in degrees"), _o("theta_deg", "number", "load impedance angle atan(wL/R) in degrees"),
              _o("Z", "resistance", "load impedance magnitude"), _o("V_avg", "voltage", "average output voltage"), _o("I_avg", "current", "average load current")],
     primary="beta_deg", conditions=["ideal diode, no freewheeling diode"],
     method="sin(beta - theta) + sin(theta) exp(-beta/(wL/R)) = 0 (bisection on pi < beta < 2 pi); Vavg = Vm (1 - cos beta)/(2 pi); Iavg = Vavg/R",
     latex=r"\sin(\beta-\theta)+\sin\theta\,e^{-\beta/\omega\tau} = 0,\quad V_{avg} = \frac{V_m(1-\cos\beta)}{2\pi}",
     run=half_wave_rl, reference=_ref_half_wave_rl,
     wrong=lambda Vs, f, R, L: (lambda r: dict(r, beta_deg=180.0 + r["theta_deg"], V_avg=_r(SQ2 * Vs * (1 - math.cos(math.pi + math.radians(r["theta_deg"]))) / TWO_PI),
                                               I_avg=_r(SQ2 * Vs * (1 - math.cos(math.pi + math.radians(r["theta_deg"]))) / TWO_PI / R)))(half_wave_rl(Vs, f, R, L)),
     rand=lambda rng: (lambda f, R: {"Vs": _sig(rng.uniform(50.0, 400.0), 3), "f": f, "R": R, "L": _sig(R * _lu(rng, 0.1, 10.0) / (TWO_PI * f), 3)})(rng.choice([50.0, 60.0]), _sig(_lu(rng, 1.0, 100.0), 3)),
     phrases=("Half-wave rectifier with RL load", "Half-wave RL rectifier"),
     examples=[_ex({"Vs": 120.0, "f": 60.0, "R": 100.0, "L": 0.1}), _ex({"Vs": 230.0, "f": 50.0, "R": 10.0, "L": 0.05})],
     level="advanced")


_HF = ("half-wave", "full-wave")


def capacitor_filter_exact(circuit, Vp, f, R, C):
    _pos("Vp, f, R and C", Vp, f, R, C)
    _need(circuit in _HF, "circuit: half-wave or full-wave")
    w = TWO_PI * f
    wrc = w * R * C
    P = TWO_PI if circuit == "half-wave" else math.pi
    th = math.pi - math.atan(wrc)
    sth = math.sin(th)
    g = lambda a: math.sin(a) - sth * math.exp(-(P + a - th) / wrc)    # noqa: E731
    lo, hi = 0.0, math.pi / 2.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi:
            break
        if g(mid) < 0:
            lo = mid
        else:
            hi = mid
    a = 0.5 * (lo + hi)
    span = P + a - th
    Vavg = (Vp * (math.cos(a) - math.cos(th)) + Vp * sth * wrc * (1.0 - math.exp(-span / wrc))) / P
    return _out(theta_off_deg=math.degrees(th), alpha_deg=math.degrees(a), dV=Vp * (1.0 - math.sin(a)), V_avg=Vavg,
                I_D_peak=Vp * (w * C * math.cos(a) + math.sin(a) / R), conduction_deg=math.degrees(th - a))


def _ref_capacitor_filter_exact(circuit, Vp, f, R, C):
    import sympy as sp
    from scipy.integrate import solve_ivp
    w = TWO_PI * f
    P = TWO_PI if circuit == "half-wave" else math.pi
    x = sp.Symbol("x")
    vs = Vp * sp.sin(x)
    i_d = sp.lambdify(x, C * w * sp.diff(vs, x) + vs / R)        # diode current while it conducts (C and R in parallel)
    th = _brentq(lambda t: float(i_d(t)), math.pi / 2, math.pi)  # the diode stops when that current reaches zero
    v0 = Vp * math.sin(th)
    # simulate the capacitor discharging into R (in angle units) until the rectified source catches it again
    src = (lambda a: abs(Vp * math.sin(a))) if circuit == "full-wave" else (lambda a: Vp * math.sin(a))
    ev = lambda a, y: y[0] - src(a) if a > th + 1e-9 else 1.0       # noqa: E731
    ev.terminal, ev.direction = True, -1
    sol = solve_ivp(lambda a, y: [-y[0] / (w * R * C), y[0]], (th, th + P + math.pi / 2), [v0, 0.0], method="DOP853",
                    rtol=1e-13, atol=1e-16 * Vp, events=ev, max_step=0.01)
    a_end = float(sol.t_events[0][0])
    alpha = a_end - P
    area = float(sol.y_events[0][0][1]) + _quad(lambda a: Vp * math.sin(a), alpha, th)
    return {"theta_off_deg": math.degrees(th), "alpha_deg": math.degrees(alpha), "dV": Vp - Vp * math.sin(alpha), "V_avg": area / P,
            "I_D_peak": float(i_d(alpha)), "conduction_deg": math.degrees(th - alpha)}


_add("pe_capacitor_filter_exact",
     name="Capacitor-input rectifier filter: exact ripple and peak diode current", topic="rectifiers / capacitor filter",
     purpose="The exact steady state of a half-wave or full-wave rectifier with a smoothing capacitor and resistive load: diode turn-off angle, recharge angle, peak-to-peak ripple, average output voltage, peak diode current and conduction angle (no small-ripple approximation).",
     inputs=[_choice("circuit", _HF, "half-wave or full-wave"), _V("Vp", "peak source voltage (after any diode drop)"), _F("f", "line frequency"),
             _R("R", "load resistance"), _C("C", "filter capacitance")],
     outputs=[_o("theta_off_deg", "number", "angle at which the diode turns off, degrees"), _o("alpha_deg", "number", "angle at which the diode turns on again, degrees"),
              _o("dV", "voltage", "peak-to-peak ripple"), _o("V_avg", "voltage", "average output voltage"), _o("I_D_peak", "current", "peak diode current"),
              _o("conduction_deg", "number", "diode conduction angle, degrees")],
     primary="dV", conditions=["ideal diode and source, steady state"],
     method="theta = pi - atan(w R C); sin(alpha) = sin(theta) exp(-(P + alpha - theta)/(w R C)) (bisection), P = 2 pi or pi; dV = Vp (1 - sin alpha); I_D,peak = Vp (w C cos alpha + sin alpha/R)",
     latex=r"\theta = \pi - \tan^{-1}(\omega RC),\ \sin\alpha = \sin\theta\,e^{-(P+\alpha-\theta)/\omega RC},\ I_{D,pk} = V_p\left(\omega C\cos\alpha+\frac{\sin\alpha}{R}\right)",
     run=capacitor_filter_exact, reference=_ref_capacitor_filter_exact,
     wrong=lambda circuit, Vp, f, R, C: (lambda r: dict(r, dV=_r(Vp / ((1 if circuit == "half-wave" else 2) * f * R * C))))(capacitor_filter_exact(circuit, Vp, f, R, C)),
     rand=lambda rng: (lambda f, R: {"circuit": rng.choice(_HF), "Vp": _sig(rng.uniform(5.0, 400.0), 3), "f": f, "R": R, "C": _sig(_lu(rng, 2.0, 100.0) / (TWO_PI * f * R), 3)})(rng.choice([50.0, 60.0]), _sig(_lu(rng, 10.0, 5000.0), 3)),
     phrases=("Exact capacitor filter", "Rectifier capacitor filter exact"),
     examples=[_ex({"circuit": "half-wave", "Vp": 169.7, "f": 60.0, "R": 500.0, "C": 100e-6}), _ex({"circuit": "full-wave", "Vp": 15.0, "f": 50.0, "R": 100.0, "C": 470e-6})],
     level="advanced")


def single_overlap(Vs, f, Ls, Id, alpha):
    _pos("Vs, f, Ls and Id", Vs, f, Ls, Id)
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Vm = SQ2 * Vs
    w = TWO_PI * f
    c = math.cos(alpha) - 2.0 * w * Ls * Id / Vm
    _need(c >= -1.0, "commutation cannot finish within the half-cycle at this current (commutation failure)")
    u = math.acos(c) - alpha
    dV = 2.0 * w * Ls * Id / math.pi
    return _out(u_deg=math.degrees(u), Vdc=2.0 * Vm * math.cos(alpha) / math.pi - dV, dV=dV)


def _ref_single_overlap(Vs, f, Ls, Id, alpha):
    Vm = SQ2 * Vs
    w = TWO_PI * f
    # during overlap the source is short-circuited through Ls: its volt-area reverses the current 2 Id
    area = lambda u: _quad(lambda th: Vm * math.sin(th), alpha, alpha + u) - 2.0 * w * Ls * Id   # noqa: E731
    u = _brentq(area, 0.0, math.pi - alpha)
    Vdc = _quad(lambda th: Vm * math.sin(th), alpha + u, alpha + math.pi) / math.pi             # output 0 while overlapping
    return {"u_deg": math.degrees(u), "Vdc": Vdc, "dV": 2 * Vm * math.cos(alpha) / math.pi - Vdc}


_add("pe_single_phase_overlap",
     name="Commutation overlap in a single-phase thyristor bridge", topic="rectifiers / commutation",
     purpose="The overlap (commutation) angle and the reduced average output voltage of a single-phase fully controlled bridge with source inductance Ls and a constant dc current.",
     inputs=[_V("Vs", "RMS source voltage"), _F("f", "line frequency"), _phys("Ls", "inductance", "source (commutating) inductance", show=_MH),
             _I("Id", "constant dc current"), _ANG("alpha", "firing angle")],
     outputs=[_o("u_deg", "number", "overlap angle in degrees"), _o("Vdc", "voltage", "average output voltage with overlap"),
              _o("dV", "voltage", "voltage lost to commutation")],
     primary="u_deg", conditions=["commutation finishes within the half-cycle"],
     method="cos(alpha + u) = cos(alpha) - 2 w Ls Id/Vm; Vdc = 2 Vm cos(alpha)/pi - 2 w Ls Id/pi",
     latex=r"\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_s I_d}{V_m},\quad V_{dc} = \frac{2V_m}{\pi}\cos\alpha - \frac{2\omega L_sI_d}{\pi}",
     run=single_overlap, reference=_ref_single_overlap,
     wrong=lambda Vs, f, Ls, Id, alpha: (lambda r: dict(r, dV=_r(TWO_PI * f * Ls * Id / math.pi), Vdc=_r(2 * SQ2 * Vs * math.cos(alpha) / math.pi - TWO_PI * f * Ls * Id / math.pi)))(single_overlap(Vs, f, Ls, Id, alpha)),
     rand=lambda rng: (lambda Vs, f: {"Vs": Vs, "f": f, "Ls": _sig(_lu(rng, 1e-4, 5e-3), 3), "Id": _sig(SQ2 * Vs / (TWO_PI * f * 2e-3) * rng.uniform(0.01, 0.2), 3),
                                      "alpha": math.radians(round(rng.uniform(0.0, 120.0), 1))})(_sig(rng.uniform(100.0, 480.0), 3), rng.choice([50.0, 60.0])),
     phrases=("Single-phase commutation overlap", "Single-phase bridge overlap angle"),
     examples=[_ex({"Vs": 230.0, "f": 50.0, "Ls": 2e-3, "Id": 20.0, "alpha": math.radians(30.0)}), _ex({"Vs": 120.0, "f": 60.0, "Ls": 1e-3, "Id": 15.0, "alpha": 0.0})],
     level="advanced")


def three_overlap(VLL, f, Ls, Id, alpha):
    _pos("VLL, f, Ls and Id", VLL, f, Ls, Id)
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Vm = SQ2 * VLL
    w = TWO_PI * f
    c = math.cos(alpha) - 2.0 * w * Ls * Id / Vm
    _need(c >= -1.0, "commutation cannot finish at this current (commutation failure)")
    u = math.acos(c) - alpha
    dV = 3.0 * w * Ls * Id / math.pi
    return _out(u_deg=math.degrees(u), Vdc=3.0 * Vm * math.cos(alpha) / math.pi - dV, dV=dV)


def _ref_three_overlap(VLL, f, Ls, Id, alpha):
    Vm = SQ2 * VLL
    w = TWO_PI * f
    # the commutating line voltage (zero at the natural commutation point) drives 2 Ls dI = v dt; the output loses
    # half of that voltage while both thyristors conduct; six commutations per period
    u = _brentq(lambda u: _quad(lambda th: Vm * math.sin(th), alpha, alpha + u) - 2.0 * w * Ls * Id, 0.0, math.pi - alpha)
    ideal = _quad(lambda th: Vm * math.sin(th + math.pi / 3), alpha, alpha + math.pi / 3)
    loss = _quad(lambda th: 0.5 * Vm * math.sin(th), alpha, alpha + u)
    Vdc = 3.0 * (ideal - loss) / math.pi
    return {"u_deg": math.degrees(u), "Vdc": Vdc, "dV": 3.0 * loss / math.pi}


_add("pe_three_phase_overlap",
     name="Commutation overlap in a three-phase thyristor bridge", topic="rectifiers / commutation",
     purpose="The overlap angle, the average output voltage and the voltage lost to commutation of a six-pulse thyristor (or diode, alpha = 0) bridge with source inductance Ls per phase and a constant dc current.",
     inputs=[_V("VLL", "RMS line-to-line voltage"), _F("f", "line frequency"), _phys("Ls", "inductance", "source inductance per phase", show=_MH),
             _I("Id", "constant dc current"), _ANG("alpha", "firing angle")],
     outputs=[_o("u_deg", "number", "overlap angle in degrees"), _o("Vdc", "voltage", "average output voltage"), _o("dV", "voltage", "voltage lost to commutation")],
     primary="u_deg", conditions=["commutation finishes before the next one starts"],
     method="cos(alpha + u) = cos(alpha) - 2 w Ls Id/(sqrt(2) VLL); Vdc = 3 sqrt(2) VLL cos(alpha)/pi - 3 w Ls Id/pi",
     latex=r"\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_sI_d}{\sqrt2 V_{LL}},\quad V_{dc} = \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha - \frac{3\omega L_sI_d}{\pi}",
     run=three_overlap, reference=_ref_three_overlap,
     wrong=lambda VLL, f, Ls, Id, alpha: (lambda r: dict(r, dV=_r(6 * TWO_PI * f * Ls * Id / math.pi), Vdc=_r(3 * SQ2 * VLL * math.cos(alpha) / math.pi - 6 * TWO_PI * f * Ls * Id / math.pi)))(three_overlap(VLL, f, Ls, Id, alpha)),
     rand=lambda rng: (lambda VLL, f: {"VLL": VLL, "f": f, "Ls": _sig(_lu(rng, 1e-5, 2e-3), 3), "Id": _sig(SQ2 * VLL / (TWO_PI * f * 1e-3) * rng.uniform(0.005, 0.1), 3),
                                       "alpha": math.radians(round(rng.uniform(0.0, 60.0), 1))})(_sig(rng.uniform(200.0, 690.0), 3), rng.choice([50.0, 60.0])),
     phrases=("Three-phase commutation overlap", "Three-phase bridge overlap angle"),
     examples=[_ex({"VLL": 400.0, "f": 50.0, "Ls": 0.5e-3, "Id": 100.0, "alpha": math.radians(30.0)}), _ex({"VLL": 480.0, "f": 60.0, "Ls": 0.2e-3, "Id": 200.0, "alpha": 0.0})],
     level="advanced")


# ================================================================================================ POWER QUALITY BASICS
def thd_harmonics(V1, Vh):
    _pos("the fundamental V1", V1)
    _need(isinstance(Vh, list) and 1 <= len(Vh) <= 12, "give 1 to 12 harmonic values")
    for v in Vh:
        _nonneg("every harmonic value", v)
    s = 0.0
    for v in Vh:
        s += v * v
    thd = math.sqrt(s) / V1
    Vrms = math.sqrt(V1 * V1 + s)
    return _out(THD=thd, THD_pct=100.0 * thd, Vrms=Vrms, distortion_factor=V1 / Vrms)


def _ref_thd_harmonics(V1, Vh):
    import numpy as np
    N = 4096
    t = np.arange(N) / N
    amps = [V1] + list(Vh)
    # a periodic waveform with these RMS harmonics (orders 1, 2, 3, ... with arbitrary phases), analysed by FFT
    x = sum(SQ2 * a * np.sin(TWO_PI * (k + 1) * t + 0.3 * k) for k, a in enumerate(amps))
    X = np.abs(np.fft.rfft(x)) * 2 / N / SQ2
    X = np.where(X <= 1e-12 * V1, 0.0, X)
    h1 = float(X[1])
    hh = [float(v) for v in X[2:len(amps) + 1]]
    rms = float(np.sqrt(np.mean(x * x)))
    return {"THD": math.sqrt(sum(v * v for v in hh)) / h1, "THD_pct": 100 * math.sqrt(sum(v * v for v in hh)) / h1, "Vrms": rms, "distortion_factor": h1 / rms}


_add("pe_thd_from_harmonics",
     name="Total harmonic distortion from harmonic values", topic="power quality / THD",
     purpose="The total harmonic distortion, RMS value and distortion factor of a waveform from the RMS value of its fundamental and of its harmonics 2, 3, 4, ... in order.",
     inputs=[_numin("V1", "RMS value of the fundamental (any unit, the same as the harmonics)"),
             _vecin("Vh", "RMS values of the harmonics in order of harmonic number, starting with the second", 1, 12)],
     outputs=[_o("THD", "number", "total harmonic distortion"), _o("THD_pct", "number", "THD in percent"), _o("Vrms", "number", "RMS value of the waveform"),
              _o("distortion_factor", "number", "distortion factor")],
     primary="THD", conditions=["V1 > 0, harmonics >= 0"],
     method="THD = sqrt(sum Vh^2)/V1; Vrms = sqrt(V1^2 + sum Vh^2); DF = V1/Vrms = 1/sqrt(1 + THD^2)",
     latex=r"THD = \frac{\sqrt{\sum_{h\ge2} V_h^2}}{V_1},\quad V_{rms} = V_1\sqrt{1+THD^2}",
     run=thd_harmonics, reference=_ref_thd_harmonics,
     wrong=lambda V1, Vh: (lambda s: _out(THD=s / V1, THD_pct=100 * s / V1, Vrms=V1 + s, distortion_factor=V1 / (V1 + s)))(sum(Vh)),
     rand=lambda rng: (lambda V1: {"V1": V1, "Vh": [_sig(V1 * rng.uniform(0.02, 0.3), 3)] + [_sig(V1 * rng.uniform(0.02, 0.3), 3) if rng.random() < 0.8 else 0.0 for _ in range(rng.randint(1, 7))]})(_sig(rng.uniform(1.0, 400.0), 3)),
     phrases=("THD from harmonics", "Total harmonic distortion"),
     examples=[_ex({"V1": 230.0, "Vh": [0.0, 11.5, 0.0, 6.9, 0.0, 4.6]}), _ex({"V1": 10.0, "Vh": [0.0, 3.33, 0.0, 2.0]})],
     level="beginner")


def pf_distortion(THD, phi):
    _nonneg("the THD", THD)
    _need(-math.pi / 2 <= phi <= math.pi / 2, "the displacement angle must lie between -90 and 90 degrees")
    dpf = _snap(math.cos(phi), 1.0)
    df = 1.0 / math.sqrt(1.0 + THD * THD)
    return _out(DPF=dpf, distortion_factor=df, PF=dpf * df)


def _ref_pf_distortion(THD, phi):
    # a sinusoidal voltage and a current whose fundamental lags by phi, plus a third harmonic of RMS THD x I1:
    # PF = P / (Vrms Irms) integrated over one period
    v = lambda t: SQ2 * math.sin(t)                                                       # noqa: E731
    i = lambda t: SQ2 * math.sin(t - phi) + SQ2 * THD * math.sin(3 * t + 0.4)               # noqa: E731
    P = _quad(lambda t: v(t) * i(t), 0, TWO_PI, points=[math.pi]) / TWO_PI
    Ir = math.sqrt(_quad(lambda t: i(t) ** 2, 0, TWO_PI, points=[math.pi]) / TWO_PI)
    i1 = math.sqrt(_quad(lambda t: (SQ2 * math.sin(t - phi)) ** 2, 0, TWO_PI) / TWO_PI)
    P1 = _quad(lambda t: v(t) * SQ2 * math.sin(t - phi), 0, TWO_PI) / TWO_PI
    dpf = P1 / i1
    dpf = 0.0 if abs(dpf) < 1e-12 else dpf
    pf = P / Ir
    return {"DPF": dpf, "distortion_factor": i1 / Ir, "PF": 0.0 if abs(pf) < 1e-12 else pf}


_add("pe_power_factor_distortion",
     name="Power factor with harmonic distortion", topic="power quality / power factor",
     purpose="The displacement power factor, distortion factor and true power factor of a load drawing a distorted current from a sinusoidal voltage, from the current THD and the displacement angle of its fundamental.",
     inputs=[_numin("THD", "current total harmonic distortion (a fraction or %)", pct=True), _ANG("phi", "displacement angle of the fundamental current (lagging positive)")],
     outputs=[_o("DPF", "number", "displacement power factor cos(phi)"), _o("distortion_factor", "number", "distortion factor"),
              _o("PF", "number", "true power factor")],
     primary="PF", conditions=["sinusoidal voltage", "-90 <= phi <= 90 degrees"],
     method="PF = cos(phi) / sqrt(1 + THD^2)",
     latex=r"PF = \frac{\cos\phi_1}{\sqrt{1+THD_i^2}}",
     run=pf_distortion, reference=_ref_pf_distortion,
     wrong=lambda THD, phi: _out(DPF=math.cos(phi), distortion_factor=1 - THD, PF=math.cos(phi) * (1 - THD)),
     rand=lambda rng: {"THD": round(rng.uniform(0.05, 1.2), 3), "phi": math.radians(round(rng.uniform(-60.0, 80.0), 1))},
     phrases=("Power factor with distortion", "True power factor"),
     examples=[_ex({"THD": 0.483, "phi": 0.0}), _ex({"THD": 0.3, "phi": math.radians(30.0)})],
     level="beginner",
     extra_q=[{"q": "Power factor with distortion: THD = 48.3%, phi = 0 deg", "like": 0}])


def pulse_rms(A, D):
    _pos("the amplitude A", A)
    _frac("the duty cycle D", D)
    avg = D * A
    rms = math.sqrt(D) * A
    return _out(avg=avg, rms=rms, ac_rms=A * math.sqrt(D * (1.0 - D)), form_factor=1.0 / math.sqrt(D), ripple_factor=math.sqrt(1.0 / D - 1.0))


def _ref_pulse_rms(A, D):
    avg = _quad(lambda t: A, 0, D)
    ms = _quad(lambda t: A * A, 0, D)
    ac = math.sqrt(_quad(lambda t: (A - avg) ** 2, 0, D) + _quad(lambda t: avg * avg, D, 1))
    return {"avg": avg, "rms": math.sqrt(ms), "ac_rms": ac, "form_factor": math.sqrt(ms) / avg, "ripple_factor": ac / avg}


_add("pe_pulse_waveform_rms",
     name="Average and RMS of a rectangular pulse train", topic="power quality / waveform values",
     purpose="The average, RMS and AC-RMS values, form factor and ripple factor of a unipolar rectangular pulse train of amplitude A and duty cycle D (a PWM switch voltage or current).",
     inputs=[_numin("A", "pulse amplitude (any unit)"), _numin("D", "duty cycle (a fraction or %)", pct=True)],
     outputs=[_o("avg", "number", "average value D A"), _o("rms", "number", "RMS value sqrt(D) A"), _o("ac_rms", "number", "RMS of the AC component"),
              _o("form_factor", "number", "form factor rms/avg"), _o("ripple_factor", "number", "ripple factor ac_rms/avg")],
     primary="rms", conditions=["0 < D < 1"],
     method="avg = D A; rms = sqrt(D) A; ac = A sqrt(D (1 - D)); FF = 1/sqrt(D); RF = sqrt(1/D - 1)",
     latex=r"V_{avg} = DA,\quad V_{rms} = \sqrt{D}\,A,\quad RF = \sqrt{\tfrac1D - 1}",
     run=pulse_rms, reference=_ref_pulse_rms,
     wrong=lambda A, D: _out(avg=D * A, rms=D * A, ac_rms=A * (1 - D), form_factor=1.0, ripple_factor=(1 - D) / D),
     rand=lambda rng: {"A": _sig(_lu(rng, 0.5, 600.0), 3), "D": round(rng.uniform(0.05, 0.95), 3)},
     phrases=("Pulse train RMS", "PWM waveform average and RMS"),
     examples=[_ex({"A": 12.0, "D": 0.25}), _ex({"A": 100.0, "D": 0.6})],
     level="beginner")


# ================================================================================================ INVERTERS AND PWM
_BR2 = ("half-bridge", "full-bridge")


def square_wave_inverter(Vdc, bridge, n):
    _pos("Vdc", Vdc)
    _need(bridge in _BR2, "bridge: half-bridge or full-bridge")
    _need(isinstance(n, int) and 3 <= n <= 99 and n % 2 == 1, "the harmonic order n must be an odd whole number from 3 to 99")
    A = Vdc if bridge == "full-bridge" else Vdc / 2.0
    V1 = 4.0 * A / (math.pi * SQ2)
    return _out(Vrms=A, V1_rms=V1, V1_peak=4.0 * A / math.pi, Vn_rms=V1 / n, THD=math.sqrt(A * A - V1 * V1) / V1)


def _ref_square_wave_inverter(Vdc, bridge, n):
    A = Vdc if bridge == "full-bridge" else Vdc / 2.0
    v = lambda t: A if t < math.pi else -A                                    # noqa: E731
    b = lambda k: (_quad(lambda t: v(t) * math.sin(k * t), 0, math.pi) + _quad(lambda t: v(t) * math.sin(k * t), math.pi, TWO_PI)) / math.pi   # noqa: E731
    rms = math.sqrt((_quad(lambda t: v(t) ** 2, 0, math.pi) + _quad(lambda t: v(t) ** 2, math.pi, TWO_PI)) / TWO_PI)
    b1 = b(1)
    V1 = b1 / SQ2
    return {"Vrms": rms, "V1_rms": V1, "V1_peak": b1, "Vn_rms": abs(b(n)) / SQ2, "THD": math.sqrt(rms * rms - V1 * V1) / V1}


_add("pe_square_wave_inverter",
     name="Square-wave inverter output and harmonics", topic="inverters / square-wave",
     purpose="The RMS output voltage, the fundamental (RMS and peak), the RMS value of an odd harmonic n and the THD of a single-phase half-bridge or full-bridge inverter operated with square-wave (180 degree) switching.",
     inputs=[_V("Vdc", "dc input voltage"), _choice("bridge", _BR2, "half-bridge or full-bridge"), _intin("n", "odd harmonic order", 3, 99)],
     outputs=[_o("Vrms", "voltage", "RMS output voltage"), _o("V1_rms", "voltage", "RMS fundamental"), _o("V1_peak", "voltage", "peak fundamental"),
              _o("Vn_rms", "voltage", "RMS value of harmonic n"), _o("THD", "number", "total harmonic distortion")],
     primary="V1_rms", conditions=["n odd, 3..99", "ideal switching, no dead time"],
     method="A = Vdc (full bridge) or Vdc/2 (half bridge); V1 = 4 A/(pi sqrt 2); Vn = V1/n; THD = sqrt(pi^2/8 - 1)",
     latex=r"V_{n,rms} = \frac{4A}{n\pi\sqrt2},\quad THD = \sqrt{\frac{\pi^2}{8}-1}",
     run=square_wave_inverter, reference=_ref_square_wave_inverter,
     wrong=lambda Vdc, bridge, n: (lambda r: dict(r, V1_rms=r["V1_peak"], Vn_rms=_r(r["V1_peak"] / n)))(square_wave_inverter(Vdc, bridge, n)),
     rand=lambda rng: {"Vdc": _sig(rng.uniform(12.0, 800.0), 3), "bridge": rng.choice(_BR2), "n": rng.choice([3, 5, 7, 9, 11, 13, 15, 21, 25])},
     phrases=("Square-wave inverter", "Square wave inverter harmonics"),
     examples=[_ex({"Vdc": 100.0, "bridge": "full-bridge", "n": 3}), _ex({"Vdc": 48.0, "bridge": "half-bridge", "n": 5})],
     level="beginner")


def quasi_square(Vdc, alpha, n):
    _pos("Vdc", Vdc)
    _need(0 <= alpha < math.pi / 2, "alpha must lie from 0 to 90 degrees")
    _need(isinstance(n, int) and 3 <= n <= 99 and n % 2 == 1, "the harmonic order n must be an odd whole number from 3 to 99")
    V1 = 4.0 * Vdc * math.cos(alpha) / (math.pi * SQ2)
    Vn = _snap(abs(4.0 * Vdc * math.cos(n * alpha) / (n * math.pi * SQ2)), Vdc)
    Vrms = Vdc * math.sqrt(1.0 - 2.0 * alpha / math.pi)
    return _out(V1_rms=V1, Vn_rms=Vn, Vrms=Vrms, THD=math.sqrt(max(Vrms * Vrms - V1 * V1, 0.0)) / V1, alpha_eliminate_n_deg=90.0 / n)


def _ref_quasi_square(Vdc, alpha, n):
    edges = [0.0, alpha, math.pi - alpha, math.pi, math.pi + alpha, TWO_PI - alpha, TWO_PI]
    lv = [0.0, Vdc, 0.0, 0.0, -Vdc, 0.0]
    q = lambda g: sum(_quad(lambda t, k=k: lv[k] * g(t), edges[k], edges[k + 1]) for k in range(6) if lv[k] != 0)   # noqa: E731
    b = lambda k: q(lambda t: math.sin(k * t)) / math.pi                     # noqa: E731
    rms = math.sqrt(sum(_quad(lambda t: lv[k] * lv[k], edges[k], edges[k + 1]) for k in range(6)) / TWO_PI)
    V1 = abs(b(1)) / SQ2
    Vn = abs(b(n)) / SQ2
    Vn = 0.0 if Vn <= 1e-12 * Vdc else Vn
    eta = _brentq(lambda a: math.cos(n * a), 1e-9, math.pi / n - 1e-9)       # the zero interval that removes harmonic n
    return {"V1_rms": V1, "Vn_rms": Vn, "Vrms": rms, "THD": math.sqrt(max(rms * rms - V1 * V1, 0.0)) / V1, "alpha_eliminate_n_deg": math.degrees(eta)}


_add("pe_quasi_square_inverter",
     name="Quasi-square-wave (phase-shifted) full-bridge inverter", topic="inverters / quasi-square-wave",
     purpose="The fundamental, the RMS value of an odd harmonic n, the RMS output and the THD of a full-bridge inverter with a zero-voltage interval alpha at each end of every half-cycle, and the alpha that eliminates harmonic n.",
     inputs=[_V("Vdc", "dc input voltage"), _ANG("alpha", "zero-voltage interval at each end of a half-cycle"), _intin("n", "odd harmonic order", 3, 99)],
     outputs=[_o("V1_rms", "voltage", "RMS fundamental"), _o("Vn_rms", "voltage", "RMS value of harmonic n"), _o("Vrms", "voltage", "RMS output voltage"),
              _o("THD", "number", "total harmonic distortion"), _o("alpha_eliminate_n_deg", "number", "zero interval in degrees that removes harmonic n")],
     primary="V1_rms", conditions=["0 <= alpha < 90 degrees", "n odd, 3..99"],
     method="Vn = 4 Vdc cos(n alpha)/(n pi sqrt 2); Vrms = Vdc sqrt(1 - 2 alpha/pi); harmonic n vanishes at alpha = 90/n degrees",
     latex=r"V_{n,rms} = \frac{4V_{dc}}{n\pi\sqrt2}\cos(n\alpha),\quad V_{rms} = V_{dc}\sqrt{1-\frac{2\alpha}{\pi}}",
     run=quasi_square, reference=_ref_quasi_square,
     wrong=lambda Vdc, alpha, n: (lambda r: dict(r, Vn_rms=_r(abs(4 * Vdc * math.cos(alpha) / (n * math.pi * SQ2)))))(quasi_square(Vdc, alpha, n)),
     rand=lambda rng: {"Vdc": _sig(rng.uniform(12.0, 800.0), 3), "alpha": math.radians(round(rng.uniform(2.0, 60.0), 1)), "n": rng.choice([3, 5, 7, 9, 11, 13])},
     phrases=("Quasi-square-wave inverter", "Quasi square wave inverter"),
     examples=[_ex({"Vdc": 100.0, "alpha": math.radians(30.0), "n": 3}), _ex({"Vdc": 200.0, "alpha": math.radians(18.0), "n": 5})],
     level="intermediate")


def six_step(Vdc, f):
    _pos("Vdc and f", Vdc, f)
    VLL = Vdc * math.sqrt(2.0 / 3.0)
    VLL1 = math.sqrt(6.0) * Vdc / math.pi
    return _out(VLL_rms=VLL, VLL1_rms=VLL1, Vph_rms=SQ2 * Vdc / 3.0, Vph1_rms=SQ2 * Vdc / math.pi,
                THD=math.sqrt(VLL * VLL - VLL1 * VLL1) / VLL1, VLL5_rms=VLL1 / 5.0, VLL7_rms=VLL1 / 7.0, f5=5.0 * f, f7=7.0 * f)


def _ref_six_step(Vdc, f):
    leg = lambda t: Vdc / 2 if (t % TWO_PI) < math.pi else -Vdc / 2           # noqa: E731  each leg 180 deg on
    va, vb, vc = (lambda t: leg(t)), (lambda t: leg(t - TWO_PI / 3)), (lambda t: leg(t + TWO_PI / 3))
    vab = lambda t: va(t) - vb(t)                                              # noqa: E731
    vaN = lambda t: va(t) - (va(t) + vb(t) + vc(t)) / 3                        # noqa: E731
    pts = [k * math.pi / 3 for k in range(7)]
    q = lambda g: sum(_quad(g, pts[i], pts[i + 1]) for i in range(6))         # noqa: E731
    def harm(g, k):
        return math.hypot(q(lambda t: g(t) * math.sin(k * t)), q(lambda t: g(t) * math.cos(k * t))) / math.pi / SQ2
    VLL = math.sqrt(q(lambda t: vab(t) ** 2) / TWO_PI)
    VLL1 = harm(vab, 1)
    return {"VLL_rms": VLL, "VLL1_rms": VLL1, "Vph_rms": math.sqrt(q(lambda t: vaN(t) ** 2) / TWO_PI), "Vph1_rms": harm(vaN, 1),
            "THD": math.sqrt(VLL * VLL - VLL1 * VLL1) / VLL1, "VLL5_rms": harm(vab, 5), "VLL7_rms": harm(vab, 7), "f5": 5 * f, "f7": 7 * f}


_add("pe_six_step_inverter",
     name="Three-phase six-step (square-wave) inverter", topic="inverters / three-phase",
     purpose="The RMS line and phase voltages, their fundamentals, the THD and the 5th and 7th harmonics of a three-phase voltage-source inverter in six-step (180 degree) operation.",
     inputs=[_V("Vdc", "dc-link voltage"), _phys("f", "frequency", "output (fundamental) frequency", show=_HZ)],
     outputs=[_o("VLL_rms", "voltage", "RMS line-to-line voltage"), _o("VLL1_rms", "voltage", "RMS fundamental line voltage"),
              _o("Vph_rms", "voltage", "RMS phase voltage"), _o("Vph1_rms", "voltage", "RMS fundamental phase voltage"),
              _o("THD", "number", "THD of the line voltage"), _o("VLL5_rms", "voltage", "RMS fifth-harmonic line voltage"),
              _o("VLL7_rms", "voltage", "RMS seventh-harmonic line voltage"), _o("f5", "frequency", "fifth-harmonic frequency"), _o("f7", "frequency", "seventh-harmonic frequency")],
     primary="VLL1_rms", conditions=["balanced star load, ideal switching"],
     method="V_LL = sqrt(2/3) Vdc; V_LL1 = sqrt(6) Vdc/pi = 0.78 Vdc; harmonics 6k +- 1 at V_LL1/h",
     latex=r"V_{LL1} = \frac{\sqrt6}{\pi}V_{dc},\quad V_{LL} = \sqrt{\tfrac23}V_{dc},\quad V_{LL,h} = \frac{V_{LL1}}{h}",
     run=six_step, reference=_ref_six_step,
     wrong=lambda Vdc, f: (lambda r: dict(r, VLL1_rms=_r(4 * Vdc / (math.pi * SQ2)), VLL5_rms=_r(4 * Vdc / (5 * math.pi * SQ2))))(six_step(Vdc, f)),
     rand=lambda rng: {"Vdc": _sig(rng.uniform(24.0, 1000.0), 3), "f": _sig(rng.uniform(5.0, 400.0), 3)},
     phrases=("Six-step inverter", "Three-phase square-wave inverter"),
     examples=[_ex({"Vdc": 600.0, "f": 50.0}), _ex({"Vdc": 48.0, "f": 200.0})],
     level="intermediate")


def spwm(Vdc, ma, mf, f1):
    _pos("Vdc and f1", Vdc, f1)
    _need(0 < ma <= 1, "the modulation index ma must lie in (0, 1] (linear modulation)")
    _need(isinstance(mf, int) and 15 <= mf <= 201 and mf % 2 == 1, "the frequency ratio mf must be an odd whole number from 15 to 201")
    pk = ma * Vdc / 2.0
    return _out(V1_leg_peak=pk, V1_leg_rms=pk / SQ2, V1_LL_rms=SQ3 * pk / SQ2, V1_fullbridge_rms=2.0 * pk / SQ2, f_carrier=mf * f1)


def _ref_spwm(Vdc, ma, mf, f1):
    # naturally sampled bipolar PWM of one leg: +Vdc/2 while ma sin(theta) exceeds the triangular carrier; every
    # crossing found by brentq, then the fundamental Fourier coefficient integrated exactly over each pulse
    Tc = TWO_PI / mf

    def tri(th):                       # carrier between -1 and 1, starting at -1 (valley) at theta = 0
        x = (th % Tc) / Tc
        return -1 + 4 * x if x < 0.5 else 3 - 4 * x
    a1 = b1 = 0.0
    for k in range(mf):
        t0 = k * Tc
        g = lambda th: ma * math.sin(th) - tri(th)      # noqa: E731
        c1 = _brentq(g, t0, t0 + Tc / 2)                # carrier rising: the output turns from + to -
        c2 = _brentq(g, t0 + Tc / 2, t0 + Tc)           # carrier falling: back to +
        for a, b, lvl in ((t0, c1, 1.0), (c1, c2, -1.0), (c2, t0 + Tc, 1.0)):
            b1 += lvl * (Vdc / 2) * (math.cos(a) - math.cos(b))
            a1 += lvl * (Vdc / 2) * (math.sin(b) - math.sin(a))
    pk = math.hypot(a1, b1) / math.pi
    return {"V1_leg_peak": pk, "V1_leg_rms": pk / SQ2, "V1_LL_rms": SQ3 * pk / SQ2, "V1_fullbridge_rms": 2 * pk / SQ2, "f_carrier": mf * f1}


_add("pe_spwm_fundamental",
     name="Sinusoidal PWM: fundamental output voltage", topic="inverters / sinusoidal PWM",
     purpose="The fundamental output voltage of sinusoidal (sine-triangle) PWM in the linear range: the peak and RMS phase-leg voltage, the RMS line voltage of a three-phase inverter, the RMS output of a bipolar full bridge, and the carrier frequency.",
     inputs=[_V("Vdc", "dc-link voltage"), _numin("ma", "amplitude modulation index"), _intin("mf", "frequency modulation ratio (odd whole number)", 15, 201),
             _phys("f1", "frequency", "fundamental output frequency", show=_HZ)],
     outputs=[_o("V1_leg_peak", "voltage", "peak fundamental of a leg against the dc midpoint"), _o("V1_leg_rms", "voltage", "RMS fundamental of a leg"),
              _o("V1_LL_rms", "voltage", "RMS fundamental line voltage of a three-phase inverter"), _o("V1_fullbridge_rms", "voltage", "RMS fundamental of a bipolar full bridge"),
              _o("f_carrier", "frequency", "carrier (switching) frequency")],
     primary="V1_leg_peak", conditions=["0 < ma <= 1", "mf odd, 15..201 (natural sampling)"],
     method="V1,leg = ma Vdc/2 (peak); V1,LL = sqrt(3) ma Vdc/(2 sqrt 2); bipolar full bridge V1 = ma Vdc/sqrt 2; fc = mf f1",
     latex=r"\hat V_{1,leg} = m_a\frac{V_{dc}}{2},\quad V_{LL1,rms} = \frac{\sqrt3}{2\sqrt2}m_aV_{dc},\quad f_c = m_f f_1",
     run=spwm, reference=_ref_spwm,
     wrong=lambda Vdc, ma, mf, f1: (lambda r: dict(r, V1_leg_peak=_r(ma * Vdc), V1_leg_rms=_r(ma * Vdc / SQ2)))(spwm(Vdc, ma, mf, f1)),
     rand=lambda rng: {"Vdc": _sig(rng.uniform(24.0, 1000.0), 3), "ma": round(rng.uniform(0.1, 1.0), 3), "mf": rng.choice([15, 21, 27, 33, 39, 45, 51]), "f1": rng.choice([50.0, 60.0])},
     phrases=("Sinusoidal PWM fundamental", "SPWM output voltage"),
     examples=[_ex({"Vdc": 600.0, "ma": 0.8, "mf": 21, "f1": 50.0}), _ex({"Vdc": 400.0, "ma": 1.0, "mf": 39, "f1": 60.0})],
     level="beginner")


def space_vector(Vref, theta, Vdc, fs):
    _pos("Vref, Vdc and fs", Vref, Vdc, fs)
    _need(0 <= theta < TWO_PI, "theta must lie from 0 to 360 degrees")
    k = int(math.floor(theta / (math.pi / 3.0)))
    k = min(k, 5)
    tp = theta - k * math.pi / 3.0
    Ts = 1.0 / fs
    c = SQ3 * Vref * Ts / Vdc
    T1 = c * math.sin(math.pi / 3.0 - tp)
    T2 = c * math.sin(tp)
    T0 = Ts - T1 - T2
    _need(T0 >= -1e-12 * Ts, "the reference lies outside the hexagon: overmodulation (T1 + T2 > Ts)")
    return _out(sector=k + 1, T1=T1, T2=_snap(T2, Ts), T0=_snap(max(T0, 0.0), Ts), m=SQ3 * Vref / Vdc)


def _ref_space_vector(Vref, theta, Vdc, fs):
    import numpy as np
    from scipy.linalg import solve
    k = min(int(math.floor(theta / (math.pi / 3))), 5)
    Ts = 1.0 / fs
    # active vectors (2/3) Vdc at k*60 and (k+1)*60 degrees: their time-weighted sum equals Vref Ts (volt-seconds)
    v1 = 2 * Vdc / 3 * np.array([math.cos(k * math.pi / 3), math.sin(k * math.pi / 3)])
    v2 = 2 * Vdc / 3 * np.array([math.cos((k + 1) * math.pi / 3), math.sin((k + 1) * math.pi / 3)])
    t = solve(np.column_stack([v1, v2]), Vref * Ts * np.array([math.cos(theta), math.sin(theta)]))
    T1, T2 = float(t[0]), float(t[1])
    T2 = 0.0 if abs(T2) <= 1e-12 * Ts else T2
    T0 = Ts - T1 - T2
    return {"sector": k + 1, "T1": T1, "T2": T2, "T0": 0.0 if abs(T0) <= 1e-12 * Ts else T0, "m": Vref / (Vdc / SQ3)}


_add("pe_space_vector_times",
     name="Space-vector modulation dwell times", topic="inverters / space-vector PWM",
     purpose="The sector and the dwell times T1, T2 of the two adjacent active vectors and T0 of the zero vectors that synthesise a reference space vector (peak phase voltage Vref at angle theta) in one switching period of a two-level three-phase inverter.",
     inputs=[_V("Vref", "reference vector magnitude (peak phase voltage)"), _ANG("theta", "reference vector angle"),
             _V("Vdc", "dc-link voltage"), _F("fs", "switching (sampling) frequency")],
     outputs=[_o("sector", "number", "sector number"), _o("T1", "time", "dwell time of the first active vector"), _o("T2", "time", "dwell time of the second active vector"),
              _o("T0", "time", "total zero-vector time"), _o("m", "number", "modulation index")],
     primary="T1", conditions=["inside the hexagon (T1 + T2 <= Ts)"],
     method="T1 = sqrt(3) Ts Vref/Vdc sin(60 deg - theta'), T2 = sqrt(3) Ts Vref/Vdc sin(theta'), T0 = Ts - T1 - T2, theta' = angle within the sector",
     latex=r"T_1 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\!\left(\tfrac{\pi}{3}-\theta'\right),\quad T_2 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\theta',\quad T_0 = T_s - T_1 - T_2",
     run=space_vector, reference=_ref_space_vector,
     wrong=lambda Vref, theta, Vdc, fs: (lambda r: dict(r, T1=_r(r["T1"] / SQ3), T2=_r(r["T2"] / SQ3), T0=_r(1 / fs - (r["T1"] + r["T2"]) / SQ3)))(space_vector(Vref, theta, Vdc, fs)),
     rand=lambda rng: (lambda Vdc: {"Vref": _sig(Vdc / SQ3 * rng.uniform(0.05, 0.85), 3), "theta": math.radians(round(rng.uniform(0.5, 359.5), 1)), "Vdc": Vdc,
                                    "fs": _sig(_lu(rng, 2e3, 5e4), 3)})(_sig(rng.uniform(100.0, 800.0), 3)),
     phrases=("Space vector dwell times", "Space-vector modulation times"),
     examples=[_ex({"Vref": 200.0, "theta": math.radians(25.0), "Vdc": 600.0, "fs": 10e3}), _ex({"Vref": 100.0, "theta": math.radians(200.0), "Vdc": 400.0, "fs": 5e3})],
     level="advanced")


def dead_time_error(Vdc, td, fs, V1):
    _pos("Vdc, td, fs and V1", Vdc, td, fs, V1)
    _need(td * fs < 0.5, "the dead time must be shorter than half the switching period")
    dV = td * fs * Vdc
    dV1 = 4.0 * dV / math.pi
    return _out(dV_avg=dV, dV1_peak=dV1, dV1_pct=100.0 * dV1 / V1)


def _ref_dead_time_error(Vdc, td, fs, V1):
    # in each switching period the leg voltage is wrong by Vdc for td (its sign set by the current): averaged per
    # period, an error of dV; over the fundamental it is a square wave whose fundamental is integrated here
    dV = _quad(lambda t: Vdc, 0.0, td) * fs
    b1 = (_quad(lambda t: dV * math.sin(t), 0, math.pi) - _quad(lambda t: dV * math.sin(t), math.pi, TWO_PI)) / math.pi
    return {"dV_avg": dV, "dV1_peak": b1, "dV1_pct": 100 * b1 / V1}


_add("pe_dead_time_error",
     name="Inverter dead-time voltage error", topic="inverters / dead time",
     purpose="The average voltage error per switching period caused by the dead time of an inverter leg, the peak of its fundamental (the error is a square wave in phase with the current) and that as a percentage of the wanted fundamental.",
     inputs=[_V("Vdc", "dc-link voltage"), _phys("td", "time", "dead time", show=_NS), _F("fs", "switching frequency"), _V("V1", "wanted fundamental peak voltage")],
     outputs=[_o("dV_avg", "voltage", "average voltage error td fs Vdc"), _o("dV1_peak", "voltage", "peak fundamental of the error"),
              _o("dV1_pct", "number", "error in percent of the wanted fundamental")],
     primary="dV_avg", conditions=["td fs < 0.5", "the current does not change sign within a switching period"],
     method="dV = td fs Vdc; its fundamental 4 dV/pi",
     latex=r"\Delta V = t_d f_s V_{dc},\quad \Delta\hat V_1 = \frac{4}{\pi}t_d f_s V_{dc}",
     run=dead_time_error, reference=_ref_dead_time_error,
     wrong=lambda Vdc, td, fs, V1: (lambda dV: _out(dV_avg=dV, dV1_peak=4 * dV / math.pi, dV1_pct=400 * dV / math.pi / V1))(2 * td * fs * Vdc),
     rand=lambda rng: (lambda Vdc: {"Vdc": Vdc, "td": _sig(_lu(rng, 1e-7, 5e-6), 3), "fs": _sig(_lu(rng, 2e3, 5e4), 3), "V1": _sig(Vdc / 2 * rng.uniform(0.2, 1.0), 3)})(_sig(rng.uniform(48.0, 1000.0), 3)),
     phrases=("Dead-time voltage error", "Inverter dead time error"),
     examples=[_ex({"Vdc": 600.0, "td": 2e-6, "fs": 10e3, "V1": 250.0}), _ex({"Vdc": 48.0, "td": 200e-9, "fs": 20e3, "V1": 20.0})],
     level="intermediate",
     weak="the same per-period definition integrated with scipy quad, fundamental by quadrature")


def inverter_rl(Vdc, f, R, L):
    _pos("Vdc, f, R and L", Vdc, f, R, L)
    tau = L / R
    h = 0.5 / f
    e = math.exp(-h / tau)
    a = Vdc / R
    Imax = a * (1.0 - e) / (1.0 + e)
    b = -Imax - a
    ms = (a * a * h + 2.0 * a * b * tau * (1.0 - e) + b * b * tau / 2.0 * (1.0 - e * e)) / h
    Irms = math.sqrt(ms)
    P = ms * R
    return _out(I_max=Imax, I_rms=Irms, P=P, Is_avg=P / Vdc)


def _ref_inverter_rl(Vdc, f, R, L):
    import numpy as np
    from scipy.linalg import expm
    h = 0.5 / f
    # augmented state [i, 1]: di/dt = (v - R i)/L, propagated exactly by the matrix exponential
    M = lambda v, t: expm(np.array([[-R / L, v / L], [0.0, 0.0]]) * t)          # noqa: E731
    Ph = M(Vdc, h)
    # half-wave symmetry i(h) = -i(0):  Ph[0,0] i0 + Ph[0,1] = -i0
    i0 = -Ph[0, 1] / (1.0 + Ph[0, 0])
    cur = lambda t: float((M(Vdc, t) @ np.array([i0, 1.0]))[0])                   # noqa: E731
    ms = _quad(lambda t: cur(t) ** 2, 0.0, h) / h
    return {"I_max": float(-i0), "I_rms": math.sqrt(ms), "P": ms * R, "Is_avg": _quad(cur, 0.0, h) / h}


_add("pe_inverter_rl_load_current",
     name="Square-wave inverter with an RL load: current", topic="inverters / load current",
     purpose="The steady-state peak current, RMS current, load power and average dc source current of a full-bridge square-wave inverter driving a series RL load (exact exponential current, not a harmonic approximation).",
     inputs=[_V("Vdc", "dc input voltage"), _phys("f", "frequency", "output frequency", show=_HZ), _R("R", "load resistance"), _phys("L", "inductance", "load inductance", show=_MH)],
     outputs=[_o("I_max", "current", "peak load current"), _o("I_rms", "current", "RMS load current"), _o("P", "power", "load power"),
              _o("Is_avg", "current", "average dc source current")],
     primary="I_max", conditions=["steady state, ideal switches with antiparallel diodes"],
     method="I_max = (Vdc/R)(1 - e^(-T/2tau))/(1 + e^(-T/2tau)); i(t) = Vdc/R + (-I_max - Vdc/R) e^(-t/tau) over a half-cycle; P = I_rms^2 R",
     latex=r"I_{max} = \frac{V_{dc}}{R}\,\frac{1-e^{-T/2\tau}}{1+e^{-T/2\tau}},\quad \tau = \frac{L}{R}",
     run=inverter_rl, reference=_ref_inverter_rl,
     wrong=lambda Vdc, f, R, L: (lambda r: dict(r, I_max=_r(Vdc / R)))(inverter_rl(Vdc, f, R, L)),
     rand=lambda rng: (lambda f, R: {"Vdc": _sig(rng.uniform(12.0, 600.0), 3), "f": f, "R": R, "L": _sig(R * _lu(rng, 0.05, 20.0) / (TWO_PI * f), 3)})(_sig(rng.uniform(20.0, 1000.0), 3), _sig(_lu(rng, 1.0, 100.0), 3)),
     phrases=("Inverter RL load current", "Square-wave inverter RL load"),
     examples=[_ex({"Vdc": 100.0, "f": 60.0, "R": 10.0, "L": 25e-3}), _ex({"Vdc": 48.0, "f": 400.0, "R": 4.0, "L": 2e-3})],
     level="advanced")


def igbt_losses(Ip, M, phi, Vce0, rce, VF0, rF):
    _pos("Ip", Ip)
    _need(0 < M <= 1, "the modulation index M must lie in (0, 1]")
    _need(-math.pi / 2 <= phi <= math.pi / 2, "the power-factor angle must lie between -90 and 90 degrees")
    _nonneg("Vce0, rce, VF0 and rF", Vce0, rce, VF0, rF)
    c = M * math.cos(phi)
    PT = Vce0 * Ip * (1.0 / TWO_PI + c / 8.0) + rce * Ip * Ip * (1.0 / 8.0 + c / (3.0 * math.pi))
    PD = VF0 * Ip * (1.0 / TWO_PI - c / 8.0) + rF * Ip * Ip * (1.0 / 8.0 - c / (3.0 * math.pi))
    return _out(P_igbt=PT, P_diode=PD, P_switch_pair=PT + PD, P_leg=2.0 * (PT + PD))


def _ref_igbt_losses(Ip, M, phi, Vce0, rce, VF0, rF):
    # over the half-cycle of positive current the upper IGBT conducts for the duty (1 + M sin(th + phi))/2 of each
    # switching period and the lower diode for the rest; average the instantaneous conduction losses
    i = lambda th: Ip * math.sin(th)                                          # noqa: E731
    d = lambda th: 0.5 * (1.0 + M * math.sin(th + phi))                      # noqa: E731
    PT = _quad(lambda th: (Vce0 * i(th) + rce * i(th) ** 2) * d(th), 0, math.pi) / TWO_PI
    PD = _quad(lambda th: (VF0 * i(th) + rF * i(th) ** 2) * (1 - d(th)), 0, math.pi) / TWO_PI
    return {"P_igbt": PT, "P_diode": PD, "P_switch_pair": PT + PD, "P_leg": 2 * (PT + PD)}


_add("pe_igbt_inverter_conduction_losses",
     name="IGBT and diode conduction losses in a sinusoidal PWM inverter", topic="losses / inverter devices",
     purpose="The conduction losses of one IGBT and one freewheeling diode (and of a whole leg) in a sinusoidal PWM inverter, from the peak phase current, modulation index, power-factor angle and the devices' threshold-voltage plus slope-resistance models.",
     inputs=[_I("Ip", "peak phase current"), _numin("M", "modulation index"), _ANG("phi", "power-factor angle (current lagging positive)"),
             _V("Vce0", "IGBT threshold voltage"), _R("rce", "IGBT slope resistance"), _V("VF0", "diode threshold voltage"), _R("rF", "diode slope resistance")],
     outputs=[_o("P_igbt", "power", "conduction loss of one IGBT"), _o("P_diode", "power", "conduction loss of one diode"),
              _o("P_switch_pair", "power", "one IGBT plus one diode"), _o("P_leg", "power", "conduction loss of a leg (two of each)")],
     primary="P_igbt", conditions=["0 < M <= 1, sinusoidal current, switching frequency far above the output frequency"],
     method="P_T = Vce0 Ip (1/(2pi) + M cos(phi)/8) + rce Ip^2 (1/8 + M cos(phi)/(3pi)); P_D = VF0 Ip (1/(2pi) - M cos(phi)/8) + rF Ip^2 (1/8 - M cos(phi)/(3pi))",
     latex=r"P_T = V_{ce0}\hat I\left(\frac{1}{2\pi}+\frac{M\cos\phi}{8}\right) + r_{ce}\hat I^2\left(\frac18+\frac{M\cos\phi}{3\pi}\right)",
     run=igbt_losses, reference=_ref_igbt_losses,
     wrong=lambda Ip, M, phi, Vce0, rce, VF0, rF: (lambda PT, PD: _out(P_igbt=PT, P_diode=PD, P_switch_pair=PT + PD, P_leg=2 * (PT + PD)))(Vce0 * Ip / TWO_PI + rce * Ip * Ip / 8, VF0 * Ip / TWO_PI + rF * Ip * Ip / 8),
     rand=lambda rng: {"Ip": _sig(_lu(rng, 5.0, 600.0), 3), "M": round(rng.uniform(0.3, 1.0), 3), "phi": math.radians(round(rng.uniform(-30.0, 60.0), 1)),
                       "Vce0": _sig(rng.uniform(0.6, 1.2), 3), "rce": _sig(_lu(rng, 1e-3, 5e-2), 3), "VF0": _sig(rng.uniform(0.6, 1.1), 3), "rF": _sig(_lu(rng, 1e-3, 5e-2), 3)},
     phrases=("IGBT inverter conduction losses", "Inverter IGBT and diode losses"),
     examples=[_ex({"Ip": 100.0, "M": 0.9, "phi": math.radians(30.0), "Vce0": 0.9, "rce": 0.01, "VF0": 0.8, "rF": 0.008}),
               _ex({"Ip": 300.0, "M": 0.7, "phi": 0.0, "Vce0": 1.0, "rce": 0.004, "VF0": 0.9, "rF": 0.003})],
     level="advanced")


# ================================================================================================ MOTOR DRIVES
_KM = lambda s, m: _phys(s, "motor_constant", m)        # noqa: E731
_W = lambda s, m: _phys(s, "angular_frequency", m)      # noqa: E731
_RPM = 30.0 / math.pi


def _dc_steady(Va, Ra, K, Ia):
    """The ref helper: steady state of La dIa/dt = Va - Ra Ia - K w, J dw/dt = K Ia - T as a linear solve."""
    import numpy as np
    from scipy.linalg import solve
    # unknowns [w, Ea]: Ea = K w, Va = Ea + Ra Ia
    x = solve(np.array([[K, -1.0], [0.0, 1.0]]), np.array([0.0, Va - Ra * Ia]))
    return float(x[0]), float(x[1])


def dc_motor_chopper(Vs, D, Ra, K, TL):
    _pos("Vs, Ra and K", Vs, Ra, K)
    _frac("the duty cycle D", D)
    _nonneg("the load torque TL", TL)
    Ia = TL / K
    Va = D * Vs
    Ea = Va - Ia * Ra
    _need(Ea > 0, "the armature drop exceeds the applied voltage: the motor stalls at this load")
    w = Ea / K
    Pout = Ea * Ia
    return _out(Ia=Ia, Va=Va, Ea=Ea, omega=w, rpm=w * _RPM, P_out=Pout, efficiency=Pout / (Va * Ia) if Ia > 0 else 1.0)


def _ref_dc_motor_chopper(Vs, D, Ra, K, TL):
    Va = _quad(lambda t: Vs, 0.0, D)                       # average of the chopped voltage over one period
    Ia = TL / K                                             # torque balance K Ia = TL
    w, Ea = _dc_steady(Va, Ra, K, Ia)
    return {"Ia": Ia, "Va": Va, "Ea": Ea, "omega": w, "rpm": w * 60 / TWO_PI, "P_out": TL * w, "efficiency": (TL * w) / (Va * Ia) if Ia > 0 else 1.0}


_add("pe_dc_motor_chopper_drive",
     name="Chopper-fed separately excited DC motor: speed and torque", topic="motor drives / DC motor",
     purpose="The armature current, average armature voltage, back EMF, speed (rad/s and rpm), output power and armature efficiency of a separately excited DC motor fed by a step-down chopper at a duty cycle and carrying a load torque (continuous current, ripple neglected).",
     inputs=[_V("Vs", "dc supply voltage"), _numin("D", "chopper duty cycle (a fraction or %)", pct=True), _R("Ra", "armature resistance"),
             _KM("K", "motor constant K = Ke = Kt"), _phys("TL", "torque", "load torque")],
     outputs=[_o("Ia", "current", "armature current TL/K"), _o("Va", "voltage", "average armature voltage D Vs"), _o("Ea", "voltage", "back EMF"),
              _o("omega", "angular_frequency", "speed"), _o("rpm", "number", "speed in rpm"), _o("P_out", "power", "developed (output) power"),
              _o("efficiency", "number", "armature efficiency Ea/Va")],
     primary="omega", conditions=["continuous armature current", "constant field"],
     method="Va = D Vs; Ia = TL/K; Ea = Va - Ia Ra; w = Ea/K",
     latex=r"V_a = DV_s,\quad I_a = \frac{T_L}{K},\quad \omega = \frac{V_a - I_aR_a}{K}",
     run=dc_motor_chopper, reference=_ref_dc_motor_chopper,
     wrong=lambda Vs, D, Ra, K, TL: (lambda r: dict(r, omega=_r(D * Vs / K), rpm=_r(D * Vs / K * _RPM)))(dc_motor_chopper(Vs, D, Ra, K, TL)),
     rand=lambda rng: (lambda Vs, K, Ra: {"Vs": Vs, "D": round(rng.uniform(0.3, 0.95), 3), "Ra": Ra, "K": K, "TL": _sig(K * Vs * 0.3 / Ra * rng.uniform(0.05, 0.5), 3)})(
         _sig(rng.uniform(24.0, 600.0), 3), _sig(rng.uniform(0.2, 3.0), 3), _sig(_lu(rng, 0.1, 5.0), 3)),
     phrases=("DC motor chopper drive", "Chopper-fed DC motor"),
     examples=[_ex({"Vs": 220.0, "D": 0.6, "Ra": 0.5, "K": 1.2, "TL": 24.0}), _ex({"Vs": 48.0, "D": 0.8, "Ra": 0.2, "K": 0.15, "TL": 1.5})],
     level="beginner",
     weak="the steady state of the motor equations re-solved as a linear system (scipy), with the chopper average by quadrature")


def bldc_six_step(Vdc, D, Rph, Keph, I, p):
    _pos("Vdc, Rph, Keph and I", Vdc, Rph, Keph, I)
    _frac("the duty cycle D", D)
    _need(isinstance(p, int) and 1 <= p <= 50, "the number of pole pairs p must be a whole number from 1 to 50")
    V = D * Vdc
    w = (V - 2.0 * Rph * I) / (2.0 * Keph)
    _need(w > 0, "the winding drop exceeds the applied voltage: the motor stalls at this current")
    fe = p * w / TWO_PI
    return _out(omega=w, rpm=w * _RPM, T=2.0 * Keph * I, omega_no_load=V / (2.0 * Keph), I_stall=V / (2.0 * Rph), f_e=fe, t_commutation=1.0 / (6.0 * fe))


def _ref_bldc_six_step(Vdc, D, Rph, Keph, I, p):
    # trapezoidal phase EMF (flat top Ke w over 120 electrical degrees) and 120-degree block currents of +-I: the
    # electromagnetic power summed over the three phases, averaged over an electrical cycle, divided by speed = torque
    def emf_shape(th):                 # unit trapezoid of phase a
        th = th % TWO_PI
        if th < math.pi / 6:
            return th / (math.pi / 6)
        if th < 5 * math.pi / 6:
            return 1.0
        if th < 7 * math.pi / 6:
            return 1.0 - 2 * (th - 5 * math.pi / 6) / (math.pi / 3)
        if th < 11 * math.pi / 6:
            return -1.0
        return -1.0 + (th - 11 * math.pi / 6) / (math.pi / 6)

    def cur(th):
        th = th % TWO_PI
        return I if math.pi / 6 <= th < 5 * math.pi / 6 else (-I if 7 * math.pi / 6 <= th < 11 * math.pi / 6 else 0.0)
    pts = [k * math.pi / 6 for k in range(13)]
    torque_per_ke = sum(_quad(lambda th: sum(emf_shape(th - k * TWO_PI / 3) * cur(th - k * TWO_PI / 3) for k in range(3)), pts[i], pts[i + 1]) for i in range(12)) / TWO_PI
    T = Keph * torque_per_ke
    V = _quad(lambda t: Vdc, 0.0, D)
    w = _brentq(lambda w: V - 2 * Rph * I - 2 * Keph * w, -1e9, 1e9)       # two phases in series against two flat EMFs
    fe = p * w / TWO_PI
    return {"omega": w, "rpm": w * 60 / TWO_PI, "T": T, "omega_no_load": V / (2 * Keph), "I_stall": V / (2 * Rph), "f_e": fe, "t_commutation": (1 / fe) / 6}


_add("pe_bldc_six_step",
     name="BLDC motor with six-step (120 degree) drive", topic="motor drives / BLDC",
     purpose="The speed, torque, no-load speed, stall current, electrical frequency and commutation interval of a trapezoidal brushless DC motor driven six-step from a PWM-chopped dc bus (two phases conduct in series).",
     inputs=[_V("Vdc", "dc bus voltage"), _numin("D", "PWM duty cycle (a fraction or %)", pct=True), _R("Rph", "phase resistance"),
             _KM("Keph", "phase back-EMF constant (flat top)"), _I("I", "dc link (phase) current"), _intin("p", "pole pairs", 1, 50)],
     outputs=[_o("omega", "angular_frequency", "speed"), _o("rpm", "number", "speed in rpm"), _o("T", "torque", "torque"),
              _o("omega_no_load", "angular_frequency", "no-load speed"), _o("I_stall", "current", "stall current"), _o("f_e", "frequency", "electrical frequency"),
              _o("t_commutation", "time", "time between commutations")],
     primary="omega", conditions=["ideal trapezoidal EMF and block currents", "winding inductance and commutation neglected"],
     method="D Vdc = 2 Rph I + 2 Keph w; T = 2 Keph I; f_e = p w/(2 pi)",
     latex=r"DV_{dc} = 2R_{ph}I + 2K_{e,ph}\omega,\quad T = 2K_{e,ph}I",
     run=bldc_six_step, reference=_ref_bldc_six_step,
     wrong=lambda Vdc, D, Rph, Keph, I, p: (lambda r: dict(r, T=_r(Keph * I), omega=_r((D * Vdc - Rph * I) / Keph)))(bldc_six_step(Vdc, D, Rph, Keph, I, p)),
     rand=lambda rng: (lambda Vdc, Ke, R: {"Vdc": Vdc, "D": round(rng.uniform(0.3, 1.0 - 1e-3), 3), "Rph": R, "Keph": Ke, "I": _sig(Vdc * 0.3 / (2 * R) * rng.uniform(0.02, 0.5), 3), "p": rng.randint(1, 8)})(
         _sig(rng.uniform(12.0, 400.0), 3), _sig(_lu(rng, 0.005, 0.5), 3), _sig(_lu(rng, 0.02, 2.0), 3)),
     phrases=("BLDC six-step drive", "Brushless DC motor six-step"),
     examples=[_ex({"Vdc": 48.0, "D": 0.8, "Rph": 0.1, "Keph": 0.05, "I": 10.0, "p": 4}), _ex({"Vdc": 24.0, "D": 0.5, "Rph": 0.4, "Keph": 0.02, "I": 2.0, "p": 2})],
     level="beginner")


def dc_drive_rectifier(converter, Vs, alpha, Ra, K, Ia):
    _pos("Vs, Ra, K and Ia", Vs, Ra, K, Ia)
    _need(converter in _PH2, "converter: single-phase or three-phase")
    _need(0 <= alpha < math.pi, "the firing angle must lie from 0 to 180 degrees")
    Va = _snap((2.0 if converter == "single-phase" else 3.0) * SQ2 * Vs * math.cos(alpha) / math.pi, Vs)
    Ea = Va - Ia * Ra
    w = Ea / K
    return _out(Va=Va, Ea=Ea, omega=w, rpm=w * _RPM, T=K * Ia, P=Ea * Ia, mode="motoring" if w >= 0 else "reverse (regenerating against an active load)")


def _ref_dc_drive_rectifier(converter, Vs, alpha, Ra, K, Ia):
    Va = (_ref_single_phase_controlled("full-bridge", Vs, alpha) if converter == "single-phase" else _ref_three_phase_controlled(Vs, alpha, Ia))["Vdc"]
    w, Ea = _dc_steady(Va, Ra, K, Ia)
    return {"Va": Va, "Ea": Ea, "omega": w, "rpm": w * 60 / TWO_PI, "T": K * Ia, "P": Ea * Ia, "mode": "motoring" if w >= 0 else "reverse (regenerating against an active load)"}


_add("pe_dc_drive_controlled_rectifier",
     name="Phase-controlled rectifier DC motor drive", topic="motor drives / DC motor",
     purpose="The armature voltage, back EMF, speed, torque and developed power of a separately excited DC motor fed by a single-phase or three-phase fully controlled thyristor bridge at a firing angle, with continuous armature current.",
     inputs=[_choice("converter", _PH2, "single-phase or three-phase full converter"), _V("Vs", "RMS supply voltage (line-to-line for three-phase)"),
             _ANG("alpha", "firing angle"), _R("Ra", "armature resistance"), _KM("K", "motor constant"), _I("Ia", "armature current")],
     outputs=[_o("Va", "voltage", "average armature voltage"), _o("Ea", "voltage", "back EMF"), _o("omega", "angular_frequency", "speed"),
              _o("rpm", "number", "speed in rpm"), _o("T", "torque", "developed torque K Ia"), _o("P", "power", "developed power Ea Ia"),
              _o("mode", "number", "motoring or reverse")],
     primary="omega", conditions=["continuous armature current"],
     method="Va = 2 sqrt(2) Vs cos(alpha)/pi (single-phase) or 3 sqrt(2) VLL cos(alpha)/pi (three-phase); Ea = Va - Ia Ra; w = Ea/K",
     latex=r"V_a = \frac{2\sqrt2V_s}{\pi}\cos\alpha\ (1\phi),\ \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha\ (3\phi),\quad \omega = \frac{V_a - I_aR_a}{K}",
     run=dc_drive_rectifier, reference=_ref_dc_drive_rectifier,
     wrong=lambda converter, Vs, alpha, Ra, K, Ia: dc_drive_rectifier(converter, Vs / SQ2, alpha, Ra, K, Ia),
     rand=lambda rng: {"converter": rng.choice(_PH2), "Vs": _sig(rng.uniform(100.0, 480.0), 3), "alpha": math.radians(round(rng.uniform(0.0, 80.0), 1)),
                       "Ra": _sig(_lu(rng, 0.05, 2.0), 3), "K": _sig(rng.uniform(0.5, 3.0), 3), "Ia": _sig(_lu(rng, 1.0, 100.0), 3)},
     phrases=("Controlled rectifier DC drive", "Thyristor DC motor drive"),
     examples=[_ex({"converter": "single-phase", "Vs": 230.0, "alpha": math.radians(45.0), "Ra": 1.0, "K": 1.0, "Ia": 10.0}),
               _ex({"converter": "three-phase", "Vs": 415.0, "alpha": math.radians(30.0), "Ra": 0.2, "K": 2.0, "Ia": 50.0})],
     level="intermediate")


def regen_chopper(Vs, D, Ra, K, omega):
    _pos("Vs, Ra, K and omega", Vs, Ra, K, omega)
    _frac("the duty cycle D", D)
    Ea = K * omega
    Va = (1.0 - D) * Vs
    Ia = (Ea - Va) / Ra
    _need(Ia > 0, "the back EMF is below the average armature voltage: no regenerative braking at this duty cycle")
    Pm = Ea * Ia
    Pr = Va * Ia
    return _out(Ea=Ea, Va=Va, Ia=Ia, T_brake=K * Ia, P_mech=Pm, P_regen=Pr, efficiency=Pr / Pm)


def _ref_regen_chopper(Vs, D, Ra, K, omega):
    import sympy as sp
    ia = sp.Symbol("Ia")
    Ea = K * omega
    # boost action: the motor (EMF Ea) is the source; while the switch is off for (1 - D) T the armature current flows
    # into the supply Vs; the inductor's volt-seconds balance gives Ea - Ra Ia = (1 - D) Vs
    Ia = _sym_solve([_sq(Ea) - _sq(Ra) * ia - (1 - _sq(D)) * _sq(Vs)], [ia])["Ia"]
    Pr = Vs * _quad(lambda t: Ia, D, 1.0)
    return {"Ea": Ea, "Va": (1 - D) * Vs, "Ia": Ia, "T_brake": K * Ia, "P_mech": Ea * Ia, "P_regen": Pr, "efficiency": Pr / (Ea * Ia)}


_add("pe_regenerative_braking_chopper",
     name="Regenerative braking of a DC motor with a chopper", topic="motor drives / regenerative braking",
     purpose="The back EMF, average armature voltage, armature current, braking torque, mechanical power absorbed, power returned to the supply and braking efficiency of a DC motor braked regeneratively by a step-up chopper.",
     inputs=[_V("Vs", "dc supply voltage"), _numin("D", "chopper duty cycle (a fraction or %)", pct=True), _R("Ra", "armature resistance"),
             _KM("K", "motor constant"), _W("omega", "motor speed")],
     outputs=[_o("Ea", "voltage", "back EMF K w"), _o("Va", "voltage", "average armature voltage"), _o("Ia", "current", "armature (braking) current"),
              _o("T_brake", "torque", "braking torque"), _o("P_mech", "power", "mechanical power absorbed Ea Ia"), _o("P_regen", "power", "power returned to the supply"),
              _o("efficiency", "number", "braking efficiency P_regen/P_mech")],
     primary="Ia", conditions=["Ea > (1 - D) Vs", "continuous armature current"],
     method="Ea = K w; Va = (1 - D) Vs; Ia = (Ea - Va)/Ra; P_regen = Va Ia",
     latex=r"I_a = \frac{K\omega - (1-D)V_s}{R_a},\quad P_{regen} = (1-D)V_sI_a",
     run=regen_chopper, reference=_ref_regen_chopper,
     wrong=lambda Vs, D, Ra, K, omega: (lambda Ia: _out(Ea=K * omega, Va=D * Vs, Ia=Ia, T_brake=K * Ia, P_mech=K * omega * Ia, P_regen=D * Vs * Ia, efficiency=D * Vs / (K * omega)))((K * omega - D * Vs) / Ra),
     rand=lambda rng: (lambda Vs, K: {"Vs": Vs, "D": round(rng.uniform(0.2, 0.9), 3), "Ra": _sig(_lu(rng, 0.05, 2.0), 3), "K": K, "omega": _sig(Vs / K * rng.uniform(0.4, 1.0), 3)})(
         _sig(rng.uniform(48.0, 600.0), 3), _sig(rng.uniform(0.3, 3.0), 3)),
     phrases=("Regenerative braking chopper", "DC motor regenerative braking"),
     examples=[_ex({"Vs": 220.0, "D": 0.5, "Ra": 0.5, "K": 1.0, "omega": 150.0}), _ex({"Vs": 600.0, "D": 0.3, "Ra": 0.1, "K": 2.5, "omega": 200.0})],
     level="intermediate")


def armature_ripple(Vs, D, fs, Ra, La, Ea):
    _pos("Vs, fs, Ra and La", Vs, fs, Ra, La)
    _frac("the duty cycle D", D)
    _nonneg("the back EMF Ea", Ea)
    T = 1.0 / fs
    tau = La / Ra
    Imax = (Vs / Ra) * (1.0 - math.exp(-D * T / tau)) / (1.0 - math.exp(-T / tau)) - Ea / Ra
    Imin = (Vs / Ra) * (math.exp(D * T / tau) - 1.0) / (math.exp(T / tau) - 1.0) - Ea / Ra
    _need(Imin > 0, "the armature current falls to zero: discontinuous conduction")
    return _out(I_max=Imax, I_min=Imin, dI=Imax - Imin, I_avg=(D * Vs - Ea) / Ra)


def _ref_armature_ripple(Vs, D, fs, Ra, La, Ea):
    import numpy as np
    from scipy.linalg import expm
    T = 1.0 / fs
    Mx = lambda v, t: expm(np.array([[-Ra / La, (v - Ea) / La], [0.0, 0.0]]) * t)     # noqa: E731  state [i, 1]
    Pon, Poff = Mx(Vs, D * T), Mx(0.0, (1 - D) * T)
    P = Poff @ Pon
    i0 = P[0, 1] / (1.0 - P[0, 0])                       # periodic steady state i(T) = i(0)
    imax = float((Pon @ np.array([i0, 1.0]))[0])
    cur = lambda t: float((Mx(Vs, t) @ np.array([i0, 1.0]))[0]) if t < D * T else float((Mx(0.0, t - D * T) @ np.array([imax, 1.0]))[0])  # noqa: E731
    avg = (_quad(cur, 0.0, D * T) + _quad(cur, D * T, T)) / T
    return {"I_max": imax, "I_min": float(i0), "dI": imax - float(i0), "I_avg": avg}


_add("pe_dc_motor_armature_ripple",
     name="Armature current ripple of a chopper-fed DC motor", topic="motor drives / current ripple",
     purpose="The maximum, minimum, peak-to-peak and average armature current of a DC motor fed by a step-down chopper, from the exact exponential (RL plus back-EMF) waveform in continuous conduction.",
     inputs=[_V("Vs", "dc supply voltage"), _numin("D", "duty cycle (a fraction or %)", pct=True), _F("fs", "chopping frequency"), _R("Ra", "armature resistance"),
             _phys("La", "inductance", "armature inductance", show=_MH), _V("Ea", "back EMF")],
     outputs=[_o("I_max", "current", "maximum armature current"), _o("I_min", "current", "minimum armature current"), _o("dI", "current", "peak-to-peak ripple"),
              _o("I_avg", "current", "average armature current")],
     primary="dI", conditions=["continuous conduction (I_min > 0)", "constant speed (constant back EMF) over a period"],
     method="I_max = (Vs/Ra)(1 - e^(-DT/tau))/(1 - e^(-T/tau)) - Ea/Ra; I_min = (Vs/Ra)(e^(DT/tau) - 1)/(e^(T/tau) - 1) - Ea/Ra; I_avg = (D Vs - Ea)/Ra",
     latex=r"I_{max} = \frac{V_s}{R_a}\frac{1-e^{-DT/\tau}}{1-e^{-T/\tau}} - \frac{E_a}{R_a},\ I_{min} = \frac{V_s}{R_a}\frac{e^{DT/\tau}-1}{e^{T/\tau}-1} - \frac{E_a}{R_a}",
     run=armature_ripple, reference=_ref_armature_ripple,
     wrong=lambda Vs, D, fs, Ra, La, Ea: (lambda r: dict(r, dI=_r(Vs * D * (1 - D) / (La * fs) / 2), I_max=_r(r["I_avg"] + Vs * D * (1 - D) / (La * fs) / 4)))(armature_ripple(Vs, D, fs, Ra, La, Ea)),
     rand=lambda rng: (lambda Vs, D, Ra: {"Vs": Vs, "D": D, "fs": _sig(_lu(rng, 200.0, 5e3), 3), "Ra": Ra, "La": _sig(_lu(rng, 1e-3, 5e-2), 3),
                                          "Ea": _sig(D * Vs * rng.uniform(0.5, 0.95), 3)})(_sig(rng.uniform(24.0, 600.0), 3), round(rng.uniform(0.2, 0.9), 3), _sig(_lu(rng, 0.1, 5.0), 3)),
     phrases=("Armature current ripple", "Chopper DC motor current ripple"),
     examples=[_ex({"Vs": 220.0, "D": 0.5, "fs": 1e3, "Ra": 1.0, "La": 10e-3, "Ea": 100.0}), _ex({"Vs": 100.0, "D": 0.7, "fs": 500.0, "Ra": 0.5, "La": 5e-3, "Ea": 60.0})],
     level="advanced")


def induction_vf(V_rated, f_rated, poles, n_rated, f, T_ratio):
    _pos("V_rated, f_rated, n_rated, f and T_ratio", V_rated, f_rated, n_rated, f, T_ratio)
    _need(isinstance(poles, int) and poles >= 2 and poles % 2 == 0, "the number of poles must be an even whole number")
    _need(f <= f_rated, "constant V/f applies up to the rated frequency")
    ns_r = 120.0 * f_rated / poles
    _need(n_rated < ns_r, "the rated speed must be below the synchronous speed")
    _need(T_ratio <= 1.5, "the load torque ratio must be at most 1.5 (the linear torque-slip region)")
    ns = 120.0 * f / poles
    dn = (ns_r - n_rated) * T_ratio
    n = ns - dn
    _need(n > 0, "at this frequency the load torque cannot be carried (the speed would be negative)")
    return _out(V=V_rated * f / f_rated, ns=ns, n=n, slip=dn / ns, f_slip=dn * poles / 120.0)


def _ref_induction_vf(V_rated, f_rated, poles, n_rated, f, T_ratio):
    # small-slip torque of the equivalent circuit: T proportional to V^2 s / (f R2'); solve T(s, f) = T_ratio T_rated
    V = _brentq(lambda v: v / f - V_rated / f_rated, 0.0, 10 * V_rated)
    ns_r = 120 * f_rated / poles
    s_r = (ns_r - n_rated) / ns_r
    torque = lambda s, v, ff: v * v * s / ff                                     # noqa: E731
    Tr = torque(s_r, V_rated, f_rated)
    s = _brentq(lambda s: torque(s, V, f) - T_ratio * Tr, 0.0, 1.0)
    ns = 120 * f / poles
    return {"V": V, "ns": ns, "n": ns * (1 - s), "slip": s, "f_slip": s * f}


_add("pe_induction_motor_vf",
     name="Induction motor constant V/f speed control", topic="motor drives / induction motor",
     purpose="The supply voltage, synchronous speed, rotor speed, slip and slip frequency of an induction motor under constant V/f control below base frequency, from its rated voltage, frequency and speed (linear torque-slip region, torque as a fraction of rated).",
     inputs=[_V("V_rated", "rated line voltage"), _phys("f_rated", "frequency", "rated (base) frequency", show=_HZ), _intin("poles", "number of poles", 2, 48),
             _numin("n_rated", "rated speed in rpm"), _phys("f", "frequency", "operating frequency", show=_HZ), _numin("T_ratio", "load torque as a fraction of rated torque (or %)", pct=True)],
     outputs=[_o("V", "voltage", "supply voltage at f (constant V/f)"), _o("ns", "number", "synchronous speed in rpm"), _o("n", "number", "rotor speed in rpm"),
              _o("slip", "number", "slip"), _o("f_slip", "frequency", "slip (rotor) frequency")],
     primary="n", conditions=["f <= f_rated", "linear torque-slip region (T_ratio <= 1.5)", "stator resistance drop neglected"],
     method="V = V_rated f/f_rated; ns = 120 f/P; the slip speed at constant V/f is proportional to torque: n = ns - (ns,rated - n_rated) T/T_rated",
     latex=r"\frac{V}{f} = \text{const},\quad n = \frac{120f}{P} - \left(n_{s,r} - n_r\right)\frac{T}{T_r}",
     run=induction_vf, reference=_ref_induction_vf,
     wrong=lambda V_rated, f_rated, poles, n_rated, f, T_ratio: (lambda r: dict(r, n=_r(n_rated * f / f_rated), slip=_r(1 - n_rated * f / f_rated / r["ns"])))(induction_vf(V_rated, f_rated, poles, n_rated, f, T_ratio)),
     rand=lambda rng: (lambda fr, P: {"V_rated": rng.choice([230.0, 400.0, 460.0, 690.0]), "f_rated": fr, "poles": P, "n_rated": _sig(120 * fr / P * rng.uniform(0.94, 0.99), 4),
                                      "f": _sig(fr * rng.uniform(0.2, 1.0), 3), "T_ratio": round(rng.uniform(0.2, 1.5), 3)})(rng.choice([50.0, 60.0]), rng.choice([2, 4, 6, 8])),
     phrases=("Induction motor V/f control", "Constant V/f induction motor"),
     examples=[_ex({"V_rated": 400.0, "f_rated": 50.0, "poles": 4, "n_rated": 1450.0, "f": 30.0, "T_ratio": 1.0}),
               _ex({"V_rated": 460.0, "f_rated": 60.0, "poles": 6, "n_rated": 1170.0, "f": 40.0, "T_ratio": 0.5})],
     level="intermediate",
     extra_q=[{"q": "Induction motor V/f control: V_rated = 400 V, f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 100%", "like": 0}],
     weak="the small-slip torque relation T ~ V^2 s/f re-solved for slip with brentq")


def pmsm_dq(Rs, Ld, Lq, psi_f, p, id, iq, omega_m):
    _pos("Ld, Lq, psi_f and omega_m", Ld, Lq, psi_f, omega_m)
    _nonneg("Rs", Rs)
    _need(isinstance(p, int) and 1 <= p <= 50, "the number of pole pairs p must be a whole number from 1 to 50")
    _fin(id, iq)
    _need(id != 0 or iq != 0, "the current must not be zero")
    we = p * omega_m
    vd = Rs * id - we * Lq * iq
    vq = Rs * iq + we * (Ld * id + psi_f)
    Te = 1.5 * p * (psi_f * iq + (Ld - Lq) * id * iq)
    Pin = 1.5 * (vd * id + vq * iq)
    Pcu = 1.5 * Rs * (id * id + iq * iq)
    Vm = math.hypot(vd, vq)
    return _out(omega_e=we, vd=vd, vq=vq, V_peak=Vm, Te=Te, P_mech=Te * omega_m, P_in=Pin, P_cu=Pcu,
                PF=Pin / (1.5 * Vm * math.hypot(id, iq)))


def _ref_pmsm_dq(Rs, Ld, Lq, psi_f, p, id, iq, omega_m):
    import sympy as sp
    t = sp.Symbol("t")
    we = p * omega_m
    th = we * t
    sh = [0, -2 * sp.pi / 3, 2 * sp.pi / 3]
    nd, nq = _sq(id), _sq(iq)
    # abc currents and flux linkages from the rotor-frame quantities (amplitude-invariant inverse Park)
    ia = [nd * sp.cos(th + s) - nq * sp.sin(th + s) for s in sh]
    psid, psiq = _sq(Ld) * nd + _sq(psi_f), _sq(Lq) * nq
    psi = [psid * sp.cos(th + s) - psiq * sp.sin(th + s) for s in sh]
    va = [_sq(Rs) * ia[k] + sp.diff(psi[k], t) for k in range(3)]     # Faraday in each phase
    t0 = sp.Rational(3, 10) / max(we, 1e-9)
    ev = lambda e: float(e.subs(t, t0).evalf(30))                               # noqa: E731
    v = [ev(x) for x in va]
    i = [ev(x) for x in ia]
    thv = float((th).subs(t, t0))
    vd = 2 / 3 * sum(v[k] * math.cos(thv + float(sh[k])) for k in range(3))
    vq = -2 / 3 * sum(v[k] * math.sin(thv + float(sh[k])) for k in range(3))
    Pin = sum(v[k] * i[k] for k in range(3))
    Pcu = Rs * sum(x * x for x in i)
    Te = (Pin - Pcu) / omega_m                                                   # power balance
    Vm = math.hypot(vd, vq)
    return {"omega_e": we, "vd": vd, "vq": vq, "V_peak": Vm, "Te": Te, "P_mech": Pin - Pcu, "P_in": Pin, "P_cu": Pcu, "PF": Pin / (1.5 * Vm * math.hypot(id, iq))}


_add("pe_pmsm_dq_model",
     name="PMSM steady state in the dq frame: voltages and torque", topic="motor drives / PMSM",
     purpose="The electrical speed, d- and q-axis voltages, voltage magnitude, electromagnetic torque, mechanical, input and copper power and power factor of a permanent-magnet synchronous motor in steady state from its dq currents (amplitude-invariant Park transform).",
     inputs=[_R("Rs", "stator resistance per phase"), _phys("Ld", "inductance", "d-axis inductance", show=_MH), _phys("Lq", "inductance", "q-axis inductance", show=_MH),
             _phys("psi_f", "magnetic_flux", "permanent-magnet flux linkage", show=_MWB), _intin("p", "pole pairs", 1, 50), _I("id", "d-axis current (negative for field weakening)"),
             _I("iq", "q-axis current"), _W("omega_m", "mechanical speed")],
     outputs=[_o("omega_e", "angular_frequency", "electrical speed p w_m"), _o("vd", "voltage", "d-axis voltage"), _o("vq", "voltage", "q-axis voltage"),
              _o("V_peak", "voltage", "peak phase voltage magnitude"), _o("Te", "torque", "electromagnetic torque"), _o("P_mech", "power", "mechanical power"),
              _o("P_in", "power", "electrical input power"), _o("P_cu", "power", "stator copper loss"), _o("PF", "number", "power factor")],
     primary="Te", conditions=["steady state, sinusoidal machine, iron loss neglected", "amplitude-invariant transform"],
     method="vd = Rs id - omega_e Lq iq; vq = Rs iq + omega_e (Ld id + psi_f); Te = 1.5 p (psi_f iq + (Ld - Lq) id iq); P_in = 1.5 (vd id + vq iq)",
     latex=r"v_d = R_si_d - \omega_eL_qi_q,\ v_q = R_si_q + \omega_e(L_di_d+\psi_f),\ T_e = \tfrac32p\left(\psi_fi_q + (L_d-L_q)i_di_q\right)",
     run=pmsm_dq, reference=_ref_pmsm_dq,
     wrong=lambda Rs, Ld, Lq, psi_f, p, id, iq, omega_m: (lambda r: dict(r, Te=_r(r["Te"] / 1.5), P_mech=_r(r["P_mech"] / 1.5)))(pmsm_dq(Rs, Ld, Lq, psi_f, p, id, iq, omega_m)),
     rand=lambda rng: (lambda Ld: {"Rs": _sig(_lu(rng, 0.005, 1.0), 3), "Ld": Ld, "Lq": _sig(Ld * rng.uniform(1.0, 3.0), 3), "psi_f": _sig(_lu(rng, 0.01, 0.3), 3), "p": rng.randint(1, 8),
                                   "id": _sig(-rng.uniform(0.0, 50.0), 3), "iq": _sig(rng.uniform(1.0, 100.0), 3), "omega_m": _sig(_lu(rng, 10.0, 600.0), 3)})(_sig(_lu(rng, 1e-4, 1e-2), 3)),
     phrases=("PMSM dq model", "Permanent magnet synchronous motor dq"),
     examples=[_ex({"Rs": 0.05, "Ld": 0.4e-3, "Lq": 1.0e-3, "psi_f": 0.08, "p": 4, "id": -20.0, "iq": 60.0, "omega_m": 300.0}),
               _ex({"Rs": 0.2, "Ld": 2e-3, "Lq": 2e-3, "psi_f": 0.1, "p": 2, "id": 0.0, "iq": 10.0, "omega_m": 150.0})],
     level="intermediate")


def mtpa(Ld, Lq, psi_f, p, Is):
    _pos("Ld, Lq, psi_f and Is", Ld, Lq, psi_f, Is)
    _need(isinstance(p, int) and 1 <= p <= 50, "the number of pole pairs p must be a whole number from 1 to 50")
    _need(Lq > Ld, "MTPA with negative id needs a salient machine with Lq > Ld")
    dL = Lq - Ld
    id = (psi_f - math.sqrt(psi_f * psi_f + 8.0 * dL * dL * Is * Is)) / (4.0 * dL)
    iq = math.sqrt(Is * Is - id * id)
    Te = 1.5 * p * (psi_f * iq + (Ld - Lq) * id * iq)
    return _out(id=id, iq=iq, beta_deg=math.degrees(math.atan2(-id, iq)), Te=Te, Te_id0=1.5 * p * psi_f * Is)


def _ref_mtpa(Ld, Lq, psi_f, p, Is):
    import sympy as sp
    b = sp.Symbol("b")
    T = sp.Rational(3, 2) * p * (_sq(psi_f) * _sq(Is) * sp.cos(b)
                                 + _sq(Ld - Lq) * (-_sq(Is) * sp.sin(b)) * _sq(Is) * sp.cos(b))
    dT = sp.lambdify(b, sp.diff(T, b))
    beta = _brentq(lambda x: float(dT(x)), 0.0, math.pi / 4 + 1e-12)      # torque is largest where its derivative vanishes
    idv, iqv = -Is * math.sin(beta), Is * math.cos(beta)
    return {"id": idv, "iq": iqv, "beta_deg": math.degrees(beta), "Te": float(T.subs(b, sp.Float(beta, 30))), "Te_id0": float(T.subs(b, 0))}


_add("pe_pmsm_mtpa",
     name="Maximum torque per ampere (MTPA) of an interior PMSM", topic="motor drives / PMSM control",
     purpose="The d- and q-axis currents, current advance angle and torque that give the most torque for a given current magnitude in a salient (interior) permanent-magnet synchronous motor, and the torque with id = 0 for comparison.",
     inputs=[_phys("Ld", "inductance", "d-axis inductance", show=_MH), _phys("Lq", "inductance", "q-axis inductance (Lq > Ld)", show=_MH),
             _phys("psi_f", "magnetic_flux", "magnet flux linkage", show=_MWB), _intin("p", "pole pairs", 1, 50), _I("Is", "current magnitude (peak)")],
     outputs=[_o("id", "current", "d-axis current (negative)"), _o("iq", "current", "q-axis current"), _o("beta_deg", "number", "current advance angle from the q axis, degrees"),
              _o("Te", "torque", "maximum torque at this current"), _o("Te_id0", "torque", "torque with zero d-axis current")],
     primary="id", conditions=["Lq > Ld", "magnetic saturation neglected"],
     method="id = (psi_f - sqrt(psi_f^2 + 8 (Lq - Ld)^2 Is^2)) / (4 (Lq - Ld)); iq = sqrt(Is^2 - id^2)",
     latex=r"i_d = \frac{\psi_f - \sqrt{\psi_f^2 + 8(L_q-L_d)^2 I_s^2}}{4(L_q-L_d)},\quad i_q = \sqrt{I_s^2 - i_d^2}",
     run=mtpa, reference=_ref_mtpa,
     wrong=lambda Ld, Lq, psi_f, p, Is: (lambda r: dict(r, id=0.0, iq=_r(Is), beta_deg=0.0, Te=r["Te_id0"]))(mtpa(Ld, Lq, psi_f, p, Is)),
     rand=lambda rng: (lambda Ld: {"Ld": Ld, "Lq": _sig(Ld * rng.uniform(1.5, 4.0), 3), "psi_f": _sig(_lu(rng, 0.01, 0.3), 3), "p": rng.randint(1, 8),
                                   "Is": _sig(_lu(rng, 5.0, 300.0), 3)})(_sig(_lu(rng, 1e-4, 5e-3), 3)),
     phrases=("PMSM MTPA", "Maximum torque per ampere"),
     examples=[_ex({"Ld": 0.4e-3, "Lq": 1.0e-3, "psi_f": 0.08, "p": 4, "Is": 100.0}), _ex({"Ld": 2e-3, "Lq": 5e-3, "psi_f": 0.1, "p": 3, "Is": 20.0})],
     level="expert")


def field_weakening(Ls, psi_f, p, Imax, Vmax, omega):
    _pos("Ls, psi_f, Imax, Vmax and omega", Ls, psi_f, Imax, Vmax, omega)
    _need(isinstance(p, int) and 1 <= p <= 50, "the number of pole pairs p must be a whole number from 1 to 50")
    _need(psi_f > Ls * Imax, "the characteristic current psi_f/Ls is within the current limit: the speed is unbounded (no finite maximum)")
    wb = Vmax / (p * math.hypot(psi_f, Ls * Imax))
    wmax = Vmax / (p * (psi_f - Ls * Imax))
    _need(omega < wmax, "the speed is above the maximum reachable at this voltage and current limit")
    if omega <= wb:
        id = 0.0
    else:
        id = ((Vmax / (p * omega)) ** 2 - psi_f * psi_f - Ls * Ls * Imax * Imax) / (2.0 * psi_f * Ls)
    iq = math.sqrt(Imax * Imax - id * id)
    T = 1.5 * p * psi_f * iq
    return _out(omega_base=wb, omega_max=wmax, rpm_base=wb * _RPM, id=id, iq=iq, T=T, P=T * omega)


def _ref_field_weakening(Ls, psi_f, p, Imax, Vmax, omega):
    vmag = lambda w, g: p * w * math.hypot(psi_f - Ls * Imax * math.sin(g), Ls * Imax * math.cos(g))   # noqa: E731  current at angle g from q
    wb = _brentq(lambda w: vmag(w, 0.0) - Vmax, 1e-12, 1e12)
    wmax = _brentq(lambda w: vmag(w, math.pi / 2) - Vmax, 1e-12, 1e12)
    g = 0.0 if vmag(omega, 0.0) <= Vmax else _brentq(lambda g: vmag(omega, g) - Vmax, 0.0, math.pi / 2)
    idv, iqv = -Imax * math.sin(g), Imax * math.cos(g)
    T = 1.5 * p * psi_f * iqv
    return {"omega_base": wb, "omega_max": wmax, "rpm_base": wb * 60 / TWO_PI, "id": 0.0 if g == 0.0 else idv, "iq": iqv, "T": T, "P": T * omega}


_add("pe_pmsm_field_weakening",
     name="Surface PMSM field weakening: base speed, maximum speed and torque", topic="motor drives / field weakening",
     purpose="The base speed, the maximum speed, and at a given speed the d- and q-axis currents, torque and power of a surface (non-salient) permanent-magnet motor at its current and voltage limits (field weakening above base speed; stator resistance neglected).",
     inputs=[_phys("Ls", "inductance", "synchronous inductance (Ld = Lq)", show=_MH), _phys("psi_f", "magnetic_flux", "magnet flux linkage", show=_MWB),
             _intin("p", "pole pairs", 1, 50), _I("Imax", "current limit (peak)"), _V("Vmax", "voltage limit (peak phase)"), _W("omega", "mechanical speed of interest")],
     outputs=[_o("omega_base", "angular_frequency", "base speed (mechanical)"), _o("omega_max", "angular_frequency", "maximum speed (mechanical)"),
              _o("rpm_base", "number", "base speed in rpm"), _o("id", "current", "d-axis current at omega"), _o("iq", "current", "q-axis current at omega"),
              _o("T", "torque", "available torque at omega"), _o("P", "power", "available power at omega")],
     primary="T", conditions=["psi_f > Ls Imax (finite maximum speed)", "omega < omega_max", "Rs = 0"],
     method="w_b = Vmax/(p sqrt(psi_f^2 + (Ls Imax)^2)); w_max = Vmax/(p (psi_f - Ls Imax)); above w_b: id = ((Vmax/(p w))^2 - psi_f^2 - Ls^2 Imax^2)/(2 psi_f Ls)",
     latex=r"\omega_b = \frac{V_{max}}{p\sqrt{\psi_f^2 + (L_sI_{max})^2}},\ \omega_{max} = \frac{V_{max}}{p(\psi_f - L_sI_{max})},\ i_d = \frac{(V_{max}/p\omega)^2 - \psi_f^2 - L_s^2I_{max}^2}{2\psi_fL_s}",
     run=field_weakening, reference=_ref_field_weakening,
     wrong=lambda Ls, psi_f, p, Imax, Vmax, omega: (lambda r: dict(r, omega_base=_r(Vmax / (p * psi_f)), rpm_base=_r(Vmax / (p * psi_f) * _RPM)))(field_weakening(Ls, psi_f, p, Imax, Vmax, omega)),
     rand=lambda rng: (lambda psi, Ls, p, V: (lambda Imax: (lambda wb, wm: {"Ls": Ls, "psi_f": psi, "p": p, "Imax": Imax, "Vmax": V, "omega": _sig(rng.uniform(0.3 * wb, wb + 0.95 * (wm - wb)), 3)})(
         V / (p * math.hypot(psi, Ls * Imax)), V / (p * (psi - Ls * Imax))))(_sig(psi / Ls * rng.uniform(0.2, 0.8), 3)))(
         _sig(_lu(rng, 0.02, 0.3), 3), _sig(_lu(rng, 1e-4, 5e-3), 3), rng.randint(1, 8), _sig(rng.uniform(24.0, 400.0), 3)),
     phrases=("PMSM field weakening", "Surface PMSM field weakening"),
     examples=[_ex({"Ls": 1e-3, "psi_f": 0.1, "p": 4, "Imax": 60.0, "Vmax": 200.0, "omega": 600.0}), _ex({"Ls": 0.5e-3, "psi_f": 0.05, "p": 2, "Imax": 50.0, "Vmax": 48.0, "omega": 150.0})],
     level="expert")


# ================================================================================================ DEVICES AND SWITCHING
def mosfet_conduction(Rds25, alpha, Tj, Irms):
    _pos("Rds25, Tj and Irms", Rds25, Tj, Irms)
    _nonneg("the temperature coefficient alpha", alpha)
    R = Rds25 * (1.0 + alpha * (Tj - 298.15))
    _need(R > 0, "the on-resistance would not be positive at this temperature")
    return _out(Rds_T=R, P=Irms * Irms * R, P_25=Irms * Irms * Rds25, ratio=R / Rds25)


def _ref_mosfet_conduction(Rds25, alpha, Tj, Irms):
    R = Rds25 + Rds25 * alpha * (Tj - T0C - 25.0)
    Ip = Irms / math.sqrt(0.37)                         # a rectangular current of duty 0.37 with this RMS value
    P = _quad(lambda t: Ip * Ip * R, 0.0, 0.37)
    return {"Rds_T": R, "P": P, "P_25": _quad(lambda t: Ip * Ip * Rds25, 0.0, 0.37), "ratio": R / Rds25}


_add("pe_mosfet_conduction_loss",
     name="MOSFET conduction loss at junction temperature", topic="devices / conduction loss",
     purpose="The on-resistance of a MOSFET at its junction temperature (linear temperature coefficient from the 25 degC value) and the conduction loss for an RMS drain current, with the loss at 25 degC for comparison.",
     inputs=[_R("Rds25", "on-resistance at room temperature"), _numin("alpha", "temperature coefficient of the on-resistance per kelvin", pct=True),
             _phys("Tj", "temperature", "junction temperature"), _I("Irms", "RMS drain current")],
     outputs=[_o("Rds_T", "resistance", "on-resistance at Tj"), _o("P", "power", "conduction loss at the junction temperature"), _o("P_25", "power", "conduction loss at room temperature"),
              _o("ratio", "number", "ratio of hot to room-temperature on-resistance")],
     primary="P", conditions=["linear temperature model"],
     method="Rds(Tj) = Rds25 (1 + alpha (Tj - 25 degC)); P = Irms^2 Rds(Tj)",
     latex=r"R_{ds}(T_j) = R_{ds,25}\left(1+\alpha(T_j-25^\circ\mathrm{C})\right),\quad P = I_{rms}^2R_{ds}(T_j)",
     run=mosfet_conduction, reference=_ref_mosfet_conduction,
     wrong=lambda Rds25, alpha, Tj, Irms: (lambda R: _out(Rds_T=R, P=Irms * Irms * R, P_25=Irms * Irms * Rds25, ratio=R / Rds25))(Rds25 * (1 + alpha * (Tj - T0C))),
     rand=lambda rng: {"Rds25": _sig(_lu(rng, 1e-3, 0.5), 3), "alpha": round(rng.uniform(0.003, 0.01), 4), "Tj": _sig(rng.uniform(313.15, 448.15), 5), "Irms": _sig(_lu(rng, 0.5, 100.0), 3)},
     phrases=("MOSFET conduction loss", "MOSFET on-state loss"),
     examples=[_ex({"Rds25": 0.01, "alpha": 0.007, "Tj": 398.15, "Irms": 20.0}), _ex({"Rds25": 0.08, "alpha": 0.006, "Tj": 373.15, "Irms": 5.0})],
     level="beginner",
     extra_q=[{"q": "MOSFET conduction loss: Rds25 = 10 mohm, alpha = 0.7%, Tj = 125 degC, Irms = 20 A", "like": 0}],
     weak="the same linear temperature model; the loss integrated with scipy quad over an equivalent rectangular current")


def switching_loss(V, I, tr, tf, fs, Coss):
    _pos("V, I and fs", V, I, fs)
    _nonneg("tr, tf and Coss", tr, tf, Coss)
    Eon = 0.5 * V * I * tr
    Eoff = 0.5 * V * I * tf
    Psw = (Eon + Eoff) * fs
    Poss = 0.5 * Coss * V * V * fs
    return _out(E_on=Eon, E_off=Eoff, P_sw=Psw, P_oss=Poss, P_total=Psw + Poss)


def _ref_switching_loss(V, I, tr, tf, fs, Coss):
    # clamped inductive switching: at turn-on the current rises at full voltage, then the voltage falls at full
    # current (each half of tr here); at turn-off the reverse within tf; energy = integral of v i
    def edge(tt):
        if tt == 0:
            return 0.0
        h = tt / 2
        return _quad(lambda t: V * (I * t / h), 0.0, h) + _quad(lambda t: V * (1 - t / h) * I, 0.0, h)
    Eon, Eoff = edge(tr), edge(tf)
    Eoss = _quad(lambda v: Coss * v, 0.0, V)              # energy stored in the output capacitance, lost at turn-on
    return {"E_on": Eon, "E_off": Eoff, "P_sw": (Eon + Eoff) * fs, "P_oss": Eoss * fs, "P_total": (Eon + Eoff + Eoss) * fs}


_add("pe_switching_loss",
     name="Hard-switching loss of a power transistor", topic="devices / switching loss",
     purpose="The turn-on and turn-off energies and the switching and output-capacitance losses of a MOSFET or IGBT switching an inductive (clamped) load with linear voltage and current transitions.",
     inputs=[_V("V", "blocking (dc bus) voltage"), _I("I", "switched current"), _phys("tr", "time", "turn-on crossover time", show=_NS),
             _phys("tf", "time", "turn-off crossover time", show=_NS), _F("fs", "switching frequency"), _phys("Coss", "capacitance", "output capacitance", show=_PF)],
     outputs=[_o("E_on", "energy", "turn-on energy"), _o("E_off", "energy", "turn-off energy"), _o("P_sw", "power", "switching loss"),
              _o("P_oss", "power", "output-capacitance loss"), _o("P_total", "power", "total switching-related loss")],
     primary="P_sw", conditions=["clamped inductive load, linear transitions"],
     method="E_on = V I tr/2; E_off = V I tf/2; P_sw = (E_on + E_off) fs; P_oss = Coss V^2 fs/2",
     latex=r"P_{sw} = \tfrac12 V I\,(t_r + t_f)\,f_s,\quad P_{oss} = \tfrac12C_{oss}V^2f_s",
     run=switching_loss, reference=_ref_switching_loss,
     wrong=lambda V, I, tr, tf, fs, Coss: (lambda Eon, Eoff: _out(E_on=Eon, E_off=Eoff, P_sw=(Eon + Eoff) * fs, P_oss=0.5 * Coss * V * V * fs, P_total=(Eon + Eoff) * fs + 0.5 * Coss * V * V * fs))(V * I * tr / 6, V * I * tf / 6),
     rand=lambda rng: {"V": _sig(rng.uniform(12.0, 1000.0), 3), "I": _sig(_lu(rng, 0.5, 300.0), 3), "tr": _sig(_lu(rng, 5e-9, 5e-7), 3), "tf": _sig(_lu(rng, 5e-9, 5e-7), 3),
                       "fs": _sig(_lu(rng, 2e3, 1e6), 3), "Coss": _sig(_lu(rng, 1e-11, 2e-9), 3)},
     phrases=("Switching loss", "Transistor switching loss"),
     examples=[_ex({"V": 400.0, "I": 10.0, "tr": 30e-9, "tf": 50e-9, "fs": 100e3, "Coss": 100e-12}), _ex({"V": 600.0, "I": 100.0, "tr": 100e-9, "tf": 200e-9, "fs": 10e3, "Coss": 0.0})],
     level="beginner")


def gate_drive(Qg, Vdrv, fs, Rg):
    _pos("Qg, Vdrv, fs and Rg", Qg, Vdrv, fs, Rg)
    return _out(P_drive=Qg * Vdrv * fs, Ig_avg=Qg * fs, Ig_peak=Vdrv / Rg, E_cycle=Qg * Vdrv)


def _ref_gate_drive(Qg, Vdrv, fs, Rg):
    # the gate as an equivalent capacitance Qg/Vdrv charged through Rg: supply energy per cycle = integral Vdrv i dt
    C = Qg / Vdrv
    tau = Rg * C
    i = lambda t: (Vdrv / Rg) * math.exp(-t / tau)                          # noqa: E731
    E = sum(_quad(lambda t: Vdrv * i(t), k * tau, (k + 1) * tau) for k in range(60))
    return {"P_drive": E * fs, "Ig_avg": sum(_quad(i, k * tau, (k + 1) * tau) for k in range(60)) * fs, "Ig_peak": i(0.0), "E_cycle": E}


_add("pe_gate_drive_power",
     name="Gate-drive power and currents", topic="devices / gate drive",
     purpose="The gate-driver power, energy per switching cycle, average gate current and peak gate current of a MOSFET or IGBT from its total gate charge, drive voltage, switching frequency and total gate resistance.",
     inputs=[_phys("Qg", "charge", "total gate charge at the drive voltage", show=_NC), _V("Vdrv", "gate-drive voltage swing"), _F("fs", "switching frequency"),
             _R("Rg", "total gate resistance (driver, external and internal)")],
     outputs=[_o("P_drive", "power", "gate-drive power Qg V fs"), _o("Ig_avg", "current", "average gate current Qg fs"), _o("Ig_peak", "current", "peak gate current V/Rg"),
              _o("E_cycle", "energy", "energy per switching cycle Qg V")],
     primary="P_drive", conditions=["the gate fully charges and discharges every cycle"],
     method="P = Qg Vdrv fs; Ig,avg = Qg fs; Ig,peak = Vdrv/Rg",
     latex=r"P_{G} = Q_gV_{drv}f_s,\quad I_{G,pk} = \frac{V_{drv}}{R_g}",
     run=gate_drive, reference=_ref_gate_drive,
     wrong=lambda Qg, Vdrv, fs, Rg: _out(P_drive=0.5 * Qg * Vdrv * fs, Ig_avg=Qg * fs, Ig_peak=Vdrv / Rg, E_cycle=0.5 * Qg * Vdrv),
     rand=lambda rng: {"Qg": _sig(_lu(rng, 5e-9, 2e-6), 3), "Vdrv": rng.choice([10.0, 12.0, 15.0, 18.0, 20.0, 5.0]), "fs": _sig(_lu(rng, 1e3, 2e6), 3), "Rg": _sig(_lu(rng, 0.5, 50.0), 3)},
     phrases=("Gate drive power", "Gate driver power"),
     examples=[_ex({"Qg": 50e-9, "Vdrv": 12.0, "fs": 200e3, "Rg": 5.0}), _ex({"Qg": 1.2e-6, "Vdrv": 15.0, "fs": 10e3, "Rg": 2.0})],
     level="beginner")


def reverse_recovery(Irm, trr, S, Vr, fs):
    _pos("Irm, trr, S, Vr and fs", Irm, trr, S, Vr, fs)
    ta = trr / (1.0 + S)
    tb = S * trr / (1.0 + S)
    Qrr = 0.5 * Irm * trr
    Ed = Vr * Irm * tb / 6.0
    Es = Vr * Irm * (ta / 2.0 + tb / 3.0)
    return _out(Qrr=Qrr, ta=ta, tb=tb, E_diode=Ed, E_switch=Es, P_diode=Ed * fs, P_switch=Es * fs, P_total=(Ed + Es) * fs)


def _ref_reverse_recovery(Irm, trr, S, Vr, fs):
    import sympy as sp
    a, b = sp.symbols("ta tb", positive=True)
    sol = sp.solve([a + b - _sq(trr), b - _sq(S) * a], [a, b], dict=True)[0]
    ta, tb = float(sol[a]), float(sol[b])
    # recovery current: rises to Irm over ta (diode still carries no voltage, the switch sees Vr), then decays to
    # zero over tb while the diode voltage rises linearly to Vr and the switch voltage falls to zero
    ir = lambda t: Irm * t / ta if t < ta else Irm * (1 - (t - ta) / tb)                       # noqa: E731
    vd = lambda t: 0.0 if t < ta else Vr * (t - ta) / tb                                       # noqa: E731
    Q = _quad(ir, 0.0, ta) + _quad(ir, ta, ta + tb)
    Ed = _quad(lambda t: vd(t) * ir(t), ta, ta + tb)
    Es = _quad(lambda t: (Vr - vd(t)) * ir(t), 0.0, ta) + _quad(lambda t: (Vr - vd(t)) * ir(t), ta, ta + tb)
    return {"Qrr": Q, "ta": ta, "tb": tb, "E_diode": Ed, "E_switch": Es, "P_diode": Ed * fs, "P_switch": Es * fs, "P_total": (Ed + Es) * fs}


_add("pe_diode_reverse_recovery",
     name="Diode reverse-recovery charge and losses", topic="devices / reverse recovery",
     purpose="The recovered charge, the two recovery intervals and the reverse-recovery energy dissipated in the diode and the extra turn-on energy in the switch (and the powers) for a triangular recovery current of peak Irm, time trr and softness S = tb/ta.",
     inputs=[_I("Irm", "peak reverse-recovery current"), _phys("trr", "time", "reverse-recovery time", show=_NS), _numin("S", "softness factor tb/ta"),
             _V("Vr", "reverse (bus) voltage"), _F("fs", "switching frequency")],
     outputs=[_o("Qrr", "charge", "recovered charge"), _o("ta", "time", "current rise interval"), _o("tb", "time", "current decay interval"),
              _o("E_diode", "energy", "recovery energy dissipated in the diode"), _o("E_switch", "energy", "extra switch turn-on energy"),
              _o("P_diode", "power", "diode recovery loss"), _o("P_switch", "power", "switch loss caused by recovery"), _o("P_total", "power", "total recovery loss (= Vr Qrr fs)")],
     primary="Qrr", conditions=["triangular recovery current; diode voltage rises linearly during tb"],
     method="ta = trr/(1 + S), tb = S trr/(1 + S); Qrr = Irm trr/2; E_D = Vr Irm tb/6; E_sw = Vr Irm (ta/2 + tb/3); together Vr Qrr",
     latex=r"Q_{rr} = \tfrac12I_{rm}t_{rr},\quad E_D = \frac{V_rI_{rm}t_b}{6},\quad E_{sw} = V_rI_{rm}\left(\frac{t_a}{2}+\frac{t_b}{3}\right)",
     run=reverse_recovery, reference=_ref_reverse_recovery,
     wrong=lambda Irm, trr, S, Vr, fs: (lambda r: dict(r, Qrr=_r(Irm * trr)))(reverse_recovery(Irm, trr, S, Vr, fs)),
     rand=lambda rng: {"Irm": _sig(_lu(rng, 0.5, 100.0), 3), "trr": _sig(_lu(rng, 1e-8, 1e-6), 3), "S": _sig(_lu(rng, 0.2, 3.0), 3), "Vr": _sig(rng.uniform(24.0, 1200.0), 3), "fs": _sig(_lu(rng, 1e3, 5e5), 3)},
     phrases=("Diode reverse recovery", "Reverse recovery loss"),
     examples=[_ex({"Irm": 10.0, "trr": 100e-9, "S": 1.0, "Vr": 400.0, "fs": 100e3}), _ex({"Irm": 40.0, "trr": 300e-9, "S": 0.5, "Vr": 600.0, "fs": 20e3})],
     level="intermediate")


def parallel_sharing(R, I):
    _need(isinstance(R, list) and 2 <= len(R) <= 12, "give 2 to 12 on-resistances")
    for r in R:
        _pos("every on-resistance", r)
    _pos("the total current I", I)
    G = 0.0
    for r in R:
        G += 1.0 / r
    cur = [I * (1.0 / r) / G for r in R]
    loss = [cur[k] * cur[k] * R[k] * 1e-3 for k in range(len(R))]
    tot = 0.0
    for x in loss:
        tot += x
    return _out(currents=cur, losses=loss, P_total=tot, imbalance=max(cur) / (I / len(R)))


def _ref_parallel_sharing(R, I):
    import numpy as np
    from scipy.linalg import solve
    g = np.array([1.0 / (r * 1e-3) for r in R])
    V = float(solve(np.array([[g.sum()]]), np.array([I]))[0])           # one node: the current source into parallel resistances
    cur = [V * float(x) for x in g]
    loss = [V * c for c in cur]
    return {"currents": cur, "losses": loss, "P_total": V * I, "imbalance": max(cur) / (I / len(R))}


_add("pe_parallel_mosfet_sharing",
     name="Current sharing between paralleled MOSFETs", topic="devices / paralleling",
     purpose="The current and conduction loss of each of several paralleled MOSFETs (or any resistive devices) sharing a total current in proportion to their on-state conductances, the total loss and the worst-case current imbalance.",
     inputs=[_vecin("R", "on-resistance of each device in milliohms", 2, 12), _I("I", "total current")],
     outputs=[_o("currents", "number", "current in each device, amperes"), _o("losses", "number", "conduction loss of each device, watts"),
              _o("P_total", "power", "total conduction loss"), _o("imbalance", "number", "largest current divided by the equal share")],
     primary="P_total", conditions=["static sharing (on-resistance only)"],
     method="I_k = I (1/R_k)/sum(1/R_j); P_k = I_k^2 R_k",
     latex=r"I_k = I\frac{1/R_k}{\sum_j 1/R_j},\quad P_k = I_k^2R_k",
     run=parallel_sharing, reference=_ref_parallel_sharing,
     wrong=lambda R, I: (lambda n: _out(currents=[I / n] * n, losses=[(I / n) ** 2 * r * 1e-3 for r in R], P_total=sum((I / n) ** 2 * r * 1e-3 for r in R), imbalance=1.0))(len(R)),
     rand=lambda rng: (lambda R0: {"R": [_sig(R0 * rng.uniform(0.8, 1.3), 3) for _ in range(rng.randint(2, 5))], "I": _sig(_lu(rng, 5.0, 300.0), 3)})(_lu(rng, 1.0, 50.0)),
     phrases=("Parallel MOSFET current sharing", "Paralleled MOSFET sharing"),
     examples=[_ex({"R": [10.0, 12.0], "I": 40.0}), _ex({"R": [4.5, 5.0, 5.5], "I": 90.0})],
     level="beginner")


def bootstrap(Qg, Ileak, fs, Dmax, dV):
    _pos("Qg, fs and dV", Qg, fs, dV)
    _nonneg("the leakage and quiescent current", Ileak)
    _frac("the maximum duty cycle", Dmax)
    Q = Qg + Ileak * Dmax / fs
    return _out(Q_total=Q, C_min=Q / dV, C_recommended=10.0 * Q / dV)


def _ref_bootstrap(Qg, Ileak, fs, Dmax, dV):
    # charge drawn from the bootstrap capacitor while the high side is on: the gate charge plus the leakage and
    # quiescent current integrated over the longest on-time; C must hold the droop to dV
    Q = Qg + _quad(lambda t: Ileak, 0.0, Dmax / fs)
    C = _brentq(lambda c: Q / c - dV, 1e-18, 1.0)
    return {"Q_total": Q, "C_min": C, "C_recommended": 10 * C}


_add("pe_bootstrap_capacitor",
     name="Bootstrap capacitor for a high-side gate driver", topic="devices / gate drive",
     purpose="The charge drawn from a bootstrap capacitor during the longest high-side on-time and the smallest capacitance that keeps its droop within a limit, with the usual tenfold design margin.",
     inputs=[_phys("Qg", "charge", "high-side gate charge", show=_NC), _phys("Ileak", "current", "quiescent plus leakage current of the high side", show=("uA", 1e6)),
             _F("fs", "switching frequency"), _numin("Dmax", "maximum high-side duty cycle (a fraction or %)", pct=True), _V("dV", "allowed bootstrap droop")],
     outputs=[_o("Q_total", "charge", "charge drawn per cycle"), _o("C_min", "capacitance", "smallest bootstrap capacitance"),
              _o("C_recommended", "capacitance", "tenfold margin")],
     primary="C_min", conditions=["the capacitor recharges fully while the low side is on"],
     method="Q = Qg + I_leak Dmax/fs; C_min = Q/dV",
     latex=r"C_{boot} \ge \frac{Q_g + I_{lk}D_{max}/f_s}{\Delta V}",
     run=bootstrap, reference=_ref_bootstrap,
     wrong=lambda Qg, Ileak, fs, Dmax, dV: (lambda Q: _out(Q_total=Q, C_min=Q / dV, C_recommended=10 * Q / dV))(Qg + Ileak / fs),
     rand=lambda rng: {"Qg": _sig(_lu(rng, 5e-9, 5e-7), 3), "Ileak": _sig(_lu(rng, 1e-5, 5e-3), 3), "fs": _sig(_lu(rng, 1e3, 5e5), 3), "Dmax": round(rng.uniform(0.3, 0.95), 3), "dV": _sig(rng.uniform(0.1, 2.0), 3)},
     phrases=("Bootstrap capacitor", "Bootstrap capacitor sizing"),
     examples=[_ex({"Qg": 60e-9, "Ileak": 200e-6, "fs": 100e3, "Dmax": 0.9, "dV": 0.5}), _ex({"Qg": 200e-9, "Ileak": 1e-3, "fs": 10e3, "Dmax": 0.95, "dV": 1.0})],
     level="intermediate",
     weak="the same charge balance with the leakage charge integrated by scipy quad and the capacitance re-solved by brentq")


def turnoff_snubber(I, V, tf, fs):
    _pos("I, V, tf and fs", I, V, tf, fs)
    Cs = I * tf / (2.0 * V)
    Ew = I * I * tf * tf / (24.0 * Cs)
    Ewo = 0.5 * V * I * tf
    Es = 0.5 * Cs * V * V
    return _out(Cs=Cs, E_off_with=Ew, E_off_without=Ewo, E_snubber=Es, P_switch=Ew * fs, P_snubber=Es * fs, P_total=(Ew + Es) * fs)


def _ref_turnoff_snubber(I, V, tf, fs):
    # during the linear current fall the capacitor takes the current the switch gives up, I t/tf, and charges;
    # choose Cs so that its voltage just reaches V at the end of tf
    vc = lambda t, C: _quad(lambda x: I * x / tf, 0.0, t) / C                   # noqa: E731
    C = _brentq(lambda C: vc(tf, C) - V, 1e-18, 1.0)
    Ew = _quad(lambda t: vc(t, C) * I * (1 - t / tf), 0.0, tf)
    # without a snubber (clamped inductive load): the voltage rises at full current, then the current falls at full voltage
    Ewo = _quad(lambda t: V * I * (t / (tf / 2)), 0.0, tf / 2) + _quad(lambda t: V * I * (1 - t / (tf / 2)), 0.0, tf / 2)
    Es = _quad(lambda v: C * v, 0.0, V)
    return {"Cs": C, "E_off_with": Ew, "E_off_without": Ewo, "E_snubber": Es, "P_switch": Ew * fs, "P_snubber": Es * fs, "P_total": (Ew + Es) * fs}


_add("pe_turnoff_snubber",
     name="RCD turn-off snubber design", topic="devices / snubbers",
     purpose="The turn-off snubber capacitance that lets the switch voltage reach the bus voltage just as the current finishes falling, the switch turn-off energy with and without the snubber, and the snubber (resistor) loss per cycle and in watts.",
     inputs=[_I("I", "switched (load) current"), _V("V", "bus voltage"), _phys("tf", "time", "current fall time", show=_NS), _F("fs", "switching frequency")],
     outputs=[_o("Cs", "capacitance", "snubber capacitance"), _o("E_off_with", "energy", "switch turn-off energy with the snubber"),
              _o("E_off_without", "energy", "switch turn-off energy without a snubber"), _o("E_snubber", "energy", "energy dumped in the snubber resistor"),
              _o("P_switch", "power", "switch turn-off loss with the snubber"), _o("P_snubber", "power", "snubber resistor loss"), _o("P_total", "power", "switch plus snubber loss")],
     primary="Cs", conditions=["linear current fall; the snubber capacitor discharges fully during the on-time"],
     method="Cs = I tf/(2V); E_sw = I^2 tf^2/(24 Cs); E_R = Cs V^2/2",
     latex=r"C_s = \frac{I t_f}{2V},\quad E_{sw} = \frac{I^2t_f^2}{24C_s} = \frac{VIt_f}{12},\quad E_R = \tfrac12C_sV^2",
     run=turnoff_snubber, reference=_ref_turnoff_snubber,
     wrong=lambda I, V, tf, fs: (lambda Cs: _out(Cs=Cs, E_off_with=I * I * tf * tf / (24 * Cs), E_off_without=V * I * tf / 2, E_snubber=Cs * V * V / 2, P_switch=I * I * tf * tf / (24 * Cs) * fs,
                                                 P_snubber=Cs * V * V / 2 * fs, P_total=(I * I * tf * tf / (24 * Cs) + Cs * V * V / 2) * fs))(I * tf / V),
     rand=lambda rng: {"I": _sig(_lu(rng, 0.5, 200.0), 3), "V": _sig(rng.uniform(24.0, 1000.0), 3), "tf": _sig(_lu(rng, 1e-8, 1e-6), 3), "fs": _sig(_lu(rng, 1e3, 2e5), 3)},
     phrases=("Turn-off snubber", "RCD snubber design"),
     examples=[_ex({"I": 10.0, "V": 300.0, "tf": 100e-9, "fs": 50e3}), _ex({"I": 50.0, "V": 600.0, "tf": 400e-9, "fs": 10e3})],
     level="intermediate")


def rc_snubber_ringing(f0, f1, Cadd, k, V, fs):
    _pos("f0, f1, Cadd, k, V and fs", f0, f1, Cadd, k, V, fs)
    _need(f1 < f0, "adding capacitance must lower the ringing frequency (f1 < f0)")
    Cp = Cadd / ((f0 / f1) ** 2 - 1.0)
    Lp = 1.0 / ((TWO_PI * f0) ** 2 * Cp)
    Z0 = math.sqrt(Lp / Cp)
    Cs = k * Cp
    return _out(Cp=Cp, Lp=Lp, Rs=Z0, Cs=Cs, P_R=Cs * V * V * fs)


def _ref_rc_snubber_ringing(f0, f1, Cadd, k, V, fs):
    import sympy as sp
    L, C = sp.symbols("L C", positive=True)
    w0, w1 = _sq(TWO_PI * f0), _sq(TWO_PI * f1)
    # the measured ringing: w0 with the parasitic capacitance alone, w1 with Cadd in parallel
    s = sp.solve([L * C * w0 ** 2 - 1, L * (C + _sq(Cadd)) * w1 ** 2 - 1], [L, C], dict=True)[0]
    Lp, Cp = float(s[L]), float(s[C])
    Cs = k * Cp
    # the snubber capacitor charges to V and discharges through Rs every cycle: Cs V^2 per cycle
    return {"Cp": Cp, "Lp": Lp, "Rs": math.sqrt(Lp / Cp), "Cs": Cs, "P_R": 2 * _quad(lambda v: Cs * v, 0.0, V) * fs}


_add("pe_rc_snubber_ringing",
     name="RC snubber from measured ringing frequencies", topic="devices / snubbers",
     purpose="The parasitic capacitance and inductance of a ringing switch node found by measuring the ringing frequency with and without an added capacitor, the damping resistor (characteristic impedance), the snubber capacitor (k times the parasitic) and the snubber resistor loss.",
     inputs=[_F("f0", "ringing frequency without the added capacitor"), _F("f1", "ringing frequency with the added capacitor"),
             _phys("Cadd", "capacitance", "added test capacitance", show=_PF), _numin("k", "snubber capacitance as a multiple of the parasitic capacitance"),
             _V("V", "voltage the node swings to"), _F("fs", "switching frequency")],
     outputs=[_o("Cp", "capacitance", "parasitic capacitance"), _o("Lp", "inductance", "parasitic inductance"), _o("Rs", "resistance", "snubber resistance sqrt(Lp/Cp)"),
              _o("Cs", "capacitance", "snubber capacitance k Cp"), _o("P_R", "power", "snubber resistor loss")],
     primary="Rs", conditions=["f1 < f0"],
     method="Cp = Cadd/((f0/f1)^2 - 1); Lp = 1/((2 pi f0)^2 Cp); Rs = sqrt(Lp/Cp); Cs = k Cp; P = Cs V^2 fs",
     latex=r"C_p = \frac{C_{add}}{(f_0/f_1)^2-1},\quad L_p = \frac{1}{(2\pi f_0)^2C_p},\quad R_s = \sqrt{L_p/C_p}",
     run=rc_snubber_ringing, reference=_ref_rc_snubber_ringing,
     wrong=lambda f0, f1, Cadd, k, V, fs: (lambda Cp: (lambda Lp: _out(Cp=Cp, Lp=Lp, Rs=math.sqrt(Lp / Cp), Cs=k * Cp, P_R=k * Cp * V * V * fs))(1 / ((TWO_PI * f0) ** 2 * Cp)))(Cadd / (f0 / f1 - 1)),
     rand=lambda rng: (lambda f0: {"f0": f0, "f1": _sig(f0 * rng.uniform(0.3, 0.85), 3), "Cadd": _sig(_lu(rng, 1e-11, 2e-9), 3), "k": rng.choice([2.0, 3.0, 4.0]),
                                   "V": _sig(rng.uniform(12.0, 800.0), 3), "fs": _sig(_lu(rng, 1e4, 1e6), 3)})(_sig(_lu(rng, 5e6, 3e8), 3)),
     phrases=("RC snubber from ringing", "RC snubber design"),
     examples=[_ex({"f0": 50e6, "f1": 25e6, "Cadd": 300e-12, "k": 4.0, "V": 48.0, "fs": 500e3}), _ex({"f0": 30e6, "f1": 20e6, "Cadd": 1e-9, "k": 3.0, "V": 400.0, "fs": 100e3})],
     level="advanced",
     weak="the two resonance equations re-solved by sympy; the loss by integrating the capacitor's charging energy")


def zvs_transition(Vin, Ceq, L, I0):
    _pos("Vin, Ceq, L and I0", Vin, Ceq, L, I0)
    Z = math.sqrt(L / Ceq)
    w = 1.0 / math.sqrt(L * Ceq)
    Imin = Vin / Z
    ok = I0 * Z >= Vin
    t = math.asin(Vin / (I0 * Z)) / w if ok else (math.pi / 2.0) / w
    return _out(Z=Z, f_res=w / TWO_PI, I_min=Imin, E_L=0.5 * L * I0 * I0, E_C=0.5 * Ceq * Vin * Vin, zvs="achieved" if ok else "not achieved",
                t_transition=t, v_reached=min(I0 * Z, Vin))


def _ref_zvs_transition(Vin, Ceq, L, I0):
    from scipy.integrate import solve_ivp
    w = 1.0 / math.sqrt(L * Ceq)
    # the inductor current charges the node capacitance (C dv/dt = i) while the voltage opposes it (L di/dt = -v)
    rhs = lambda s, y: [y[1] / (Ceq * w), -y[0] / (L * w)]           # noqa: E731  in scaled time s = w t (event times near 1)
    hit = lambda t, y: y[0] - Vin                                      # noqa: E731
    hit.terminal, hit.direction = True, 1
    top = lambda t, y: y[1]                                            # noqa: E731  the current reaches zero: the voltage peaks
    top.terminal, top.direction = True, -1
    sol = solve_ivp(rhs, (0.0, 4.0), [0.0, I0], method="DOP853", rtol=1e-13, atol=1e-18 + 1e-16 * Vin, events=[hit, top], max_step=0.02)
    ok = len(sol.t_events[0]) > 0
    t = (float(sol.t_events[0][0]) if ok else float(sol.t_events[1][0])) / w
    vr = Vin if ok else float(sol.y_events[1][0][0])
    return {"Z": math.sqrt(L / Ceq), "f_res": w / TWO_PI, "I_min": Vin * math.sqrt(Ceq / L), "E_L": _quad(lambda i: L * i, 0.0, I0),
            "E_C": _quad(lambda v: Ceq * v, 0.0, Vin), "zvs": "achieved" if ok else "not achieved", "t_transition": t, "v_reached": vr}


_add("pe_zvs_transition",
     name="Zero-voltage switching: resonant transition of a switch node", topic="devices / soft switching",
     purpose="Whether an inductor current can swing a switch node's capacitance from 0 to the input voltage (zero-voltage switching), the minimum current needed, the energies, and the transition time (or the time to the voltage peak when it cannot), for a resonant LC transition.",
     inputs=[_V("Vin", "voltage the node must swing"), _phys("Ceq", "capacitance", "total node capacitance", show=_PF),
             _L("L", "resonant (leakage) inductance"), _I("I0", "inductor current at the start of the transition")],
     outputs=[_o("Z", "resistance", "characteristic impedance sqrt(L/C)"), _o("f_res", "frequency", "resonant frequency"), _o("I_min", "current", "minimum current for ZVS Vin/Z"),
              _o("E_L", "energy", "inductor energy"), _o("E_C", "energy", "capacitor energy"), _o("zvs", "number", "achieved or not achieved"),
              _o("t_transition", "time", "time to reach Vin (or to the voltage peak when ZVS fails)"), _o("v_reached", "voltage", "highest node voltage reached (capped at Vin)")],
     primary="t_transition", conditions=["lossless resonant transition starting at 0 V"],
     method="v(t) = I0 Z sin(w t), Z = sqrt(L/C); ZVS when I0 Z >= Vin; t = asin(Vin/(I0 Z))/w",
     latex=r"v(t) = I_0Z\sin\omega t,\quad Z = \sqrt{L/C},\quad t_{zvs} = \frac{1}{\omega}\sin^{-1}\frac{V_{in}}{I_0Z}",
     run=zvs_transition, reference=_ref_zvs_transition,
     wrong=lambda Vin, Ceq, L, I0: (lambda r: dict(r, t_transition=_r(Ceq * Vin / I0)))(zvs_transition(Vin, Ceq, L, I0)),
     rand=lambda rng: (lambda Vin, C, L: {"Vin": Vin, "Ceq": C, "L": L, "I0": _sig(Vin / math.sqrt(L / C) * rng.choice([rng.uniform(0.5, 0.95), rng.uniform(1.05, 5.0), rng.uniform(1.05, 5.0)]), 3)})(
         _sig(rng.uniform(12.0, 800.0), 3), _sig(_lu(rng, 5e-11, 5e-9), 3), _sig(_lu(rng, 1e-7, 1e-4), 3)),
     phrases=("Zero-voltage switching transition", "ZVS transition"),
     examples=[_ex({"Vin": 400.0, "Ceq": 400e-12, "L": 20e-6, "I0": 3.0}), _ex({"Vin": 48.0, "Ceq": 2e-9, "L": 0.5e-6, "I0": 1.0})],
     level="advanced")


# ================================================================================================ THERMAL DESIGN
def thermal_multi(P, Rjc, Rcs, Rsa, Ta):
    _need(isinstance(P, list) and 1 <= len(P) <= 12, "give 1 to 12 device losses")
    for x in P:
        _nonneg("every device loss", x)
    _pos("the thermal resistances and Ta", Rjc, Rcs, Rsa, Ta)
    tot = 0.0
    for x in P:
        tot += x
    Ts = Ta + Rsa * tot
    Tj = [Ts + x * (Rjc + Rcs) for x in P]
    return _out(P_total=tot, Ts=Ts, Tj=Tj, Tj_max=max(Tj), Tj_max_C=max(Tj) - T0C)


def _ref_thermal_multi(P, Rjc, Rcs, Rsa, Ta):
    import numpy as np
    from scipy.linalg import solve
    n = len(P)
    # nodes: junctions 0..n-1, cases n..2n-1, sink 2n; the ambient a fixed temperature; power injected at junctions
    N = 2 * n + 1
    G = np.zeros((N, N))
    b = np.zeros(N)

    def link(a, c, R):
        G[a, a] += 1 / R
        G[c, c] += 1 / R
        G[a, c] -= 1 / R
        G[c, a] -= 1 / R
    for k in range(n):
        link(k, n + k, Rjc)
        link(n + k, 2 * n, Rcs)
        b[k] += P[k]
    G[2 * n, 2 * n] += 1 / Rsa
    b[2 * n] += Ta / Rsa
    T = solve(G, b)
    Tj = [float(T[k]) for k in range(n)]
    return {"P_total": float(sum(P)), "Ts": float(T[2 * n]), "Tj": Tj, "Tj_max": max(Tj), "Tj_max_C": max(Tj) - T0C}


_add("pe_thermal_shared_heatsink",
     name="Several power devices on one heat sink", topic="thermal / steady state",
     purpose="The heat-sink temperature and every device's junction temperature when several devices, each with its own junction-to-case and case-to-sink resistance, share one heat sink to ambient.",
     inputs=[_vecin("P", "loss of each device in watts", 1, 12), _phys("Rjc", "thermal_resistance", "junction-to-case thermal resistance of each device"),
             _phys("Rcs", "thermal_resistance", "case-to-sink (interface) resistance of each device"), _phys("Rsa", "thermal_resistance", "sink-to-ambient resistance"),
             _phys("Ta", "temperature", "ambient temperature")],
     outputs=[_o("P_total", "power", "total loss"), _o("Ts", "temperature", "heat-sink temperature"), _o("Tj", "number", "junction temperature of each device, kelvin"),
              _o("Tj_max", "temperature", "hottest junction"), _o("Tj_max_C", "number", "hottest junction in degC")],
     primary="Tj_max", conditions=["steady state; the sink is isothermal"],
     method="Ts = Ta + Rsa sum(P); Tj,k = Ts + P_k (Rjc + Rcs)",
     latex=r"T_s = T_a + R_{sa}\sum_kP_k,\quad T_{j,k} = T_s + P_k(R_{jc}+R_{cs})",
     run=thermal_multi, reference=_ref_thermal_multi,
     wrong=lambda P, Rjc, Rcs, Rsa, Ta: (lambda Tj: _out(P_total=sum(P), Ts=Ta + Rsa * max(P), Tj=Tj, Tj_max=max(Tj), Tj_max_C=max(Tj) - T0C))([Ta + x * (Rjc + Rcs + Rsa) for x in P]),
     rand=lambda rng: {"P": [_sig(_lu(rng, 1.0, 100.0), 3) for _ in range(rng.randint(2, 6))], "Rjc": _sig(_lu(rng, 0.1, 2.0), 3), "Rcs": _sig(_lu(rng, 0.05, 0.5), 3),
                       "Rsa": _sig(_lu(rng, 0.05, 1.0), 3), "Ta": _sig(rng.uniform(288.15, 333.15), 5)},
     phrases=("Shared heat sink temperatures", "Devices on one heat sink"),
     examples=[_ex({"P": [20.0, 20.0, 15.0], "Rjc": 0.8, "Rcs": 0.2, "Rsa": 0.5, "Ta": 313.15}), _ex({"P": [50.0, 30.0], "Rjc": 0.4, "Rcs": 0.1, "Rsa": 0.3, "Ta": 298.15})],
     level="intermediate",
     extra_q=[{"q": "Shared heat sink temperatures: P = [20, 20, 15], Rjc = 0.8 degC/W, Rcs = 0.2 degC/W, Rsa = 0.5 degC/W, Ta = 40 degC", "like": 0}])


def _foster_lists(R, tau):
    _need(isinstance(R, list) and isinstance(tau, list) and 1 <= len(R) <= 6 and len(R) == len(tau),
          "give 1 to 6 thermal resistances and as many time constants")
    for x in R:
        _pos("every thermal resistance", x)
    for x in tau:
        _pos("every time constant", x)


def foster_pulse(R, tau, P, tp):
    _foster_lists(R, tau)
    _pos("P and tp", P, tp)
    Z = 0.0
    Rt = 0.0
    for k in range(len(R)):
        Z += R[k] * (1.0 - math.exp(-tp / tau[k]))
        Rt += R[k]
    return _out(Zth=Z, dTj=P * Z, Rth_total=Rt, dTj_steady=P * Rt)


def _ref_foster_pulse(R, tau, P, tp):
    from scipy.integrate import solve_ivp
    # each Foster element is a parallel RC (C = tau/R) carrying the same power; their temperature rises add
    C = [t / r for r, t in zip(R, tau)]
    sol = solve_ivp(lambda t, y: [(P - y[k] / R[k]) / C[k] for k in range(len(R))], (0.0, tp), [0.0] * len(R), method="DOP853", rtol=1e-13, atol=1e-16 * P * sum(R))
    dT = float(sum(sol.y[k][-1] for k in range(len(R))))
    return {"Zth": dT / P, "dTj": dT, "Rth_total": float(sum(R)), "dTj_steady": P * float(sum(R))}


_add("pe_foster_transient_thermal",
     name="Transient thermal impedance of a Foster network", topic="thermal / transient",
     purpose="The transient thermal impedance Zth(tp) of a device described by a Foster RC network (datasheet R_i, tau_i pairs) and the junction temperature rise for a single power pulse of duration tp, with the steady-state values.",
     inputs=[_vecin("R", "Foster thermal resistances in kelvin per watt", 1, 6), _vecin("tau", "Foster time constants in seconds", 1, 6),
             _phys("P", "power", "pulse power"), _phys("tp", "time", "pulse duration", show=_MS)],
     outputs=[_o("Zth", "thermal_resistance", "transient thermal impedance at tp"), _o("dTj", "temperature_difference", "junction temperature rise at the end of the pulse"),
              _o("Rth_total", "thermal_resistance", "steady-state thermal resistance"), _o("dTj_steady", "temperature_difference", "steady-state rise for continuous power")],
     primary="dTj", conditions=["linear network, starting from thermal equilibrium"],
     method="Zth(t) = sum R_i (1 - exp(-t/tau_i)); dTj = P Zth(tp)",
     latex=r"Z_{th}(t) = \sum_i R_i\left(1-e^{-t/\tau_i}\right),\quad \Delta T_j = P\,Z_{th}(t_p)",
     run=foster_pulse, reference=_ref_foster_pulse,
     wrong=lambda R, tau, P, tp: (lambda Z: _out(Zth=Z, dTj=P * Z, Rth_total=sum(R), dTj_steady=P * sum(R)))(sum(r * math.exp(-tp / t) for r, t in zip(R, tau))),
     rand=lambda rng: (lambda n: {"R": [_sig(_lu(rng, 0.01, 0.5), 3) for _ in range(n)], "tau": [_sig(_lu(rng, 1e-5, 1.0), 3) for _ in range(n)],
                                  "P": _sig(_lu(rng, 10.0, 2000.0), 3), "tp": _sig(_lu(rng, 1e-5, 1.0), 3)})(rng.randint(1, 4)),
     phrases=("Foster transient thermal impedance", "Transient thermal impedance"),
     examples=[_ex({"R": [0.05, 0.1, 0.15], "tau": [0.0001, 0.003, 0.05], "P": 500.0, "tp": 0.01}), _ex({"R": [0.2, 0.3], "tau": [0.001, 0.1], "P": 100.0, "tp": 0.02})],
     level="advanced")


def foster_periodic(R, tau, P, tp, T):
    _foster_lists(R, tau)
    _pos("P, tp and T", P, tp, T)
    _need(tp < T, "the pulse must be shorter than the period")
    pk = 0.0
    mn = 0.0
    Rt = 0.0
    for k in range(len(R)):
        a = -math.expm1(-tp / tau[k])
        b = -math.expm1(-T / tau[k])
        pk += P * R[k] * a / b
        mn += P * R[k] * a * math.exp(-(T - tp) / tau[k]) / b
        Rt += R[k]
    return _out(dT_peak=pk, dT_min=mn, dT_avg=P * Rt * tp / T, ripple=pk - mn)


def _ref_foster_periodic(R, tau, P, tp, T):
    import numpy as np
    from scipy.integrate import solve_ivp
    from scipy.linalg import expm
    n = len(R)
    C = [t / r for r, t in zip(R, tau)]
    A = np.diag([-1.0 / (R[k] * C[k]) for k in range(n)])
    rhs = lambda p: (lambda t, y: [(p - y[k] / R[k]) / C[k] for k in range(n)])       # noqa: E731
    # response over one period from zero (simulated), then the periodic steady state x0 = (I - Phi)^-1 x_T|0
    on = solve_ivp(rhs(P), (0.0, tp), [0.0] * n, method="DOP853", rtol=1e-13, atol=1e-18)
    off = solve_ivp(rhs(0.0), (tp, T), on.y[:, -1], method="DOP853", rtol=1e-13, atol=1e-18)
    Phi = expm(A * T)
    x0 = np.linalg.solve(np.eye(n) - Phi, off.y[:, -1])
    peak = expm(A * tp) @ x0 + on.y[:, -1]
    avg = _quad(lambda t: 1.0, 0.0, tp) * P * float(sum(R)) / T
    return {"dT_peak": float(peak.sum()), "dT_min": float(x0.sum()), "dT_avg": avg, "ripple": float(peak.sum() - x0.sum())}


_add("pe_foster_periodic_pulses",
     name="Junction temperature under periodic power pulses", topic="thermal / transient",
     purpose="The peak, minimum and average junction temperature rise and the temperature ripple of a device described by a Foster network under a periodic rectangular power pulse train (exact periodic steady state, no single-pulse approximation).",
     inputs=[_vecin("R", "Foster thermal resistances in kelvin per watt", 1, 6), _vecin("tau", "Foster time constants in seconds", 1, 6),
             _phys("P", "power", "pulse power"), _phys("tp", "time", "pulse duration", show=_MS), _phys("T", "time", "pulse period", show=_MS)],
     outputs=[_o("dT_peak", "temperature_difference", "peak temperature rise (end of each pulse)"), _o("dT_min", "temperature_difference", "minimum temperature rise (start of each pulse)"),
              _o("dT_avg", "temperature_difference", "average rise P Rth tp/T"), _o("ripple", "temperature_difference", "peak minus minimum")],
     primary="dT_peak", conditions=["periodic steady state", "tp < T"],
     method="per element: peak = P R (1 - e^(-tp/tau))/(1 - e^(-T/tau)); minimum = peak e^(-(T - tp)/tau); summed over the network",
     latex=r"\Delta T_{pk} = \sum_iPR_i\frac{1-e^{-t_p/\tau_i}}{1-e^{-T/\tau_i}}",
     run=foster_periodic, reference=_ref_foster_periodic,
     wrong=lambda R, tau, P, tp, T: (lambda pk: _out(dT_peak=pk, dT_min=0.0, dT_avg=P * sum(R) * tp / T, ripple=pk))(P * sum(r * (1 - math.exp(-tp / t)) for r, t in zip(R, tau))),
     rand=lambda rng: (lambda n, T: {"R": [_sig(_lu(rng, 0.01, 0.5), 3) for _ in range(n)], "tau": [_sig(_lu(rng, 1e-5, 1.0), 3) for _ in range(n)],
                                     "P": _sig(_lu(rng, 10.0, 2000.0), 3), "tp": _sig(T * rng.uniform(0.05, 0.9), 3), "T": T})(rng.randint(1, 4), _sig(_lu(rng, 1e-4, 1.0), 3)),
     phrases=("Periodic pulse junction temperature", "Foster network periodic pulses"),
     examples=[_ex({"R": [0.05, 0.1, 0.15], "tau": [0.0001, 0.003, 0.05], "P": 500.0, "tp": 0.002, "T": 0.01}),
               _ex({"R": [0.2, 0.3], "tau": [0.001, 0.1], "P": 100.0, "tp": 0.01, "T": 0.04})],
     level="expert")


# ================================================================================================ MAGNETICS
_WAVE = ("sine", "square")
_B = lambda s, m: _phys(s, "magnetic_field", m, show=_MT)        # noqa: E731
_AC = lambda s, m: _phys(s, "area", m)                           # noqa: E731
_CM3 = ("cm^3", 1e6)


def _kf(waveform):
    return math.pi * SQ2 if waveform == "sine" else 4.0


def transformer_turns(waveform, V, f, Bmax, Ac, Vs):
    _pos("V, f, Bmax, Ac and Vs", V, f, Bmax, Ac, Vs)
    _need(waveform in _WAVE, "waveform: sine or square")
    Np = V / (_kf(waveform) * f * Bmax * Ac)
    return _out(Np=Np, Ns=Np * Vs / V, n=V / Vs, volts_per_turn=V / Np)


def _ref_transformer_turns(waveform, V, f, Bmax, Ac, Vs):
    T = 1.0 / f
    v = (lambda t: SQ2 * V * math.sin(TWO_PI * f * t)) if waveform == "sine" else (lambda t: V)
    # Faraday: the positive half-cycle's volt-seconds swing the flux from -Bmax Ac to +Bmax Ac
    vs = _quad(v, 0.0, T / 2)
    Np = vs / (2 * Bmax * Ac)
    return {"Np": Np, "Ns": Np * Vs / V, "n": V / Vs, "volts_per_turn": V / Np}


_add("pe_transformer_turns",
     name="Transformer turns from Faraday's law", topic="magnetics / transformers",
     purpose="The primary and secondary turns of a transformer that keep the peak flux density at Bmax in a core of cross-section Ac (sine or square-wave excitation), the turns ratio and the volts per turn.",
     inputs=[_choice("waveform", _WAVE, "sine or square"), _V("V", "primary voltage (RMS for sine, amplitude for square)"),
             _F("f", "frequency"), _B("Bmax", "peak flux density"), _AC("Ac", "core cross-section area"), _V("Vs", "secondary voltage (same measure as V)")],
     outputs=[_o("Np", "number", "primary turns (before rounding up)"), _o("Ns", "number", "secondary turns (before rounding)"), _o("n", "number", "turns ratio Np/Ns"),
              _o("volts_per_turn", "voltage", "volts per turn")],
     primary="Np", conditions=["the core does not saturate; no dc bias"],
     method="Np = V/(Kf f Bmax Ac), Kf = 4.44 (sine, RMS) or 4 (square)",
     latex=r"N_p = \frac{V}{K_f f B_{max}A_c},\quad K_f = \sqrt2\pi\ (\text{sine}),\ 4\ (\text{square})",
     run=transformer_turns, reference=_ref_transformer_turns,
     wrong=lambda waveform, V, f, Bmax, Ac, Vs: (lambda r: dict(r, Np=_r(r["Np"] * 2), Ns=_r(r["Ns"] * 2), volts_per_turn=_r(r["volts_per_turn"] / 2)))(transformer_turns(waveform, V, f, Bmax, Ac, Vs)),
     rand=lambda rng: {"waveform": rng.choice(_WAVE), "V": _sig(rng.uniform(12.0, 400.0), 3), "f": _sig(_lu(rng, 50.0, 5e5), 3), "Bmax": _sig(rng.uniform(0.05, 1.5), 3),
                       "Ac": _sig(_lu(rng, 1e-5, 5e-3), 3), "Vs": _sig(rng.uniform(5.0, 400.0), 3)},
     phrases=("Transformer turns", "Transformer turns from flux density"),
     examples=[_ex({"waveform": "square", "V": 48.0, "f": 100e3, "Bmax": 0.15, "Ac": 1.25e-4, "Vs": 12.0}),
               _ex({"waveform": "sine", "V": 230.0, "f": 50.0, "Bmax": 1.2, "Ac": 1.0e-3, "Vs": 24.0})],
     level="beginner")


def gapped_inductor(L, Ipk, Bmax, Ac):
    _pos("L, Ipk, Bmax and Ac", L, Ipk, Bmax, Ac)
    N = L * Ipk / (Bmax * Ac)
    lg = MU0 * N * N * Ac / L
    return _out(N=N, l_gap=lg, E=0.5 * L * Ipk * Ipk, AL=L / (N * N), V_gap=lg * Ac)


def _ref_gapped_inductor(L, Ipk, Bmax, Ac):
    import sympy as sp
    from scipy.constants import mu_0
    N, g = sp.symbols("N g", positive=True)
    # magnetic circuit with the reluctance in the gap: L = mu0 N^2 Ac/g and the peak flux density mu0 N Ipk/g = Bmax
    s = sp.solve([sp.Float(mu_0, 30) * N ** 2 * _sq(Ac) / g - _sq(L), sp.Float(mu_0, 30) * N * _sq(Ipk) / g - _sq(Bmax)], [N, g], dict=True)[0]
    Nv, gv = float(s[N]), float(s[g])
    return {"N": Nv, "l_gap": gv, "E": _quad(lambda i: L * i, 0.0, Ipk), "AL": L / Nv ** 2, "V_gap": gv * Ac}


_add("pe_gapped_inductor_design",
     name="Gapped-core inductor: turns and air gap", topic="magnetics / inductors",
     purpose="The number of turns and the air-gap length of a gapped-core inductor that gives an inductance L with the peak flux density Bmax at the peak current (core reluctance and fringing neglected), the stored energy, the inductance factor AL and the gap volume.",
     inputs=[_L("L", "inductance"), _I("Ipk", "peak current"), _B("Bmax", "peak flux density allowed"), _AC("Ac", "core cross-section area")],
     outputs=[_o("N", "number", "turns (before rounding)"), _o("l_gap", "length", "air-gap length"), _o("E", "energy", "peak stored energy"),
              _o("AL", "inductance", "inductance per turn squared"), _o("V_gap", "volume", "gap volume")],
     primary="N", conditions=["gap reluctance dominates; no fringing"],
     method="N = L Ipk/(Bmax Ac); l_g = mu0 N^2 Ac/L; E = L Ipk^2/2",
     latex=r"N = \frac{LI_{pk}}{B_{max}A_c},\quad l_g = \frac{\mu_0N^2A_c}{L}",
     run=gapped_inductor, reference=_ref_gapped_inductor,
     wrong=lambda L, Ipk, Bmax, Ac: (lambda N: _out(N=N, l_gap=MU0 * N * N * Ac / L, E=0.5 * L * Ipk * Ipk, AL=L / (N * N), V_gap=MU0 * N * N * Ac * Ac / L))(L * Ipk / (2 * Bmax * Ac)),
     rand=lambda rng: {"L": _sig(_lu(rng, 1e-6, 1e-2), 3), "Ipk": _sig(_lu(rng, 0.5, 50.0), 3), "Bmax": _sig(rng.uniform(0.1, 0.4), 3), "Ac": _sig(_lu(rng, 2e-5, 1e-3), 3)},
     phrases=("Gapped inductor design", "Inductor turns and air gap"),
     examples=[_ex({"L": 100e-6, "Ipk": 10.0, "Bmax": 0.3, "Ac": 1.0e-4}), _ex({"L": 1e-3, "Ipk": 2.0, "Bmax": 0.25, "Ac": 0.5e-4})],
     level="intermediate")


def inductor_area_product(L, Ipk, Irms, Bmax, J, Ku):
    _pos("L, Ipk, Irms, Bmax and J", L, Ipk, Irms, Bmax, J)
    _frac("the window utilisation Ku", Ku)
    Ap = L * Ipk * Irms / (Bmax * J * Ku)
    return _out(Ap=Ap, Ap_cm4=Ap * 1e8)


def _ref_inductor_area_product(L, Ipk, Irms, Bmax, J, Ku):
    # any number of turns: the core must carry the flux L Ipk/N at Bmax and the window the copper N Irms/(Ku J)
    vals = []
    for N in (7.0, 31.0):
        Ac = L * Ipk / (N * Bmax)
        Wa = N * Irms / (Ku * J)
        vals.append(Ac * Wa)
    return {"Ap": vals[1], "Ap_cm4": vals[0] * 1e8}


_add("pe_inductor_area_product",
     name="Inductor core area product", topic="magnetics / core selection",
     purpose="The core area product Ap = Ac Wa an inductor needs to store its energy at the peak flux density Bmax with a current density J and window utilisation Ku (core selection by the area-product method).",
     inputs=[_L("L", "inductance"), _I("Ipk", "peak current"), _I("Irms", "RMS current"), _B("Bmax", "peak flux density"),
             _phys("J", "current_density", "winding current density"), _numin("Ku", "window utilisation factor (a fraction or %)", pct=True)],
     outputs=[_o("Ap", "area_product", "area product Ac Wa"), _o("Ap_cm4", "number", "area product in centimetres to the fourth")],
     primary="Ap", conditions=["0 < Ku < 1"],
     method="Ap = L Ipk Irms/(Bmax J Ku)",
     latex=r"A_p = A_cW_a = \frac{L\,I_{pk}I_{rms}}{B_{max}JK_u}",
     run=inductor_area_product, reference=_ref_inductor_area_product,
     wrong=lambda L, Ipk, Irms, Bmax, J, Ku: (lambda Ap: _out(Ap=Ap, Ap_cm4=Ap * 1e8))(L * Ipk * Ipk / (Bmax * J * Ku) * 0.5),
     rand=lambda rng: (lambda Ipk: {"L": _sig(_lu(rng, 1e-6, 1e-2), 3), "Ipk": Ipk, "Irms": _sig(Ipk * rng.uniform(0.5, 0.95), 3), "Bmax": _sig(rng.uniform(0.1, 0.4), 3),
                                    "J": _sig(rng.uniform(2e6, 6e6), 3), "Ku": round(rng.uniform(0.2, 0.6), 3)})(_sig(_lu(rng, 0.5, 50.0), 3)),
     phrases=("Inductor area product", "Inductor core area product"),
     examples=[_ex({"L": 100e-6, "Ipk": 12.0, "Irms": 10.0, "Bmax": 0.3, "J": 4e6, "Ku": 0.4}), _ex({"L": 2e-3, "Ipk": 3.0, "Irms": 2.5, "Bmax": 0.25, "J": 3e6, "Ku": 0.3})],
     level="intermediate",
     weak="the same area-product definition rebuilt from the core flux and window copper at two arbitrary turn counts")


def transformer_area_product(Po, eff, waveform, Bm, f, J, Ku):
    _pos("Po, Bm, f and J", Po, Bm, f, J)
    _frac("the efficiency", eff)
    _frac("the window utilisation Ku", Ku)
    _need(waveform in _WAVE, "waveform: sine or square")
    Pt = Po * (1.0 + 1.0 / eff)
    Ap = Pt / (_kf(waveform) * Ku * Bm * f * J)
    return _out(Pt=Pt, Ap=Ap, Ap_cm4=Ap * 1e8)


def _ref_transformer_area_product(Po, eff, waveform, Bm, f, J, Ku):
    # Kf from the waveform itself: RMS voltage over (frequency x half-cycle volt-seconds / 2) ... Kf = Vrms / (f * flux swing per turn)
    T = 1.0 / f
    v = (lambda t: math.sin(TWO_PI * f * t)) if waveform == "sine" else (lambda t: 1.0)
    vrms = math.sqrt(2 * _quad(lambda t: v(t) ** 2, 0.0, T / 2) / T)
    swing = _quad(v, 0.0, T / 2)                       # = 2 B A N for a unit-amplitude voltage
    Kf = vrms / (f * swing / 2)
    # arbitrary design: Vp = 100 V on Np = 10 turns; secondary 50 V on 5 turns; both windings at density J
    Vp, Np, Vsec, Ns = 100.0, 10.0, 50.0, 5.0
    Ac = Vp / (Kf * f * Np * Bm)
    Ip, Is = Po / (eff * Vp), Po / Vsec
    Wa = (Np * Ip + Ns * Is) / (Ku * J)
    return {"Pt": Vp * Ip + Vsec * Is, "Ap": Ac * Wa, "Ap_cm4": Ac * Wa * 1e8}


_add("pe_transformer_area_product",
     name="Transformer core area product", topic="magnetics / core selection",
     purpose="The apparent power the windings handle and the core area product Ap = Ac Wa a two-winding transformer needs for an output power at an efficiency, flux density, frequency, current density and window utilisation (sine or square-wave excitation).",
     inputs=[_phys("Po", "power", "output power"), _numin("eff", "efficiency (a fraction or %)", pct=True), _choice("waveform", _WAVE, "sine or square"),
             _B("Bm", "peak flux density"), _F("f", "frequency"), _phys("J", "current_density", "current density"), _numin("Ku", "window utilisation (a fraction or %)", pct=True)],
     outputs=[_o("Pt", "power", "apparent power of both windings"), _o("Ap", "area_product", "area product"), _o("Ap_cm4", "number", "area product in centimetres to the fourth")],
     primary="Ap", conditions=["two windings at the same current density"],
     method="Pt = Po (1 + 1/eff); Ap = Pt/(Kf Ku Bm f J), Kf = 4.44 (sine) or 4 (square)",
     latex=r"A_p = \frac{P_t}{K_fK_uB_mfJ},\quad P_t = P_o\left(1+\frac{1}{\eta}\right)",
     run=transformer_area_product, reference=_ref_transformer_area_product,
     wrong=lambda Po, eff, waveform, Bm, f, J, Ku: (lambda Ap: _out(Pt=2 * Po, Ap=Ap, Ap_cm4=Ap * 1e8))(2 * Po / (_kf(waveform) * Ku * Bm * f * J)),
     rand=lambda rng: {"Po": _sig(_lu(rng, 5.0, 5000.0), 3), "eff": round(rng.uniform(0.8, 0.98), 3), "waveform": rng.choice(_WAVE), "Bm": _sig(rng.uniform(0.05, 1.4), 3),
                       "f": _sig(_lu(rng, 50.0, 5e5), 3), "J": _sig(rng.uniform(2e6, 6e6), 3), "Ku": round(rng.uniform(0.2, 0.5), 3)},
     phrases=("Transformer area product", "Transformer core area product"),
     examples=[_ex({"Po": 250.0, "eff": 0.95, "waveform": "square", "Bm": 0.1, "f": 100e3, "J": 4e6, "Ku": 0.4}),
               _ex({"Po": 500.0, "eff": 0.9, "waveform": "sine", "Bm": 1.2, "f": 50.0, "J": 3e6, "Ku": 0.35})],
     level="advanced",
     extra_q=[{"q": "Transformer area product: Po = 250 W, eff = 95%, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 40%", "like": 0}])


def _igse_int(alpha):
    """Integral of |cos t|^alpha over a full period (closed form with the gamma function)."""
    return 2.0 * math.sqrt(math.pi) * math.gamma((alpha + 1.0) / 2.0) / math.gamma(alpha / 2.0 + 1.0)


def steinmetz(k, alpha, beta, f, Bpk, Vc):
    _pos("k, alpha, beta, f, Bpk and Vc", k, alpha, beta, f, Bpk, Vc)
    Pv = k * f ** alpha * Bpk ** beta
    return _out(Pv=Pv, Pv_mW_cm3=Pv * 1e-3, P=Pv * Vc)


def _ref_steinmetz(k, alpha, beta, f, Bpk, Vc):
    # improved generalized Steinmetz equation (iGSE) evaluated for the sinusoidal flux: it reduces to the original
    # equation, so a correct implementation of k f^a B^b must agree with this time-domain integral
    ki = k / ((TWO_PI) ** (alpha - 1) * _quad(lambda t: abs(math.cos(t)) ** alpha, 0.0, TWO_PI, points=[math.pi / 2, 3 * math.pi / 2]) * 2 ** (beta - alpha))
    T = 1.0 / f
    w = TWO_PI * f
    dB = 2 * Bpk
    Pv = _quad(lambda t: ki * abs(Bpk * w * math.cos(w * t)) ** alpha * dB ** (beta - alpha), 0.0, T, points=[T / 4, 3 * T / 4]) / T
    return {"Pv": Pv, "Pv_mW_cm3": Pv * 1e-3, "P": Pv * Vc}


_add("pe_steinmetz_core_loss",
     name="Core loss by the Steinmetz equation", topic="magnetics / core loss",
     purpose="The core loss density and total core loss of a magnetic core under sinusoidal flux from the Steinmetz parameters k, alpha, beta (SI: f in Hz, B in T, loss density in W/m^3), the frequency, the peak flux density and the core volume.",
     inputs=[_numin("k", "Steinmetz coefficient (SI units)"), _numin("alpha", "frequency exponent"), _numin("beta", "flux-density exponent"), _F("f", "frequency"),
             _B("Bpk", "peak flux density"), _phys("Vc", "volume", "core volume", show=_CM3)],
     outputs=[_o("Pv", "core_loss_density", "core loss density"), _o("Pv_mW_cm3", "number", "core loss density in milliwatts per cubic centimetre"), _o("P", "power", "core loss")],
     primary="Pv", conditions=["sinusoidal flux, within the parameters' fitted range"],
     method="Pv = k f^alpha Bpk^beta; P = Pv Vc",
     latex=r"P_v = k f^{\alpha} \hat B^{\beta}",
     run=steinmetz, reference=_ref_steinmetz,
     wrong=lambda k, alpha, beta, f, Bpk, Vc: (lambda Pv: _out(Pv=Pv, Pv_mW_cm3=Pv * 1e-3, P=Pv * Vc))(k * f ** alpha * (2 * Bpk) ** beta),
     rand=lambda rng: {"k": _sig(_lu(rng, 0.5, 30.0), 3), "alpha": _sig(rng.uniform(1.1, 1.8), 3), "beta": _sig(rng.uniform(2.0, 3.0), 3), "f": _sig(_lu(rng, 1e3, 5e5), 3),
                       "Bpk": _sig(rng.uniform(0.02, 0.3), 3), "Vc": _sig(_lu(rng, 1e-7, 1e-4), 3)},
     phrases=("Steinmetz core loss", "Core loss Steinmetz"),
     examples=[_ex({"k": 3.2, "alpha": 1.4, "beta": 2.5, "f": 100e3, "Bpk": 0.1, "Vc": 5e-6}), _ex({"k": 10.0, "alpha": 1.3, "beta": 2.7, "f": 20e3, "Bpk": 0.2, "Vc": 2e-5})],
     level="beginner")


def igse_triangular(k, alpha, beta, f, dB, D, Vc):
    _pos("k, alpha, beta, f, dB and Vc", k, alpha, beta, f, dB, Vc)
    _frac("the duty cycle D", D)
    ki = k / (TWO_PI ** (alpha - 1.0) * _igse_int(alpha) * 2.0 ** (beta - alpha))
    Pv = ki * dB ** beta * f ** alpha * (D ** (1.0 - alpha) + (1.0 - D) ** (1.0 - alpha))
    sine = k * f ** alpha * (dB / 2.0) ** beta
    return _out(ki=ki, Pv=Pv, P=Pv * Vc, ratio_to_sine=Pv / sine)


def _ref_igse_triangular(k, alpha, beta, f, dB, D, Vc):
    ki = k / ((TWO_PI) ** (alpha - 1) * _quad(lambda t: abs(math.cos(t)) ** alpha, 0.0, TWO_PI, points=[math.pi / 2, 3 * math.pi / 2]) * 2 ** (beta - alpha))
    T = 1.0 / f
    # triangular flux: rises by dB in D T, falls in (1 - D) T; iGSE integrand ki |dB/dt|^alpha dB^(beta - alpha)
    up = _quad(lambda t: ki * (dB / (D * T)) ** alpha * dB ** (beta - alpha), 0.0, D * T)
    dn = _quad(lambda t: ki * (dB / ((1 - D) * T)) ** alpha * dB ** (beta - alpha), D * T, T)
    Pv = (up + dn) / T
    return {"ki": ki, "Pv": Pv, "P": Pv * Vc, "ratio_to_sine": Pv / (k * f ** alpha * (dB / 2) ** beta)}


_add("pe_igse_triangular_core_loss",
     name="Core loss under triangular flux (iGSE)", topic="magnetics / core loss",
     purpose="The core loss of a magnetic component whose flux is triangular (a converter inductor or transformer with duty cycle D) by the improved generalized Steinmetz equation, from the sinusoidal Steinmetz parameters; also its ratio to the sinusoidal loss at the same peak flux.",
     inputs=[_numin("k", "Steinmetz coefficient (SI units)"), _numin("alpha", "frequency exponent"), _numin("beta", "flux-density exponent"), _F("f", "switching frequency"),
             _B("dB", "peak-to-peak flux density swing"), _numin("D", "duty cycle of the rising flux (a fraction or %)", pct=True), _phys("Vc", "volume", "core volume", show=_CM3)],
     outputs=[_o("ki", "number", "iGSE coefficient"), _o("Pv", "core_loss_density", "core loss density"), _o("P", "power", "core loss"),
              _o("ratio_to_sine", "number", "loss relative to a sinusoid of the same peak flux")],
     primary="Pv", conditions=["piecewise-linear flux, no relaxation effects"],
     method="ki = k/((2 pi)^(alpha-1) int|cos|^alpha 2^(beta-alpha)); Pv = ki dB^beta f^alpha (D^(1-alpha) + (1-D)^(1-alpha))",
     latex=r"P_v = k_i(\Delta B)^{\beta}f^{\alpha}\left(D^{1-\alpha}+(1-D)^{1-\alpha}\right),\ k_i = \frac{k}{(2\pi)^{\alpha-1}\int_0^{2\pi}|\cos\theta|^{\alpha}2^{\beta-\alpha}d\theta}",
     run=igse_triangular, reference=_ref_igse_triangular,
     wrong=lambda k, alpha, beta, f, dB, D, Vc: (lambda Pv: _out(ki=k, Pv=Pv, P=Pv * Vc, ratio_to_sine=1.0))(k * f ** alpha * (dB / 2) ** beta),
     rand=lambda rng: {"k": _sig(_lu(rng, 0.5, 30.0), 3), "alpha": _sig(rng.uniform(1.1, 1.8), 3), "beta": _sig(rng.uniform(2.0, 3.0), 3), "f": _sig(_lu(rng, 1e4, 5e5), 3),
                       "dB": _sig(rng.uniform(0.02, 0.4), 3), "D": round(rng.uniform(0.1, 0.9), 3), "Vc": _sig(_lu(rng, 1e-7, 1e-4), 3)},
     phrases=("iGSE core loss", "Triangular flux core loss"),
     examples=[_ex({"k": 3.2, "alpha": 1.4, "beta": 2.5, "f": 100e3, "dB": 0.2, "D": 0.5, "Vc": 5e-6}), _ex({"k": 3.2, "alpha": 1.4, "beta": 2.5, "f": 100e3, "dB": 0.2, "D": 0.2, "Vc": 5e-6})],
     level="expert")


def _bessel01(z):
    """J0 and J1 of a complex argument by their power series (|z| <= 15 here)."""
    q = -(z * z) / 4.0
    t0 = complex(1.0, 0.0)
    t1 = z / 2.0
    s0, s1 = t0, t1
    m = 0
    while m < 200:
        m += 1
        t0 = t0 * q / (m * m)
        t1 = t1 * q / (m * (m + 1))
        s0 += t0
        s1 += t1
        if abs(t0.real) + abs(t0.imag) + abs(t1.real) + abs(t1.imag) < 1e-18 * (abs(s0.real) + abs(s0.imag) + abs(s1.real) + abs(s1.imag)):
            break
    return s0, s1


def skin_effect(d, f, rho, length):
    _pos("d, f, rho and length", d, f, rho, length)
    delta = math.sqrt(rho / (math.pi * f * MU0))
    r = d / 2.0
    _need(r / delta <= 10.0, "the wire radius exceeds 10 skin depths (outside the range of this series evaluation)")
    z = complex(1.0, -1.0) * (r / delta)
    j0, j1 = _bessel01(z)
    ratio = ((z / 2.0) * j0 / j1).real
    Rdc = rho * length / (math.pi * r * r)
    return _out(delta=delta, d_over_delta=d / delta, Rdc=Rdc, Rac=Rdc * ratio, ratio=ratio)


def _ref_skin_effect(d, f, rho, length):
    from scipy.constants import mu_0
    from scipy.special import jv
    delta = math.sqrt(rho / (math.pi * f * mu_0))
    r = d / 2
    # current density in a round wire J(r) ~ J0(k r), k = (1 - j)/delta; internal impedance per length k J0/(2 pi r sigma J1)
    k = (1 - 1j) / delta
    Z = k * rho * complex(jv(0, k * r)) / (TWO_PI * r * complex(jv(1, k * r)))
    Rdc = rho * length / (math.pi * r * r)
    return {"delta": delta, "d_over_delta": d / delta, "Rdc": Rdc, "Rac": Z.real * length, "ratio": Z.real * length / Rdc}


_add("pe_skin_effect_round_wire",
     name="Skin effect in a round wire", topic="magnetics / winding losses",
     purpose="The skin depth and the exact AC-to-DC resistance ratio (Bessel-function solution) of an isolated round conductor at a frequency, with its DC and AC resistance for a length.",
     inputs=[_phys("d", "length", "wire diameter", show=_MM), _F("f", "frequency"), _phys("rho", "resistivity", "conductor resistivity"),
             _phys("length", "length", "conductor length")],
     outputs=[_o("delta", "length", "skin depth"), _o("d_over_delta", "number", "diameter in skin depths"), _o("Rdc", "resistance", "DC resistance"),
              _o("Rac", "resistance", "AC resistance"), _o("ratio", "number", "Rac/Rdc")],
     primary="ratio", conditions=["isolated straight wire (no proximity effect)", "radius at most 10 skin depths"],
     method="delta = sqrt(rho/(pi f mu0)); Rac/Rdc = Re[(k r/2) J0(k r)/J1(k r)], k = (1 - j)/delta",
     latex=r"\delta = \sqrt{\frac{\rho}{\pi f\mu_0}},\quad \frac{R_{ac}}{R_{dc}} = \mathrm{Re}\left[\frac{kr}{2}\frac{J_0(kr)}{J_1(kr)}\right],\ k = \frac{1-j}{\delta}",
     run=skin_effect, reference=_ref_skin_effect,
     wrong=lambda d, f, rho, length: (lambda r: dict(r, ratio=_r(max(1.0, r["d_over_delta"] / 4)), Rac=_r(r["Rdc"] * max(1.0, r["d_over_delta"] / 4))))(skin_effect(d, f, rho, length)),
     rand=lambda rng: (lambda f: {"d": _sig(math.sqrt(1.72e-8 / (math.pi * f * MU0)) * rng.uniform(0.5, 15.0), 3), "f": f, "rho": rng.choice([1.72e-8, 2.82e-8, 1.68e-8]),
                                  "length": _sig(_lu(rng, 0.1, 100.0), 3)})(_sig(_lu(rng, 1e3, 1e6), 3)),
     phrases=("Skin effect in round wire", "Round wire skin effect"),
     examples=[_ex({"d": 1.0e-3, "f": 100e3, "rho": 1.72e-8, "length": 1.0}), _ex({"d": 0.5e-3, "f": 50e3, "rho": 1.72e-8, "length": 2.0})],
     level="advanced")


def dowell(h, f, rho, m):
    _pos("h, f and rho", h, f, rho)
    _need(isinstance(m, int) and 1 <= m <= 30, "the number of layers m must be a whole number from 1 to 30")
    delta = math.sqrt(rho / (math.pi * f * MU0))
    D = h / delta
    _need(D <= 20.0, "the layer is thicker than 20 skin depths (outside the range of this evaluation)")
    a = (math.sinh(2 * D) + math.sin(2 * D)) / (math.cosh(2 * D) - math.cos(2 * D))
    b = (math.sinh(D) - math.sin(D)) / (math.cosh(D) + math.cos(D))
    FR = D * (a + 2.0 * (m * m - 1) / 3.0 * b)
    return _out(delta=delta, Delta=D, FR=FR)


def _ref_dowell(h, f, rho, m):
    import numpy as np
    from scipy.constants import mu_0
    delta = math.sqrt(rho / (math.pi * f * mu_0))
    # one-dimensional field solution of a layer: complex surface impedance terms coth(alpha h) and tanh(alpha h / 2)
    x = (1 + 1j) * h / delta
    FR = (x * (1 / np.tanh(x) + 2 * (m * m - 1) / 3 * np.tanh(x / 2))).real
    return {"delta": delta, "Delta": h / delta, "FR": float(FR)}


_add("pe_dowell_proximity_factor",
     name="Winding AC resistance factor (Dowell)", topic="magnetics / winding losses",
     purpose="The ratio of AC to DC resistance of a winding of m layers of foil (or equivalent layer) thickness h at a frequency, including skin and proximity effects by Dowell's one-dimensional solution.",
     inputs=[_phys("h", "length", "layer (conductor) thickness", show=_MM), _F("f", "frequency"), _phys("rho", "resistivity", "conductor resistivity"), _intin("m", "number of layers", 1, 30)],
     outputs=[_o("delta", "length", "skin depth"), _o("Delta", "number", "layer thickness in skin depths h/delta"), _o("FR", "number", "AC-to-DC resistance factor")],
     primary="FR", conditions=["full-width layers (foil or porosity-corrected)", "sinusoidal current"],
     method="F_R = D [ (sinh 2D + sin 2D)/(cosh 2D - cos 2D) + 2 (m^2 - 1)/3 (sinh D - sin D)/(cosh D + cos D) ], D = h/delta",
     latex=r"F_R = \Delta\left[\frac{\sinh2\Delta+\sin2\Delta}{\cosh2\Delta-\cos2\Delta} + \frac{2(m^2-1)}{3}\frac{\sinh\Delta-\sin\Delta}{\cosh\Delta+\cos\Delta}\right]",
     run=dowell, reference=_ref_dowell,
     wrong=lambda h, f, rho, m: (lambda r: dict(r, FR=_r(r["Delta"] * ((math.sinh(2 * r["Delta"]) + math.sin(2 * r["Delta"])) / (math.cosh(2 * r["Delta"]) - math.cos(2 * r["Delta"])) + 2 * (m * m - 1) * (math.sinh(r["Delta"]) - math.sin(r["Delta"])) / (math.cosh(r["Delta"]) + math.cos(r["Delta"]))))))(dowell(h, f, rho, m)),
     rand=lambda rng: (lambda f: {"h": _sig(math.sqrt(1.72e-8 / (math.pi * f * MU0)) * rng.uniform(0.2, 5.0), 3), "f": f, "rho": rng.choice([1.72e-8, 2.82e-8]), "m": rng.randint(2, 10)})(_sig(_lu(rng, 1e3, 1e6), 3)),
     phrases=("Dowell AC resistance factor", "Winding proximity effect"),
     examples=[_ex({"h": 0.2e-3, "f": 100e3, "rho": 1.72e-8, "m": 4}), _ex({"h": 0.1e-3, "f": 200e3, "rho": 1.72e-8, "m": 6})],
     level="expert")


# ================================================================================================ RESONANT AND BIDIRECTIONAL CONVERTERS
def series_resonant(Lr, Cr, R, f):
    _pos("Lr, Cr, R and f", Lr, Cr, R, f)
    w0 = 1.0 / math.sqrt(Lr * Cr)
    fr = w0 / TWO_PI
    Z0 = math.sqrt(Lr / Cr)
    Q = Z0 / R
    fn = f / fr
    M = 1.0 / math.sqrt(1.0 + Q * Q * (fn - 1.0 / fn) ** 2)
    return _out(fr=fr, Z0=Z0, Q=Q, fn=fn, M=M)


def _ref_series_resonant(Lr, Cr, R, f):
    from scipy.signal import freqs
    # tank transfer function from the source to the ac load: H(s) = R/(R + s Lr + 1/(s Cr)) = s Cr R/(s^2 Lr Cr + s Cr R + 1)
    _, h = freqs([Cr * R, 0.0], [Lr * Cr, Cr * R, 1.0], worN=[TWO_PI * f])
    w0 = _brentq(lambda w: w * Lr - 1 / (w * Cr), 1e-3, 1e15)           # resonance: the reactances cancel
    return {"fr": w0 / TWO_PI, "Z0": w0 * Lr, "Q": w0 * Lr / R, "fn": f * TWO_PI / w0, "M": float(abs(h[0]))}


_add("pe_series_resonant_gain",
     name="Series resonant converter: tank gain (first harmonic)", topic="resonant converters / series resonant",
     purpose="The resonant frequency, characteristic impedance, quality factor, normalised frequency and voltage gain of a series resonant tank driving an equivalent AC load resistance (first-harmonic approximation).",
     inputs=[_L("Lr", "resonant inductance"), _phys("Cr", "capacitance", "resonant capacitance", show=_NF), _R("R", "equivalent AC load resistance"), _F("f", "switching frequency")],
     outputs=[_o("fr", "frequency", "resonant frequency"), _o("Z0", "resistance", "characteristic impedance sqrt(Lr/Cr)"), _o("Q", "number", "quality factor"),
              _o("fn", "number", "normalised frequency f/fr"), _o("M", "number", "tank voltage gain")],
     primary="M", conditions=["first-harmonic approximation"],
     method="fr = 1/(2 pi sqrt(Lr Cr)); Q = sqrt(Lr/Cr)/R; M = 1/sqrt(1 + Q^2 (fn - 1/fn)^2)",
     latex=r"M = \frac{1}{\sqrt{1+Q^2\left(f_n-\frac{1}{f_n}\right)^2}},\quad Q = \frac{\sqrt{L_r/C_r}}{R}",
     run=series_resonant, reference=_ref_series_resonant,
     wrong=lambda Lr, Cr, R, f: (lambda r: dict(r, M=_r(1 / (1 + r["Q"] * abs(r["fn"] - 1 / r["fn"])))))(series_resonant(Lr, Cr, R, f)),
     rand=lambda rng: (lambda Lr, Cr: {"Lr": Lr, "Cr": Cr, "R": _sig(math.sqrt(Lr / Cr) / _lu(rng, 0.2, 5.0), 3), "f": _sig(rng.uniform(0.5, 2.0) / (TWO_PI * math.sqrt(Lr * Cr)), 3)})(
         _sig(_lu(rng, 1e-6, 1e-3), 3), _sig(_lu(rng, 1e-9, 1e-6), 3)),
     phrases=("Series resonant gain", "Series resonant converter gain"),
     examples=[_ex({"Lr": 100e-6, "Cr": 25e-9, "R": 30.0, "f": 120e3}), _ex({"Lr": 50e-6, "Cr": 100e-9, "R": 10.0, "f": 60e3})],
     level="advanced")


def llc_gain(Lr, Cr, Lm, n, R, f):
    _pos("Lr, Cr, Lm, n, R and f", Lr, Cr, Lm, n, R, f)
    Rac = 8.0 * n * n * R / (math.pi * math.pi)
    w = TWO_PI * f
    zp = complex(0.0, w * Lm) * Rac / complex(Rac, w * Lm)
    zs = complex(0.0, w * Lr - 1.0 / (w * Cr))
    h = zp / (zs + zp)
    return _out(fr1=1.0 / (TWO_PI * math.sqrt(Lr * Cr)), fr2=1.0 / (TWO_PI * math.sqrt((Lr + Lm) * Cr)), Rac=Rac, Q=math.sqrt(Lr / Cr) / Rac,
                k=Lm / Lr, M=math.hypot(h.real, h.imag))


def _ref_llc_gain(Lr, Cr, Lm, n, R, f):
    from scipy.signal import freqs
    Rac = 8 * n * n * R / math.pi ** 2
    # H(s) = Zp/(Zs + Zp), Zp = s Lm Rac/(Rac + s Lm), Zs = s Lr + 1/(s Cr): a third-order rational function
    num = [Lm * Rac * Cr, 0.0, 0.0]
    den = [Lr * Cr * Lm, Lr * Cr * Rac + Lm * Rac * Cr, Lm, Rac]
    _, h = freqs(num, den, worN=[TWO_PI * f])
    w1 = _brentq(lambda w: w * Lr - 1 / (w * Cr), 1e-3, 1e15)
    w2 = _brentq(lambda w: w * (Lr + Lm) - 1 / (w * Cr), 1e-3, 1e15)
    return {"fr1": w1 / TWO_PI, "fr2": w2 / TWO_PI, "Rac": Rac, "Q": w1 * Lr / Rac, "k": Lm / Lr, "M": float(abs(h[0]))}


_add("pe_llc_resonant_gain",
     name="LLC resonant converter: tank gain (first harmonic)", topic="resonant converters / LLC",
     purpose="The two resonant frequencies, the reflected AC load resistance, quality factor, inductance ratio and voltage gain of an LLC resonant tank at a switching frequency (first-harmonic approximation, full-wave rectified output).",
     inputs=[_L("Lr", "series resonant inductance"), _phys("Cr", "capacitance", "resonant capacitance", show=_NF), _L("Lm", "magnetizing inductance"),
             _numin("n", "transformer turns ratio Np/Ns"), _R("R", "dc load resistance"), _F("f", "switching frequency")],
     outputs=[_o("fr1", "frequency", "series resonant frequency"), _o("fr2", "frequency", "lower resonant frequency"),
              _o("Rac", "resistance", "reflected AC load resistance"), _o("Q", "number", "quality factor sqrt(Lr/Cr)/Rac"), _o("k", "number", "inductance ratio Lm/Lr"),
              _o("M", "number", "tank voltage gain")],
     primary="M", conditions=["first-harmonic approximation"],
     method="Rac = 8 n^2 R/pi^2; M = |Zp/(Zs + Zp)|, Zp = jwLm || Rac, Zs = jwLr + 1/(jwCr)",
     latex=r"M = \left|\frac{j\omega L_m\parallel R_{ac}}{j\omega L_r + \frac{1}{j\omega C_r} + j\omega L_m\parallel R_{ac}}\right|,\quad R_{ac} = \frac{8n^2R}{\pi^2}",
     run=llc_gain, reference=_ref_llc_gain,
     wrong=lambda Lr, Cr, Lm, n, R, f: (lambda Rac: (lambda zp, zs: dict(llc_gain(Lr, Cr, Lm, n, R, f), Rac=_r(Rac), M=_r(abs(zp / (zs + zp)))))(
         complex(0, TWO_PI * f * Lm) * Rac / complex(Rac, TWO_PI * f * Lm), complex(0, TWO_PI * f * Lr - 1 / (TWO_PI * f * Cr))))(n * n * R),
     rand=lambda rng: (lambda Lr, Cr: {"Lr": Lr, "Cr": Cr, "Lm": _sig(Lr * rng.uniform(3.0, 10.0), 3), "n": _sig(rng.uniform(0.5, 20.0), 3),
                                       "R": _sig(math.sqrt(Lr / Cr) / _lu(rng, 0.1, 1.5) * math.pi ** 2 / 8, 3), "f": _sig(rng.uniform(0.5, 1.8) / (TWO_PI * math.sqrt(Lr * Cr)), 3)})(
         _sig(_lu(rng, 1e-6, 2e-4), 3), _sig(_lu(rng, 1e-9, 1e-6), 3)),
     phrases=("LLC resonant gain", "LLC converter gain"),
     examples=[_ex({"Lr": 60e-6, "Cr": 27e-9, "Lm": 300e-6, "n": 8.0, "R": 2.0, "f": 100e3}), _ex({"Lr": 20e-6, "Cr": 100e-9, "Lm": 120e-6, "n": 1.0, "R": 20.0, "f": 80e3})],
     level="expert")


def dab_power(V1, V2, n, L, fs, phi):
    _pos("V1, V2, n, L and fs", V1, V2, n, L, fs)
    _need(0 < phi <= math.pi / 2, "the phase shift must lie in (0, 90] degrees")
    V2p = n * V2
    wL = TWO_PI * fs * L
    P = V1 * V2p * phi * (math.pi - phi) / (TWO_PI * math.pi * fs * L)
    i0 = -(V1 * math.pi + V2p * (2.0 * phi - math.pi)) / (2.0 * wL)
    ip = i0 + (V1 + V2p) * phi / wL
    ms = (phi * (i0 * i0 + i0 * ip + ip * ip) + (math.pi - phi) * (ip * ip - ip * i0 + i0 * i0)) / (3.0 * math.pi)
    return _out(P=P, P_max=V1 * V2p / (8.0 * fs * L), i0=i0, i_phi=ip, I_rms=math.sqrt(ms))


def _ref_dab_power(V1, V2, n, L, fs, phi):
    V2p = n * V2
    wL = TWO_PI * fs * L

    def sim(ph):
        # angle domain over half a period: bridge 1 at +V1; bridge 2 (lagging by ph) at -V2' then +V2'
        segs = [(ph, V1 + V2p), (math.pi - ph, V1 - V2p)]
        rel = [0.0]
        for th, v in segs:
            rel.append(rel[-1] + _quad(lambda x, v=v: v / wL, 0.0, th))
        i0 = -rel[-1] / 2.0                               # half-wave symmetry i(pi) = -i(0)
        cur = [i0 + x for x in rel]
        seg_i = lambda k: (lambda x: cur[k] + segs[k][1] * x / wL)   # noqa: E731
        P = V1 * sum(_quad(seg_i(k), 0.0, segs[k][0]) for k in range(2)) / math.pi
        ms = sum(_quad(lambda x, k=k: seg_i(k)(x) ** 2, 0.0, segs[k][0]) for k in range(2)) / math.pi
        return P, cur, ms
    P, cur, ms = sim(phi)
    return {"P": P, "P_max": sim(math.pi / 2)[0], "i0": cur[0], "i_phi": cur[1], "I_rms": math.sqrt(ms)}


_add("pe_dual_active_bridge_power",
     name="Dual active bridge: power transfer and inductor current", topic="resonant converters / dual active bridge",
     purpose="The power transferred by a dual-active-bridge converter with single phase-shift modulation, the maximum transferable power, the inductor current at the switching instants of both bridges and its RMS value.",
     inputs=[_V("V1", "primary dc voltage"), _V("V2", "secondary dc voltage"), _numin("n", "turns ratio Np/Ns"), _L("L", "series (leakage) inductance referred to the primary"),
             _F("fs", "switching frequency"), _ANG("phi", "phase shift of the secondary bridge")],
     outputs=[_o("P", "power", "transferred power"), _o("P_max", "power", "maximum transferable power"), _o("i0", "current", "inductor current when the primary bridge switches"),
              _o("i_phi", "current", "inductor current when the secondary bridge switches"), _o("I_rms", "current", "RMS inductor current")],
     primary="P", conditions=["square-wave bridges, single phase shift 0 < phi <= 90 degrees"],
     method="P = n V1 V2 phi (pi - phi)/(2 pi^2 fs L); i(0) = -(V1 pi + nV2 (2 phi - pi))/(2 w L); i(phi) = i(0) + (V1 + nV2) phi/(w L)",
     latex=r"P = \frac{nV_1V_2\,\phi(\pi-\phi)}{2\pi^2f_sL},\quad P_{max} = \frac{nV_1V_2}{8f_sL}",
     run=dab_power, reference=_ref_dab_power,
     wrong=lambda V1, V2, n, L, fs, phi: (lambda r: dict(r, P=_r(n * V1 * V2 * math.sin(phi) / (TWO_PI * fs * L))))(dab_power(V1, V2, n, L, fs, phi)),
     rand=lambda rng: {"V1": _sig(rng.uniform(24.0, 800.0), 3), "V2": _sig(rng.uniform(12.0, 800.0), 3), "n": _sig(rng.uniform(0.5, 5.0), 3), "L": _sig(_lu(rng, 1e-6, 1e-4), 3),
                       "fs": _sig(_lu(rng, 2e4, 2e5), 3), "phi": math.radians(round(rng.uniform(5.0, 90.0), 1))},
     phrases=("Dual active bridge power", "DAB power transfer"),
     examples=[_ex({"V1": 400.0, "V2": 48.0, "n": 8.0, "L": 50e-6, "fs": 100e3, "phi": math.radians(30.0)}), _ex({"V1": 200.0, "V2": 200.0, "n": 1.0, "L": 20e-6, "fs": 50e3, "phi": math.radians(60.0)})],
     level="expert")


# ================================================================================================ FRONT ENDS
def hold_up(Po, t_hold, V1, V2):
    _pos("Po, t_hold, V1 and V2", Po, t_hold, V1, V2)
    _need(V2 < V1, "the minimum voltage V2 must be below the starting voltage V1")
    C = 2.0 * Po * t_hold / (V1 * V1 - V2 * V2)
    return _out(C=C, E_used=Po * t_hold, E_initial=0.5 * C * V1 * V1, E_remaining=0.5 * C * V2 * V2)


def _ref_hold_up(Po, t_hold, V1, V2):
    from scipy.integrate import solve_ivp
    # constant-power discharge C dv/dt = -Po/v, simulated with C = 1 F: the time to fall from V1 to V2 scales with C
    ev = lambda t, y: y[0] - V2                                              # noqa: E731
    ev.terminal, ev.direction = True, -1
    sol = solve_ivp(lambda t, y: [-Po / y[0]], (0.0, 10 * V1 * V1 / Po), [V1], method="DOP853", rtol=1e-13, atol=1e-16 * V1, events=ev)
    C = t_hold / float(sol.t_events[0][0])
    return {"C": C, "E_used": _quad(lambda t: Po, 0.0, t_hold), "E_initial": _quad(lambda v: C * v, 0.0, V1), "E_remaining": _quad(lambda v: C * v, 0.0, V2)}


_add("pe_hold_up_capacitor",
     name="Hold-up (bulk) capacitor", topic="front ends / energy storage",
     purpose="The bulk capacitance that keeps a constant-power load running for a hold-up time while the capacitor voltage falls from V1 to the minimum operating voltage V2, with the energies involved.",
     inputs=[_phys("Po", "power", "load power drawn from the capacitor"), _phys("t_hold", "time", "hold-up time", show=_MS), _V("V1", "starting capacitor voltage"),
             _V("V2", "minimum operating voltage")],
     outputs=[_o("C", "capacitance", "required capacitance"), _o("E_used", "energy", "energy delivered Po t"), _o("E_initial", "energy", "energy stored at the starting voltage"),
              _o("E_remaining", "energy", "energy left at the minimum voltage")],
     primary="C", conditions=["constant-power load", "V2 < V1"],
     method="C = 2 Po t_hold/(V1^2 - V2^2)",
     latex=r"C = \frac{2P_ot_{hold}}{V_1^2 - V_2^2}",
     run=hold_up, reference=_ref_hold_up,
     wrong=lambda Po, t_hold, V1, V2: (lambda C: _out(C=C, E_used=Po * t_hold, E_initial=0.5 * C * V1 * V1, E_remaining=0.5 * C * V2 * V2))(Po * t_hold / (V1 * V1 - V2 * V2)),
     rand=lambda rng: (lambda V1: {"Po": _sig(_lu(rng, 1.0, 3000.0), 3), "t_hold": _sig(_lu(rng, 1e-3, 0.1), 3), "V1": V1, "V2": _sig(V1 * rng.uniform(0.3, 0.9), 3)})(_sig(rng.uniform(12.0, 450.0), 3)),
     phrases=("Hold-up capacitor", "Bulk capacitor hold-up time"),
     examples=[_ex({"Po": 300.0, "t_hold": 20e-3, "V1": 390.0, "V2": 300.0}), _ex({"Po": 50.0, "t_hold": 10e-3, "V1": 48.0, "V2": 36.0})],
     level="beginner")


def pfc_boost(Vac, f_line, Vo, Po, eff, L, fs, C):
    _pos("Vac, f_line, Vo, Po, L, fs and C", Vac, f_line, Vo, Po, L, fs, C)
    _frac("the efficiency", eff)
    Vpk = SQ2 * Vac
    _need(Vpk < Vo, "the output voltage must exceed the peak line voltage")
    Ipk = 2.0 * Po / (eff * Vpk)
    Dp = 1.0 - Vpk / Vo
    dIp = Vpk * Dp / (L * fs)
    dImax = Vo / (4.0 * L * fs) if Vpk >= Vo / 2.0 else dIp
    return _out(I_in_pk=Ipk, D_peak=Dp, dI_peak=dIp, dI_max=dImax, IL_at_peak=Ipk + dIp / 2.0, dVo_pp=Po / (TWO_PI * f_line * C * Vo))


def _ref_pfc_boost(Vac, f_line, Vo, Po, eff, L, fs, C):
    from scipy.optimize import minimize_scalar
    Vpk = SQ2 * Vac
    T = 1.0 / fs
    # input current amplitude from power balance over a line cycle: Pin = Vpk Ipk / 2
    Pin = Po / eff
    Ipk = _brentq(lambda i: _quad(lambda th: Vpk * math.sin(th) * i * math.sin(th), 0, math.pi) / math.pi - Pin, 0.0, 1e9)

    def ripple(v):                      # boost cell at an instantaneous line voltage v: simulate one switching period
        D = 1 - v / Vo
        c, _ = _pwl([(D * T, v), ((1 - D) * T, v - Vo)], L, 1.0)
        return max(c) - min(c)
    dIp = ripple(Vpk)
    res = minimize_scalar(lambda v: -ripple(v), bounds=(1e-9 * Vpk, Vpk), method="bounded", options={"xatol": 1e-12 * Vpk})
    dImax = max(-res.fun, dIp)
    # 2nd-harmonic ripple of the bus: the capacitor absorbs p_in - Po = -Po cos(2 w t); its peak-to-peak charge swing
    w = TWO_PI * f_line
    dE = _quad(lambda t: max(0.0, -Po * math.cos(2 * w * t)), math.pi / (4 * w), 3 * math.pi / (4 * w))
    return {"I_in_pk": Ipk, "D_peak": 1 - Vpk / Vo, "dI_peak": dIp, "dI_max": dImax, "IL_at_peak": Ipk + dIp / 2, "dVo_pp": dE / (C * Vo)}


_add("pe_boost_pfc",
     name="Boost power-factor-correction stage", topic="front ends / power factor correction",
     purpose="The peak line current, duty cycle and inductor ripple at the line peak, the largest ripple over the line cycle, the inductor current at the line peak and the second-harmonic output voltage ripple of a continuous-conduction boost PFC stage with unity power factor.",
     inputs=[_V("Vac", "RMS line voltage"), _phys("f_line", "frequency", "line frequency", show=_HZ), _V("Vo", "dc output voltage"), _phys("Po", "power", "output power"),
             _numin("eff", "efficiency (a fraction or %)", pct=True), _L("L", "boost inductance"), _F("fs", "switching frequency"), _C("C", "output (bulk) capacitance")],
     outputs=[_o("I_in_pk", "current", "peak line current"), _o("D_peak", "number", "duty cycle at the line peak"), _o("dI_peak", "current", "inductor ripple at the line peak"),
              _o("dI_max", "current", "largest inductor ripple over the line cycle"), _o("IL_at_peak", "current", "peak inductor current at the line peak"),
              _o("dVo_pp", "voltage", "peak-to-peak second-harmonic output ripple")],
     primary="dI_max", conditions=["unity power factor, continuous conduction", "Vo > sqrt(2) Vac", "small output ripple"],
     method="Ipk = 2 Po/(eff Vpk); dI(v) = v (1 - v/Vo)/(L fs), largest at v = Vo/2; dVo = Po/(2 pi f_line C Vo)",
     latex=r"\Delta i_L(v) = \frac{v\,(1-v/V_o)}{Lf_s},\quad \Delta i_{L,max} = \frac{V_o}{4Lf_s},\quad \Delta V_o = \frac{P_o}{2\pi f_{line}CV_o}",
     run=pfc_boost, reference=_ref_pfc_boost,
     wrong=lambda Vac, f_line, Vo, Po, eff, L, fs, C: (lambda r: dict(r, dI_max=r["dI_peak"], dVo_pp=_r(Po / (math.pi * f_line * C * Vo))))(pfc_boost(Vac, f_line, Vo, Po, eff, L, fs, C)),
     rand=lambda rng: (lambda Vac: {"Vac": Vac, "f_line": rng.choice([50.0, 60.0]), "Vo": _sig(SQ2 * Vac * rng.uniform(1.05, 1.6), 3), "Po": _sig(_lu(rng, 50.0, 3000.0), 3),
                                    "eff": round(rng.uniform(0.9, 0.98), 3), "L": _sig(_lu(rng, 1e-4, 2e-3), 3), "fs": _sig(_lu(rng, 2e4, 2e5), 3), "C": _sig(_lu(rng, 1e-4, 2e-3), 3)})(_sig(rng.uniform(85.0, 265.0), 3)),
     phrases=("Boost PFC stage", "Boost power factor correction"),
     examples=[_ex({"Vac": 230.0, "f_line": 50.0, "Vo": 400.0, "Po": 500.0, "eff": 0.95, "L": 500e-6, "fs": 65e3, "C": 330e-6}),
               _ex({"Vac": 120.0, "f_line": 60.0, "Vo": 390.0, "Po": 300.0, "eff": 0.94, "L": 1e-3, "fs": 100e3, "C": 220e-6})],
     level="advanced")


# the worked examples' outputs (computed by their references on the build machine): filled in below
_EXV: dict = {
    'pe_buck_ccm': [{'D': 0.208333333333, 'dIL': 0.899621212121, 'IL_max': 3.44981060606, 'IL_min': 2.55018939394, 'ripple_ratio': 0.299873737374, 'Iin': 0.625}, {'D': 0.25, 'dIL': 1.91489361702, 'IL_max': 5.95744680851, 'IL_min': 4.04255319149, 'ripple_ratio': 0.382978723404, 'Iin': 1.25}],
    'pe_boost_ccm': [{'D': 0.75, 'IL': 4.0, 'dIL': 0.9, 'IL_max': 4.45, 'IL_min': 3.55, 'V_switch': 48.0}, {'D': 0.583333333333, 'IL': 1.92, 'dIL': 0.583333333333, 'IL_max': 2.21166666667, 'IL_min': 1.62833333333, 'V_switch': 12.0}],
    'pe_buck_boost_ccm': [{'D': 0.555555555556, 'IL': 2.25, 'dIL': 1.41843971631, 'IL_max': 2.95921985816, 'IL_min': 1.54078014184, 'V_switch': 27.0}, {'D': 0.333333333333, 'IL': 3.0, 'dIL': 0.969696969697, 'IL_max': 3.48484848485, 'IL_min': 2.51515151515, 'V_switch': 36.0}],
    'pe_conversion_ratio': [{'M': 2.5, 'Vout': 30.0, 'polarity': 'same'}, {'M': 0.666666666667, 'Vout': 16.0, 'polarity': 'inverted'}],
    'pe_buck_output_ripple': [{'D': 0.208333333333, 'dIL': 0.899621212121, 'dVo': 0.00562263257576, 'dVo_pct': 0.112452651515}, {'D': 0.275, 'dIL': 1.01808510638, 'dVo': 0.00541534631055, 'dVo_pct': 0.16410140335}],
    'pe_boost_output_ripple': [{'D': 0.75, 'dVo': 0.159574468085, 'dVo_pct': 0.332446808511}, {'D': 0.583333333333, 'dVo': 0.0265151515152, 'dVo_pct': 0.22095959596}],
    'pe_cuk_ccm': [{'D': 0.555555555556, 'IL1': 1.25, 'IL2': 1.0, 'VC1': 27.0, 'dIL1': 0.666666666667, 'dIL2': 0.666666666667, 'dVC1': 0.555555555556}, {'D': 0.333333333333, 'IL1': 1.0, 'IL2': 2.0, 'VC1': 36.0, 'dIL1': 0.727272727273, 'dIL2': 1.06666666667, 'dVC1': 0.606060606061}],
    'pe_sepic_ccm': [{'D': 0.571428571429, 'IL1': 1.33333333333, 'IL2': 1.0, 'VC1': 9.0, 'dIL1': 0.547112462006, 'dIL2': 0.547112462006, 'dVC1': 0.285714285714, 'V_switch': 21.0}, {'D': 0.333333333333, 'IL1': 0.75, 'IL2': 1.5, 'VC1': 24.0, 'dIL1': 0.8, 'dIL2': 0.8, 'dVC1': 1.06382978723, 'V_switch': 36.0}],
    'pe_flyback_ccm': [{'D': 0.333333333333, 'Im': 1.5, 'dIm': 0.8, 'Ipk_primary': 1.9, 'Ipk_secondary': 3.8, 'V_switch': 72.0, 'V_diode': 36.0}, {'D': 0.235294117647, 'Im': 0.196153846154, 'dIm': 0.196078431373, 'Ipk_primary': 0.29419306184, 'Ipk_secondary': 5.8838612368, 'V_switch': 425.0, 'V_diode': 21.25}],
    'pe_forward_ccm': [{'D': 0.3125, 'D_max': 0.5, 'V_switch': 96.0, 'dIL': 0.78125, 'IL_max': 5.390625, 'IL_min': 4.609375}, {'D': 0.32, 'D_max': 0.5, 'V_switch': 600.0, 'dIL': 0.816, 'IL_max': 4.408, 'IL_min': 3.592}],
    'pe_bridge_converter': [{'D': 0.3, 'V_switch': 400.0, 'f_ripple': 200000.0, 'dIL': 1.92, 'IL_min': 9.04}, {'D': 0.25, 'V_switch': 48.0, 'f_ripple': 200000.0, 'dIL': 1.36363636364, 'IL_min': 2.31818181818}],
    'pe_critical_inductance': [{'D': 0.5, 'L_crit': 2.5e-05, 'Io': 1.2}, {'D': 0.666666666667, 'L_crit': 3.7037037037e-05, 'Io': 0.72}],
    'pe_buck_lc_design': [{'D': 0.275, 'L': 3.19e-06, 'C': 3.75e-05, 'IL_max': 5.75, 'dIL': 1.5}, {'D': 0.25, 'L': 2.25e-05, 'C': 0.0001, 'IL_max': 12.0, 'dIL': 4.0}],
    'pe_buck_component_currents': [{'D': 0.208333333333, 'I_sw_avg': 0.625, 'I_sw_rms': 1.37442739545, 'I_d_avg': 2.375, 'I_d_rms': 2.67925223307, 'I_L_rms': 3.01121955257, 'I_pk': 3.44981060606, 'I_cin_rms': 1.22410198324}, {'D': 0.25, 'I_sw_avg': 2.5, 'I_sw_rms': 5.03474498111, 'I_d_avg': 7.5, 'I_d_rms': 8.72043411043, 'I_L_rms': 10.0694899622, 'I_pk': 12.0454545455, 'I_cin_rms': 4.37020102796}],
    'pe_interleaved_buck_ripple': [{'D': 0.1, 'dI_phase': 4.59574468085, 'dI_total': 4.08510638298, 'cancellation': 0.888888888889, 'f_ripple': 1000000.0}, {'D': 0.15, 'dI_phase': 7.72727272727, 'dI_total': 3.63636363636, 'cancellation': 0.470588235294, 'f_ripple': 2400000.0}],
    'pe_buck_dcm': [{'K': 0.1, 'M': 0.6, 'Vout': 14.4, 'D2': 0.2, 'I_pk': 2.88, 'Io': 0.72}, {'K': 0.0376, 'M': 0.628586395174, 'Vout': 7.54303674209, 'D2': 0.118174242293, 'I_pk': 0.948290054875, 'Io': 0.150860734842}],
    'pe_boost_dcm': [{'K': 0.02, 'M': 3.37228132327, 'Vout': 40.4673758792, 'D2': 0.168614066163, 'I_pk': 4.8, 'Io': 0.404673758792}, {'K': 0.011, 'M': 3.40375931885, 'Vout': 17.0187965943, 'D2': 0.124804508358, 'I_pk': 1.36363636364, 'Io': 0.0850939829713}],
    'pe_buck_boost_dcm': [{'K': 0.04, 'M': 2.0, 'Vout': 24.0, 'D2': 0.2, 'I_pk': 4.8, 'Io': 0.48}, {'K': 0.02, 'M': 1.76776695297, 'Vout': 42.4264068712, 'D2': 0.141421356237, 'I_pk': 6.0, 'Io': 0.424264068712}],
    'pe_flyback_dcm_design': [{'Pin': 35.2941176471, 'Lm': 0.000286875, 'I_pk': 1.56862745098, 'I_rms': 0.607526799405, 'I_avg': 0.352941176471, 'E_cycle': 0.000352941176471}, {'Pin': 12.5, 'Lm': 1.0368e-05, 'I_pk': 3.47222222222, 'I_rms': 1.26787629052, 'I_avg': 0.694444444444, 'E_cycle': 6.25e-05}],
    'pe_boost_nonideal_gain': [{'M': 3.44827586207, 'Vout': 41.3793103448, 'efficiency': 0.862068965517, 'IL': 3.31034482759, 'D_peak': 0.9, 'M_max': 5.0}, {'M': 1.96078431373, 'Vout': 47.0588235294, 'efficiency': 0.980392156863, 'IL': 4.70588235294, 'D_peak': 0.929289321881, 'M_max': 7.07106781187}],
    'pe_buck_with_losses': [{'D': 0.226863226863, 'P_switch_cond': 0.0725962325962, 'P_diode': 1.54627354627, 'P_inductor': 0.16, 'P_switching': 0.384, 'P_loss': 2.16286977887, 'efficiency': 0.902410211293}, {'D': 0.262345679012, 'P_switch_cond': 0.262345679012, 'P_diode': 5.16358024691, 'P_inductor': 0.5, 'P_switching': 1.44, 'P_loss': 7.36592592593, 'efficiency': 0.942167217234}],
    'pe_converter_small_signal': [{'Gvd0': 48.0, 'f0': 1125.3953952, 'Q': 7.07106781187, 'f_rhpz': 7957.74715459, 'rhp_zero': 'present'}, {'Gvd0': 24.0, 'f0': 2287.69145929, 'Q': 6.32455532034, 'f_rhpz': 0.0, 'rhp_zero': 'none'}],
    'pe_peak_current_mode_slope': [{'D': 0.666666666667, 'm1': 400000.0, 'm2': 800000.0, 'alpha': -0.714285714286, 'stable': 'stable', 'ma_min': 200000.0, 'ma_deadbeat': 800000.0}, {'D': 0.208333333333, 'm1': 863636.363636, 'm2': 227272.727273, 'alpha': -0.132075471698, 'stable': 'stable', 'ma_min': 0.0, 'ma_deadbeat': 227272.727273}],
    'pe_three_phase_diode_bridge': [{'Vdc': 540.189789694, 'Vrms': 540.665270663, 'ripple_factor': 0.0419666138094, 'form_factor': 1.00088021095, 'f_ripple': 300.0, 'PIV': 565.685424949, 'Idc': 10.8037957939, 'I_diode_avg': 3.60126526463, 'I_diode_rms': 6.24306479118, 'Is_rms': 8.82902689846}, {'Vdc': 648.227747633, 'Vrms': 648.798324796, 'ripple_factor': 0.0419666138094, 'form_factor': 1.00088021095, 'f_ripple': 360.0, 'PIV': 678.822509939, 'Idc': 32.4113873817, 'I_diode_avg': 10.8037957939, 'I_diode_rms': 18.7291943735, 'Is_rms': 26.4870806954}],
    'pe_three_phase_half_wave_rectifier': [{'Vdc': 268.995396426, 'Vrms': 273.448302108, 'ripple_factor': 0.182706599572, 'form_factor': 1.01655383602, 'f_ripple': 150.0, 'PIV': 563.38264084, 'Idc': 13.4497698213, 'I_diode_avg': 4.48325660709, 'I_diode_rms': 7.89377254158}, {'Vdc': 140.345424222, 'Vrms': 142.668679361, 'ripple_factor': 0.182706599572, 'form_factor': 1.01655383602, 'f_ripple': 180.0, 'PIV': 293.938769134, 'Idc': 14.0345424222, 'I_diode_avg': 4.6781808074, 'I_diode_rms': 8.23698004339}],
    'pe_single_phase_controlled_rectifier': [{'Vdc': 179.330264284, 'Vrms': 230.0, 'Vac_rms': 144.016166842, 'mode': 'rectifying'}, {'Vdc': 40.5142342271, 'Vrms': 76.1077657783, 'Vac_rms': 64.4281680382, 'mode': 'rectifying'}],
    'pe_three_phase_controlled_rectifier': [{'Vdc': 396.295808299, 'Vrms': 415.0, 'P': 19814.7904149, 'I_thy_avg': 16.6666666667, 'I_thy_rms': 28.8675134595, 'Is_rms': 40.8248290464, 'mode': 'rectifying'}, {'Vdc': -270.094894847, 'Vrms': 306.334021208, 'P': -5401.89789694, 'I_thy_avg': 6.66666666667, 'I_thy_rms': 11.5470053838, 'Is_rms': 16.3299316186, 'mode': 'inverting'}],
    'pe_ac_voltage_controller': [{'Vo_rms': 206.295882917, 'Io_rms': 20.6295882917, 'P': 4255.79913086, 'PF': 0.896938621379, 'I_scr_avg': 7.76522822686, 'I_scr_rms': 14.5873217742}, {'Vo_rms': 84.8528137424, 'Io_rms': 4.24264068712, 'P': 360.0, 'PF': 0.707106781187, 'I_scr_avg': 1.35047447424, 'I_scr_rms': 3.0}],
    'pe_rectifier_line_current_pf': [{'Is_rms': 10.0, 'Is1_rms': 9.00316316157, 'THD': 0.483425847609, 'DPF': 1.0, 'PF': 0.900316316157, 'P': 2070.72752716, 'S': 2300.0}, {'Is_rms': 81.6496580928, 'Is1_rms': 77.9696801234, 'THD': 0.310841939307, 'DPF': 0.866025403784, 'PF': 0.826993343133, 'P': 46781.808074, 'S': 56568.5424949}],
    'pe_rectifier_line_harmonics': [{'Ih': 15.5939360247, 'Ih_over_I1': 0.2, 'THD_to_h': 0.2, 'I1': 77.9696801234}, {'Ih': 2.57233233188, 'Ih_over_I1': 0.142857142857, 'THD_to_h': 0.414148855336, 'I1': 18.0063263231}],
    'pe_half_wave_rl_rectifier': [{'beta_deg': 200.6578643, 'theta_deg': 20.6559973823, 'Z': 106.870122269, 'V_avg': 52.2823694227, 'I_avg': 0.522823694227}, {'beta_deg': 240.847093223, 'theta_deg': 57.5183634095, 'Z': 18.6209588912, 'V_avg': 76.9866475269, 'I_avg': 7.69866475269}],
    'pe_capacitor_filter_exact': [{'theta_off_deg': 93.0367886534, 'alpha_deg': 48.2308238926, 'dV': 43.1318901238, 'V_avg': 148.102862563, 'I_D_peak': 4.51473752936, 'conduction_deg': 44.8059647608}, {'theta_off_deg': 93.8744692513, 'alpha_deg': 57.3593659828, 'dV': 2.36894864482, 'V_avg': 13.8708494422, 'I_D_peak': 1.32091532236, 'conduction_deg': 36.5151032685}],
    'pe_single_phase_overlap': [{'u_deg': 7.93041601856, 'Vdc': 171.330264284, 'dV': 8.0}, {'u_deg': 21.0357313731, 'Vdc': 104.437957939, 'dV': 3.6}],
    'pe_three_phase_overlap': [{'u_deg': 5.85622851056, 'Vdc': 452.81808074, 'dV': 15.0}, {'u_deg': 17.1431749909, 'Vdc': 633.827747633, 'dV': 14.4}],
    'pe_thd_from_harmonics': [{'THD': 0.0616441400297, 'THD_pct': 6.16441400297, 'Vrms': 230.436585637, 'distortion_factor': 0.998105397909}, {'THD': 0.388444333206, 'THD_pct': 38.8444333206, 'Vrms': 10.7279494779, 'distortion_factor': 0.932144583699}],
    'pe_power_factor_distortion': [{'DPF': 1.0, 'distortion_factor': 0.900466521987, 'PF': 0.900466521987}, {'DPF': 0.866025403784, 'distortion_factor': 0.957826285221, 'PF': 0.829501895414}],
    'pe_pulse_waveform_rms': [{'avg': 3.0, 'rms': 6.0, 'ac_rms': 5.19615242271, 'form_factor': 2.0, 'ripple_factor': 1.73205080757}, {'avg': 60.0, 'rms': 77.4596669241, 'ac_rms': 48.9897948557, 'form_factor': 1.29099444874, 'ripple_factor': 0.816496580928}],
    'pe_square_wave_inverter': [{'Vrms': 100.0, 'V1_rms': 90.0316316157, 'V1_peak': 127.323954474, 'Vn_rms': 30.0105438719, 'THD': 0.483425847609}, {'Vrms': 24.0, 'V1_rms': 21.6075915878, 'V1_peak': 30.5577490736, 'Vn_rms': 4.32151831755, 'THD': 0.483425847609}],
    'pe_quasi_square_inverter': [{'V1_rms': 77.9696801234, 'Vn_rms': 0.0, 'Vrms': 81.6496580928, 'THD': 0.310841939307, 'alpha_eliminate_n_deg': 30.0}, {'V1_rms': 171.250339842, 'Vn_rms': 0.0, 'Vrms': 178.8854382, 'THD': 0.301921556274, 'alpha_eliminate_n_deg': 18.0}],
    'pe_six_step_inverter': [{'VLL_rms': 489.897948557, 'VLL1_rms': 467.81808074, 'Vph_rms': 282.842712475, 'Vph1_rms': 270.094894847, 'THD': 0.310841939307, 'VLL5_rms': 93.563616148, 'VLL7_rms': 66.8311543915, 'f5': 250.0, 'f7': 350.0}, {'VLL_rms': 39.1918358845, 'VLL1_rms': 37.4254464592, 'Vph_rms': 22.627416998, 'Vph1_rms': 21.6075915878, 'THD': 0.310841939307, 'VLL5_rms': 7.48508929184, 'VLL7_rms': 5.34649235132, 'f5': 1000.0, 'f7': 1400.0}],
    'pe_spwm_fundamental': [{'V1_leg_peak': 240.0, 'V1_leg_rms': 169.705627485, 'V1_LL_rms': 293.938769134, 'V1_fullbridge_rms': 339.41125497, 'f_carrier': 1050.0}, {'V1_leg_peak': 200.0, 'V1_leg_rms': 141.421356237, 'V1_LL_rms': 244.948974278, 'V1_fullbridge_rms': 282.842712475, 'f_carrier': 2340.0}],
    'pe_space_vector_times': [{'sector': 1, 'T1': 3.31154509928e-05, 'T2': 2.4399876718e-05, 'T0': 4.24846722891e-05, 'm': 0.57735026919}, {'sector': 4, 'T1': 5.56670399226e-05, 'T2': 2.96198132726e-05, 'T0': 0.000114713146805, 'm': 0.433012701892}],
    'pe_dead_time_error': [{'dV_avg': 12.0, 'dV1_peak': 15.2788745368, 'dV1_pct': 6.11154981473}, {'dV_avg': 0.192, 'dV1_peak': 0.244461992589, 'dV1_pct': 1.22230996295}],
    'pe_inverter_rl_load_current': [{'I_max': 9.31109608668, 'I_rms': 6.64329914124, 'P': 441.334234799, 'Is_avg': 4.41334234799}, {'I_max': 10.1794036795, 'I_rms': 6.80277330777, 'P': 185.110898708, 'Is_avg': 3.85647705641}],
    'pe_igbt_inverter_conduction_losses': [{'P_igbt': 43.8623855229, 'P_diode': 8.32222006823, 'P_switch_pair': 52.1846055911, 'P_leg': 104.369211182}, {'P_igbt': 145.734513367, 'P_diode': 33.0433118052, 'P_switch_pair': 178.777825172, 'P_leg': 357.555650344}],
    'pe_dc_motor_chopper_drive': [{'Ia': 20.0, 'Va': 132.0, 'Ea': 122.0, 'omega': 101.666666667, 'rpm': 970.845152861, 'P_out': 2440.0, 'efficiency': 0.924242424242}, {'Ia': 10.0, 'Va': 38.4, 'Ea': 36.4, 'omega': 242.666666667, 'rpm': 2317.29597142, 'P_out': 364.0, 'efficiency': 0.947916666667}],
    'pe_bldc_six_step': [{'omega': 364.0, 'rpm': 3475.94395713, 'T': 1.0, 'omega_no_load': 384.0, 'I_stall': 192.0, 'f_e': 231.729597142, 't_commutation': 0.00071922908736}, {'omega': 260.0, 'rpm': 2482.81711223, 'T': 0.08, 'omega_no_load': 300.0, 'I_stall': 15.0, 'f_e': 82.7605704078, 't_commutation': 0.00201384144461}],
    'pe_dc_drive_controlled_rectifier': [{'Va': 146.422547645, 'Ea': 136.422547645, 'omega': 136.422547645, 'rpm': 1302.73936841, 'T': 10.0, 'P': 1364.22547645, 'mode': 'motoring'}, {'Va': 485.361258768, 'Ea': 475.361258768, 'omega': 237.680629384, 'rpm': 2269.68282262, 'T': 100.0, 'P': 23768.0629384, 'mode': 'motoring'}],
    'pe_regenerative_braking_chopper': [{'Ea': 150.0, 'Va': 110.0, 'Ia': 80.0, 'T_brake': 80.0, 'P_mech': 12000.0, 'P_regen': 8800.0, 'efficiency': 0.733333333333}, {'Ea': 500.0, 'Va': 420.0, 'Ia': 800.0, 'T_brake': 2000.0, 'P_mech': 400000.0, 'P_regen': 336000.0, 'efficiency': 0.84}],
    'pe_dc_motor_armature_ripple': [{'I_max': 12.7494272265, 'I_min': 7.25057277347, 'dI': 5.49885445305, 'I_avg': 10.0}, {'I_max': 24.1411235712, 'I_min': 15.7469980103, 'dI': 8.39412556086, 'I_avg': 20.0}],
    'pe_induction_motor_vf': [{'V': 240.0, 'ns': 900.0, 'n': 850.0, 'slip': 0.0555555555556, 'f_slip': 1.66666666667}, {'V': 306.666666667, 'ns': 800.0, 'n': 785.0, 'slip': 0.01875, 'f_slip': 0.75}],
    'pe_pmsm_dq_model': [{'omega_e': 1200.0, 'vd': -73.0, 'vq': 89.4, 'V_peak': 115.418196139, 'Te': 33.12, 'P_mech': 9936.0, 'P_in': 10236.0, 'P_cu': 300.0, 'PF': 0.934834518077}, {'omega_e': 300.0, 'vd': -6.0, 'vq': 32.0, 'V_peak': 32.5576411922, 'Te': 3.0, 'P_mech': 450.0, 'P_in': 480.0, 'P_cu': 30.0, 'PF': 0.982872186934}],
    'pe_pmsm_mtpa': [{'id': -44.8402626637, 'iq': 89.3831686854, 'beta_deg': 26.6412442347, 'Te': 57.3325941107, 'Te_id0': 48.0}, {'id': -8.08142966966, 'iq': 18.2945482178, 'beta_deg': 23.8329545979, 'Te': 10.2284691123, 'Te_id0': 9.0}],
    'pe_pmsm_field_weakening': [{'omega_base': 428.746462856, 'omega_max': 1250.0, 'rpm_base': 4094.2271338, 'id': -33.2777777778, 'iq': 49.9258400648, 'T': 29.9555040389, 'P': 17973.3024233}, {'omega_base': 429.32505168, 'omega_max': 960.0, 'rpm_base': 4099.75225008, 'id': 0.0, 'iq': 50.0, 'T': 7.5, 'P': 1125.0}],
    'pe_mosfet_conduction_loss': [{'Rds_T': 0.017, 'P': 6.8, 'P_25': 4.0, 'ratio': 1.7}, {'Rds_T': 0.116, 'P': 2.9, 'P_25': 2.0, 'ratio': 1.45}],
    'pe_switching_loss': [{'E_on': 6e-05, 'E_off': 0.0001, 'P_sw': 16.0, 'P_oss': 0.8, 'P_total': 16.8}, {'E_on': 0.003, 'E_off': 0.006, 'P_sw': 90.0, 'P_oss': 0.0, 'P_total': 90.0}],
    'pe_gate_drive_power': [{'P_drive': 0.12, 'Ig_avg': 0.01, 'Ig_peak': 2.4, 'E_cycle': 6e-07}, {'P_drive': 0.18, 'Ig_avg': 0.012, 'Ig_peak': 7.5, 'E_cycle': 1.8e-05}],
    'pe_diode_reverse_recovery': [{'Qrr': 5e-07, 'ta': 5e-08, 'tb': 5e-08, 'E_diode': 3.33333333333e-05, 'E_switch': 0.000166666666667, 'P_diode': 3.33333333333, 'P_switch': 16.6666666667, 'P_total': 20.0}, {'Qrr': 6e-06, 'ta': 2e-07, 'tb': 1e-07, 'E_diode': 0.0004, 'E_switch': 0.0032, 'P_diode': 8.0, 'P_switch': 64.0, 'P_total': 72.0}],
    'pe_parallel_mosfet_sharing': [{'currents': [21.8181818182, 18.1818181818], 'losses': [4.76033057851, 3.96694214876], 'P_total': 8.72727272727, 'imbalance': 1.09090909091}, {'currents': [33.110367893, 29.7993311037, 27.0903010033], 'losses': [4.93333407904, 4.44000067113, 4.03636424648], 'P_total': 13.4096989967, 'imbalance': 1.10367892977}],
    'pe_bootstrap_capacitor': [{'Q_total': 6.18e-08, 'C_min': 1.236e-07, 'C_recommended': 1.236e-06}, {'Q_total': 2.95e-07, 'C_min': 2.95e-07, 'C_recommended': 2.95e-06}],
    'pe_turnoff_snubber': [{'Cs': 1.66666666667e-09, 'E_off_with': 2.5e-05, 'E_off_without': 0.00015, 'E_snubber': 7.5e-05, 'P_switch': 1.25, 'P_snubber': 3.75, 'P_total': 5.0}, {'Cs': 1.66666666667e-08, 'E_off_with': 0.001, 'E_off_without': 0.006, 'E_snubber': 0.003, 'P_switch': 10.0, 'P_snubber': 30.0, 'P_total': 40.0}],
    'pe_rc_snubber_ringing': [{'Cp': 1e-10, 'Lp': 1.01321183642e-07, 'Rs': 31.8309886184, 'Cs': 4e-10, 'P_R': 0.4608}, {'Cp': 8e-10, 'Lp': 3.51809665425e-08, 'Rs': 6.63145596216, 'Cs': 2.4e-09, 'P_R': 38.4}],
    'pe_zvs_transition': [{'Z': 223.60679775, 'f_res': 1779406.35854, 'I_min': 1.788854382, 'E_L': 9e-05, 'E_C': 3.2e-05, 'zvs': 'achieved', 't_transition': 5.71418356386e-08, 'v_reached': 400.0}, {'Z': 15.8113883008, 'f_res': 5032921.21045, 'I_min': 3.03578655376, 'E_L': 2.5e-07, 'E_C': 2.304e-06, 'zvs': 'not achieved', 't_transition': 4.9672941329e-08, 'v_reached': 15.8113883008}],
    'pe_thermal_shared_heatsink': [{'P_total': 55.0, 'Ts': 340.65, 'Tj': [360.65, 360.65, 355.65], 'Tj_max': 360.65, 'Tj_max_C': 87.5}, {'P_total': 80.0, 'Ts': 322.15, 'Tj': [347.15, 337.15], 'Tj_max': 347.15, 'Tj_max_C': 74.0}],
    'pe_foster_transient_thermal': [{'Zth': 0.173622987704, 'dTj': 86.8114938518, 'Rth_total': 0.3, 'dTj_steady': 150.0}, {'Zth': 0.254380773664, 'dTj': 25.4380773664, 'Rth_total': 0.5, 'dTj_steady': 50.0}],
    'pe_foster_periodic_pulses': [{'dT_peak': 66.4525061197, 'dT_min': 15.5776255331, 'dT_avg': 30.0, 'ripple': 50.8748805866}, {'dT_peak': 28.6586341561, 'dT_min': 6.41514661098, 'dT_avg': 12.5, 'ripple': 22.2434875451}],
    'pe_transformer_turns': [{'Np': 6.4, 'Ns': 1.6, 'n': 4.0, 'volts_per_turn': 7.5}, {'Np': 862.803136317, 'Ns': 90.0316316157, 'n': 9.58333333333, 'volts_per_turn': 0.26657297629}],
    'pe_gapped_inductor_design': [{'N': 33.3333333333, 'l_gap': 0.00139626340141, 'E': 0.005, 'AL': 9e-08, 'V_gap': 1.39626340141e-07}, {'N': 160.0, 'l_gap': 0.00160849543843, 'E': 0.002, 'AL': 3.90625e-08, 'V_gap': 8.04247719213e-08}],
    'pe_inductor_area_product': [{'Ap': 2.5e-08, 'Ap_cm4': 2.5}, {'Ap': 6.66666666667e-08, 'Ap_cm4': 6.66666666667}],
    'pe_transformer_area_product': [{'Pt': 513.157894737, 'Ap': 8.01809210526e-09, 'Ap_cm4': 0.801809210526}, {'Pt': 1055.55555556, 'Ap': 3.77116622729e-06, 'Ap_cm4': 377.116622729}],
    'pe_steinmetz_core_loss': [{'Pv': 101192.885125, 'Pv_mW_cm3': 101.192885125, 'P': 0.505964425627}, {'Pv': 50596.4425627, 'Pv_mW_cm3': 50.5964425627, 'P': 1.01192885125}],
    'pe_igse_triangular_core_loss': [{'ki': 0.199806147242, 'Pv': 94324.7860298, 'P': 0.471623930149, 'ratio_to_sine': 0.932128636445}, {'ki': 0.199806147242, 'Pv': 107120.575714, 'P': 0.535602878569, 'ratio_to_sine': 1.05857813601}],
    'pe_skin_effect_round_wire': [{'delta': 0.000208729751047, 'd_over_delta': 4.79088388208, 'Rdc': 0.0218997201694, 'Rac': 0.0317822606402, 'ratio': 1.45126332183}, {'delta': 0.000295188444801, 'd_over_delta': 1.69383324045, 'Rdc': 0.175197761356, 'Rac': 0.177059613754, 'ratio': 1.0106271472}],
    'pe_dowell_proximity_factor': [{'delta': 0.000208729751047, 'Delta': 0.958176776416, 'FR': 2.43112682554}, {'delta': 0.0001475942224, 'Delta': 0.67753329618, 'FR': 1.83115076644}],
    'pe_series_resonant_gain': [{'fr': 100658.424209, 'Z0': 63.2455532034, 'Q': 2.10818510678, 'fn': 1.1921505919, 'M': 0.801964343169}, {'fr': 71176.2543417, 'Z0': 22.360679775, 'Q': 2.2360679775, 'fn': 0.842977767725, 'M': 0.79323846955}],
    'pe_llc_resonant_gain': [{'fr1': 125043.9328, 'fr2': 51048.9717983, 'Rac': 103.75289205, 'Q': 0.454353137997, 'k': 5.0, 'M': 1.09816950021}, {'fr1': 112539.53952, 'fr2': 42535.9477472, 'Rac': 16.2113893828, 'Q': 0.872358024955, 'k': 6.0, 'M': 0.967265249813}],
    'pe_dual_active_bridge_power': [{'P': 2133.33333333, 'P_max': 3840.0, 'i0': -7.2, 'i_phi': 5.86666666667, 'I_rms': 6.17569909073}, {'P': 4444.44444444, 'P_max': 5000.0, 'i0': -33.3333333333, 'i_phi': 33.3333333333, 'I_rms': 29.3972367896}],
    'pe_hold_up_capacitor': [{'C': 0.000193236714976, 'E_used': 6.0, 'E_initial': 14.6956521739, 'E_remaining': 8.69565217391}, {'C': 0.000992063492063, 'E_used': 0.5, 'E_initial': 1.14285714286, 'E_remaining': 0.642857142857}],
    'pe_boost_pfc': [{'I_in_pk': 3.23618664159, 'D_peak': 0.186827201635, 'dI_peak': 1.86981905679, 'dI_max': 3.07692307692, 'IL_at_peak': 4.17109616998, 'dVo_pp': 12.0571926585}, {'I_in_pk': 3.76120628291, 'D_peak': 0.564857365424, 'dI_peak': 0.958594736386, 'dI_max': 0.958594736386, 'IL_at_peak': 4.2405036511, 'dVo_pp': 9.27476358344}],
}

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
