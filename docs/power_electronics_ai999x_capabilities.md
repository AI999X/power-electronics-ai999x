# power-electronics-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `power_electronics_ai999x.about()` for the same answer.

Every capability power-electronics-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import power_electronics_ai999x

result = power_electronics_ai999x.solve('pe_buck_ccm', Vin='24 V', Vout='5 V', L='2.2e-05 H', fs='200000 Hz', Io='3 A')
result.show()
# D = 0.208333, dIL = 0.899621 A, IL_max = 3.44981 A, IL_min = 2.55019 A, ripple_ratio = 0.299874, Iin = 0.625 A
```

## By topic

| topic | capabilities |
|---|---|
| Control / averaged small-signal model | [Control-to-output transfer function of a DC-DC converter](#pe_converter_small_signal) |
| Control / current mode | [Peak current mode: subharmonic stability and slope compensation](#pe_peak_current_mode_slope) |
| DC-DC converters / boost | [Boost converter in continuous conduction](#pe_boost_ccm), [Boost converter output voltage ripple](#pe_boost_output_ripple) |
| DC-DC converters / buck | [Buck converter in continuous conduction](#pe_buck_ccm), [Buck converter output voltage ripple](#pe_buck_output_ripple) |
| DC-DC converters / buck-boost | [Buck-boost converter in continuous conduction](#pe_buck_boost_ccm) |
| DC-DC converters / component stresses | [Buck converter switch, diode, inductor and input-capacitor currents](#pe_buck_component_currents) |
| DC-DC converters / conduction boundary | [Critical inductance at the CCM/DCM boundary](#pe_critical_inductance) |
| DC-DC converters / conversion ratio | [Ideal DC-DC conversion ratio from the duty cycle](#pe_conversion_ratio) |
| DC-DC converters / Cuk | [Cuk converter in continuous conduction](#pe_cuk_ccm) |
| DC-DC converters / discontinuous conduction | [Buck converter in discontinuous conduction](#pe_buck_dcm), [Boost converter in discontinuous conduction](#pe_boost_dcm), [Buck-boost converter in discontinuous conduction](#pe_buck_boost_dcm) |
| DC-DC converters / filter design | [Buck converter inductor and capacitor design](#pe_buck_lc_design) |
| DC-DC converters / interleaving | [Interleaved multiphase buck: output current ripple cancellation](#pe_interleaved_buck_ripple) |
| DC-DC converters / SEPIC | [SEPIC converter in continuous conduction](#pe_sepic_ccm) |
| Devices / conduction loss | [MOSFET conduction loss at junction temperature](#pe_mosfet_conduction_loss) |
| Devices / gate drive | [Gate-drive power and currents](#pe_gate_drive_power), [Bootstrap capacitor for a high-side gate driver](#pe_bootstrap_capacitor) |
| Devices / paralleling | [Current sharing between paralleled MOSFETs](#pe_parallel_mosfet_sharing) |
| Devices / reverse recovery | [Diode reverse-recovery charge and losses](#pe_diode_reverse_recovery) |
| Devices / snubbers | [RCD turn-off snubber design](#pe_turnoff_snubber), [RC snubber from measured ringing frequencies](#pe_rc_snubber_ringing) |
| Devices / soft switching | [Zero-voltage switching: resonant transition of a switch node](#pe_zvs_transition) |
| Devices / switching loss | [Hard-switching loss of a power transistor](#pe_switching_loss) |
| Front ends / energy storage | [Hold-up (bulk) capacitor](#pe_hold_up_capacitor) |
| Front ends / power factor correction | [Boost power-factor-correction stage](#pe_boost_pfc) |
| Inverters / dead time | [Inverter dead-time voltage error](#pe_dead_time_error) |
| Inverters / load current | [Square-wave inverter with an RL load: current](#pe_inverter_rl_load_current) |
| Inverters / quasi-square-wave | [Quasi-square-wave (phase-shifted) full-bridge inverter](#pe_quasi_square_inverter) |
| Inverters / sinusoidal PWM | [Sinusoidal PWM: fundamental output voltage](#pe_spwm_fundamental) |
| Inverters / space-vector PWM | [Space-vector modulation dwell times](#pe_space_vector_times) |
| Inverters / square-wave | [Square-wave inverter output and harmonics](#pe_square_wave_inverter) |
| Inverters / three-phase | [Three-phase six-step (square-wave) inverter](#pe_six_step_inverter) |
| Isolated converters / bridge | [Push-pull, half-bridge and full-bridge converters](#pe_bridge_converter) |
| Isolated converters / flyback | [Flyback converter in continuous conduction](#pe_flyback_ccm) |
| Isolated converters / flyback design | [Flyback magnetizing inductance for discontinuous conduction](#pe_flyback_dcm_design) |
| Isolated converters / forward | [Forward converter with reset winding](#pe_forward_ccm) |
| Losses / efficiency | [Buck converter efficiency with conduction and switching losses](#pe_buck_with_losses) |
| Losses / inverter devices | [IGBT and diode conduction losses in a sinusoidal PWM inverter](#pe_igbt_inverter_conduction_losses) |
| Losses / non-ideal converters | [Boost converter with inductor resistance: gain and efficiency](#pe_boost_nonideal_gain) |
| Magnetics / core loss | [Core loss by the Steinmetz equation](#pe_steinmetz_core_loss), [Core loss under triangular flux (iGSE)](#pe_igse_triangular_core_loss) |
| Magnetics / core selection | [Inductor core area product](#pe_inductor_area_product), [Transformer core area product](#pe_transformer_area_product) |
| Magnetics / inductors | [Gapped-core inductor: turns and air gap](#pe_gapped_inductor_design) |
| Magnetics / transformers | [Transformer turns from Faraday's law](#pe_transformer_turns) |
| Magnetics / winding losses | [Skin effect in a round wire](#pe_skin_effect_round_wire), [Winding AC resistance factor (Dowell)](#pe_dowell_proximity_factor) |
| Motor drives / BLDC | [BLDC motor with six-step (120 degree) drive](#pe_bldc_six_step) |
| Motor drives / current ripple | [Armature current ripple of a chopper-fed DC motor](#pe_dc_motor_armature_ripple) |
| Motor drives / DC motor | [Chopper-fed separately excited DC motor: speed and torque](#pe_dc_motor_chopper_drive), [Phase-controlled rectifier DC motor drive](#pe_dc_drive_controlled_rectifier) |
| Motor drives / field weakening | [Surface PMSM field weakening: base speed, maximum speed and torque](#pe_pmsm_field_weakening) |
| Motor drives / induction motor | [Induction motor constant V/f speed control](#pe_induction_motor_vf) |
| Motor drives / PMSM | [PMSM steady state in the dq frame: voltages and torque](#pe_pmsm_dq_model) |
| Motor drives / PMSM control | [Maximum torque per ampere (MTPA) of an interior PMSM](#pe_pmsm_mtpa) |
| Motor drives / regenerative braking | [Regenerative braking of a DC motor with a chopper](#pe_regenerative_braking_chopper) |
| Power quality / power factor | [Power factor with harmonic distortion](#pe_power_factor_distortion) |
| Power quality / THD | [Total harmonic distortion from harmonic values](#pe_thd_from_harmonics) |
| Power quality / waveform values | [Average and RMS of a rectangular pulse train](#pe_pulse_waveform_rms) |
| Rectifiers / AC voltage control | [Single-phase AC voltage controller (resistive load)](#pe_ac_voltage_controller) |
| Rectifiers / capacitor filter | [Capacitor-input rectifier filter: exact ripple and peak diode current](#pe_capacitor_filter_exact) |
| Rectifiers / commutation | [Commutation overlap in a single-phase thyristor bridge](#pe_single_phase_overlap), [Commutation overlap in a three-phase thyristor bridge](#pe_three_phase_overlap) |
| Rectifiers / phase-controlled | [Single-phase phase-controlled thyristor rectifier](#pe_single_phase_controlled_rectifier), [Three-phase fully controlled thyristor bridge](#pe_three_phase_controlled_rectifier) |
| Rectifiers / power quality | [Rectifier line current: THD, displacement and power factor](#pe_rectifier_line_current_pf), [Rectifier line-current harmonic spectrum](#pe_rectifier_line_harmonics) |
| Rectifiers / RL load | [Half-wave diode rectifier with an RL load](#pe_half_wave_rl_rectifier) |
| Rectifiers / three-phase uncontrolled | [Three-phase diode bridge rectifier with resistive load](#pe_three_phase_diode_bridge), [Three-phase half-wave (three-pulse) diode rectifier](#pe_three_phase_half_wave_rectifier) |
| Resonant converters / dual active bridge | [Dual active bridge: power transfer and inductor current](#pe_dual_active_bridge_power) |
| Resonant converters / LLC | [LLC resonant converter: tank gain (first harmonic)](#pe_llc_resonant_gain) |
| Resonant converters / series resonant | [Series resonant converter: tank gain (first harmonic)](#pe_series_resonant_gain) |
| Thermal / steady state | [Several power devices on one heat sink](#pe_thermal_shared_heatsink) |
| Thermal / transient | [Transient thermal impedance of a Foster network](#pe_foster_transient_thermal), [Junction temperature under periodic power pulses](#pe_foster_periodic_pulses) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| pe_buck_ccm | Buck converter in continuous conduction | Vin, Vout, L, fs, Io | D, dIL, IL_max, IL_min, ripple_ratio, Iin |
| pe_boost_ccm | Boost converter in continuous conduction | Vin, Vout, L, fs, Io | D, IL, dIL, IL_max, IL_min, V_switch |
| pe_buck_boost_ccm | Buck-boost converter in continuous conduction | Vin, Vout, L, fs, Io | D, IL, dIL, IL_max, IL_min, V_switch |
| pe_conversion_ratio | Ideal DC-DC conversion ratio from the duty cycle | topology, Vin, D | M, Vout, polarity |
| pe_buck_output_ripple | Buck converter output voltage ripple | Vin, Vout, L, C, fs | D, dIL, dVo, dVo_pct |
| pe_boost_output_ripple | Boost converter output voltage ripple | Vin, Vout, Io, C, fs | D, dVo, dVo_pct |
| pe_cuk_ccm | Cuk converter in continuous conduction | Vin, Vout, L1, L2, C1, fs, Io | D, IL1, IL2, VC1, dIL1, dIL2, dVC1 |
| pe_sepic_ccm | SEPIC converter in continuous conduction | Vin, Vout, L1, L2, C1, fs, Io | D, IL1, IL2, VC1, dIL1, dIL2, dVC1, V_switch |
| pe_flyback_ccm | Flyback converter in continuous conduction | Vin, Vout, n, Lm, fs, Io | D, Im, dIm, Ipk_primary, Ipk_secondary, V_switch, V_diode |
| pe_forward_ccm | Forward converter with reset winding | Vin, Vout, n, Nr, L, fs, Io | D, D_max, V_switch, dIL, IL_max, IL_min |
| pe_bridge_converter | Push-pull, half-bridge and full-bridge converters | topology, Vin, Vout, n, L, fs, Io | D, V_switch, f_ripple, dIL, IL_min |
| pe_critical_inductance | Critical inductance at the CCM/DCM boundary | topology, Vin, Vout, fs, R | D, L_crit, Io |
| pe_buck_lc_design | Buck converter inductor and capacitor design | Vin, Vout, Io, fs, r, dV | D, L, C, IL_max, dIL |
| pe_buck_component_currents | Buck converter switch, diode, inductor and input-capacitor currents | Vin, Vout, Io, L, fs | D, I_sw_avg, I_sw_rms, I_d_avg, I_d_rms, I_L_rms, I_pk, I_cin_rms |
| pe_interleaved_buck_ripple | Interleaved multiphase buck: output current ripple cancellation | N, Vin, Vout, L, fs | D, dI_phase, dI_total, cancellation, f_ripple |
| pe_buck_dcm | Buck converter in discontinuous conduction | Vin, L, fs, R, D | K, M, Vout, D2, I_pk, Io |
| pe_boost_dcm | Boost converter in discontinuous conduction | Vin, L, fs, R, D | K, M, Vout, D2, I_pk, Io |
| pe_buck_boost_dcm | Buck-boost converter in discontinuous conduction | Vin, L, fs, R, D | K, M, Vout, D2, I_pk, Io |
| pe_flyback_dcm_design | Flyback magnetizing inductance for discontinuous conduction | Vin, Po, eff, fs, D | Pin, Lm, I_pk, I_rms, I_avg, E_cycle |
| pe_boost_nonideal_gain | Boost converter with inductor resistance: gain and efficiency | Vin, D, R, RL | M, Vout, efficiency, IL, D_peak, M_max |
| pe_buck_with_losses | Buck converter efficiency with conduction and switching losses | Vin, Vout, Io, fs, Rds, RL, VF, tsw | D, P_switch_cond, P_diode, P_inductor, P_switching, P_loss, efficiency |
| pe_converter_small_signal | Control-to-output transfer function of a DC-DC converter | topology, Vin, D, L, C, R | Gvd0, f0, Q, f_rhpz, rhp_zero |
| pe_peak_current_mode_slope | Peak current mode: subharmonic stability and slope compensation | Vin, Vout, L, fs, ma | D, m1, m2, alpha, stable, ma_min, ma_deadbeat |
| pe_three_phase_diode_bridge | Three-phase diode bridge rectifier with resistive load | VLL, f, R | Vdc, Vrms, ripple_factor, form_factor, f_ripple, PIV, Idc, I_diode_avg, I_diode_rms, Is_rms |
| pe_three_phase_half_wave_rectifier | Three-phase half-wave (three-pulse) diode rectifier | Vph, f, R | Vdc, Vrms, ripple_factor, form_factor, f_ripple, PIV, Idc, I_diode_avg, I_diode_rms |
| pe_single_phase_controlled_rectifier | Single-phase phase-controlled thyristor rectifier | circuit, Vs, alpha | Vdc, Vrms, Vac_rms, mode |
| pe_three_phase_controlled_rectifier | Three-phase fully controlled thyristor bridge | VLL, alpha, Id | Vdc, Vrms, P, I_thy_avg, I_thy_rms, Is_rms, mode |
| pe_ac_voltage_controller | Single-phase AC voltage controller (resistive load) | Vs, alpha, R | Vo_rms, Io_rms, P, PF, I_scr_avg, I_scr_rms |
| pe_rectifier_line_current_pf | Rectifier line current: THD, displacement and power factor | circuit, Id, Vs, alpha | Is_rms, Is1_rms, THD, DPF, PF, P, S |
| pe_rectifier_line_harmonics | Rectifier line-current harmonic spectrum | circuit, Id, h | Ih, Ih_over_I1, THD_to_h, I1 |
| pe_half_wave_rl_rectifier | Half-wave diode rectifier with an RL load | Vs, f, R, L | beta_deg, theta_deg, Z, V_avg, I_avg |
| pe_capacitor_filter_exact | Capacitor-input rectifier filter: exact ripple and peak diode current | circuit, Vp, f, R, C | theta_off_deg, alpha_deg, dV, V_avg, I_D_peak, conduction_deg |
| pe_single_phase_overlap | Commutation overlap in a single-phase thyristor bridge | Vs, f, Ls, Id, alpha | u_deg, Vdc, dV |
| pe_three_phase_overlap | Commutation overlap in a three-phase thyristor bridge | VLL, f, Ls, Id, alpha | u_deg, Vdc, dV |
| pe_thd_from_harmonics | Total harmonic distortion from harmonic values | V1, Vh | THD, THD_pct, Vrms, distortion_factor |
| pe_power_factor_distortion | Power factor with harmonic distortion | THD, phi | DPF, distortion_factor, PF |
| pe_pulse_waveform_rms | Average and RMS of a rectangular pulse train | A, D | avg, rms, ac_rms, form_factor, ripple_factor |
| pe_square_wave_inverter | Square-wave inverter output and harmonics | Vdc, bridge, n | Vrms, V1_rms, V1_peak, Vn_rms, THD |
| pe_quasi_square_inverter | Quasi-square-wave (phase-shifted) full-bridge inverter | Vdc, alpha, n | V1_rms, Vn_rms, Vrms, THD, alpha_eliminate_n_deg |
| pe_six_step_inverter | Three-phase six-step (square-wave) inverter | Vdc, f | VLL_rms, VLL1_rms, Vph_rms, Vph1_rms, THD, VLL5_rms, VLL7_rms, f5, f7 |
| pe_spwm_fundamental | Sinusoidal PWM: fundamental output voltage | Vdc, ma, mf, f1 | V1_leg_peak, V1_leg_rms, V1_LL_rms, V1_fullbridge_rms, f_carrier |
| pe_space_vector_times | Space-vector modulation dwell times | Vref, theta, Vdc, fs | sector, T1, T2, T0, m |
| pe_dead_time_error | Inverter dead-time voltage error | Vdc, td, fs, V1 | dV_avg, dV1_peak, dV1_pct |
| pe_inverter_rl_load_current | Square-wave inverter with an RL load: current | Vdc, f, R, L | I_max, I_rms, P, Is_avg |
| pe_igbt_inverter_conduction_losses | IGBT and diode conduction losses in a sinusoidal PWM inverter | Ip, M, phi, Vce0, rce, VF0, rF | P_igbt, P_diode, P_switch_pair, P_leg |
| pe_dc_motor_chopper_drive | Chopper-fed separately excited DC motor: speed and torque | Vs, D, Ra, K, TL | Ia, Va, Ea, omega, rpm, P_out, efficiency |
| pe_bldc_six_step | BLDC motor with six-step (120 degree) drive | Vdc, D, Rph, Keph, I, p | omega, rpm, T, omega_no_load, I_stall, f_e, t_commutation |
| pe_dc_drive_controlled_rectifier | Phase-controlled rectifier DC motor drive | converter, Vs, alpha, Ra, K, Ia | Va, Ea, omega, rpm, T, P, mode |
| pe_regenerative_braking_chopper | Regenerative braking of a DC motor with a chopper | Vs, D, Ra, K, omega | Ea, Va, Ia, T_brake, P_mech, P_regen, efficiency |
| pe_dc_motor_armature_ripple | Armature current ripple of a chopper-fed DC motor | Vs, D, fs, Ra, La, Ea | I_max, I_min, dI, I_avg |
| pe_induction_motor_vf | Induction motor constant V/f speed control | V_rated, f_rated, poles, n_rated, f, T_ratio | V, ns, n, slip, f_slip |
| pe_pmsm_dq_model | PMSM steady state in the dq frame: voltages and torque | Rs, Ld, Lq, psi_f, p, id, iq, omega_m | omega_e, vd, vq, V_peak, Te, P_mech, P_in, P_cu, PF |
| pe_pmsm_mtpa | Maximum torque per ampere (MTPA) of an interior PMSM | Ld, Lq, psi_f, p, Is | id, iq, beta_deg, Te, Te_id0 |
| pe_pmsm_field_weakening | Surface PMSM field weakening: base speed, maximum speed and torque | Ls, psi_f, p, Imax, Vmax, omega | omega_base, omega_max, rpm_base, id, iq, T, P |
| pe_mosfet_conduction_loss | MOSFET conduction loss at junction temperature | Rds25, alpha, Tj, Irms | Rds_T, P, P_25, ratio |
| pe_switching_loss | Hard-switching loss of a power transistor | V, I, tr, tf, fs, Coss | E_on, E_off, P_sw, P_oss, P_total |
| pe_gate_drive_power | Gate-drive power and currents | Qg, Vdrv, fs, Rg | P_drive, Ig_avg, Ig_peak, E_cycle |
| pe_diode_reverse_recovery | Diode reverse-recovery charge and losses | Irm, trr, S, Vr, fs | Qrr, ta, tb, E_diode, E_switch, P_diode, P_switch, P_total |
| pe_parallel_mosfet_sharing | Current sharing between paralleled MOSFETs | R, I | currents, losses, P_total, imbalance |
| pe_bootstrap_capacitor | Bootstrap capacitor for a high-side gate driver | Qg, Ileak, fs, Dmax, dV | Q_total, C_min, C_recommended |
| pe_turnoff_snubber | RCD turn-off snubber design | I, V, tf, fs | Cs, E_off_with, E_off_without, E_snubber, P_switch, P_snubber, P_total |
| pe_rc_snubber_ringing | RC snubber from measured ringing frequencies | f0, f1, Cadd, k, V, fs | Cp, Lp, Rs, Cs, P_R |
| pe_zvs_transition | Zero-voltage switching: resonant transition of a switch node | Vin, Ceq, L, I0 | Z, f_res, I_min, E_L, E_C, zvs, t_transition, v_reached |
| pe_thermal_shared_heatsink | Several power devices on one heat sink | P, Rjc, Rcs, Rsa, Ta | P_total, Ts, Tj, Tj_max, Tj_max_C |
| pe_foster_transient_thermal | Transient thermal impedance of a Foster network | R, tau, P, tp | Zth, dTj, Rth_total, dTj_steady |
| pe_foster_periodic_pulses | Junction temperature under periodic power pulses | R, tau, P, tp, T | dT_peak, dT_min, dT_avg, ripple |
| pe_transformer_turns | Transformer turns from Faraday's law | waveform, V, f, Bmax, Ac, Vs | Np, Ns, n, volts_per_turn |
| pe_gapped_inductor_design | Gapped-core inductor: turns and air gap | L, Ipk, Bmax, Ac | N, l_gap, E, AL, V_gap |
| pe_inductor_area_product | Inductor core area product | L, Ipk, Irms, Bmax, J, Ku | Ap, Ap_cm4 |
| pe_transformer_area_product | Transformer core area product | Po, eff, waveform, Bm, f, J, Ku | Pt, Ap, Ap_cm4 |
| pe_steinmetz_core_loss | Core loss by the Steinmetz equation | k, alpha, beta, f, Bpk, Vc | Pv, Pv_mW_cm3, P |
| pe_igse_triangular_core_loss | Core loss under triangular flux (iGSE) | k, alpha, beta, f, dB, D, Vc | ki, Pv, P, ratio_to_sine |
| pe_skin_effect_round_wire | Skin effect in a round wire | d, f, rho, length | delta, d_over_delta, Rdc, Rac, ratio |
| pe_dowell_proximity_factor | Winding AC resistance factor (Dowell) | h, f, rho, m | delta, Delta, FR |
| pe_series_resonant_gain | Series resonant converter: tank gain (first harmonic) | Lr, Cr, R, f | fr, Z0, Q, fn, M |
| pe_llc_resonant_gain | LLC resonant converter: tank gain (first harmonic) | Lr, Cr, Lm, n, R, f | fr1, fr2, Rac, Q, k, M |
| pe_dual_active_bridge_power | Dual active bridge: power transfer and inductor current | V1, V2, n, L, fs, phi | P, P_max, i0, i_phi, I_rms |
| pe_hold_up_capacitor | Hold-up (bulk) capacitor | Po, t_hold, V1, V2 | C, E_used, E_initial, E_remaining |
| pe_boost_pfc | Boost power-factor-correction stage | Vac, f_line, Vo, Po, eff, L, fs, C | I_in_pk, D_peak, dI_peak, dI_max, IL_at_peak, dVo_pp |

<a id="pe_buck_ccm"></a>

### Buck converter in continuous conduction: `pe_buck_ccm`

The duty cycle, inductor ripple, peak and valley inductor currents, ripple ratio and average input current of an ideal buck (step-down) converter in continuous conduction.

$$
D = \frac{V_o}{V_{in}},\quad \Delta i_L = \frac{(V_{in}-V_o)D}{L f_s},\quad I_{L,\max/\min} = I_o \pm \tfrac{\Delta i_L}{2}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `Io` | load (output) current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `dIL` | peak-to-peak inductor ripple |
| `IL_max` | peak inductor current |
| `IL_min` | valley inductor current |
| `ripple_ratio` | ripple ratio dIL/Io |
| `Iin` | average input current |

- **Method:** D = Vout/Vin; dIL = (Vin - Vout) D/(L fs); $IL = Io$; IL_max,min = Io +- dIL/2; Iin = D Io
- **Assumes:** 0 < Vout < Vin; continuous conduction (IL_min > 0)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dIL (peak-to-peak inductor ripple) against Vin (input voltage), with Io = 3 A, L = 2](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-buck-ccm-dil-vs-vin.svg)

*dIL (peak-to-peak inductor ripple) against Vin (input voltage), with Io = 3 A, L = 2.20e-5 H, Vout = 5 V, fs = 2.00e5 Hz. 31 points computed by power-electronics-ai999x; marked: Vin = 24, dIL = 0.899621 A.*

![dIL (peak-to-peak inductor ripple) against Vout (output voltage) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-ccm-curve)

*dIL (peak-to-peak inductor ripple) against Vout (output voltage) from 0.5 V to 50 V on a logarithmic axis. One curve for each value of L (inductance): 2.2000e-05 H, 1.1000e-05 H, 4.4000e-05 H. Other inputs as in the worked example: Vin = 24.0 V, fs = 200000.0 Hz, Io = 3.0 A. 34 points per curve computed by power-electronics-ai999x; marked: the worked example, Vout = 5 V, dIL = 0.899621 A.*

```python
power_electronics_ai999x.solve('pe_buck_ccm', Vin=24.0, Vout=5.0, L=2.2e-05, fs=200000.0, Io=3.0)
# -> D = 0.208333, dIL = 0.899621 A, IL_max = 3.44981 A, IL_min = 2.55019 A, ripple_ratio = 0.299874, Iin = 0.625 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_ccm', Vin=48.0, Vout=12.0, L=4.7e-05, fs=100000.0, Io=5.0)
# -> D = 0.25, dIL = 1.91489 A, IL_max = 5.95745 A, IL_min = 4.04255 A, ripple_ratio = 0.382979, Iin = 1.25 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A | SOLVED | D = 0.208333, dIL = 0.899621 A, IL_max = 3.44981 A, IL_min = 2.55019 A, ripple_ratio = 0.299874, Iin = 0.625 A |
| Buck converter in CCM: Vin = 48 V, Vout = 12 V, L = 47 uH, fs = 100 kHz, Io = 5 A | SOLVED | D = 0.25, dIL = 1.91489 A, IL_max = 5.95745 A, IL_min = 4.04255 A, ripple_ratio = 0.382979, Iin = 1.25 A |
| Buck converter in CCM: Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A | NEEDS_INPUT | pe_buck_ccm needs Vin (input voltage). Example: solve('pe_buck_ccm', Vin='24 V', Vout='5 V', L='2.2e-05 H', fs='200000 H |
| Buck converter in CCM: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, Io = 3 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_boost_ccm"></a>

### Boost converter in continuous conduction: `pe_boost_ccm`

The duty cycle, average inductor (input) current, inductor ripple, peak and valley currents and switch voltage stress of an ideal boost (step-up) converter in continuous conduction.

$$
D = 1-\frac{V_{in}}{V_o},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `Io` | load (output) current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `IL` | average inductor current (input current) |
| `dIL` | peak-to-peak inductor ripple |
| `IL_max` | peak inductor current |
| `IL_min` | valley inductor current |
| `V_switch` | switch and diode voltage stress |

- **Method:** D = 1 - Vin/Vout; $IL = Io/(1 - D)$; dIL = Vin D/(L fs)
- **Assumes:** Vout > Vin > 0; continuous conduction (IL_min > 0)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dIL (peak-to-peak inductor ripple) against Vin (input voltage) from 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-boost-ccm-curve)

*dIL (peak-to-peak inductor ripple) against Vin (input voltage) from 1.2 V to 120 V on a logarithmic axis. One curve for each value of L (inductance): 1.0000e-04 H, 5.0000e-05 H, 0.0002 H. Other inputs as in the worked example: Vout = 48.0 V, fs = 100000.0 Hz, Io = 1.0 A. 33 points per curve computed by power-electronics-ai999x; marked: the worked example, Vin = 12 V, dIL = 0.9 A.*

```python
power_electronics_ai999x.solve('pe_boost_ccm', Vin=12.0, Vout=48.0, L=0.0001, fs=100000.0, Io=1.0)
# -> D = 0.75, IL = 4 A, dIL = 0.9 A, IL_max = 4.45 A, IL_min = 3.55 A, V_switch = 48 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_boost_ccm', Vin=5.0, Vout=12.0, L=1e-05, fs=500000.0, Io=0.8)
# -> D = 0.583333, IL = 1.92 A, dIL = 0.583333 A, IL_max = 2.21167 A, IL_min = 1.62833 A, V_switch = 12 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Boost converter in CCM: Vin = 12 V, Vout = 48 V, L = 100 uH, fs = 100 kHz, Io = 1 A | SOLVED | D = 0.75, IL = 4 A, dIL = 0.9 A, IL_max = 4.45 A, IL_min = 3.55 A, V_switch = 48 V |
| Boost converter in CCM: Vin = 5 V, Vout = 12 V, L = 10 uH, fs = 500 kHz, Io = 0.8 A | SOLVED | D = 0.583333, IL = 1.92 A, dIL = 0.583333 A, IL_max = 2.21167 A, IL_min = 1.62833 A, V_switch = 12 V |
| Boost converter in CCM: Vout = 48 V, L = 100 uH, fs = 100 kHz, Io = 1 A | NEEDS_INPUT | pe_boost_ccm needs Vin (input voltage). Example: solve('pe_boost_ccm', Vin='12 V', Vout='48 V', L='0.0001 H', fs='100000 |
| Boost converter in CCM: Vin = 12 V, Vout = 48 V, L = 100 uH, fs = 100 kHz, Io = 1 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_buck_boost_ccm"></a>

### Buck-boost converter in continuous conduction, `pe_buck_boost_ccm`

The duty cycle, average inductor current, ripple, peak and valley currents and switch voltage stress of an ideal inverting buck-boost converter in continuous conduction (output voltage as a magnitude).

$$
D = \frac{|V_o|}{V_{in}+|V_o|},\quad I_L = \frac{I_o}{1-D},\quad \Delta i_L = \frac{V_{in}D}{L f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage magnitude (the output is inverted) |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `Io` | load (output) current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `IL` | average inductor current |
| `dIL` | peak-to-peak inductor ripple |
| `IL_max` | peak inductor current |
| `IL_min` | valley inductor current |
| `V_switch` | switch voltage stress Vin + Vout |

- **Method:** D = Vout/(Vin + Vout); $IL = Io/(1 - D)$; dIL = Vin D/(L fs)
- **Assumes:** Vin, Vout > 0; continuous conduction (IL_min > 0)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dIL (peak-to-peak inductor ripple) against L (inductance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-boost-ccm-curve)

*dIL (peak-to-peak inductor ripple) against L (inductance) from 2.3500e-05 H to 7.0500e-05 H. One curve for each value of fs (switching frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: Vin = 12.0 V, Vout = 15.0 V, Io = 1.0 A. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 4.7000e-05 H, dIL = 1.41844 A.*

```python
power_electronics_ai999x.solve('pe_buck_boost_ccm', Vin=12.0, Vout=15.0, L=4.7e-05, fs=100000.0, Io=1.0)
# -> D = 0.555556, IL = 2.25 A, dIL = 1.41844 A, IL_max = 2.95922 A, IL_min = 1.54078 A, V_switch = 27 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_boost_ccm', Vin=24.0, Vout=12.0, L=3.3e-05, fs=250000.0, Io=2.0)
# -> D = 0.333333, IL = 3 A, dIL = 0.969697 A, IL_max = 3.48485 A, IL_min = 2.51515 A, V_switch = 36 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck-boost converter in CCM: Vin = 12 V, Vout = 15 V, L = 47 uH, fs = 100 kHz, Io = 1 A | SOLVED | D = 0.555556, IL = 2.25 A, dIL = 1.41844 A, IL_max = 2.95922 A, IL_min = 1.54078 A, V_switch = 27 V |
| Buck-boost converter in CCM: Vin = 24 V, Vout = 12 V, L = 33 uH, fs = 250 kHz, Io = 2 A | SOLVED | D = 0.333333, IL = 3 A, dIL = 0.969697 A, IL_max = 3.48485 A, IL_min = 2.51515 A, V_switch = 36 V |
| Buck-boost converter in CCM: Vout = 15 V, L = 47 uH, fs = 100 kHz, Io = 1 A | NEEDS_INPUT | pe_buck_boost_ccm needs Vin (input voltage). Example: solve('pe_buck_boost_ccm', Vin='12 V', Vout='15 V', L='4.7e-05 H', |
| Buck-boost converter in CCM: Vin = 12 V, Vout = 15 V, L = 47 uH, fs = 100 kHz, Io = 1 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_conversion_ratio"></a>

### Ideal DC-DC conversion ratio from the duty cycle, `pe_conversion_ratio`

The ideal continuous-conduction voltage conversion ratio M = Vout/Vin and output voltage of a buck, boost, buck-boost, Cuk or SEPIC converter at a duty cycle, with the output polarity.

$$
M_{buck} = D,\quad M_{boost} = \frac{1}{1-D},\quad M_{buck\text{-}boost,\,Cuk,\,SEPIC} = \frac{D}{1-D}
$$

| input | meaning | condition |
|---|---|---|
| `topology` | converter topology: buck, boost, buck-boost, Cuk or SEPIC |  |
| `Vin` | input voltage |  |
| `D` | duty cycle (a fraction or %) |  |

| output | meaning |
|---|---|
| `M` | conversion ratio Vout/Vin (magnitude) |
| `Vout` | output voltage magnitude |
| `polarity` | output polarity relative to the input (same or inverted) |

- **Method:** buck $M = D$; boost $M = 1/(1 - D)$; buck-boost, Cuk, SEPIC $M = D/(1 - D)$
- **Assumes:** $0 < D < 1$; ideal, continuous conduction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (conversion ratio Vout/Vin (magnitude)) against D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-conversion-ratio-curve)

*M (conversion ratio Vout/Vin (magnitude)) against D (duty cycle (a fraction or %)) from 0.02 to 0.98. One curve for each value of topology (converter topology: buck, boost, buck-boost, Cuk or SEPIC): boost, buck. Other inputs as in the worked example: Vin = 12.0 V. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.6, M = 2.5.*

```python
power_electronics_ai999x.solve('pe_conversion_ratio', topology='boost', Vin=12.0, D=0.6)
# -> M = 2.5, Vout = 30 V, polarity = same   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_conversion_ratio', topology='Cuk', Vin=24.0, D=0.4)
# -> M = 0.666667, Vout = 16 V, polarity = inverted   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| DC-DC conversion ratio: topology = boost, Vin = 12 V, D = 0.6 | SOLVED | M = 2.5, Vout = 30 V, polarity = same |
| DC-DC conversion ratio: topology = Cuk, Vin = 24 V, D = 0.4 | SOLVED | M = 0.666667, Vout = 16 V, polarity = inverted |
| DC-DC conversion ratio: Vin = 12 V, D = 0.6 | NEEDS_INPUT | pe_conversion_ratio needs topology (converter topology: buck, boost, buck-boost, Cuk or SEPIC). Example: solve('pe_conve |
| DC-DC conversion ratio: topology = boost, Vin = 12 V, D = 0.6, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| DC-DC conversion ratio: topology = Boost, Vin = 12 V, D = 60% | SOLVED | M = 2.5, Vout = 30 V, polarity = same |

<a id="pe_buck_output_ripple"></a>

### Buck converter output voltage ripple: `pe_buck_output_ripple`

The peak-to-peak output voltage ripple of a buck converter in continuous conduction, with the inductor ripple current flowing entirely into the output capacitor (no ESR).

$$
\Delta v_o = \frac{\Delta i_L}{8 C f_s} = \frac{(1-D)V_o}{8 L C f_s^2}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L` | inductance |  |
| `C` | output capacitance |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `dIL` | inductor ripple |
| `dVo` | peak-to-peak output ripple |
| `dVo_pct` | output ripple in percent of Vout |

- **Method:** dIL = (Vin - Vout) D/(L fs); dVo = dIL/(8 C fs) = (1 - D) Vout/(8 L C fs^2)
- **Assumes:** 0 < Vout < Vin; continuous conduction; ripple current into C, ideal capacitor
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dVo (peak-to-peak output ripple) against Vout (output voltage) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-output-ripple-curve)

*dVo (peak-to-peak output ripple) against Vout (output voltage) from 0.5 V to 50 V on a logarithmic axis. One curve for each value of fs (switching frequency): 200000 Hz, 100000 Hz, 400000 Hz. Other inputs as in the worked example: Vin = 24.0 V, L = 2.2e-05 H, C = 9.999999999999999e-05 F. 34 points per curve computed by power-electronics-ai999x; marked: the worked example, Vout = 5 V, dVo = 0.00562263 V.*

```python
power_electronics_ai999x.solve('pe_buck_output_ripple', Vin=24.0, Vout=5.0, L=2.2e-05, C=0.0001, fs=200000.0)
# -> D = 0.208333, dIL = 0.899621 A, dVo = 0.00562263 V, dVo_pct = 0.112453   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_output_ripple', Vin=12.0, Vout=3.3, L=4.7e-06, C=4.7e-05, fs=500000.0)
# -> D = 0.275, dIL = 1.01809 A, dVo = 0.00541535 V, dVo_pct = 0.164101   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck converter output ripple: Vin = 24 V, Vout = 5 V, L = 22 uH, C = 100 uF, fs = 200 kHz | SOLVED | D = 0.208333, dIL = 0.899621 A, dVo = 0.00562263 V, dVo_pct = 0.112453 |
| Buck converter output ripple: Vin = 12 V, Vout = 3.3 V, L = 4.7 uH, C = 47 uF, fs = 500 kHz | SOLVED | D = 0.275, dIL = 1.01809 A, dVo = 0.00541535 V, dVo_pct = 0.164101 |
| Buck converter output ripple: Vout = 5 V, L = 22 uH, C = 100 uF, fs = 200 kHz | NEEDS_INPUT | pe_buck_output_ripple needs Vin (input voltage). Example: solve('pe_buck_output_ripple', Vin='24 V', Vout='5 V', L='2.2e |
| Buck converter output ripple: Vin = 24 V, Vout = 5 V, L = 22 uH, C = 100 uF, fs = 200 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_boost_output_ripple"></a>

### Boost converter output voltage ripple: `pe_boost_output_ripple`

The peak-to-peak output voltage ripple of a boost converter: the capacitor alone supplies the load current during the switch on-time (ideal capacitor, no ESR).

$$
\Delta v_o = \frac{I_o D}{C f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `Io` | load current |  |
| `C` | output capacitance |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `dVo` | peak-to-peak output ripple |
| `dVo_pct` | ripple in percent of Vout |

- **Method:** D = 1 - Vin/Vout; dVo = Io D/(C fs)
- **Assumes:** Vout > Vin > 0; continuous conduction, ideal capacitor
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dVo (peak-to-peak output ripple) against C (output capacitance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-boost-output-ripple-curve)

*dVo (peak-to-peak output ripple) against C (output capacitance) from 2.3500e-05 F to 7.0500e-05 F. One curve for each value of Io (load current): 1 A, 0.5 A, 2 A. Other inputs as in the worked example: Vin = 12.0 V, Vout = 48.0 V, fs = 100000.0 Hz. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, C = 4.7000e-05 F, dVo = 0.159574 V.*

```python
power_electronics_ai999x.solve('pe_boost_output_ripple', Vin=12.0, Vout=48.0, Io=1.0, C=4.7e-05, fs=100000.0)
# -> D = 0.75, dVo = 0.159574 V, dVo_pct = 0.332447   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_boost_output_ripple', Vin=5.0, Vout=12.0, Io=0.5, C=2.2e-05, fs=500000.0)
# -> D = 0.583333, dVo = 0.0265152 V, dVo_pct = 0.22096   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Boost converter output ripple: Vin = 12 V, Vout = 48 V, Io = 1 A, C = 47 uF, fs = 100 kHz | SOLVED | D = 0.75, dVo = 0.159574 V, dVo_pct = 0.332447 |
| Boost converter output ripple: Vin = 5 V, Vout = 12 V, Io = 0.5 A, C = 22 uF, fs = 500 kHz | SOLVED | D = 0.583333, dVo = 0.0265152 V, dVo_pct = 0.22096 |
| Boost converter output ripple: Vout = 48 V, Io = 1 A, C = 47 uF, fs = 100 kHz | NEEDS_INPUT | pe_boost_output_ripple needs Vin (input voltage). Example: solve('pe_boost_output_ripple', Vin='12 V', Vout='48 V', Io=' |
| Boost converter output ripple: Vin = 12 V, Vout = 48 V, Io = 1 A, C = 47 uF, fs = 100 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_cuk_ccm"></a>

### Cuk converter in continuous conduction: `pe_cuk_ccm`

The duty cycle, inductor currents and ripples, coupling-capacitor voltage and its ripple of an ideal Cuk converter (inverted output, given as a magnitude) in continuous conduction.

$$
D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in}+V_o,\ \Delta i_{L1} = \frac{V_{in}D}{L_1 f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage magnitude |  |
| `L1` | input inductance |  |
| `L2` | output inductance |  |
| `C1` | coupling (energy-transfer) capacitance |  |
| `fs` | switching frequency |  |
| `Io` | load current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `IL1` | average input inductor current |
| `IL2` | average output inductor current |
| `VC1` | average coupling-capacitor voltage Vin + Vout |
| `dIL1` | input inductor ripple |
| `dIL2` | output inductor ripple |
| `dVC1` | coupling-capacitor ripple |

- **Method:** D = Vo/(Vin + Vo); $IL1 = Io$ Vo/Vin; VC1 = Vin + Vo; dIL1 = Vin D/(L1 fs); dIL2 = Vo (1 - D)/(L2 fs); dVC1 = Io D/(C1 fs)
- **Assumes:** continuous conduction in both inductors
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dIL1 (input inductor ripple) against L1 (input inductance) from 5](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-cuk-ccm-curve)

*dIL1 (input inductor ripple) against L1 (input inductance) from 5.0000e-05 H to 0.00015 H. One curve for each value of fs (switching frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: Vin = 12.0 V, Vout = 15.0 V, L2 = 9.999999999999999e-05 H, C1 = 9.999999999999999e-06 F, Io = 1.0 A. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L1 = 1.0000e-04 H, dIL1 = 0.666667 A.*

```python
power_electronics_ai999x.solve('pe_cuk_ccm', Vin=12.0, Vout=15.0, L1=0.0001, L2=0.0001, C1=1e-05, fs=100000.0, Io=1.0)
# -> D = 0.555556, IL1 = 1.25 A, IL2 = 1 A, VC1 = 27 V, dIL1 = 0.666667 A, dIL2 = 0.666667 A, dVC1 = 0.555556 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_cuk_ccm', Vin=24.0, Vout=12.0, L1=0.00022, L2=0.00015, C1=2.2e-05, fs=50000.0, Io=2.0)
# -> D = 0.333333, IL1 = 1 A, IL2 = 2 A, VC1 = 36 V, dIL1 = 0.727273 A, dIL2 = 1.06667 A, dVC1 = 0.606061 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cuk converter: Vin = 12 V, Vout = 15 V, L1 = 100 uH, L2 = 100 uH, C1 = 10 uF, fs = 100 kHz, Io = 1 A | SOLVED | D = 0.555556, IL1 = 1.25 A, IL2 = 1 A, VC1 = 27 V, dIL1 = 0.666667 A, dIL2 = 0.666667 A, dVC1 = 0.555556 V |
| Cuk converter: Vin = 24 V, Vout = 12 V, L1 = 220 uH, L2 = 150 uH, C1 = 22 uF, fs = 50 kHz, Io = 2 A | SOLVED | D = 0.333333, IL1 = 1 A, IL2 = 2 A, VC1 = 36 V, dIL1 = 0.727273 A, dIL2 = 1.06667 A, dVC1 = 0.606061 V |
| Cuk converter: Vout = 15 V, L1 = 100 uH, L2 = 100 uH, C1 = 10 uF, fs = 100 kHz, Io = 1 A | NEEDS_INPUT | pe_cuk_ccm needs Vin (input voltage). Example: solve('pe_cuk_ccm', Vin='12 V', Vout='15 V', L1='0.0001 H', L2='0.0001 H' |
| Cuk converter: Vin = 12 V, Vout = 15 V, L1 = 100 uH, L2 = 100 uH, C1 = 10 uF, fs = 100 kHz, Io = 1 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_sepic_ccm"></a>

### SEPIC converter in continuous conduction: `pe_sepic_ccm`

The duty cycle, inductor currents and ripples, coupling-capacitor voltage and ripple and switch voltage stress of an ideal SEPIC (non-inverting buck-boost) converter in continuous conduction.

$$
D = \frac{V_o}{V_{in}+V_o},\ V_{C1} = V_{in},\ \Delta i_{L1,2} = \frac{V_{in}D}{L_{1,2} f_s},\ \Delta v_{C1} = \frac{I_o D}{C_1 f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L1` | input inductance |  |
| `L2` | second inductance |  |
| `C1` | coupling capacitance |  |
| `fs` | switching frequency |  |
| `Io` | load current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `IL1` | average input inductor current |
| `IL2` | average second inductor current |
| `VC1` | average coupling-capacitor voltage (= Vin) |
| `dIL1` | input inductor ripple |
| `dIL2` | second inductor ripple |
| `dVC1` | coupling-capacitor ripple |
| `V_switch` | switch voltage stress Vin + Vout |

- **Method:** D = Vo/(Vin + Vo); $IL1 = Io$ Vo/Vin; $IL2 = Io$; VC1 = Vin; dIL1 = Vin D/(L1 fs); dIL2 = Vin D/(L2 fs); dVC1 = Io D/(C1 fs)
- **Assumes:** continuous conduction in both inductors
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dIL1 (input inductor ripple) against L1 (input inductance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-sepic-ccm-curve)

*dIL1 (input inductor ripple) against L1 (input inductance) from 2.3500e-05 H to 7.0500e-05 H. One curve for each value of fs (switching frequency): 200000 Hz, 100000 Hz, 400000 Hz. Other inputs as in the worked example: Vin = 9.0 V, Vout = 12.0 V, L2 = 4.7e-05 H, C1 = 9.999999999999999e-06 F, Io = 1.0 A. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L1 = 4.7000e-05 H, dIL1 = 0.547112 A.*

```python
power_electronics_ai999x.solve('pe_sepic_ccm', Vin=9.0, Vout=12.0, L1=4.7e-05, L2=4.7e-05, C1=1e-05, fs=200000.0, Io=1.0)
# -> D = 0.571429, IL1 = 1.33333 A, IL2 = 1 A, VC1 = 9 V, dIL1 = 0.547112 A, dIL2 = 0.547112 A, dVC1 = 0.285714 V, V_switch = 21 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_sepic_ccm', Vin=24.0, Vout=12.0, L1=0.0001, L2=0.0001, C1=4.7e-06, fs=100000.0, Io=1.5)
# -> D = 0.333333, IL1 = 0.75 A, IL2 = 1.5 A, VC1 = 24 V, dIL1 = 0.8 A, dIL2 = 0.8 A, dVC1 = 1.06383 V, V_switch = 36 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SEPIC converter: Vin = 9 V, Vout = 12 V, L1 = 47 uH, L2 = 47 uH, C1 = 10 uF, fs = 200 kHz, Io = 1 A | SOLVED | D = 0.571429, IL1 = 1.33333 A, IL2 = 1 A, VC1 = 9 V, dIL1 = 0.547112 A, dIL2 = 0.547112 A, dVC1 = 0.285714 V, V_switch = 21 V |
| SEPIC converter: Vin = 24 V, Vout = 12 V, L1 = 100 uH, L2 = 100 uH, C1 = 4.7 uF, fs = 100 kHz, Io = 1.5 A | SOLVED | D = 0.333333, IL1 = 0.75 A, IL2 = 1.5 A, VC1 = 24 V, dIL1 = 0.8 A, dIL2 = 0.8 A, dVC1 = 1.06383 V, V_switch = 36 V |
| SEPIC converter: Vout = 12 V, L1 = 47 uH, L2 = 47 uH, C1 = 10 uF, fs = 200 kHz, Io = 1 A | NEEDS_INPUT | pe_sepic_ccm needs Vin (input voltage). Example: solve('pe_sepic_ccm', Vin='9 V', Vout='12 V', L1='4.7e-05 H', L2='4.7e- |
| SEPIC converter: Vin = 9 V, Vout = 12 V, L1 = 47 uH, L2 = 47 uH, C1 = 10 uF, fs = 200 kHz, Io = 1 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_flyback_ccm"></a>

### Flyback converter in continuous conduction, `pe_flyback_ccm`

The duty cycle, magnetizing current (primary-referred) and ripple, peak primary and secondary currents and the switch and diode voltage stresses of an ideal flyback converter in continuous conduction.

$$
\frac{V_o}{V_{in}} = \frac{1}{n}\frac{D}{1-D},\quad I_m = \frac{I_o}{n(1-D)},\quad \Delta i_m = \frac{V_{in}D}{L_m f_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `n` | turns ratio Np/Ns |  |
| `Lm` | magnetizing inductance (primary) |  |
| `fs` | switching frequency |  |
| `Io` | load current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `Im` | average magnetizing current referred to the primary |
| `dIm` | magnetizing current ripple |
| `Ipk_primary` | peak primary (switch) current |
| `Ipk_secondary` | peak secondary (diode) current |
| `V_switch` | switch voltage stress Vin + n Vo |
| `V_diode` | diode reverse voltage Vo + Vin/n |

- **Method:** $D = n$ Vo/(Vin + n Vo); Im = Io/(n $(1 - D)$); dIm = Vin D/(Lm fs)
- **Assumes:** continuous conduction of the magnetizing current; no leakage inductance
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![D (duty cycle) against Vin (input voltage), with Io = 2 A, Lm = 2](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-flyback-ccm-d-vs-vin.svg)

*D (duty cycle) against Vin (input voltage), with Io = 2 A, Lm = 2.00e-4 H, Vout = 12 V, fs = 1.00e5 Hz, n = 2. 31 points computed by power-electronics-ai999x; marked: Vin = 48, D = 0.333333.*

![D (duty cycle) against Vin (input voltage) from 4](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-flyback-ccm-curve)

*D (duty cycle) against Vin (input voltage) from 4.8 V to 480 V on a logarithmic axis. One curve for each value of Vout (output voltage): 12 V, 6 V, 24 V. Other inputs as in the worked example: n = 2.0, Lm = 0.00019999999999999998 H, fs = 100000.0 Hz, Io = 2.0 A. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, Vin = 48 V, D = 0.333333.*

```python
power_electronics_ai999x.solve('pe_flyback_ccm', Vin=48.0, Vout=12.0, n=2.0, Lm=0.0002, fs=100000.0, Io=2.0)
# -> D = 0.333333, Im = 1.5 A, dIm = 0.8 A, Ipk_primary = 1.9 A, Ipk_secondary = 3.8 A, V_switch = 72 V, V_diode = 36 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_flyback_ccm', Vin=325.0, Vout=5.0, n=20.0, Lm=0.006, fs=65000.0, Io=3.0)
# -> D = 0.235294, Im = 0.196154 A, dIm = 0.196078 A, Ipk_primary = 0.294193 A, Ipk_secondary = 5.88386 A, V_switch = 425 V, V_diode = 21.25 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Flyback converter: Vin = 48 V, Vout = 12 V, n = 2, Lm = 200 uH, fs = 100 kHz, Io = 2 A | SOLVED | D = 0.333333, Im = 1.5 A, dIm = 0.8 A, Ipk_primary = 1.9 A, Ipk_secondary = 3.8 A, V_switch = 72 V, V_diode = 36 V |
| Flyback converter: Vin = 325 V, Vout = 5 V, n = 20, Lm = 6000 uH, fs = 65 kHz, Io = 3 A | SOLVED | D = 0.235294, Im = 0.196154 A, dIm = 0.196078 A, Ipk_primary = 0.294193 A, Ipk_secondary = 5.88386 A, V_switch = 425 V, V_diode = 21.25 V |
| Flyback converter: Vout = 12 V, n = 2, Lm = 200 uH, fs = 100 kHz, Io = 2 A | NEEDS_INPUT | pe_flyback_ccm needs Vin (input voltage). Example: solve('pe_flyback_ccm', Vin='48 V', Vout='12 V', n=2, Lm='0.0002 H',  |
| Flyback converter: Vin = 48 V, Vout = 12 V, n = 2, Lm = 200 uH, fs = 100 kHz, Io = 2 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_forward_ccm"></a>

### Forward converter with reset winding: `pe_forward_ccm`

The duty cycle, the largest duty cycle the reset winding allows, the switch voltage stress and the output inductor ripple of a single-switch forward converter in continuous conduction.

$$
V_o = \frac{V_{in} D}{n},\quad D_{max} = \frac{1}{1+N_r/N_p},\quad V_{sw} = V_{in}\left(1+\frac{N_p}{N_r}\right)
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `n` | turns ratio Np/Ns |  |
| `Nr` | reset-winding to primary turns ratio Nr/Np |  |
| `L` | output inductance |  |
| `fs` | switching frequency |  |
| `Io` | load current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `D_max` | largest duty cycle that resets the core |
| `V_switch` | switch voltage stress |
| `dIL` | output inductor ripple |
| `IL_max` | peak output inductor current |
| `IL_min` | valley output inductor current |

- **Method:** $D = n$ Vo/Vin; D_max = 1/(1 + Nr/Np); Vsw = Vin $(1 + Np/Nr)$; dIL = (Vin/n - Vo) D/(L fs)
- **Assumes:** D < D_max; continuous conduction of the output inductor
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![D (duty cycle) against Vin (input voltage) from 24 V to 72 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-forward-ccm-curve)

*D (duty cycle) against Vin (input voltage) from 24 V to 72 V. One curve for each value of Vout (output voltage): 5 V, 2.5 V, 10 V. Other inputs as in the worked example: n = 3.0, Nr = 1.0, L = 2.2e-05 H, fs = 200000.0 Hz, Io = 5.0 A. 27 points per curve computed by power-electronics-ai999x; marked: the worked example, Vin = 48 V, D = 0.3125.*

```python
power_electronics_ai999x.solve('pe_forward_ccm', Vin=48.0, Vout=5.0, n=3.0, Nr=1.0, L=2.2e-05, fs=200000.0, Io=5.0)
# -> D = 0.3125, D_max = 0.5, V_switch = 96 V, dIL = 0.78125 A, IL_max = 5.39062 A, IL_min = 4.60938 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_forward_ccm', Vin=300.0, Vout=12.0, n=8.0, Nr=1.0, L=0.0001, fs=100000.0, Io=4.0)
# -> D = 0.32, D_max = 0.5, V_switch = 600 V, dIL = 0.816 A, IL_max = 4.408 A, IL_min = 3.592 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Forward converter: Vin = 48 V, Vout = 5 V, n = 3, Nr = 1, L = 22 uH, fs = 200 kHz, Io = 5 A | SOLVED | D = 0.3125, D_max = 0.5, V_switch = 96 V, dIL = 0.78125 A, IL_max = 5.39062 A, IL_min = 4.60938 A |
| Forward converter: Vin = 300 V, Vout = 12 V, n = 8, Nr = 1, L = 100 uH, fs = 100 kHz, Io = 4 A | SOLVED | D = 0.32, D_max = 0.5, V_switch = 600 V, dIL = 0.816 A, IL_max = 4.408 A, IL_min = 3.592 A |
| Forward converter: Vout = 5 V, n = 3, Nr = 1, L = 22 uH, fs = 200 kHz, Io = 5 A | NEEDS_INPUT | pe_forward_ccm needs Vin (input voltage). Example: solve('pe_forward_ccm', Vin='48 V', Vout='5 V', n=3, Nr=1, L='2.2e-05 |
| Forward converter: Vin = 48 V, Vout = 5 V, n = 3, Nr = 1, L = 22 uH, fs = 200 kHz, Io = 5 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_bridge_converter"></a>

### Push-pull, half-bridge and full-bridge converters, `pe_bridge_converter`

The duty cycle per switch, switch voltage stress, output ripple frequency and output inductor ripple of an ideal push-pull, half-bridge or full-bridge DC-DC converter with a centre-tapped (full-wave) rectifier, in continuous conduction.

$$
V_o = \frac{2 D V_{in}}{n}\ (\text{push-pull, full-bridge}),\quad V_o = \frac{D V_{in}}{n}\ (\text{half-bridge})
$$

| input | meaning | condition |
|---|---|---|
| `topology` | push-pull, half-bridge or full-bridge |  |
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `n` | turns ratio Np/Ns (primary to each secondary half) |  |
| `L` | output inductance |  |
| `fs` | switching frequency |  |
| `Io` | load current |  |

| output | meaning |
|---|---|
| `D` | duty cycle of each switch |
| `V_switch` | switch voltage stress |
| `f_ripple` | output ripple frequency |
| `dIL` | output inductor ripple |
| `IL_min` | valley output inductor current |

- **Method:** $Vo = 2$ D Vin/n (push-pull, full-bridge), $Vo = D$ Vin/n (half-bridge); dIL = (Vsec - Vo) D/(L fs)
- **Assumes:** $D < 0.5$; continuous conduction of the output inductor
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![D (duty cycle of each switch) against Vin (input voltage) from 200 V to 600 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-bridge-converter-curve)

*D (duty cycle of each switch) against Vin (input voltage) from 200 V to 600 V. One curve for each value of Vout (output voltage): 48 V, 24 V, 96 V. Other inputs as in the worked example: topology = full-bridge, n = 5.0, L = 4.9999999999999996e-05 H, fs = 100000.0 Hz, Io = 10.0 A. 27 points per curve computed by power-electronics-ai999x; marked: the worked example, Vin = 400 V, D = 0.3.*

```python
power_electronics_ai999x.solve('pe_bridge_converter', topology='full-bridge', Vin=400.0, Vout=48.0, n=5.0, L=5e-05, fs=100000.0, Io=10.0)
# -> D = 0.3, V_switch = 400 V, f_ripple = 200000 Hz, dIL = 1.92 A, IL_min = 9.04 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_bridge_converter', topology='push-pull', Vin=24.0, Vout=12.0, n=1.0, L=2.2e-05, fs=100000.0, Io=3.0)
# -> D = 0.25, V_switch = 48 V, f_ripple = 200000 Hz, dIL = 1.36364 A, IL_min = 2.31818 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Isolated bridge converter: topology = full-bridge, Vin = 400 V, Vout = 48 V, n = 5, L = 50 uH, fs = 100 kHz, Io = 10 A | SOLVED | D = 0.3, V_switch = 400 V, f_ripple = 200000 Hz, dIL = 1.92 A, IL_min = 9.04 A |
| Isolated bridge converter: topology = push-pull, Vin = 24 V, Vout = 12 V, n = 1, L = 22 uH, fs = 100 kHz, Io = 3 A | SOLVED | D = 0.25, V_switch = 48 V, f_ripple = 200000 Hz, dIL = 1.36364 A, IL_min = 2.31818 A |
| Isolated bridge converter: Vin = 400 V, Vout = 48 V, n = 5, L = 50 uH, fs = 100 kHz, Io = 10 A | NEEDS_INPUT | pe_bridge_converter needs topology (push-pull, half-bridge or full-bridge). Example: solve('pe_bridge_converter', topolo |
| Isolated bridge converter: topology = full-bridge, Vin = 400 V, Vout = 48 V, n = 5, L = 50 uH, fs = 100 kHz, Io = 10 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_critical_inductance"></a>

### Critical inductance at the CCM/DCM boundary, `pe_critical_inductance`

The smallest inductance that keeps a buck, boost or buck-boost converter in continuous conduction at a load resistance (the inductor current just reaches zero at the end of each period).

$$
L_{c,buck} = \frac{(1-D)R}{2f_s},\ L_{c,boost} = \frac{D(1-D)^2R}{2f_s},\ L_{c,buck\text{-}boost} = \frac{(1-D)^2R}{2f_s}
$$

| input | meaning | condition |
|---|---|---|
| `topology` | buck, boost or buck-boost |  |
| `Vin` | input voltage |  |
| `Vout` | output voltage (magnitude) |  |
| `fs` | switching frequency |  |
| `R` | load resistance |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `L_crit` | critical (boundary) inductance |
| `Io` | load current |

- **Method:** buck $Lc = (1 - D)$ R/(2 fs); boost $Lc = D$ $(1 - D)^{2}$ R/(2 fs); buck-boost $Lc = (1 - D)^{2}$ R/(2 fs)
- **Assumes:** ideal converter at the boundary of continuous conduction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![L_crit (critical (boundary) inductance) against fs (switching frequency) from 50000 Hz to 150000 Hz](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-critical-inductance-curve)

*L_crit (critical (boundary) inductance) against fs (switching frequency) from 50000 Hz to 150000 Hz. One curve for each value of R (load resistance): 10 ohm, 5 ohm, 20 ohm. Other inputs as in the worked example: topology = buck, Vin = 24.0 V, Vout = 12.0 V. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, fs = 100000 Hz, L_crit = 2.5000e-05 H.*

```python
power_electronics_ai999x.solve('pe_critical_inductance', topology='buck', Vin=24.0, Vout=12.0, fs=100000.0, R=10.0)
# -> D = 0.5, L_crit = 2.5e-05 H, Io = 1.2 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_critical_inductance', topology='boost', Vin=12.0, Vout=36.0, fs=50000.0, R=50.0)
# -> D = 0.666667, L_crit = 3.7037e-05 H, Io = 0.72 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Critical inductance: topology = buck, Vin = 24 V, Vout = 12 V, fs = 100 kHz, R = 10 ohm | SOLVED | D = 0.5, L_crit = 2.5e-05 H, Io = 1.2 A |
| Critical inductance: topology = boost, Vin = 12 V, Vout = 36 V, fs = 50 kHz, R = 50 ohm | SOLVED | D = 0.666667, L_crit = 3.7037e-05 H, Io = 0.72 A |
| Critical inductance: Vin = 24 V, Vout = 12 V, fs = 100 kHz, R = 10 ohm | NEEDS_INPUT | pe_critical_inductance needs topology (buck, boost or buck-boost). Example: solve('pe_critical_inductance', topology='bu |
| Critical inductance: topology = buck, Vin = 24 V, Vout = 12 V, fs = 100 kHz, R = 10 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_buck_lc_design"></a>

### Buck converter inductor and capacitor design, `pe_buck_lc_design`

The inductance that gives a chosen ripple ratio and the output capacitance that limits the output ripple to a chosen value in a buck converter (continuous conduction, ideal capacitor).

$$
L = \frac{(V_{in}-V_o)D}{r I_o f_s},\quad C = \frac{r I_o}{8 f_s \Delta v_o}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `Io` | load current |  |
| `fs` | switching frequency |  |
| `r` | ripple ratio dIL/Io (a fraction or %) |  |
| `dV` | allowed peak-to-peak output ripple |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `L` | required inductance |
| `C` | required output capacitance |
| `IL_max` | peak inductor current |
| `dIL` | inductor ripple |

- **Method:** L = (Vin - Vo) D/(r Io fs); $C = r$ Io/(8 fs dV)
- **Assumes:** 0 < Vout < Vin; $0 < r < 2$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![L (required inductance) against Vout (output voltage) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-lc-design-curve)

*L (required inductance) against Vout (output voltage) from 0.33 V to 33 V on a logarithmic axis. One curve for each value of r (ripple ratio dIL/Io (a fraction or %)): 0.3, 0.26, 0.74. Other inputs as in the worked example: Vin = 12.0 V, Io = 5.0 A, fs = 500000.0 Hz, dV = 0.01 V. 32 points per curve computed by power-electronics-ai999x; marked: the worked example, Vout = 3.3 V, L = 3.1900e-06 H.*

```python
power_electronics_ai999x.solve('pe_buck_lc_design', Vin=12.0, Vout=3.3, Io=5.0, fs=500000.0, r=0.3, dV=0.01)
# -> D = 0.275, L = 3.19e-06 H, C = 3.75e-05 F, IL_max = 5.75 A, dIL = 1.5 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_lc_design', Vin=48.0, Vout=12.0, Io=10.0, fs=100000.0, r=0.4, dV=0.05)
# -> D = 0.25, L = 2.25e-05 H, C = 0.0001 F, IL_max = 12 A, dIL = 4 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck LC filter design: Vin = 12 V, Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 0.3, dV = 0.01 V | SOLVED | D = 0.275, L = 3.19e-06 H, C = 3.75e-05 F, IL_max = 5.75 A, dIL = 1.5 A |
| Buck LC filter design: Vin = 48 V, Vout = 12 V, Io = 10 A, fs = 100 kHz, r = 0.4, dV = 0.05 V | SOLVED | D = 0.25, L = 2.25e-05 H, C = 0.0001 F, IL_max = 12 A, dIL = 4 A |
| Buck LC filter design: Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 0.3, dV = 0.01 V | NEEDS_INPUT | pe_buck_lc_design needs Vin (input voltage). Example: solve('pe_buck_lc_design', Vin='12 V', Vout='3.3 V', Io='5 A', fs= |
| Buck LC filter design: Vin = 12 V, Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 0.3, dV = 0.01 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Buck LC filter design: Vin = 12 V, Vout = 3.3 V, Io = 5 A, fs = 500 kHz, r = 30%, dV = 10 mV | SOLVED | D = 0.275, L = 3.19e-06 H, C = 3.75e-05 F, IL_max = 5.75 A, dIL = 1.5 A |

<a id="pe_buck_component_currents"></a>

### Buck converter switch, diode, inductor and input-capacitor currents, `pe_buck_component_currents`

The average and RMS currents of the switch and the diode, the RMS inductor current, the peak current and the RMS input-capacitor current of a buck converter in continuous conduction (trapezoidal waveforms with ripple).

$$
I_{sw,rms} = \sqrt{D\left(I_o^2 + \tfrac{\Delta i_L^2}{12}\right)},\quad I_{C_{in},rms} = \sqrt{I_{sw,rms}^2 - (D I_o)^2}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `Io` | load current |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `I_sw_avg` | average switch current |
| `I_sw_rms` | RMS switch current |
| `I_d_avg` | average diode current |
| `I_d_rms` | RMS diode current |
| `I_L_rms` | RMS inductor current |
| `I_pk` | peak switch and inductor current |
| `I_cin_rms` | RMS input-capacitor ripple current |

- **Method:** I_sw,rms = sqrt(D $(Io^{2} + dI^{2}/12)$); I_d,rms = sqrt($(1 - D)$$(Io^{2} + dI^{2}/12)$); I_cin,rms = sqrt(I_sw,rms^2 - (D Io)^2)
- **Assumes:** 0 < Vout < Vin; continuous conduction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![I_sw_rms (RMS switch current) against L (inductance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-component-currents-curve)

*I_sw_rms (RMS switch current) against L (inductance) from 2.2000e-06 H to 0.00022 H on a logarithmic axis. One curve for each value of Io (load current): 3 A, 1.5 A, 6 A. Other inputs as in the worked example: Vin = 24.0 V, Vout = 5.0 V, fs = 200000.0 Hz. 37 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 2.2000e-05 H, I_sw_rms = 1.37443 A.*

```python
power_electronics_ai999x.solve('pe_buck_component_currents', Vin=24.0, Vout=5.0, Io=3.0, L=2.2e-05, fs=200000.0)
# -> D = 0.208333, I_sw_avg = 0.625 A, I_sw_rms = 1.37443 A, I_d_avg = 2.375 A, I_d_rms = 2.67925 A, I_L_rms = 3.01122 A, I_pk = 3.44981 A, I_cin_rms = 1.2241 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_component_currents', Vin=48.0, Vout=12.0, Io=10.0, L=2.2e-05, fs=100000.0)
# -> D = 0.25, I_sw_avg = 2.5 A, I_sw_rms = 5.03474 A, I_d_avg = 7.5 A, I_d_rms = 8.72043 A, I_L_rms = 10.0695 A, I_pk = 12.0455 A, I_cin_rms = 4.3702 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck converter component currents: Vin = 24 V, Vout = 5 V, Io = 3 A, L = 22 uH, fs = 200 kHz | SOLVED | D = 0.208333, I_sw_avg = 0.625 A, I_sw_rms = 1.37443 A, I_d_avg = 2.375 A, I_d_rms = 2.67925 A, I_L_rms = 3.01122 A, I_pk = 3.44981 A, I_cin_rms = 1.2241 A |
| Buck converter component currents: Vin = 48 V, Vout = 12 V, Io = 10 A, L = 22 uH, fs = 100 kHz | SOLVED | D = 0.25, I_sw_avg = 2.5 A, I_sw_rms = 5.03474 A, I_d_avg = 7.5 A, I_d_rms = 8.72043 A, I_L_rms = 10.0695 A, I_pk = 12.0455 A, I_cin_rms = 4.3702 A |
| Buck converter component currents: Vout = 5 V, Io = 3 A, L = 22 uH, fs = 200 kHz | NEEDS_INPUT | pe_buck_component_currents needs Vin (input voltage). Example: solve('pe_buck_component_currents', Vin='24 V', Vout='5 V |
| Buck converter component currents: Vin = 24 V, Vout = 5 V, Io = 3 A, L = 22 uH, fs = 200 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_interleaved_buck_ripple"></a>

### Interleaved multiphase buck: output current ripple cancellation, `pe_interleaved_buck_ripple`

The per-phase and total output current ripple of an N-phase interleaved buck converter (phases shifted by T/N, equal inductors), the cancellation factor and the ripple frequency N fs.

$$
\frac{\Delta i_{tot}}{\Delta i_{ph}} = \frac{N\left(D-\frac{m}{N}\right)\left(\frac{m+1}{N}-D\right)}{D(1-D)},\ m = \lfloor ND\rfloor
$$

| input | meaning | condition |
|---|---|---|
| `N` | number of phases |  |
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L` | inductance of each phase |  |
| `fs` | switching frequency of each phase |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `dI_phase` | ripple of one phase |
| `dI_total` | ripple of the summed output current |
| `cancellation` | ratio of total to per-phase ripple |
| `f_ripple` | output ripple frequency N fs |

- **Method:** m = floor(N D); dI_total/dI_phase = N (D - m/N)($(m + 1)/N - D)/(D$ $(1 - D)$); zero when N D is a whole number
- **Assumes:** $2  \leq  N  \leq  12$; 0 < Vout < Vin; continuous conduction in each phase
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dI_total (ripple of the summed output current) against Vout (output voltage) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-interleaved-buck-ripple-curve)

*dI_total (ripple of the summed output current) against Vout (output voltage) from 0.12 V to 12 V on a logarithmic axis. One curve for each value of L (inductance of each phase): 4.7000e-07 H, 2.3500e-07 H, 9.4000e-07 H. Other inputs as in the worked example: N = 2, Vin = 12.0 V, fs = 500000.0 Hz. 40 points per curve computed by power-electronics-ai999x; marked: the worked example, Vout = 1.2 V, dI_total = 4.08511 A.*

```python
power_electronics_ai999x.solve('pe_interleaved_buck_ripple', N=2, Vin=12.0, Vout=1.2, L=4.7e-07, fs=500000.0)
# -> D = 0.1, dI_phase = 4.59574 A, dI_total = 4.08511 A, cancellation = 0.888889, f_ripple = 1e+06 Hz   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_interleaved_buck_ripple', N=4, Vin=12.0, Vout=1.8, L=3.3e-07, fs=600000.0)
# -> D = 0.15, dI_phase = 7.72727 A, dI_total = 3.63636 A, cancellation = 0.470588, f_ripple = 2.4e+06 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Interleaved buck ripple: N = 2, Vin = 12 V, Vout = 1.2 V, L = 0.47 uH, fs = 500 kHz | SOLVED | D = 0.1, dI_phase = 4.59574 A, dI_total = 4.08511 A, cancellation = 0.888889, f_ripple = 1e+06 Hz |
| Interleaved buck ripple: N = 4, Vin = 12 V, Vout = 1.8 V, L = 0.33 uH, fs = 600 kHz | SOLVED | D = 0.15, dI_phase = 7.72727 A, dI_total = 3.63636 A, cancellation = 0.470588, f_ripple = 2.4e+06 Hz |
| Interleaved buck ripple: Vin = 12 V, Vout = 1.2 V, L = 0.47 uH, fs = 500 kHz | NEEDS_INPUT | pe_interleaved_buck_ripple needs N (number of phases). Example: solve('pe_interleaved_buck_ripple', N=2, Vin='12 V', Vou |
| Interleaved buck ripple: N = 2, Vin = 12 V, Vout = 1.2 V, L = 0.47 uH, fs = 500 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_buck_dcm"></a>

### Buck converter in discontinuous conduction, `pe_buck_dcm`

The conversion ratio, output voltage, second-interval duty D2 (diode conduction), peak inductor current and load current of an ideal buck converter in discontinuous conduction.

$$
M = \frac{2}{1+\sqrt{1+4K/D^2}},\quad K = \frac{2L}{R T_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `R` | load resistance |  |
| `D` | duty cycle (a fraction or %) |  |

| output | meaning |
|---|---|
| `K` | dimensionless conduction parameter K |
| `M` | conversion ratio Vout/Vin |
| `Vout` | output voltage |
| `D2` | diode conduction interval as a fraction of the period |
| `I_pk` | peak inductor current |
| `Io` | load current |

- **Method:** K = 2L/(R Ts); M = 2/(1 + sqrt(1 + 4K/D^2)); $D_{2} = D$ $(1 - M)/M$; I_pk = (Vin - Vo) D Ts/L
- **Assumes:** $K < 1 - D$ (discontinuous conduction)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (conversion ratio Vout/Vin) against D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-dcm-curve)

*M (conversion ratio Vout/Vin) against D (duty cycle (a fraction or %)) from 0.02 to 0.98. One curve for each value of L (inductance): 1.0000e-05 H, 5.0000e-06 H, 2.0000e-05 H. Other inputs as in the worked example: Vin = 24.0 V, fs = 100000.0 Hz, R = 20.0 ohm. 37 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.3, M = 0.6.*

```python
power_electronics_ai999x.solve('pe_buck_dcm', Vin=24.0, L=1e-05, fs=100000.0, R=20.0, D=0.3)
# -> K = 0.1, M = 0.6, Vout = 14.4 V, D2 = 0.2, I_pk = 2.88 A, Io = 0.72 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_dcm', Vin=12.0, L=4.7e-06, fs=200000.0, R=50.0, D=0.2)
# -> K = 0.0376, M = 0.628586, Vout = 7.54304 V, D2 = 0.118174, I_pk = 0.94829 A, Io = 0.150861 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck converter in DCM: Vin = 24 V, L = 10 uH, fs = 100 kHz, R = 20 ohm, D = 0.3 | SOLVED | K = 0.1, M = 0.6, Vout = 14.4 V, D2 = 0.2, I_pk = 2.88 A, Io = 0.72 A |
| Buck converter in DCM: Vin = 12 V, L = 4.7 uH, fs = 200 kHz, R = 50 ohm, D = 0.2 | SOLVED | K = 0.0376, M = 0.628586, Vout = 7.54304 V, D2 = 0.118174, I_pk = 0.94829 A, Io = 0.150861 A |
| Buck converter in DCM: L = 10 uH, fs = 100 kHz, R = 20 ohm, D = 0.3 | NEEDS_INPUT | pe_buck_dcm needs Vin (input voltage). Example: solve('pe_buck_dcm', Vin='24 V', L='1e-05 H', fs='100000 Hz', R='20 ohm' |
| Buck converter in DCM: Vin = 24 V, L = 10 uH, fs = 100 kHz, R = 20 ohm, D = 0.3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_boost_dcm"></a>

### Boost converter in discontinuous conduction, `pe_boost_dcm`

The conversion ratio, output voltage, diode conduction interval D2, peak inductor current and load current of an ideal boost converter in discontinuous conduction.

$$
M = \frac{1+\sqrt{1+4D^2/K}}{2},\quad D_2 = \frac{D}{M-1}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `R` | load resistance |  |
| `D` | duty cycle (a fraction or %) |  |

| output | meaning |
|---|---|
| `K` | dimensionless conduction parameter K |
| `M` | conversion ratio Vout/Vin |
| `Vout` | output voltage |
| `D2` | diode conduction interval as a fraction of the period |
| `I_pk` | peak inductor current |
| `Io` | load current |

- **Method:** M = (1 + sqrt($1 + 4$ D^2/K))/2; $D_{2} = D/(M - 1)$; I_pk = Vin D Ts/L
- **Assumes:** $K < D$ $(1 - D)^{2}$ (discontinuous conduction)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (conversion ratio Vout/Vin) against L (inductance) from 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-boost-dcm-curve)

*M (conversion ratio Vout/Vin) against L (inductance) from 1.0000e-06 H to 0.0001 H on a logarithmic axis. One curve for each value of fs (switching frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: Vin = 12.0 V, R = 100.0 ohm, D = 0.4. 38 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 1.0000e-05 H, M = 3.37228.*

```python
power_electronics_ai999x.solve('pe_boost_dcm', Vin=12.0, L=1e-05, fs=100000.0, R=100.0, D=0.4)
# -> K = 0.02, M = 3.37228, Vout = 40.4674 V, D2 = 0.168614, I_pk = 4.8 A, Io = 0.404674 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_boost_dcm', Vin=5.0, L=2.2e-06, fs=500000.0, R=200.0, D=0.3)
# -> K = 0.011, M = 3.40376, Vout = 17.0188 V, D2 = 0.124805, I_pk = 1.36364 A, Io = 0.085094 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Boost converter in DCM: Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 100 ohm, D = 0.4 | SOLVED | K = 0.02, M = 3.37228, Vout = 40.4674 V, D2 = 0.168614, I_pk = 4.8 A, Io = 0.404674 A |
| Boost converter in DCM: Vin = 5 V, L = 2.2 uH, fs = 500 kHz, R = 200 ohm, D = 0.3 | SOLVED | K = 0.011, M = 3.40376, Vout = 17.0188 V, D2 = 0.124805, I_pk = 1.36364 A, Io = 0.085094 A |
| Boost converter in DCM: L = 10 uH, fs = 100 kHz, R = 100 ohm, D = 0.4 | NEEDS_INPUT | pe_boost_dcm needs Vin (input voltage). Example: solve('pe_boost_dcm', Vin='12 V', L='1e-05 H', fs='100000 Hz', R='100 o |
| Boost converter in DCM: Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 100 ohm, D = 0.4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_buck_boost_dcm"></a>

### Buck-boost converter in discontinuous conduction, `pe_buck_boost_dcm`

The conversion ratio (magnitude), output voltage, diode conduction interval, peak inductor current and load current of an ideal inverting buck-boost converter in discontinuous conduction.

$$
M = \frac{D}{\sqrt{K}},\quad D_2 = \sqrt{K}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `R` | load resistance |  |
| `D` | duty cycle (a fraction or %) |  |

| output | meaning |
|---|---|
| `K` | dimensionless conduction parameter K |
| `M` | conversion ratio \|Vout\|/Vin |
| `Vout` | output voltage magnitude |
| `D2` | diode conduction interval (= sqrt K) |
| `I_pk` | peak inductor current |
| `Io` | load current |

- **Method:** $M = \frac{D}{\sqrt{K}}$; $D_{2} = \sqrt{K}$; I_pk = Vin D Ts/L
- **Assumes:** $K < (1 - D)^{2}$ (discontinuous conduction)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (conversion ratio |Vout|/Vin) against L (inductance) from 5](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-boost-dcm-curve)

*M (conversion ratio |Vout|/Vin) against L (inductance) from 5.0000e-06 H to 1.5000e-05 H. One curve for each value of fs (switching frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: Vin = 12.0 V, R = 50.0 ohm, D = 0.4. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 1.0000e-05 H, M = 2.*

```python
power_electronics_ai999x.solve('pe_buck_boost_dcm', Vin=12.0, L=1e-05, fs=100000.0, R=50.0, D=0.4)
# -> K = 0.04, M = 2, Vout = 24 V, D2 = 0.2, I_pk = 4.8 A, Io = 0.48 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_boost_dcm', Vin=24.0, L=2e-05, fs=50000.0, R=100.0, D=0.25)
# -> K = 0.02, M = 1.76777, Vout = 42.4264 V, D2 = 0.141421, I_pk = 6 A, Io = 0.424264 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck-boost converter in DCM: Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 50 ohm, D = 0.4 | SOLVED | K = 0.04, M = 2, Vout = 24 V, D2 = 0.2, I_pk = 4.8 A, Io = 0.48 A |
| Buck-boost converter in DCM: Vin = 24 V, L = 20 uH, fs = 50 kHz, R = 100 ohm, D = 0.25 | SOLVED | K = 0.02, M = 1.76777, Vout = 42.4264 V, D2 = 0.141421, I_pk = 6 A, Io = 0.424264 A |
| Buck-boost converter in DCM: L = 10 uH, fs = 100 kHz, R = 50 ohm, D = 0.4 | NEEDS_INPUT | pe_buck_boost_dcm needs Vin (input voltage). Example: solve('pe_buck_boost_dcm', Vin='12 V', L='1e-05 H', fs='100000 Hz' |
| Buck-boost converter in DCM: Vin = 12 V, L = 10 uH, fs = 100 kHz, R = 50 ohm, D = 0.4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_flyback_dcm_design"></a>

### Flyback magnetizing inductance for discontinuous conduction, `pe_flyback_dcm_design`

The magnetizing inductance at which a flyback converter delivers a power at a duty cycle in discontinuous conduction (the primary current ramps from zero every period), with the peak, RMS and average primary currents.

$$
L_m = \frac{(V_{in}D)^2}{2 P_{in} f_s},\quad I_{pk} = \frac{V_{in}D}{L_m f_s},\quad I_{rms} = I_{pk}\sqrt{D/3}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage (the minimum, for design) |  |
| `Po` | output power |  |
| `eff` | efficiency (a fraction or %) |  |
| `fs` | switching frequency |  |
| `D` | duty cycle at this input voltage (a fraction or %) |  |

| output | meaning |
|---|---|
| `Pin` | input power |
| `Lm` | magnetizing inductance |
| `I_pk` | peak primary current |
| `I_rms` | RMS primary current |
| `I_avg` | average input current |
| `E_cycle` | energy stored per cycle |

- **Method:** Pin = Po/eff = Lm Ipk^2 $fs/2$ with Ipk = Vin D/(Lm fs) -> Lm = (Vin $D)^{2}/(2$ Pin fs); I_rms = Ipk sqrt(D/3)
- **Assumes:** discontinuous conduction: the stored energy is fully released every period
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Lm (magnetizing inductance) against Po (output power) from 15 W to 45 W](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-flyback-dcm-design-curve)

*Lm (magnetizing inductance) against Po (output power) from 15 W to 45 W. One curve for each value of Vin (input voltage (the minimum, for design)): 100 V, 50 V, 200 V. Other inputs as in the worked example: eff = 0.85, fs = 100000.0 Hz, D = 0.45. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Po = 30 W, Lm = 0.000286875 H.*

```python
power_electronics_ai999x.solve('pe_flyback_dcm_design', Vin=100.0, Po=30.0, eff=0.85, fs=100000.0, D=0.45)
# -> Pin = 35.2941 W, Lm = 0.000286875 H, I_pk = 1.56863 A, I_rms = 0.607527 A, I_avg = 0.352941 A, E_cycle = 0.000352941 J   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_flyback_dcm_design', Vin=18.0, Po=10.0, eff=0.8, fs=200000.0, D=0.4)
# -> Pin = 12.5 W, Lm = 1.0368e-05 H, I_pk = 3.47222 A, I_rms = 1.26788 A, I_avg = 0.694444 A, E_cycle = 6.25e-05 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Flyback DCM design: Vin = 100 V, Po = 30 W, eff = 0.85, fs = 100 kHz, D = 0.45 | SOLVED | Pin = 35.2941 W, Lm = 0.000286875 H, I_pk = 1.56863 A, I_rms = 0.607527 A, I_avg = 0.352941 A, E_cycle = 0.000352941 J |
| Flyback DCM design: Vin = 18 V, Po = 10 W, eff = 0.8, fs = 200 kHz, D = 0.4 | SOLVED | Pin = 12.5 W, Lm = 1.0368e-05 H, I_pk = 3.47222 A, I_rms = 1.26788 A, I_avg = 0.694444 A, E_cycle = 6.25e-05 J |
| Flyback DCM design: Po = 30 W, eff = 0.85, fs = 100 kHz, D = 0.45 | NEEDS_INPUT | pe_flyback_dcm_design needs Vin (input voltage (the minimum, for design)). Example: solve('pe_flyback_dcm_design', Vin=' |
| Flyback DCM design: Vin = 100 V, Po = 30 W, eff = 0.85, fs = 100 kHz, D = 0.45, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Flyback DCM design: Vin = 100 V, Po = 30 W, eff = 85%, fs = 100 kHz, D = 45% | SOLVED | Pin = 35.2941 W, Lm = 0.000286875 H, I_pk = 1.56863 A, I_rms = 0.607527 A, I_avg = 0.352941 A, E_cycle = 0.000352941 J |

<a id="pe_boost_nonideal_gain"></a>

### Boost converter with inductor resistance: gain and efficiency, `pe_boost_nonideal_gain`

The conversion ratio, output voltage, efficiency and inductor current of a boost converter whose inductor has a series resistance, with the duty cycle of maximum gain and that maximum gain.

$$
M = \frac{1}{1-D}\,\frac{1}{1+\frac{R_L}{(1-D)^2R}},\quad \eta = \frac{1}{1+\frac{R_L}{(1-D)^2R}}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `D` | duty cycle (a fraction or %) |  |
| `R` | load resistance |  |
| `RL` | inductor series resistance |  |

| output | meaning |
|---|---|
| `M` | conversion ratio |
| `Vout` | output voltage |
| `efficiency` | efficiency |
| `IL` | average inductor current |
| `D_peak` | duty cycle of maximum gain |
| `M_max` | maximum gain |

- **Method:** M = (1/(1 - D)) / (1 + RL/($(1 - D)^{2}$ R)); efficiency = 1/(1 + RL/($(1 - D)^{2}$ R)); $D_{peak} = 1 - \sqrt{\frac{RL}{R}}$; $M_{max} = \frac{\sqrt{\frac{R}{RL}}}{2}$
- **Assumes:** $0 < RL < R$; continuous conduction, ideal switches
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (conversion ratio) against D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-boost-nonideal-gain-curve)

*M (conversion ratio) against D (duty cycle (a fraction or %)) from 0.02 to 0.98. One curve for each value of R (load resistance): 50 ohm, 25 ohm, 100 ohm. Other inputs as in the worked example: Vin = 12.0 V, RL = 0.5 ohm. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.75, M = 3.44828.*

```python
power_electronics_ai999x.solve('pe_boost_nonideal_gain', Vin=12.0, D=0.75, R=50.0, RL=0.5)
# -> M = 3.44828, Vout = 41.3793 V, efficiency = 0.862069, IL = 3.31034 A, D_peak = 0.9, M_max = 5   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_boost_nonideal_gain', Vin=24.0, D=0.5, R=20.0, RL=0.1)
# -> M = 1.96078, Vout = 47.0588 V, efficiency = 0.980392, IL = 4.70588 A, D_peak = 0.929289, M_max = 7.07107   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Non-ideal boost converter: Vin = 12 V, D = 0.75, R = 50 ohm, RL = 0.5 ohm | SOLVED | M = 3.44828, Vout = 41.3793 V, efficiency = 0.862069, IL = 3.31034 A, D_peak = 0.9, M_max = 5 |
| Non-ideal boost converter: Vin = 24 V, D = 0.5, R = 20 ohm, RL = 0.1 ohm | SOLVED | M = 1.96078, Vout = 47.0588 V, efficiency = 0.980392, IL = 4.70588 A, D_peak = 0.929289, M_max = 7.07107 |
| Non-ideal boost converter: D = 0.75, R = 50 ohm, RL = 0.5 ohm | NEEDS_INPUT | pe_boost_nonideal_gain needs Vin (input voltage). Example: solve('pe_boost_nonideal_gain', Vin='12 V', D=0.75, R='50 ohm |
| Non-ideal boost converter: Vin = 12 V, D = 0.75, R = 50 ohm, RL = 0.5 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_buck_with_losses"></a>

### Buck converter efficiency with conduction and switching losses, `pe_buck_with_losses`

The duty cycle needed and the losses of a buck converter with a switch on-resistance, inductor resistance, diode forward drop and hard switching: conduction, diode, inductor and switching losses and the efficiency (ripple neglected).

$$
D = \frac{V_o+V_F+I_oR_L}{V_{in}-I_oR_{ds}+V_F},\quad P_{sw} = \tfrac12 V_{in}I_o t_{sw} f_s,\quad \eta = \frac{P_o}{P_o+\sum P}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `Io` | load current |  |
| `fs` | switching frequency |  |
| `Rds` | switch on-resistance |  |
| `RL` | inductor resistance |  |
| `VF` | diode forward voltage |  |
| `tsw` | total switching (crossover) time per period, turn-on plus turn-off |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `P_switch_cond` | switch conduction loss |
| `P_diode` | diode conduction loss |
| `P_inductor` | inductor copper loss |
| `P_switching` | switching loss |
| `P_loss` | total loss |
| `efficiency` | efficiency |

- **Method:** D = (Vo + VF + Io RL)/(Vin - Io Rds + VF); P_sw,cond = Io^2 Rds D; $P_D = VF$ Io $(1 - D)$; $P_L = Io^{2}$ RL; P_sw = Vin Io tsw $fs/2$
- **Assumes:** continuous conduction, ripple neglected; hard switching with linear crossover at Vin and Io
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![efficiency (efficiency) against fs (switching frequency) from 20000 Hz to 2e+06 Hz on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-buck-with-losses-curve)

*efficiency (efficiency) against fs (switching frequency) from 20000 Hz to 2e+06 Hz on a logarithmic axis. One curve for each value of Vout (output voltage): 5 V, 2.5 V, 10 V. Other inputs as in the worked example: Vin = 24.0 V, Io = 4.0 A, Rds = 0.02 ohm, RL = 0.01 ohm, VF = 0.5 V, tsw = 4e-08 s. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, fs = 200000 Hz, efficiency = 0.90241.*

```python
power_electronics_ai999x.solve('pe_buck_with_losses', Vin=24.0, Vout=5.0, Io=4.0, fs=200000.0, Rds=0.02, RL=0.01, VF=0.5, tsw=4e-08)
# -> D = 0.226863, P_switch_cond = 0.0725962 W, P_diode = 1.54627 W, P_inductor = 0.16 W, P_switching = 0.384 W, P_loss = 2.16287 W, efficiency = 0.90241   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_buck_with_losses', Vin=48.0, Vout=12.0, Io=10.0, fs=100000.0, Rds=0.01, RL=0.005, VF=0.7, tsw=6e-08)
# -> D = 0.262346, P_switch_cond = 0.262346 W, P_diode = 5.16358 W, P_inductor = 0.5 W, P_switching = 1.44 W, P_loss = 7.36593 W, efficiency = 0.942167   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Buck converter losses: Vin = 24 V, Vout = 5 V, Io = 4 A, fs = 200 kHz, Rds = 0.02 ohm, RL = 0.01 ohm, VF = 0.5 V, tsw = 40 ns | SOLVED | D = 0.226863, P_switch_cond = 0.0725962 W, P_diode = 1.54627 W, P_inductor = 0.16 W, P_switching = 0.384 W, P_loss = 2.16287 W, efficiency = 0.90241 |
| Buck converter losses: Vin = 48 V, Vout = 12 V, Io = 10 A, fs = 100 kHz, Rds = 0.01 ohm, RL = 0.005 ohm, VF = 0.7 V, tsw = 60 ns | SOLVED | D = 0.262346, P_switch_cond = 0.262346 W, P_diode = 5.16358 W, P_inductor = 0.5 W, P_switching = 1.44 W, P_loss = 7.36593 W, efficiency = 0.942167 |
| Buck converter losses: Vout = 5 V, Io = 4 A, fs = 200 kHz, Rds = 0.02 ohm, RL = 0.01 ohm, VF = 0.5 V, tsw = 40 ns | NEEDS_INPUT | pe_buck_with_losses needs Vin (input voltage). Example: solve('pe_buck_with_losses', Vin='24 V', Vout='5 V', Io='4 A', f |
| Buck converter losses: Vin = 24 V, Vout = 5 V, Io = 4 A, fs = 200 kHz, Rds = 0.02 ohm, RL = 0.01 ohm, VF = 0.5 V, tsw = 40 ns, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_converter_small_signal"></a>

### Control-to-output transfer function of a DC-DC converter, `pe_converter_small_signal`

The control-to-output (duty-to-voltage) transfer function features of an ideal buck, boost or buck-boost converter in continuous conduction: dc gain, double-pole frequency, quality factor and right-half-plane zero.

$$
G_{vd}(s) = G_{d0}\frac{1 - s/\omega_z}{1 + s/(Q\omega_0) + (s/\omega_0)^2},\quad \omega_{z,boost} = \frac{D'^2R}{L}
$$

| input | meaning | condition |
|---|---|---|
| `topology` | buck, boost or buck-boost |  |
| `Vin` | input voltage |  |
| `D` | duty cycle (a fraction or %) |  |
| `L` | inductance |  |
| `C` | output capacitance |  |
| `R` | load resistance |  |

| output | meaning |
|---|---|
| `Gvd0` | dc gain of vo/d (volts per unit duty) |
| `f0` | double-pole (corner) frequency |
| `Q` | quality factor of the double pole |
| `f_rhpz` | right-half-plane zero frequency |
| `rhp_zero` | whether a right-half-plane zero is present |

- **Method:** buck: Gvd0 = Vin, $w_{0} = \frac{1}{\sqrt{LC}}$, $Q = R$ sqrt(C/L); boost: Gvd0 = Vin/D'^2, $w_{0} = D$'/sqrt(LC), $Q = D$' R sqrt(C/L), $wz = D$'^2 R/L; buck-boost: $wz = D$'^2 R/(D L)
- **Assumes:** ideal converter in continuous conduction, no ESR
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![f0 (double-pole (corner) frequency) against L (inductance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-converter-small-signal-curve)

*f0 (double-pole (corner) frequency) against L (inductance) from 2.5000e-05 H to 7.5000e-05 H. One curve for each value of C (output capacitance): 1.0000e-04 F, 5.0000e-05 F, 0.0002 F. Other inputs as in the worked example: topology = boost, Vin = 12.0 V, D = 0.5, R = 10.0 ohm. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 5.0000e-05 H, f0 = 1125.4 Hz.*

```python
power_electronics_ai999x.solve('pe_converter_small_signal', topology='boost', Vin=12.0, D=0.5, L=5e-05, C=0.0001, R=10.0)
# -> Gvd0 = 48 V, f0 = 1125.4 Hz, Q = 7.07107, f_rhpz = 7957.75 Hz, rhp_zero = present   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_converter_small_signal', topology='buck', Vin=24.0, D=0.4, L=2.2e-05, C=0.00022, R=2.0)
# -> Gvd0 = 24 V, f0 = 2287.69 Hz, Q = 6.32456, f_rhpz = 0 Hz, rhp_zero = none   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Converter small-signal model: topology = boost, Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm | SOLVED | Gvd0 = 48 V, f0 = 1125.4 Hz, Q = 7.07107, f_rhpz = 7957.75 Hz, rhp_zero = present |
| Converter small-signal model: topology = buck, Vin = 24 V, D = 0.4, L = 22 uH, C = 220 uF, R = 2 ohm | SOLVED | Gvd0 = 24 V, f0 = 2287.69 Hz, Q = 6.32456, f_rhpz = 0 Hz, rhp_zero = none |
| Converter small-signal model: Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm | NEEDS_INPUT | pe_converter_small_signal needs topology (buck, boost or buck-boost). Example: solve('pe_converter_small_signal', topolo |
| Converter small-signal model: topology = boost, Vin = 12 V, D = 0.5, L = 50 uH, C = 100 uF, R = 10 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_peak_current_mode_slope"></a>

### Peak current mode: subharmonic stability and slope compensation, `pe_peak_current_mode_slope`

The inductor current slopes, the period-to-period perturbation factor, the stability verdict and the smallest and the deadbeat compensation ramp slopes of a peak-current-mode buck converter.

$$
\alpha = -\frac{m_2 - m_a}{m_1 + m_a},\quad |\alpha| < 1 \Leftarrow m_a > \frac{m_2-m_1}{2}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | input voltage |  |
| `Vout` | output voltage |  |
| `L` | inductance |  |
| `fs` | switching frequency |  |
| `ma` | compensation ramp slope referred to the inductor current |  |

| output | meaning |
|---|---|
| `D` | duty cycle |
| `m1` | rising inductor current slope |
| `m2` | falling inductor current slope (magnitude) |
| `alpha` | perturbation factor per switching period |
| `stable` | stable or unstable |
| `ma_min` | smallest ramp slope for stability at this duty cycle |
| `ma_deadbeat` | deadbeat compensation ramp slope |

- **Method:** m1 = (Vin - Vo)/L, $m_{2} = Vo/L$; alpha = -$(m_{2} - ma)/(m_{1} + ma)$; stable when |alpha| < 1; ma,min = (m2 - m1)/2
- **Assumes:** 0 < Vout < Vin; continuous conduction
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![alpha (perturbation factor per switching period) against Vin (input voltage), with L = 1](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-peak-current-mode-slope-alpha-vs-vin.svg)

*alpha (perturbation factor per switching period) against Vin (input voltage), with L = 1.00e-5 H, Vout = 8 V, fs = 3.00e5 Hz, ma = 3.00e5 A/s. 25 points computed by power-electronics-ai999x; marked: Vin = 12, alpha = -0.714286.*

![stable or unstable (stable) over Vin (input voltage) from 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-peak-current-mode-slope-decision)

*stable or unstable (stable) over Vin (input voltage) from 1.2 V to 120 V and Vout (output voltage) from 0.8 V to 80 V (logarithmic axes): each of the 31 × 21 cells is one run of power-electronics-ai999x, coloured by the verdict it returned, stable: 341 cells, unstable: 39 cells; blank cells: inputs the capability refuses. Other inputs as in the worked example: L = 9.999999999999999e-06 H, fs = 300000.0 Hz, ma = 300000.0 A/s. Marked: the worked example, Vin = 12 V, Vout = 8 V → stable.*

![alpha (perturbation factor per switching period) against Vout (output voltage) from 4 V to 12 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-peak-current-mode-slope-curve)

*alpha (perturbation factor per switching period) against Vout (output voltage) from 4 V to 12 V. One curve for each value of L (inductance): 1.0000e-05 H, 5.0000e-06 H, 2.0000e-05 H. Other inputs as in the worked example: Vin = 12.0 V, fs = 300000.0 Hz, ma = 300000.0 A/s. 30 points per curve computed by power-electronics-ai999x; marked: the worked example, Vout = 8 V, alpha = -0.714286.*

```python
power_electronics_ai999x.solve('pe_peak_current_mode_slope', Vin=12.0, Vout=8.0, L=1e-05, fs=300000.0, ma=300000.0)
# -> D = 0.666667, m1 = 400000 A/s, m2 = 800000 A/s, alpha = -0.714286, stable = stable, ma_min = 200000 A/s, ma_deadbeat = 800000 A/s   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_peak_current_mode_slope', Vin=24.0, Vout=5.0, L=2.2e-05, fs=200000.0, ma=100000.0)
# -> D = 0.208333, m1 = 863636 A/s, m2 = 227273 A/s, alpha = -0.132075, stable = stable, ma_min = 0 A/s, ma_deadbeat = 227273 A/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Peak current mode slope compensation: Vin = 12 V, Vout = 8 V, L = 10 uH, fs = 300 kHz, ma = 0.3 A/us | SOLVED | D = 0.666667, m1 = 400000 A/s, m2 = 800000 A/s, alpha = -0.714286, stable = stable, ma_min = 200000 A/s, ma_deadbeat = 800000 A/s |
| Peak current mode slope compensation: Vin = 24 V, Vout = 5 V, L = 22 uH, fs = 200 kHz, ma = 0.1 A/us | SOLVED | D = 0.208333, m1 = 863636 A/s, m2 = 227273 A/s, alpha = -0.132075, stable = stable, ma_min = 0 A/s, ma_deadbeat = 227273 A/s |
| Peak current mode slope compensation: Vout = 8 V, L = 10 uH, fs = 300 kHz, ma = 0.3 A/us | NEEDS_INPUT | pe_peak_current_mode_slope needs Vin (input voltage). Example: solve('pe_peak_current_mode_slope', Vin='12 V', Vout='8 V |
| Peak current mode slope compensation: Vin = 12 V, Vout = 8 V, L = 10 uH, fs = 300 kHz, ma = 0.3 A/us, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_three_phase_diode_bridge"></a>

### Three-phase diode bridge rectifier with resistive load, `pe_three_phase_diode_bridge`

The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage, and the load, diode and line currents of a six-pulse three-phase diode bridge feeding a resistor.

$$
V_{dc} = \frac{3\sqrt2\,V_{LL}}{\pi},\quad V_{rms} = \sqrt2 V_{LL}\sqrt{\tfrac12+\tfrac{3\sqrt3}{4\pi}}
$$

| input | meaning | condition |
|---|---|---|
| `VLL` | RMS line-to-line source voltage |  |
| `f` | line frequency |  |
| `R` | load resistance |  |

| output | meaning |
|---|---|
| `Vdc` | average output voltage |
| `Vrms` | RMS output voltage |
| `ripple_factor` | ripple factor |
| `form_factor` | form factor |
| `f_ripple` | ripple frequency |
| `PIV` | diode peak inverse voltage |
| `Idc` | average load current |
| `I_diode_avg` | average diode current |
| `I_diode_rms` | RMS diode current |
| `Is_rms` | RMS line current |

- **Method:** $Vm = sqrt$(2) VLL; Vdc = 3 Vm/pi; Vrms = Vm sqrt($1/2 + 3$ sqrt($3)/(4$ pi)); diode avg = Idc/3
- **Assumes:** ideal diodes, no source inductance, resistive load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vdc (average output voltage) against VLL (RMS line-to-line source voltage) from 200 V to 600 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-three-phase-diode-bridge-curve)

*Vdc (average output voltage) against VLL (RMS line-to-line source voltage) from 200 V to 600 V. Outputs with the same unit drawn together: Vrms (RMS output voltage), PIV (diode peak inverse voltage). Other inputs as in the worked example: f = 50.0 Hz, R = 50.0 ohm. 31 points computed by power-electronics-ai999x; marked: the worked example, VLL = 400 V, Vdc = 540.19 V.*

```python
power_electronics_ai999x.solve('pe_three_phase_diode_bridge', VLL=400.0, f=50.0, R=50.0)
# -> Vdc = 540.19 V, Vrms = 540.665 V, ripple_factor = 0.0419666, form_factor = 1.00088, f_ripple = 300 Hz, PIV = 565.685 V, Idc = 10.8038 A, I_diode_avg = 3.60127 A, I_diode_rms = 6.24306 A, Is_rms = 8.82903 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_three_phase_diode_bridge', VLL=480.0, f=60.0, R=20.0)
# -> Vdc = 648.228 V, Vrms = 648.798 V, ripple_factor = 0.0419666, form_factor = 1.00088, f_ripple = 360 Hz, PIV = 678.823 V, Idc = 32.4114 A, I_diode_avg = 10.8038 A, I_diode_rms = 18.7292 A, Is_rms = 26.4871 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Three-phase diode bridge: VLL = 400 V, f = 0.05 kHz, R = 50 ohm | SOLVED | Vdc = 540.19 V, Vrms = 540.665 V, ripple_factor = 0.0419666, form_factor = 1.00088, f_ripple = 300 Hz, PIV = 565.685 V, Idc = 10.8038 A, I_diode_avg = 3.60127 A, I_diode_rms = 6.24306 A, Is_rms = 8.82903 A |
| Three-phase diode bridge: VLL = 480 V, f = 0.06 kHz, R = 20 ohm | SOLVED | Vdc = 648.228 V, Vrms = 648.798 V, ripple_factor = 0.0419666, form_factor = 1.00088, f_ripple = 360 Hz, PIV = 678.823 V, Idc = 32.4114 A, I_diode_avg = 10.8038 A, I_diode_rms = 18.7292 A, Is_rms = 26.4871 A |
| Three-phase diode bridge: f = 0.05 kHz, R = 50 ohm | NEEDS_INPUT | pe_three_phase_diode_bridge needs VLL (RMS line-to-line source voltage). Example: solve('pe_three_phase_diode_bridge', V |
| Three-phase diode bridge: VLL = 400 V, f = 0.05 kHz, R = 50 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_three_phase_half_wave_rectifier"></a>

### Three-phase half-wave (three-pulse) diode rectifier, `pe_three_phase_half_wave_rectifier`

The average and RMS output voltage, ripple and form factors, ripple frequency, diode peak inverse voltage and the load and diode currents of a three-pulse (star, half-wave) diode rectifier feeding a resistor.

$$
V_{dc} = \frac{3\sqrt3\,V_m}{2\pi},\quad V_{rms} = V_m\sqrt{\tfrac12+\tfrac{3\sqrt3}{8\pi}},\quad PIV = \sqrt3 V_m
$$

| input | meaning | condition |
|---|---|---|
| `Vph` | RMS phase (line-to-neutral) voltage |  |
| `f` | line frequency |  |
| `R` | load resistance |  |

| output | meaning |
|---|---|
| `Vdc` | average output voltage |
| `Vrms` | RMS output voltage |
| `ripple_factor` | ripple factor |
| `form_factor` | form factor |
| `f_ripple` | ripple frequency |
| `PIV` | diode peak inverse voltage |
| `Idc` | average load current |
| `I_diode_avg` | average diode current |
| `I_diode_rms` | RMS diode current |

- **Method:** $Vm = sqrt$(2) Vph; Vdc = 3 sqrt(3) Vm/(2 pi); Vrms = Vm sqrt($1/2 + 3$ sqrt($3)/(8$ pi)); PIV = sqrt(3) Vm
- **Assumes:** ideal diodes, resistive load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vdc (average output voltage) against Vph (RMS phase (line-to-neutral) voltage) from 115 V to 345 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-three-phase-half-wave-rectifier-curve)

*Vdc (average output voltage) against Vph (RMS phase (line-to-neutral) voltage) from 115 V to 345 V. Outputs with the same unit drawn together: Vrms (RMS output voltage), PIV (diode peak inverse voltage). Other inputs as in the worked example: f = 50.0 Hz, R = 20.0 ohm. 31 points computed by power-electronics-ai999x; marked: the worked example, Vph = 230 V, Vdc = 268.995 V.*

```python
power_electronics_ai999x.solve('pe_three_phase_half_wave_rectifier', Vph=230.0, f=50.0, R=20.0)
# -> Vdc = 268.995 V, Vrms = 273.448 V, ripple_factor = 0.182707, form_factor = 1.01655, f_ripple = 150 Hz, PIV = 563.383 V, Idc = 13.4498 A, I_diode_avg = 4.48326 A, I_diode_rms = 7.89377 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_three_phase_half_wave_rectifier', Vph=120.0, f=60.0, R=10.0)
# -> Vdc = 140.345 V, Vrms = 142.669 V, ripple_factor = 0.182707, form_factor = 1.01655, f_ripple = 180 Hz, PIV = 293.939 V, Idc = 14.0345 A, I_diode_avg = 4.67818 A, I_diode_rms = 8.23698 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Three-phase half-wave rectifier: Vph = 230 V, f = 0.05 kHz, R = 20 ohm | SOLVED | Vdc = 268.995 V, Vrms = 273.448 V, ripple_factor = 0.182707, form_factor = 1.01655, f_ripple = 150 Hz, PIV = 563.383 V, Idc = 13.4498 A, I_diode_avg = 4.48326 A, I_diode_rms = 7.89377 A |
| Three-phase half-wave rectifier: Vph = 120 V, f = 0.06 kHz, R = 10 ohm | SOLVED | Vdc = 140.345 V, Vrms = 142.669 V, ripple_factor = 0.182707, form_factor = 1.01655, f_ripple = 180 Hz, PIV = 293.939 V, Idc = 14.0345 A, I_diode_avg = 4.67818 A, I_diode_rms = 8.23698 A |
| Three-phase half-wave rectifier: f = 0.05 kHz, R = 20 ohm | NEEDS_INPUT | pe_three_phase_half_wave_rectifier needs Vph (RMS phase (line-to-neutral) voltage). Example: solve('pe_three_phase_half_ |
| Three-phase half-wave rectifier: Vph = 230 V, f = 0.05 kHz, R = 20 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_single_phase_controlled_rectifier"></a>

### Single-phase phase-controlled thyristor rectifier, `pe_single_phase_controlled_rectifier`

The average, RMS and AC (ripple) output voltage of a single-phase thyristor rectifier at a firing angle: half-wave with a resistive load, fully controlled bridge with continuous load current, or semi-converter (half-controlled) with continuous current.

$$
V_{dc,HW} = \frac{V_m(1+\cos\alpha)}{2\pi},\ V_{dc,FB} = \frac{2V_m\cos\alpha}{\pi},\ V_{dc,semi} = \frac{V_m(1+\cos\alpha)}{\pi}
$$

| input | meaning | condition |
|---|---|---|
| `circuit` | half-wave, full-bridge or semi-converter |  |
| `Vs` | RMS source voltage |  |
| `alpha` | firing angle (delay from the natural commutation point) |  |

| output | meaning |
|---|---|
| `Vdc` | average output voltage |
| `Vrms` | RMS output voltage |
| `Vac_rms` | RMS of the AC component |
| `mode` | rectifying or inverting (negative average voltage) |

- **Method:** half-wave: Vdc = Vm (1 + cos $a)/(2$ pi); full bridge: Vdc = 2 Vm cos a/pi; semi-converter: Vdc = Vm (1 + cos a)/pi
- **Assumes:** $0  \leq  \alpha < 180$ degrees; ideal thyristors, no source inductance
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vdc (average output voltage) against alpha (firing angle (delay from the natural commutation point)) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-single-phase-controlled-rectifier-curve)

*Vdc (average output voltage) against alpha (firing angle (delay from the natural commutation point)) from 0 rad to 1.5708 rad. One curve for each value of Vs (RMS source voltage): 230 V, 115 V, 460 V. Other inputs as in the worked example: circuit = full-bridge. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0.523599 rad, Vdc = 179.33 V.*

![Vdc (average output voltage) over Vs (RMS source voltage) from 115 V to 345 V and alpha (firing angle (delay from the natural commutation point)) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-single-phase-controlled-rectifier-field)

*Vdc (average output voltage) over Vs (RMS source voltage) from 115 V to 345 V and alpha (firing angle (delay from the natural commutation point)) from 0 rad to 1.5708 rad: each of the 21 × 15 cells is one run of power-electronics-ai999x, shaded by its value (light 5.0698e-10 V to dark 310.609 V); iso-lines at Vdc = 100, 200, 300 V. Other inputs as in the worked example: circuit = full-bridge. Marked: the worked example, Vs = 230 V, alpha = 0.523599 rad, Vdc = 179.33 V.*

```python
power_electronics_ai999x.solve('pe_single_phase_controlled_rectifier', circuit='full-bridge', Vs=230.0, alpha=0.5235987755982988)
# -> Vdc = 179.33 V, Vrms = 230 V, Vac_rms = 144.016 V, mode = rectifying   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_single_phase_controlled_rectifier', circuit='half-wave', Vs=120.0, alpha=1.0471975511965976)
# -> Vdc = 40.5142 V, Vrms = 76.1078 V, Vac_rms = 64.4282 V, mode = rectifying   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Single-phase controlled rectifier: circuit = full-bridge, Vs = 230 V, alpha = 30 deg | SOLVED | Vdc = 179.33 V, Vrms = 230 V, Vac_rms = 144.016 V, mode = rectifying |
| Single-phase controlled rectifier: circuit = half-wave, Vs = 120 V, alpha = 60 deg | SOLVED | Vdc = 40.5142 V, Vrms = 76.1078 V, Vac_rms = 64.4282 V, mode = rectifying |
| Single-phase controlled rectifier: Vs = 230 V, alpha = 30 deg | NEEDS_INPUT | pe_single_phase_controlled_rectifier needs circuit (half-wave, full-bridge or semi-converter). Example: solve('pe_single |
| Single-phase controlled rectifier: circuit = full-bridge, Vs = 230 V, alpha = 30 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_three_phase_controlled_rectifier"></a>

### Three-phase fully controlled thyristor bridge, `pe_three_phase_controlled_rectifier`

The average and RMS output voltage, the power, and the thyristor and line currents of a six-pulse fully controlled thyristor bridge with continuous (constant) load current at a firing angle, rectifying or inverting.

$$
V_{dc} = \frac{3\sqrt2 V_{LL}}{\pi}\cos\alpha,\quad I_{s,rms} = \sqrt{\tfrac23}\,I_d
$$

| input | meaning | condition |
|---|---|---|
| `VLL` | RMS line-to-line source voltage |  |
| `alpha` | firing angle |  |
| `Id` | constant dc output current |  |

| output | meaning |
|---|---|
| `Vdc` | average output voltage |
| `Vrms` | RMS output voltage |
| `P` | dc output power Vdc Id |
| `I_thy_avg` | average thyristor current |
| `I_thy_rms` | RMS thyristor current |
| `Is_rms` | RMS line current |
| `mode` | rectifying or inverting |

- **Method:** Vdc = 3 sqrt(2) VLL cos(alpha)/pi; Vrms = Vm sqrt($1/2 + 3$ sqrt(3) cos(2 $\alpha)/(4$ pi)); I_thy = Id/3 (avg), Id/sqrt(3) (rms)
- **Assumes:** $0  \leq  \alpha < 180$ degrees; continuous constant current, no source inductance
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vdc (average output voltage) against alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-three-phase-controlled-rectifier-curve)

*Vdc (average output voltage) against alpha (firing angle) from 0 rad to 1.5708 rad. One curve for each value of VLL (RMS line-to-line source voltage): 415 V, 207.5 V, 830 V. Other inputs as in the worked example: Id = 50.0 A. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0.785398 rad, Vdc = 396.296 V.*

```python
power_electronics_ai999x.solve('pe_three_phase_controlled_rectifier', VLL=415.0, alpha=0.7853981633974483, Id=50.0)
# -> Vdc = 396.296 V, Vrms = 415 V, P = 19814.8 W, I_thy_avg = 16.6667 A, I_thy_rms = 28.8675 A, Is_rms = 40.8248 A, mode = rectifying   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_three_phase_controlled_rectifier', VLL=400.0, alpha=2.0943951023931953, Id=20.0)
# -> Vdc = -270.095 V, Vrms = 306.334 V, P = -5401.9 W, I_thy_avg = 6.66667 A, I_thy_rms = 11.547 A, Is_rms = 16.3299 A, mode = inverting   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Three-phase controlled rectifier: VLL = 415 V, alpha = 45 deg, Id = 50 A | SOLVED | Vdc = 396.296 V, Vrms = 415 V, P = 19814.8 W, I_thy_avg = 16.6667 A, I_thy_rms = 28.8675 A, Is_rms = 40.8248 A, mode = rectifying |
| Three-phase controlled rectifier: VLL = 400 V, alpha = 120 deg, Id = 20 A | SOLVED | Vdc = -270.095 V, Vrms = 306.334 V, P = -5401.9 W, I_thy_avg = 6.66667 A, I_thy_rms = 11.547 A, Is_rms = 16.3299 A, mode = inverting |
| Three-phase controlled rectifier: alpha = 45 deg, Id = 50 A | NEEDS_INPUT | pe_three_phase_controlled_rectifier needs VLL (RMS line-to-line source voltage). Example: solve('pe_three_phase_controll |
| Three-phase controlled rectifier: VLL = 415 V, alpha = 45 deg, Id = 50 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_ac_voltage_controller"></a>

### Single-phase AC voltage controller (resistive load), `pe_ac_voltage_controller`

The RMS output voltage and current, load power, input power factor and the average and RMS thyristor currents of a single-phase AC voltage controller (antiparallel thyristors or a triac) with a resistive load at a firing angle.

$$
V_{o} = V_s\sqrt{1-\frac{\alpha}{\pi}+\frac{\sin2\alpha}{2\pi}},\quad PF = \frac{V_o}{V_s}
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | RMS source voltage |  |
| `alpha` | firing angle |  |
| `R` | load resistance |  |

| output | meaning |
|---|---|
| `Vo_rms` | RMS output voltage |
| `Io_rms` | RMS load current |
| `P` | load power |
| `PF` | input power factor |
| `I_scr_avg` | average thyristor current |
| `I_scr_rms` | RMS thyristor current |

- **Method:** $Vo = Vs$ sqrt($1 - \alpha/pi + sin$(2 $\alpha)/(2$ pi)); $PF = Vo/Vs$; I_SCR,avg = sqrt(2) Vs (1 + cos $\alpha)/(2$ pi R)
- **Assumes:** $0  \leq  \alpha < 180$ degrees; resistive load
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vo_rms (RMS output voltage) against alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-ac-voltage-controller-curve)

*Vo_rms (RMS output voltage) against alpha (firing angle) from 0 rad to 1.5708 rad. One curve for each value of Vs (RMS source voltage): 230 V, 115 V, 460 V. Other inputs as in the worked example: R = 10.0 ohm. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 1.0472 rad, Vo_rms = 206.296 V.*

```python
power_electronics_ai999x.solve('pe_ac_voltage_controller', Vs=230.0, alpha=1.0471975511965976, R=10.0)
# -> Vo_rms = 206.296 V, Io_rms = 20.6296 A, P = 4255.8 W, PF = 0.896939, I_scr_avg = 7.76523 A, I_scr_rms = 14.5873 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_ac_voltage_controller', Vs=120.0, alpha=1.5707963267948966, R=20.0)
# -> Vo_rms = 84.8528 V, Io_rms = 4.24264 A, P = 360 W, PF = 0.707107, I_scr_avg = 1.35047 A, I_scr_rms = 3 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| AC voltage controller: Vs = 230 V, alpha = 60 deg, R = 10 ohm | SOLVED | Vo_rms = 206.296 V, Io_rms = 20.6296 A, P = 4255.8 W, PF = 0.896939, I_scr_avg = 7.76523 A, I_scr_rms = 14.5873 A |
| AC voltage controller: Vs = 120 V, alpha = 90 deg, R = 20 ohm | SOLVED | Vo_rms = 84.8528 V, Io_rms = 4.24264 A, P = 360 W, PF = 0.707107, I_scr_avg = 1.35047 A, I_scr_rms = 3 A |
| AC voltage controller: alpha = 60 deg, R = 10 ohm | NEEDS_INPUT | pe_ac_voltage_controller needs Vs (RMS source voltage). Example: solve('pe_ac_voltage_controller', Vs='230 V', alpha='1. |
| AC voltage controller: Vs = 230 V, alpha = 60 deg, R = 10 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_rectifier_line_current_pf"></a>

### Rectifier line current: THD, displacement and power factor, `pe_rectifier_line_current_pf`

The RMS and fundamental line current, current THD, displacement power factor, power factor, real power and apparent power of a single-phase or three-phase (six-pulse) bridge rectifier with a constant dc current at a firing angle (0 for diodes).

$$
PF = \frac{I_{s1}}{I_s}\cos\alpha,\quad \frac{I_{s1}}{I_s} = \frac{2\sqrt2}{\pi}\ (1\phi),\ \frac{3}{\pi}\ (3\phi)
$$

| input | meaning | condition |
|---|---|---|
| `circuit` | single-phase or three-phase bridge |  |
| `Id` | constant dc output current |  |
| `Vs` | RMS source voltage (line-to-line for three-phase) |  |
| `alpha` | firing angle |  |

| output | meaning |
|---|---|
| `Is_rms` | RMS line current |
| `Is1_rms` | RMS fundamental line current |
| `THD` | current THD |
| `DPF` | displacement power factor cos(alpha) |
| `PF` | power factor |
| `P` | real power |
| `S` | apparent power |

- **Method:** single-phase: $Is = Id$, $Is1 = 2$ sqrt(2) Id/pi (PF = 0.9 cos a); three-phase: $Is = sqrt$(2/3) Id, $Is1 = sqrt$(6) Id/pi (PF = 0.955 cos a)
- **Assumes:** constant (ripple-free) dc current, no commutation overlap
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PF (power factor) against alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-rectifier-line-current-pf-curve)

*PF (power factor) against alpha (firing angle) from 0 rad to 1.5708 rad. One curve for each value of circuit (single-phase or three-phase bridge): single-phase, three-phase. Other inputs as in the worked example: Id = 10.0 A, Vs = 230.0 V. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0 rad, PF = 0.900316.*

```python
power_electronics_ai999x.solve('pe_rectifier_line_current_pf', circuit='single-phase', Id=10.0, Vs=230.0, alpha=0.0)
# -> Is_rms = 10 A, Is1_rms = 9.00316 A, THD = 0.483426, DPF = 1, PF = 0.900316, P = 2070.73 W, S = 2300 VA   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_rectifier_line_current_pf', circuit='three-phase', Id=100.0, Vs=400.0, alpha=0.5235987755982988)
# -> Is_rms = 81.6497 A, Is1_rms = 77.9697 A, THD = 0.310842, DPF = 0.866025, PF = 0.826993, P = 46781.8 W, S = 56568.5 VA   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rectifier line current power factor: circuit = single-phase, Id = 10 A, Vs = 230 V, alpha = 0 deg | SOLVED | Is_rms = 10 A, Is1_rms = 9.00316 A, THD = 0.483426, DPF = 1, PF = 0.900316, P = 2070.73 W, S = 2300 VA |
| Rectifier line current power factor: circuit = three-phase, Id = 100 A, Vs = 400 V, alpha = 30 deg | SOLVED | Is_rms = 81.6497 A, Is1_rms = 77.9697 A, THD = 0.310842, DPF = 0.866025, PF = 0.826993, P = 46781.8 W, S = 56568.5 VA |
| Rectifier line current power factor: Id = 10 A, Vs = 230 V, alpha = 0 deg | NEEDS_INPUT | pe_rectifier_line_current_pf needs circuit (single-phase or three-phase bridge). Example: solve('pe_rectifier_line_curre |
| Rectifier line current power factor: circuit = single-phase, Id = 10 A, Vs = 230 V, alpha = 0 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_rectifier_line_harmonics"></a>

### Rectifier line-current harmonic spectrum: `pe_rectifier_line_harmonics`

The RMS value of harmonic h of the line current of a single-phase or six-pulse three-phase bridge with a constant dc current, its ratio to the fundamental (1/h for characteristic harmonics), and the THD of harmonics up to h.

$$
I_h = \frac{I_1}{h},\quad h = 6k\pm1\ (3\phi),\ h\ \text{odd}\ (1\phi)
$$

| input | meaning | condition |
|---|---|---|
| `circuit` | single-phase or three-phase bridge |  |
| `Id` | constant dc output current |  |
| `h` | harmonic order |  |

| output | meaning |
|---|---|
| `Ih` | RMS current of harmonic h |
| `Ih_over_I1` | ratio to the fundamental |
| `THD_to_h` | THD counting harmonics up to order h |
| `I1` | RMS fundamental current |

- **Method:** single-phase: odd h, $Ih = 2$ sqrt(2) Id/(h pi); three-phase: $h = 6k$ +- 1, $Ih = sqrt$(6) Id/(h pi); others 0
- **Assumes:** $1  \leq  h  \leq  49$; constant dc current, no overlap
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ih (RMS current of harmonic h) against h (harmonic order) from 1 to 13](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-rectifier-line-harmonics-curve)

*Ih (RMS current of harmonic h) against h (harmonic order) from 1 to 13. One curve for each value of Id (constant dc output current): 100 A, 50 A, 200 A. Other inputs as in the worked example: circuit = three-phase. 13 points per curve computed by power-electronics-ai999x; marked: the worked example, h = 5, Ih = 15.5939 A.*

![Ih (RMS current of harmonic h) over Id (constant dc output current) from 50 A to 150 A and h (harmonic order) from 1 to 13: each of the 21 × 13 cells is one run of power-electronics-ai999x, shaded by its value (light 0 A to dark 116](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-rectifier-line-harmonics-field)

*Ih (RMS current of harmonic h) over Id (constant dc output current) from 50 A to 150 A and h (harmonic order) from 1 to 13: each of the 21 × 13 cells is one run of power-electronics-ai999x, shaded by its value (light 0 A to dark 116.955 A); iso-lines at Ih = 20, 40, 60, 80, 100 A. Other inputs as in the worked example: circuit = three-phase. Marked: the worked example, Id = 100 A, h = 5, Ih = 15.5939 A.*

```python
power_electronics_ai999x.solve('pe_rectifier_line_harmonics', circuit='three-phase', Id=100.0, h=5)
# -> Ih = 15.5939 A, Ih_over_I1 = 0.2, THD_to_h = 0.2, I1 = 77.9697 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_rectifier_line_harmonics', circuit='single-phase', Id=20.0, h=7)
# -> Ih = 2.57233 A, Ih_over_I1 = 0.142857, THD_to_h = 0.414149, I1 = 18.0063 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Rectifier line harmonics: circuit = three-phase, Id = 100 A, h = 5 | SOLVED | Ih = 15.5939 A, Ih_over_I1 = 0.2, THD_to_h = 0.2, I1 = 77.9697 A |
| Rectifier line harmonics: circuit = single-phase, Id = 20 A, h = 7 | SOLVED | Ih = 2.57233 A, Ih_over_I1 = 0.142857, THD_to_h = 0.414149, I1 = 18.0063 A |
| Rectifier line harmonics: Id = 100 A, h = 5 | NEEDS_INPUT | pe_rectifier_line_harmonics needs circuit (single-phase or three-phase bridge). Example: solve('pe_rectifier_line_harmon |
| Rectifier line harmonics: circuit = three-phase, Id = 100 A, h = 5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_half_wave_rl_rectifier"></a>

### Half-wave diode rectifier with an RL load: `pe_half_wave_rl_rectifier`

The extinction angle (where the load current returns to zero), the load impedance angle and magnitude, and the average output voltage and current of a half-wave diode rectifier feeding a series RL load.

$$
\sin(\beta-\theta)+\sin\theta\,e^{-\beta/\omega\tau} = 0,\quad V_{avg} = \frac{V_m(1-\cos\beta)}{2\pi}
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | RMS source voltage |  |
| `f` | line frequency |  |
| `R` | load resistance |  |
| `L` | load inductance |  |

| output | meaning |
|---|---|
| `beta_deg` | extinction angle in degrees |
| `theta_deg` | load impedance angle atan(wL/R) in degrees |
| `Z` | load impedance magnitude |
| `V_avg` | average output voltage |
| `I_avg` | average load current |

- **Method:** sin(beta - theta) + sin(theta) exp(-beta/(wL/R)) = 0 (bisection on $pi < \beta < 2$ pi); Vavg = Vm (1 - cos $\beta)/(2$ pi); Iavg = Vavg/R
- **Assumes:** ideal diode, no freewheeling diode
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![beta_deg (extinction angle in degrees) against f (line frequency) from 6 Hz to 600 Hz on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-half-wave-rl-rectifier-curve)

*beta_deg (extinction angle in degrees) against f (line frequency) from 6 Hz to 600 Hz on a logarithmic axis. One curve for each value of R (load resistance): 100 ohm, 50 ohm, 200 ohm. Other inputs as in the worked example: Vs = 120.0 V, L = 0.1 H. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 60 Hz, beta_deg = 200.658.*

```python
power_electronics_ai999x.solve('pe_half_wave_rl_rectifier', Vs=120.0, f=60.0, R=100.0, L=0.1)
# -> beta_deg = 200.658, theta_deg = 20.656, Z = 106.87 ohm, V_avg = 52.2824 V, I_avg = 0.522824 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_half_wave_rl_rectifier', Vs=230.0, f=50.0, R=10.0, L=0.05)
# -> beta_deg = 240.847, theta_deg = 57.5184, Z = 18.621 ohm, V_avg = 76.9866 V, I_avg = 7.69866 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Half-wave rectifier with RL load: Vs = 120 V, f = 0.06 kHz, R = 100 ohm, L = 100 mH | SOLVED | beta_deg = 200.658, theta_deg = 20.656, Z = 106.87 ohm, V_avg = 52.2824 V, I_avg = 0.522824 A |
| Half-wave rectifier with RL load: Vs = 230 V, f = 0.05 kHz, R = 10 ohm, L = 50 mH | SOLVED | beta_deg = 240.847, theta_deg = 57.5184, Z = 18.621 ohm, V_avg = 76.9866 V, I_avg = 7.69866 A |
| Half-wave rectifier with RL load: f = 0.06 kHz, R = 100 ohm, L = 100 mH | NEEDS_INPUT | pe_half_wave_rl_rectifier needs Vs (RMS source voltage). Example: solve('pe_half_wave_rl_rectifier', Vs='120 V', f='60 H |
| Half-wave rectifier with RL load: Vs = 120 V, f = 0.06 kHz, R = 100 ohm, L = 100 mH, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_capacitor_filter_exact"></a>

### Capacitor-input rectifier filter: exact ripple and peak diode current, `pe_capacitor_filter_exact`

The exact steady state of a half-wave or full-wave rectifier with a smoothing capacitor and resistive load: diode turn-off angle, recharge angle, peak-to-peak ripple, average output voltage, peak diode current and conduction angle (no small-ripple approximation).

$$
\theta = \pi - \tan^{-1}(\omega RC),\ \sin\alpha = \sin\theta\,e^{-(P+\alpha-\theta)/\omega RC},\ I_{D,pk} = V_p\left(\omega C\cos\alpha+\frac{\sin\alpha}{R}\right)
$$

| input | meaning | condition |
|---|---|---|
| `circuit` | half-wave or full-wave |  |
| `Vp` | peak source voltage (after any diode drop) |  |
| `f` | line frequency |  |
| `R` | load resistance |  |
| `C` | filter capacitance |  |

| output | meaning |
|---|---|
| `theta_off_deg` | angle at which the diode turns off, degrees |
| `alpha_deg` | angle at which the diode turns on again, degrees |
| `dV` | peak-to-peak ripple |
| `V_avg` | average output voltage |
| `I_D_peak` | peak diode current |
| `conduction_deg` | diode conduction angle, degrees |

- **Method:** theta = pi - atan(w R C); sin(alpha) = sin(theta) exp(-(P + alpha - theta)/(w R C)) (bisection), $P = 2$ pi or pi; $dV = Vp$ (1 - sin alpha); I_D,peak = Vp (w C cos alpha + sin alpha/R)
- **Assumes:** ideal diode and source, steady state
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dV (peak-to-peak ripple) against f (line frequency) from 6 Hz to 600 Hz on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-capacitor-filter-exact-curve)

*dV (peak-to-peak ripple) against f (line frequency) from 6 Hz to 600 Hz on a logarithmic axis. One curve for each value of Vp (peak source voltage (after any diode drop)): 169.7 V, 84.85 V, 339.4 V. Other inputs as in the worked example: circuit = half-wave, R = 500.0 ohm, C = 9.999999999999999e-05 F. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 60 Hz, dV = 43.1319 V.*

```python
power_electronics_ai999x.solve('pe_capacitor_filter_exact', circuit='half-wave', Vp=169.7, f=60.0, R=500.0, C=0.0001)
# -> theta_off_deg = 93.0368, alpha_deg = 48.2308, dV = 43.1319 V, V_avg = 148.103 V, I_D_peak = 4.51474 A, conduction_deg = 44.806   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_capacitor_filter_exact', circuit='full-wave', Vp=15.0, f=50.0, R=100.0, C=0.00047)
# -> theta_off_deg = 93.8745, alpha_deg = 57.3594, dV = 2.36895 V, V_avg = 13.8708 V, I_D_peak = 1.32092 A, conduction_deg = 36.5151   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Exact capacitor filter: circuit = half-wave, Vp = 169.7 V, f = 0.06 kHz, R = 500 ohm, C = 100 uF | SOLVED | theta_off_deg = 93.0368, alpha_deg = 48.2308, dV = 43.1319 V, V_avg = 148.103 V, I_D_peak = 4.51474 A, conduction_deg = 44.806 |
| Exact capacitor filter: circuit = full-wave, Vp = 15 V, f = 0.05 kHz, R = 100 ohm, C = 470 uF | SOLVED | theta_off_deg = 93.8745, alpha_deg = 57.3594, dV = 2.36895 V, V_avg = 13.8708 V, I_D_peak = 1.32092 A, conduction_deg = 36.5151 |
| Exact capacitor filter: Vp = 169.7 V, f = 0.06 kHz, R = 500 ohm, C = 100 uF | NEEDS_INPUT | pe_capacitor_filter_exact needs circuit (half-wave or full-wave). Example: solve('pe_capacitor_filter_exact', circuit='h |
| Exact capacitor filter: circuit = half-wave, Vp = 169.7 V, f = 0.06 kHz, R = 500 ohm, C = 100 uF, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_single_phase_overlap"></a>

### Commutation overlap in a single-phase thyristor bridge, `pe_single_phase_overlap`

The overlap (commutation) angle and the reduced average output voltage of a single-phase fully controlled bridge with source inductance Ls and a constant dc current.

$$
\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_s I_d}{V_m},\quad V_{dc} = \frac{2V_m}{\pi}\cos\alpha - \frac{2\omega L_sI_d}{\pi}
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | RMS source voltage |  |
| `f` | line frequency |  |
| `Ls` | source (commutating) inductance |  |
| `Id` | constant dc current |  |
| `alpha` | firing angle |  |

| output | meaning |
|---|---|
| `u_deg` | overlap angle in degrees |
| `Vdc` | average output voltage with overlap |
| `dV` | voltage lost to commutation |

- **Method:** cos(alpha + u) = cos(alpha) - 2 w Ls Id/Vm; Vdc = 2 Vm cos(alpha)/pi - 2 w Ls Id/pi
- **Assumes:** commutation finishes within the half-cycle
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![u_deg (overlap angle in degrees) against alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-single-phase-overlap-curve)

*u_deg (overlap angle in degrees) against alpha (firing angle) from 0 rad to 1.5708 rad. One curve for each value of Vs (RMS source voltage): 230 V, 115 V, 460 V. Other inputs as in the worked example: f = 50.0 Hz, Ls = 0.002 H, Id = 20.0 A. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0.523599 rad, u_deg = 7.93042.*

```python
power_electronics_ai999x.solve('pe_single_phase_overlap', Vs=230.0, f=50.0, Ls=0.002, Id=20.0, alpha=0.5235987755982988)
# -> u_deg = 7.93042, Vdc = 171.33 V, dV = 8 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_single_phase_overlap', Vs=120.0, f=60.0, Ls=0.001, Id=15.0, alpha=0.0)
# -> u_deg = 21.0357, Vdc = 104.438 V, dV = 3.6 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Single-phase commutation overlap: Vs = 230 V, f = 0.05 kHz, Ls = 2 mH, Id = 20 A, alpha = 30 deg | SOLVED | u_deg = 7.93042, Vdc = 171.33 V, dV = 8 V |
| Single-phase commutation overlap: Vs = 120 V, f = 0.06 kHz, Ls = 1 mH, Id = 15 A, alpha = 0 deg | SOLVED | u_deg = 21.0357, Vdc = 104.438 V, dV = 3.6 V |
| Single-phase commutation overlap: f = 0.05 kHz, Ls = 2 mH, Id = 20 A, alpha = 30 deg | NEEDS_INPUT | pe_single_phase_overlap needs Vs (RMS source voltage). Example: solve('pe_single_phase_overlap', Vs='230 V', f='50 Hz',  |
| Single-phase commutation overlap: Vs = 230 V, f = 0.05 kHz, Ls = 2 mH, Id = 20 A, alpha = 30 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_three_phase_overlap"></a>

### Commutation overlap in a three-phase thyristor bridge, `pe_three_phase_overlap`

The overlap angle, the average output voltage and the voltage lost to commutation of a six-pulse thyristor (or diode, alpha = 0) bridge with source inductance Ls per phase and a constant dc current.

$$
\cos(\alpha+u) = \cos\alpha - \frac{2\omega L_sI_d}{\sqrt2 V_{LL}},\quad V_{dc} = \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha - \frac{3\omega L_sI_d}{\pi}
$$

| input | meaning | condition |
|---|---|---|
| `VLL` | RMS line-to-line voltage |  |
| `f` | line frequency |  |
| `Ls` | source inductance per phase |  |
| `Id` | constant dc current |  |
| `alpha` | firing angle |  |

| output | meaning |
|---|---|
| `u_deg` | overlap angle in degrees |
| `Vdc` | average output voltage |
| `dV` | voltage lost to commutation |

- **Method:** cos(alpha + u) = cos(alpha) - 2 w Ls Id/(sqrt(2) VLL); Vdc = 3 sqrt(2) VLL cos(alpha)/pi - 3 w Ls Id/pi
- **Assumes:** commutation finishes before the next one starts
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![u_deg (overlap angle in degrees) against alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-three-phase-overlap-curve)

*u_deg (overlap angle in degrees) against alpha (firing angle) from 0 rad to 1.5708 rad. One curve for each value of VLL (RMS line-to-line voltage): 400 V, 200 V, 800 V. Other inputs as in the worked example: f = 50.0 Hz, Ls = 0.0005 H, Id = 100.0 A. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0.523599 rad, u_deg = 5.85623.*

```python
power_electronics_ai999x.solve('pe_three_phase_overlap', VLL=400.0, f=50.0, Ls=0.0005, Id=100.0, alpha=0.5235987755982988)
# -> u_deg = 5.85623, Vdc = 452.818 V, dV = 15 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_three_phase_overlap', VLL=480.0, f=60.0, Ls=0.0002, Id=200.0, alpha=0.0)
# -> u_deg = 17.1432, Vdc = 633.828 V, dV = 14.4 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Three-phase commutation overlap: VLL = 400 V, f = 0.05 kHz, Ls = 0.5 mH, Id = 100 A, alpha = 30 deg | SOLVED | u_deg = 5.85623, Vdc = 452.818 V, dV = 15 V |
| Three-phase commutation overlap: VLL = 480 V, f = 0.06 kHz, Ls = 0.2 mH, Id = 200 A, alpha = 0 deg | SOLVED | u_deg = 17.1432, Vdc = 633.828 V, dV = 14.4 V |
| Three-phase commutation overlap: f = 0.05 kHz, Ls = 0.5 mH, Id = 100 A, alpha = 30 deg | NEEDS_INPUT | pe_three_phase_overlap needs VLL (RMS line-to-line voltage). Example: solve('pe_three_phase_overlap', VLL='400 V', f='50 |
| Three-phase commutation overlap: VLL = 400 V, f = 0.05 kHz, Ls = 0.5 mH, Id = 100 A, alpha = 30 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_thd_from_harmonics"></a>

### Total harmonic distortion from harmonic values, `pe_thd_from_harmonics`

The total harmonic distortion, RMS value and distortion factor of a waveform from the RMS value of its fundamental and of its harmonics 2, 3, 4, ... in order.

$$
THD = \frac{\sqrt{\sum_{h\ge2} V_h^2}}{V_1},\quad V_{rms} = V_1\sqrt{1+THD^2}
$$

| input | meaning | condition |
|---|---|---|
| `V1` | RMS value of the fundamental (any unit, the same as the harmonics) |  |
| `Vh` | RMS values of the harmonics in order of harmonic number, starting with the second |  |

| output | meaning |
|---|---|
| `THD` | total harmonic distortion |
| `THD_pct` | THD in percent |
| `Vrms` | RMS value of the waveform |
| `distortion_factor` | distortion factor |

- **Method:** THD = sqrt(sum Vh^2)/V1; Vrms = sqrt(V1^2 + sum Vh^2); $DF = V_{1}$/$V_{\mathrm{rms}} = \frac{1}{\sqrt{1 + THD^{2}}}$
- **Assumes:** $V_{1} > 0$, harmonics >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![THD (total harmonic distortion) against V1 (RMS value of the fundamental (any unit, the same as the harmonics)) from 115 to 345](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-thd-from-harmonics-curve)

*THD (total harmonic distortion) against V1 (RMS value of the fundamental (any unit, the same as the harmonics)) from 115 to 345. Other inputs as in the worked example: Vh = [0.0, 11.5, 0.0, 6.9, 0.0, 4.6]. 31 points computed by power-electronics-ai999x; marked: the worked example, V1 = 230, THD = 0.0616441.*

```python
power_electronics_ai999x.solve('pe_thd_from_harmonics', V1=230.0, Vh=[0.0, 11.5, 0.0, 6.9, 0.0, 4.6])
# -> THD = 0.0616441, THD_pct = 6.16441, Vrms = 230.437, distortion_factor = 0.998105   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_thd_from_harmonics', V1=10.0, Vh=[0.0, 3.33, 0.0, 2.0])
# -> THD = 0.388444, THD_pct = 38.8444, Vrms = 10.7279, distortion_factor = 0.932145   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| THD from harmonics: V1 = 230, Vh = [0, 11.5, 0, 6.9, 0, 4.6] | SOLVED | THD = 0.0616441, THD_pct = 6.16441, Vrms = 230.437, distortion_factor = 0.998105 |
| THD from harmonics: V1 = 10, Vh = [0, 3.33, 0, 2] | SOLVED | THD = 0.388444, THD_pct = 38.8444, Vrms = 10.7279, distortion_factor = 0.932145 |
| THD from harmonics: Vh = [0, 11.5, 0, 6.9, 0, 4.6] | NEEDS_INPUT | pe_thd_from_harmonics needs V1 (RMS value of the fundamental (any unit, the same as the harmonics)). Example: solve('pe_ |
| THD from harmonics: V1 = 230, Vh = [0, 11.5, 0, 6.9, 0, 4.6], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_power_factor_distortion"></a>

### Power factor with harmonic distortion: `pe_power_factor_distortion`

The displacement power factor, distortion factor and true power factor of a load drawing a distorted current from a sinusoidal voltage, from the current THD and the displacement angle of its fundamental.

$$
PF = \frac{\cos\phi_1}{\sqrt{1+THD_i^2}}
$$

| input | meaning | condition |
|---|---|---|
| `THD` | current total harmonic distortion (a fraction or %) |  |
| `phi` | displacement angle of the fundamental current (lagging positive) |  |

| output | meaning |
|---|---|
| `DPF` | displacement power factor cos(phi) |
| `distortion_factor` | distortion factor |
| `PF` | true power factor |

- **Method:** $PF = \frac{\cos{\left(\phi \right)}}{\sqrt{1 + THD^{2}}}$
- **Assumes:** sinusoidal voltage; $-90  \leq  \phi  \leq  90$ degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PF (true power factor) against phi (displacement angle of the fundamental current (lagging positive)) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-power-factor-distortion-curve)

*PF (true power factor) against phi (displacement angle of the fundamental current (lagging positive)) from 0 rad to 1.5708 rad. One curve for each value of THD (current total harmonic distortion (a fraction or %)): 0.483, 0.26, 0.74. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, phi = 0 rad, PF = 0.900467.*

![PF (true power factor) over phi (displacement angle of the fundamental current (lagging positive)) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-power-factor-distortion-field)

*PF (true power factor) over phi (displacement angle of the fundamental current (lagging positive)) from 0 rad to 1.5708 rad and THD (current total harmonic distortion (a fraction or %)) from 0.02 to 0.98: each of the 21 × 15 cells is one run of power-electronics-ai999x, shaded by its value (light 3.4972e-12 to dark 0.9998); iso-lines at PF = 0.2, 0.4, 0.6, 0.8. Marked: the worked example, phi = 0 rad, THD = 0.483, PF = 0.900467.*

```python
power_electronics_ai999x.solve('pe_power_factor_distortion', THD=0.483, phi=0.0)
# -> DPF = 1, distortion_factor = 0.900467, PF = 0.900467   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_power_factor_distortion', THD=0.3, phi=0.5235987755982988)
# -> DPF = 0.866025, distortion_factor = 0.957826, PF = 0.829502   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Power factor with distortion: THD = 0.483, phi = 0 deg | SOLVED | DPF = 1, distortion_factor = 0.900467, PF = 0.900467 |
| Power factor with distortion: THD = 0.3, phi = 30 deg | SOLVED | DPF = 0.866025, distortion_factor = 0.957826, PF = 0.829502 |
| Power factor with distortion: phi = 0 deg | NEEDS_INPUT | pe_power_factor_distortion needs THD (current total harmonic distortion (a fraction or %)). Example: solve('pe_power_fac |
| Power factor with distortion: THD = 0.483, phi = 0 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Power factor with distortion: THD = 48.3%, phi = 0 deg | SOLVED | DPF = 1, distortion_factor = 0.900467, PF = 0.900467 |

<a id="pe_pulse_waveform_rms"></a>

### Average and RMS of a rectangular pulse train, `pe_pulse_waveform_rms`

The average, RMS and AC-RMS values, form factor and ripple factor of a unipolar rectangular pulse train of amplitude A and duty cycle D (a PWM switch voltage or current).

$$
V_{avg} = DA,\quad V_{rms} = \sqrt{D}\,A,\quad RF = \sqrt{\tfrac1D - 1}
$$

| input | meaning | condition |
|---|---|---|
| `A` | pulse amplitude (any unit) |  |
| `D` | duty cycle (a fraction or %) |  |

| output | meaning |
|---|---|
| `avg` | average value D A |
| `rms` | RMS value sqrt(D) A |
| `ac_rms` | RMS of the AC component |
| `form_factor` | form factor rms/avg |
| `ripple_factor` | ripple factor ac_rms/avg |

- **Method:** avg = D A; rms = sqrt(D) A; $ac = A$ sqrt(D $(1 - D)$); $FF = \frac{1}{\sqrt{D}}$; $RF = \sqrt{\frac{1}{D} - 1}$
- **Assumes:** $0 < D < 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![rms (RMS value sqrt(D) A) against D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-pulse-waveform-rms-curve)

*rms (RMS value sqrt(D) A) against D (duty cycle (a fraction or %)) from 0.02 to 0.98. One curve for each value of A (pulse amplitude (any unit)): 12, 6, 24. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.25, rms = 6.*

![rms (RMS value sqrt(D) A) over A (pulse amplitude (any unit)) from 6 to 18 and D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-pulse-waveform-rms-field)

*rms (RMS value sqrt(D) A) over A (pulse amplitude (any unit)) from 6 to 18 and D (duty cycle (a fraction or %)) from 0.02 to 0.98: each of the 21 × 15 cells is one run of power-electronics-ai999x, shaded by its value (light 0.848528 to dark 17.8191); iso-lines at rms = 5, 10, 15. Marked: the worked example, A = 12, D = 0.25, rms = 6.*

```python
power_electronics_ai999x.solve('pe_pulse_waveform_rms', A=12.0, D=0.25)
# -> avg = 3, rms = 6, ac_rms = 5.19615, form_factor = 2, ripple_factor = 1.73205   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_pulse_waveform_rms', A=100.0, D=0.6)
# -> avg = 60, rms = 77.4597, ac_rms = 48.9898, form_factor = 1.29099, ripple_factor = 0.816497   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Pulse train RMS: A = 12, D = 0.25 | SOLVED | avg = 3, rms = 6, ac_rms = 5.19615, form_factor = 2, ripple_factor = 1.73205 |
| Pulse train RMS: A = 100, D = 0.6 | SOLVED | avg = 60, rms = 77.4597, ac_rms = 48.9898, form_factor = 1.29099, ripple_factor = 0.816497 |
| Pulse train RMS: D = 0.25 | NEEDS_INPUT | pe_pulse_waveform_rms needs A (pulse amplitude (any unit)). Example: solve('pe_pulse_waveform_rms', A=12, D=0.25). |
| Pulse train RMS: A = 12, D = 0.25, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_square_wave_inverter"></a>

### Square-wave inverter output and harmonics: `pe_square_wave_inverter`

The RMS output voltage, the fundamental (RMS and peak), the RMS value of an odd harmonic n and the THD of a single-phase half-bridge or full-bridge inverter operated with square-wave (180 degree) switching.

$$
V_{n,rms} = \frac{4A}{n\pi\sqrt2},\quad THD = \sqrt{\frac{\pi^2}{8}-1}
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc input voltage |  |
| `bridge` | half-bridge or full-bridge |  |
| `n` | odd harmonic order |  |

| output | meaning |
|---|---|
| `Vrms` | RMS output voltage |
| `V1_rms` | RMS fundamental |
| `V1_peak` | peak fundamental |
| `Vn_rms` | RMS value of harmonic n |
| `THD` | total harmonic distortion |

- **Method:** A = Vdc (full bridge) or Vdc/2 (half bridge); $V_{1} = 4$ A/(pi sqrt 2); $Vn = V_{1}/n$; $THD = \sqrt{\frac{\pi^{2}}{8} - 1}$
- **Assumes:** n odd, 3..99; ideal switching, no dead time
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V1_rms (RMS fundamental) against Vdc (dc input voltage) from 50 V to 150 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-square-wave-inverter-curve)

*V1_rms (RMS fundamental) against Vdc (dc input voltage) from 50 V to 150 V. One curve for each value of bridge (half-bridge or full-bridge): full-bridge, half-bridge. Other inputs as in the worked example: n = 3. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Vdc = 100 V, V1_rms = 90.0316 V.*

```python
power_electronics_ai999x.solve('pe_square_wave_inverter', Vdc=100.0, bridge='full-bridge', n=3)
# -> Vrms = 100 V, V1_rms = 90.0316 V, V1_peak = 127.324 V, Vn_rms = 30.0105 V, THD = 0.483426   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_square_wave_inverter', Vdc=48.0, bridge='half-bridge', n=5)
# -> Vrms = 24 V, V1_rms = 21.6076 V, V1_peak = 30.5577 V, Vn_rms = 4.32152 V, THD = 0.483426   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Square-wave inverter: Vdc = 100 V, bridge = full-bridge, n = 3 | SOLVED | Vrms = 100 V, V1_rms = 90.0316 V, V1_peak = 127.324 V, Vn_rms = 30.0105 V, THD = 0.483426 |
| Square-wave inverter: Vdc = 48 V, bridge = half-bridge, n = 5 | SOLVED | Vrms = 24 V, V1_rms = 21.6076 V, V1_peak = 30.5577 V, Vn_rms = 4.32152 V, THD = 0.483426 |
| Square-wave inverter: bridge = full-bridge, n = 3 | NEEDS_INPUT | pe_square_wave_inverter needs Vdc (dc input voltage). Example: solve('pe_square_wave_inverter', Vdc='100 V', bridge='ful |
| Square-wave inverter: Vdc = 100 V, bridge = full-bridge, n = 3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_quasi_square_inverter"></a>

### Quasi-square-wave (phase-shifted) full-bridge inverter, `pe_quasi_square_inverter`

The fundamental, the RMS value of an odd harmonic n, the RMS output and the THD of a full-bridge inverter with a zero-voltage interval alpha at each end of every half-cycle, and the alpha that eliminates harmonic n.

$$
V_{n,rms} = \frac{4V_{dc}}{n\pi\sqrt2}\cos(n\alpha),\quad V_{rms} = V_{dc}\sqrt{1-\frac{2\alpha}{\pi}}
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc input voltage |  |
| `alpha` | zero-voltage interval at each end of a half-cycle |  |
| `n` | odd harmonic order |  |

| output | meaning |
|---|---|
| `V1_rms` | RMS fundamental |
| `Vn_rms` | RMS value of harmonic n |
| `Vrms` | RMS output voltage |
| `THD` | total harmonic distortion |
| `alpha_eliminate_n_deg` | zero interval in degrees that removes harmonic n |

- **Method:** $Vn = 4$ Vdc cos(n alpha)/(n pi sqrt 2); Vrms = Vdc sqrt($1 - 2$ alpha/pi); harmonic n vanishes at $\alpha = 90/n$ degrees
- **Assumes:** $0  \leq  \alpha < 90$ degrees; n odd, 3..99
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V1_rms (RMS fundamental) against alpha (zero-voltage interval at each end of a half-cycle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-quasi-square-inverter-curve)

*V1_rms (RMS fundamental) against alpha (zero-voltage interval at each end of a half-cycle) from 0 rad to 1.5708 rad. One curve for each value of Vdc (dc input voltage): 100 V, 50 V, 200 V. Other inputs as in the worked example: n = 3. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, alpha = 0.523599 rad, V1_rms = 77.9697 V.*

```python
power_electronics_ai999x.solve('pe_quasi_square_inverter', Vdc=100.0, alpha=0.5235987755982988, n=3)
# -> V1_rms = 77.9697 V, Vn_rms = 0 V, Vrms = 81.6497 V, THD = 0.310842, alpha_eliminate_n_deg = 30   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_quasi_square_inverter', Vdc=200.0, alpha=0.3141592653589793, n=5)
# -> V1_rms = 171.25 V, Vn_rms = 0 V, Vrms = 178.885 V, THD = 0.301922, alpha_eliminate_n_deg = 18   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Quasi-square-wave inverter: Vdc = 100 V, alpha = 30 deg, n = 3 | SOLVED | V1_rms = 77.9697 V, Vn_rms = 0 V, Vrms = 81.6497 V, THD = 0.310842, alpha_eliminate_n_deg = 30 |
| Quasi-square-wave inverter: Vdc = 200 V, alpha = 18 deg, n = 5 | SOLVED | V1_rms = 171.25 V, Vn_rms = 0 V, Vrms = 178.885 V, THD = 0.301922, alpha_eliminate_n_deg = 18 |
| Quasi-square-wave inverter: alpha = 30 deg, n = 3 | NEEDS_INPUT | pe_quasi_square_inverter needs Vdc (dc input voltage). Example: solve('pe_quasi_square_inverter', Vdc='100 V', alpha='0. |
| Quasi-square-wave inverter: Vdc = 100 V, alpha = 30 deg, n = 3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_six_step_inverter"></a>

### Three-phase six-step (square-wave) inverter, `pe_six_step_inverter`

The RMS line and phase voltages, their fundamentals, the THD and the 5th and 7th harmonics of a three-phase voltage-source inverter in six-step (180 degree) operation.

$$
V_{LL1} = \frac{\sqrt6}{\pi}V_{dc},\quad V_{LL} = \sqrt{\tfrac23}V_{dc},\quad V_{LL,h} = \frac{V_{LL1}}{h}
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc-link voltage |  |
| `f` | output (fundamental) frequency |  |

| output | meaning |
|---|---|
| `VLL_rms` | RMS line-to-line voltage |
| `VLL1_rms` | RMS fundamental line voltage |
| `Vph_rms` | RMS phase voltage |
| `Vph1_rms` | RMS fundamental phase voltage |
| `THD` | THD of the line voltage |
| `VLL5_rms` | RMS fifth-harmonic line voltage |
| `VLL7_rms` | RMS seventh-harmonic line voltage |
| `f5` | fifth-harmonic frequency |
| `f7` | seventh-harmonic frequency |

- **Method:** $V_LL = sqrt$(2/3) Vdc; $V_LL1 = sqrt$(6) Vdc/pi = 0.78 Vdc; harmonics 6k +- 1 at $V_LL1/h$
- **Assumes:** balanced star load, ideal switching
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![VLL1_rms (RMS fundamental line voltage) against Vdc (dc-link voltage) from 300 V to 900 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-six-step-inverter-curve)

*VLL1_rms (RMS fundamental line voltage) against Vdc (dc-link voltage) from 300 V to 900 V. Outputs with the same unit drawn together: VLL_rms (RMS line-to-line voltage), Vph_rms (RMS phase voltage), Vph1_rms (RMS fundamental phase voltage). Other inputs as in the worked example: f = 50.0 Hz. 31 points computed by power-electronics-ai999x; marked: the worked example, Vdc = 600 V, VLL1_rms = 467.818 V.*

```python
power_electronics_ai999x.solve('pe_six_step_inverter', Vdc=600.0, f=50.0)
# -> VLL_rms = 489.898 V, VLL1_rms = 467.818 V, Vph_rms = 282.843 V, Vph1_rms = 270.095 V, THD = 0.310842, VLL5_rms = 93.5636 V, VLL7_rms = 66.8312 V, f5 = 250 Hz, f7 = 350 Hz   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_six_step_inverter', Vdc=48.0, f=200.0)
# -> VLL_rms = 39.1918 V, VLL1_rms = 37.4254 V, Vph_rms = 22.6274 V, Vph1_rms = 21.6076 V, THD = 0.310842, VLL5_rms = 7.48509 V, VLL7_rms = 5.34649 V, f5 = 1000 Hz, f7 = 1400 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Six-step inverter: Vdc = 600 V, f = 50 Hz | SOLVED | VLL_rms = 489.898 V, VLL1_rms = 467.818 V, Vph_rms = 282.843 V, Vph1_rms = 270.095 V, THD = 0.310842, VLL5_rms = 93.5636 V, VLL7_rms = 66.8312 V, f5 = 250 Hz, f7 = 350 Hz |
| Six-step inverter: Vdc = 48 V, f = 200 Hz | SOLVED | VLL_rms = 39.1918 V, VLL1_rms = 37.4254 V, Vph_rms = 22.6274 V, Vph1_rms = 21.6076 V, THD = 0.310842, VLL5_rms = 7.48509 V, VLL7_rms = 5.34649 V, f5 = 1000 Hz, f7 = 1400 Hz |
| Six-step inverter: f = 50 Hz | NEEDS_INPUT | pe_six_step_inverter needs Vdc (dc-link voltage). Example: solve('pe_six_step_inverter', Vdc='600 V', f='50 Hz'). |
| Six-step inverter: Vdc = 600 V, f = 50 Hz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_spwm_fundamental"></a>

### Sinusoidal PWM: fundamental output voltage, `pe_spwm_fundamental`

The fundamental output voltage of sinusoidal (sine-triangle) PWM in the linear range: the peak and RMS phase-leg voltage, the RMS line voltage of a three-phase inverter, the RMS output of a bipolar full bridge, and the carrier frequency.

$$
\hat V_{1,leg} = m_a\frac{V_{dc}}{2},\quad V_{LL1,rms} = \frac{\sqrt3}{2\sqrt2}m_aV_{dc},\quad f_c = m_f f_1
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc-link voltage |  |
| `ma` | amplitude modulation index |  |
| `mf` | frequency modulation ratio (odd whole number) |  |
| `f1` | fundamental output frequency |  |

| output | meaning |
|---|---|
| `V1_leg_peak` | peak fundamental of a leg against the dc midpoint |
| `V1_leg_rms` | RMS fundamental of a leg |
| `V1_LL_rms` | RMS fundamental line voltage of a three-phase inverter |
| `V1_fullbridge_rms` | RMS fundamental of a bipolar full bridge |
| `f_carrier` | carrier (switching) frequency |

- **Method:** V1,leg = ma Vdc/2 (peak); V1,$LL = sqrt$(3) ma Vdc/(2 sqrt 2); bipolar full bridge $V_{1} = ma$ Vdc/sqrt 2; $fc = mf$ f1
- **Assumes:** $0 < ma  \leq  1$; mf odd, 15..201 (natural sampling)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V1_leg_peak (peak fundamental of a leg against the dc midpoint) against Vdc (dc-link voltage) from 300 V to 900 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-spwm-fundamental-curve)

*V1_leg_peak (peak fundamental of a leg against the dc midpoint) against Vdc (dc-link voltage) from 300 V to 900 V. One curve for each value of ma (amplitude modulation index): 0.8, 0.4. Other inputs as in the worked example: mf = 21, f1 = 50.0 Hz. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Vdc = 600 V, V1_leg_peak = 240 V.*

```python
power_electronics_ai999x.solve('pe_spwm_fundamental', Vdc=600.0, ma=0.8, mf=21, f1=50.0)
# -> V1_leg_peak = 240 V, V1_leg_rms = 169.706 V, V1_LL_rms = 293.939 V, V1_fullbridge_rms = 339.411 V, f_carrier = 1050 Hz   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_spwm_fundamental', Vdc=400.0, ma=1.0, mf=39, f1=60.0)
# -> V1_leg_peak = 200 V, V1_leg_rms = 141.421 V, V1_LL_rms = 244.949 V, V1_fullbridge_rms = 282.843 V, f_carrier = 2340 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Sinusoidal PWM fundamental: Vdc = 600 V, ma = 0.8, mf = 21, f1 = 50 Hz | SOLVED | V1_leg_peak = 240 V, V1_leg_rms = 169.706 V, V1_LL_rms = 293.939 V, V1_fullbridge_rms = 339.411 V, f_carrier = 1050 Hz |
| Sinusoidal PWM fundamental: Vdc = 400 V, ma = 1, mf = 39, f1 = 60 Hz | SOLVED | V1_leg_peak = 200 V, V1_leg_rms = 141.421 V, V1_LL_rms = 244.949 V, V1_fullbridge_rms = 282.843 V, f_carrier = 2340 Hz |
| Sinusoidal PWM fundamental: ma = 0.8, mf = 21, f1 = 50 Hz | NEEDS_INPUT | pe_spwm_fundamental needs Vdc (dc-link voltage). Example: solve('pe_spwm_fundamental', Vdc='600 V', ma=0.8, mf=21, f1='5 |
| Sinusoidal PWM fundamental: Vdc = 600 V, ma = 0.8, mf = 21, f1 = 50 Hz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_space_vector_times"></a>

### Space-vector modulation dwell times: `pe_space_vector_times`

The sector and the dwell times T1, T2 of the two adjacent active vectors and T0 of the zero vectors that synthesise a reference space vector (peak phase voltage Vref at angle theta) in one switching period of a two-level three-phase inverter.

$$
T_1 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\!\left(\tfrac{\pi}{3}-\theta'\right),\quad T_2 = \frac{\sqrt3 T_s V_{ref}}{V_{dc}}\sin\theta',\quad T_0 = T_s - T_1 - T_2
$$

| input | meaning | condition |
|---|---|---|
| `Vref` | reference vector magnitude (peak phase voltage) |  |
| `theta` | reference vector angle |  |
| `Vdc` | dc-link voltage |  |
| `fs` | switching (sampling) frequency |  |

| output | meaning |
|---|---|
| `sector` | sector number |
| `T1` | dwell time of the first active vector |
| `T2` | dwell time of the second active vector |
| `T0` | total zero-vector time |
| `m` | modulation index |

- **Method:** $T_{1} = sqrt$(3) Ts Vref/Vdc sin(60 deg - theta'), $T_{2} = sqrt$(3) Ts Vref/Vdc sin(theta'), $T_{0} = Ts - T_{1} - T_{2}$, theta' = angle within the sector
- **Assumes:** inside the hexagon $(T_{1} + T_{2}  \leq  Ts)$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![T1 (dwell time of the first active vector) against theta (reference vector angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-space-vector-times-curve)

*T1 (dwell time of the first active vector) against theta (reference vector angle) from 0 rad to 1.5708 rad. One curve for each value of fs (switching (sampling) frequency): 10000 Hz, 5000 Hz, 20000 Hz. Other inputs as in the worked example: Vref = 200.0 V, Vdc = 600.0 V. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, theta = 0.436332 rad, T1 = 3.3115e-05 s.*

```python
power_electronics_ai999x.solve('pe_space_vector_times', Vref=200.0, theta=0.4363323129985824, Vdc=600.0, fs=10000.0)
# -> sector = 1, T1 = 3.31155e-05 s, T2 = 2.43999e-05 s, T0 = 4.24847e-05 s, m = 0.57735   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_space_vector_times', Vref=100.0, theta=3.490658503988659, Vdc=400.0, fs=5000.0)
# -> sector = 4, T1 = 5.5667e-05 s, T2 = 2.96198e-05 s, T0 = 0.000114713 s, m = 0.433013   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Space vector dwell times: Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz | SOLVED | sector = 1, T1 = 3.31155e-05 s, T2 = 2.43999e-05 s, T0 = 4.24847e-05 s, m = 0.57735 |
| Space vector dwell times: Vref = 100 V, theta = 200 deg, Vdc = 400 V, fs = 5 kHz | SOLVED | sector = 4, T1 = 5.5667e-05 s, T2 = 2.96198e-05 s, T0 = 0.000114713 s, m = 0.433013 |
| Space vector dwell times: theta = 25 deg, Vdc = 600 V, fs = 10 kHz | NEEDS_INPUT | pe_space_vector_times needs Vref (reference vector magnitude (peak phase voltage)). Example: solve('pe_space_vector_time |
| Space vector dwell times: Vref = 200 V, theta = 25 deg, Vdc = 600 V, fs = 10 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dead_time_error"></a>

### Inverter dead-time voltage error: `pe_dead_time_error`

The average voltage error per switching period caused by the dead time of an inverter leg, the peak of its fundamental (the error is a square wave in phase with the current) and that as a percentage of the wanted fundamental.

$$
\Delta V = t_d f_s V_{dc},\quad \Delta\hat V_1 = \frac{4}{\pi}t_d f_s V_{dc}
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc-link voltage |  |
| `td` | dead time |  |
| `fs` | switching frequency |  |
| `V1` | wanted fundamental peak voltage |  |

| output | meaning |
|---|---|
| `dV_avg` | average voltage error td fs Vdc |
| `dV1_peak` | peak fundamental of the error |
| `dV1_pct` | error in percent of the wanted fundamental |

- **Method:** $dV = td$ fs Vdc; its fundamental 4 dV/pi
- **Assumes:** td $fs < 0.5$; the current does not change sign within a switching period
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dV_avg (average voltage error td fs Vdc) against Vdc (dc-link voltage) from 300 V to 900 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dead-time-error-curve)

*dV_avg (average voltage error td fs Vdc) against Vdc (dc-link voltage) from 300 V to 900 V. One curve for each value of td (dead time): 2.0000e-06 s, 1.0000e-06 s, 4.0000e-06 s. Other inputs as in the worked example: fs = 10000.0 Hz, V1 = 250.0 V. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Vdc = 600 V, dV_avg = 12 V.*

```python
power_electronics_ai999x.solve('pe_dead_time_error', Vdc=600.0, td=2e-06, fs=10000.0, V1=250.0)
# -> dV_avg = 12 V, dV1_peak = 15.2789 V, dV1_pct = 6.11155   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dead_time_error', Vdc=48.0, td=2e-07, fs=20000.0, V1=20.0)
# -> dV_avg = 0.192 V, dV1_peak = 0.244462 V, dV1_pct = 1.22231   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Dead-time voltage error: Vdc = 600 V, td = 2000 ns, fs = 10 kHz, V1 = 250 V | SOLVED | dV_avg = 12 V, dV1_peak = 15.2789 V, dV1_pct = 6.11155 |
| Dead-time voltage error: Vdc = 48 V, td = 200 ns, fs = 20 kHz, V1 = 20 V | SOLVED | dV_avg = 0.192 V, dV1_peak = 0.244462 V, dV1_pct = 1.22231 |
| Dead-time voltage error: td = 2000 ns, fs = 10 kHz, V1 = 250 V | NEEDS_INPUT | pe_dead_time_error needs Vdc (dc-link voltage). Example: solve('pe_dead_time_error', Vdc='600 V', td='2e-06 s', fs='1000 |
| Dead-time voltage error: Vdc = 600 V, td = 2000 ns, fs = 10 kHz, V1 = 250 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_inverter_rl_load_current"></a>

### Square-wave inverter with an RL load: current, `pe_inverter_rl_load_current`

The steady-state peak current, RMS current, load power and average dc source current of a full-bridge square-wave inverter driving a series RL load (exact exponential current, not a harmonic approximation).

$$
I_{max} = \frac{V_{dc}}{R}\,\frac{1-e^{-T/2\tau}}{1+e^{-T/2\tau}},\quad \tau = \frac{L}{R}
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc input voltage |  |
| `f` | output frequency |  |
| `R` | load resistance |  |
| `L` | load inductance |  |

| output | meaning |
|---|---|
| `I_max` | peak load current |
| `I_rms` | RMS load current |
| `P` | load power |
| `Is_avg` | average dc source current |

- **Method:** I_max = (Vdc/R)(1 - e^(-T/2tau))/(1 + e^(-T/2tau)); i(t) = Vdc/R + (-I_max - Vdc/R) $e^{-t/\tau}$ over a half-cycle; P = I_rms^2 R
- **Assumes:** steady state, ideal switches with antiparallel diodes
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![I_max (peak load current) against f (output frequency) from 6 Hz to 600 Hz on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-inverter-rl-load-current-curve)

*I_max (peak load current) against f (output frequency) from 6 Hz to 600 Hz on a logarithmic axis. One curve for each value of Vdc (dc input voltage): 100 V, 50 V, 200 V. Other inputs as in the worked example: R = 10.0 ohm, L = 0.025 H. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 60 Hz, I_max = 9.3111 A.*

```python
power_electronics_ai999x.solve('pe_inverter_rl_load_current', Vdc=100.0, f=60.0, R=10.0, L=0.025)
# -> I_max = 9.3111 A, I_rms = 6.6433 A, P = 441.334 W, Is_avg = 4.41334 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_inverter_rl_load_current', Vdc=48.0, f=400.0, R=4.0, L=0.002)
# -> I_max = 10.1794 A, I_rms = 6.80277 A, P = 185.111 W, Is_avg = 3.85648 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Inverter RL load current: Vdc = 100 V, f = 60 Hz, R = 10 ohm, L = 25 mH | SOLVED | I_max = 9.3111 A, I_rms = 6.6433 A, P = 441.334 W, Is_avg = 4.41334 A |
| Inverter RL load current: Vdc = 48 V, f = 400 Hz, R = 4 ohm, L = 2 mH | SOLVED | I_max = 10.1794 A, I_rms = 6.80277 A, P = 185.111 W, Is_avg = 3.85648 A |
| Inverter RL load current: f = 60 Hz, R = 10 ohm, L = 25 mH | NEEDS_INPUT | pe_inverter_rl_load_current needs Vdc (dc input voltage). Example: solve('pe_inverter_rl_load_current', Vdc='100 V', f=' |
| Inverter RL load current: Vdc = 100 V, f = 60 Hz, R = 10 ohm, L = 25 mH, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_igbt_inverter_conduction_losses"></a>

### IGBT and diode conduction losses in a sinusoidal PWM inverter, `pe_igbt_inverter_conduction_losses`

The conduction losses of one IGBT and one freewheeling diode (and of a whole leg) in a sinusoidal PWM inverter, from the peak phase current, modulation index, power-factor angle and the devices' threshold-voltage plus slope-resistance models.

$$
P_T = V_{ce0}\hat I\left(\frac{1}{2\pi}+\frac{M\cos\phi}{8}\right) + r_{ce}\hat I^2\left(\frac18+\frac{M\cos\phi}{3\pi}\right)
$$

| input | meaning | condition |
|---|---|---|
| `Ip` | peak phase current |  |
| `M` | modulation index |  |
| `phi` | power-factor angle (current lagging positive) |  |
| `Vce0` | IGBT threshold voltage |  |
| `rce` | IGBT slope resistance |  |
| `VF0` | diode threshold voltage |  |
| `rF` | diode slope resistance |  |

| output | meaning |
|---|---|
| `P_igbt` | conduction loss of one IGBT |
| `P_diode` | conduction loss of one diode |
| `P_switch_pair` | one IGBT plus one diode |
| `P_leg` | conduction loss of a leg (two of each) |

- **Method:** P_T = Vce0 Ip (1/(2pi) + M cos(phi)/8) + rce $Ip^{2}$ (1/8 + M cos(phi)/(3pi)); $P_D = VF0$ Ip (1/(2pi) - M cos(phi)/8) + rF $Ip^{2}$ (1/8 - M cos(phi)/(3pi))
- **Assumes:** $0 < M  \leq  1$, sinusoidal current, switching frequency far above the output frequency
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P_igbt (conduction loss of one IGBT) against Ip (peak phase current) from 10 A to 1000 A on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-igbt-inverter-conduction-losses-curve)

*P_igbt (conduction loss of one IGBT) against Ip (peak phase current) from 10 A to 1000 A on a logarithmic axis. Outputs with the same unit drawn together: P_diode (conduction loss of one diode), P_switch_pair (one IGBT plus one diode), P_leg (conduction loss of a leg (two of each)). Other inputs as in the worked example: M = 0.9, phi = 0.5235987755982988 rad, Vce0 = 0.9 V, rce = 0.01 ohm, VF0 = 0.8 V, rF = 0.008 ohm. 41 points computed by power-electronics-ai999x; marked: the worked example, Ip = 100 A, P_igbt = 43.8624 W.*

```python
power_electronics_ai999x.solve('pe_igbt_inverter_conduction_losses', Ip=100.0, M=0.9, phi=0.5235987755982988, Vce0=0.9, rce=0.01, VF0=0.8, rF=0.008)
# -> P_igbt = 43.8624 W, P_diode = 8.32222 W, P_switch_pair = 52.1846 W, P_leg = 104.369 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_igbt_inverter_conduction_losses', Ip=300.0, M=0.7, phi=0.0, Vce0=1.0, rce=0.004, VF0=0.9, rF=0.003)
# -> P_igbt = 145.735 W, P_diode = 33.0433 W, P_switch_pair = 178.778 W, P_leg = 357.556 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| IGBT inverter conduction losses: Ip = 100 A, M = 0.9, phi = 30 deg, Vce0 = 0.9 V, rce = 0.01 ohm, VF0 = 0.8 V, rF = 0.008 ohm | SOLVED | P_igbt = 43.8624 W, P_diode = 8.32222 W, P_switch_pair = 52.1846 W, P_leg = 104.369 W |
| IGBT inverter conduction losses: Ip = 300 A, M = 0.7, phi = 0 deg, Vce0 = 1 V, rce = 0.004 ohm, VF0 = 0.9 V, rF = 0.003 ohm | SOLVED | P_igbt = 145.735 W, P_diode = 33.0433 W, P_switch_pair = 178.778 W, P_leg = 357.556 W |
| IGBT inverter conduction losses: M = 0.9, phi = 30 deg, Vce0 = 0.9 V, rce = 0.01 ohm, VF0 = 0.8 V, rF = 0.008 ohm | NEEDS_INPUT | pe_igbt_inverter_conduction_losses needs Ip (peak phase current). Example: solve('pe_igbt_inverter_conduction_losses', I |
| IGBT inverter conduction losses: Ip = 100 A, M = 0.9, phi = 30 deg, Vce0 = 0.9 V, rce = 0.01 ohm, VF0 = 0.8 V, rF = 0.008 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dc_motor_chopper_drive"></a>

### Chopper-fed separately excited DC motor: speed and torque, `pe_dc_motor_chopper_drive`

The armature current, average armature voltage, back EMF, speed (rad/s and rpm), output power and armature efficiency of a separately excited DC motor fed by a step-down chopper at a duty cycle and carrying a load torque (continuous current, ripple neglected).

$$
V_a = DV_s,\quad I_a = \frac{T_L}{K},\quad \omega = \frac{V_a - I_aR_a}{K}
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | dc supply voltage |  |
| `D` | chopper duty cycle (a fraction or %) |  |
| `Ra` | armature resistance |  |
| `K` | motor constant $K = Ke = Kt$ |  |
| `TL` | load torque |  |

| output | meaning |
|---|---|
| `Ia` | armature current TL/K |
| `Va` | average armature voltage D Vs |
| `Ea` | back EMF |
| `omega` | speed |
| `rpm` | speed in rpm |
| `P_out` | developed (output) power |
| `efficiency` | armature efficiency Ea/Va |

- **Method:** $Va = D$ Vs; $Ia = TL/K$; $Ea = Va - Ia$ Ra; $w = Ea/K$
- **Assumes:** continuous armature current; constant field
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![omega (speed) against K (motor constant K = Ke = Kt) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dc-motor-chopper-drive-curve)

*omega (speed) against K (motor constant K = Ke = Kt) from 0.12 V*s/rad to 12 V*s/rad on a logarithmic axis. One curve for each value of Vs (dc supply voltage): 220 V, 110 V, 440 V. Other inputs as in the worked example: D = 0.6, Ra = 0.5 ohm, TL = 24.0 N*m. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, K = 1.2 V*s/rad, omega = 101.667 rad/s.*

```python
power_electronics_ai999x.solve('pe_dc_motor_chopper_drive', Vs=220.0, D=0.6, Ra=0.5, K=1.2, TL=24.0)
# -> Ia = 20 A, Va = 132 V, Ea = 122 V, omega = 101.667 rad/s, rpm = 970.845, P_out = 2440 W, efficiency = 0.924242   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dc_motor_chopper_drive', Vs=48.0, D=0.8, Ra=0.2, K=0.15, TL=1.5)
# -> Ia = 10 A, Va = 38.4 V, Ea = 36.4 V, omega = 242.667 rad/s, rpm = 2317.3, P_out = 364 W, efficiency = 0.947917   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| DC motor chopper drive: Vs = 220 V, D = 0.6, Ra = 0.5 ohm, K = 1.2 V*s/rad, TL = 24 N*m | SOLVED | Ia = 20 A, Va = 132 V, Ea = 122 V, omega = 101.667 rad/s, rpm = 970.845, P_out = 2440 W, efficiency = 0.924242 |
| DC motor chopper drive: Vs = 48 V, D = 0.8, Ra = 0.2 ohm, K = 0.15 V*s/rad, TL = 1.5 N*m | SOLVED | Ia = 10 A, Va = 38.4 V, Ea = 36.4 V, omega = 242.667 rad/s, rpm = 2317.3, P_out = 364 W, efficiency = 0.947917 |
| DC motor chopper drive: D = 0.6, Ra = 0.5 ohm, K = 1.2 V*s/rad, TL = 24 N*m | NEEDS_INPUT | pe_dc_motor_chopper_drive needs Vs (dc supply voltage). Example: solve('pe_dc_motor_chopper_drive', Vs='220 V', D=0.6, R |
| DC motor chopper drive: Vs = 220 V, D = 0.6, Ra = 0.5 ohm, K = 1.2 V*s/rad, TL = 24 N*m, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_bldc_six_step"></a>

### BLDC motor with six-step (120 degree) drive, `pe_bldc_six_step`

The speed, torque, no-load speed, stall current, electrical frequency and commutation interval of a trapezoidal brushless DC motor driven six-step from a PWM-chopped dc bus (two phases conduct in series).

$$
DV_{dc} = 2R_{ph}I + 2K_{e,ph}\omega,\quad T = 2K_{e,ph}I
$$

| input | meaning | condition |
|---|---|---|
| `Vdc` | dc bus voltage |  |
| `D` | PWM duty cycle (a fraction or %) |  |
| `Rph` | phase resistance |  |
| `Keph` | phase back-EMF constant (flat top) |  |
| `I` | dc link (phase) current |  |
| `p` | pole pairs |  |

| output | meaning |
|---|---|
| `omega` | speed |
| `rpm` | speed in rpm |
| `T` | torque |
| `omega_no_load` | no-load speed |
| `I_stall` | stall current |
| `f_e` | electrical frequency |
| `t_commutation` | time between commutations |

- **Method:** D Vdc = 2 Rph $I + 2$ Keph w; $T = 2$ Keph I; $f_e = p$ w/(2 pi)
- **Assumes:** ideal trapezoidal EMF and block currents; winding inductance and commutation neglected
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![omega (speed) against Keph (phase back-EMF constant (flat top)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-bldc-six-step-curve)

*omega (speed) against Keph (phase back-EMF constant (flat top)) from 0.025 V*s/rad to 0.075 V*s/rad. One curve for each value of Vdc (dc bus voltage): 48 V, 24 V, 96 V. Other inputs as in the worked example: D = 0.8, Rph = 0.1 ohm, I = 10.0 A, p = 4. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Keph = 0.05 V*s/rad, omega = 364 rad/s.*

```python
power_electronics_ai999x.solve('pe_bldc_six_step', Vdc=48.0, D=0.8, Rph=0.1, Keph=0.05, I=10.0, p=4)
# -> omega = 364 rad/s, rpm = 3475.94, T = 1 N*m, omega_no_load = 384 rad/s, I_stall = 192 A, f_e = 231.73 Hz, t_commutation = 0.000719229 s   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_bldc_six_step', Vdc=24.0, D=0.5, Rph=0.4, Keph=0.02, I=2.0, p=2)
# -> omega = 260 rad/s, rpm = 2482.82, T = 0.08 N*m, omega_no_load = 300 rad/s, I_stall = 15 A, f_e = 82.7606 Hz, t_commutation = 0.00201384 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| BLDC six-step drive: Vdc = 48 V, D = 0.8, Rph = 0.1 ohm, Keph = 0.05 V*s/rad, I = 10 A, p = 4 | SOLVED | omega = 364 rad/s, rpm = 3475.94, T = 1 N*m, omega_no_load = 384 rad/s, I_stall = 192 A, f_e = 231.73 Hz, t_commutation = 0.000719229 s |
| BLDC six-step drive: Vdc = 24 V, D = 0.5, Rph = 0.4 ohm, Keph = 0.02 V*s/rad, I = 2 A, p = 2 | SOLVED | omega = 260 rad/s, rpm = 2482.82, T = 0.08 N*m, omega_no_load = 300 rad/s, I_stall = 15 A, f_e = 82.7606 Hz, t_commutation = 0.00201384 s |
| BLDC six-step drive: D = 0.8, Rph = 0.1 ohm, Keph = 0.05 V*s/rad, I = 10 A, p = 4 | NEEDS_INPUT | pe_bldc_six_step needs Vdc (dc bus voltage). Example: solve('pe_bldc_six_step', Vdc='48 V', D=0.8, Rph='0.1 ohm', Keph=' |
| BLDC six-step drive: Vdc = 48 V, D = 0.8, Rph = 0.1 ohm, Keph = 0.05 V*s/rad, I = 10 A, p = 4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dc_drive_controlled_rectifier"></a>

### Phase-controlled rectifier DC motor drive: `pe_dc_drive_controlled_rectifier`

The armature voltage, back EMF, speed, torque and developed power of a separately excited DC motor fed by a single-phase or three-phase fully controlled thyristor bridge at a firing angle, with continuous armature current.

$$
V_a = \frac{2\sqrt2V_s}{\pi}\cos\alpha\ (1\phi),\ \frac{3\sqrt2V_{LL}}{\pi}\cos\alpha\ (3\phi),\quad \omega = \frac{V_a - I_aR_a}{K}
$$

| input | meaning | condition |
|---|---|---|
| `converter` | single-phase or three-phase full converter |  |
| `Vs` | RMS supply voltage (line-to-line for three-phase) |  |
| `alpha` | firing angle |  |
| `Ra` | armature resistance |  |
| `K` | motor constant |  |
| `Ia` | armature current |  |

| output | meaning |
|---|---|
| `Va` | average armature voltage |
| `Ea` | back EMF |
| `omega` | speed |
| `rpm` | speed in rpm |
| `T` | developed torque K Ia |
| `P` | developed power Ea Ia |
| `mode` | motoring or reverse |

- **Method:** $Va = 2$ sqrt(2) Vs cos(alpha)/pi (single-phase) or 3 sqrt(2) VLL cos(alpha)/pi (three-phase); $Ea = Va - Ia$ Ra; $w = Ea/K$
- **Assumes:** continuous armature current
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![motoring or reverse (mode) over alpha (firing angle) from 0 rad to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dc-drive-controlled-rectifier-decision)

*motoring or reverse (mode) over alpha (firing angle) from 0 rad to 1.5708 rad and Vs (RMS supply voltage (line-to-line for three-phase)) from 23 V to 2300 V (logarithmic vertical axis): each of the 31 × 21 cells is one run of power-electronics-ai999x, coloured by the verdict it returned, motoring: 595 cells, reverse (regenerating against an active load): 56 cells. Other inputs as in the worked example: converter = single-phase, Ra = 1.0 ohm, K = 1.0 V*s/rad, Ia = 10.0 A. Marked: the worked example, alpha = 0.785398 rad, Vs = 230 V → motoring.*

![omega (speed) against K (motor constant) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dc-drive-controlled-rectifier-curve)

*omega (speed) against K (motor constant) from 0.5 V*s/rad to 1.5 V*s/rad. One curve for each value of Vs (RMS supply voltage (line-to-line for three-phase)): 230 V, 115 V, 460 V. Other inputs as in the worked example: converter = single-phase, alpha = 0.7853981633974483 rad, Ra = 1.0 ohm, Ia = 10.0 A. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, K = 1 V*s/rad, omega = 136.423 rad/s.*

```python
power_electronics_ai999x.solve('pe_dc_drive_controlled_rectifier', converter='single-phase', Vs=230.0, alpha=0.7853981633974483, Ra=1.0, K=1.0, Ia=10.0)
# -> Va = 146.423 V, Ea = 136.423 V, omega = 136.423 rad/s, rpm = 1302.74, T = 10 N*m, P = 1364.23 W, mode = motoring   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dc_drive_controlled_rectifier', converter='three-phase', Vs=415.0, alpha=0.5235987755982988, Ra=0.2, K=2.0, Ia=50.0)
# -> Va = 485.361 V, Ea = 475.361 V, omega = 237.681 rad/s, rpm = 2269.68, T = 100 N*m, P = 23768.1 W, mode = motoring   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Controlled rectifier DC drive: converter = single-phase, Vs = 230 V, alpha = 45 deg, Ra = 1 ohm, K = 1 V*s/rad, Ia = 10 A | SOLVED | Va = 146.423 V, Ea = 136.423 V, omega = 136.423 rad/s, rpm = 1302.74, T = 10 N*m, P = 1364.23 W, mode = motoring |
| Controlled rectifier DC drive: converter = three-phase, Vs = 415 V, alpha = 30 deg, Ra = 0.2 ohm, K = 2 V*s/rad, Ia = 50 A | SOLVED | Va = 485.361 V, Ea = 475.361 V, omega = 237.681 rad/s, rpm = 2269.68, T = 100 N*m, P = 23768.1 W, mode = motoring |
| Controlled rectifier DC drive: Vs = 230 V, alpha = 45 deg, Ra = 1 ohm, K = 1 V*s/rad, Ia = 10 A | NEEDS_INPUT | pe_dc_drive_controlled_rectifier needs converter (single-phase or three-phase full converter). Example: solve('pe_dc_dri |
| Controlled rectifier DC drive: converter = single-phase, Vs = 230 V, alpha = 45 deg, Ra = 1 ohm, K = 1 V*s/rad, Ia = 10 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_regenerative_braking_chopper"></a>

### Regenerative braking of a DC motor with a chopper, `pe_regenerative_braking_chopper`

The back EMF, average armature voltage, armature current, braking torque, mechanical power absorbed, power returned to the supply and braking efficiency of a DC motor braked regeneratively by a step-up chopper.

$$
I_a = \frac{K\omega - (1-D)V_s}{R_a},\quad P_{regen} = (1-D)V_sI_a
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | dc supply voltage |  |
| `D` | chopper duty cycle (a fraction or %) |  |
| `Ra` | armature resistance |  |
| `K` | motor constant |  |
| `omega` | motor speed |  |

| output | meaning |
|---|---|
| `Ea` | back EMF K w |
| `Va` | average armature voltage |
| `Ia` | armature (braking) current |
| `T_brake` | braking torque |
| `P_mech` | mechanical power absorbed Ea Ia |
| `P_regen` | power returned to the supply |
| `efficiency` | braking efficiency P_regen/P_mech |

- **Method:** $Ea = K$ w; $Va = (1 - D)$ Vs; $Ia = (Ea - Va)/Ra$; P_regen = Va Ia
- **Assumes:** $Ea > (1 - D)$ Vs; continuous armature current
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ia (armature (braking) current) against Ra (armature resistance) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-regenerative-braking-chopper-curve)

*Ia (armature (braking) current) against Ra (armature resistance) from 0.25 ohm to 0.75 ohm. One curve for each value of K (motor constant): 1 V*s/rad, 2 V*s/rad. Other inputs as in the worked example: Vs = 220.0 V, D = 0.5, omega = 150.0 rad/s. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Ra = 0.5 ohm, Ia = 80 A.*

```python
power_electronics_ai999x.solve('pe_regenerative_braking_chopper', Vs=220.0, D=0.5, Ra=0.5, K=1.0, omega=150.0)
# -> Ea = 150 V, Va = 110 V, Ia = 80 A, T_brake = 80 N*m, P_mech = 12000 W, P_regen = 8800 W, efficiency = 0.733333   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_regenerative_braking_chopper', Vs=600.0, D=0.3, Ra=0.1, K=2.5, omega=200.0)
# -> Ea = 500 V, Va = 420 V, Ia = 800 A, T_brake = 2000 N*m, P_mech = 400000 W, P_regen = 336000 W, efficiency = 0.84   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Regenerative braking chopper: Vs = 220 V, D = 0.5, Ra = 0.5 ohm, K = 1 V*s/rad, omega = 150 rad/s | SOLVED | Ea = 150 V, Va = 110 V, Ia = 80 A, T_brake = 80 N*m, P_mech = 12000 W, P_regen = 8800 W, efficiency = 0.733333 |
| Regenerative braking chopper: Vs = 600 V, D = 0.3, Ra = 0.1 ohm, K = 2.5 V*s/rad, omega = 200 rad/s | SOLVED | Ea = 500 V, Va = 420 V, Ia = 800 A, T_brake = 2000 N*m, P_mech = 400000 W, P_regen = 336000 W, efficiency = 0.84 |
| Regenerative braking chopper: D = 0.5, Ra = 0.5 ohm, K = 1 V*s/rad, omega = 150 rad/s | NEEDS_INPUT | pe_regenerative_braking_chopper needs Vs (dc supply voltage). Example: solve('pe_regenerative_braking_chopper', Vs='220  |
| Regenerative braking chopper: Vs = 220 V, D = 0.5, Ra = 0.5 ohm, K = 1 V*s/rad, omega = 150 rad/s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dc_motor_armature_ripple"></a>

### Armature current ripple of a chopper-fed DC motor, `pe_dc_motor_armature_ripple`

The maximum, minimum, peak-to-peak and average armature current of a DC motor fed by a step-down chopper, from the exact exponential (RL plus back-EMF) waveform in continuous conduction.

$$
I_{max} = \frac{V_s}{R_a}\frac{1-e^{-DT/\tau}}{1-e^{-T/\tau}} - \frac{E_a}{R_a},\ I_{min} = \frac{V_s}{R_a}\frac{e^{DT/\tau}-1}{e^{T/\tau}-1} - \frac{E_a}{R_a}
$$

| input | meaning | condition |
|---|---|---|
| `Vs` | dc supply voltage |  |
| `D` | duty cycle (a fraction or %) |  |
| `fs` | chopping frequency |  |
| `Ra` | armature resistance |  |
| `La` | armature inductance |  |
| `Ea` | back EMF |  |

| output | meaning |
|---|---|
| `I_max` | maximum armature current |
| `I_min` | minimum armature current |
| `dI` | peak-to-peak ripple |
| `I_avg` | average armature current |

- **Method:** I_max = (Vs/Ra)(1 - e^(-DT/tau))/(1 - e^(-T/tau)) - Ea/Ra; I_min = (Vs/Ra)$(e^{DT/\tau} - 1)/(e^{T/\tau} - 1) - Ea/Ra$; I_avg = (D Vs - Ea)/Ra
- **Assumes:** continuous conduction (I_min > 0); constant speed (constant back EMF) over a period
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dI (peak-to-peak ripple) against D (duty cycle (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dc-motor-armature-ripple-curve)

*dI (peak-to-peak ripple) against D (duty cycle (a fraction or %)) from 0.25 to 0.75. One curve for each value of Vs (dc supply voltage): 220 V, 440 V. Other inputs as in the worked example: fs = 1000.0 Hz, Ra = 1.0 ohm, La = 0.01 H, Ea = 100.0 V. 17 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.5, dI = 5.49885 A.*

```python
power_electronics_ai999x.solve('pe_dc_motor_armature_ripple', Vs=220.0, D=0.5, fs=1000.0, Ra=1.0, La=0.01, Ea=100.0)
# -> I_max = 12.7494 A, I_min = 7.25057 A, dI = 5.49885 A, I_avg = 10 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dc_motor_armature_ripple', Vs=100.0, D=0.7, fs=500.0, Ra=0.5, La=0.005, Ea=60.0)
# -> I_max = 24.1411 A, I_min = 15.747 A, dI = 8.39413 A, I_avg = 20 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Armature current ripple: Vs = 220 V, D = 0.5, fs = 1 kHz, Ra = 1 ohm, La = 10 mH, Ea = 100 V | SOLVED | I_max = 12.7494 A, I_min = 7.25057 A, dI = 5.49885 A, I_avg = 10 A |
| Armature current ripple: Vs = 100 V, D = 0.7, fs = 0.5 kHz, Ra = 0.5 ohm, La = 5 mH, Ea = 60 V | SOLVED | I_max = 24.1411 A, I_min = 15.747 A, dI = 8.39413 A, I_avg = 20 A |
| Armature current ripple: D = 0.5, fs = 1 kHz, Ra = 1 ohm, La = 10 mH, Ea = 100 V | NEEDS_INPUT | pe_dc_motor_armature_ripple needs Vs (dc supply voltage). Example: solve('pe_dc_motor_armature_ripple', Vs='220 V', D=0. |
| Armature current ripple: Vs = 220 V, D = 0.5, fs = 1 kHz, Ra = 1 ohm, La = 10 mH, Ea = 100 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_induction_motor_vf"></a>

### Induction motor constant V/f speed control, `pe_induction_motor_vf`

The supply voltage, synchronous speed, rotor speed, slip and slip frequency of an induction motor under constant V/f control below base frequency, from its rated voltage, frequency and speed (linear torque-slip region, torque as a fraction of rated).

$$
\frac{V}{f} = \text{const},\quad n = \frac{120f}{P} - \left(n_{s,r} - n_r\right)\frac{T}{T_r}
$$

| input | meaning | condition |
|---|---|---|
| `V_rated` | rated line voltage |  |
| `f_rated` | rated (base) frequency |  |
| `poles` | number of poles |  |
| `n_rated` | rated speed in rpm |  |
| `f` | operating frequency |  |
| `T_ratio` | load torque as a fraction of rated torque (or %) |  |

| output | meaning |
|---|---|
| `V` | supply voltage at f (constant V/f) |
| `ns` | synchronous speed in rpm |
| `n` | rotor speed in rpm |
| `slip` | slip |
| `f_slip` | slip (rotor) frequency |

- **Method:** V = V_rated f/f_rated; $ns = 120$ f/P; the slip speed at constant V/f is proportional to torque: n = ns - (ns,rated - n_rated) T/T_rated
- **Assumes:** f <= f_rated; linear torque-slip region (T_ratio <= 1.5); stator resistance drop neglected
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![n (rotor speed in rpm) against f (operating frequency) from 15 Hz to 45 Hz](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-induction-motor-vf-curve)

*n (rotor speed in rpm) against f (operating frequency) from 15 Hz to 45 Hz. Other inputs as in the worked example: V_rated = 400.0 V, f_rated = 50.0 Hz, poles = 4, n_rated = 1450.0, T_ratio = 1.0. 31 points computed by power-electronics-ai999x; marked: the worked example, f = 30 Hz, n = 850.*

```python
power_electronics_ai999x.solve('pe_induction_motor_vf', V_rated=400.0, f_rated=50.0, poles=4, n_rated=1450.0, f=30.0, T_ratio=1.0)
# -> V = 240 V, ns = 900, n = 850, slip = 0.0555556, f_slip = 1.66667 Hz   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_induction_motor_vf', V_rated=460.0, f_rated=60.0, poles=6, n_rated=1170.0, f=40.0, T_ratio=0.5)
# -> V = 306.667 V, ns = 800, n = 785, slip = 0.01875, f_slip = 0.75 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Induction motor V/f control: V_rated = 400 V, f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 1 | SOLVED | V = 240 V, ns = 900, n = 850, slip = 0.0555556, f_slip = 1.66667 Hz |
| Induction motor V/f control: V_rated = 460 V, f_rated = 60 Hz, poles = 6, n_rated = 1170, f = 40 Hz, T_ratio = 0.5 | SOLVED | V = 306.667 V, ns = 800, n = 785, slip = 0.01875, f_slip = 0.75 Hz |
| Induction motor V/f control: f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 1 | NEEDS_INPUT | pe_induction_motor_vf needs V_rated (rated line voltage). Example: solve('pe_induction_motor_vf', V_rated='400 V', f_rat |
| Induction motor V/f control: V_rated = 400 V, f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 1, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Induction motor V/f control: V_rated = 400 V, f_rated = 50 Hz, poles = 4, n_rated = 1450, f = 30 Hz, T_ratio = 100% | SOLVED | V = 240 V, ns = 900, n = 850, slip = 0.0555556, f_slip = 1.66667 Hz |

<a id="pe_pmsm_dq_model"></a>

### PMSM steady state in the dq frame: voltages and torque, `pe_pmsm_dq_model`

The electrical speed, d- and q-axis voltages, voltage magnitude, electromagnetic torque, mechanical, input and copper power and power factor of a permanent-magnet synchronous motor in steady state from its dq currents (amplitude-invariant Park transform).

$$
v_d = R_si_d - \omega_eL_qi_q,\ v_q = R_si_q + \omega_e(L_di_d+\psi_f),\ T_e = \tfrac32p\left(\psi_fi_q + (L_d-L_q)i_di_q\right)
$$

| input | meaning | condition |
|---|---|---|
| `Rs` | stator resistance per phase |  |
| `Ld` | d-axis inductance |  |
| `Lq` | q-axis inductance |  |
| `psi_f` | permanent-magnet flux linkage |  |
| `p` | pole pairs |  |
| `id` | d-axis current (negative for field weakening) |  |
| `iq` | q-axis current |  |
| `omega_m` | mechanical speed |  |

| output | meaning |
|---|---|
| `omega_e` | electrical speed p w_m |
| `vd` | d-axis voltage |
| `vq` | q-axis voltage |
| `V_peak` | peak phase voltage magnitude |
| `Te` | electromagnetic torque |
| `P_mech` | mechanical power |
| `P_in` | electrical input power |
| `P_cu` | stator copper loss |
| `PF` | power factor |

- **Method:** $vd = Rs$ id - omega_e Lq iq; $vq = Rs$ iq + omega_e (Ld id + psi_f); $Te = 1.5$ p (psi_f iq + (Ld - Lq) id iq); $P_in = 1.5$ (vd id + vq iq)
- **Assumes:** steady state, sinusoidal machine, iron loss neglected; amplitude-invariant transform
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Te (electromagnetic torque) against Lq (q-axis inductance) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-pmsm-dq-model-curve)

*Te (electromagnetic torque) against Lq (q-axis inductance) from 0.0005 H to 0.0015 H. One curve for each value of iq (q-axis current): 60 A, 30 A, 120 A. Other inputs as in the worked example: Rs = 0.05 ohm, Ld = 0.0004 H, psi_f = 0.08 Wb, p = 4, id = -20.0 A, omega_m = 300.0 rad/s. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Lq = 0.001 H, Te = 33.12 N*m.*

```python
power_electronics_ai999x.solve('pe_pmsm_dq_model', Rs=0.05, Ld=0.0004, Lq=0.001, psi_f=0.08, p=4, id=-20.0, iq=60.0, omega_m=300.0)
# -> omega_e = 1200 rad/s, vd = -73 V, vq = 89.4 V, V_peak = 115.418 V, Te = 33.12 N*m, P_mech = 9936 W, P_in = 10236 W, P_cu = 300 W, PF = 0.934835   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_pmsm_dq_model', Rs=0.2, Ld=0.002, Lq=0.002, psi_f=0.1, p=2, id=0.0, iq=10.0, omega_m=150.0)
# -> omega_e = 300 rad/s, vd = -6 V, vq = 32 V, V_peak = 32.5576 V, Te = 3 N*m, P_mech = 450 W, P_in = 480 W, P_cu = 30 W, PF = 0.982872   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PMSM dq model: Rs = 0.05 ohm, Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, id = -20 A, iq = 60 A, omega_m = 300 rad/s | SOLVED | omega_e = 1200 rad/s, vd = -73 V, vq = 89.4 V, V_peak = 115.418 V, Te = 33.12 N*m, P_mech = 9936 W, P_in = 10236 W, P_cu = 300 W, PF = 0.934835 |
| PMSM dq model: Rs = 0.2 ohm, Ld = 2 mH, Lq = 2 mH, psi_f = 100 mWb, p = 2, id = 0 A, iq = 10 A, omega_m = 150 rad/s | SOLVED | omega_e = 300 rad/s, vd = -6 V, vq = 32 V, V_peak = 32.5576 V, Te = 3 N*m, P_mech = 450 W, P_in = 480 W, P_cu = 30 W, PF = 0.982872 |
| PMSM dq model: Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, id = -20 A, iq = 60 A, omega_m = 300 rad/s | NEEDS_INPUT | pe_pmsm_dq_model needs Rs (stator resistance per phase). Example: solve('pe_pmsm_dq_model', Rs='0.05 ohm', Ld='0.0004 H' |
| PMSM dq model: Rs = 0.05 ohm, Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, id = -20 A, iq = 60 A, omega_m = 300 rad/s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_pmsm_mtpa"></a>

### Maximum torque per ampere (MTPA) of an interior PMSM, `pe_pmsm_mtpa`

The d- and q-axis currents, current advance angle and torque that give the most torque for a given current magnitude in a salient (interior) permanent-magnet synchronous motor, and the torque with $id = 0$ for comparison.

$$
i_d = \frac{\psi_f - \sqrt{\psi_f^2 + 8(L_q-L_d)^2 I_s^2}}{4(L_q-L_d)},\quad i_q = \sqrt{I_s^2 - i_d^2}
$$

| input | meaning | condition |
|---|---|---|
| `Ld` | d-axis inductance |  |
| `Lq` | q-axis inductance $(Lq > Ld)$ |  |
| `psi_f` | magnet flux linkage |  |
| `p` | pole pairs |  |
| `Is` | current magnitude (peak) |  |

| output | meaning |
|---|---|
| `id` | d-axis current (negative) |
| `iq` | q-axis current |
| `beta_deg` | current advance angle from the q axis, degrees |
| `Te` | maximum torque at this current |
| `Te_id0` | torque with zero d-axis current |

- **Method:** id = (psi_f - sqrt($psi_f^{2} + 8$ $(Lq - Ld)^{2}$ Is^2)) / (4 (Lq - Ld)); $iq = \sqrt{Is^{2} - id^{2}}$
- **Assumes:** $Lq > Ld$; magnetic saturation neglected
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![id (d-axis current (negative)) against Is (current magnitude (peak)) from 10 A to 1000 A on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-pmsm-mtpa-curve)

*id (d-axis current (negative)) against Is (current magnitude (peak)) from 10 A to 1000 A on a logarithmic axis. Outputs with the same unit drawn together: iq (q-axis current). Other inputs as in the worked example: Ld = 0.0004 H, Lq = 0.001 H, psi_f = 0.08 Wb, p = 4. 41 points computed by power-electronics-ai999x; marked: the worked example, Is = 100 A, id = -44.8403 A.*

```python
power_electronics_ai999x.solve('pe_pmsm_mtpa', Ld=0.0004, Lq=0.001, psi_f=0.08, p=4, Is=100.0)
# -> id = -44.8403 A, iq = 89.3832 A, beta_deg = 26.6412, Te = 57.3326 N*m, Te_id0 = 48 N*m   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_pmsm_mtpa', Ld=0.002, Lq=0.005, psi_f=0.1, p=3, Is=20.0)
# -> id = -8.08143 A, iq = 18.2945 A, beta_deg = 23.833, Te = 10.2285 N*m, Te_id0 = 9 N*m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PMSM MTPA: Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, Is = 100 A | SOLVED | id = -44.8403 A, iq = 89.3832 A, beta_deg = 26.6412, Te = 57.3326 N*m, Te_id0 = 48 N*m |
| PMSM MTPA: Ld = 2 mH, Lq = 5 mH, psi_f = 100 mWb, p = 3, Is = 20 A | SOLVED | id = -8.08143 A, iq = 18.2945 A, beta_deg = 23.833, Te = 10.2285 N*m, Te_id0 = 9 N*m |
| PMSM MTPA: Lq = 1 mH, psi_f = 80 mWb, p = 4, Is = 100 A | NEEDS_INPUT | pe_pmsm_mtpa needs Ld (d-axis inductance). Example: solve('pe_pmsm_mtpa', Ld='0.0004 H', Lq='0.001 H', psi_f='0.08 Wb',  |
| PMSM MTPA: Ld = 0.4 mH, Lq = 1 mH, psi_f = 80 mWb, p = 4, Is = 100 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_pmsm_field_weakening"></a>

### Surface PMSM field weakening: base speed, maximum speed and torque, `pe_pmsm_field_weakening`

The base speed, the maximum speed, and at a given speed the d- and q-axis currents, torque and power of a surface (non-salient) permanent-magnet motor at its current and voltage limits (field weakening above base speed; stator resistance neglected).

$$
\omega_b = \frac{V_{max}}{p\sqrt{\psi_f^2 + (L_sI_{max})^2}},\ \omega_{max} = \frac{V_{max}}{p(\psi_f - L_sI_{max})},\ i_d = \frac{(V_{max}/p\omega)^2 - \psi_f^2 - L_s^2I_{max}^2}{2\psi_fL_s}
$$

| input | meaning | condition |
|---|---|---|
| `Ls` | synchronous inductance $(Ld = Lq)$ |  |
| `psi_f` | magnet flux linkage |  |
| `p` | pole pairs |  |
| `Imax` | current limit (peak) |  |
| `Vmax` | voltage limit (peak phase) |  |
| `omega` | mechanical speed of interest |  |

| output | meaning |
|---|---|
| `omega_base` | base speed (mechanical) |
| `omega_max` | maximum speed (mechanical) |
| `rpm_base` | base speed in rpm |
| `id` | d-axis current at omega |
| `iq` | q-axis current at omega |
| `T` | available torque at omega |
| `P` | available power at omega |

- **Method:** w_b = Vmax/(p sqrt(psi_f^2 + (Ls Imax)^2)); w_max = Vmax/(p (psi_f - Ls Imax)); above w_b: id = ((Vmax/(p w))^$2 - psi_f^{2} - Ls^{2}$ Imax^2)/(2 psi_f Ls)
- **Assumes:** $psi_f > Ls$ Imax (finite maximum speed); omega < omega_max; $Rs = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![T (available torque at omega) against p (pole pairs) from 1 to 12](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-pmsm-field-weakening-curve)

*T (available torque at omega) against p (pole pairs) from 1 to 12. Other inputs as in the worked example: Ls = 0.001 H, psi_f = 0.1 Wb, Imax = 60.0 A, Vmax = 200.0 V, omega = 600.0 rad/s. 8 points computed by power-electronics-ai999x; marked: the worked example, p = 4, T = 29.9555 N*m.*

```python
power_electronics_ai999x.solve('pe_pmsm_field_weakening', Ls=0.001, psi_f=0.1, p=4, Imax=60.0, Vmax=200.0, omega=600.0)
# -> omega_base = 428.746 rad/s, omega_max = 1250 rad/s, rpm_base = 4094.23, id = -33.2778 A, iq = 49.9258 A, T = 29.9555 N*m, P = 17973.3 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_pmsm_field_weakening', Ls=0.0005, psi_f=0.05, p=2, Imax=50.0, Vmax=48.0, omega=150.0)
# -> omega_base = 429.325 rad/s, omega_max = 960 rad/s, rpm_base = 4099.75, id = 0 A, iq = 50 A, T = 7.5 N*m, P = 1125 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| PMSM field weakening: Ls = 1 mH, psi_f = 100 mWb, p = 4, Imax = 60 A, Vmax = 200 V, omega = 600 rad/s | SOLVED | omega_base = 428.746 rad/s, omega_max = 1250 rad/s, rpm_base = 4094.23, id = -33.2778 A, iq = 49.9258 A, T = 29.9555 N*m, P = 17973.3 W |
| PMSM field weakening: Ls = 0.5 mH, psi_f = 50 mWb, p = 2, Imax = 50 A, Vmax = 48 V, omega = 150 rad/s | SOLVED | omega_base = 429.325 rad/s, omega_max = 960 rad/s, rpm_base = 4099.75, id = 0 A, iq = 50 A, T = 7.5 N*m, P = 1125 W |
| PMSM field weakening: psi_f = 100 mWb, p = 4, Imax = 60 A, Vmax = 200 V, omega = 600 rad/s | NEEDS_INPUT | pe_pmsm_field_weakening needs Ls (synchronous inductance (Ld = Lq)). Example: solve('pe_pmsm_field_weakening', Ls='0.001 |
| PMSM field weakening: Ls = 1 mH, psi_f = 100 mWb, p = 4, Imax = 60 A, Vmax = 200 V, omega = 600 rad/s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_mosfet_conduction_loss"></a>

### MOSFET conduction loss at junction temperature, `pe_mosfet_conduction_loss`

The on-resistance of a MOSFET at its junction temperature (linear temperature coefficient from the 25 degC value) and the conduction loss for an RMS drain current, with the loss at 25 degC for comparison.

$$
R_{ds}(T_j) = R_{ds,25}\left(1+\alpha(T_j-25^\circ\mathrm{C})\right),\quad P = I_{rms}^2R_{ds}(T_j)
$$

| input | meaning | condition |
|---|---|---|
| `Rds25` | on-resistance at room temperature |  |
| `alpha` | temperature coefficient of the on-resistance per kelvin |  |
| `Tj` | junction temperature |  |
| `Irms` | RMS drain current |  |

| output | meaning |
|---|---|
| `Rds_T` | on-resistance at Tj |
| `P` | conduction loss at the junction temperature |
| `P_25` | conduction loss at room temperature |
| `ratio` | ratio of hot to room-temperature on-resistance |

- **Method:** Rds(Tj) = Rds25 (1 + alpha (Tj - 25 degC)); P = Irms^2 Rds(Tj)
- **Assumes:** linear temperature model
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P (conduction loss at the junction temperature) against Irms (RMS drain current) from 10 A to 30 A](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-mosfet-conduction-loss-curve)

*P (conduction loss at the junction temperature) against Irms (RMS drain current) from 10 A to 30 A. One curve for each value of alpha (temperature coefficient of the on-resistance per kelvin): 0.007, 0.26, 0.74. Other inputs as in the worked example: Rds25 = 0.01 ohm, Tj = 398.15 K. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Irms = 20 A, P = 6.8 W.*

```python
power_electronics_ai999x.solve('pe_mosfet_conduction_loss', Rds25=0.01, alpha=0.007, Tj=398.15, Irms=20.0)
# -> Rds_T = 0.017 ohm, P = 6.8 W, P_25 = 4 W, ratio = 1.7   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_mosfet_conduction_loss', Rds25=0.08, alpha=0.006, Tj=373.15, Irms=5.0)
# -> Rds_T = 0.116 ohm, P = 2.9 W, P_25 = 2 W, ratio = 1.45   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| MOSFET conduction loss: Rds25 = 0.01 ohm, alpha = 0.007, Tj = 125 degC, Irms = 20 A | SOLVED | Rds_T = 0.017 ohm, P = 6.8 W, P_25 = 4 W, ratio = 1.7 |
| MOSFET conduction loss: Rds25 = 0.08 ohm, alpha = 0.006, Tj = 100 degC, Irms = 5 A | SOLVED | Rds_T = 0.116 ohm, P = 2.9 W, P_25 = 2 W, ratio = 1.45 |
| MOSFET conduction loss: alpha = 0.007, Tj = 125 degC, Irms = 20 A | NEEDS_INPUT | pe_mosfet_conduction_loss needs Rds25 (on-resistance at room temperature). Example: solve('pe_mosfet_conduction_loss', R |
| MOSFET conduction loss: Rds25 = 0.01 ohm, alpha = 0.007, Tj = 125 degC, Irms = 20 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| MOSFET conduction loss: Rds25 = 10 mohm, alpha = 0.7%, Tj = 125 degC, Irms = 20 A | SOLVED | Rds_T = 0.017 ohm, P = 6.8 W, P_25 = 4 W, ratio = 1.7 |

<a id="pe_switching_loss"></a>

### Hard-switching loss of a power transistor: `pe_switching_loss`

The turn-on and turn-off energies and the switching and output-capacitance losses of a MOSFET or IGBT switching an inductive (clamped) load with linear voltage and current transitions.

$$
P_{sw} = \tfrac12 V I\,(t_r + t_f)\,f_s,\quad P_{oss} = \tfrac12C_{oss}V^2f_s
$$

| input | meaning | condition |
|---|---|---|
| `V` | blocking (dc bus) voltage |  |
| `I` | switched current |  |
| `tr` | turn-on crossover time |  |
| `tf` | turn-off crossover time |  |
| `fs` | switching frequency |  |
| `Coss` | output capacitance |  |

| output | meaning |
|---|---|
| `E_on` | turn-on energy |
| `E_off` | turn-off energy |
| `P_sw` | switching loss |
| `P_oss` | output-capacitance loss |
| `P_total` | total switching-related loss |

- **Method:** $E_on = V$ I $tr/2$; E_off = V I $tf/2$; P_sw = (E_on + E_off) fs; P_oss = Coss $V^{2}$ $fs/2$
- **Assumes:** clamped inductive load, linear transitions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P_sw (switching loss) against V (blocking (dc bus) voltage) from 200 V to 600 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-switching-loss-curve)

*P_sw (switching loss) against V (blocking (dc bus) voltage) from 200 V to 600 V. One curve for each value of I (switched current): 10 A, 5 A, 20 A. Other inputs as in the worked example: tr = 3.0000000000000004e-08 s, tf = 5.0000000000000004e-08 s, fs = 100000.0 Hz, Coss = 1e-10 F. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, V = 400 V, P_sw = 16 W.*

```python
power_electronics_ai999x.solve('pe_switching_loss', V=400.0, I=10.0, tr=3e-08, tf=5e-08, fs=100000.0, Coss=1e-10)
# -> E_on = 6e-05 J, E_off = 0.0001 J, P_sw = 16 W, P_oss = 0.8 W, P_total = 16.8 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_switching_loss', V=600.0, I=100.0, tr=1e-07, tf=2e-07, fs=10000.0, Coss=0.0)
# -> E_on = 0.003 J, E_off = 0.006 J, P_sw = 90 W, P_oss = 0 W, P_total = 90 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Switching loss: V = 400 V, I = 10 A, tr = 30 ns, tf = 50 ns, fs = 100 kHz, Coss = 100 pF | SOLVED | E_on = 6e-05 J, E_off = 0.0001 J, P_sw = 16 W, P_oss = 0.8 W, P_total = 16.8 W |
| Switching loss: V = 600 V, I = 100 A, tr = 100 ns, tf = 200 ns, fs = 10 kHz, Coss = 0 pF | SOLVED | E_on = 0.003 J, E_off = 0.006 J, P_sw = 90 W, P_oss = 0 W, P_total = 90 W |
| Switching loss: I = 10 A, tr = 30 ns, tf = 50 ns, fs = 100 kHz, Coss = 100 pF | NEEDS_INPUT | pe_switching_loss needs V (blocking (dc bus) voltage). Example: solve('pe_switching_loss', V='400 V', I='10 A', tr='3e-0 |
| Switching loss: V = 400 V, I = 10 A, tr = 30 ns, tf = 50 ns, fs = 100 kHz, Coss = 100 pF, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_gate_drive_power"></a>

### Gate-drive power and currents: `pe_gate_drive_power`

The gate-driver power, energy per switching cycle, average gate current and peak gate current of a MOSFET or IGBT from its total gate charge, drive voltage, switching frequency and total gate resistance.

$$
P_{G} = Q_gV_{drv}f_s,\quad I_{G,pk} = \frac{V_{drv}}{R_g}
$$

| input | meaning | condition |
|---|---|---|
| `Qg` | total gate charge at the drive voltage |  |
| `Vdrv` | gate-drive voltage swing |  |
| `fs` | switching frequency |  |
| `Rg` | total gate resistance (driver, external and internal) |  |

| output | meaning |
|---|---|
| `P_drive` | gate-drive power Qg V fs |
| `Ig_avg` | average gate current Qg fs |
| `Ig_peak` | peak gate current V/Rg |
| `E_cycle` | energy per switching cycle Qg V |

- **Method:** $P = Qg$ Vdrv fs; Ig,avg = Qg fs; Ig,peak = Vdrv/Rg
- **Assumes:** the gate fully charges and discharges every cycle
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P_drive (gate-drive power Qg V fs) against Qg (total gate charge at the drive voltage) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-gate-drive-power-curve)

*P_drive (gate-drive power Qg V fs) against Qg (total gate charge at the drive voltage) from 2.5000e-08 C to 7.5000e-08 C. One curve for each value of Vdrv (gate-drive voltage swing): 12 V, 6 V, 24 V. Other inputs as in the worked example: fs = 200000.0 Hz, Rg = 5.0 ohm. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Qg = 5.0000e-08 C, P_drive = 0.12 W.*

```python
power_electronics_ai999x.solve('pe_gate_drive_power', Qg=5e-08, Vdrv=12.0, fs=200000.0, Rg=5.0)
# -> P_drive = 0.12 W, Ig_avg = 0.01 A, Ig_peak = 2.4 A, E_cycle = 6e-07 J   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_gate_drive_power', Qg=1.2e-06, Vdrv=15.0, fs=10000.0, Rg=2.0)
# -> P_drive = 0.18 W, Ig_avg = 0.012 A, Ig_peak = 7.5 A, E_cycle = 1.8e-05 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gate drive power: Qg = 50 nC, Vdrv = 12 V, fs = 200 kHz, Rg = 5 ohm | SOLVED | P_drive = 0.12 W, Ig_avg = 0.01 A, Ig_peak = 2.4 A, E_cycle = 6e-07 J |
| Gate drive power: Qg = 1200 nC, Vdrv = 15 V, fs = 10 kHz, Rg = 2 ohm | SOLVED | P_drive = 0.18 W, Ig_avg = 0.012 A, Ig_peak = 7.5 A, E_cycle = 1.8e-05 J |
| Gate drive power: Vdrv = 12 V, fs = 200 kHz, Rg = 5 ohm | NEEDS_INPUT | pe_gate_drive_power needs Qg (total gate charge at the drive voltage). Example: solve('pe_gate_drive_power', Qg='5e-08 C |
| Gate drive power: Qg = 50 nC, Vdrv = 12 V, fs = 200 kHz, Rg = 5 ohm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_diode_reverse_recovery"></a>

### Diode reverse-recovery charge and losses: `pe_diode_reverse_recovery`

The recovered charge, the two recovery intervals and the reverse-recovery energy dissipated in the diode and the extra turn-on energy in the switch (and the powers) for a triangular recovery current of peak Irm, time trr and softness $S = tb/ta.$

$$
Q_{rr} = \tfrac12I_{rm}t_{rr},\quad E_D = \frac{V_rI_{rm}t_b}{6},\quad E_{sw} = V_rI_{rm}\left(\frac{t_a}{2}+\frac{t_b}{3}\right)
$$

| input | meaning | condition |
|---|---|---|
| `Irm` | peak reverse-recovery current |  |
| `trr` | reverse-recovery time |  |
| `S` | softness factor tb/ta |  |
| `Vr` | reverse (bus) voltage |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `Qrr` | recovered charge |
| `ta` | current rise interval |
| `tb` | current decay interval |
| `E_diode` | recovery energy dissipated in the diode |
| `E_switch` | extra switch turn-on energy |
| `P_diode` | diode recovery loss |
| `P_switch` | switch loss caused by recovery |
| `P_total` | total recovery loss (= Vr Qrr fs) |

- **Method:** ta = trr/(1 + S), $tb = S$ trr/(1 + S); Qrr = Irm trr/2; $E_D = Vr$ Irm $tb/6$; $E_sw = Vr$ Irm $(ta/2 + tb/3)$; together Vr Qrr
- **Assumes:** triangular recovery current; diode voltage rises linearly during tb
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Qrr (recovered charge) against Irm (peak reverse-recovery current) from 5 A to 15 A](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-diode-reverse-recovery-curve)

*Qrr (recovered charge) against Irm (peak reverse-recovery current) from 5 A to 15 A. One curve for each value of trr (reverse-recovery time): 1.0000e-07 s, 5.0000e-08 s, 2.0000e-07 s. Other inputs as in the worked example: S = 1.0, Vr = 400.0 V, fs = 100000.0 Hz. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Irm = 10 A, Qrr = 5.0000e-07 C.*

```python
power_electronics_ai999x.solve('pe_diode_reverse_recovery', Irm=10.0, trr=1e-07, S=1.0, Vr=400.0, fs=100000.0)
# -> Qrr = 5e-07 C, ta = 5e-08 s, tb = 5e-08 s, E_diode = 3.33333e-05 J, E_switch = 0.000166667 J, P_diode = 3.33333 W, P_switch = 16.6667 W, P_total = 20 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_diode_reverse_recovery', Irm=40.0, trr=3e-07, S=0.5, Vr=600.0, fs=20000.0)
# -> Qrr = 6e-06 C, ta = 2e-07 s, tb = 1e-07 s, E_diode = 0.0004 J, E_switch = 0.0032 J, P_diode = 8 W, P_switch = 64 W, P_total = 72 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Diode reverse recovery: Irm = 10 A, trr = 100 ns, S = 1, Vr = 400 V, fs = 100 kHz | SOLVED | Qrr = 5e-07 C, ta = 5e-08 s, tb = 5e-08 s, E_diode = 3.33333e-05 J, E_switch = 0.000166667 J, P_diode = 3.33333 W, P_switch = 16.6667 W, P_total = 20 W |
| Diode reverse recovery: Irm = 40 A, trr = 300 ns, S = 0.5, Vr = 600 V, fs = 20 kHz | SOLVED | Qrr = 6e-06 C, ta = 2e-07 s, tb = 1e-07 s, E_diode = 0.0004 J, E_switch = 0.0032 J, P_diode = 8 W, P_switch = 64 W, P_total = 72 W |
| Diode reverse recovery: trr = 100 ns, S = 1, Vr = 400 V, fs = 100 kHz | NEEDS_INPUT | pe_diode_reverse_recovery needs Irm (peak reverse-recovery current). Example: solve('pe_diode_reverse_recovery', Irm='10 |
| Diode reverse recovery: Irm = 10 A, trr = 100 ns, S = 1, Vr = 400 V, fs = 100 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_parallel_mosfet_sharing"></a>

### Current sharing between paralleled MOSFETs, `pe_parallel_mosfet_sharing`

The current and conduction loss of each of several paralleled MOSFETs (or any resistive devices) sharing a total current in proportion to their on-state conductances, the total loss and the worst-case current imbalance.

$$
I_k = I\frac{1/R_k}{\sum_j 1/R_j},\quad P_k = I_k^2R_k
$$

| input | meaning | condition |
|---|---|---|
| `R` | on-resistance of each device in milliohms |  |
| `I` | total current |  |

| output | meaning |
|---|---|
| `currents` | current in each device, amperes |
| `losses` | conduction loss of each device, watts |
| `P_total` | total conduction loss |
| `imbalance` | largest current divided by the equal share |

- **Method:** $I_k = I$ (1/R_k)/sum(1/R_j); $P_k = I_k^{2}$ R_k
- **Assumes:** static sharing (on-resistance only)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The plane vectors of the worked example as arrows from the origin: input R = (10, 12); returned by power-electronics-ai999x: currents = (21](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-parallel-mosfet-sharing-vectors)

*The plane vectors of the worked example as arrows from the origin: input R = (10, 12); returned by power-electronics-ai999x: currents = (21.8182, 18.1818), losses = (4.76033, 3.96694); also returned P_total = 8.72727, imbalance = 1.09091. Other inputs: I = 40.0 A. Marked: the tip of each returned vector.*

![P_total (total conduction loss) against I (total current) from 20 A to 60 A](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-parallel-mosfet-sharing-curve)

*P_total (total conduction loss) against I (total current) from 20 A to 60 A. Other inputs as in the worked example: R = [10.0, 12.0]. 31 points computed by power-electronics-ai999x; marked: the worked example, I = 40 A, P_total = 8.72727 W.*

```python
power_electronics_ai999x.solve('pe_parallel_mosfet_sharing', R=[10.0, 12.0], I=40.0)
# -> currents = [21.8181818182, 18.1818181818], losses = [4.76033057851, 3.96694214876], P_total = 8.72727 W, imbalance = 1.09091   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_parallel_mosfet_sharing', R=[4.5, 5.0, 5.5], I=90.0)
# -> currents = [33.110367893, 29.7993311037, 27.0903010033], losses = [4.93333407904, 4.44000067113, 4.03636424648], P_total = 13.4097 W, imbalance = 1.10368   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Parallel MOSFET current sharing: R = [10, 12], I = 40 A | SOLVED | currents = [21.8181818182, 18.1818181818], losses = [4.76033057851, 3.96694214876], P_total = 8.72727 W, imbalance = 1.09091 |
| Parallel MOSFET current sharing: R = [4.5, 5, 5.5], I = 90 A | SOLVED | currents = [33.110367893, 29.7993311037, 27.0903010033], losses = [4.93333407904, 4.44000067113, 4.03636424648], P_total = 13.4097 W, imbalance = 1.10368 |
| Parallel MOSFET current sharing: I = 40 A | NEEDS_INPUT | pe_parallel_mosfet_sharing needs R (on-resistance of each device in milliohms). Example: solve('pe_parallel_mosfet_shari |
| Parallel MOSFET current sharing: R = [10, 12], I = 40 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_bootstrap_capacitor"></a>

### Bootstrap capacitor for a high-side gate driver, `pe_bootstrap_capacitor`

The charge drawn from a bootstrap capacitor during the longest high-side on-time and the smallest capacitance that keeps its droop within a limit, with the usual tenfold design margin.

$$
C_{boot} \ge \frac{Q_g + I_{lk}D_{max}/f_s}{\Delta V}
$$

| input | meaning | condition |
|---|---|---|
| `Qg` | high-side gate charge |  |
| `Ileak` | quiescent plus leakage current of the high side |  |
| `fs` | switching frequency |  |
| `Dmax` | maximum high-side duty cycle (a fraction or %) |  |
| `dV` | allowed bootstrap droop |  |

| output | meaning |
|---|---|
| `Q_total` | charge drawn per cycle |
| `C_min` | smallest bootstrap capacitance |
| `C_recommended` | tenfold margin |

- **Method:** Q = Qg + I_leak Dmax/fs; C_min = Q/dV
- **Assumes:** the capacitor recharges fully while the low side is on
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![C_min (smallest bootstrap capacitance) against dV (allowed bootstrap droop) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-bootstrap-capacitor-curve)

*C_min (smallest bootstrap capacitance) against dV (allowed bootstrap droop) from 0.25 V to 0.75 V. One curve for each value of Qg (high-side gate charge): 6.0000e-08 C, 3.0000e-08 C, 1.2000e-07 C. Other inputs as in the worked example: Ileak = 0.00019999999999999998 A, fs = 100000.0 Hz, Dmax = 0.9. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, dV = 0.5 V, C_min = 1.2360e-07 F.*

```python
power_electronics_ai999x.solve('pe_bootstrap_capacitor', Qg=6e-08, Ileak=0.0002, fs=100000.0, Dmax=0.9, dV=0.5)
# -> Q_total = 6.18e-08 C, C_min = 1.236e-07 F, C_recommended = 1.236e-06 F   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_bootstrap_capacitor', Qg=2e-07, Ileak=0.001, fs=10000.0, Dmax=0.95, dV=1.0)
# -> Q_total = 2.95e-07 C, C_min = 2.95e-07 F, C_recommended = 2.95e-06 F   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bootstrap capacitor: Qg = 60 nC, Ileak = 200 uA, fs = 100 kHz, Dmax = 0.9, dV = 0.5 V | SOLVED | Q_total = 6.18e-08 C, C_min = 1.236e-07 F, C_recommended = 1.236e-06 F |
| Bootstrap capacitor: Qg = 200 nC, Ileak = 1000 uA, fs = 10 kHz, Dmax = 0.95, dV = 1 V | SOLVED | Q_total = 2.95e-07 C, C_min = 2.95e-07 F, C_recommended = 2.95e-06 F |
| Bootstrap capacitor: Ileak = 200 uA, fs = 100 kHz, Dmax = 0.9, dV = 0.5 V | NEEDS_INPUT | pe_bootstrap_capacitor needs Qg (high-side gate charge). Example: solve('pe_bootstrap_capacitor', Qg='6e-08 C', Ileak='0 |
| Bootstrap capacitor: Qg = 60 nC, Ileak = 200 uA, fs = 100 kHz, Dmax = 0.9, dV = 0.5 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_turnoff_snubber"></a>

### RCD turn-off snubber design: `pe_turnoff_snubber`

The turn-off snubber capacitance that lets the switch voltage reach the bus voltage just as the current finishes falling, the switch turn-off energy with and without the snubber, and the snubber (resistor) loss per cycle and in watts.

$$
C_s = \frac{I t_f}{2V},\quad E_{sw} = \frac{I^2t_f^2}{24C_s} = \frac{VIt_f}{12},\quad E_R = \tfrac12C_sV^2
$$

| input | meaning | condition |
|---|---|---|
| `I` | switched (load) current |  |
| `V` | bus voltage |  |
| `tf` | current fall time |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `Cs` | snubber capacitance |
| `E_off_with` | switch turn-off energy with the snubber |
| `E_off_without` | switch turn-off energy without a snubber |
| `E_snubber` | energy dumped in the snubber resistor |
| `P_switch` | switch turn-off loss with the snubber |
| `P_snubber` | snubber resistor loss |
| `P_total` | switch plus snubber loss |

- **Method:** $Cs = I$ $tf/(2V)$; $E_sw = I^{2}$ tf^2/(24 Cs); $E_R = Cs$ $V^{2}/2$
- **Assumes:** linear current fall; the snubber capacitor discharges fully during the on-time
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Cs (snubber capacitance) against V (bus voltage) from 150 V to 450 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-turnoff-snubber-curve)

*Cs (snubber capacitance) against V (bus voltage) from 150 V to 450 V. One curve for each value of I (switched (load) current): 10 A, 5 A, 20 A. Other inputs as in the worked example: tf = 1.0000000000000001e-07 s, fs = 50000.0 Hz. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, V = 300 V, Cs = 1.6667e-09 F.*

```python
power_electronics_ai999x.solve('pe_turnoff_snubber', I=10.0, V=300.0, tf=1e-07, fs=50000.0)
# -> Cs = 1.66667e-09 F, E_off_with = 2.5e-05 J, E_off_without = 0.00015 J, E_snubber = 7.5e-05 J, P_switch = 1.25 W, P_snubber = 3.75 W, P_total = 5 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_turnoff_snubber', I=50.0, V=600.0, tf=4e-07, fs=10000.0)
# -> Cs = 1.66667e-08 F, E_off_with = 0.001 J, E_off_without = 0.006 J, E_snubber = 0.003 J, P_switch = 10 W, P_snubber = 30 W, P_total = 40 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Turn-off snubber: I = 10 A, V = 300 V, tf = 100 ns, fs = 50 kHz | SOLVED | Cs = 1.66667e-09 F, E_off_with = 2.5e-05 J, E_off_without = 0.00015 J, E_snubber = 7.5e-05 J, P_switch = 1.25 W, P_snubber = 3.75 W, P_total = 5 W |
| Turn-off snubber: I = 50 A, V = 600 V, tf = 400 ns, fs = 10 kHz | SOLVED | Cs = 1.66667e-08 F, E_off_with = 0.001 J, E_off_without = 0.006 J, E_snubber = 0.003 J, P_switch = 10 W, P_snubber = 30 W, P_total = 40 W |
| Turn-off snubber: V = 300 V, tf = 100 ns, fs = 50 kHz | NEEDS_INPUT | pe_turnoff_snubber needs I (switched (load) current). Example: solve('pe_turnoff_snubber', I='10 A', V='300 V', tf='1e-0 |
| Turn-off snubber: I = 10 A, V = 300 V, tf = 100 ns, fs = 50 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_rc_snubber_ringing"></a>

### RC snubber from measured ringing frequencies, `pe_rc_snubber_ringing`

The parasitic capacitance and inductance of a ringing switch node found by measuring the ringing frequency with and without an added capacitor, the damping resistor (characteristic impedance), the snubber capacitor (k times the parasitic) and the snubber resistor loss.

$$
C_p = \frac{C_{add}}{(f_0/f_1)^2-1},\quad L_p = \frac{1}{(2\pi f_0)^2C_p},\quad R_s = \sqrt{L_p/C_p}
$$

| input | meaning | condition |
|---|---|---|
| `f0` | ringing frequency without the added capacitor |  |
| `f1` | ringing frequency with the added capacitor |  |
| `Cadd` | added test capacitance |  |
| `k` | snubber capacitance as a multiple of the parasitic capacitance |  |
| `V` | voltage the node swings to |  |
| `fs` | switching frequency |  |

| output | meaning |
|---|---|
| `Cp` | parasitic capacitance |
| `Lp` | parasitic inductance |
| `Rs` | snubber resistance sqrt(Lp/Cp) |
| `Cs` | snubber capacitance k Cp |
| `P_R` | snubber resistor loss |

- **Method:** Cp = Cadd/((f0/f1)^2 - 1); $Lp = 1$/((2 pi f0)^2 Cp); $R_{s} = \sqrt{\frac{L}{C_{p}}}$; $Cs = k$ Cp; $P = Cs$ $V^{2}$ fs
- **Assumes:** $f_{1} < f_{0}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Rs (snubber resistance sqrt(Lp/Cp)) against f1 (ringing frequency with the added capacitor) from 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-rc-snubber-ringing-curve)

*Rs (snubber resistance sqrt(Lp/Cp)) against f1 (ringing frequency with the added capacitor) from 1.2500e+07 Hz to 3.7500e+07 Hz. One curve for each value of f0 (ringing frequency without the added capacitor): 5.0000e+07 Hz, 2.5000e+07 Hz, 1.0000e+08 Hz. Other inputs as in the worked example: Cadd = 3e-10 F, k = 4.0, V = 48.0 V, fs = 500000.0 Hz. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, f1 = 2.5000e+07 Hz, Rs = 31.831 ohm.*

```python
power_electronics_ai999x.solve('pe_rc_snubber_ringing', f0=50000000.0, f1=25000000.0, Cadd=3e-10, k=4.0, V=48.0, fs=500000.0)
# -> Cp = 1e-10 F, Lp = 1.01321e-07 H, Rs = 31.831 ohm, Cs = 4e-10 F, P_R = 0.4608 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_rc_snubber_ringing', f0=30000000.0, f1=20000000.0, Cadd=1e-09, k=3.0, V=400.0, fs=100000.0)
# -> Cp = 8e-10 F, Lp = 3.5181e-08 H, Rs = 6.63146 ohm, Cs = 2.4e-09 F, P_R = 38.4 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| RC snubber from ringing: f0 = 50000 kHz, f1 = 25000 kHz, Cadd = 300 pF, k = 4, V = 48 V, fs = 500 kHz | SOLVED | Cp = 1e-10 F, Lp = 1.01321e-07 H, Rs = 31.831 ohm, Cs = 4e-10 F, P_R = 0.4608 W |
| RC snubber from ringing: f0 = 30000 kHz, f1 = 20000 kHz, Cadd = 1000 pF, k = 3, V = 400 V, fs = 100 kHz | SOLVED | Cp = 8e-10 F, Lp = 3.5181e-08 H, Rs = 6.63146 ohm, Cs = 2.4e-09 F, P_R = 38.4 W |
| RC snubber from ringing: f1 = 25000 kHz, Cadd = 300 pF, k = 4, V = 48 V, fs = 500 kHz | NEEDS_INPUT | pe_rc_snubber_ringing needs f0 (ringing frequency without the added capacitor). Example: solve('pe_rc_snubber_ringing',  |
| RC snubber from ringing: f0 = 50000 kHz, f1 = 25000 kHz, Cadd = 300 pF, k = 4, V = 48 V, fs = 500 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_zvs_transition"></a>

### Zero-voltage switching: resonant transition of a switch node, `pe_zvs_transition`

Whether an inductor current can swing a switch node's capacitance from 0 to the input voltage (zero-voltage switching), the minimum current needed, the energies, and the transition time (or the time to the voltage peak when it cannot), for a resonant LC transition.

$$
v(t) = I_0Z\sin\omega t,\quad Z = \sqrt{L/C},\quad t_{zvs} = \frac{1}{\omega}\sin^{-1}\frac{V_{in}}{I_0Z}
$$

| input | meaning | condition |
|---|---|---|
| `Vin` | voltage the node must swing |  |
| `Ceq` | total node capacitance |  |
| `L` | resonant (leakage) inductance |  |
| `I0` | inductor current at the start of the transition |  |

| output | meaning |
|---|---|
| `Z` | characteristic impedance sqrt(L/C) |
| `f_res` | resonant frequency |
| `I_min` | minimum current for ZVS Vin/Z |
| `E_L` | inductor energy |
| `E_C` | capacitor energy |
| `zvs` | achieved or not achieved |
| `t_transition` | time to reach Vin (or to the voltage peak when ZVS fails) |
| `v_reached` | highest node voltage reached (capped at Vin) |

- **Method:** v(t) = I0 Z sin(w t), $Z = \sqrt{\frac{L}{C}}$; ZVS when I0 Z >= Vin; t = asin(Vin/(I0 Z))/w
- **Assumes:** lossless resonant transition starting at 0 V
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![achieved or not achieved (zvs) over Vin (voltage the node must swing) from 40 V to 4000 V and Ceq (total node capacitance) from 4](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-zvs-transition-decision)

*achieved or not achieved (zvs) over Vin (voltage the node must swing) from 40 V to 4000 V and Ceq (total node capacitance) from 4.0000e-11 F to 4.0000e-09 F (logarithmic axes): each of the 31 × 21 cells is one run of power-electronics-ai999x, coloured by the verdict it returned, achieved: 396 cells, not achieved: 255 cells. Other inputs as in the worked example: L = 1.9999999999999998e-05 H, I0 = 3.0 A. Marked: the worked example, Vin = 400 V, Ceq = 4.0000e-10 F → achieved.*

![t_transition (time to reach Vin (or to the voltage peak when ZVS fails)) against L (resonant (leakage) inductance) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-zvs-transition-curve)

*t_transition (time to reach Vin (or to the voltage peak when ZVS fails)) against L (resonant (leakage) inductance) from 2.0000e-06 H to 0.0002 H on a logarithmic axis. One curve for each value of Ceq (total node capacitance): 4.0000e-10 F, 2.0000e-10 F, 8.0000e-10 F. Other inputs as in the worked example: Vin = 400.0 V, I0 = 3.0 A. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 2.0000e-05 H, t_transition = 5.7142e-08 s.*

```python
power_electronics_ai999x.solve('pe_zvs_transition', Vin=400.0, Ceq=4e-10, L=2e-05, I0=3.0)
# -> Z = 223.607 ohm, f_res = 1.77941e+06 Hz, I_min = 1.78885 A, E_L = 9e-05 J, E_C = 3.2e-05 J, zvs = achieved, t_transition = 5.71418e-08 s, v_reached = 400 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_zvs_transition', Vin=48.0, Ceq=2e-09, L=5e-07, I0=1.0)
# -> Z = 15.8114 ohm, f_res = 5.03292e+06 Hz, I_min = 3.03579 A, E_L = 2.5e-07 J, E_C = 2.304e-06 J, zvs = not achieved, t_transition = 4.96729e-08 s, v_reached = 15.8114 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Zero-voltage switching transition: Vin = 400 V, Ceq = 400 pF, L = 20 uH, I0 = 3 A | SOLVED | Z = 223.607 ohm, f_res = 1.77941e+06 Hz, I_min = 1.78885 A, E_L = 9e-05 J, E_C = 3.2e-05 J, zvs = achieved, t_transition = 5.71418e-08 s, v_reached = 400 V |
| Zero-voltage switching transition: Vin = 48 V, Ceq = 2000 pF, L = 0.5 uH, I0 = 1 A | SOLVED | Z = 15.8114 ohm, f_res = 5.03292e+06 Hz, I_min = 3.03579 A, E_L = 2.5e-07 J, E_C = 2.304e-06 J, zvs = not achieved, t_transition = 4.96729e-08 s, v_reached = 15.8114 V |
| Zero-voltage switching transition: Ceq = 400 pF, L = 20 uH, I0 = 3 A | NEEDS_INPUT | pe_zvs_transition needs Vin (voltage the node must swing). Example: solve('pe_zvs_transition', Vin='400 V', Ceq='4e-10 F |
| Zero-voltage switching transition: Vin = 400 V, Ceq = 400 pF, L = 20 uH, I0 = 3 A, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_thermal_shared_heatsink"></a>

### Several power devices on one heat sink: `pe_thermal_shared_heatsink`

The heat-sink temperature and every device's junction temperature when several devices, each with its own junction-to-case and case-to-sink resistance, share one heat sink to ambient.

$$
T_s = T_a + R_{sa}\sum_kP_k,\quad T_{j,k} = T_s + P_k(R_{jc}+R_{cs})
$$

| input | meaning | condition |
|---|---|---|
| `P` | loss of each device in watts |  |
| `Rjc` | junction-to-case thermal resistance of each device |  |
| `Rcs` | case-to-sink (interface) resistance of each device |  |
| `Rsa` | sink-to-ambient resistance |  |
| `Ta` | ambient temperature |  |

| output | meaning |
|---|---|
| `P_total` | total loss |
| `Ts` | heat-sink temperature |
| `Tj` | junction temperature of each device, kelvin |
| `Tj_max` | hottest junction |
| `Tj_max_C` | hottest junction in degC |

- **Method:** Ts = Ta + Rsa sum(P); Tj,$k = Ts + P_k$ (Rjc + Rcs)
- **Assumes:** steady state; the sink is isothermal
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![junction temperature of each device, kelvin (Tj) returned by power-electronics-ai999x for the worked example, one value for each position in the list (3 values, first 360](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-thermal-shared-heatsink-series)

*junction temperature of each device, kelvin (Tj) returned by power-electronics-ai999x for the worked example, one value for each position in the list (3 values, first 360.65, last 355.65, largest 360.65). Inputs: P = [20.0, 20.0, 15.0], Rjc = 0.8 K/W, Rcs = 0.2 K/W, Rsa = 0.5 K/W, Ta = 313.15 K.*

![Tj_max (hottest junction) against Ta (ambient temperature) from 156](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-thermal-shared-heatsink-curve)

*Tj_max (hottest junction) against Ta (ambient temperature) from 156.575 K to 469.725 K. One curve for each value of Rsa (sink-to-ambient resistance): 0.5 K/W, 0.25 K/W, 1 K/W. Other inputs as in the worked example: P = [20.0, 20.0, 15.0], Rjc = 0.8 K/W, Rcs = 0.2 K/W. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Ta = 313.15 K, Tj_max = 360.65 K.*

```python
power_electronics_ai999x.solve('pe_thermal_shared_heatsink', P=[20.0, 20.0, 15.0], Rjc=0.8, Rcs=0.2, Rsa=0.5, Ta=313.15)
# -> P_total = 55 W, Ts = 340.65 K, Tj = [360.65, 360.65, 355.65], Tj_max = 360.65 K, Tj_max_C = 87.5   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_thermal_shared_heatsink', P=[50.0, 30.0], Rjc=0.4, Rcs=0.1, Rsa=0.3, Ta=298.15)
# -> P_total = 80 W, Ts = 322.15 K, Tj = [347.15, 337.15], Tj_max = 347.15 K, Tj_max_C = 74   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Shared heat sink temperatures: P = [20, 20, 15], Rjc = 0.8 K/W, Rcs = 0.2 K/W, Rsa = 0.5 K/W, Ta = 40 degC | SOLVED | P_total = 55 W, Ts = 340.65 K, Tj = [360.65, 360.65, 355.65], Tj_max = 360.65 K, Tj_max_C = 87.5 |
| Shared heat sink temperatures: P = [50, 30], Rjc = 0.4 K/W, Rcs = 0.1 K/W, Rsa = 0.3 K/W, Ta = 25 degC | SOLVED | P_total = 80 W, Ts = 322.15 K, Tj = [347.15, 337.15], Tj_max = 347.15 K, Tj_max_C = 74 |
| Shared heat sink temperatures: Rjc = 0.8 K/W, Rcs = 0.2 K/W, Rsa = 0.5 K/W, Ta = 40 degC | NEEDS_INPUT | pe_thermal_shared_heatsink needs P (loss of each device in watts). Example: solve('pe_thermal_shared_heatsink', P=[20.0, |
| Shared heat sink temperatures: P = [20, 20, 15], Rjc = 0.8 K/W, Rcs = 0.2 K/W, Rsa = 0.5 K/W, Ta = 40 degC, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Shared heat sink temperatures: P = [20, 20, 15], Rjc = 0.8 degC/W, Rcs = 0.2 degC/W, Rsa = 0.5 degC/W, Ta = 40 degC | SOLVED | P_total = 55 W, Ts = 340.65 K, Tj = [360.65, 360.65, 355.65], Tj_max = 360.65 K, Tj_max_C = 87.5 |

<a id="pe_foster_transient_thermal"></a>

### Transient thermal impedance of a Foster network, `pe_foster_transient_thermal`

The transient thermal impedance Zth(tp) of a device described by a Foster RC network (datasheet R_i, tau_i pairs) and the junction temperature rise for a single power pulse of duration tp, with the steady-state values.

$$
Z_{th}(t) = \sum_i R_i\left(1-e^{-t/\tau_i}\right),\quad \Delta T_j = P\,Z_{th}(t_p)
$$

| input | meaning | condition |
|---|---|---|
| `R` | Foster thermal resistances in kelvin per watt |  |
| `tau` | Foster time constants in seconds |  |
| `P` | pulse power |  |
| `tp` | pulse duration |  |

| output | meaning |
|---|---|
| `Zth` | transient thermal impedance at tp |
| `dTj` | junction temperature rise at the end of the pulse |
| `Rth_total` | steady-state thermal resistance |
| `dTj_steady` | steady-state rise for continuous power |

- **Method:** Zth(t) = sum R_i (1 - exp(-t/tau_i)); dTj = P Zth(tp)
- **Assumes:** linear network, starting from thermal equilibrium
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The data R (Foster thermal resistances in kelvin per watt) against tau (Foster time constants in seconds) as given in the worked example (3 points), logarithmic horizontal axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-foster-transient-thermal-xy)

*The data R (Foster thermal resistances in kelvin per watt) against tau (Foster time constants in seconds) as given in the worked example (3 points), logarithmic horizontal axis. Also returned: Zth = 0.173623 K/W, dTj = 86.8115 K, Rth_total = 0.3 K/W, dTj_steady = 150 K. Other inputs: P = 500.0 W, tp = 0.01 s.*

![dTj (junction temperature rise at the end of the pulse) against P (pulse power) from 250 W to 750 W](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-foster-transient-thermal-curve)

*dTj (junction temperature rise at the end of the pulse) against P (pulse power) from 250 W to 750 W. One curve for each value of tp (pulse duration): 0.01 s, 0.005 s, 0.02 s. Other inputs as in the worked example: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05]. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, P = 500 W, dTj = 86.8115 K.*

```python
power_electronics_ai999x.solve('pe_foster_transient_thermal', R=[0.05, 0.1, 0.15], tau=[0.0001, 0.003, 0.05], P=500.0, tp=0.01)
# -> Zth = 0.173623 K/W, dTj = 86.8115 K, Rth_total = 0.3 K/W, dTj_steady = 150 K   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_foster_transient_thermal', R=[0.2, 0.3], tau=[0.001, 0.1], P=100.0, tp=0.02)
# -> Zth = 0.254381 K/W, dTj = 25.4381 K, Rth_total = 0.5 K/W, dTj_steady = 50 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Foster transient thermal impedance: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 10 ms | SOLVED | Zth = 0.173623 K/W, dTj = 86.8115 K, Rth_total = 0.3 K/W, dTj_steady = 150 K |
| Foster transient thermal impedance: R = [0.2, 0.3], tau = [0.001, 0.1], P = 100 W, tp = 20 ms | SOLVED | Zth = 0.254381 K/W, dTj = 25.4381 K, Rth_total = 0.5 K/W, dTj_steady = 50 K |
| Foster transient thermal impedance: tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 10 ms | NEEDS_INPUT | pe_foster_transient_thermal needs R (Foster thermal resistances in kelvin per watt). Example: solve('pe_foster_transient |
| Foster transient thermal impedance: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 10 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_foster_periodic_pulses"></a>

### Junction temperature under periodic power pulses, `pe_foster_periodic_pulses`

The peak, minimum and average junction temperature rise and the temperature ripple of a device described by a Foster network under a periodic rectangular power pulse train (exact periodic steady state, no single-pulse approximation).

$$
\Delta T_{pk} = \sum_iPR_i\frac{1-e^{-t_p/\tau_i}}{1-e^{-T/\tau_i}}
$$

| input | meaning | condition |
|---|---|---|
| `R` | Foster thermal resistances in kelvin per watt |  |
| `tau` | Foster time constants in seconds |  |
| `P` | pulse power |  |
| `tp` | pulse duration |  |
| `T` | pulse period |  |

| output | meaning |
|---|---|
| `dT_peak` | peak temperature rise (end of each pulse) |
| `dT_min` | minimum temperature rise (start of each pulse) |
| `dT_avg` | average rise P Rth tp/T |
| `ripple` | peak minus minimum |

- **Method:** per element: peak = P R (1 - e^(-tp/tau))/(1 - e^(-T/tau)); minimum = peak e^(-(T - tp)/tau); summed over the network
- **Assumes:** periodic steady state; $tp < T$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The data R (Foster thermal resistances in kelvin per watt) against tau (Foster time constants in seconds) as given in the worked example (3 points), logarithmic horizontal axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-foster-periodic-pulses-xy)

*The data R (Foster thermal resistances in kelvin per watt) against tau (Foster time constants in seconds) as given in the worked example (3 points), logarithmic horizontal axis. Also returned: dT_peak = 66.4525 K, dT_min = 15.5776 K, dT_avg = 30 K, ripple = 50.8749 K. Other inputs: P = 500.0 W, tp = 0.002 s, T = 0.01 s.*

![dT_peak (peak temperature rise (end of each pulse)) against T (pulse period) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-foster-periodic-pulses-curve)

*dT_peak (peak temperature rise (end of each pulse)) against T (pulse period) from 0.001 s to 0.1 s on a logarithmic axis. One curve for each value of P (pulse power): 500 W, 250 W, 1000 W. Other inputs as in the worked example: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], tp = 0.002 s. 34 points per curve computed by power-electronics-ai999x; marked: the worked example, T = 0.01 s, dT_peak = 66.4525 K.*

```python
power_electronics_ai999x.solve('pe_foster_periodic_pulses', R=[0.05, 0.1, 0.15], tau=[0.0001, 0.003, 0.05], P=500.0, tp=0.002, T=0.01)
# -> dT_peak = 66.4525 K, dT_min = 15.5776 K, dT_avg = 30 K, ripple = 50.8749 K   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_foster_periodic_pulses', R=[0.2, 0.3], tau=[0.001, 0.1], P=100.0, tp=0.01, T=0.04)
# -> dT_peak = 28.6586 K, dT_min = 6.41515 K, dT_avg = 12.5 K, ripple = 22.2435 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Periodic pulse junction temperature: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 2 ms, T = 10 ms | SOLVED | dT_peak = 66.4525 K, dT_min = 15.5776 K, dT_avg = 30 K, ripple = 50.8749 K |
| Periodic pulse junction temperature: R = [0.2, 0.3], tau = [0.001, 0.1], P = 100 W, tp = 10 ms, T = 40 ms | SOLVED | dT_peak = 28.6586 K, dT_min = 6.41515 K, dT_avg = 12.5 K, ripple = 22.2435 K |
| Periodic pulse junction temperature: tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 2 ms, T = 10 ms | NEEDS_INPUT | pe_foster_periodic_pulses needs R (Foster thermal resistances in kelvin per watt). Example: solve('pe_foster_periodic_pu |
| Periodic pulse junction temperature: R = [0.05, 0.1, 0.15], tau = [0.0001, 0.003, 0.05], P = 500 W, tp = 2 ms, T = 10 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_transformer_turns"></a>

### Transformer turns from Faraday's law: `pe_transformer_turns`

The primary and secondary turns of a transformer that keep the peak flux density at Bmax in a core of cross-section Ac (sine or square-wave excitation), the turns ratio and the volts per turn.

$$
N_p = \frac{V}{K_f f B_{max}A_c},\quad K_f = \sqrt2\pi\ (\text{sine}),\ 4\ (\text{square})
$$

| input | meaning | condition |
|---|---|---|
| `waveform` | sine or square |  |
| `V` | primary voltage (RMS for sine, amplitude for square) |  |
| `f` | frequency |  |
| `Bmax` | peak flux density |  |
| `Ac` | core cross-section area |  |
| `Vs` | secondary voltage (same measure as V) |  |

| output | meaning |
|---|---|
| `Np` | primary turns (before rounding up) |
| `Ns` | secondary turns (before rounding) |
| `n` | turns ratio Np/Ns |
| `volts_per_turn` | volts per turn |

- **Method:** Np = V/(Kf f Bmax Ac), $Kf = 4.44$ (sine, RMS) or 4 (square)
- **Assumes:** the core does not saturate; no dc bias
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Np (primary turns (before rounding up)) against f (frequency) from 50000 Hz to 150000 Hz](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-transformer-turns-curve)

*Np (primary turns (before rounding up)) against f (frequency) from 50000 Hz to 150000 Hz. One curve for each value of V (primary voltage (RMS for sine, amplitude for square)): 48 V, 24 V, 96 V. Other inputs as in the worked example: waveform = square, Bmax = 0.15 T, Ac = 0.000125 m^2, Vs = 12.0 V. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 100000 Hz, Np = 6.4.*

```python
power_electronics_ai999x.solve('pe_transformer_turns', waveform='square', V=48.0, f=100000.0, Bmax=0.15, Ac=0.000125, Vs=12.0)
# -> Np = 6.4, Ns = 1.6, n = 4, volts_per_turn = 7.5 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_transformer_turns', waveform='sine', V=230.0, f=50.0, Bmax=1.2, Ac=0.001, Vs=24.0)
# -> Np = 862.803, Ns = 90.0316, n = 9.58333, volts_per_turn = 0.266573 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transformer turns: waveform = square, V = 48 V, f = 100 kHz, Bmax = 150 mT, Ac = 1.25 cm^2, Vs = 12 V | SOLVED | Np = 6.4, Ns = 1.6, n = 4, volts_per_turn = 7.5 V |
| Transformer turns: waveform = sine, V = 230 V, f = 0.05 kHz, Bmax = 1200 mT, Ac = 10 cm^2, Vs = 24 V | SOLVED | Np = 862.803, Ns = 90.0316, n = 9.58333, volts_per_turn = 0.266573 V |
| Transformer turns: V = 48 V, f = 100 kHz, Bmax = 150 mT, Ac = 1.25 cm^2, Vs = 12 V | NEEDS_INPUT | pe_transformer_turns needs waveform (sine or square). Example: solve('pe_transformer_turns', waveform='square', V='48 V' |
| Transformer turns: waveform = square, V = 48 V, f = 100 kHz, Bmax = 150 mT, Ac = 1.25 cm^2, Vs = 12 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_gapped_inductor_design"></a>

### Gapped-core inductor: turns and air gap, `pe_gapped_inductor_design`

The number of turns and the air-gap length of a gapped-core inductor that gives an inductance L with the peak flux density Bmax at the peak current (core reluctance and fringing neglected), the stored energy, the inductance factor AL and the gap volume.

$$
N = \frac{LI_{pk}}{B_{max}A_c},\quad l_g = \frac{\mu_0N^2A_c}{L}
$$

| input | meaning | condition |
|---|---|---|
| `L` | inductance |  |
| `Ipk` | peak current |  |
| `Bmax` | peak flux density allowed |  |
| `Ac` | core cross-section area |  |

| output | meaning |
|---|---|
| `N` | turns (before rounding) |
| `l_gap` | air-gap length |
| `E` | peak stored energy |
| `AL` | inductance per turn squared |
| `V_gap` | gap volume |

- **Method:** $N = L$ Ipk/(Bmax Ac); $l_g = mu0$ $N^{2}$ Ac/L; $E = L$ Ipk^2/2
- **Assumes:** gap reluctance dominates; no fringing
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![N (turns (before rounding)) against Bmax (peak flux density allowed) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-gapped-inductor-design-curve)

*N (turns (before rounding)) against Bmax (peak flux density allowed) from 0.15 T to 0.45 T. One curve for each value of L (inductance): 1.0000e-04 H, 5.0000e-05 H, 0.0002 H. Other inputs as in the worked example: Ipk = 10.0 A, Ac = 0.0001 m^2. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, Bmax = 0.3 T, N = 33.3333.*

```python
power_electronics_ai999x.solve('pe_gapped_inductor_design', L=0.0001, Ipk=10.0, Bmax=0.3, Ac=0.0001)
# -> N = 33.3333, l_gap = 0.00139626 m, E = 0.005 J, AL = 9e-08 H, V_gap = 1.39626e-07 m^3   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_gapped_inductor_design', L=0.001, Ipk=2.0, Bmax=0.25, Ac=5e-05)
# -> N = 160, l_gap = 0.0016085 m, E = 0.002 J, AL = 3.90625e-08 H, V_gap = 8.04248e-08 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gapped inductor design: L = 100 uH, Ipk = 10 A, Bmax = 300 mT, Ac = 1 cm^2 | SOLVED | N = 33.3333, l_gap = 0.00139626 m, E = 0.005 J, AL = 9e-08 H, V_gap = 1.39626e-07 m^3 |
| Gapped inductor design: L = 1000 uH, Ipk = 2 A, Bmax = 250 mT, Ac = 0.5 cm^2 | SOLVED | N = 160, l_gap = 0.0016085 m, E = 0.002 J, AL = 3.90625e-08 H, V_gap = 8.04248e-08 m^3 |
| Gapped inductor design: Ipk = 10 A, Bmax = 300 mT, Ac = 1 cm^2 | NEEDS_INPUT | pe_gapped_inductor_design needs L (inductance). Example: solve('pe_gapped_inductor_design', L='0.0001 H', Ipk='10 A', Bm |
| Gapped inductor design: L = 100 uH, Ipk = 10 A, Bmax = 300 mT, Ac = 1 cm^2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_inductor_area_product"></a>

### Inductor core area product: `pe_inductor_area_product`

The core area product $Ap = Ac$ Wa an inductor needs to store its energy at the peak flux density Bmax with a current density J and window utilisation Ku (core selection by the area-product method).

$$
A_p = A_cW_a = \frac{L\,I_{pk}I_{rms}}{B_{max}JK_u}
$$

| input | meaning | condition |
|---|---|---|
| `L` | inductance |  |
| `Ipk` | peak current |  |
| `Irms` | RMS current |  |
| `Bmax` | peak flux density |  |
| `J` | winding current density |  |
| `Ku` | window utilisation factor (a fraction or %) |  |

| output | meaning |
|---|---|
| `Ap` | area product Ac Wa |
| `Ap_cm4` | area product in centimetres to the fourth |

- **Method:** $Ap = L$ Ipk Irms/(Bmax J Ku)
- **Assumes:** $0 < Ku < 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ap (area product Ac Wa) against Ku (window utilisation factor (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-inductor-area-product-curve)

*Ap (area product Ac Wa) against Ku (window utilisation factor (a fraction or %)) from 0.02 to 0.98. One curve for each value of L (inductance): 1.0000e-04 H, 5.0000e-05 H, 0.0002 H. Other inputs as in the worked example: Ipk = 12.0 A, Irms = 10.0 A, Bmax = 0.3 T, J = 4000000.0 A/m^2. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, Ku = 0.4, Ap = 2.5000e-08 m^4.*

```python
power_electronics_ai999x.solve('pe_inductor_area_product', L=0.0001, Ipk=12.0, Irms=10.0, Bmax=0.3, J=4000000.0, Ku=0.4)
# -> Ap = 2.5e-08 m^4, Ap_cm4 = 2.5   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_inductor_area_product', L=0.002, Ipk=3.0, Irms=2.5, Bmax=0.25, J=3000000.0, Ku=0.3)
# -> Ap = 6.66667e-08 m^4, Ap_cm4 = 6.66667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Inductor area product: L = 100 uH, Ipk = 12 A, Irms = 10 A, Bmax = 300 mT, J = 4 A/mm^2, Ku = 0.4 | SOLVED | Ap = 2.5e-08 m^4, Ap_cm4 = 2.5 |
| Inductor area product: L = 2000 uH, Ipk = 3 A, Irms = 2.5 A, Bmax = 250 mT, J = 3 A/mm^2, Ku = 0.3 | SOLVED | Ap = 6.66667e-08 m^4, Ap_cm4 = 6.66667 |
| Inductor area product: Ipk = 12 A, Irms = 10 A, Bmax = 300 mT, J = 4 A/mm^2, Ku = 0.4 | NEEDS_INPUT | pe_inductor_area_product needs L (inductance). Example: solve('pe_inductor_area_product', L='0.0001 H', Ipk='12 A', Irms |
| Inductor area product: L = 100 uH, Ipk = 12 A, Irms = 10 A, Bmax = 300 mT, J = 4 A/mm^2, Ku = 0.4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_transformer_area_product"></a>

### Transformer core area product: `pe_transformer_area_product`

The apparent power the windings handle and the core area product $Ap = Ac$ Wa a two-winding transformer needs for an output power at an efficiency, flux density, frequency, current density and window utilisation (sine or square-wave excitation).

$$
A_p = \frac{P_t}{K_fK_uB_mfJ},\quad P_t = P_o\left(1+\frac{1}{\eta}\right)
$$

| input | meaning | condition |
|---|---|---|
| `Po` | output power |  |
| `eff` | efficiency (a fraction or %) |  |
| `waveform` | sine or square |  |
| `Bm` | peak flux density |  |
| `f` | frequency |  |
| `J` | current density |  |
| `Ku` | window utilisation (a fraction or %) |  |

| output | meaning |
|---|---|
| `Pt` | apparent power of both windings |
| `Ap` | area product |
| `Ap_cm4` | area product in centimetres to the fourth |

- **Method:** $Pt = Po$ (1 + 1/eff); Ap = Pt/(Kf Ku Bm f J), $Kf = 4.44$ (sine) or 4 (square)
- **Assumes:** two windings at the same current density
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ap (area product) against eff (efficiency (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-transformer-area-product-curve)

*Ap (area product) against eff (efficiency (a fraction or %)) from 0.02 to 0.98. One curve for each value of Ku (window utilisation (a fraction or %)): 0.4, 0.26, 0.74. Other inputs as in the worked example: Po = 250.0 W, waveform = square, Bm = 0.1 T, f = 100000.0 Hz, J = 4000000.0 A/m^2. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, eff = 0.95, Ap = 8.0181e-09 m^4.*

```python
power_electronics_ai999x.solve('pe_transformer_area_product', Po=250.0, eff=0.95, waveform='square', Bm=0.1, f=100000.0, J=4000000.0, Ku=0.4)
# -> Pt = 513.158 W, Ap = 8.01809e-09 m^4, Ap_cm4 = 0.801809   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_transformer_area_product', Po=500.0, eff=0.9, waveform='sine', Bm=1.2, f=50.0, J=3000000.0, Ku=0.35)
# -> Pt = 1055.56 W, Ap = 3.77117e-06 m^4, Ap_cm4 = 377.117   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Transformer area product: Po = 250 W, eff = 0.95, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 0.4 | SOLVED | Pt = 513.158 W, Ap = 8.01809e-09 m^4, Ap_cm4 = 0.801809 |
| Transformer area product: Po = 500 W, eff = 0.9, waveform = sine, Bm = 1200 mT, f = 0.05 kHz, J = 3 A/mm^2, Ku = 0.35 | SOLVED | Pt = 1055.56 W, Ap = 3.77117e-06 m^4, Ap_cm4 = 377.117 |
| Transformer area product: eff = 0.95, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 0.4 | NEEDS_INPUT | pe_transformer_area_product needs Po (output power). Example: solve('pe_transformer_area_product', Po='250 W', eff=0.95, |
| Transformer area product: Po = 250 W, eff = 0.95, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 0.4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
| Transformer area product: Po = 250 W, eff = 95%, waveform = square, Bm = 100 mT, f = 100 kHz, J = 4 A/mm^2, Ku = 40% | SOLVED | Pt = 513.158 W, Ap = 8.01809e-09 m^4, Ap_cm4 = 0.801809 |

<a id="pe_steinmetz_core_loss"></a>

### Core loss by the Steinmetz equation: `pe_steinmetz_core_loss`

The core loss density and total core loss of a magnetic core under sinusoidal flux from the Steinmetz parameters k, alpha, beta (SI: f in Hz, B in T, loss density in W/m^3), the frequency, the peak flux density and the core volume.

$$
P_v = k f^{\alpha} \hat B^{\beta}
$$

| input | meaning | condition |
|---|---|---|
| `k` | Steinmetz coefficient (SI units) |  |
| `alpha` | frequency exponent |  |
| `beta` | flux-density exponent |  |
| `f` | frequency |  |
| `Bpk` | peak flux density |  |
| `Vc` | core volume |  |

| output | meaning |
|---|---|
| `Pv` | core loss density |
| `Pv_mW_cm3` | core loss density in milliwatts per cubic centimetre |
| `P` | core loss |

- **Method:** $Pv = k$ $f^{\alpha}$ Bpk^beta; $P = Pv$ Vc
- **Assumes:** sinusoidal flux, within the parameters' fitted range
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Pv (core loss density) against beta (flux-density exponent) from 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-steinmetz-core-loss-curve)

*Pv (core loss density) against beta (flux-density exponent) from 1.25 to 3.75. One curve for each value of alpha (frequency exponent): 1.4, 0.7, 2.8. Other inputs as in the worked example: k = 3.2, f = 100000.0 Hz, Bpk = 0.1 T, Vc = 5.000000000000001e-06 m^3. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, beta = 2.5, Pv = 101193 W/m^3.*

```python
power_electronics_ai999x.solve('pe_steinmetz_core_loss', k=3.2, alpha=1.4, beta=2.5, f=100000.0, Bpk=0.1, Vc=5e-06)
# -> Pv = 101193 W/m^3, Pv_mW_cm3 = 101.193, P = 0.505964 W   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_steinmetz_core_loss', k=10.0, alpha=1.3, beta=2.7, f=20000.0, Bpk=0.2, Vc=2e-05)
# -> Pv = 50596.4 W/m^3, Pv_mW_cm3 = 50.5964, P = 1.01193 W   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Steinmetz core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3 | SOLVED | Pv = 101193 W/m^3, Pv_mW_cm3 = 101.193, P = 0.505964 W |
| Steinmetz core loss: k = 10, alpha = 1.3, beta = 2.7, f = 20 kHz, Bpk = 200 mT, Vc = 20 cm^3 | SOLVED | Pv = 50596.4 W/m^3, Pv_mW_cm3 = 50.5964, P = 1.01193 W |
| Steinmetz core loss: alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3 | NEEDS_INPUT | pe_steinmetz_core_loss needs k (Steinmetz coefficient (SI units)). Example: solve('pe_steinmetz_core_loss', k=3.2, alpha |
| Steinmetz core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, Bpk = 100 mT, Vc = 5 cm^3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_igse_triangular_core_loss"></a>

### Core loss under triangular flux (iGSE): `pe_igse_triangular_core_loss`

The core loss of a magnetic component whose flux is triangular (a converter inductor or transformer with duty cycle D) by the improved generalized Steinmetz equation, from the sinusoidal Steinmetz parameters; also its ratio to the sinusoidal loss at the same peak flux.

$$
P_v = k_i(\Delta B)^{\beta}f^{\alpha}\left(D^{1-\alpha}+(1-D)^{1-\alpha}\right),\ k_i = \frac{k}{(2\pi)^{\alpha-1}\int_0^{2\pi}|\cos\theta|^{\alpha}2^{\beta-\alpha}d\theta}
$$

| input | meaning | condition |
|---|---|---|
| `k` | Steinmetz coefficient (SI units) |  |
| `alpha` | frequency exponent |  |
| `beta` | flux-density exponent |  |
| `f` | switching frequency |  |
| `dB` | peak-to-peak flux density swing |  |
| `D` | duty cycle of the rising flux (a fraction or %) |  |
| `Vc` | core volume |  |

| output | meaning |
|---|---|
| `ki` | iGSE coefficient |
| `Pv` | core loss density |
| `P` | core loss |
| `ratio_to_sine` | loss relative to a sinusoid of the same peak flux |

- **Method:** $ki = k$/((2 pi)^(alpha-1) int|cos|^alpha $2^{\beta-\alpha}$); $Pv = ki$ $dB^{\beta}$ $f^{\alpha}$ (D^(1-alpha) + (1-D)^(1-alpha))
- **Assumes:** piecewise-linear flux, no relaxation effects
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Pv (core loss density) against D (duty cycle of the rising flux (a fraction or %)) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-igse-triangular-core-loss-curve)

*Pv (core loss density) against D (duty cycle of the rising flux (a fraction or %)) from 0.02 to 0.98. One curve for each value of alpha (frequency exponent): 1.4, 0.7, 2.8. Other inputs as in the worked example: k = 3.2, beta = 2.5, f = 100000.0 Hz, dB = 0.2 T, Vc = 5.000000000000001e-06 m^3. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, D = 0.5, Pv = 94324.8 W/m^3.*

```python
power_electronics_ai999x.solve('pe_igse_triangular_core_loss', k=3.2, alpha=1.4, beta=2.5, f=100000.0, dB=0.2, D=0.5, Vc=5e-06)
# -> ki = 0.199806, Pv = 94324.8 W/m^3, P = 0.471624 W, ratio_to_sine = 0.932129   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_igse_triangular_core_loss', k=3.2, alpha=1.4, beta=2.5, f=100000.0, dB=0.2, D=0.2, Vc=5e-06)
# -> ki = 0.199806, Pv = 107121 W/m^3, P = 0.535603 W, ratio_to_sine = 1.05858   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| iGSE core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, dB = 200 mT, D = 0.5, Vc = 5 cm^3 | SOLVED | ki = 0.199806, Pv = 94324.8 W/m^3, P = 0.471624 W, ratio_to_sine = 0.932129 |
| iGSE core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, dB = 200 mT, D = 0.2, Vc = 5 cm^3 | SOLVED | ki = 0.199806, Pv = 107121 W/m^3, P = 0.535603 W, ratio_to_sine = 1.05858 |
| iGSE core loss: alpha = 1.4, beta = 2.5, f = 100 kHz, dB = 200 mT, D = 0.5, Vc = 5 cm^3 | NEEDS_INPUT | pe_igse_triangular_core_loss needs k (Steinmetz coefficient (SI units)). Example: solve('pe_igse_triangular_core_loss',  |
| iGSE core loss: k = 3.2, alpha = 1.4, beta = 2.5, f = 100 kHz, dB = 200 mT, D = 0.5, Vc = 5 cm^3, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_skin_effect_round_wire"></a>

### Skin effect in a round wire: `pe_skin_effect_round_wire`

The skin depth and the exact AC-to-DC resistance ratio (Bessel-function solution) of an isolated round conductor at a frequency, with its DC and AC resistance for a length.

$$
\delta = \sqrt{\frac{\rho}{\pi f\mu_0}},\quad \frac{R_{ac}}{R_{dc}} = \mathrm{Re}\left[\frac{kr}{2}\frac{J_0(kr)}{J_1(kr)}\right],\ k = \frac{1-j}{\delta}
$$

| input | meaning | condition |
|---|---|---|
| `d` | wire diameter |  |
| `f` | frequency |  |
| `rho` | conductor resistivity |  |
| `length` | conductor length |  |

| output | meaning |
|---|---|
| `delta` | skin depth |
| `d_over_delta` | diameter in skin depths |
| `Rdc` | DC resistance |
| `Rac` | AC resistance |
| `ratio` | Rac/Rdc |

- **Method:** $\delta = sqrt$(rho/(pi f mu0)); Rac/Rdc = Re[(k r/2) J0(k r)/J1(k r)], $k = (1 - j)/\delta$
- **Assumes:** isolated straight wire (no proximity effect); radius at most 10 skin depths
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![ratio (Rac/Rdc) against d (wire diameter) from 0](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-skin-effect-round-wire-curve)

*ratio (Rac/Rdc) against d (wire diameter) from 0.0001 m to 0.01 m on a logarithmic axis. One curve for each value of f (frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: rho = 1.72e-08 ohm*m, length = 1.0 m. 33 points per curve computed by power-electronics-ai999x; marked: the worked example, d = 0.001 m, ratio = 1.45126.*

```python
power_electronics_ai999x.solve('pe_skin_effect_round_wire', d=0.001, f=100000.0, rho=1.72e-08, length=1.0)
# -> delta = 0.00020873 m, d_over_delta = 4.79088, Rdc = 0.0218997 ohm, Rac = 0.0317823 ohm, ratio = 1.45126   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_skin_effect_round_wire', d=0.0005, f=50000.0, rho=1.72e-08, length=2.0)
# -> delta = 0.000295188 m, d_over_delta = 1.69383, Rdc = 0.175198 ohm, Rac = 0.17706 ohm, ratio = 1.01063   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Skin effect in round wire: d = 1 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, length = 1 m | SOLVED | delta = 0.00020873 m, d_over_delta = 4.79088, Rdc = 0.0218997 ohm, Rac = 0.0317823 ohm, ratio = 1.45126 |
| Skin effect in round wire: d = 0.5 mm, f = 50 kHz, rho = 1.72e-08 ohm*m, length = 2 m | SOLVED | delta = 0.000295188 m, d_over_delta = 1.69383, Rdc = 0.175198 ohm, Rac = 0.17706 ohm, ratio = 1.01063 |
| Skin effect in round wire: f = 100 kHz, rho = 1.72e-08 ohm*m, length = 1 m | NEEDS_INPUT | pe_skin_effect_round_wire needs d (wire diameter). Example: solve('pe_skin_effect_round_wire', d='0.001 m', f='100000 Hz |
| Skin effect in round wire: d = 1 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, length = 1 m, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dowell_proximity_factor"></a>

### Winding AC resistance factor (Dowell): `pe_dowell_proximity_factor`

The ratio of AC to DC resistance of a winding of m layers of foil (or equivalent layer) thickness h at a frequency, including skin and proximity effects by Dowell's one-dimensional solution.

$$
F_R = \Delta\left[\frac{\sinh2\Delta+\sin2\Delta}{\cosh2\Delta-\cos2\Delta} + \frac{2(m^2-1)}{3}\frac{\sinh\Delta-\sin\Delta}{\cosh\Delta+\cos\Delta}\right]
$$

| input | meaning | condition |
|---|---|---|
| `h` | layer (conductor) thickness |  |
| `f` | frequency |  |
| `rho` | conductor resistivity |  |
| `m` | number of layers |  |

| output | meaning |
|---|---|
| `delta` | skin depth |
| `Delta` | layer thickness in skin depths h/delta |
| `FR` | AC-to-DC resistance factor |

- **Method:** $F_R = D$ [ (sinh $2D + sin$ 2D)/(cosh $2D - cos$ 2D) + 2 $(m^{2} - 1)/3$ (sinh D - sin D)/(cosh D + cos D) ], $D = h/\delta$
- **Assumes:** full-width layers (foil or porosity-corrected); sinusoidal current
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![FR (AC-to-DC resistance factor) against h (layer (conductor) thickness) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dowell-proximity-factor-curve)

*FR (AC-to-DC resistance factor) against h (layer (conductor) thickness) from 2.0000e-05 m to 0.002 m on a logarithmic axis. One curve for each value of f (frequency): 100000 Hz, 50000 Hz, 200000 Hz. Other inputs as in the worked example: rho = 1.72e-08 ohm*m, m = 4. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, h = 0.0002 m, FR = 2.43113.*

```python
power_electronics_ai999x.solve('pe_dowell_proximity_factor', h=0.0002, f=100000.0, rho=1.72e-08, m=4)
# -> delta = 0.00020873 m, Delta = 0.958177, FR = 2.43113   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dowell_proximity_factor', h=0.0001, f=200000.0, rho=1.72e-08, m=6)
# -> delta = 0.000147594 m, Delta = 0.677533, FR = 1.83115   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Dowell AC resistance factor: h = 0.2 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, m = 4 | SOLVED | delta = 0.00020873 m, Delta = 0.958177, FR = 2.43113 |
| Dowell AC resistance factor: h = 0.1 mm, f = 200 kHz, rho = 1.72e-08 ohm*m, m = 6 | SOLVED | delta = 0.000147594 m, Delta = 0.677533, FR = 1.83115 |
| Dowell AC resistance factor: f = 100 kHz, rho = 1.72e-08 ohm*m, m = 4 | NEEDS_INPUT | pe_dowell_proximity_factor needs h (layer (conductor) thickness). Example: solve('pe_dowell_proximity_factor', h='0.0002 |
| Dowell AC resistance factor: h = 0.2 mm, f = 100 kHz, rho = 1.72e-08 ohm*m, m = 4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_series_resonant_gain"></a>

### Series resonant converter: tank gain (first harmonic), `pe_series_resonant_gain`

The resonant frequency, characteristic impedance, quality factor, normalised frequency and voltage gain of a series resonant tank driving an equivalent AC load resistance (first-harmonic approximation).

$$
M = \frac{1}{\sqrt{1+Q^2\left(f_n-\frac{1}{f_n}\right)^2}},\quad Q = \frac{\sqrt{L_r/C_r}}{R}
$$

| input | meaning | condition |
|---|---|---|
| `Lr` | resonant inductance |  |
| `Cr` | resonant capacitance |  |
| `R` | equivalent AC load resistance |  |
| `f` | switching frequency |  |

| output | meaning |
|---|---|
| `fr` | resonant frequency |
| `Z0` | characteristic impedance sqrt(Lr/Cr) |
| `Q` | quality factor |
| `fn` | normalised frequency f/fr |
| `M` | tank voltage gain |

- **Method:** fr = 1/(2 pi sqrt(Lr Cr)); $Q = \frac{\sqrt{\frac{Lr}{Cr}}}{R}$; $M = 1/sqrt$($1 + Q^{2}$ (fn - 1/fn)^2)
- **Assumes:** first-harmonic approximation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (tank voltage gain) against f (switching frequency) from 12000 Hz to 1](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-series-resonant-gain-curve)

*M (tank voltage gain) against f (switching frequency) from 12000 Hz to 1.2e+06 Hz on a logarithmic axis. One curve for each value of Lr (resonant inductance): 1.0000e-04 H, 5.0000e-05 H, 0.0002 H. Other inputs as in the worked example: Cr = 2.5000000000000002e-08 F, R = 30.0 ohm. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 120000 Hz, M = 0.801964.*

```python
power_electronics_ai999x.solve('pe_series_resonant_gain', Lr=0.0001, Cr=2.5e-08, R=30.0, f=120000.0)
# -> fr = 100658 Hz, Z0 = 63.2456 ohm, Q = 2.10819, fn = 1.19215, M = 0.801964   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_series_resonant_gain', Lr=5e-05, Cr=1e-07, R=10.0, f=60000.0)
# -> fr = 71176.3 Hz, Z0 = 22.3607 ohm, Q = 2.23607, fn = 0.842978, M = 0.793238   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Series resonant gain: Lr = 100 uH, Cr = 25 nF, R = 30 ohm, f = 120 kHz | SOLVED | fr = 100658 Hz, Z0 = 63.2456 ohm, Q = 2.10819, fn = 1.19215, M = 0.801964 |
| Series resonant gain: Lr = 50 uH, Cr = 100 nF, R = 10 ohm, f = 60 kHz | SOLVED | fr = 71176.3 Hz, Z0 = 22.3607 ohm, Q = 2.23607, fn = 0.842978, M = 0.793238 |
| Series resonant gain: Cr = 25 nF, R = 30 ohm, f = 120 kHz | NEEDS_INPUT | pe_series_resonant_gain needs Lr (resonant inductance). Example: solve('pe_series_resonant_gain', Lr='0.0001 H', Cr='2.5 |
| Series resonant gain: Lr = 100 uH, Cr = 25 nF, R = 30 ohm, f = 120 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_llc_resonant_gain"></a>

### LLC resonant converter: tank gain (first harmonic), `pe_llc_resonant_gain`

The two resonant frequencies, the reflected AC load resistance, quality factor, inductance ratio and voltage gain of an LLC resonant tank at a switching frequency (first-harmonic approximation, full-wave rectified output).

$$
M = \left|\frac{j\omega L_m\parallel R_{ac}}{j\omega L_r + \frac{1}{j\omega C_r} + j\omega L_m\parallel R_{ac}}\right|,\quad R_{ac} = \frac{8n^2R}{\pi^2}
$$

| input | meaning | condition |
|---|---|---|
| `Lr` | series resonant inductance |  |
| `Cr` | resonant capacitance |  |
| `Lm` | magnetizing inductance |  |
| `n` | transformer turns ratio Np/Ns |  |
| `R` | dc load resistance |  |
| `f` | switching frequency |  |

| output | meaning |
|---|---|
| `fr1` | series resonant frequency |
| `fr2` | lower resonant frequency |
| `Rac` | reflected AC load resistance |
| `Q` | quality factor sqrt(Lr/Cr)/Rac |
| `k` | inductance ratio Lm/Lr |
| `M` | tank voltage gain |

- **Method:** Rac = 8 $n^{2}$ $R/pi^{2}$; M = |Zp/(Zs + Zp)|, Zp = jwLm || Rac, Zs = jwLr + 1/(jwCr)
- **Assumes:** first-harmonic approximation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (tank voltage gain) against f (switching frequency) from 10000 Hz to 1e+06 Hz on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-llc-resonant-gain-curve)

*M (tank voltage gain) against f (switching frequency) from 10000 Hz to 1e+06 Hz on a logarithmic axis. One curve for each value of Lm (magnetizing inductance): 0.0003 H, 0.00015 H, 0.0006 H. Other inputs as in the worked example: Lr = 5.9999999999999995e-05 H, Cr = 2.7e-08 F, n = 8.0, R = 2.0 ohm. 41 points per curve computed by power-electronics-ai999x; marked: the worked example, f = 100000 Hz, M = 1.09817.*

```python
power_electronics_ai999x.solve('pe_llc_resonant_gain', Lr=6e-05, Cr=2.7e-08, Lm=0.0003, n=8.0, R=2.0, f=100000.0)
# -> fr1 = 125044 Hz, fr2 = 51049 Hz, Rac = 103.753 ohm, Q = 0.454353, k = 5, M = 1.09817   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_llc_resonant_gain', Lr=2e-05, Cr=1e-07, Lm=0.00012, n=1.0, R=20.0, f=80000.0)
# -> fr1 = 112540 Hz, fr2 = 42535.9 Hz, Rac = 16.2114 ohm, Q = 0.872358, k = 6, M = 0.967265   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| LLC resonant gain: Lr = 60 uH, Cr = 27 nF, Lm = 300 uH, n = 8, R = 2 ohm, f = 100 kHz | SOLVED | fr1 = 125044 Hz, fr2 = 51049 Hz, Rac = 103.753 ohm, Q = 0.454353, k = 5, M = 1.09817 |
| LLC resonant gain: Lr = 20 uH, Cr = 100 nF, Lm = 120 uH, n = 1, R = 20 ohm, f = 80 kHz | SOLVED | fr1 = 112540 Hz, fr2 = 42535.9 Hz, Rac = 16.2114 ohm, Q = 0.872358, k = 6, M = 0.967265 |
| LLC resonant gain: Cr = 27 nF, Lm = 300 uH, n = 8, R = 2 ohm, f = 100 kHz | NEEDS_INPUT | pe_llc_resonant_gain needs Lr (series resonant inductance). Example: solve('pe_llc_resonant_gain', Lr='6e-05 H', Cr='2.7 |
| LLC resonant gain: Lr = 60 uH, Cr = 27 nF, Lm = 300 uH, n = 8, R = 2 ohm, f = 100 kHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_dual_active_bridge_power"></a>

### Dual active bridge: power transfer and inductor current, `pe_dual_active_bridge_power`

The power transferred by a dual-active-bridge converter with single phase-shift modulation, the maximum transferable power, the inductor current at the switching instants of both bridges and its RMS value.

$$
P = \frac{nV_1V_2\,\phi(\pi-\phi)}{2\pi^2f_sL},\quad P_{max} = \frac{nV_1V_2}{8f_sL}
$$

| input | meaning | condition |
|---|---|---|
| `V1` | primary dc voltage |  |
| `V2` | secondary dc voltage |  |
| `n` | turns ratio Np/Ns |  |
| `L` | series (leakage) inductance referred to the primary |  |
| `fs` | switching frequency |  |
| `phi` | phase shift of the secondary bridge |  |

| output | meaning |
|---|---|
| `P` | transferred power |
| `P_max` | maximum transferable power |
| `i0` | inductor current when the primary bridge switches |
| `i_phi` | inductor current when the secondary bridge switches |
| `I_rms` | RMS inductor current |

- **Method:** $P = n$ V1 V2 phi (pi - phi)/(2 $pi^{2}$ fs L); i(0) = -(V1 $pi + nV2$ (2 phi - pi))/(2 w L); i(phi) = i(0) + (V1 + nV2) phi/(w L)
- **Assumes:** square-wave bridges, single phase shift $0 < \phi  \leq  90$ degrees
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P (transferred power) against L (series (leakage) inductance referred to the primary) from 2](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-dual-active-bridge-power-curve)

*P (transferred power) against L (series (leakage) inductance referred to the primary) from 2.5000e-05 H to 7.5000e-05 H. One curve for each value of V1 (primary dc voltage): 400 V, 200 V, 800 V. Other inputs as in the worked example: V2 = 48.0 V, n = 8.0, fs = 100000.0 Hz, phi = 0.5235987755982988 rad. 31 points per curve computed by power-electronics-ai999x; marked: the worked example, L = 5.0000e-05 H, P = 2133.33 W.*

```python
power_electronics_ai999x.solve('pe_dual_active_bridge_power', V1=400.0, V2=48.0, n=8.0, L=5e-05, fs=100000.0, phi=0.5235987755982988)
# -> P = 2133.33 W, P_max = 3840 W, i0 = -7.2 A, i_phi = 5.86667 A, I_rms = 6.1757 A   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_dual_active_bridge_power', V1=200.0, V2=200.0, n=1.0, L=2e-05, fs=50000.0, phi=1.0471975511965976)
# -> P = 4444.44 W, P_max = 5000 W, i0 = -33.3333 A, i_phi = 33.3333 A, I_rms = 29.3972 A   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Dual active bridge power: V1 = 400 V, V2 = 48 V, n = 8, L = 50 uH, fs = 100 kHz, phi = 30 deg | SOLVED | P = 2133.33 W, P_max = 3840 W, i0 = -7.2 A, i_phi = 5.86667 A, I_rms = 6.1757 A |
| Dual active bridge power: V1 = 200 V, V2 = 200 V, n = 1, L = 20 uH, fs = 50 kHz, phi = 60 deg | SOLVED | P = 4444.44 W, P_max = 5000 W, i0 = -33.3333 A, i_phi = 33.3333 A, I_rms = 29.3972 A |
| Dual active bridge power: V2 = 48 V, n = 8, L = 50 uH, fs = 100 kHz, phi = 30 deg | NEEDS_INPUT | pe_dual_active_bridge_power needs V1 (primary dc voltage). Example: solve('pe_dual_active_bridge_power', V1='400 V', V2= |
| Dual active bridge power: V1 = 400 V, V2 = 48 V, n = 8, L = 50 uH, fs = 100 kHz, phi = 30 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_hold_up_capacitor"></a>

### Hold-up (bulk) capacitor: `pe_hold_up_capacitor`

The bulk capacitance that keeps a constant-power load running for a hold-up time while the capacitor voltage falls from V1 to the minimum operating voltage V2, with the energies involved.

$$
C = \frac{2P_ot_{hold}}{V_1^2 - V_2^2}
$$

| input | meaning | condition |
|---|---|---|
| `Po` | load power drawn from the capacitor |  |
| `t_hold` | hold-up time |  |
| `V1` | starting capacitor voltage |  |
| `V2` | minimum operating voltage |  |

| output | meaning |
|---|---|
| `C` | required capacitance |
| `E_used` | energy delivered Po t |
| `E_initial` | energy stored at the starting voltage |
| `E_remaining` | energy left at the minimum voltage |

- **Method:** $C = 2$ Po t_hold/(V1^2 - V2^2)
- **Assumes:** constant-power load; $V_{2} < V_{1}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![C (required capacitance) against V1 (starting capacitor voltage) from 195 V to 585 V](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-hold-up-capacitor-curve)

*C (required capacitance) against V1 (starting capacitor voltage) from 195 V to 585 V. One curve for each value of Po (load power drawn from the capacitor): 300 W, 150 W, 600 W. Other inputs as in the worked example: t_hold = 0.02 s, V2 = 300.0 V. 22 points per curve computed by power-electronics-ai999x; marked: the worked example, V1 = 390 V, C = 0.000193237 F.*

```python
power_electronics_ai999x.solve('pe_hold_up_capacitor', Po=300.0, t_hold=0.02, V1=390.0, V2=300.0)
# -> C = 0.000193237 F, E_used = 6 J, E_initial = 14.6957 J, E_remaining = 8.69565 J   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_hold_up_capacitor', Po=50.0, t_hold=0.01, V1=48.0, V2=36.0)
# -> C = 0.000992063 F, E_used = 0.5 J, E_initial = 1.14286 J, E_remaining = 0.642857 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hold-up capacitor: Po = 300 W, t_hold = 20 ms, V1 = 390 V, V2 = 300 V | SOLVED | C = 0.000193237 F, E_used = 6 J, E_initial = 14.6957 J, E_remaining = 8.69565 J |
| Hold-up capacitor: Po = 50 W, t_hold = 10 ms, V1 = 48 V, V2 = 36 V | SOLVED | C = 0.000992063 F, E_used = 0.5 J, E_initial = 1.14286 J, E_remaining = 0.642857 J |
| Hold-up capacitor: t_hold = 20 ms, V1 = 390 V, V2 = 300 V | NEEDS_INPUT | pe_hold_up_capacitor needs Po (load power drawn from the capacitor). Example: solve('pe_hold_up_capacitor', Po='300 W',  |
| Hold-up capacitor: Po = 300 W, t_hold = 20 ms, V1 = 390 V, V2 = 300 V, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |

<a id="pe_boost_pfc"></a>

### Boost power-factor-correction stage: `pe_boost_pfc`

The peak line current, duty cycle and inductor ripple at the line peak, the largest ripple over the line cycle, the inductor current at the line peak and the second-harmonic output voltage ripple of a continuous-conduction boost PFC stage with unity power factor.

$$
\Delta i_L(v) = \frac{v\,(1-v/V_o)}{Lf_s},\quad \Delta i_{L,max} = \frac{V_o}{4Lf_s},\quad \Delta V_o = \frac{P_o}{2\pi f_{line}CV_o}
$$

| input | meaning | condition |
|---|---|---|
| `Vac` | RMS line voltage |  |
| `f_line` | line frequency |  |
| `Vo` | dc output voltage |  |
| `Po` | output power |  |
| `eff` | efficiency (a fraction or %) |  |
| `L` | boost inductance |  |
| `fs` | switching frequency |  |
| `C` | output (bulk) capacitance |  |

| output | meaning |
|---|---|
| `I_in_pk` | peak line current |
| `D_peak` | duty cycle at the line peak |
| `dI_peak` | inductor ripple at the line peak |
| `dI_max` | largest inductor ripple over the line cycle |
| `IL_at_peak` | peak inductor current at the line peak |
| `dVo_pp` | peak-to-peak second-harmonic output ripple |

- **Method:** Ipk = 2 Po/(eff Vpk); dI(v) = v (1 - v/Vo)/(L fs), largest at $v = Vo/2$; dVo = Po/(2 pi f_line C Vo)
- **Assumes:** unity power factor, continuous conduction; $Vo > sqrt$(2) Vac; small output ripple
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dI_max (largest inductor ripple over the line cycle) against Vac (RMS line voltage), with C = 3](https://huggingface.co/AI999X/power-electronics-ai999x/resolve/main/figures/pe-boost-pfc-di-max-vs-vac.svg)

*dI_max (largest inductor ripple over the line cycle) against Vac (RMS line voltage), with C = 3.30e-4 F, L = 5.00e-4 H, Po = 500 W, Vo = 400 V, eff = 0.95, f_line = 50 Hz, fs = 65000 Hz. 22 points computed by power-electronics-ai999x; marked: Vac = 230, dI_max = 3.07692 A.*

![dI_max (largest inductor ripple over the line cycle) against Vac (RMS line voltage) from 1 V to 460 V on a logarithmic axis](https://www.ai999x.com/?ax_fig=power-electronics-ai999x/pe-boost-pfc-curve)

*dI_max (largest inductor ripple over the line cycle) against Vac (RMS line voltage) from 1 V to 460 V on a logarithmic axis. One curve for each value of L (boost inductance): 0.0005 H, 0.00025 H, 0.001 H. Other inputs as in the worked example: f_line = 50.0 Hz, Vo = 400.0 V, Po = 500.0 W, eff = 0.95, fs = 65000.0 Hz, C = 0.00033 F. 25 points per curve computed by power-electronics-ai999x; marked: the worked example, Vac = 230 V, dI_max = 3.07692 A.*

```python
power_electronics_ai999x.solve('pe_boost_pfc', Vac=230.0, f_line=50.0, Vo=400.0, Po=500.0, eff=0.95, L=0.0005, fs=65000.0, C=0.00033)
# -> I_in_pk = 3.23619 A, D_peak = 0.186827, dI_peak = 1.86982 A, dI_max = 3.07692 A, IL_at_peak = 4.1711 A, dVo_pp = 12.0572 V   (SOLVED)
```

```python
power_electronics_ai999x.solve('pe_boost_pfc', Vac=120.0, f_line=60.0, Vo=390.0, Po=300.0, eff=0.94, L=0.001, fs=100000.0, C=0.00022)
# -> I_in_pk = 3.76121 A, D_peak = 0.564857, dI_peak = 0.958595 A, dI_max = 0.958595 A, IL_at_peak = 4.2405 A, dVo_pp = 9.27476 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Boost PFC stage: Vac = 230 V, f_line = 50 Hz, Vo = 400 V, Po = 500 W, eff = 0.95, L = 500 uH, fs = 65 kHz, C = 330 uF | SOLVED | I_in_pk = 3.23619 A, D_peak = 0.186827, dI_peak = 1.86982 A, dI_max = 3.07692 A, IL_at_peak = 4.1711 A, dVo_pp = 12.0572 V |
| Boost PFC stage: Vac = 120 V, f_line = 60 Hz, Vo = 390 V, Po = 300 W, eff = 0.94, L = 1000 uH, fs = 100 kHz, C = 220 uF | SOLVED | I_in_pk = 3.76121 A, D_peak = 0.564857, dI_peak = 0.958595 A, dI_max = 0.958595 A, IL_at_peak = 4.2405 A, dVo_pp = 9.27476 V |
| Boost PFC stage: f_line = 50 Hz, Vo = 400 V, Po = 500 W, eff = 0.95, L = 500 uH, fs = 65 kHz, C = 330 uF | NEEDS_INPUT | pe_boost_pfc needs Vac (RMS line voltage). Example: solve('pe_boost_pfc', Vac='230 V', f_line='50 Hz', Vo='400 V', Po='5 |
| Boost PFC stage: Vac = 230 V, f_line = 50 Hz, Vo = 400 V, Po = 500 W, eff = 0.95, L = 500 uH, fs = 65 kHz, C = 330 uF, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: power_electronics_ai999x.template |
