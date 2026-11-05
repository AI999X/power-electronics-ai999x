"""power-electronics-ai999x — offline power electronics answers with their formula, steps and verification.

    import power_electronics_ai999x as m
    m.ask("Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A").show()
    m.solve("pe_three_phase_controlled_rectifier", VLL=415.0, alpha=0.7853981634, Id=50.0)
    m.capabilities()                   # every certified capability

Certified capabilities: DC-DC converters in continuous conduction (buck, boost, buck-boost, Cuk, SEPIC, flyback,
forward, push-pull and bridge converters: duty cycle, inductor and capacitor ripple, component currents, critical
inductance, filter design, interleaving) and discontinuous conduction (conversion ratio, diode interval, peak current),
efficiency with conduction and switching losses, the non-ideal boost gain; rectifiers (three-phase bridge and
half-wave, single- and three-phase phase-controlled, half-wave RL load, the exact capacitor filter, commutation overlap,
line-current harmonics, THD and power factor, AC voltage controllers); inverters and PWM (square-wave, quasi-square,
six-step, sinusoidal PWM, space-vector dwell times, dead time, RL load current, IGBT and diode losses); motor drives
(chopper- and rectifier-fed DC motors, regenerative braking, armature ripple, BLDC six-step, induction motor V/f, PMSM
dq model, MTPA, field weakening); devices (MOSFET conduction and switching loss, reverse recovery, gate drive,
bootstrap capacitor, current sharing, snubbers, zero-voltage switching); thermal design (shared heat sinks, Foster
transient impedance, periodic pulses); magnetics (transformer turns, gapped inductors, area product, Steinmetz and iGSE
core loss, skin effect, Dowell factor); resonant and bidirectional converters (series resonant, LLC, dual active
bridge); control (averaged small-signal model, peak-current-mode slope compensation); boost PFC and hold-up capacitors.
Questions use one documented form, "<capability phrase>: name = value, ..." with units; a question that is not
determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.5.0"
_P = Product("power_electronics", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
