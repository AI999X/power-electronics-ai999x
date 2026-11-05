//! power_electronics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/power_electronics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "pe_ac_voltage_controller",
    "pe_bldc_six_step",
    "pe_boost_ccm",
    "pe_boost_dcm",
    "pe_boost_nonideal_gain",
    "pe_boost_output_ripple",
    "pe_boost_pfc",
    "pe_bootstrap_capacitor",
    "pe_bridge_converter",
    "pe_buck_boost_ccm",
    "pe_buck_boost_dcm",
    "pe_buck_ccm",
    "pe_buck_component_currents",
    "pe_buck_dcm",
    "pe_buck_lc_design",
    "pe_buck_output_ripple",
    "pe_buck_with_losses",
    "pe_capacitor_filter_exact",
    "pe_conversion_ratio",
    "pe_converter_small_signal",
    "pe_critical_inductance",
    "pe_cuk_ccm",
    "pe_dc_drive_controlled_rectifier",
    "pe_dc_motor_armature_ripple",
    "pe_dc_motor_chopper_drive",
    "pe_dead_time_error",
    "pe_diode_reverse_recovery",
    "pe_dowell_proximity_factor",
    "pe_dual_active_bridge_power",
    "pe_flyback_ccm",
    "pe_flyback_dcm_design",
    "pe_forward_ccm",
    "pe_foster_periodic_pulses",
    "pe_foster_transient_thermal",
    "pe_gapped_inductor_design",
    "pe_gate_drive_power",
    "pe_half_wave_rl_rectifier",
    "pe_hold_up_capacitor",
    "pe_igbt_inverter_conduction_losses",
    "pe_igse_triangular_core_loss",
    "pe_induction_motor_vf",
    "pe_inductor_area_product",
    "pe_interleaved_buck_ripple",
    "pe_inverter_rl_load_current",
    "pe_llc_resonant_gain",
    "pe_mosfet_conduction_loss",
    "pe_parallel_mosfet_sharing",
    "pe_peak_current_mode_slope",
    "pe_pmsm_dq_model",
    "pe_pmsm_field_weakening",
    "pe_pmsm_mtpa",
    "pe_power_factor_distortion",
    "pe_pulse_waveform_rms",
    "pe_quasi_square_inverter",
    "pe_rc_snubber_ringing",
    "pe_rectifier_line_current_pf",
    "pe_rectifier_line_harmonics",
    "pe_regenerative_braking_chopper",
    "pe_sepic_ccm",
    "pe_series_resonant_gain",
    "pe_single_phase_controlled_rectifier",
    "pe_single_phase_overlap",
    "pe_six_step_inverter",
    "pe_skin_effect_round_wire",
    "pe_space_vector_times",
    "pe_spwm_fundamental",
    "pe_square_wave_inverter",
    "pe_steinmetz_core_loss",
    "pe_switching_loss",
    "pe_thd_from_harmonics",
    "pe_thermal_shared_heatsink",
    "pe_three_phase_controlled_rectifier",
    "pe_three_phase_diode_bridge",
    "pe_three_phase_half_wave_rectifier",
    "pe_three_phase_overlap",
    "pe_transformer_area_product",
    "pe_transformer_turns",
    "pe_turnoff_snubber",
    "pe_zvs_transition",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "pe_ac_voltage_controller" => {
            let r = f_ac_voltage_controller(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_22(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_bldc_six_step" => {
            let r = f_bldc_six_step(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rph") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Keph") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_78(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_boost_ccm" => {
            let r = f_boost_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_126(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_boost_dcm" => {
            let r = f_boost_dcm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_178(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_boost_nonideal_gain" => {
            let r = f_boost_nonideal(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_230(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_boost_output_ripple" => {
            let r = f_boost_output_ripple(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_276(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_boost_pfc" => {
            let r = f_pfc_boost(match v.get("Vac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_line") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_330(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_bootstrap_capacitor" => {
            let r = f_bootstrap(match v.get("Qg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ileak") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Dmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dV") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_bridge_converter" => {
            let r = f_bridge_converter(match v.get("topology") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_440(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_boost_ccm" => {
            let r = f_buck_boost_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_486(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_boost_dcm" => {
            let r = f_buck_boost_dcm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_538(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_ccm" => {
            let r = f_buck_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_component_currents" => {
            let r = f_buck_currents(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_634(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_dcm" => {
            let r = f_buck_dcm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_686(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_lc_design" => {
            let r = f_buck_lc_design(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dV") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_735(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_output_ripple" => {
            let r = f_buck_output_ripple(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_781(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_buck_with_losses" => {
            let r = f_buck_losses(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tsw") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_capacitor_filter_exact" => {
            let r = f_capacitor_filter_exact(match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_conversion_ratio" => {
            let r = f_conversion_ratio(match v.get("topology") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_converter_small_signal" => {
            let r = f_small_signal(match v.get("topology") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1012(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_critical_inductance" => {
            let r = f_critical_inductance(match v.get("topology") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1066(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_cuk_ccm" => {
            let r = f_cuk_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dc_drive_controlled_rectifier" => {
            let r = f_dc_drive_rectifier(match v.get("converter") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ia") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1168(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dc_motor_armature_ripple" => {
            let r = f_armature_ripple(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("La") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ea") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1226(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dc_motor_chopper_drive" => {
            let r = f_dc_motor_chopper(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dead_time_error" => {
            let r = f_dead_time_error(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("td") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1332(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_diode_reverse_recovery" => {
            let r = f_reverse_recovery(match v.get("Irm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("trr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1376(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dowell_proximity_factor" => {
            let r = f_dowell(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_dual_active_bridge_power" => {
            let r = f_dab_power(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_flyback_ccm" => {
            let r = f_flyback_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1519(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_flyback_dcm_design" => {
            let r = f_flyback_dcm_design(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1569(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_forward_ccm" => {
            let r = f_forward_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1617(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_foster_periodic_pulses" => {
            let r = f_foster_periodic(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1674(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_foster_transient_thermal" => {
            let r = f_foster_pulse(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1729(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_gapped_inductor_design" => {
            let r = f_gapped_inductor(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ipk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Bmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ac") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1773(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_gate_drive_power" => {
            let r = f_gate_drive(match v.get("Qg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vdrv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_half_wave_rl_rectifier" => {
            let r = f_half_wave_rl(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_hold_up_capacitor" => {
            let r = f_hold_up(match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_hold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1916(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_igbt_inverter_conduction_losses" => {
            let r = f_igbt_losses(match v.get("Ip") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vce0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rce") { Some(x) => py::from_val(x), None => { return None } }, match v.get("VF0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1972(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_igse_triangular_core_loss" => {
            let r = f_igse_triangular(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2022(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_induction_motor_vf" => {
            let r = f_induction_vf(match v.get("V_rated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f_rated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("poles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_rated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2080(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_inductor_area_product" => {
            let r = f_inductor_area_product(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ipk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Irms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Bmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ku") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_interleaved_buck_ripple" => {
            let r = f_interleaved_buck(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2182(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_inverter_rl_load_current" => {
            let r = f_inverter_rl(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2226(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_llc_resonant_gain" => {
            let r = f_llc_gain(match v.get("Lr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2270(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_mosfet_conduction_loss" => {
            let r = f_mosfet_conduction(match v.get("Rds25") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Irms") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2322(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_parallel_mosfet_sharing" => {
            let r = f_parallel_sharing(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_peak_current_mode_slope" => {
            let r = f_peak_current_mode(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_pmsm_dq_model" => {
            let r = f_pmsm_dq(match v.get("Rs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ld") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("id") { Some(x) => py::from_val(x), None => { return None } }, match v.get("iq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega_m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2493(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_pmsm_field_weakening" => {
            let r = f_field_weakening(match v.get("Ls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Imax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2547(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_pmsm_mtpa" => {
            let r = f_mtpa(match v.get("Ld") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Is") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_power_factor_distortion" => {
            let r = f_pf_distortion(match v.get("THD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2646(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_pulse_waveform_rms" => {
            let r = f_pulse_rms(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2696(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_quasi_square_inverter" => {
            let r = f_quasi_square(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2752(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_rc_snubber_ringing" => {
            let r = f_rc_snubber_ringing(match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cadd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2798(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_rectifier_line_current_pf" => {
            let r = f_line_current_pf(match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Id") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2855(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_rectifier_line_harmonics" => {
            let r = f_line_current_harmonics(match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Id") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2915(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_regenerative_braking_chopper" => {
            let r = f_regen_chopper(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2967(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_sepic_ccm" => {
            let r = f_sepic_ccm(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vout") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Io") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3014(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_series_resonant_gain" => {
            let r = f_series_resonant(match v.get("Lr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3058(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_single_phase_controlled_rectifier" => {
            let r = f_single_phase_controlled(match v.get("circuit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_single_phase_overlap" => {
            let r = f_single_overlap(match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Id") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3164(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_six_step_inverter" => {
            let r = f_six_step(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3208(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_skin_effect_round_wire" => {
            let r = f_skin_effect(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3260(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_space_vector_times" => {
            let r = f_space_vector(match v.get("Vref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3311(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_spwm_fundamental" => {
            let r = f_spwm(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3365(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_square_wave_inverter" => {
            let r = f_square_wave_inverter(match v.get("Vdc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bridge") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3420(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_steinmetz_core_loss" => {
            let r = f_steinmetz(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Bpk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3464(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_switching_loss" => {
            let r = f_switching_loss(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Coss") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3514(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_thd_from_harmonics" => {
            let r = f_thd_harmonics(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3570(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_thermal_shared_heatsink" => {
            let r = f_thermal_multi(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rjc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rcs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rsa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3627(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_three_phase_controlled_rectifier" => {
            let r = f_three_phase_controlled(match v.get("VLL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Id") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_three_phase_diode_bridge" => {
            let r = f_three_phase_bridge(match v.get("VLL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3722(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_three_phase_half_wave_rectifier" => {
            let r = f_three_phase_half_wave(match v.get("Vph") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3766(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_three_phase_overlap" => {
            let r = f_three_overlap(match v.get("VLL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ls") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Id") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3815(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_transformer_area_product" => {
            let r = f_transformer_area_product(match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eff") { Some(x) => py::from_val(x), None => { return None } }, match v.get("waveform") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Bm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ku") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3869(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_transformer_turns" => {
            let r = f_transformer_turns(match v.get("waveform") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Bmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ac") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3917(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_turnoff_snubber" => {
            let r = f_turnoff_snubber(match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3961(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pe_zvs_transition" => {
            let r = f_zvs_transition(match v.get("Vin") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ceq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_MU0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.25663706127e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SQ2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.4142135623730951)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_SQ3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.7320508075688772)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0C() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(273.15)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_TWO_PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.283185307179586)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BR2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("half-bridge"), Py::str("full-bridge")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BRIDGE3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("push-pull"), Py::str("half-bridge"), Py::str("full-bridge")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__HF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("half-wave"), Py::str("full-wave")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PH2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("single-phase"), Py::str("three-phase")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RPM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.549296585513721)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SPC() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("half-wave"), Py::str("full-bridge"), Py::str("semi-converter")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TOPO3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("buck"), Py::str("boost"), Py::str("buck-boost")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TOPO5() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("buck"), Py::str("boost"), Py::str("buck-boost"), Py::str("Cuk"), Py::str("SEPIC")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WAVE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("sine"), Py::str("square")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bessel01(mut v_z: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_t1: Py = Py::None;
    v_q = py::truediv(&py::neg(&py::mul(&v_z.clone(), &v_z.clone())?)?, &Py::Float(4.0))?;
    v_t0 = py::complex(&[Py::Float(1.0), Py::Float(0.0)])?;
    v_t1 = py::truediv(&v_z.clone(), &Py::Float(2.0))?;
    let t3242 = (Py::tuple(vec![v_t0.clone(), v_t1.clone()])).items()?;
    if t3242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t3242[0].clone();
    v_s1 = t3242[1].clone();
    v_m = Py::Int(0i64);
    while ({ let t3245 = v_m.clone(); Py::Bool({ let t3246 = Py::Int(200i64); if py::lt(&t3245, &t3246)? { true } else { false } }) }).truthy() {
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
        v_t0 = py::truediv(&py::mul(&v_t0.clone(), &v_q.clone())?, &py::mul(&v_m.clone(), &v_m.clone())?)?;
        v_t1 = py::truediv(&py::mul(&v_t1.clone(), &v_q.clone())?, &py::mul(&v_m.clone(), &py::add(&v_m.clone(), &Py::Int(1i64))?)?)?;
        v_s0 = py::iadd(&v_s0, &v_t0.clone())?;
        v_s1 = py::iadd(&v_s1, &v_t1.clone())?;
        if ({ let t3243 = py::add(&py::add(&py::add(&py::abs(&py::real(&v_t0.clone())?)?, &py::abs(&py::imag(&v_t0.clone())?)?)?, &py::abs(&py::real(&v_t1.clone())?)?)?, &py::abs(&py::imag(&v_t1.clone())?)?)?; Py::Bool({ let t3244 = py::mul(&Py::Float(1e-18), &py::add(&py::add(&py::add(&py::abs(&py::real(&v_s0.clone())?)?, &py::abs(&py::imag(&v_s0.clone())?)?)?, &py::abs(&py::real(&v_s1.clone())?)?)?, &py::abs(&py::imag(&v_s1.clone())?)?)?)?; if py::lt(&t3243, &t3244)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(Py::tuple(vec![v_s0.clone(), v_s1.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ccm_ok(mut v_imin: Py, mut v_scale: Py) -> R {
    let _ = f__need({ let t3000 = v_imin.clone(); Py::Bool({ let t3001 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t3000, &t3001)? { true } else { false } }) }, Py::str("the inductor current would reach zero: the converter is in discontinuous conduction (use the discontinuous-mode capability)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dcm_common(mut v_Vin: Py, mut v_L: Py, mut v_fs: Py, mut v_R: Py, mut v_D: Py) -> R {
    let _ = f__pos(Py::str("Vin, L, fs and R"), Py::tuple(vec![v_Vin.clone(), v_L.clone(), v_fs.clone(), v_R.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    return Ok(py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_L.clone())?, &v_fs.clone())?, &v_R.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t2473 in py::iter(&(v_vals.clone()))? {
        v_v = t2473;
        let _ = f__need({ let t2475 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2475.truthy() { t2475 } else { { let t2474 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2474.truthy() { t2474 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__foster_lists(mut v_R: Py, mut v_tau: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t1707 = Py::Bool(matches!(v_R.clone(), Py::List(_))); if !t1707.truthy() { t1707 } else { { let t1706 = Py::Bool(matches!(v_tau.clone(), Py::List(_))); if !t1706.truthy() { t1706 } else { { let t1705 = { let t1700 = Py::Int(1i64); Py::Bool({ let t1701 = py::len(&v_R.clone())?; if py::le(&t1700, &t1701)? { { let t1702 = Py::Int(6i64); if py::le(&t1701, &t1702)? { true } else { false } } } else { false } }) }; if !t1705.truthy() { t1705 } else { { let t1703 = py::len(&v_R.clone())?; Py::Bool({ let t1704 = py::len(&v_tau.clone())?; if py::eq(&t1703, &t1704) { true } else { false } }) } } } } } } }, Py::str("give 1 to 6 thermal resistances and as many time constants"))?;
    for t1708 in py::iter(&(v_R.clone()))? {
        v_x = t1708;
        let _ = f__pos(Py::str("every thermal resistance"), Py::tuple(vec![v_x.clone()]))?;
    }
    for t1715 in py::iter(&(v_tau.clone()))? {
        v_x = t1715;
        let _ = f__pos(Py::str("every time constant"), Py::tuple(vec![v_x.clone()]))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_x: Py) -> R {
    let _ = f__need({ let t3852 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3852.truthy() { t3852 } else { { let t3851 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t3851.truthy() { t3851 } else { { let t3850 = py::math1("isfinite", &v_x.clone())?; if !t3850.truthy() { t3850 } else { { let t3847 = Py::Int(0i64); Py::Bool({ let t3848 = v_x.clone(); if py::lt(&t3847, &t3848)? { { let t3849 = Py::Int(1i64); if py::lt(&t3848, &t3849)? { true } else { false } } } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must lie strictly between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__igse_int(mut v_alpha: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &py::math1("sqrt", &Py::Float(std::f64::consts::PI))?)?, &py::math1("gamma", &py::truediv(&py::add(&v_alpha.clone(), &Py::Float(1.0))?, &Py::Float(2.0))?)?)?, &py::math1("gamma", &py::add(&py::truediv(&v_alpha.clone(), &Py::Float(2.0))?, &Py::Float(1.0))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kf(mut v_waveform: Py) -> R {
    return Ok((if ({ let t3903 = v_waveform.clone(); Py::Bool({ let t3904 = Py::str("sine"); if py::eq(&t3903, &t3904) { true } else { false } }) }).truthy() { py::mul(&Py::Float(std::f64::consts::PI), &c_SQ2()?)? } else { Py::Float(4.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3601 in py::iter(&(v_vals.clone()))? {
        v_v = t3601;
        let _ = f__need({ let t3606 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3606.truthy() { t3606 } else { { let t3605 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3605.truthy() { t3605 } else { { let t3604 = py::math1("isfinite", &v_v.clone())?; if !t3604.truthy() { t3604 } else { { let t3602 = v_v.clone(); Py::Bool({ let t3603 = Py::Int(0i64); if py::ge(&t3602, &t3603)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t3995 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3996 = (t3995).items()?;
if t3996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3996[0].clone();
v_v = t3996[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3987 in py::iter(&(v_vals.clone()))? {
        v_v = t3987;
        let _ = f__need({ let t3992 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3992.truthy() { t3992 } else { { let t3991 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3991.truthy() { t3991 } else { { let t3990 = py::math1("isfinite", &v_v.clone())?; if !t3990.truthy() { t3990 } else { { let t3988 = v_v.clone(); Py::Bool({ let t3989 = Py::Int(0i64); if py::gt(&t3988, &t3989)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t4000 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t4000.truthy() { t4000 } else { { let t3999 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t3999.truthy() { t3999 } else { { let t3997 = v_x.clone(); Py::Bool({ let t3998 = Py::None; if py::is(&t3997, &t3998) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4001 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4001;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4004 = { let t4002 = v_x.clone(); Py::Bool({ let t4003 = Py::Int(0i64); if py::eq(&t4002, &t4003) { true } else { false } }) }; if t4004.truthy() { t4004 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4005 = v_x.clone(); Py::Bool({ let t4006 = Py::Int(0i64); if py::eq(&t4005, &t4006) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t4013 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4013.truthy() { t4013 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4017 = py::not(&{ let t4014 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4014.truthy() { t4014 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4017.truthy() { t4017 } else { { let t4015 = py::len(&v_a.clone())?; Py::Bool({ let t4016 = py::len(&v_b.clone())?; if py::ne(&t4015, &t4016) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4018 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4019 = (t4018).items()?;
if t4019.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4019[0].clone();
v_y = t4019[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t4020 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4020.truthy() { t4020 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4021 = v_a.clone(); Py::Bool({ let t4022 = v_b.clone(); if py::eq(&t4021, &t4022) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t4024: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4023 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4023[0].clone();
            v_b = t4023[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4024 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4025 = v_d.clone(); Py::Bool({ let t4026 = Py::Int(0i64); if py::eq(&t4025, &t4026) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4031 = { let t4027 = py::abs(&v_a.clone())?; Py::Bool({ let t4028 = Py::Float(1e-15); if py::lt(&t4027, &t4028)? { true } else { false } }) }; if !t4031.truthy() { t4031 } else { { let t4029 = py::abs(&v_b.clone())?; Py::Bool({ let t4030 = Py::Float(1e-15); if py::lt(&t4029, &t4030)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t3662 = py::abs(&v_x.clone())?; Py::Bool({ let t3663 = py::mul(&Py::Float(1e-12), &py::abs(&v_scale.clone())?)?; if py::le(&t3662, &t3663)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ac_voltage_controller(mut v_Vs: Py, mut v_alpha: Py, mut v_R: Py) -> R {
    let mut v_Io: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    let _ = f__pos(Py::str("Vs and R"), Py::tuple(vec![v_Vs.clone(), v_R.clone()]))?;
    let _ = f__need({ let t7 = Py::Int(0i64); Py::Bool({ let t8 = v_alpha.clone(); if py::le(&t7, &t8)? { { let t9 = Py::Float(std::f64::consts::PI); if py::lt(&t8, &t9)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Vo = py::mul(&v_Vs.clone(), &py::math1("sqrt", &py::add(&py::sub(&Py::Float(1.0), &py::truediv(&v_alpha.clone(), &Py::Float(std::f64::consts::PI))?)?, &py::truediv(&py::math1("sin", &py::mul(&Py::Float(2.0), &v_alpha.clone())?)?, &c_TWO_PI()?)?)?)?)?;
    v_Io = py::truediv(&v_Vo.clone(), &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vo_rms"), v_Vo.clone()), (Py::str("Io_rms"), v_Io.clone()), (Py::str("P"), py::truediv(&py::mul(&v_Vo.clone(), &v_Vo.clone())?, &v_R.clone())?), (Py::str("PF"), py::truediv(&v_Vo.clone(), &v_Vs.clone())?), (Py::str("I_scr_avg"), py::truediv(&py::mul(&py::mul(&c_SQ2()?, &v_Vs.clone())?, &py::add(&Py::Float(1.0), &py::math1("cos", &v_alpha.clone())?)?)?, &py::mul(&c_TWO_PI()?, &v_R.clone())?)?), (Py::str("I_scr_rms"), py::truediv(&v_Io.clone(), &c_SQ2()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_armature_ripple(mut v_Vs: Py, mut v_D: Py, mut v_fs: Py, mut v_Ra: Py, mut v_La: Py, mut v_Ea: Py) -> R {
    let mut v_Imax: Py = Py::None;
    let mut v_Imin: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("Vs, fs, Ra and La"), Py::tuple(vec![v_Vs.clone(), v_fs.clone(), v_Ra.clone(), v_La.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__nonneg(Py::str("the back EMF Ea"), Py::tuple(vec![v_Ea.clone()]))?;
    v_T = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_tau = py::truediv(&v_La.clone(), &v_Ra.clone())?;
    v_Imax = py::sub(&py::truediv(&py::mul(&py::truediv(&v_Vs.clone(), &v_Ra.clone())?, &py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_D.clone())?, &v_T.clone())?, &v_tau.clone())?)?)?)?, &py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&v_T.clone())?, &v_tau.clone())?)?)?)?, &py::truediv(&v_Ea.clone(), &v_Ra.clone())?)?;
    v_Imin = py::sub(&py::truediv(&py::mul(&py::truediv(&v_Vs.clone(), &v_Ra.clone())?, &py::sub(&py::math1("exp", &py::truediv(&py::mul(&v_D.clone(), &v_T.clone())?, &v_tau.clone())?)?, &Py::Float(1.0))?)?, &py::sub(&py::math1("exp", &py::truediv(&v_T.clone(), &v_tau.clone())?)?, &Py::Float(1.0))?)?, &py::truediv(&v_Ea.clone(), &v_Ra.clone())?)?;
    let _ = f__need({ let t1212 = v_Imin.clone(); Py::Bool({ let t1213 = Py::Int(0i64); if py::gt(&t1212, &t1213)? { true } else { false } }) }, Py::str("the armature current falls to zero: discontinuous conduction"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("I_max"), v_Imax.clone()), (Py::str("I_min"), v_Imin.clone()), (Py::str("dI"), py::sub(&v_Imax.clone(), &v_Imin.clone())?), (Py::str("I_avg"), py::truediv(&py::sub(&py::mul(&v_D.clone(), &v_Vs.clone())?, &v_Ea.clone())?, &v_Ra.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bldc_six_step(mut v_Vdc: Py, mut v_D: Py, mut v_Rph: Py, mut v_Keph: Py, mut v_I: Py, mut v_p: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_fe: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("Vdc, Rph, Keph and I"), Py::tuple(vec![v_Vdc.clone(), v_Rph.clone(), v_Keph.clone(), v_I.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__need({ let t63 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t63.truthy() { t63 } else { { let t60 = Py::Int(1i64); Py::Bool({ let t61 = v_p.clone(); if py::le(&t60, &t61)? { { let t62 = Py::Int(50i64); if py::le(&t61, &t62)? { true } else { false } } } else { false } }) } } }, Py::str("the number of pole pairs p must be a whole number from 1 to 50"))?;
    v_V = py::mul(&v_D.clone(), &v_Vdc.clone())?;
    v_w = py::truediv(&py::sub(&v_V.clone(), &py::mul(&py::mul(&Py::Float(2.0), &v_Rph.clone())?, &v_I.clone())?)?, &py::mul(&Py::Float(2.0), &v_Keph.clone())?)?;
    let _ = f__need({ let t64 = v_w.clone(); Py::Bool({ let t65 = Py::Int(0i64); if py::gt(&t64, &t65)? { true } else { false } }) }, Py::str("the winding drop exceeds the applied voltage: the motor stalls at this current"))?;
    v_fe = py::truediv(&py::mul(&v_p.clone(), &v_w.clone())?, &c_TWO_PI()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("T"), py::mul(&py::mul(&Py::Float(2.0), &v_Keph.clone())?, &v_I.clone())?), (Py::str("omega_no_load"), py::truediv(&v_V.clone(), &py::mul(&Py::Float(2.0), &v_Keph.clone())?)?), (Py::str("I_stall"), py::truediv(&v_V.clone(), &py::mul(&Py::Float(2.0), &v_Rph.clone())?)?), (Py::str("f_e"), v_fe.clone()), (Py::str("t_commutation"), py::truediv(&Py::Float(1.0), &py::mul(&Py::Float(6.0), &v_fe.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boost_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, L, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_fs.clone(), v_Io.clone()]))?;
    let _ = f__need({ let t110 = v_Vout.clone(); Py::Bool({ let t111 = v_Vin.clone(); if py::gt(&t110, &t111)? { true } else { false } }) }, Py::str("a boost converter needs Vout > Vin"))?;
    v_D = py::sub(&Py::Float(1.0), &py::truediv(&v_Vin.clone(), &v_Vout.clone())?)?;
    v_IL = py::truediv(&py::mul(&v_Io.clone(), &v_Vout.clone())?, &v_Vin.clone())?;
    v_dI = py::truediv(&py::mul(&v_Vin.clone(), &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    let _ = f__ccm_ok(py::sub(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_IL.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("IL"), v_IL.clone()), (Py::str("dIL"), v_dI.clone()), (Py::str("IL_max"), py::add(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("IL_min"), py::sub(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("V_switch"), v_Vout.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boost_dcm(mut v_Vin: Py, mut v_L: Py, mut v_fs: Py, mut v_R: Py, mut v_D: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    v_K = f__dcm_common(v_Vin.clone(), v_L.clone(), v_fs.clone(), v_R.clone(), v_D.clone())?;
    let _ = f__need({ let t164 = v_K.clone(); Py::Bool({ let t165 = py::mul(&v_D.clone(), &py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &Py::Int(2i64))?)?; if py::lt(&t164, &t165)? { true } else { false } }) }, Py::str("K = 2L/(R Ts) >= D (1 - D)^2: the converter is in continuous conduction (use the CCM capability)"))?;
    v_M = py::truediv(&py::add(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_D.clone())?, &v_D.clone())?, &v_K.clone())?)?)?)?, &Py::Float(2.0))?;
    v_Vo = py::mul(&v_M.clone(), &v_Vin.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("M"), v_M.clone()), (Py::str("Vout"), v_Vo.clone()), (Py::str("D2"), py::truediv(&v_D.clone(), &py::sub(&v_M.clone(), &Py::Float(1.0))?)?), (Py::str("I_pk"), py::truediv(&py::mul(&v_Vin.clone(), &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?), (Py::str("Io"), py::truediv(&v_Vo.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boost_nonideal(mut v_Vin: Py, mut v_D: Py, mut v_R: Py, mut v_RL: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let _ = f__pos(Py::str("Vin, R and RL"), Py::tuple(vec![v_Vin.clone(), v_R.clone(), v_RL.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__need({ let t216 = v_RL.clone(); Py::Bool({ let t217 = v_R.clone(); if py::lt(&t216, &t217)? { true } else { false } }) }, Py::str("the inductor resistance must be below the load resistance"))?;
    v_a = py::truediv(&v_RL.clone(), &py::mul(&py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &Py::Int(2i64))?, &v_R.clone())?)?;
    v_eta = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_a.clone())?)?;
    v_M = py::truediv(&v_eta.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?;
    v_Vo = py::mul(&v_M.clone(), &v_Vin.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("Vout"), v_Vo.clone()), (Py::str("efficiency"), v_eta.clone()), (Py::str("IL"), py::truediv(&v_Vo.clone(), &py::mul(&v_R.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?)?), (Py::str("D_peak"), py::sub(&Py::Float(1.0), &py::math1("sqrt", &py::truediv(&v_RL.clone(), &v_R.clone())?)?)?), (Py::str("M_max"), py::mul(&Py::Float(0.5), &py::math1("sqrt", &py::truediv(&v_R.clone(), &v_RL.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boost_output_ripple(mut v_Vin: Py, mut v_Vout: Py, mut v_Io: Py, mut v_C: Py, mut v_fs: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, Io, C and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_Io.clone(), v_C.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t262 = v_Vout.clone(); Py::Bool({ let t263 = v_Vin.clone(); if py::gt(&t262, &t263)? { true } else { false } }) }, Py::str("a boost converter needs Vout > Vin"))?;
    v_D = py::sub(&Py::Float(1.0), &py::truediv(&v_Vin.clone(), &v_Vout.clone())?)?;
    v_dV = py::truediv(&py::mul(&v_Io.clone(), &v_D.clone())?, &py::mul(&v_C.clone(), &v_fs.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("dVo"), v_dV.clone()), (Py::str("dVo_pct"), py::truediv(&py::mul(&Py::Float(100.0), &v_dV.clone())?, &v_Vout.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bootstrap(mut v_Qg: Py, mut v_Ileak: Py, mut v_fs: Py, mut v_Dmax: Py, mut v_dV: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::str("Qg, fs and dV"), Py::tuple(vec![v_Qg.clone(), v_fs.clone(), v_dV.clone()]))?;
    let _ = f__nonneg(Py::str("the leakage and quiescent current"), Py::tuple(vec![v_Ileak.clone()]))?;
    let _ = f__frac(Py::str("the maximum duty cycle"), v_Dmax.clone())?;
    v_Q = py::add(&v_Qg.clone(), &py::truediv(&py::mul(&v_Ileak.clone(), &v_Dmax.clone())?, &v_fs.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q_total"), v_Q.clone()), (Py::str("C_min"), py::truediv(&v_Q.clone(), &v_dV.clone())?), (Py::str("C_recommended"), py::truediv(&py::mul(&Py::Float(10.0), &v_Q.clone())?, &v_dV.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bridge_converter(mut v_topology: Py, mut v_Vin: Py, mut v_Vout: Py, mut v_n: Py, mut v_L: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Vsec: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_vsw: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, n, L, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_n.clone(), v_L.clone(), v_fs.clone(), v_Io.clone()]))?;
    let _ = f__need({ let t418 = v_topology.clone(); Py::Bool({ let t419 = c__BRIDGE3()?; if py::contains(&t419, &t418)? { true } else { false } }) }, Py::str("topology: push-pull, half-bridge or full-bridge"))?;
    v_Vsec = (if ({ let t420 = v_topology.clone(); Py::Bool({ let t421 = Py::str("half-bridge"); if py::ne(&t420, &t421) { true } else { false } }) }).truthy() { py::truediv(&v_Vin.clone(), &v_n.clone())? } else { py::truediv(&v_Vin.clone(), &py::mul(&Py::Float(2.0), &v_n.clone())?)? });
    v_D = py::truediv(&v_Vout.clone(), &py::mul(&Py::Float(2.0), &v_Vsec.clone())?)?;
    let _ = f__need({ let t422 = v_D.clone(); Py::Bool({ let t423 = Py::Float(0.5); if py::lt(&t422, &t423)? { true } else { false } }) }, Py::str("the output voltage needs a duty cycle of 50 % or more per switch, which is not possible"))?;
    v_dI = py::truediv(&py::mul(&py::sub(&v_Vsec.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    v_vsw = (if ({ let t426 = v_topology.clone(); Py::Bool({ let t427 = Py::str("push-pull"); if py::eq(&t426, &t427) { true } else { false } }) }).truthy() { py::mul(&Py::Float(2.0), &v_Vin.clone())? } else { v_Vin.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("V_switch"), v_vsw.clone()), (Py::str("f_ripple"), py::mul(&Py::Float(2.0), &v_fs.clone())?), (Py::str("dIL"), v_dI.clone()), (Py::str("IL_min"), py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_boost_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_IL: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout (magnitude), L, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_fs.clone(), v_Io.clone()]))?;
    v_D = py::truediv(&v_Vout.clone(), &py::add(&v_Vin.clone(), &v_Vout.clone())?)?;
    v_IL = py::truediv(&v_Io.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?;
    v_dI = py::truediv(&py::mul(&v_Vin.clone(), &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    let _ = f__ccm_ok(py::sub(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_IL.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("IL"), v_IL.clone()), (Py::str("dIL"), v_dI.clone()), (Py::str("IL_max"), py::add(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("IL_min"), py::sub(&v_IL.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("V_switch"), py::add(&v_Vin.clone(), &v_Vout.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_boost_dcm(mut v_Vin: Py, mut v_L: Py, mut v_fs: Py, mut v_R: Py, mut v_D: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    v_K = f__dcm_common(v_Vin.clone(), v_L.clone(), v_fs.clone(), v_R.clone(), v_D.clone())?;
    let _ = f__need({ let t524 = v_K.clone(); Py::Bool({ let t525 = py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &Py::Int(2i64))?; if py::lt(&t524, &t525)? { true } else { false } }) }, Py::str("K = 2L/(R Ts) >= (1 - D)^2: the converter is in continuous conduction (use the CCM capability)"))?;
    v_M = py::truediv(&v_D.clone(), &py::math1("sqrt", &v_K.clone())?)?;
    v_Vo = py::mul(&v_M.clone(), &v_Vin.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("M"), v_M.clone()), (Py::str("Vout"), v_Vo.clone()), (Py::str("D2"), py::math1("sqrt", &v_K.clone())?), (Py::str("I_pk"), py::truediv(&py::mul(&v_Vin.clone(), &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?), (Py::str("Io"), py::truediv(&v_Vo.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, L, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_fs.clone(), v_Io.clone()]))?;
    let _ = f__need({ let t570 = v_Vout.clone(); Py::Bool({ let t571 = v_Vin.clone(); if py::lt(&t570, &t571)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
    v_dI = py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("dIL"), v_dI.clone()), (Py::str("IL_max"), py::add(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("IL_min"), py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("ripple_ratio"), py::truediv(&v_dI.clone(), &v_Io.clone())?), (Py::str("Iin"), py::mul(&v_D.clone(), &v_Io.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_currents(mut v_Vin: Py, mut v_Vout: Py, mut v_Io: Py, mut v_L: Py, mut v_fs: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, Io, L and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_Io.clone(), v_L.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t618 = v_Vout.clone(); Py::Bool({ let t619 = v_Vin.clone(); if py::lt(&t618, &t619)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
    v_dI = py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    v_m2 = py::add(&py::mul(&v_Io.clone(), &v_Io.clone())?, &py::truediv(&py::mul(&v_dI.clone(), &v_dI.clone())?, &Py::Float(12.0))?)?;
    v_sw = py::math1("sqrt", &py::mul(&v_D.clone(), &v_m2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("I_sw_avg"), py::mul(&v_D.clone(), &v_Io.clone())?), (Py::str("I_sw_rms"), v_sw.clone()), (Py::str("I_d_avg"), py::mul(&py::sub(&Py::Float(1.0), &v_D.clone())?, &v_Io.clone())?), (Py::str("I_d_rms"), py::math1("sqrt", &py::mul(&py::sub(&Py::Float(1.0), &v_D.clone())?, &v_m2.clone())?)?), (Py::str("I_L_rms"), py::math1("sqrt", &v_m2.clone())?), (Py::str("I_pk"), py::add(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("I_cin_rms"), py::math1("sqrt", &py::sub(&py::mul(&v_D.clone(), &v_m2.clone())?, &py::pow(&py::mul(&v_D.clone(), &v_Io.clone())?, &Py::Int(2i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_dcm(mut v_Vin: Py, mut v_L: Py, mut v_fs: Py, mut v_R: Py, mut v_D: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Vo: Py = Py::None;
    v_K = f__dcm_common(v_Vin.clone(), v_L.clone(), v_fs.clone(), v_R.clone(), v_D.clone())?;
    let _ = f__need({ let t672 = v_K.clone(); Py::Bool({ let t673 = py::sub(&Py::Float(1.0), &v_D.clone())?; if py::lt(&t672, &t673)? { true } else { false } }) }, Py::str("K = 2L/(R Ts) >= 1 - D: the converter is in continuous conduction (use the CCM capability)"))?;
    v_M = py::truediv(&Py::Float(2.0), &py::add(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(4.0), &v_K.clone())?, &py::mul(&v_D.clone(), &v_D.clone())?)?)?)?)?)?;
    v_Vo = py::mul(&v_M.clone(), &v_Vin.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("M"), v_M.clone()), (Py::str("Vout"), v_Vo.clone()), (Py::str("D2"), py::truediv(&py::mul(&v_D.clone(), &py::sub(&Py::Float(1.0), &v_M.clone())?)?, &v_M.clone())?), (Py::str("I_pk"), py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vo.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?), (Py::str("Io"), py::truediv(&v_Vo.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_lc_design(mut v_Vin: Py, mut v_Vout: Py, mut v_Io: Py, mut v_fs: Py, mut v_r: Py, mut v_dV: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, Io, fs and dV"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_Io.clone(), v_fs.clone(), v_dV.clone()]))?;
    let _ = f__need({ let t718 = v_Vout.clone(); Py::Bool({ let t719 = v_Vin.clone(); if py::lt(&t718, &t719)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    let _ = f__need({ let t720 = Py::Int(0i64); Py::Bool({ let t721 = v_r.clone(); if py::lt(&t720, &t721)? { { let t722 = Py::Int(2i64); if py::lt(&t721, &t722)? { true } else { false } } } else { false } }) }, Py::str("the ripple ratio r must lie between 0 and 2 (continuous conduction)"))?;
    v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
    v_dI = py::mul(&v_r.clone(), &v_Io.clone())?;
    v_L = py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_dI.clone(), &v_fs.clone())?)?;
    v_C = py::truediv(&v_dI.clone(), &py::mul(&py::mul(&Py::Float(8.0), &v_fs.clone())?, &v_dV.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("L"), v_L.clone()), (Py::str("C"), v_C.clone()), (Py::str("IL_max"), py::add(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("dIL"), v_dI.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_losses(mut v_Vin: Py, mut v_Vout: Py, mut v_Io: Py, mut v_fs: Py, mut v_Rds: Py, mut v_RL: Py, mut v_VF: Py, mut v_tsw: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Pc: Py = Py::None;
    let mut v_Pd: Py = Py::None;
    let mut v_Pin: Py = Py::None;
    let mut v_Pl: Py = Py::None;
    let mut v_Po: Py = Py::None;
    let mut v_Psw: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, Io and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_Io.clone(), v_fs.clone()]))?;
    let _ = f__nonneg(Py::str("Rds, RL, VF and tsw"), Py::tuple(vec![v_Rds.clone(), v_RL.clone(), v_VF.clone(), v_tsw.clone()]))?;
    let _ = f__need({ let t819 = py::mul(&v_Io.clone(), &v_Rds.clone())?; Py::Bool({ let t820 = v_Vin.clone(); if py::lt(&t819, &t820)? { true } else { false } }) }, Py::str("the switch drop exceeds the input voltage"))?;
    v_D = py::truediv(&py::add(&py::add(&v_Vout.clone(), &v_VF.clone())?, &py::mul(&v_Io.clone(), &v_RL.clone())?)?, &py::add(&py::sub(&v_Vin.clone(), &py::mul(&v_Io.clone(), &v_Rds.clone())?)?, &v_VF.clone())?)?;
    let _ = f__need({ let t821 = Py::Int(0i64); Py::Bool({ let t822 = v_D.clone(); if py::lt(&t821, &t822)? { { let t823 = Py::Int(1i64); if py::lt(&t822, &t823)? { true } else { false } } } else { false } }) }, Py::str("no duty cycle below 1 reaches this output voltage with these losses"))?;
    v_Pc = py::mul(&py::mul(&py::mul(&v_Io.clone(), &v_Io.clone())?, &v_Rds.clone())?, &v_D.clone())?;
    v_Pd = py::mul(&py::mul(&v_VF.clone(), &v_Io.clone())?, &py::sub(&Py::Float(1.0), &v_D.clone())?)?;
    v_Pl = py::mul(&py::mul(&v_Io.clone(), &v_Io.clone())?, &v_RL.clone())?;
    v_Psw = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Vin.clone())?, &v_Io.clone())?, &v_tsw.clone())?, &v_fs.clone())?;
    v_Po = py::mul(&v_Vout.clone(), &v_Io.clone())?;
    v_Pin = py::add(&py::add(&py::add(&py::add(&v_Po.clone(), &v_Pc.clone())?, &v_Pd.clone())?, &v_Pl.clone())?, &v_Psw.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("P_switch_cond"), v_Pc.clone()), (Py::str("P_diode"), v_Pd.clone()), (Py::str("P_inductor"), v_Pl.clone()), (Py::str("P_switching"), v_Psw.clone()), (Py::str("P_loss"), py::sub(&v_Pin.clone(), &v_Po.clone())?), (Py::str("efficiency"), py::truediv(&v_Po.clone(), &v_Pin.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buck_output_ripple(mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_C: Py, mut v_fs: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, L, C and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_C.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t767 = v_Vout.clone(); Py::Bool({ let t768 = v_Vin.clone(); if py::lt(&t767, &t768)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
    v_dI = py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    v_dV = py::truediv(&v_dI.clone(), &py::mul(&py::mul(&Py::Float(8.0), &v_C.clone())?, &v_fs.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("dIL"), v_dI.clone()), (Py::str("dVo"), v_dV.clone()), (Py::str("dVo_pct"), py::truediv(&py::mul(&Py::Float(100.0), &v_dV.clone())?, &v_Vout.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_capacitor_filter_exact(mut v_circuit: Py, mut v_Vp: Py, mut v_f: Py, mut v_R: Py, mut v_C: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Vavg: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_span: Py = Py::None;
    let mut v_sth: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wrc: Py = Py::None;
    let _ = f__pos(Py::str("Vp, f, R and C"), Py::tuple(vec![v_Vp.clone(), v_f.clone(), v_R.clone(), v_C.clone()]))?;
    let _ = f__need({ let t868 = v_circuit.clone(); Py::Bool({ let t869 = c__HF()?; if py::contains(&t869, &t868)? { true } else { false } }) }, Py::str("circuit: half-wave or full-wave"))?;
    v_w = py::mul(&c_TWO_PI()?, &v_f.clone())?;
    v_wrc = py::mul(&py::mul(&v_w.clone(), &v_R.clone())?, &v_C.clone())?;
    v_P = (if ({ let t870 = v_circuit.clone(); Py::Bool({ let t871 = Py::str("half-wave"); if py::eq(&t870, &t871) { true } else { false } }) }).truthy() { c_TWO_PI()? } else { Py::Float(std::f64::consts::PI) });
    v_th = py::sub(&Py::Float(std::f64::consts::PI), &py::math1("atan", &v_wrc.clone())?)?;
    v_sth = py::math1("sin", &v_th.clone())?;
    v_g = { let v_C = v_C.clone(); let v_P = v_P.clone(); let v_R = v_R.clone(); let v_Vp = v_Vp.clone(); let v_circuit = v_circuit.clone(); let v_f = v_f.clone(); let v_sth = v_sth.clone(); let v_th = v_th.clone(); let v_w = v_w.clone(); let v_wrc = v_wrc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("sin", &v_a.clone())?, &py::mul(&v_sth.clone(), &py::math1("exp", &py::truediv(&py::neg(&py::sub(&py::add(&v_P.clone(), &v_a.clone())?, &v_th.clone())?)?, &v_wrc.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t872 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&Py::Float(std::f64::consts::PI), &Py::Float(2.0))?])).items()?;
    if t872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t872[0].clone();
    v_hi = t872[1].clone();
    for t873 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t873;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t878 = { let t874 = v_mid.clone(); Py::Bool({ let t875 = v_lo.clone(); if py::eq(&t874, &t875) { true } else { false } }) }; if t878.truthy() { t878 } else { { let t876 = v_mid.clone(); Py::Bool({ let t877 = v_hi.clone(); if py::eq(&t876, &t877) { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t879 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t880 = Py::Int(0i64); if py::lt(&t879, &t880)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_a = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    v_span = py::sub(&py::add(&v_P.clone(), &v_a.clone())?, &v_th.clone())?;
    v_Vavg = py::truediv(&py::add(&py::mul(&v_Vp.clone(), &py::sub(&py::math1("cos", &v_a.clone())?, &py::math1("cos", &v_th.clone())?)?)?, &py::mul(&py::mul(&py::mul(&v_Vp.clone(), &v_sth.clone())?, &v_wrc.clone())?, &py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&v_span.clone())?, &v_wrc.clone())?)?)?)?)?, &v_P.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta_off_deg"), py::math1("degrees", &v_th.clone())?), (Py::str("alpha_deg"), py::math1("degrees", &v_a.clone())?), (Py::str("dV"), py::mul(&v_Vp.clone(), &py::sub(&Py::Float(1.0), &py::math1("sin", &v_a.clone())?)?)?), (Py::str("V_avg"), v_Vavg.clone()), (Py::str("I_D_peak"), py::mul(&v_Vp.clone(), &py::add(&py::mul(&py::mul(&v_w.clone(), &v_C.clone())?, &py::math1("cos", &v_a.clone())?)?, &py::truediv(&py::math1("sin", &v_a.clone())?, &v_R.clone())?)?)?), (Py::str("conduction_deg"), py::math1("degrees", &py::sub(&v_th.clone(), &v_a.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conversion_ratio(mut v_topology: Py, mut v_Vin: Py, mut v_D: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_pol: Py = Py::None;
    let _ = f__pos(Py::str("Vin"), Py::tuple(vec![v_Vin.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__need({ let t931 = v_topology.clone(); Py::Bool({ let t932 = c__TOPO5()?; if py::contains(&t932, &t931)? { true } else { false } }) }, Py::str("topology: buck, boost, buck-boost, Cuk or SEPIC"))?;
    if ({ let t933 = v_topology.clone(); Py::Bool({ let t934 = Py::str("buck"); if py::eq(&t933, &t934) { true } else { false } }) }).truthy() {
        v_M = v_D.clone();
    } else {
        if ({ let t935 = v_topology.clone(); Py::Bool({ let t936 = Py::str("boost"); if py::eq(&t935, &t936) { true } else { false } }) }).truthy() {
            v_M = py::truediv(&Py::Float(1.0), &py::sub(&Py::Float(1.0), &v_D.clone())?)?;
        } else {
            v_M = py::truediv(&v_D.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?;
        }
    }
    v_pol = (if ({ let t937 = v_topology.clone(); Py::Bool({ let t938 = Py::tuple(vec![Py::str("buck-boost"), Py::str("Cuk")]); if py::contains(&t938, &t937)? { true } else { false } }) }).truthy() { Py::str("inverted") } else { Py::str("same") });
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("Vout"), py::mul(&v_M.clone(), &v_Vin.clone())?), (Py::str("polarity"), v_pol.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_inductance(mut v_topology: Py, mut v_Vin: Py, mut v_Vout: Py, mut v_fs: Py, mut v_R: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Lc: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, fs and R"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_fs.clone(), v_R.clone()]))?;
    let _ = f__need({ let t1044 = v_topology.clone(); Py::Bool({ let t1045 = c__TOPO3()?; if py::contains(&t1045, &t1044)? { true } else { false } }) }, Py::str("topology: buck, boost or buck-boost"))?;
    if ({ let t1046 = v_topology.clone(); Py::Bool({ let t1047 = Py::str("buck"); if py::eq(&t1046, &t1047) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t1048 = v_Vout.clone(); Py::Bool({ let t1049 = v_Vin.clone(); if py::lt(&t1048, &t1049)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
        v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
        v_Lc = py::truediv(&py::mul(&py::sub(&Py::Float(1.0), &v_D.clone())?, &v_R.clone())?, &py::mul(&Py::Float(2.0), &v_fs.clone())?)?;
    } else {
        if ({ let t1050 = v_topology.clone(); Py::Bool({ let t1051 = Py::str("boost"); if py::eq(&t1050, &t1051) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t1052 = v_Vout.clone(); Py::Bool({ let t1053 = v_Vin.clone(); if py::gt(&t1052, &t1053)? { true } else { false } }) }, Py::str("a boost converter needs Vout > Vin"))?;
            v_D = py::sub(&Py::Float(1.0), &py::truediv(&v_Vin.clone(), &v_Vout.clone())?)?;
            v_Lc = py::truediv(&py::mul(&py::mul(&v_D.clone(), &py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &Py::Int(2i64))?)?, &v_R.clone())?, &py::mul(&Py::Float(2.0), &v_fs.clone())?)?;
        } else {
            v_D = py::truediv(&v_Vout.clone(), &py::add(&v_Vin.clone(), &v_Vout.clone())?)?;
            v_Lc = py::truediv(&py::mul(&py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &Py::Int(2i64))?, &v_R.clone())?, &py::mul(&Py::Float(2.0), &v_fs.clone())?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("L_crit"), v_Lc.clone()), (Py::str("Io"), py::truediv(&v_Vout.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cuk_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_L1: Py, mut v_L2: Py, mut v_C1: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_IL1: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout (magnitude), L1, L2, C1, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L1.clone(), v_L2.clone(), v_C1.clone(), v_fs.clone(), v_Io.clone()]))?;
    v_D = py::truediv(&v_Vout.clone(), &py::add(&v_Vin.clone(), &v_Vout.clone())?)?;
    v_T = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_IL1 = py::truediv(&py::mul(&v_Io.clone(), &v_Vout.clone())?, &v_Vin.clone())?;
    let t1098 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&v_Vin.clone(), &v_D.clone())?, &v_T.clone())?, &v_L1.clone())?, py::truediv(&py::mul(&py::mul(&v_Vout.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?, &v_T.clone())?, &v_L2.clone())?])).items()?;
    if t1098.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t1098[0].clone();
    v_d2 = t1098[1].clone();
    let _ = f__ccm_ok(py::sub(&v_IL1.clone(), &py::truediv(&v_d1.clone(), &Py::Float(2.0))?)?, v_IL1.clone())?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_d2.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("IL1"), v_IL1.clone()), (Py::str("IL2"), v_Io.clone()), (Py::str("VC1"), py::add(&v_Vin.clone(), &v_Vout.clone())?), (Py::str("dIL1"), v_d1.clone()), (Py::str("dIL2"), v_d2.clone()), (Py::str("dVC1"), py::truediv(&py::mul(&py::mul(&v_Io.clone(), &v_D.clone())?, &v_T.clone())?, &v_C1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dab_power(mut v_V1: Py, mut v_V2: Py, mut v_n: Py, mut v_L: Py, mut v_fs: Py, mut v_phi: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_V2p: Py = Py::None;
    let mut v_i0: Py = Py::None;
    let mut v_ip: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_wL: Py = Py::None;
    let _ = f__pos(Py::str("V1, V2, n, L and fs"), Py::tuple(vec![v_V1.clone(), v_V2.clone(), v_n.clone(), v_L.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t1458 = Py::Int(0i64); Py::Bool({ let t1459 = v_phi.clone(); if py::lt(&t1458, &t1459)? { { let t1460 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t1459, &t1460)? { true } else { false } } } else { false } }) }, Py::str("the phase shift must lie in (0, 90] degrees"))?;
    v_V2p = py::mul(&v_n.clone(), &v_V2.clone())?;
    v_wL = py::mul(&py::mul(&c_TWO_PI()?, &v_fs.clone())?, &v_L.clone())?;
    v_P = py::truediv(&py::mul(&py::mul(&py::mul(&v_V1.clone(), &v_V2p.clone())?, &v_phi.clone())?, &py::sub(&Py::Float(std::f64::consts::PI), &v_phi.clone())?)?, &py::mul(&py::mul(&py::mul(&c_TWO_PI()?, &Py::Float(std::f64::consts::PI))?, &v_fs.clone())?, &v_L.clone())?)?;
    v_i0 = py::truediv(&py::neg(&py::add(&py::mul(&v_V1.clone(), &Py::Float(std::f64::consts::PI))?, &py::mul(&v_V2p.clone(), &py::sub(&py::mul(&Py::Float(2.0), &v_phi.clone())?, &Py::Float(std::f64::consts::PI))?)?)?)?, &py::mul(&Py::Float(2.0), &v_wL.clone())?)?;
    v_ip = py::add(&v_i0.clone(), &py::truediv(&py::mul(&py::add(&v_V1.clone(), &v_V2p.clone())?, &v_phi.clone())?, &v_wL.clone())?)?;
    v_ms = py::truediv(&py::add(&py::mul(&v_phi.clone(), &py::add(&py::add(&py::mul(&v_i0.clone(), &v_i0.clone())?, &py::mul(&v_i0.clone(), &v_ip.clone())?)?, &py::mul(&v_ip.clone(), &v_ip.clone())?)?)?, &py::mul(&py::sub(&Py::Float(std::f64::consts::PI), &v_phi.clone())?, &py::add(&py::sub(&py::mul(&v_ip.clone(), &v_ip.clone())?, &py::mul(&v_ip.clone(), &v_i0.clone())?)?, &py::mul(&v_i0.clone(), &v_i0.clone())?)?)?)?, &py::mul(&Py::Float(3.0), &Py::Float(std::f64::consts::PI))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("P_max"), py::truediv(&py::mul(&v_V1.clone(), &v_V2p.clone())?, &py::mul(&py::mul(&Py::Float(8.0), &v_fs.clone())?, &v_L.clone())?)?), (Py::str("i0"), v_i0.clone()), (Py::str("i_phi"), v_ip.clone()), (Py::str("I_rms"), py::math1("sqrt", &v_ms.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_drive_rectifier(mut v_converter: Py, mut v_Vs: Py, mut v_alpha: Py, mut v_Ra: Py, mut v_K: Py, mut v_Ia: Py) -> R {
    let mut v_Ea: Py = Py::None;
    let mut v_Va: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("Vs, Ra, K and Ia"), Py::tuple(vec![v_Vs.clone(), v_Ra.clone(), v_K.clone(), v_Ia.clone()]))?;
    let _ = f__need({ let t1145 = v_converter.clone(); Py::Bool({ let t1146 = c__PH2()?; if py::contains(&t1146, &t1145)? { true } else { false } }) }, Py::str("converter: single-phase or three-phase"))?;
    let _ = f__need({ let t1147 = Py::Int(0i64); Py::Bool({ let t1148 = v_alpha.clone(); if py::le(&t1147, &t1148)? { { let t1149 = Py::Float(std::f64::consts::PI); if py::lt(&t1148, &t1149)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Va = f__snap(py::truediv(&py::mul(&py::mul(&py::mul(&(if ({ let t1152 = v_converter.clone(); Py::Bool({ let t1153 = Py::str("single-phase"); if py::eq(&t1152, &t1153) { true } else { false } }) }).truthy() { Py::Float(2.0) } else { Py::Float(3.0) }), &c_SQ2()?)?, &v_Vs.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &Py::Float(std::f64::consts::PI))?, v_Vs.clone())?;
    v_Ea = py::sub(&v_Va.clone(), &py::mul(&v_Ia.clone(), &v_Ra.clone())?)?;
    v_w = py::truediv(&v_Ea.clone(), &v_K.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Va"), v_Va.clone()), (Py::str("Ea"), v_Ea.clone()), (Py::str("omega"), v_w.clone()), (Py::str("rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("T"), py::mul(&v_K.clone(), &v_Ia.clone())?), (Py::str("P"), py::mul(&v_Ea.clone(), &v_Ia.clone())?), (Py::str("mode"), (if ({ let t1166 = v_w.clone(); Py::Bool({ let t1167 = Py::Int(0i64); if py::ge(&t1166, &t1167)? { true } else { false } }) }).truthy() { Py::str("motoring") } else { Py::str("reverse (regenerating against an active load)") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_motor_chopper(mut v_Vs: Py, mut v_D: Py, mut v_Ra: Py, mut v_K: Py, mut v_TL: Py) -> R {
    let mut v_Ea: Py = Py::None;
    let mut v_Ia: Py = Py::None;
    let mut v_Pout: Py = Py::None;
    let mut v_Va: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("Vs, Ra and K"), Py::tuple(vec![v_Vs.clone(), v_Ra.clone(), v_K.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__nonneg(Py::str("the load torque TL"), Py::tuple(vec![v_TL.clone()]))?;
    v_Ia = py::truediv(&v_TL.clone(), &v_K.clone())?;
    v_Va = py::mul(&v_D.clone(), &v_Vs.clone())?;
    v_Ea = py::sub(&v_Va.clone(), &py::mul(&v_Ia.clone(), &v_Ra.clone())?)?;
    let _ = f__need({ let t1270 = v_Ea.clone(); Py::Bool({ let t1271 = Py::Int(0i64); if py::gt(&t1270, &t1271)? { true } else { false } }) }, Py::str("the armature drop exceeds the applied voltage: the motor stalls at this load"))?;
    v_w = py::truediv(&v_Ea.clone(), &v_K.clone())?;
    v_Pout = py::mul(&v_Ea.clone(), &v_Ia.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ia"), v_Ia.clone()), (Py::str("Va"), v_Va.clone()), (Py::str("Ea"), v_Ea.clone()), (Py::str("omega"), v_w.clone()), (Py::str("rpm"), py::mul(&v_w.clone(), &c__RPM()?)?), (Py::str("P_out"), v_Pout.clone()), (Py::str("efficiency"), (if ({ let t1284 = v_Ia.clone(); Py::Bool({ let t1285 = Py::Int(0i64); if py::gt(&t1284, &t1285)? { true } else { false } }) }).truthy() { py::truediv(&v_Pout.clone(), &py::mul(&v_Va.clone(), &v_Ia.clone())?)? } else { Py::Float(1.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dead_time_error(mut v_Vdc: Py, mut v_td: Py, mut v_fs: Py, mut v_V1: Py) -> R {
    let mut v_dV: Py = Py::None;
    let mut v_dV1: Py = Py::None;
    let _ = f__pos(Py::str("Vdc, td, fs and V1"), Py::tuple(vec![v_Vdc.clone(), v_td.clone(), v_fs.clone(), v_V1.clone()]))?;
    let _ = f__need({ let t1318 = py::mul(&v_td.clone(), &v_fs.clone())?; Py::Bool({ let t1319 = Py::Float(0.5); if py::lt(&t1318, &t1319)? { true } else { false } }) }, Py::str("the dead time must be shorter than half the switching period"))?;
    v_dV = py::mul(&py::mul(&v_td.clone(), &v_fs.clone())?, &v_Vdc.clone())?;
    v_dV1 = py::truediv(&py::mul(&Py::Float(4.0), &v_dV.clone())?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("dV_avg"), v_dV.clone()), (Py::str("dV1_peak"), v_dV1.clone()), (Py::str("dV1_pct"), py::truediv(&py::mul(&Py::Float(100.0), &v_dV1.clone())?, &v_V1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dowell(mut v_h: Py, mut v_f: Py, mut v_rho: Py, mut v_m: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_FR: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let _ = f__pos(Py::str("h, f and rho"), Py::tuple(vec![v_h.clone(), v_f.clone(), v_rho.clone()]))?;
    let _ = f__need({ let t1411 = Py::Bool(matches!(v_m.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1411.truthy() { t1411 } else { { let t1408 = Py::Int(1i64); Py::Bool({ let t1409 = v_m.clone(); if py::le(&t1408, &t1409)? { { let t1410 = Py::Int(30i64); if py::le(&t1409, &t1410)? { true } else { false } } } else { false } }) } } }, Py::str("the number of layers m must be a whole number from 1 to 30"))?;
    v_delta = py::math1("sqrt", &py::truediv(&v_rho.clone(), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &c_MU0()?)?)?)?;
    v_D = py::truediv(&v_h.clone(), &v_delta.clone())?;
    let _ = f__need({ let t1412 = v_D.clone(); Py::Bool({ let t1413 = Py::Float(20.0); if py::le(&t1412, &t1413)? { true } else { false } }) }, Py::str("the layer is thicker than 20 skin depths (outside the range of this evaluation)"))?;
    v_a = py::truediv(&py::add(&py::math1("sinh", &py::mul(&Py::Int(2i64), &v_D.clone())?)?, &py::math1("sin", &py::mul(&Py::Int(2i64), &v_D.clone())?)?)?, &py::sub(&py::math1("cosh", &py::mul(&Py::Int(2i64), &v_D.clone())?)?, &py::math1("cos", &py::mul(&Py::Int(2i64), &v_D.clone())?)?)?)?;
    v_b = py::truediv(&py::sub(&py::math1("sinh", &v_D.clone())?, &py::math1("sin", &v_D.clone())?)?, &py::add(&py::math1("cosh", &v_D.clone())?, &py::math1("cos", &v_D.clone())?)?)?;
    v_FR = py::mul(&v_D.clone(), &py::add(&v_a.clone(), &py::mul(&py::truediv(&py::mul(&Py::Float(2.0), &py::sub(&py::mul(&v_m.clone(), &v_m.clone())?, &Py::Int(1i64))?)?, &Py::Float(3.0))?, &v_b.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("delta"), v_delta.clone()), (Py::str("Delta"), v_D.clone()), (Py::str("FR"), v_FR.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_field_weakening(mut v_Ls: Py, mut v_psi_f: Py, mut v_p: Py, mut v_Imax: Py, mut v_Vmax: Py, mut v_omega: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_id: Py = Py::None;
    let mut v_iq: Py = Py::None;
    let mut v_wb: Py = Py::None;
    let mut v_wmax: Py = Py::None;
    let _ = f__pos(Py::str("Ls, psi_f, Imax, Vmax and omega"), Py::tuple(vec![v_Ls.clone(), v_psi_f.clone(), v_Imax.clone(), v_Vmax.clone(), v_omega.clone()]))?;
    let _ = f__need({ let t2528 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2528.truthy() { t2528 } else { { let t2525 = Py::Int(1i64); Py::Bool({ let t2526 = v_p.clone(); if py::le(&t2525, &t2526)? { { let t2527 = Py::Int(50i64); if py::le(&t2526, &t2527)? { true } else { false } } } else { false } }) } } }, Py::str("the number of pole pairs p must be a whole number from 1 to 50"))?;
    let _ = f__need({ let t2529 = v_psi_f.clone(); Py::Bool({ let t2530 = py::mul(&v_Ls.clone(), &v_Imax.clone())?; if py::gt(&t2529, &t2530)? { true } else { false } }) }, Py::str("the characteristic current psi_f/Ls is within the current limit: the speed is unbounded (no finite maximum)"))?;
    v_wb = py::truediv(&v_Vmax.clone(), &py::mul(&v_p.clone(), &py::hypot(&[v_psi_f.clone(), py::mul(&v_Ls.clone(), &v_Imax.clone())?])?)?)?;
    v_wmax = py::truediv(&v_Vmax.clone(), &py::mul(&v_p.clone(), &py::sub(&v_psi_f.clone(), &py::mul(&v_Ls.clone(), &v_Imax.clone())?)?)?)?;
    let _ = f__need({ let t2531 = v_omega.clone(); Py::Bool({ let t2532 = v_wmax.clone(); if py::lt(&t2531, &t2532)? { true } else { false } }) }, Py::str("the speed is above the maximum reachable at this voltage and current limit"))?;
    if ({ let t2533 = v_omega.clone(); Py::Bool({ let t2534 = v_wb.clone(); if py::le(&t2533, &t2534)? { true } else { false } }) }).truthy() {
        v_id = Py::Float(0.0);
    } else {
        v_id = py::truediv(&py::sub(&py::sub(&py::pow(&py::truediv(&v_Vmax.clone(), &py::mul(&v_p.clone(), &v_omega.clone())?)?, &Py::Int(2i64))?, &py::mul(&v_psi_f.clone(), &v_psi_f.clone())?)?, &py::mul(&py::mul(&py::mul(&v_Ls.clone(), &v_Ls.clone())?, &v_Imax.clone())?, &v_Imax.clone())?)?, &py::mul(&py::mul(&Py::Float(2.0), &v_psi_f.clone())?, &v_Ls.clone())?)?;
    }
    v_iq = py::math1("sqrt", &py::sub(&py::mul(&v_Imax.clone(), &v_Imax.clone())?, &py::mul(&v_id.clone(), &v_id.clone())?)?)?;
    v_T = py::mul(&py::mul(&py::mul(&Py::Float(1.5), &v_p.clone())?, &v_psi_f.clone())?, &v_iq.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_base"), v_wb.clone()), (Py::str("omega_max"), v_wmax.clone()), (Py::str("rpm_base"), py::mul(&v_wb.clone(), &c__RPM()?)?), (Py::str("id"), v_id.clone()), (Py::str("iq"), v_iq.clone()), (Py::str("T"), v_T.clone()), (Py::str("P"), py::mul(&v_T.clone(), &v_omega.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flyback_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_n: Py, mut v_Lm: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Im: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, n, Lm, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_n.clone(), v_Lm.clone(), v_fs.clone(), v_Io.clone()]))?;
    v_D = py::truediv(&py::mul(&v_n.clone(), &v_Vout.clone())?, &py::add(&v_Vin.clone(), &py::mul(&v_n.clone(), &v_Vout.clone())?)?)?;
    v_T = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_Im = py::truediv(&v_Io.clone(), &py::mul(&v_n.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?)?;
    v_dI = py::truediv(&py::mul(&py::mul(&v_Vin.clone(), &v_D.clone())?, &v_T.clone())?, &v_Lm.clone())?;
    let _ = f__ccm_ok(py::sub(&v_Im.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_Im.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("Im"), v_Im.clone()), (Py::str("dIm"), v_dI.clone()), (Py::str("Ipk_primary"), py::add(&v_Im.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("Ipk_secondary"), py::mul(&v_n.clone(), &py::add(&v_Im.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?)?), (Py::str("V_switch"), py::add(&v_Vin.clone(), &py::mul(&v_n.clone(), &v_Vout.clone())?)?), (Py::str("V_diode"), py::add(&v_Vout.clone(), &py::truediv(&v_Vin.clone(), &v_n.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flyback_dcm_design(mut v_Vin: Py, mut v_Po: Py, mut v_eff: Py, mut v_fs: Py, mut v_D: Py) -> R {
    let mut v_Ipk: Py = Py::None;
    let mut v_Lm: Py = Py::None;
    let mut v_Pin: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Po and fs"), Py::tuple(vec![v_Vin.clone(), v_Po.clone(), v_fs.clone()]))?;
    let _ = f__frac(Py::str("the efficiency"), v_eff.clone())?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    v_Pin = py::truediv(&v_Po.clone(), &v_eff.clone())?;
    v_Lm = py::truediv(&py::pow(&py::mul(&v_Vin.clone(), &v_D.clone())?, &Py::Int(2i64))?, &py::mul(&py::mul(&Py::Float(2.0), &v_Pin.clone())?, &v_fs.clone())?)?;
    v_Ipk = py::truediv(&py::mul(&v_Vin.clone(), &v_D.clone())?, &py::mul(&v_Lm.clone(), &v_fs.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Pin"), v_Pin.clone()), (Py::str("Lm"), v_Lm.clone()), (Py::str("I_pk"), v_Ipk.clone()), (Py::str("I_rms"), py::mul(&v_Ipk.clone(), &py::math1("sqrt", &py::truediv(&v_D.clone(), &Py::Float(3.0))?)?)?), (Py::str("I_avg"), py::truediv(&v_Pin.clone(), &v_Vin.clone())?), (Py::str("E_cycle"), py::truediv(&v_Pin.clone(), &v_fs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forward_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_n: Py, mut v_Nr: Py, mut v_L: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_Dmax: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, n, Nr, L, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_n.clone(), v_Nr.clone(), v_L.clone(), v_fs.clone(), v_Io.clone()]))?;
    v_D = py::truediv(&py::mul(&v_n.clone(), &v_Vout.clone())?, &v_Vin.clone())?;
    v_Dmax = py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_Nr.clone())?)?;
    let _ = f__need({ let t1601 = v_D.clone(); Py::Bool({ let t1602 = v_Dmax.clone(); if py::lt(&t1601, &t1602)? { true } else { false } }) }, Py::str("the duty cycle needed exceeds the maximum the reset winding allows (the core would not reset)"))?;
    v_T = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_dI = py::truediv(&py::mul(&py::mul(&py::sub(&py::truediv(&v_Vin.clone(), &v_n.clone())?, &v_Vout.clone())?, &v_D.clone())?, &v_T.clone())?, &v_L.clone())?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_max"), v_Dmax.clone()), (Py::str("V_switch"), py::mul(&v_Vin.clone(), &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_Nr.clone())?)?)?), (Py::str("dIL"), v_dI.clone()), (Py::str("IL_max"), py::add(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?), (Py::str("IL_min"), py::sub(&v_Io.clone(), &py::truediv(&v_dI.clone(), &Py::Float(2.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_foster_periodic(mut v_R: Py, mut v_tau: Py, mut v_P: Py, mut v_tp: Py, mut v_T: Py) -> R {
    let mut v_Rt: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_pk: Py = Py::None;
    let _ = f__foster_lists(v_R.clone(), v_tau.clone())?;
    let _ = f__pos(Py::str("P, tp and T"), Py::tuple(vec![v_P.clone(), v_tp.clone(), v_T.clone()]))?;
    let _ = f__need({ let t1659 = v_tp.clone(); Py::Bool({ let t1660 = v_T.clone(); if py::lt(&t1659, &t1660)? { true } else { false } }) }, Py::str("the pulse must be shorter than the period"))?;
    v_pk = Py::Float(0.0);
    v_mn = Py::Float(0.0);
    v_Rt = Py::Float(0.0);
    for t1661 in py::range_iter(&[(py::len(&v_R.clone())?).i()?]) {
        v_k = t1661;
        v_a = py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_tp.clone())?, &py::getitem(&v_tau.clone(), &v_k.clone())?)?)?)?;
        v_b = py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_T.clone())?, &py::getitem(&v_tau.clone(), &v_k.clone())?)?)?)?;
        v_pk = py::iadd(&v_pk, &py::truediv(&py::mul(&py::mul(&v_P.clone(), &py::getitem(&v_R.clone(), &v_k.clone())?)?, &v_a.clone())?, &v_b.clone())?)?;
        v_mn = py::iadd(&v_mn, &py::truediv(&py::mul(&py::mul(&py::mul(&v_P.clone(), &py::getitem(&v_R.clone(), &v_k.clone())?)?, &v_a.clone())?, &py::math1("exp", &py::truediv(&py::neg(&py::sub(&v_T.clone(), &v_tp.clone())?)?, &py::getitem(&v_tau.clone(), &v_k.clone())?)?)?)?, &v_b.clone())?)?;
        v_Rt = py::iadd(&v_Rt, &py::getitem(&v_R.clone(), &v_k.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dT_peak"), v_pk.clone()), (Py::str("dT_min"), v_mn.clone()), (Py::str("dT_avg"), py::truediv(&py::mul(&py::mul(&v_P.clone(), &v_Rt.clone())?, &v_tp.clone())?, &v_T.clone())?), (Py::str("ripple"), py::sub(&v_pk.clone(), &v_mn.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_foster_pulse(mut v_R: Py, mut v_tau: Py, mut v_P: Py, mut v_tp: Py) -> R {
    let mut v_Rt: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__foster_lists(v_R.clone(), v_tau.clone())?;
    let _ = f__pos(Py::str("P and tp"), Py::tuple(vec![v_P.clone(), v_tp.clone()]))?;
    v_Z = Py::Float(0.0);
    v_Rt = Py::Float(0.0);
    for t1716 in py::range_iter(&[(py::len(&v_R.clone())?).i()?]) {
        v_k = t1716;
        v_Z = py::iadd(&v_Z, &py::mul(&py::getitem(&v_R.clone(), &v_k.clone())?, &py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&v_tp.clone())?, &py::getitem(&v_tau.clone(), &v_k.clone())?)?)?)?)?)?;
        v_Rt = py::iadd(&v_Rt, &py::getitem(&v_R.clone(), &v_k.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Zth"), v_Z.clone()), (Py::str("dTj"), py::mul(&v_P.clone(), &v_Z.clone())?), (Py::str("Rth_total"), v_Rt.clone()), (Py::str("dTj_steady"), py::mul(&v_P.clone(), &v_Rt.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gapped_inductor(mut v_L: Py, mut v_Ipk: Py, mut v_Bmax: Py, mut v_Ac: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let _ = f__pos(Py::str("L, Ipk, Bmax and Ac"), Py::tuple(vec![v_L.clone(), v_Ipk.clone(), v_Bmax.clone(), v_Ac.clone()]))?;
    v_N = py::truediv(&py::mul(&v_L.clone(), &v_Ipk.clone())?, &py::mul(&v_Bmax.clone(), &v_Ac.clone())?)?;
    v_lg = py::truediv(&py::mul(&py::mul(&py::mul(&c_MU0()?, &v_N.clone())?, &v_N.clone())?, &v_Ac.clone())?, &v_L.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("l_gap"), v_lg.clone()), (Py::str("E"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_L.clone())?, &v_Ipk.clone())?, &v_Ipk.clone())?), (Py::str("AL"), py::truediv(&v_L.clone(), &py::mul(&v_N.clone(), &v_N.clone())?)?), (Py::str("V_gap"), py::mul(&v_lg.clone(), &v_Ac.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gate_drive(mut v_Qg: Py, mut v_Vdrv: Py, mut v_fs: Py, mut v_Rg: Py) -> R {
    let _ = f__pos(Py::str("Qg, Vdrv, fs and Rg"), Py::tuple(vec![v_Qg.clone(), v_Vdrv.clone(), v_fs.clone(), v_Rg.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_drive"), py::mul(&py::mul(&v_Qg.clone(), &v_Vdrv.clone())?, &v_fs.clone())?), (Py::str("Ig_avg"), py::mul(&v_Qg.clone(), &v_fs.clone())?), (Py::str("Ig_peak"), py::truediv(&v_Vdrv.clone(), &v_Rg.clone())?), (Py::str("E_cycle"), py::mul(&v_Qg.clone(), &v_Vdrv.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_half_wave_rl(mut v_Vs: Py, mut v_f: Py, mut v_R: Py, mut v_L: Py) -> R {
    let mut v_Vavg: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_wt: Py = Py::None;
    let _ = f__pos(Py::str("Vs, f, R and L"), Py::tuple(vec![v_Vs.clone(), v_f.clone(), v_R.clone(), v_L.clone()]))?;
    v_Vm = py::mul(&c_SQ2()?, &v_Vs.clone())?;
    v_w = py::mul(&c_TWO_PI()?, &v_f.clone())?;
    v_th = py::atan2(&py::mul(&v_w.clone(), &v_L.clone())?, &v_R.clone())?;
    v_wt = py::truediv(&py::mul(&v_w.clone(), &v_L.clone())?, &v_R.clone())?;
    v_g = { let v_L = v_L.clone(); let v_R = v_R.clone(); let v_Vm = v_Vm.clone(); let v_Vs = v_Vs.clone(); let v_f = v_f.clone(); let v_th = v_th.clone(); let v_w = v_w.clone(); let v_wt = v_wt.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_b: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::math1("sin", &py::sub(&v_b.clone(), &v_th.clone())?)?, &py::mul(&py::math1("sin", &v_th.clone())?, &py::math1("exp", &py::truediv(&py::neg(&v_b.clone())?, &v_wt.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t1849 = (Py::tuple(vec![Py::Float(std::f64::consts::PI), c_TWO_PI()?])).items()?;
    if t1849.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t1849[0].clone();
    v_hi = t1849[1].clone();
    for t1850 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t1850;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t1855 = { let t1851 = v_mid.clone(); Py::Bool({ let t1852 = v_lo.clone(); if py::eq(&t1851, &t1852) { true } else { false } }) }; if t1855.truthy() { t1855 } else { { let t1853 = v_mid.clone(); Py::Bool({ let t1854 = v_hi.clone(); if py::eq(&t1853, &t1854) { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1856 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t1857 = Py::Int(0i64); if py::gt(&t1856, &t1857)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_beta = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    v_Vavg = py::truediv(&py::mul(&v_Vm.clone(), &py::sub(&Py::Float(1.0), &py::math1("cos", &v_beta.clone())?)?)?, &c_TWO_PI()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta_deg"), py::math1("degrees", &v_beta.clone())?), (Py::str("theta_deg"), py::math1("degrees", &v_th.clone())?), (Py::str("Z"), py::hypot(&[v_R.clone(), py::mul(&v_w.clone(), &v_L.clone())?])?), (Py::str("V_avg"), v_Vavg.clone()), (Py::str("I_avg"), py::truediv(&v_Vavg.clone(), &v_R.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hold_up(mut v_Po: Py, mut v_t_hold: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("Po, t_hold, V1 and V2"), Py::tuple(vec![v_Po.clone(), v_t_hold.clone(), v_V1.clone(), v_V2.clone()]))?;
    let _ = f__need({ let t1902 = v_V2.clone(); Py::Bool({ let t1903 = v_V1.clone(); if py::lt(&t1902, &t1903)? { true } else { false } }) }, Py::str("the minimum voltage V2 must be below the starting voltage V1"))?;
    v_C = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_Po.clone())?, &v_t_hold.clone())?, &py::sub(&py::mul(&v_V1.clone(), &v_V1.clone())?, &py::mul(&v_V2.clone(), &v_V2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("E_used"), py::mul(&v_Po.clone(), &v_t_hold.clone())?), (Py::str("E_initial"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_C.clone())?, &v_V1.clone())?, &v_V1.clone())?), (Py::str("E_remaining"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_C.clone())?, &v_V2.clone())?, &v_V2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_igbt_losses(mut v_Ip: Py, mut v_M: Py, mut v_phi: Py, mut v_Vce0: Py, mut v_rce: Py, mut v_VF0: Py, mut v_rF: Py) -> R {
    let mut v_PD: Py = Py::None;
    let mut v_PT: Py = Py::None;
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::str("Ip"), Py::tuple(vec![v_Ip.clone()]))?;
    let _ = f__need({ let t1948 = Py::Int(0i64); Py::Bool({ let t1949 = v_M.clone(); if py::lt(&t1948, &t1949)? { { let t1950 = Py::Int(1i64); if py::le(&t1949, &t1950)? { true } else { false } } } else { false } }) }, Py::str("the modulation index M must lie in (0, 1]"))?;
    let _ = f__need({ let t1951 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t1952 = v_phi.clone(); if py::le(&t1951, &t1952)? { { let t1953 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t1952, &t1953)? { true } else { false } } } else { false } }) }, Py::str("the power-factor angle must lie between -90 and 90 degrees"))?;
    let _ = f__nonneg(Py::str("Vce0, rce, VF0 and rF"), Py::tuple(vec![v_Vce0.clone(), v_rce.clone(), v_VF0.clone(), v_rF.clone()]))?;
    v_c = py::mul(&v_M.clone(), &py::math1("cos", &v_phi.clone())?)?;
    v_PT = py::add(&py::mul(&py::mul(&v_Vce0.clone(), &v_Ip.clone())?, &py::add(&py::truediv(&Py::Float(1.0), &c_TWO_PI()?)?, &py::truediv(&v_c.clone(), &Py::Float(8.0))?)?)?, &py::mul(&py::mul(&py::mul(&v_rce.clone(), &v_Ip.clone())?, &v_Ip.clone())?, &py::add(&py::truediv(&Py::Float(1.0), &Py::Float(8.0))?, &py::truediv(&v_c.clone(), &py::mul(&Py::Float(3.0), &Py::Float(std::f64::consts::PI))?)?)?)?)?;
    v_PD = py::add(&py::mul(&py::mul(&v_VF0.clone(), &v_Ip.clone())?, &py::sub(&py::truediv(&Py::Float(1.0), &c_TWO_PI()?)?, &py::truediv(&v_c.clone(), &Py::Float(8.0))?)?)?, &py::mul(&py::mul(&py::mul(&v_rF.clone(), &v_Ip.clone())?, &v_Ip.clone())?, &py::sub(&py::truediv(&Py::Float(1.0), &Py::Float(8.0))?, &py::truediv(&v_c.clone(), &py::mul(&Py::Float(3.0), &Py::Float(std::f64::consts::PI))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P_igbt"), v_PT.clone()), (Py::str("P_diode"), v_PD.clone()), (Py::str("P_switch_pair"), py::add(&v_PT.clone(), &v_PD.clone())?), (Py::str("P_leg"), py::mul(&Py::Float(2.0), &py::add(&v_PT.clone(), &v_PD.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_igse_triangular(mut v_k: Py, mut v_alpha: Py, mut v_beta: Py, mut v_f: Py, mut v_dB: Py, mut v_D: Py, mut v_Vc: Py) -> R {
    let mut v_Pv: Py = Py::None;
    let mut v_ki: Py = Py::None;
    let mut v_sine: Py = Py::None;
    let _ = f__pos(Py::str("k, alpha, beta, f, dB and Vc"), Py::tuple(vec![v_k.clone(), v_alpha.clone(), v_beta.clone(), v_f.clone(), v_dB.clone(), v_Vc.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    v_ki = py::truediv(&v_k.clone(), &py::mul(&py::mul(&py::pow(&c_TWO_PI()?, &py::sub(&v_alpha.clone(), &Py::Float(1.0))?)?, &f__igse_int(v_alpha.clone())?)?, &py::pow(&Py::Float(2.0), &py::sub(&v_beta.clone(), &v_alpha.clone())?)?)?)?;
    v_Pv = py::mul(&py::mul(&py::mul(&v_ki.clone(), &py::pow(&v_dB.clone(), &v_beta.clone())?)?, &py::pow(&v_f.clone(), &v_alpha.clone())?)?, &py::add(&py::pow(&v_D.clone(), &py::sub(&Py::Float(1.0), &v_alpha.clone())?)?, &py::pow(&py::sub(&Py::Float(1.0), &v_D.clone())?, &py::sub(&Py::Float(1.0), &v_alpha.clone())?)?)?)?;
    v_sine = py::mul(&py::mul(&v_k.clone(), &py::pow(&v_f.clone(), &v_alpha.clone())?)?, &py::pow(&py::truediv(&v_dB.clone(), &Py::Float(2.0))?, &v_beta.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ki"), v_ki.clone()), (Py::str("Pv"), v_Pv.clone()), (Py::str("P"), py::mul(&v_Pv.clone(), &v_Vc.clone())?), (Py::str("ratio_to_sine"), py::truediv(&v_Pv.clone(), &v_sine.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induction_vf(mut v_V_rated: Py, mut v_f_rated: Py, mut v_poles: Py, mut v_n_rated: Py, mut v_f: Py, mut v_T_ratio: Py) -> R {
    let mut v_dn: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ns: Py = Py::None;
    let mut v_ns_r: Py = Py::None;
    let _ = f__pos(Py::str("V_rated, f_rated, n_rated, f and T_ratio"), Py::tuple(vec![v_V_rated.clone(), v_f_rated.clone(), v_n_rated.clone(), v_f.clone(), v_T_ratio.clone()]))?;
    let _ = f__need({ let t2059 = Py::Bool(matches!(v_poles.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2059.truthy() { t2059 } else { { let t2058 = { let t2054 = v_poles.clone(); Py::Bool({ let t2055 = Py::Int(2i64); if py::ge(&t2054, &t2055)? { true } else { false } }) }; if !t2058.truthy() { t2058 } else { { let t2056 = py::modulo(&v_poles.clone(), &Py::Int(2i64))?; Py::Bool({ let t2057 = Py::Int(0i64); if py::eq(&t2056, &t2057) { true } else { false } }) } } } } }, Py::str("the number of poles must be an even whole number"))?;
    let _ = f__need({ let t2060 = v_f.clone(); Py::Bool({ let t2061 = v_f_rated.clone(); if py::le(&t2060, &t2061)? { true } else { false } }) }, Py::str("constant V/f applies up to the rated frequency"))?;
    v_ns_r = py::truediv(&py::mul(&Py::Float(120.0), &v_f_rated.clone())?, &v_poles.clone())?;
    let _ = f__need({ let t2062 = v_n_rated.clone(); Py::Bool({ let t2063 = v_ns_r.clone(); if py::lt(&t2062, &t2063)? { true } else { false } }) }, Py::str("the rated speed must be below the synchronous speed"))?;
    let _ = f__need({ let t2064 = v_T_ratio.clone(); Py::Bool({ let t2065 = Py::Float(1.5); if py::le(&t2064, &t2065)? { true } else { false } }) }, Py::str("the load torque ratio must be at most 1.5 (the linear torque-slip region)"))?;
    v_ns = py::truediv(&py::mul(&Py::Float(120.0), &v_f.clone())?, &v_poles.clone())?;
    v_dn = py::mul(&py::sub(&v_ns_r.clone(), &v_n_rated.clone())?, &v_T_ratio.clone())?;
    v_n = py::sub(&v_ns.clone(), &v_dn.clone())?;
    let _ = f__need({ let t2066 = v_n.clone(); Py::Bool({ let t2067 = Py::Int(0i64); if py::gt(&t2066, &t2067)? { true } else { false } }) }, Py::str("at this frequency the load torque cannot be carried (the speed would be negative)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::truediv(&py::mul(&v_V_rated.clone(), &v_f.clone())?, &v_f_rated.clone())?), (Py::str("ns"), v_ns.clone()), (Py::str("n"), v_n.clone()), (Py::str("slip"), py::truediv(&v_dn.clone(), &v_ns.clone())?), (Py::str("f_slip"), py::truediv(&py::mul(&v_dn.clone(), &v_poles.clone())?, &Py::Float(120.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inductor_area_product(mut v_L: Py, mut v_Ipk: Py, mut v_Irms: Py, mut v_Bmax: Py, mut v_J: Py, mut v_Ku: Py) -> R {
    let mut v_Ap: Py = Py::None;
    let _ = f__pos(Py::str("L, Ipk, Irms, Bmax and J"), Py::tuple(vec![v_L.clone(), v_Ipk.clone(), v_Irms.clone(), v_Bmax.clone(), v_J.clone()]))?;
    let _ = f__frac(Py::str("the window utilisation Ku"), v_Ku.clone())?;
    v_Ap = py::truediv(&py::mul(&py::mul(&v_L.clone(), &v_Ipk.clone())?, &v_Irms.clone())?, &py::mul(&py::mul(&v_Bmax.clone(), &v_J.clone())?, &v_Ku.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ap"), v_Ap.clone()), (Py::str("Ap_cm4"), py::mul(&v_Ap.clone(), &Py::Float(100000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_interleaved_buck(mut v_N: Py, mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_fs: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_dI: Py = Py::None;
    let mut v_m: Py = Py::None;
    let _ = f__need({ let t2159 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2159.truthy() { t2159 } else { { let t2156 = Py::Int(2i64); Py::Bool({ let t2157 = v_N.clone(); if py::le(&t2156, &t2157)? { { let t2158 = Py::Int(12i64); if py::le(&t2157, &t2158)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number of phases from 2 to 12"))?;
    let _ = f__pos(Py::str("Vin, Vout, L and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t2166 = v_Vout.clone(); Py::Bool({ let t2167 = v_Vin.clone(); if py::lt(&t2166, &t2167)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    v_D = py::truediv(&v_Vout.clone(), &v_Vin.clone())?;
    v_dI = py::truediv(&py::mul(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_D.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    v_m = py::math1("floor", &py::mul(&v_N.clone(), &v_D.clone())?)?;
    v_K = py::truediv(&py::mul(&py::mul(&v_N.clone(), &py::sub(&v_D.clone(), &py::truediv(&v_m.clone(), &v_N.clone())?)?)?, &py::sub(&py::truediv(&py::add(&v_m.clone(), &Py::Int(1i64))?, &v_N.clone())?, &v_D.clone())?)?, &py::mul(&v_D.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?)?;
    v_K = f__snap(v_K.clone(), Py::Float(1.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("dI_phase"), v_dI.clone()), (Py::str("dI_total"), py::mul(&v_K.clone(), &v_dI.clone())?), (Py::str("cancellation"), v_K.clone()), (Py::str("f_ripple"), py::mul(&v_N.clone(), &v_fs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverter_rl(mut v_Vdc: Py, mut v_f: Py, mut v_R: Py, mut v_L: Py) -> R {
    let mut v_Imax: Py = Py::None;
    let mut v_Irms: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("Vdc, f, R and L"), Py::tuple(vec![v_Vdc.clone(), v_f.clone(), v_R.clone(), v_L.clone()]))?;
    v_tau = py::truediv(&v_L.clone(), &v_R.clone())?;
    v_h = py::truediv(&Py::Float(0.5), &v_f.clone())?;
    v_e = py::math1("exp", &py::truediv(&py::neg(&v_h.clone())?, &v_tau.clone())?)?;
    v_a = py::truediv(&v_Vdc.clone(), &v_R.clone())?;
    v_Imax = py::truediv(&py::mul(&v_a.clone(), &py::sub(&Py::Float(1.0), &v_e.clone())?)?, &py::add(&Py::Float(1.0), &v_e.clone())?)?;
    v_b = py::sub(&py::neg(&v_Imax.clone())?, &v_a.clone())?;
    v_ms = py::truediv(&py::add(&py::add(&py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &v_h.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_a.clone())?, &v_b.clone())?, &v_tau.clone())?, &py::sub(&Py::Float(1.0), &v_e.clone())?)?)?, &py::mul(&py::truediv(&py::mul(&py::mul(&v_b.clone(), &v_b.clone())?, &v_tau.clone())?, &Py::Float(2.0))?, &py::sub(&Py::Float(1.0), &py::mul(&v_e.clone(), &v_e.clone())?)?)?)?, &v_h.clone())?;
    v_Irms = py::math1("sqrt", &v_ms.clone())?;
    v_P = py::mul(&v_ms.clone(), &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("I_max"), v_Imax.clone()), (Py::str("I_rms"), v_Irms.clone()), (Py::str("P"), v_P.clone()), (Py::str("Is_avg"), py::truediv(&v_P.clone(), &v_Vdc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_current_harmonics(mut v_circuit: Py, mut v_Id: Py, mut v_h: Py) -> R {
    let mut v_I1: Py = Py::None;
    let mut v_amp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("Id"), Py::tuple(vec![v_Id.clone()]))?;
    let _ = f__need({ let t2887 = v_circuit.clone(); Py::Bool({ let t2888 = c__PH2()?; if py::contains(&t2888, &t2887)? { true } else { false } }) }, Py::str("circuit: single-phase or three-phase"))?;
    let _ = f__need({ let t2892 = Py::Bool(matches!(v_h.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2892.truthy() { t2892 } else { { let t2889 = Py::Int(1i64); Py::Bool({ let t2890 = v_h.clone(); if py::le(&t2889, &t2890)? { { let t2891 = Py::Int(49i64); if py::le(&t2890, &t2891)? { true } else { false } } } else { false } }) } } }, Py::str("the harmonic order h must be a whole number from 1 to 49"))?;
    v_amp = { let v_Id = v_Id.clone(); let v_amp = v_amp.clone(); let v_circuit = v_circuit.clone(); let v_h = v_h.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            if ({ let t2893 = v_circuit.clone(); Py::Bool({ let t2894 = Py::str("single-phase"); if py::eq(&t2893, &t2894) { true } else { false } }) }).truthy() {
                return Ok((if ({ let t2895 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; Py::Bool({ let t2896 = Py::Int(1i64); if py::eq(&t2895, &t2896) { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &c_SQ2()?)?, &v_Id.clone())?, &py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?)? } else { Py::Float(0.0) }));
            }
            return Ok((if ({ let t2901 = { let t2897 = py::modulo(&v_k.clone(), &Py::Int(6i64))?; Py::Bool({ let t2898 = Py::Int(1i64); if py::eq(&t2897, &t2898) { true } else { false } }) }; if t2901.truthy() { t2901 } else { { let t2899 = py::modulo(&v_k.clone(), &Py::Int(6i64))?; Py::Bool({ let t2900 = Py::Int(5i64); if py::eq(&t2899, &t2900) { true } else { false } }) } } }).truthy() { py::truediv(&py::mul(&py::math1("sqrt", &Py::Float(6.0))?, &v_Id.clone())?, &py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?)? } else { Py::Float(0.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_I1 = py::call(&v_amp.clone(), &[Py::Int(1i64)])?;
    v_s = Py::Float(0.0);
    for t2902 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2902;
        v_s = py::iadd(&v_s, &py::pow(&py::call(&v_amp.clone(), &[v_k.clone()])?, &Py::Int(2i64))?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Ih"), py::call(&v_amp.clone(), &[v_h.clone()])?), (Py::str("Ih_over_I1"), py::truediv(&py::call(&v_amp.clone(), &[v_h.clone()])?, &v_I1.clone())?), (Py::str("THD_to_h"), py::truediv(&py::math1("sqrt", &v_s.clone())?, &v_I1.clone())?), (Py::str("I1"), v_I1.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_current_pf(mut v_circuit: Py, mut v_Id: Py, mut v_Vs: Py, mut v_alpha: Py) -> R {
    let mut v_Is: Py = Py::None;
    let mut v_Is1: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::str("Id and Vs"), Py::tuple(vec![v_Id.clone(), v_Vs.clone()]))?;
    let _ = f__need({ let t2830 = v_circuit.clone(); Py::Bool({ let t2831 = c__PH2()?; if py::contains(&t2831, &t2830)? { true } else { false } }) }, Py::str("circuit: single-phase or three-phase"))?;
    let _ = f__need({ let t2832 = Py::Int(0i64); Py::Bool({ let t2833 = v_alpha.clone(); if py::le(&t2832, &t2833)? { { let t2834 = Py::Float(std::f64::consts::PI); if py::lt(&t2833, &t2834)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees (0 for diodes)"))?;
    v_c = f__snap(py::math1("cos", &v_alpha.clone())?, Py::Float(1.0))?;
    if ({ let t2837 = v_circuit.clone(); Py::Bool({ let t2838 = Py::str("single-phase"); if py::eq(&t2837, &t2838) { true } else { false } }) }).truthy() {
        let t2839 = (Py::tuple(vec![v_Id.clone(), py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &c_SQ2()?)?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?])).items()?;
        if t2839.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Is = t2839[0].clone();
        v_Is1 = t2839[1].clone();
        let t2840 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &c_SQ2()?)?, &v_Vs.clone())?, &v_c.clone())?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?, py::mul(&v_Vs.clone(), &v_Id.clone())?])).items()?;
        if t2840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_P = t2840[0].clone();
        v_S = t2840[1].clone();
    } else {
        let t2841 = (Py::tuple(vec![py::mul(&v_Id.clone(), &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?, py::truediv(&py::mul(&py::math1("sqrt", &Py::Float(6.0))?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?])).items()?;
        if t2841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Is = t2841[0].clone();
        v_Is1 = t2841[1].clone();
        let t2842 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(3.0), &c_SQ2()?)?, &v_Vs.clone())?, &v_c.clone())?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?, py::mul(&py::mul(&py::mul(&c_SQ3()?, &v_Vs.clone())?, &v_Id.clone())?, &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?])).items()?;
        if t2842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_P = t2842[0].clone();
        v_S = t2842[1].clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Is_rms"), v_Is.clone()), (Py::str("Is1_rms"), v_Is1.clone()), (Py::str("THD"), py::truediv(&py::math1("sqrt", &py::sub(&py::mul(&v_Is.clone(), &v_Is.clone())?, &py::mul(&v_Is1.clone(), &v_Is1.clone())?)?)?, &v_Is1.clone())?), (Py::str("DPF"), v_c.clone()), (Py::str("PF"), py::truediv(&v_P.clone(), &v_S.clone())?), (Py::str("P"), v_P.clone()), (Py::str("S"), v_S.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_llc_gain(mut v_Lr: Py, mut v_Cr: Py, mut v_Lm: Py, mut v_n: Py, mut v_R: Py, mut v_f: Py) -> R {
    let mut v_Rac: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_zp: Py = Py::None;
    let mut v_zs: Py = Py::None;
    let _ = f__pos(Py::str("Lr, Cr, Lm, n, R and f"), Py::tuple(vec![v_Lr.clone(), v_Cr.clone(), v_Lm.clone(), v_n.clone(), v_R.clone(), v_f.clone()]))?;
    v_Rac = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(8.0), &v_n.clone())?, &v_n.clone())?, &v_R.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &Py::Float(std::f64::consts::PI))?)?;
    v_w = py::mul(&c_TWO_PI()?, &v_f.clone())?;
    v_zp = py::truediv(&py::mul(&py::complex(&[Py::Float(0.0), py::mul(&v_w.clone(), &v_Lm.clone())?])?, &v_Rac.clone())?, &py::complex(&[v_Rac.clone(), py::mul(&v_w.clone(), &v_Lm.clone())?])?)?;
    v_zs = py::complex(&[Py::Float(0.0), py::sub(&py::mul(&v_w.clone(), &v_Lr.clone())?, &py::truediv(&Py::Float(1.0), &py::mul(&v_w.clone(), &v_Cr.clone())?)?)?])?;
    v_h = py::truediv(&v_zp.clone(), &py::add(&v_zs.clone(), &v_zp.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("fr1"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&v_Lr.clone(), &v_Cr.clone())?)?)?)?), (Py::str("fr2"), py::truediv(&Py::Float(1.0), &py::mul(&c_TWO_PI()?, &py::math1("sqrt", &py::mul(&py::add(&v_Lr.clone(), &v_Lm.clone())?, &v_Cr.clone())?)?)?)?), (Py::str("Rac"), v_Rac.clone()), (Py::str("Q"), py::truediv(&py::math1("sqrt", &py::truediv(&v_Lr.clone(), &v_Cr.clone())?)?, &v_Rac.clone())?), (Py::str("k"), py::truediv(&v_Lm.clone(), &v_Lr.clone())?), (Py::str("M"), py::hypot(&[py::real(&v_h.clone())?, py::imag(&v_h.clone())?])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mosfet_conduction(mut v_Rds25: Py, mut v_alpha: Py, mut v_Tj: Py, mut v_Irms: Py) -> R {
    let mut v_R: Py = Py::None;
    let _ = f__pos(Py::str("Rds25, Tj and Irms"), Py::tuple(vec![v_Rds25.clone(), v_Tj.clone(), v_Irms.clone()]))?;
    let _ = f__nonneg(Py::str("the temperature coefficient alpha"), Py::tuple(vec![v_alpha.clone()]))?;
    v_R = py::mul(&v_Rds25.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_alpha.clone(), &py::sub(&v_Tj.clone(), &Py::Float(298.15))?)?)?)?;
    let _ = f__need({ let t2308 = v_R.clone(); Py::Bool({ let t2309 = Py::Int(0i64); if py::gt(&t2308, &t2309)? { true } else { false } }) }, Py::str("the on-resistance would not be positive at this temperature"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Rds_T"), v_R.clone()), (Py::str("P"), py::mul(&py::mul(&v_Irms.clone(), &v_Irms.clone())?, &v_R.clone())?), (Py::str("P_25"), py::mul(&py::mul(&v_Irms.clone(), &v_Irms.clone())?, &v_Rds25.clone())?), (Py::str("ratio"), py::truediv(&v_R.clone(), &v_Rds25.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mtpa(mut v_Ld: Py, mut v_Lq: Py, mut v_psi_f: Py, mut v_p: Py, mut v_Is: Py) -> R {
    let mut v_Te: Py = Py::None;
    let mut v_dL: Py = Py::None;
    let mut v_id: Py = Py::None;
    let mut v_iq: Py = Py::None;
    let _ = f__pos(Py::str("Ld, Lq, psi_f and Is"), Py::tuple(vec![v_Ld.clone(), v_Lq.clone(), v_psi_f.clone(), v_Is.clone()]))?;
    let _ = f__need({ let t2582 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2582.truthy() { t2582 } else { { let t2579 = Py::Int(1i64); Py::Bool({ let t2580 = v_p.clone(); if py::le(&t2579, &t2580)? { { let t2581 = Py::Int(50i64); if py::le(&t2580, &t2581)? { true } else { false } } } else { false } }) } } }, Py::str("the number of pole pairs p must be a whole number from 1 to 50"))?;
    let _ = f__need({ let t2583 = v_Lq.clone(); Py::Bool({ let t2584 = v_Ld.clone(); if py::gt(&t2583, &t2584)? { true } else { false } }) }, Py::str("MTPA with negative id needs a salient machine with Lq > Ld"))?;
    v_dL = py::sub(&v_Lq.clone(), &v_Ld.clone())?;
    v_id = py::truediv(&py::sub(&v_psi_f.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_psi_f.clone(), &v_psi_f.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(8.0), &v_dL.clone())?, &v_dL.clone())?, &v_Is.clone())?, &v_Is.clone())?)?)?)?, &py::mul(&Py::Float(4.0), &v_dL.clone())?)?;
    v_iq = py::math1("sqrt", &py::sub(&py::mul(&v_Is.clone(), &v_Is.clone())?, &py::mul(&v_id.clone(), &v_id.clone())?)?)?;
    v_Te = py::mul(&py::mul(&Py::Float(1.5), &v_p.clone())?, &py::add(&py::mul(&v_psi_f.clone(), &v_iq.clone())?, &py::mul(&py::mul(&py::sub(&v_Ld.clone(), &v_Lq.clone())?, &v_id.clone())?, &v_iq.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("id"), v_id.clone()), (Py::str("iq"), v_iq.clone()), (Py::str("beta_deg"), py::math1("degrees", &py::atan2(&py::neg(&v_id.clone())?, &v_iq.clone())?)?), (Py::str("Te"), v_Te.clone()), (Py::str("Te_id0"), py::mul(&py::mul(&py::mul(&Py::Float(1.5), &v_p.clone())?, &v_psi_f.clone())?, &v_Is.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_sharing(mut v_R: Py, mut v_I: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_cur: Py = Py::None;
    let mut v_loss: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t2351 = Py::Bool(matches!(v_R.clone(), Py::List(_))); if !t2351.truthy() { t2351 } else { { let t2348 = Py::Int(2i64); Py::Bool({ let t2349 = py::len(&v_R.clone())?; if py::le(&t2348, &t2349)? { { let t2350 = Py::Int(12i64); if py::le(&t2349, &t2350)? { true } else { false } } } else { false } }) } } }, Py::str("give 2 to 12 on-resistances"))?;
    for t2352 in py::iter(&(v_R.clone()))? {
        v_r = t2352;
        let _ = f__pos(Py::str("every on-resistance"), Py::tuple(vec![v_r.clone()]))?;
    }
    let _ = f__pos(Py::str("the total current I"), Py::tuple(vec![v_I.clone()]))?;
    v_G = Py::Float(0.0);
    for t2359 in py::iter(&(v_R.clone()))? {
        v_r = t2359;
        v_G = py::iadd(&v_G, &py::truediv(&Py::Float(1.0), &v_r.clone())?)?;
    }
    v_cur = { let mut out = vec![]; for t2360 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t2360;
out.push(py::truediv(&py::mul(&v_I.clone(), &py::truediv(&Py::Float(1.0), &v_r.clone())?)?, &v_G.clone())?); } Py::list(out) };
    v_loss = { let mut out = vec![]; for t2361 in py::range_iter(&[(py::len(&v_R.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2361;
out.push(py::mul(&py::mul(&py::mul(&py::getitem(&v_cur.clone(), &v_k.clone())?, &py::getitem(&v_cur.clone(), &v_k.clone())?)?, &py::getitem(&v_R.clone(), &v_k.clone())?)?, &Py::Float(0.001))?); } Py::list(out) };
    v_tot = Py::Float(0.0);
    for t2362 in py::iter(&(v_loss.clone()))? {
        v_x = t2362;
        v_tot = py::iadd(&v_tot, &v_x.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("currents"), v_cur.clone()), (Py::str("losses"), v_loss.clone()), (Py::str("P_total"), v_tot.clone()), (Py::str("imbalance"), py::truediv(&py::minmax(&[v_cur.clone()], true)?, &py::truediv(&v_I.clone(), &py::len(&v_R.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_peak_current_mode(mut v_Vin: Py, mut v_Vout: Py, mut v_L: Py, mut v_fs: Py, mut v_ma: Py) -> R {
    let mut v_alpha: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_ma_min: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, L and fs"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L.clone(), v_fs.clone()]))?;
    let _ = f__nonneg(Py::str("the compensation ramp slope ma"), Py::tuple(vec![v_ma.clone()]))?;
    let _ = f__need({ let t2413 = v_Vout.clone(); Py::Bool({ let t2414 = v_Vin.clone(); if py::lt(&t2413, &t2414)? { true } else { false } }) }, Py::str("a buck converter needs Vout < Vin"))?;
    v_m1 = py::truediv(&py::sub(&v_Vin.clone(), &v_Vout.clone())?, &v_L.clone())?;
    v_m2 = py::truediv(&v_Vout.clone(), &v_L.clone())?;
    v_alpha = py::truediv(&py::neg(&py::sub(&v_m2.clone(), &v_ma.clone())?)?, &py::add(&v_m1.clone(), &v_ma.clone())?)?;
    v_ma_min = py::minmax(&[Py::Float(0.0), py::truediv(&py::sub(&v_m2.clone(), &v_m1.clone())?, &Py::Float(2.0))?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), py::truediv(&v_Vout.clone(), &v_Vin.clone())?), (Py::str("m1"), v_m1.clone()), (Py::str("m2"), v_m2.clone()), (Py::str("alpha"), v_alpha.clone()), (Py::str("stable"), (if ({ let t2427 = py::abs(&v_alpha.clone())?; Py::Bool({ let t2428 = Py::Int(1i64); if py::lt(&t2427, &t2428)? { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") })), (Py::str("ma_min"), f__snap(v_ma_min.clone(), v_m2.clone())?), (Py::str("ma_deadbeat"), v_m2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pf_distortion(mut v_THD: Py, mut v_phi: Py) -> R {
    let mut v_df: Py = Py::None;
    let mut v_dpf: Py = Py::None;
    let _ = f__nonneg(Py::str("the THD"), Py::tuple(vec![v_THD.clone()]))?;
    let _ = f__need({ let t2629 = py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?; Py::Bool({ let t2630 = v_phi.clone(); if py::le(&t2629, &t2630)? { { let t2631 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t2630, &t2631)? { true } else { false } } } else { false } }) }, Py::str("the displacement angle must lie between -90 and 90 degrees"))?;
    v_dpf = f__snap(py::math1("cos", &v_phi.clone())?, Py::Float(1.0))?;
    v_df = py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::mul(&v_THD.clone(), &v_THD.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("DPF"), v_dpf.clone()), (Py::str("distortion_factor"), v_df.clone()), (Py::str("PF"), py::mul(&v_dpf.clone(), &v_df.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfc_boost(mut v_Vac: Py, mut v_f_line: Py, mut v_Vo: Py, mut v_Po: Py, mut v_eff: Py, mut v_L: Py, mut v_fs: Py, mut v_C: Py) -> R {
    let mut v_Dp: Py = Py::None;
    let mut v_Ipk: Py = Py::None;
    let mut v_Vpk: Py = Py::None;
    let mut v_dImax: Py = Py::None;
    let mut v_dIp: Py = Py::None;
    let _ = f__pos(Py::str("Vac, f_line, Vo, Po, L, fs and C"), Py::tuple(vec![v_Vac.clone(), v_f_line.clone(), v_Vo.clone(), v_Po.clone(), v_L.clone(), v_fs.clone(), v_C.clone()]))?;
    let _ = f__frac(Py::str("the efficiency"), v_eff.clone())?;
    v_Vpk = py::mul(&c_SQ2()?, &v_Vac.clone())?;
    let _ = f__need({ let t314 = v_Vpk.clone(); Py::Bool({ let t315 = v_Vo.clone(); if py::lt(&t314, &t315)? { true } else { false } }) }, Py::str("the output voltage must exceed the peak line voltage"))?;
    v_Ipk = py::truediv(&py::mul(&Py::Float(2.0), &v_Po.clone())?, &py::mul(&v_eff.clone(), &v_Vpk.clone())?)?;
    v_Dp = py::sub(&Py::Float(1.0), &py::truediv(&v_Vpk.clone(), &v_Vo.clone())?)?;
    v_dIp = py::truediv(&py::mul(&v_Vpk.clone(), &v_Dp.clone())?, &py::mul(&v_L.clone(), &v_fs.clone())?)?;
    v_dImax = (if ({ let t316 = v_Vpk.clone(); Py::Bool({ let t317 = py::truediv(&v_Vo.clone(), &Py::Float(2.0))?; if py::ge(&t316, &t317)? { true } else { false } }) }).truthy() { py::truediv(&v_Vo.clone(), &py::mul(&py::mul(&Py::Float(4.0), &v_L.clone())?, &v_fs.clone())?)? } else { v_dIp.clone() });
    return Ok(f__out(Py::dict(vec![(Py::str("I_in_pk"), v_Ipk.clone()), (Py::str("D_peak"), v_Dp.clone()), (Py::str("dI_peak"), v_dIp.clone()), (Py::str("dI_max"), v_dImax.clone()), (Py::str("IL_at_peak"), py::add(&v_Ipk.clone(), &py::truediv(&v_dIp.clone(), &Py::Float(2.0))?)?), (Py::str("dVo_pp"), py::truediv(&v_Po.clone(), &py::mul(&py::mul(&py::mul(&c_TWO_PI()?, &v_f_line.clone())?, &v_C.clone())?, &v_Vo.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pmsm_dq(mut v_Rs: Py, mut v_Ld: Py, mut v_Lq: Py, mut v_psi_f: Py, mut v_p: Py, mut v_id: Py, mut v_iq: Py, mut v_omega_m: Py) -> R {
    let mut v_Pcu: Py = Py::None;
    let mut v_Pin: Py = Py::None;
    let mut v_Te: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_vd: Py = Py::None;
    let mut v_vq: Py = Py::None;
    let mut v_we: Py = Py::None;
    let _ = f__pos(Py::str("Ld, Lq, psi_f and omega_m"), Py::tuple(vec![v_Ld.clone(), v_Lq.clone(), v_psi_f.clone(), v_omega_m.clone()]))?;
    let _ = f__nonneg(Py::str("Rs"), Py::tuple(vec![v_Rs.clone()]))?;
    let _ = f__need({ let t2472 = Py::Bool(matches!(v_p.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2472.truthy() { t2472 } else { { let t2469 = Py::Int(1i64); Py::Bool({ let t2470 = v_p.clone(); if py::le(&t2469, &t2470)? { { let t2471 = Py::Int(50i64); if py::le(&t2470, &t2471)? { true } else { false } } } else { false } }) } } }, Py::str("the number of pole pairs p must be a whole number from 1 to 50"))?;
    let _ = f__fin(Py::tuple(vec![v_id.clone(), v_iq.clone()]))?;
    let _ = f__need({ let t2480 = { let t2476 = v_id.clone(); Py::Bool({ let t2477 = Py::Int(0i64); if py::ne(&t2476, &t2477) { true } else { false } }) }; if t2480.truthy() { t2480 } else { { let t2478 = v_iq.clone(); Py::Bool({ let t2479 = Py::Int(0i64); if py::ne(&t2478, &t2479) { true } else { false } }) } } }, Py::str("the current must not be zero"))?;
    v_we = py::mul(&v_p.clone(), &v_omega_m.clone())?;
    v_vd = py::sub(&py::mul(&v_Rs.clone(), &v_id.clone())?, &py::mul(&py::mul(&v_we.clone(), &v_Lq.clone())?, &v_iq.clone())?)?;
    v_vq = py::add(&py::mul(&v_Rs.clone(), &v_iq.clone())?, &py::mul(&v_we.clone(), &py::add(&py::mul(&v_Ld.clone(), &v_id.clone())?, &v_psi_f.clone())?)?)?;
    v_Te = py::mul(&py::mul(&Py::Float(1.5), &v_p.clone())?, &py::add(&py::mul(&v_psi_f.clone(), &v_iq.clone())?, &py::mul(&py::mul(&py::sub(&v_Ld.clone(), &v_Lq.clone())?, &v_id.clone())?, &v_iq.clone())?)?)?;
    v_Pin = py::mul(&Py::Float(1.5), &py::add(&py::mul(&v_vd.clone(), &v_id.clone())?, &py::mul(&v_vq.clone(), &v_iq.clone())?)?)?;
    v_Pcu = py::mul(&py::mul(&Py::Float(1.5), &v_Rs.clone())?, &py::add(&py::mul(&v_id.clone(), &v_id.clone())?, &py::mul(&v_iq.clone(), &v_iq.clone())?)?)?;
    v_Vm = py::hypot(&[v_vd.clone(), v_vq.clone()])?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega_e"), v_we.clone()), (Py::str("vd"), v_vd.clone()), (Py::str("vq"), v_vq.clone()), (Py::str("V_peak"), v_Vm.clone()), (Py::str("Te"), v_Te.clone()), (Py::str("P_mech"), py::mul(&v_Te.clone(), &v_omega_m.clone())?), (Py::str("P_in"), v_Pin.clone()), (Py::str("P_cu"), v_Pcu.clone()), (Py::str("PF"), py::truediv(&v_Pin.clone(), &py::mul(&py::mul(&Py::Float(1.5), &v_Vm.clone())?, &py::hypot(&[v_id.clone(), v_iq.clone()])?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pulse_rms(mut v_A: Py, mut v_D: Py) -> R {
    let mut v_avg: Py = Py::None;
    let mut v_rms: Py = Py::None;
    let _ = f__pos(Py::str("the amplitude A"), Py::tuple(vec![v_A.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    v_avg = py::mul(&v_D.clone(), &v_A.clone())?;
    v_rms = py::mul(&py::math1("sqrt", &v_D.clone())?, &v_A.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("avg"), v_avg.clone()), (Py::str("rms"), v_rms.clone()), (Py::str("ac_rms"), py::mul(&v_A.clone(), &py::math1("sqrt", &py::mul(&v_D.clone(), &py::sub(&Py::Float(1.0), &v_D.clone())?)?)?)?), (Py::str("form_factor"), py::truediv(&Py::Float(1.0), &py::math1("sqrt", &v_D.clone())?)?), (Py::str("ripple_factor"), py::math1("sqrt", &py::sub(&py::truediv(&Py::Float(1.0), &v_D.clone())?, &Py::Float(1.0))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quasi_square(mut v_Vdc: Py, mut v_alpha: Py, mut v_n: Py) -> R {
    let mut v_V1: Py = Py::None;
    let mut v_Vn: Py = Py::None;
    let mut v_Vrms: Py = Py::None;
    let _ = f__pos(Py::str("Vdc"), Py::tuple(vec![v_Vdc.clone()]))?;
    let _ = f__need({ let t2728 = Py::Int(0i64); Py::Bool({ let t2729 = v_alpha.clone(); if py::le(&t2728, &t2729)? { { let t2730 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::lt(&t2729, &t2730)? { true } else { false } } } else { false } }) }, Py::str("alpha must lie from 0 to 90 degrees"))?;
    let _ = f__need({ let t2737 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2737.truthy() { t2737 } else { { let t2736 = { let t2731 = Py::Int(3i64); Py::Bool({ let t2732 = v_n.clone(); if py::le(&t2731, &t2732)? { { let t2733 = Py::Int(99i64); if py::le(&t2732, &t2733)? { true } else { false } } } else { false } }) }; if !t2736.truthy() { t2736 } else { { let t2734 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t2735 = Py::Int(1i64); if py::eq(&t2734, &t2735) { true } else { false } }) } } } } }, Py::str("the harmonic order n must be an odd whole number from 3 to 99"))?;
    v_V1 = py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_Vdc.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &c_SQ2()?)?)?;
    v_Vn = f__snap(py::abs(&py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_Vdc.clone())?, &py::math1("cos", &py::mul(&v_n.clone(), &v_alpha.clone())?)?)?, &py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &c_SQ2()?)?)?)?, v_Vdc.clone())?;
    v_Vrms = py::mul(&v_Vdc.clone(), &py::math1("sqrt", &py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&Py::Float(2.0), &v_alpha.clone())?, &Py::Float(std::f64::consts::PI))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V1_rms"), v_V1.clone()), (Py::str("Vn_rms"), v_Vn.clone()), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("THD"), py::truediv(&py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_Vrms.clone(), &v_Vrms.clone())?, &py::mul(&v_V1.clone(), &v_V1.clone())?)?, Py::Float(0.0)], true)?)?, &v_V1.clone())?), (Py::str("alpha_eliminate_n_deg"), py::truediv(&Py::Float(90.0), &v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rc_snubber_ringing(mut v_f0: Py, mut v_f1: Py, mut v_Cadd: Py, mut v_k: Py, mut v_V: Py, mut v_fs: Py) -> R {
    let mut v_Cp: Py = Py::None;
    let mut v_Cs: Py = Py::None;
    let mut v_Lp: Py = Py::None;
    let mut v_Z0: Py = Py::None;
    let _ = f__pos(Py::str("f0, f1, Cadd, k, V and fs"), Py::tuple(vec![v_f0.clone(), v_f1.clone(), v_Cadd.clone(), v_k.clone(), v_V.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t2784 = v_f1.clone(); Py::Bool({ let t2785 = v_f0.clone(); if py::lt(&t2784, &t2785)? { true } else { false } }) }, Py::str("adding capacitance must lower the ringing frequency (f1 < f0)"))?;
    v_Cp = py::truediv(&v_Cadd.clone(), &py::sub(&py::pow(&py::truediv(&v_f0.clone(), &v_f1.clone())?, &Py::Int(2i64))?, &Py::Float(1.0))?)?;
    v_Lp = py::truediv(&Py::Float(1.0), &py::mul(&py::pow(&py::mul(&c_TWO_PI()?, &v_f0.clone())?, &Py::Int(2i64))?, &v_Cp.clone())?)?;
    v_Z0 = py::math1("sqrt", &py::truediv(&v_Lp.clone(), &v_Cp.clone())?)?;
    v_Cs = py::mul(&v_k.clone(), &v_Cp.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cp"), v_Cp.clone()), (Py::str("Lp"), v_Lp.clone()), (Py::str("Rs"), v_Z0.clone()), (Py::str("Cs"), v_Cs.clone()), (Py::str("P_R"), py::mul(&py::mul(&py::mul(&v_Cs.clone(), &v_V.clone())?, &v_V.clone())?, &v_fs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_regen_chopper(mut v_Vs: Py, mut v_D: Py, mut v_Ra: Py, mut v_K: Py, mut v_omega: Py) -> R {
    let mut v_Ea: Py = Py::None;
    let mut v_Ia: Py = Py::None;
    let mut v_Pm: Py = Py::None;
    let mut v_Pr: Py = Py::None;
    let mut v_Va: Py = Py::None;
    let _ = f__pos(Py::str("Vs, Ra, K and omega"), Py::tuple(vec![v_Vs.clone(), v_Ra.clone(), v_K.clone(), v_omega.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    v_Ea = py::mul(&v_K.clone(), &v_omega.clone())?;
    v_Va = py::mul(&py::sub(&Py::Float(1.0), &v_D.clone())?, &v_Vs.clone())?;
    v_Ia = py::truediv(&py::sub(&v_Ea.clone(), &v_Va.clone())?, &v_Ra.clone())?;
    let _ = f__need({ let t2953 = v_Ia.clone(); Py::Bool({ let t2954 = Py::Int(0i64); if py::gt(&t2953, &t2954)? { true } else { false } }) }, Py::str("the back EMF is below the average armature voltage: no regenerative braking at this duty cycle"))?;
    v_Pm = py::mul(&v_Ea.clone(), &v_Ia.clone())?;
    v_Pr = py::mul(&v_Va.clone(), &v_Ia.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ea"), v_Ea.clone()), (Py::str("Va"), v_Va.clone()), (Py::str("Ia"), v_Ia.clone()), (Py::str("T_brake"), py::mul(&v_K.clone(), &v_Ia.clone())?), (Py::str("P_mech"), v_Pm.clone()), (Py::str("P_regen"), v_Pr.clone()), (Py::str("efficiency"), py::truediv(&v_Pr.clone(), &v_Pm.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reverse_recovery(mut v_Irm: Py, mut v_trr: Py, mut v_S: Py, mut v_Vr: Py, mut v_fs: Py) -> R {
    let mut v_Ed: Py = Py::None;
    let mut v_Es: Py = Py::None;
    let mut v_Qrr: Py = Py::None;
    let mut v_ta: Py = Py::None;
    let mut v_tb: Py = Py::None;
    let _ = f__pos(Py::str("Irm, trr, S, Vr and fs"), Py::tuple(vec![v_Irm.clone(), v_trr.clone(), v_S.clone(), v_Vr.clone(), v_fs.clone()]))?;
    v_ta = py::truediv(&v_trr.clone(), &py::add(&Py::Float(1.0), &v_S.clone())?)?;
    v_tb = py::truediv(&py::mul(&v_S.clone(), &v_trr.clone())?, &py::add(&Py::Float(1.0), &v_S.clone())?)?;
    v_Qrr = py::mul(&py::mul(&Py::Float(0.5), &v_Irm.clone())?, &v_trr.clone())?;
    v_Ed = py::truediv(&py::mul(&py::mul(&v_Vr.clone(), &v_Irm.clone())?, &v_tb.clone())?, &Py::Float(6.0))?;
    v_Es = py::mul(&py::mul(&v_Vr.clone(), &v_Irm.clone())?, &py::add(&py::truediv(&v_ta.clone(), &Py::Float(2.0))?, &py::truediv(&v_tb.clone(), &Py::Float(3.0))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Qrr"), v_Qrr.clone()), (Py::str("ta"), v_ta.clone()), (Py::str("tb"), v_tb.clone()), (Py::str("E_diode"), v_Ed.clone()), (Py::str("E_switch"), v_Es.clone()), (Py::str("P_diode"), py::mul(&v_Ed.clone(), &v_fs.clone())?), (Py::str("P_switch"), py::mul(&v_Es.clone(), &v_fs.clone())?), (Py::str("P_total"), py::mul(&py::add(&v_Ed.clone(), &v_Es.clone())?, &v_fs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sepic_ccm(mut v_Vin: Py, mut v_Vout: Py, mut v_L1: Py, mut v_L2: Py, mut v_C1: Py, mut v_fs: Py, mut v_Io: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_IL1: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Vout, L1, L2, C1, fs and Io"), Py::tuple(vec![v_Vin.clone(), v_Vout.clone(), v_L1.clone(), v_L2.clone(), v_C1.clone(), v_fs.clone(), v_Io.clone()]))?;
    v_D = py::truediv(&v_Vout.clone(), &py::add(&v_Vin.clone(), &v_Vout.clone())?)?;
    v_T = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_IL1 = py::truediv(&py::mul(&v_Io.clone(), &v_Vout.clone())?, &v_Vin.clone())?;
    let t2999 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&v_Vin.clone(), &v_D.clone())?, &v_T.clone())?, &v_L1.clone())?, py::truediv(&py::mul(&py::mul(&v_Vin.clone(), &v_D.clone())?, &v_T.clone())?, &v_L2.clone())?])).items()?;
    if t2999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t2999[0].clone();
    v_d2 = t2999[1].clone();
    let _ = f__ccm_ok(py::sub(&v_IL1.clone(), &py::truediv(&v_d1.clone(), &Py::Float(2.0))?)?, v_IL1.clone())?;
    let _ = f__ccm_ok(py::sub(&v_Io.clone(), &py::truediv(&v_d2.clone(), &Py::Float(2.0))?)?, v_Io.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("IL1"), v_IL1.clone()), (Py::str("IL2"), v_Io.clone()), (Py::str("VC1"), v_Vin.clone()), (Py::str("dIL1"), v_d1.clone()), (Py::str("dIL2"), v_d2.clone()), (Py::str("dVC1"), py::truediv(&py::mul(&py::mul(&v_Io.clone(), &v_D.clone())?, &v_T.clone())?, &v_C1.clone())?), (Py::str("V_switch"), py::add(&v_Vin.clone(), &v_Vout.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_resonant(mut v_Lr: Py, mut v_Cr: Py, mut v_R: Py, mut v_f: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Z0: Py = Py::None;
    let mut v_fn: Py = Py::None;
    let mut v_fr: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let _ = f__pos(Py::str("Lr, Cr, R and f"), Py::tuple(vec![v_Lr.clone(), v_Cr.clone(), v_R.clone(), v_f.clone()]))?;
    v_w0 = py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::mul(&v_Lr.clone(), &v_Cr.clone())?)?)?;
    v_fr = py::truediv(&v_w0.clone(), &c_TWO_PI()?)?;
    v_Z0 = py::math1("sqrt", &py::truediv(&v_Lr.clone(), &v_Cr.clone())?)?;
    v_Q = py::truediv(&v_Z0.clone(), &v_R.clone())?;
    v_fn = py::truediv(&v_f.clone(), &v_fr.clone())?;
    v_M = py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::mul(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::pow(&py::sub(&v_fn.clone(), &py::truediv(&Py::Float(1.0), &v_fn.clone())?)?, &Py::Int(2i64))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("fr"), v_fr.clone()), (Py::str("Z0"), v_Z0.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("fn"), v_fn.clone()), (Py::str("M"), v_M.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_overlap(mut v_Vs: Py, mut v_f: Py, mut v_Ls: Py, mut v_Id: Py, mut v_alpha: Py) -> R {
    let mut v_Vm: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("Vs, f, Ls and Id"), Py::tuple(vec![v_Vs.clone(), v_f.clone(), v_Ls.clone(), v_Id.clone()]))?;
    let _ = f__need({ let t3147 = Py::Int(0i64); Py::Bool({ let t3148 = v_alpha.clone(); if py::le(&t3147, &t3148)? { { let t3149 = Py::Float(std::f64::consts::PI); if py::lt(&t3148, &t3149)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Vm = py::mul(&c_SQ2()?, &v_Vs.clone())?;
    v_w = py::mul(&c_TWO_PI()?, &v_f.clone())?;
    v_c = py::sub(&py::math1("cos", &v_alpha.clone())?, &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_w.clone())?, &v_Ls.clone())?, &v_Id.clone())?, &v_Vm.clone())?)?;
    let _ = f__need({ let t3150 = v_c.clone(); Py::Bool({ let t3151 = py::neg(&Py::Float(1.0))?; if py::ge(&t3150, &t3151)? { true } else { false } }) }, Py::str("commutation cannot finish within the half-cycle at this current (commutation failure)"))?;
    v_u = py::sub(&py::math1("acos", &v_c.clone())?, &v_alpha.clone())?;
    v_dV = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_w.clone())?, &v_Ls.clone())?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("u_deg"), py::math1("degrees", &v_u.clone())?), (Py::str("Vdc"), py::sub(&py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_Vm.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &Py::Float(std::f64::consts::PI))?, &v_dV.clone())?), (Py::str("dV"), v_dV.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_single_phase_controlled(mut v_circuit: Py, mut v_Vs: Py, mut v_alpha: Py) -> R {
    let mut v_Vdc: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_Vrms: Py = Py::None;
    let mut v_ac: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let _ = f__pos(Py::str("Vs"), Py::tuple(vec![v_Vs.clone()]))?;
    let _ = f__need({ let t3090 = v_circuit.clone(); Py::Bool({ let t3091 = c__SPC()?; if py::contains(&t3091, &t3090)? { true } else { false } }) }, Py::str("circuit: half-wave, full-bridge or semi-converter"))?;
    let _ = f__need({ let t3092 = Py::Int(0i64); Py::Bool({ let t3093 = v_alpha.clone(); if py::le(&t3092, &t3093)? { { let t3094 = Py::Float(std::f64::consts::PI); if py::lt(&t3093, &t3094)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Vm = py::mul(&c_SQ2()?, &v_Vs.clone())?;
    v_s2 = py::truediv(&py::math1("sin", &py::mul(&Py::Float(2.0), &v_alpha.clone())?)?, &Py::Float(2.0))?;
    if ({ let t3095 = v_circuit.clone(); Py::Bool({ let t3096 = Py::str("half-wave"); if py::eq(&t3095, &t3096) { true } else { false } }) }).truthy() {
        v_Vdc = py::truediv(&py::mul(&v_Vm.clone(), &py::add(&Py::Float(1.0), &py::math1("cos", &v_alpha.clone())?)?)?, &c_TWO_PI()?)?;
        v_Vrms = py::mul(&v_Vm.clone(), &py::math1("sqrt", &py::truediv(&py::add(&py::sub(&Py::Float(std::f64::consts::PI), &v_alpha.clone())?, &v_s2.clone())?, &py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?)?)?)?;
    } else {
        if ({ let t3097 = v_circuit.clone(); Py::Bool({ let t3098 = Py::str("full-bridge"); if py::eq(&t3097, &t3098) { true } else { false } }) }).truthy() {
            v_Vdc = f__snap(py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_Vm.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &Py::Float(std::f64::consts::PI))?, v_Vm.clone())?;
            v_Vrms = v_Vs.clone();
        } else {
            v_Vdc = py::truediv(&py::mul(&v_Vm.clone(), &py::add(&Py::Float(1.0), &py::math1("cos", &v_alpha.clone())?)?)?, &Py::Float(std::f64::consts::PI))?;
            v_Vrms = py::mul(&v_Vm.clone(), &py::math1("sqrt", &py::truediv(&py::add(&py::sub(&Py::Float(std::f64::consts::PI), &v_alpha.clone())?, &v_s2.clone())?, &c_TWO_PI()?)?)?)?;
        }
    }
    v_ac = py::math1("sqrt", &py::minmax(&[py::sub(&py::mul(&v_Vrms.clone(), &v_Vrms.clone())?, &py::mul(&v_Vdc.clone(), &v_Vdc.clone())?)?, Py::Float(0.0)], true)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vdc"), v_Vdc.clone()), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("Vac_rms"), v_ac.clone()), (Py::str("mode"), (if ({ let t3113 = v_Vdc.clone(); Py::Bool({ let t3114 = Py::Int(0i64); if py::lt(&t3113, &t3114)? { true } else { false } }) }).truthy() { Py::str("inverting") } else { Py::str("rectifying") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_six_step(mut v_Vdc: Py, mut v_f: Py) -> R {
    let mut v_VLL: Py = Py::None;
    let mut v_VLL1: Py = Py::None;
    let _ = f__pos(Py::str("Vdc and f"), Py::tuple(vec![v_Vdc.clone(), v_f.clone()]))?;
    v_VLL = py::mul(&v_Vdc.clone(), &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?;
    v_VLL1 = py::truediv(&py::mul(&py::math1("sqrt", &Py::Float(6.0))?, &v_Vdc.clone())?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("VLL_rms"), v_VLL.clone()), (Py::str("VLL1_rms"), v_VLL1.clone()), (Py::str("Vph_rms"), py::truediv(&py::mul(&c_SQ2()?, &v_Vdc.clone())?, &Py::Float(3.0))?), (Py::str("Vph1_rms"), py::truediv(&py::mul(&c_SQ2()?, &v_Vdc.clone())?, &Py::Float(std::f64::consts::PI))?), (Py::str("THD"), py::truediv(&py::math1("sqrt", &py::sub(&py::mul(&v_VLL.clone(), &v_VLL.clone())?, &py::mul(&v_VLL1.clone(), &v_VLL1.clone())?)?)?, &v_VLL1.clone())?), (Py::str("VLL5_rms"), py::truediv(&v_VLL1.clone(), &Py::Float(5.0))?), (Py::str("VLL7_rms"), py::truediv(&v_VLL1.clone(), &Py::Float(7.0))?), (Py::str("f5"), py::mul(&Py::Float(5.0), &v_f.clone())?), (Py::str("f7"), py::mul(&Py::Float(7.0), &v_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_skin_effect(mut v_d: Py, mut v_f: Py, mut v_rho: Py, mut v_length: Py) -> R {
    let mut v_Rdc: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_j0: Py = Py::None;
    let mut v_j1: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("d, f, rho and length"), Py::tuple(vec![v_d.clone(), v_f.clone(), v_rho.clone(), v_length.clone()]))?;
    v_delta = py::math1("sqrt", &py::truediv(&v_rho.clone(), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_f.clone())?, &c_MU0()?)?)?)?;
    v_r = py::truediv(&v_d.clone(), &Py::Float(2.0))?;
    let _ = f__need({ let t3240 = py::truediv(&v_r.clone(), &v_delta.clone())?; Py::Bool({ let t3241 = Py::Float(10.0); if py::le(&t3240, &t3241)? { true } else { false } }) }, Py::str("the wire radius exceeds 10 skin depths (outside the range of this series evaluation)"))?;
    v_z = py::mul(&py::complex(&[Py::Float(1.0), py::neg(&Py::Float(1.0))?])?, &py::truediv(&v_r.clone(), &v_delta.clone())?)?;
    let t3247 = (f__bessel01(v_z.clone())?).items()?;
    if t3247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_j0 = t3247[0].clone();
    v_j1 = t3247[1].clone();
    v_ratio = py::real(&py::truediv(&py::mul(&py::truediv(&v_z.clone(), &Py::Float(2.0))?, &v_j0.clone())?, &v_j1.clone())?)?;
    v_Rdc = py::truediv(&py::mul(&v_rho.clone(), &v_length.clone())?, &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("delta"), v_delta.clone()), (Py::str("d_over_delta"), py::truediv(&v_d.clone(), &v_delta.clone())?), (Py::str("Rdc"), v_Rdc.clone()), (Py::str("Rac"), py::mul(&v_Rdc.clone(), &v_ratio.clone())?), (Py::str("ratio"), v_ratio.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_small_signal(mut v_topology: Py, mut v_Vin: Py, mut v_D: Py, mut v_L: Py, mut v_C: Py, mut v_R: Py) -> R {
    let mut v_Dp: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_g0: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let mut v_wz: Py = Py::None;
    let _ = f__pos(Py::str("Vin, L, C and R"), Py::tuple(vec![v_Vin.clone(), v_L.clone(), v_C.clone(), v_R.clone()]))?;
    let _ = f__frac(Py::str("the duty cycle D"), v_D.clone())?;
    let _ = f__need({ let t989 = v_topology.clone(); Py::Bool({ let t990 = c__TOPO3()?; if py::contains(&t990, &t989)? { true } else { false } }) }, Py::str("topology: buck, boost or buck-boost"))?;
    v_Dp = py::sub(&Py::Float(1.0), &v_D.clone())?;
    if ({ let t991 = v_topology.clone(); Py::Bool({ let t992 = Py::str("buck"); if py::eq(&t991, &t992) { true } else { false } }) }).truthy() {
        let t993 = (Py::tuple(vec![v_Vin.clone(), py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?, py::mul(&v_R.clone(), &py::math1("sqrt", &py::truediv(&v_C.clone(), &v_L.clone())?)?)?, Py::Float(0.0)])).items()?;
        if t993.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_g0 = t993[0].clone();
        v_w0 = t993[1].clone();
        v_Q = t993[2].clone();
        v_wz = t993[3].clone();
    } else {
        if ({ let t994 = v_topology.clone(); Py::Bool({ let t995 = Py::str("boost"); if py::eq(&t994, &t995) { true } else { false } }) }).truthy() {
            let t996 = (Py::tuple(vec![py::truediv(&v_Vin.clone(), &py::mul(&v_Dp.clone(), &v_Dp.clone())?)?, py::truediv(&v_Dp.clone(), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?, py::mul(&py::mul(&v_Dp.clone(), &v_R.clone())?, &py::math1("sqrt", &py::truediv(&v_C.clone(), &v_L.clone())?)?)?, py::truediv(&py::mul(&py::mul(&v_Dp.clone(), &v_Dp.clone())?, &v_R.clone())?, &v_L.clone())?])).items()?;
            if t996.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_g0 = t996[0].clone();
            v_w0 = t996[1].clone();
            v_Q = t996[2].clone();
            v_wz = t996[3].clone();
        } else {
            let t997 = (Py::tuple(vec![py::truediv(&v_Vin.clone(), &py::mul(&v_Dp.clone(), &v_Dp.clone())?)?, py::truediv(&v_Dp.clone(), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_C.clone())?)?)?, py::mul(&py::mul(&v_Dp.clone(), &v_R.clone())?, &py::math1("sqrt", &py::truediv(&v_C.clone(), &v_L.clone())?)?)?, py::truediv(&py::mul(&py::mul(&v_Dp.clone(), &v_Dp.clone())?, &v_R.clone())?, &py::mul(&v_D.clone(), &v_L.clone())?)?])).items()?;
            if t997.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_g0 = t997[0].clone();
            v_w0 = t997[1].clone();
            v_Q = t997[2].clone();
            v_wz = t997[3].clone();
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Gvd0"), v_g0.clone()), (Py::str("f0"), py::truediv(&v_w0.clone(), &c_TWO_PI()?)?), (Py::str("Q"), v_Q.clone()), (Py::str("f_rhpz"), py::truediv(&v_wz.clone(), &c_TWO_PI()?)?), (Py::str("rhp_zero"), (if ({ let t1010 = v_topology.clone(); Py::Bool({ let t1011 = Py::str("buck"); if py::eq(&t1010, &t1011) { true } else { false } }) }).truthy() { Py::str("none") } else { Py::str("present") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_space_vector(mut v_Vref: Py, mut v_theta: Py, mut v_Vdc: Py, mut v_fs: Py) -> R {
    let mut v_T0: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_Ts: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__pos(Py::str("Vref, Vdc and fs"), Py::tuple(vec![v_Vref.clone(), v_Vdc.clone(), v_fs.clone()]))?;
    let _ = f__need({ let t3292 = Py::Int(0i64); Py::Bool({ let t3293 = v_theta.clone(); if py::le(&t3292, &t3293)? { { let t3294 = c_TWO_PI()?; if py::lt(&t3293, &t3294)? { true } else { false } } } else { false } }) }, Py::str("theta must lie from 0 to 360 degrees"))?;
    v_k = py::int(&py::math1("floor", &py::truediv(&v_theta.clone(), &py::truediv(&Py::Float(std::f64::consts::PI), &Py::Float(3.0))?)?)?)?;
    v_k = py::minmax(&[v_k.clone(), Py::Int(5i64)], false)?;
    v_tp = py::sub(&v_theta.clone(), &py::truediv(&py::mul(&v_k.clone(), &Py::Float(std::f64::consts::PI))?, &Py::Float(3.0))?)?;
    v_Ts = py::truediv(&Py::Float(1.0), &v_fs.clone())?;
    v_c = py::truediv(&py::mul(&py::mul(&c_SQ3()?, &v_Vref.clone())?, &v_Ts.clone())?, &v_Vdc.clone())?;
    v_T1 = py::mul(&v_c.clone(), &py::math1("sin", &py::sub(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Float(3.0))?, &v_tp.clone())?)?)?;
    v_T2 = py::mul(&v_c.clone(), &py::math1("sin", &v_tp.clone())?)?;
    v_T0 = py::sub(&py::sub(&v_Ts.clone(), &v_T1.clone())?, &v_T2.clone())?;
    let _ = f__need({ let t3295 = v_T0.clone(); Py::Bool({ let t3296 = py::mul(&py::neg(&Py::Float(1e-12))?, &v_Ts.clone())?; if py::ge(&t3295, &t3296)? { true } else { false } }) }, Py::str("the reference lies outside the hexagon: overmodulation (T1 + T2 > Ts)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("sector"), py::add(&v_k.clone(), &Py::Int(1i64))?), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), f__snap(v_T2.clone(), v_Ts.clone())?), (Py::str("T0"), f__snap(py::minmax(&[v_T0.clone(), Py::Float(0.0)], true)?, v_Ts.clone())?), (Py::str("m"), py::truediv(&py::mul(&c_SQ3()?, &v_Vref.clone())?, &v_Vdc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_spwm(mut v_Vdc: Py, mut v_ma: Py, mut v_mf: Py, mut v_f1: Py) -> R {
    let mut v_pk: Py = Py::None;
    let _ = f__pos(Py::str("Vdc and f1"), Py::tuple(vec![v_Vdc.clone(), v_f1.clone()]))?;
    let _ = f__need({ let t3343 = Py::Int(0i64); Py::Bool({ let t3344 = v_ma.clone(); if py::lt(&t3343, &t3344)? { { let t3345 = Py::Int(1i64); if py::le(&t3344, &t3345)? { true } else { false } } } else { false } }) }, Py::str("the modulation index ma must lie in (0, 1] (linear modulation)"))?;
    let _ = f__need({ let t3352 = Py::Bool(matches!(v_mf.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3352.truthy() { t3352 } else { { let t3351 = { let t3346 = Py::Int(15i64); Py::Bool({ let t3347 = v_mf.clone(); if py::le(&t3346, &t3347)? { { let t3348 = Py::Int(201i64); if py::le(&t3347, &t3348)? { true } else { false } } } else { false } }) }; if !t3351.truthy() { t3351 } else { { let t3349 = py::modulo(&v_mf.clone(), &Py::Int(2i64))?; Py::Bool({ let t3350 = Py::Int(1i64); if py::eq(&t3349, &t3350) { true } else { false } }) } } } } }, Py::str("the frequency ratio mf must be an odd whole number from 15 to 201"))?;
    v_pk = py::truediv(&py::mul(&v_ma.clone(), &v_Vdc.clone())?, &Py::Float(2.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V1_leg_peak"), v_pk.clone()), (Py::str("V1_leg_rms"), py::truediv(&v_pk.clone(), &c_SQ2()?)?), (Py::str("V1_LL_rms"), py::truediv(&py::mul(&c_SQ3()?, &v_pk.clone())?, &c_SQ2()?)?), (Py::str("V1_fullbridge_rms"), py::truediv(&py::mul(&Py::Float(2.0), &v_pk.clone())?, &c_SQ2()?)?), (Py::str("f_carrier"), py::mul(&v_mf.clone(), &v_f1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_square_wave_inverter(mut v_Vdc: Py, mut v_bridge: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_V1: Py = Py::None;
    let _ = f__pos(Py::str("Vdc"), Py::tuple(vec![v_Vdc.clone()]))?;
    let _ = f__need({ let t3397 = v_bridge.clone(); Py::Bool({ let t3398 = c__BR2()?; if py::contains(&t3398, &t3397)? { true } else { false } }) }, Py::str("bridge: half-bridge or full-bridge"))?;
    let _ = f__need({ let t3405 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3405.truthy() { t3405 } else { { let t3404 = { let t3399 = Py::Int(3i64); Py::Bool({ let t3400 = v_n.clone(); if py::le(&t3399, &t3400)? { { let t3401 = Py::Int(99i64); if py::le(&t3400, &t3401)? { true } else { false } } } else { false } }) }; if !t3404.truthy() { t3404 } else { { let t3402 = py::modulo(&v_n.clone(), &Py::Int(2i64))?; Py::Bool({ let t3403 = Py::Int(1i64); if py::eq(&t3402, &t3403) { true } else { false } }) } } } } }, Py::str("the harmonic order n must be an odd whole number from 3 to 99"))?;
    v_A = (if ({ let t3406 = v_bridge.clone(); Py::Bool({ let t3407 = Py::str("full-bridge"); if py::eq(&t3406, &t3407) { true } else { false } }) }).truthy() { v_Vdc.clone() } else { py::truediv(&v_Vdc.clone(), &Py::Float(2.0))? });
    v_V1 = py::truediv(&py::mul(&Py::Float(4.0), &v_A.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &c_SQ2()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vrms"), v_A.clone()), (Py::str("V1_rms"), v_V1.clone()), (Py::str("V1_peak"), py::truediv(&py::mul(&Py::Float(4.0), &v_A.clone())?, &Py::Float(std::f64::consts::PI))?), (Py::str("Vn_rms"), py::truediv(&v_V1.clone(), &v_n.clone())?), (Py::str("THD"), py::truediv(&py::math1("sqrt", &py::sub(&py::mul(&v_A.clone(), &v_A.clone())?, &py::mul(&v_V1.clone(), &v_V1.clone())?)?)?, &v_V1.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_steinmetz(mut v_k: Py, mut v_alpha: Py, mut v_beta: Py, mut v_f: Py, mut v_Bpk: Py, mut v_Vc: Py) -> R {
    let mut v_Pv: Py = Py::None;
    let _ = f__pos(Py::str("k, alpha, beta, f, Bpk and Vc"), Py::tuple(vec![v_k.clone(), v_alpha.clone(), v_beta.clone(), v_f.clone(), v_Bpk.clone(), v_Vc.clone()]))?;
    v_Pv = py::mul(&py::mul(&v_k.clone(), &py::pow(&v_f.clone(), &v_alpha.clone())?)?, &py::pow(&v_Bpk.clone(), &v_beta.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Pv"), v_Pv.clone()), (Py::str("Pv_mW_cm3"), py::mul(&v_Pv.clone(), &Py::Float(0.001))?), (Py::str("P"), py::mul(&v_Pv.clone(), &v_Vc.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_switching_loss(mut v_V: Py, mut v_I: Py, mut v_tr: Py, mut v_tf: Py, mut v_fs: Py, mut v_Coss: Py) -> R {
    let mut v_Eoff: Py = Py::None;
    let mut v_Eon: Py = Py::None;
    let mut v_Poss: Py = Py::None;
    let mut v_Psw: Py = Py::None;
    let _ = f__pos(Py::str("V, I and fs"), Py::tuple(vec![v_V.clone(), v_I.clone(), v_fs.clone()]))?;
    let _ = f__nonneg(Py::str("tr, tf and Coss"), Py::tuple(vec![v_tr.clone(), v_tf.clone(), v_Coss.clone()]))?;
    v_Eon = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_V.clone())?, &v_I.clone())?, &v_tr.clone())?;
    v_Eoff = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_V.clone())?, &v_I.clone())?, &v_tf.clone())?;
    v_Psw = py::mul(&py::add(&v_Eon.clone(), &v_Eoff.clone())?, &v_fs.clone())?;
    v_Poss = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Coss.clone())?, &v_V.clone())?, &v_V.clone())?, &v_fs.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("E_on"), v_Eon.clone()), (Py::str("E_off"), v_Eoff.clone()), (Py::str("P_sw"), v_Psw.clone()), (Py::str("P_oss"), v_Poss.clone()), (Py::str("P_total"), py::add(&v_Psw.clone(), &v_Poss.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thd_harmonics(mut v_V1: Py, mut v_Vh: Py) -> R {
    let mut v_Vrms: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_thd: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the fundamental V1"), Py::tuple(vec![v_V1.clone()]))?;
    let _ = f__need({ let t3549 = Py::Bool(matches!(v_Vh.clone(), Py::List(_))); if !t3549.truthy() { t3549 } else { { let t3546 = Py::Int(1i64); Py::Bool({ let t3547 = py::len(&v_Vh.clone())?; if py::le(&t3546, &t3547)? { { let t3548 = Py::Int(12i64); if py::le(&t3547, &t3548)? { true } else { false } } } else { false } }) } } }, Py::str("give 1 to 12 harmonic values"))?;
    for t3550 in py::iter(&(v_Vh.clone()))? {
        v_v = t3550;
        let _ = f__nonneg(Py::str("every harmonic value"), Py::tuple(vec![v_v.clone()]))?;
    }
    v_s = Py::Float(0.0);
    for t3557 in py::iter(&(v_Vh.clone()))? {
        v_v = t3557;
        v_s = py::iadd(&v_s, &py::mul(&v_v.clone(), &v_v.clone())?)?;
    }
    v_thd = py::truediv(&py::math1("sqrt", &v_s.clone())?, &v_V1.clone())?;
    v_Vrms = py::math1("sqrt", &py::add(&py::mul(&v_V1.clone(), &v_V1.clone())?, &v_s.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("THD"), v_thd.clone()), (Py::str("THD_pct"), py::mul(&Py::Float(100.0), &v_thd.clone())?), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("distortion_factor"), py::truediv(&v_V1.clone(), &v_Vrms.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thermal_multi(mut v_P: Py, mut v_Rjc: Py, mut v_Rcs: Py, mut v_Rsa: Py, mut v_Ta: Py) -> R {
    let mut v_Tj: Py = Py::None;
    let mut v_Ts: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t3599 = Py::Bool(matches!(v_P.clone(), Py::List(_))); if !t3599.truthy() { t3599 } else { { let t3596 = Py::Int(1i64); Py::Bool({ let t3597 = py::len(&v_P.clone())?; if py::le(&t3596, &t3597)? { { let t3598 = Py::Int(12i64); if py::le(&t3597, &t3598)? { true } else { false } } } else { false } }) } } }, Py::str("give 1 to 12 device losses"))?;
    for t3600 in py::iter(&(v_P.clone()))? {
        v_x = t3600;
        let _ = f__nonneg(Py::str("every device loss"), Py::tuple(vec![v_x.clone()]))?;
    }
    let _ = f__pos(Py::str("the thermal resistances and Ta"), Py::tuple(vec![v_Rjc.clone(), v_Rcs.clone(), v_Rsa.clone(), v_Ta.clone()]))?;
    v_tot = Py::Float(0.0);
    for t3613 in py::iter(&(v_P.clone()))? {
        v_x = t3613;
        v_tot = py::iadd(&v_tot, &v_x.clone())?;
    }
    v_Ts = py::add(&v_Ta.clone(), &py::mul(&v_Rsa.clone(), &v_tot.clone())?)?;
    v_Tj = { let mut out = vec![]; for t3614 in py::iter(&(v_P.clone()))? { let mut v_x: Py; 
v_x = t3614;
out.push(py::add(&v_Ts.clone(), &py::mul(&v_x.clone(), &py::add(&v_Rjc.clone(), &v_Rcs.clone())?)?)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("P_total"), v_tot.clone()), (Py::str("Ts"), v_Ts.clone()), (Py::str("Tj"), v_Tj.clone()), (Py::str("Tj_max"), py::minmax(&[v_Tj.clone()], true)?), (Py::str("Tj_max_C"), py::sub(&py::minmax(&[v_Tj.clone()], true)?, &c_T0C()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_overlap(mut v_VLL: Py, mut v_f: Py, mut v_Ls: Py, mut v_Id: Py, mut v_alpha: Py) -> R {
    let mut v_Vm: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dV: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("VLL, f, Ls and Id"), Py::tuple(vec![v_VLL.clone(), v_f.clone(), v_Ls.clone(), v_Id.clone()]))?;
    let _ = f__need({ let t3798 = Py::Int(0i64); Py::Bool({ let t3799 = v_alpha.clone(); if py::le(&t3798, &t3799)? { { let t3800 = Py::Float(std::f64::consts::PI); if py::lt(&t3799, &t3800)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Vm = py::mul(&c_SQ2()?, &v_VLL.clone())?;
    v_w = py::mul(&c_TWO_PI()?, &v_f.clone())?;
    v_c = py::sub(&py::math1("cos", &v_alpha.clone())?, &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_w.clone())?, &v_Ls.clone())?, &v_Id.clone())?, &v_Vm.clone())?)?;
    let _ = f__need({ let t3801 = v_c.clone(); Py::Bool({ let t3802 = py::neg(&Py::Float(1.0))?; if py::ge(&t3801, &t3802)? { true } else { false } }) }, Py::str("commutation cannot finish at this current (commutation failure)"))?;
    v_u = py::sub(&py::math1("acos", &v_c.clone())?, &v_alpha.clone())?;
    v_dV = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(3.0), &v_w.clone())?, &v_Ls.clone())?, &v_Id.clone())?, &Py::Float(std::f64::consts::PI))?;
    return Ok(f__out(Py::dict(vec![(Py::str("u_deg"), py::math1("degrees", &v_u.clone())?), (Py::str("Vdc"), py::sub(&py::truediv(&py::mul(&py::mul(&Py::Float(3.0), &v_Vm.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &Py::Float(std::f64::consts::PI))?, &v_dV.clone())?), (Py::str("dV"), v_dV.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_phase_bridge(mut v_VLL: Py, mut v_f: Py, mut v_R: Py) -> R {
    let mut v_Vdc: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_Vrms: Py = Py::None;
    let mut v_ff: Py = Py::None;
    let _ = f__pos(Py::str("VLL, f and R"), Py::tuple(vec![v_VLL.clone(), v_f.clone(), v_R.clone()]))?;
    v_Vm = py::mul(&c_SQ2()?, &v_VLL.clone())?;
    v_Vdc = py::truediv(&py::mul(&Py::Float(3.0), &v_Vm.clone())?, &Py::Float(std::f64::consts::PI))?;
    v_Vrms = py::mul(&v_Vm.clone(), &py::math1("sqrt", &py::add(&Py::Float(0.5), &py::truediv(&py::mul(&Py::Float(3.0), &c_SQ3()?)?, &py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?)?)?)?)?;
    v_ff = py::truediv(&v_Vrms.clone(), &v_Vdc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vdc"), v_Vdc.clone()), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("ripple_factor"), py::math1("sqrt", &py::sub(&py::mul(&v_ff.clone(), &v_ff.clone())?, &Py::Float(1.0))?)?), (Py::str("form_factor"), v_ff.clone()), (Py::str("f_ripple"), py::mul(&Py::Float(6.0), &v_f.clone())?), (Py::str("PIV"), v_Vm.clone()), (Py::str("Idc"), py::truediv(&v_Vdc.clone(), &v_R.clone())?), (Py::str("I_diode_avg"), py::truediv(&v_Vdc.clone(), &py::mul(&Py::Float(3.0), &v_R.clone())?)?), (Py::str("I_diode_rms"), py::truediv(&v_Vrms.clone(), &py::mul(&v_R.clone(), &c_SQ3()?)?)?), (Py::str("Is_rms"), py::mul(&py::truediv(&v_Vrms.clone(), &v_R.clone())?, &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_phase_controlled(mut v_VLL: Py, mut v_alpha: Py, mut v_Id: Py) -> R {
    let mut v_Vdc: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_Vrms: Py = Py::None;
    let _ = f__pos(Py::str("VLL and Id"), Py::tuple(vec![v_VLL.clone(), v_Id.clone()]))?;
    let _ = f__need({ let t3659 = Py::Int(0i64); Py::Bool({ let t3660 = v_alpha.clone(); if py::le(&t3659, &t3660)? { { let t3661 = Py::Float(std::f64::consts::PI); if py::lt(&t3660, &t3661)? { true } else { false } } } else { false } }) }, Py::str("the firing angle must lie from 0 to 180 degrees"))?;
    v_Vm = py::mul(&c_SQ2()?, &v_VLL.clone())?;
    v_Vdc = f__snap(py::truediv(&py::mul(&py::mul(&Py::Float(3.0), &v_Vm.clone())?, &py::math1("cos", &v_alpha.clone())?)?, &Py::Float(std::f64::consts::PI))?, v_Vm.clone())?;
    v_Vrms = py::mul(&v_Vm.clone(), &py::math1("sqrt", &py::add(&Py::Float(0.5), &py::truediv(&py::mul(&py::mul(&Py::Float(3.0), &c_SQ3()?)?, &py::math1("cos", &py::mul(&Py::Float(2.0), &v_alpha.clone())?)?)?, &py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vdc"), v_Vdc.clone()), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("P"), py::mul(&v_Vdc.clone(), &v_Id.clone())?), (Py::str("I_thy_avg"), py::truediv(&v_Id.clone(), &Py::Float(3.0))?), (Py::str("I_thy_rms"), py::truediv(&v_Id.clone(), &c_SQ3()?)?), (Py::str("Is_rms"), py::mul(&v_Id.clone(), &py::math1("sqrt", &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?)?), (Py::str("mode"), (if ({ let t3676 = v_Vdc.clone(); Py::Bool({ let t3677 = Py::Int(0i64); if py::lt(&t3676, &t3677)? { true } else { false } }) }).truthy() { Py::str("inverting") } else { Py::str("rectifying") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_phase_half_wave(mut v_Vph: Py, mut v_f: Py, mut v_R: Py) -> R {
    let mut v_Vdc: Py = Py::None;
    let mut v_Vm: Py = Py::None;
    let mut v_Vrms: Py = Py::None;
    let mut v_ff: Py = Py::None;
    let _ = f__pos(Py::str("Vph, f and R"), Py::tuple(vec![v_Vph.clone(), v_f.clone(), v_R.clone()]))?;
    v_Vm = py::mul(&c_SQ2()?, &v_Vph.clone())?;
    v_Vdc = py::truediv(&py::mul(&py::mul(&Py::Float(3.0), &c_SQ3()?)?, &v_Vm.clone())?, &c_TWO_PI()?)?;
    v_Vrms = py::mul(&v_Vm.clone(), &py::math1("sqrt", &py::add(&Py::Float(0.5), &py::truediv(&py::mul(&Py::Float(3.0), &c_SQ3()?)?, &py::mul(&Py::Float(8.0), &Py::Float(std::f64::consts::PI))?)?)?)?)?;
    v_ff = py::truediv(&v_Vrms.clone(), &v_Vdc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vdc"), v_Vdc.clone()), (Py::str("Vrms"), v_Vrms.clone()), (Py::str("ripple_factor"), py::math1("sqrt", &py::sub(&py::mul(&v_ff.clone(), &v_ff.clone())?, &Py::Float(1.0))?)?), (Py::str("form_factor"), v_ff.clone()), (Py::str("f_ripple"), py::mul(&Py::Float(3.0), &v_f.clone())?), (Py::str("PIV"), py::mul(&c_SQ3()?, &v_Vm.clone())?), (Py::str("Idc"), py::truediv(&v_Vdc.clone(), &v_R.clone())?), (Py::str("I_diode_avg"), py::truediv(&v_Vdc.clone(), &py::mul(&Py::Float(3.0), &v_R.clone())?)?), (Py::str("I_diode_rms"), py::truediv(&v_Vrms.clone(), &py::mul(&v_R.clone(), &c_SQ3()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_area_product(mut v_Po: Py, mut v_eff: Py, mut v_waveform: Py, mut v_Bm: Py, mut v_f: Py, mut v_J: Py, mut v_Ku: Py) -> R {
    let mut v_Ap: Py = Py::None;
    let mut v_Pt: Py = Py::None;
    let _ = f__pos(Py::str("Po, Bm, f and J"), Py::tuple(vec![v_Po.clone(), v_Bm.clone(), v_f.clone(), v_J.clone()]))?;
    let _ = f__frac(Py::str("the efficiency"), v_eff.clone())?;
    let _ = f__frac(Py::str("the window utilisation Ku"), v_Ku.clone())?;
    let _ = f__need({ let t3853 = v_waveform.clone(); Py::Bool({ let t3854 = c__WAVE()?; if py::contains(&t3854, &t3853)? { true } else { false } }) }, Py::str("waveform: sine or square"))?;
    v_Pt = py::mul(&v_Po.clone(), &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_eff.clone())?)?)?;
    v_Ap = py::truediv(&v_Pt.clone(), &py::mul(&py::mul(&py::mul(&py::mul(&f__kf(v_waveform.clone())?, &v_Ku.clone())?, &v_Bm.clone())?, &v_f.clone())?, &v_J.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Pt"), v_Pt.clone()), (Py::str("Ap"), v_Ap.clone()), (Py::str("Ap_cm4"), py::mul(&v_Ap.clone(), &Py::Float(100000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformer_turns(mut v_waveform: Py, mut v_V: Py, mut v_f: Py, mut v_Bmax: Py, mut v_Ac: Py, mut v_Vs: Py) -> R {
    let mut v_Np: Py = Py::None;
    let _ = f__pos(Py::str("V, f, Bmax, Ac and Vs"), Py::tuple(vec![v_V.clone(), v_f.clone(), v_Bmax.clone(), v_Ac.clone(), v_Vs.clone()]))?;
    let _ = f__need({ let t3901 = v_waveform.clone(); Py::Bool({ let t3902 = c__WAVE()?; if py::contains(&t3902, &t3901)? { true } else { false } }) }, Py::str("waveform: sine or square"))?;
    v_Np = py::truediv(&v_V.clone(), &py::mul(&py::mul(&py::mul(&f__kf(v_waveform.clone())?, &v_f.clone())?, &v_Bmax.clone())?, &v_Ac.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Np"), v_Np.clone()), (Py::str("Ns"), py::truediv(&py::mul(&v_Np.clone(), &v_Vs.clone())?, &v_V.clone())?), (Py::str("n"), py::truediv(&v_V.clone(), &v_Vs.clone())?), (Py::str("volts_per_turn"), py::truediv(&v_V.clone(), &v_Np.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_turnoff_snubber(mut v_I: Py, mut v_V: Py, mut v_tf: Py, mut v_fs: Py) -> R {
    let mut v_Cs: Py = Py::None;
    let mut v_Es: Py = Py::None;
    let mut v_Ew: Py = Py::None;
    let mut v_Ewo: Py = Py::None;
    let _ = f__pos(Py::str("I, V, tf and fs"), Py::tuple(vec![v_I.clone(), v_V.clone(), v_tf.clone(), v_fs.clone()]))?;
    v_Cs = py::truediv(&py::mul(&v_I.clone(), &v_tf.clone())?, &py::mul(&Py::Float(2.0), &v_V.clone())?)?;
    v_Ew = py::truediv(&py::mul(&py::mul(&py::mul(&v_I.clone(), &v_I.clone())?, &v_tf.clone())?, &v_tf.clone())?, &py::mul(&Py::Float(24.0), &v_Cs.clone())?)?;
    v_Ewo = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_V.clone())?, &v_I.clone())?, &v_tf.clone())?;
    v_Es = py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Cs.clone())?, &v_V.clone())?, &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Cs"), v_Cs.clone()), (Py::str("E_off_with"), v_Ew.clone()), (Py::str("E_off_without"), v_Ewo.clone()), (Py::str("E_snubber"), v_Es.clone()), (Py::str("P_switch"), py::mul(&v_Ew.clone(), &v_fs.clone())?), (Py::str("P_snubber"), py::mul(&v_Es.clone(), &v_fs.clone())?), (Py::str("P_total"), py::mul(&py::add(&v_Ew.clone(), &v_Es.clone())?, &v_fs.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zvs_transition(mut v_Vin: Py, mut v_Ceq: Py, mut v_L: Py, mut v_I0: Py) -> R {
    let mut v_Imin: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("Vin, Ceq, L and I0"), Py::tuple(vec![v_Vin.clone(), v_Ceq.clone(), v_L.clone(), v_I0.clone()]))?;
    v_Z = py::math1("sqrt", &py::truediv(&v_L.clone(), &v_Ceq.clone())?)?;
    v_w = py::truediv(&Py::Float(1.0), &py::math1("sqrt", &py::mul(&v_L.clone(), &v_Ceq.clone())?)?)?;
    v_Imin = py::truediv(&v_Vin.clone(), &v_Z.clone())?;
    v_ok = { let t3993 = py::mul(&v_I0.clone(), &v_Z.clone())?; Py::Bool({ let t3994 = v_Vin.clone(); if py::ge(&t3993, &t3994)? { true } else { false } }) };
    v_t = (if (v_ok.clone()).truthy() { py::truediv(&py::math1("asin", &py::truediv(&v_Vin.clone(), &py::mul(&v_I0.clone(), &v_Z.clone())?)?)?, &v_w.clone())? } else { py::truediv(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Float(2.0))?, &v_w.clone())? });
    return Ok(f__out(Py::dict(vec![(Py::str("Z"), v_Z.clone()), (Py::str("f_res"), py::truediv(&v_w.clone(), &c_TWO_PI()?)?), (Py::str("I_min"), v_Imin.clone()), (Py::str("E_L"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_L.clone())?, &v_I0.clone())?, &v_I0.clone())?), (Py::str("E_C"), py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Ceq.clone())?, &v_Vin.clone())?, &v_Vin.clone())?), (Py::str("zvs"), (if (v_ok.clone()).truthy() { Py::str("achieved") } else { Py::str("not achieved") })), (Py::str("t_transition"), v_t.clone()), (Py::str("v_reached"), py::minmax(&[py::mul(&v_I0.clone(), &v_Z.clone())?, v_Vin.clone()], false)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1012(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1014: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1013 = v_inputs.clone(); f_small_signal(py::getitem(&t1013, &Py::str("topology"))?, py::getitem(&t1013, &Py::str("Vin"))?, py::getitem(&t1013, &Py::str("D"))?, py::getitem(&t1013, &Py::str("L"))?, py::getitem(&t1013, &Py::str("C"))?, py::getitem(&t1013, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1014 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1015 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1037 = (t1015).items()?;
        if t1037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1037[0].clone();
        v_v = t1037[1].clone();
        if ({ let t1016 = v_k.clone(); Py::Bool({ let t1017 = v_outputs.clone(); if !py::contains(&t1017, &t1016)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1066(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1068: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1067 = v_inputs.clone(); f_critical_inductance(py::getitem(&t1067, &Py::str("topology"))?, py::getitem(&t1067, &Py::str("Vin"))?, py::getitem(&t1067, &Py::str("Vout"))?, py::getitem(&t1067, &Py::str("fs"))?, py::getitem(&t1067, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1068 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1069 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1091 = (t1069).items()?;
        if t1091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1091[0].clone();
        v_v = t1091[1].clone();
        if ({ let t1070 = v_k.clone(); Py::Bool({ let t1071 = v_outputs.clone(); if !py::contains(&t1071, &t1070)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1114 = v_inputs.clone(); f_cuk_ccm(py::getitem(&t1114, &Py::str("Vin"))?, py::getitem(&t1114, &Py::str("Vout"))?, py::getitem(&t1114, &Py::str("L1"))?, py::getitem(&t1114, &Py::str("L2"))?, py::getitem(&t1114, &Py::str("C1"))?, py::getitem(&t1114, &Py::str("fs"))?, py::getitem(&t1114, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1138 = (t1116).items()?;
        if t1138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1138[0].clone();
        v_v = t1138[1].clone();
        if ({ let t1117 = v_k.clone(); Py::Bool({ let t1118 = v_outputs.clone(); if !py::contains(&t1118, &t1117)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1168(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1169 = v_inputs.clone(); f_dc_drive_rectifier(py::getitem(&t1169, &Py::str("converter"))?, py::getitem(&t1169, &Py::str("Vs"))?, py::getitem(&t1169, &Py::str("alpha"))?, py::getitem(&t1169, &Py::str("Ra"))?, py::getitem(&t1169, &Py::str("K"))?, py::getitem(&t1169, &Py::str("Ia"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1171 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1193 = (t1171).items()?;
        if t1193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1193[0].clone();
        v_v = t1193[1].clone();
        if ({ let t1172 = v_k.clone(); Py::Bool({ let t1173 = v_outputs.clone(); if !py::contains(&t1173, &t1172)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1226(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1228: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1227 = v_inputs.clone(); f_armature_ripple(py::getitem(&t1227, &Py::str("Vs"))?, py::getitem(&t1227, &Py::str("D"))?, py::getitem(&t1227, &Py::str("fs"))?, py::getitem(&t1227, &Py::str("Ra"))?, py::getitem(&t1227, &Py::str("La"))?, py::getitem(&t1227, &Py::str("Ea"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1228 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1229 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1251 = (t1229).items()?;
        if t1251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1251[0].clone();
        v_v = t1251[1].clone();
        if ({ let t1230 = v_k.clone(); Py::Bool({ let t1231 = v_outputs.clone(); if !py::contains(&t1231, &t1230)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_126(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t128: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t127 = v_inputs.clone(); f_boost_ccm(py::getitem(&t127, &Py::str("Vin"))?, py::getitem(&t127, &Py::str("Vout"))?, py::getitem(&t127, &Py::str("L"))?, py::getitem(&t127, &Py::str("fs"))?, py::getitem(&t127, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t128 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t129 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t151 = (t129).items()?;
        if t151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t151[0].clone();
        v_v = t151[1].clone();
        if ({ let t130 = v_k.clone(); Py::Bool({ let t131 = v_outputs.clone(); if !py::contains(&t131, &t130)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1288: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1287 = v_inputs.clone(); f_dc_motor_chopper(py::getitem(&t1287, &Py::str("Vs"))?, py::getitem(&t1287, &Py::str("D"))?, py::getitem(&t1287, &Py::str("Ra"))?, py::getitem(&t1287, &Py::str("K"))?, py::getitem(&t1287, &Py::str("TL"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1288 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1289 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1311 = (t1289).items()?;
        if t1311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1311[0].clone();
        v_v = t1311[1].clone();
        if ({ let t1290 = v_k.clone(); Py::Bool({ let t1291 = v_outputs.clone(); if !py::contains(&t1291, &t1290)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1334: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1333 = v_inputs.clone(); f_dead_time_error(py::getitem(&t1333, &Py::str("Vdc"))?, py::getitem(&t1333, &Py::str("td"))?, py::getitem(&t1333, &Py::str("fs"))?, py::getitem(&t1333, &Py::str("V1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1334 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1335 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1357 = (t1335).items()?;
        if t1357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1357[0].clone();
        v_v = t1357[1].clone();
        if ({ let t1336 = v_k.clone(); Py::Bool({ let t1337 = v_outputs.clone(); if !py::contains(&t1337, &t1336)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1376(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1378: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1377 = v_inputs.clone(); f_reverse_recovery(py::getitem(&t1377, &Py::str("Irm"))?, py::getitem(&t1377, &Py::str("trr"))?, py::getitem(&t1377, &Py::str("S"))?, py::getitem(&t1377, &Py::str("Vr"))?, py::getitem(&t1377, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1378 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1379 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1401 = (t1379).items()?;
        if t1401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1401[0].clone();
        v_v = t1401[1].clone();
        if ({ let t1380 = v_k.clone(); Py::Bool({ let t1381 = v_outputs.clone(); if !py::contains(&t1381, &t1380)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1427 = v_inputs.clone(); f_dowell(py::getitem(&t1427, &Py::str("h"))?, py::getitem(&t1427, &Py::str("f"))?, py::getitem(&t1427, &Py::str("rho"))?, py::getitem(&t1427, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1429 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1451 = (t1429).items()?;
        if t1451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1451[0].clone();
        v_v = t1451[1].clone();
        if ({ let t1430 = v_k.clone(); Py::Bool({ let t1431 = v_outputs.clone(); if !py::contains(&t1431, &t1430)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1474 = v_inputs.clone(); f_dab_power(py::getitem(&t1474, &Py::str("V1"))?, py::getitem(&t1474, &Py::str("V2"))?, py::getitem(&t1474, &Py::str("n"))?, py::getitem(&t1474, &Py::str("L"))?, py::getitem(&t1474, &Py::str("fs"))?, py::getitem(&t1474, &Py::str("phi"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1498 = (t1476).items()?;
        if t1498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1498[0].clone();
        v_v = t1498[1].clone();
        if ({ let t1477 = v_k.clone(); Py::Bool({ let t1478 = v_outputs.clone(); if !py::contains(&t1478, &t1477)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1519(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1521: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1520 = v_inputs.clone(); f_flyback_ccm(py::getitem(&t1520, &Py::str("Vin"))?, py::getitem(&t1520, &Py::str("Vout"))?, py::getitem(&t1520, &Py::str("n"))?, py::getitem(&t1520, &Py::str("Lm"))?, py::getitem(&t1520, &Py::str("fs"))?, py::getitem(&t1520, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1521 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1522 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1544 = (t1522).items()?;
        if t1544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1544[0].clone();
        v_v = t1544[1].clone();
        if ({ let t1523 = v_k.clone(); Py::Bool({ let t1524 = v_outputs.clone(); if !py::contains(&t1524, &t1523)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1569(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1571: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1570 = v_inputs.clone(); f_flyback_dcm_design(py::getitem(&t1570, &Py::str("Vin"))?, py::getitem(&t1570, &Py::str("Po"))?, py::getitem(&t1570, &Py::str("eff"))?, py::getitem(&t1570, &Py::str("fs"))?, py::getitem(&t1570, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1571 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1572 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1594 = (t1572).items()?;
        if t1594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1594[0].clone();
        v_v = t1594[1].clone();
        if ({ let t1573 = v_k.clone(); Py::Bool({ let t1574 = v_outputs.clone(); if !py::contains(&t1574, &t1573)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1617(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1619: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1618 = v_inputs.clone(); f_forward_ccm(py::getitem(&t1618, &Py::str("Vin"))?, py::getitem(&t1618, &Py::str("Vout"))?, py::getitem(&t1618, &Py::str("n"))?, py::getitem(&t1618, &Py::str("Nr"))?, py::getitem(&t1618, &Py::str("L"))?, py::getitem(&t1618, &Py::str("fs"))?, py::getitem(&t1618, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1619 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1620 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1642 = (t1620).items()?;
        if t1642.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1642[0].clone();
        v_v = t1642[1].clone();
        if ({ let t1621 = v_k.clone(); Py::Bool({ let t1622 = v_outputs.clone(); if !py::contains(&t1622, &t1621)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1674(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1676: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1675 = v_inputs.clone(); f_foster_periodic(py::getitem(&t1675, &Py::str("R"))?, py::getitem(&t1675, &Py::str("tau"))?, py::getitem(&t1675, &Py::str("P"))?, py::getitem(&t1675, &Py::str("tp"))?, py::getitem(&t1675, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1676 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1677 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1699 = (t1677).items()?;
        if t1699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1699[0].clone();
        v_v = t1699[1].clone();
        if ({ let t1678 = v_k.clone(); Py::Bool({ let t1679 = v_outputs.clone(); if !py::contains(&t1679, &t1678)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1729(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1731: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1730 = v_inputs.clone(); f_foster_pulse(py::getitem(&t1730, &Py::str("R"))?, py::getitem(&t1730, &Py::str("tau"))?, py::getitem(&t1730, &Py::str("P"))?, py::getitem(&t1730, &Py::str("tp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1731 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1732 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1754 = (t1732).items()?;
        if t1754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1754[0].clone();
        v_v = t1754[1].clone();
        if ({ let t1733 = v_k.clone(); Py::Bool({ let t1734 = v_outputs.clone(); if !py::contains(&t1734, &t1733)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1773(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1775: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1774 = v_inputs.clone(); f_gapped_inductor(py::getitem(&t1774, &Py::str("L"))?, py::getitem(&t1774, &Py::str("Ipk"))?, py::getitem(&t1774, &Py::str("Bmax"))?, py::getitem(&t1774, &Py::str("Ac"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1775 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1776 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1798 = (t1776).items()?;
        if t1798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1798[0].clone();
        v_v = t1798[1].clone();
        if ({ let t1777 = v_k.clone(); Py::Bool({ let t1778 = v_outputs.clone(); if !py::contains(&t1778, &t1777)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_178(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t180: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t179 = v_inputs.clone(); f_boost_dcm(py::getitem(&t179, &Py::str("Vin"))?, py::getitem(&t179, &Py::str("L"))?, py::getitem(&t179, &Py::str("fs"))?, py::getitem(&t179, &Py::str("R"))?, py::getitem(&t179, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t180 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t181 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t203 = (t181).items()?;
        if t203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t203[0].clone();
        v_v = t203[1].clone();
        if ({ let t182 = v_k.clone(); Py::Bool({ let t183 = v_outputs.clone(); if !py::contains(&t183, &t182)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1819: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1818 = v_inputs.clone(); f_gate_drive(py::getitem(&t1818, &Py::str("Qg"))?, py::getitem(&t1818, &Py::str("Vdrv"))?, py::getitem(&t1818, &Py::str("fs"))?, py::getitem(&t1818, &Py::str("Rg"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1819 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1820 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1842 = (t1820).items()?;
        if t1842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1842[0].clone();
        v_v = t1842[1].clone();
        if ({ let t1821 = v_k.clone(); Py::Bool({ let t1822 = v_outputs.clone(); if !py::contains(&t1822, &t1821)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1872: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1871 = v_inputs.clone(); f_half_wave_rl(py::getitem(&t1871, &Py::str("Vs"))?, py::getitem(&t1871, &Py::str("f"))?, py::getitem(&t1871, &Py::str("R"))?, py::getitem(&t1871, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1872 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1873 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1895 = (t1873).items()?;
        if t1895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1895[0].clone();
        v_v = t1895[1].clone();
        if ({ let t1874 = v_k.clone(); Py::Bool({ let t1875 = v_outputs.clone(); if !py::contains(&t1875, &t1874)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1916(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1918: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1917 = v_inputs.clone(); f_hold_up(py::getitem(&t1917, &Py::str("Po"))?, py::getitem(&t1917, &Py::str("t_hold"))?, py::getitem(&t1917, &Py::str("V1"))?, py::getitem(&t1917, &Py::str("V2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1918 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1919 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1941 = (t1919).items()?;
        if t1941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1941[0].clone();
        v_v = t1941[1].clone();
        if ({ let t1920 = v_k.clone(); Py::Bool({ let t1921 = v_outputs.clone(); if !py::contains(&t1921, &t1920)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1972(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1974: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1973 = v_inputs.clone(); f_igbt_losses(py::getitem(&t1973, &Py::str("Ip"))?, py::getitem(&t1973, &Py::str("M"))?, py::getitem(&t1973, &Py::str("phi"))?, py::getitem(&t1973, &Py::str("Vce0"))?, py::getitem(&t1973, &Py::str("rce"))?, py::getitem(&t1973, &Py::str("VF0"))?, py::getitem(&t1973, &Py::str("rF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1974 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1975 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1997 = (t1975).items()?;
        if t1997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1997[0].clone();
        v_v = t1997[1].clone();
        if ({ let t1976 = v_k.clone(); Py::Bool({ let t1977 = v_outputs.clone(); if !py::contains(&t1977, &t1976)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2022(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2024: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2023 = v_inputs.clone(); f_igse_triangular(py::getitem(&t2023, &Py::str("k"))?, py::getitem(&t2023, &Py::str("alpha"))?, py::getitem(&t2023, &Py::str("beta"))?, py::getitem(&t2023, &Py::str("f"))?, py::getitem(&t2023, &Py::str("dB"))?, py::getitem(&t2023, &Py::str("D"))?, py::getitem(&t2023, &Py::str("Vc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2024 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2025 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2047 = (t2025).items()?;
        if t2047.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2047[0].clone();
        v_v = t2047[1].clone();
        if ({ let t2026 = v_k.clone(); Py::Bool({ let t2027 = v_outputs.clone(); if !py::contains(&t2027, &t2026)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2080(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2082: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2081 = v_inputs.clone(); f_induction_vf(py::getitem(&t2081, &Py::str("V_rated"))?, py::getitem(&t2081, &Py::str("f_rated"))?, py::getitem(&t2081, &Py::str("poles"))?, py::getitem(&t2081, &Py::str("n_rated"))?, py::getitem(&t2081, &Py::str("f"))?, py::getitem(&t2081, &Py::str("T_ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2082 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2083 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2105 = (t2083).items()?;
        if t2105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2105[0].clone();
        v_v = t2105[1].clone();
        if ({ let t2084 = v_k.clone(); Py::Bool({ let t2085 = v_outputs.clone(); if !py::contains(&t2085, &t2084)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2132: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2131 = v_inputs.clone(); f_inductor_area_product(py::getitem(&t2131, &Py::str("L"))?, py::getitem(&t2131, &Py::str("Ipk"))?, py::getitem(&t2131, &Py::str("Irms"))?, py::getitem(&t2131, &Py::str("Bmax"))?, py::getitem(&t2131, &Py::str("J"))?, py::getitem(&t2131, &Py::str("Ku"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2132 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2133 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2155 = (t2133).items()?;
        if t2155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2155[0].clone();
        v_v = t2155[1].clone();
        if ({ let t2134 = v_k.clone(); Py::Bool({ let t2135 = v_outputs.clone(); if !py::contains(&t2135, &t2134)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2182(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2183 = v_inputs.clone(); f_interleaved_buck(py::getitem(&t2183, &Py::str("N"))?, py::getitem(&t2183, &Py::str("Vin"))?, py::getitem(&t2183, &Py::str("Vout"))?, py::getitem(&t2183, &Py::str("L"))?, py::getitem(&t2183, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2184 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2185 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2207 = (t2185).items()?;
        if t2207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2207[0].clone();
        v_v = t2207[1].clone();
        if ({ let t2186 = v_k.clone(); Py::Bool({ let t2187 = v_outputs.clone(); if !py::contains(&t2187, &t2186)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_22(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t24: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t23 = v_inputs.clone(); f_ac_voltage_controller(py::getitem(&t23, &Py::str("Vs"))?, py::getitem(&t23, &Py::str("alpha"))?, py::getitem(&t23, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t24 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t25 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t47 = (t25).items()?;
        if t47.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t47[0].clone();
        v_v = t47[1].clone();
        if ({ let t26 = v_k.clone(); Py::Bool({ let t27 = v_outputs.clone(); if !py::contains(&t27, &t26)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2226(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2228: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2227 = v_inputs.clone(); f_inverter_rl(py::getitem(&t2227, &Py::str("Vdc"))?, py::getitem(&t2227, &Py::str("f"))?, py::getitem(&t2227, &Py::str("R"))?, py::getitem(&t2227, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2228 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2229 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2251 = (t2229).items()?;
        if t2251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2251[0].clone();
        v_v = t2251[1].clone();
        if ({ let t2230 = v_k.clone(); Py::Bool({ let t2231 = v_outputs.clone(); if !py::contains(&t2231, &t2230)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2270(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2272: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2271 = v_inputs.clone(); f_llc_gain(py::getitem(&t2271, &Py::str("Lr"))?, py::getitem(&t2271, &Py::str("Cr"))?, py::getitem(&t2271, &Py::str("Lm"))?, py::getitem(&t2271, &Py::str("n"))?, py::getitem(&t2271, &Py::str("R"))?, py::getitem(&t2271, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2272 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2273 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2295 = (t2273).items()?;
        if t2295.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2295[0].clone();
        v_v = t2295[1].clone();
        if ({ let t2274 = v_k.clone(); Py::Bool({ let t2275 = v_outputs.clone(); if !py::contains(&t2275, &t2274)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_230(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t232: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t231 = v_inputs.clone(); f_boost_nonideal(py::getitem(&t231, &Py::str("Vin"))?, py::getitem(&t231, &Py::str("D"))?, py::getitem(&t231, &Py::str("R"))?, py::getitem(&t231, &Py::str("RL"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t232 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t233 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t255 = (t233).items()?;
        if t255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t255[0].clone();
        v_v = t255[1].clone();
        if ({ let t234 = v_k.clone(); Py::Bool({ let t235 = v_outputs.clone(); if !py::contains(&t235, &t234)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2322(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2324: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2323 = v_inputs.clone(); f_mosfet_conduction(py::getitem(&t2323, &Py::str("Rds25"))?, py::getitem(&t2323, &Py::str("alpha"))?, py::getitem(&t2323, &Py::str("Tj"))?, py::getitem(&t2323, &Py::str("Irms"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2324 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2325 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2347 = (t2325).items()?;
        if t2347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2347[0].clone();
        v_v = t2347[1].clone();
        if ({ let t2326 = v_k.clone(); Py::Bool({ let t2327 = v_outputs.clone(); if !py::contains(&t2327, &t2326)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2377: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2376 = v_inputs.clone(); f_parallel_sharing(py::getitem(&t2376, &Py::str("R"))?, py::getitem(&t2376, &Py::str("I"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2377 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2378 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2400 = (t2378).items()?;
        if t2400.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2400[0].clone();
        v_v = t2400[1].clone();
        if ({ let t2379 = v_k.clone(); Py::Bool({ let t2380 = v_outputs.clone(); if !py::contains(&t2380, &t2379)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2433: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2432 = v_inputs.clone(); f_peak_current_mode(py::getitem(&t2432, &Py::str("Vin"))?, py::getitem(&t2432, &Py::str("Vout"))?, py::getitem(&t2432, &Py::str("L"))?, py::getitem(&t2432, &Py::str("fs"))?, py::getitem(&t2432, &Py::str("ma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2433 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2434 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2456 = (t2434).items()?;
        if t2456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2456[0].clone();
        v_v = t2456[1].clone();
        if ({ let t2435 = v_k.clone(); Py::Bool({ let t2436 = v_outputs.clone(); if !py::contains(&t2436, &t2435)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2493(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2494 = v_inputs.clone(); f_pmsm_dq(py::getitem(&t2494, &Py::str("Rs"))?, py::getitem(&t2494, &Py::str("Ld"))?, py::getitem(&t2494, &Py::str("Lq"))?, py::getitem(&t2494, &Py::str("psi_f"))?, py::getitem(&t2494, &Py::str("p"))?, py::getitem(&t2494, &Py::str("id"))?, py::getitem(&t2494, &Py::str("iq"))?, py::getitem(&t2494, &Py::str("omega_m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2495 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2518 = (t2496).items()?;
        if t2518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2518[0].clone();
        v_v = t2518[1].clone();
        if ({ let t2497 = v_k.clone(); Py::Bool({ let t2498 = v_outputs.clone(); if !py::contains(&t2498, &t2497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2547(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2549: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2548 = v_inputs.clone(); f_field_weakening(py::getitem(&t2548, &Py::str("Ls"))?, py::getitem(&t2548, &Py::str("psi_f"))?, py::getitem(&t2548, &Py::str("p"))?, py::getitem(&t2548, &Py::str("Imax"))?, py::getitem(&t2548, &Py::str("Vmax"))?, py::getitem(&t2548, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2549 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2550 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2572 = (t2550).items()?;
        if t2572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2572[0].clone();
        v_v = t2572[1].clone();
        if ({ let t2551 = v_k.clone(); Py::Bool({ let t2552 = v_outputs.clone(); if !py::contains(&t2552, &t2551)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2598 = v_inputs.clone(); f_mtpa(py::getitem(&t2598, &Py::str("Ld"))?, py::getitem(&t2598, &Py::str("Lq"))?, py::getitem(&t2598, &Py::str("psi_f"))?, py::getitem(&t2598, &Py::str("p"))?, py::getitem(&t2598, &Py::str("Is"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2599 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2600 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2622 = (t2600).items()?;
        if t2622.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2622[0].clone();
        v_v = t2622[1].clone();
        if ({ let t2601 = v_k.clone(); Py::Bool({ let t2602 = v_outputs.clone(); if !py::contains(&t2602, &t2601)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2646(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2648: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2647 = v_inputs.clone(); f_pf_distortion(py::getitem(&t2647, &Py::str("THD"))?, py::getitem(&t2647, &Py::str("phi"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2648 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2649 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2671 = (t2649).items()?;
        if t2671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2671[0].clone();
        v_v = t2671[1].clone();
        if ({ let t2650 = v_k.clone(); Py::Bool({ let t2651 = v_outputs.clone(); if !py::contains(&t2651, &t2650)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2696(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2697 = v_inputs.clone(); f_pulse_rms(py::getitem(&t2697, &Py::str("A"))?, py::getitem(&t2697, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2698 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2699 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2721 = (t2699).items()?;
        if t2721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2721[0].clone();
        v_v = t2721[1].clone();
        if ({ let t2700 = v_k.clone(); Py::Bool({ let t2701 = v_outputs.clone(); if !py::contains(&t2701, &t2700)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2752(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2754: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2753 = v_inputs.clone(); f_quasi_square(py::getitem(&t2753, &Py::str("Vdc"))?, py::getitem(&t2753, &Py::str("alpha"))?, py::getitem(&t2753, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2754 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2755 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2777 = (t2755).items()?;
        if t2777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2777[0].clone();
        v_v = t2777[1].clone();
        if ({ let t2756 = v_k.clone(); Py::Bool({ let t2757 = v_outputs.clone(); if !py::contains(&t2757, &t2756)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_276(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t278: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t277 = v_inputs.clone(); f_boost_output_ripple(py::getitem(&t277, &Py::str("Vin"))?, py::getitem(&t277, &Py::str("Vout"))?, py::getitem(&t277, &Py::str("Io"))?, py::getitem(&t277, &Py::str("C"))?, py::getitem(&t277, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t278 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t279 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t301 = (t279).items()?;
        if t301.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t301[0].clone();
        v_v = t301[1].clone();
        if ({ let t280 = v_k.clone(); Py::Bool({ let t281 = v_outputs.clone(); if !py::contains(&t281, &t280)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2798(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2800: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2799 = v_inputs.clone(); f_rc_snubber_ringing(py::getitem(&t2799, &Py::str("f0"))?, py::getitem(&t2799, &Py::str("f1"))?, py::getitem(&t2799, &Py::str("Cadd"))?, py::getitem(&t2799, &Py::str("k"))?, py::getitem(&t2799, &Py::str("V"))?, py::getitem(&t2799, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2800 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2801 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2823 = (t2801).items()?;
        if t2823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2823[0].clone();
        v_v = t2823[1].clone();
        if ({ let t2802 = v_k.clone(); Py::Bool({ let t2803 = v_outputs.clone(); if !py::contains(&t2803, &t2802)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2855(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2857: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2856 = v_inputs.clone(); f_line_current_pf(py::getitem(&t2856, &Py::str("circuit"))?, py::getitem(&t2856, &Py::str("Id"))?, py::getitem(&t2856, &Py::str("Vs"))?, py::getitem(&t2856, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2857 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2858 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2880 = (t2858).items()?;
        if t2880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2880[0].clone();
        v_v = t2880[1].clone();
        if ({ let t2859 = v_k.clone(); Py::Bool({ let t2860 = v_outputs.clone(); if !py::contains(&t2860, &t2859)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2915(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2917: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2916 = v_inputs.clone(); f_line_current_harmonics(py::getitem(&t2916, &Py::str("circuit"))?, py::getitem(&t2916, &Py::str("Id"))?, py::getitem(&t2916, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2917 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2918 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2940 = (t2918).items()?;
        if t2940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2940[0].clone();
        v_v = t2940[1].clone();
        if ({ let t2919 = v_k.clone(); Py::Bool({ let t2920 = v_outputs.clone(); if !py::contains(&t2920, &t2919)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2967(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2969: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2968 = v_inputs.clone(); f_regen_chopper(py::getitem(&t2968, &Py::str("Vs"))?, py::getitem(&t2968, &Py::str("D"))?, py::getitem(&t2968, &Py::str("Ra"))?, py::getitem(&t2968, &Py::str("K"))?, py::getitem(&t2968, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2969 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2970 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2992 = (t2970).items()?;
        if t2992.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2992[0].clone();
        v_v = t2992[1].clone();
        if ({ let t2971 = v_k.clone(); Py::Bool({ let t2972 = v_outputs.clone(); if !py::contains(&t2972, &t2971)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3014(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3016: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3015 = v_inputs.clone(); f_sepic_ccm(py::getitem(&t3015, &Py::str("Vin"))?, py::getitem(&t3015, &Py::str("Vout"))?, py::getitem(&t3015, &Py::str("L1"))?, py::getitem(&t3015, &Py::str("L2"))?, py::getitem(&t3015, &Py::str("C1"))?, py::getitem(&t3015, &Py::str("fs"))?, py::getitem(&t3015, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3016 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3017 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3039 = (t3017).items()?;
        if t3039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3039[0].clone();
        v_v = t3039[1].clone();
        if ({ let t3018 = v_k.clone(); Py::Bool({ let t3019 = v_outputs.clone(); if !py::contains(&t3019, &t3018)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3058(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3060: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3059 = v_inputs.clone(); f_series_resonant(py::getitem(&t3059, &Py::str("Lr"))?, py::getitem(&t3059, &Py::str("Cr"))?, py::getitem(&t3059, &Py::str("R"))?, py::getitem(&t3059, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3060 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3061 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3083 = (t3061).items()?;
        if t3083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3083[0].clone();
        v_v = t3083[1].clone();
        if ({ let t3062 = v_k.clone(); Py::Bool({ let t3063 = v_outputs.clone(); if !py::contains(&t3063, &t3062)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3117: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3116 = v_inputs.clone(); f_single_phase_controlled(py::getitem(&t3116, &Py::str("circuit"))?, py::getitem(&t3116, &Py::str("Vs"))?, py::getitem(&t3116, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3117 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3118 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3140 = (t3118).items()?;
        if t3140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3140[0].clone();
        v_v = t3140[1].clone();
        if ({ let t3119 = v_k.clone(); Py::Bool({ let t3120 = v_outputs.clone(); if !py::contains(&t3120, &t3119)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3164(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3166: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3165 = v_inputs.clone(); f_single_overlap(py::getitem(&t3165, &Py::str("Vs"))?, py::getitem(&t3165, &Py::str("f"))?, py::getitem(&t3165, &Py::str("Ls"))?, py::getitem(&t3165, &Py::str("Id"))?, py::getitem(&t3165, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3166 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3167 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3189 = (t3167).items()?;
        if t3189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3189[0].clone();
        v_v = t3189[1].clone();
        if ({ let t3168 = v_k.clone(); Py::Bool({ let t3169 = v_outputs.clone(); if !py::contains(&t3169, &t3168)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3208(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3210: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3209 = v_inputs.clone(); f_six_step(py::getitem(&t3209, &Py::str("Vdc"))?, py::getitem(&t3209, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3210 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3211 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3233 = (t3211).items()?;
        if t3233.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3233[0].clone();
        v_v = t3233[1].clone();
        if ({ let t3212 = v_k.clone(); Py::Bool({ let t3213 = v_outputs.clone(); if !py::contains(&t3213, &t3212)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3260(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3262: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3261 = v_inputs.clone(); f_skin_effect(py::getitem(&t3261, &Py::str("d"))?, py::getitem(&t3261, &Py::str("f"))?, py::getitem(&t3261, &Py::str("rho"))?, py::getitem(&t3261, &Py::str("length"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3262 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3263 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3285 = (t3263).items()?;
        if t3285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3285[0].clone();
        v_v = t3285[1].clone();
        if ({ let t3264 = v_k.clone(); Py::Bool({ let t3265 = v_outputs.clone(); if !py::contains(&t3265, &t3264)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_330(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t332: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t331 = v_inputs.clone(); f_pfc_boost(py::getitem(&t331, &Py::str("Vac"))?, py::getitem(&t331, &Py::str("f_line"))?, py::getitem(&t331, &Py::str("Vo"))?, py::getitem(&t331, &Py::str("Po"))?, py::getitem(&t331, &Py::str("eff"))?, py::getitem(&t331, &Py::str("L"))?, py::getitem(&t331, &Py::str("fs"))?, py::getitem(&t331, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t332 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t333 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t355 = (t333).items()?;
        if t355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t355[0].clone();
        v_v = t355[1].clone();
        if ({ let t334 = v_k.clone(); Py::Bool({ let t335 = v_outputs.clone(); if !py::contains(&t335, &t334)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3311(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3313: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3312 = v_inputs.clone(); f_space_vector(py::getitem(&t3312, &Py::str("Vref"))?, py::getitem(&t3312, &Py::str("theta"))?, py::getitem(&t3312, &Py::str("Vdc"))?, py::getitem(&t3312, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3313 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3314 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3336 = (t3314).items()?;
        if t3336.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3336[0].clone();
        v_v = t3336[1].clone();
        if ({ let t3315 = v_k.clone(); Py::Bool({ let t3316 = v_outputs.clone(); if !py::contains(&t3316, &t3315)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3365(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3366 = v_inputs.clone(); f_spwm(py::getitem(&t3366, &Py::str("Vdc"))?, py::getitem(&t3366, &Py::str("ma"))?, py::getitem(&t3366, &Py::str("mf"))?, py::getitem(&t3366, &Py::str("f1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3368 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3390 = (t3368).items()?;
        if t3390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3390[0].clone();
        v_v = t3390[1].clone();
        if ({ let t3369 = v_k.clone(); Py::Bool({ let t3370 = v_outputs.clone(); if !py::contains(&t3370, &t3369)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3420(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3421 = v_inputs.clone(); f_square_wave_inverter(py::getitem(&t3421, &Py::str("Vdc"))?, py::getitem(&t3421, &Py::str("bridge"))?, py::getitem(&t3421, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3422 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3423 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3445 = (t3423).items()?;
        if t3445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3445[0].clone();
        v_v = t3445[1].clone();
        if ({ let t3424 = v_k.clone(); Py::Bool({ let t3425 = v_outputs.clone(); if !py::contains(&t3425, &t3424)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3464(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3465 = v_inputs.clone(); f_steinmetz(py::getitem(&t3465, &Py::str("k"))?, py::getitem(&t3465, &Py::str("alpha"))?, py::getitem(&t3465, &Py::str("beta"))?, py::getitem(&t3465, &Py::str("f"))?, py::getitem(&t3465, &Py::str("Bpk"))?, py::getitem(&t3465, &Py::str("Vc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3466 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3467 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3489 = (t3467).items()?;
        if t3489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3489[0].clone();
        v_v = t3489[1].clone();
        if ({ let t3468 = v_k.clone(); Py::Bool({ let t3469 = v_outputs.clone(); if !py::contains(&t3469, &t3468)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3514(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3515 = v_inputs.clone(); f_switching_loss(py::getitem(&t3515, &Py::str("V"))?, py::getitem(&t3515, &Py::str("I"))?, py::getitem(&t3515, &Py::str("tr"))?, py::getitem(&t3515, &Py::str("tf"))?, py::getitem(&t3515, &Py::str("fs"))?, py::getitem(&t3515, &Py::str("Coss"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3517 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3539 = (t3517).items()?;
        if t3539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3539[0].clone();
        v_v = t3539[1].clone();
        if ({ let t3518 = v_k.clone(); Py::Bool({ let t3519 = v_outputs.clone(); if !py::contains(&t3519, &t3518)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3570(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3572: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3571 = v_inputs.clone(); f_thd_harmonics(py::getitem(&t3571, &Py::str("V1"))?, py::getitem(&t3571, &Py::str("Vh"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3572 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3573 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3595 = (t3573).items()?;
        if t3595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3595[0].clone();
        v_v = t3595[1].clone();
        if ({ let t3574 = v_k.clone(); Py::Bool({ let t3575 = v_outputs.clone(); if !py::contains(&t3575, &t3574)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3627(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3629: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3628 = v_inputs.clone(); f_thermal_multi(py::getitem(&t3628, &Py::str("P"))?, py::getitem(&t3628, &Py::str("Rjc"))?, py::getitem(&t3628, &Py::str("Rcs"))?, py::getitem(&t3628, &Py::str("Rsa"))?, py::getitem(&t3628, &Py::str("Ta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3629 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3630 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3652 = (t3630).items()?;
        if t3652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3652[0].clone();
        v_v = t3652[1].clone();
        if ({ let t3631 = v_k.clone(); Py::Bool({ let t3632 = v_outputs.clone(); if !py::contains(&t3632, &t3631)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3678(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3680: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3679 = v_inputs.clone(); f_three_phase_controlled(py::getitem(&t3679, &Py::str("VLL"))?, py::getitem(&t3679, &Py::str("alpha"))?, py::getitem(&t3679, &Py::str("Id"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3680 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3681 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3703 = (t3681).items()?;
        if t3703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3703[0].clone();
        v_v = t3703[1].clone();
        if ({ let t3682 = v_k.clone(); Py::Bool({ let t3683 = v_outputs.clone(); if !py::contains(&t3683, &t3682)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3722(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3724: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3723 = v_inputs.clone(); f_three_phase_bridge(py::getitem(&t3723, &Py::str("VLL"))?, py::getitem(&t3723, &Py::str("f"))?, py::getitem(&t3723, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3724 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3725 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3747 = (t3725).items()?;
        if t3747.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3747[0].clone();
        v_v = t3747[1].clone();
        if ({ let t3726 = v_k.clone(); Py::Bool({ let t3727 = v_outputs.clone(); if !py::contains(&t3727, &t3726)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3766(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3768: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3767 = v_inputs.clone(); f_three_phase_half_wave(py::getitem(&t3767, &Py::str("Vph"))?, py::getitem(&t3767, &Py::str("f"))?, py::getitem(&t3767, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3768 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3769 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3791 = (t3769).items()?;
        if t3791.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3791[0].clone();
        v_v = t3791[1].clone();
        if ({ let t3770 = v_k.clone(); Py::Bool({ let t3771 = v_outputs.clone(); if !py::contains(&t3771, &t3770)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3815(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3817: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3816 = v_inputs.clone(); f_three_overlap(py::getitem(&t3816, &Py::str("VLL"))?, py::getitem(&t3816, &Py::str("f"))?, py::getitem(&t3816, &Py::str("Ls"))?, py::getitem(&t3816, &Py::str("Id"))?, py::getitem(&t3816, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3817 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3818 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3840 = (t3818).items()?;
        if t3840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3840[0].clone();
        v_v = t3840[1].clone();
        if ({ let t3819 = v_k.clone(); Py::Bool({ let t3820 = v_outputs.clone(); if !py::contains(&t3820, &t3819)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t387 = v_inputs.clone(); f_bootstrap(py::getitem(&t387, &Py::str("Qg"))?, py::getitem(&t387, &Py::str("Ileak"))?, py::getitem(&t387, &Py::str("fs"))?, py::getitem(&t387, &Py::str("Dmax"))?, py::getitem(&t387, &Py::str("dV"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t389 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t411 = (t389).items()?;
        if t411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t411[0].clone();
        v_v = t411[1].clone();
        if ({ let t390 = v_k.clone(); Py::Bool({ let t391 = v_outputs.clone(); if !py::contains(&t391, &t390)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3869(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3871: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3870 = v_inputs.clone(); f_transformer_area_product(py::getitem(&t3870, &Py::str("Po"))?, py::getitem(&t3870, &Py::str("eff"))?, py::getitem(&t3870, &Py::str("waveform"))?, py::getitem(&t3870, &Py::str("Bm"))?, py::getitem(&t3870, &Py::str("f"))?, py::getitem(&t3870, &Py::str("J"))?, py::getitem(&t3870, &Py::str("Ku"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3871 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3872 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3894 = (t3872).items()?;
        if t3894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3894[0].clone();
        v_v = t3894[1].clone();
        if ({ let t3873 = v_k.clone(); Py::Bool({ let t3874 = v_outputs.clone(); if !py::contains(&t3874, &t3873)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3917(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3919: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3918 = v_inputs.clone(); f_transformer_turns(py::getitem(&t3918, &Py::str("waveform"))?, py::getitem(&t3918, &Py::str("V"))?, py::getitem(&t3918, &Py::str("f"))?, py::getitem(&t3918, &Py::str("Bmax"))?, py::getitem(&t3918, &Py::str("Ac"))?, py::getitem(&t3918, &Py::str("Vs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3919 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3920 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3942 = (t3920).items()?;
        if t3942.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3942[0].clone();
        v_v = t3942[1].clone();
        if ({ let t3921 = v_k.clone(); Py::Bool({ let t3922 = v_outputs.clone(); if !py::contains(&t3922, &t3921)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3961(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3963: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3962 = v_inputs.clone(); f_turnoff_snubber(py::getitem(&t3962, &Py::str("I"))?, py::getitem(&t3962, &Py::str("V"))?, py::getitem(&t3962, &Py::str("tf"))?, py::getitem(&t3962, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3963 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3964 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3986 = (t3964).items()?;
        if t3986.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3986[0].clone();
        v_v = t3986[1].clone();
        if ({ let t3965 = v_k.clone(); Py::Bool({ let t3966 = v_outputs.clone(); if !py::contains(&t3966, &t3965)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4008 = v_inputs.clone(); f_zvs_transition(py::getitem(&t4008, &Py::str("Vin"))?, py::getitem(&t4008, &Py::str("Ceq"))?, py::getitem(&t4008, &Py::str("L"))?, py::getitem(&t4008, &Py::str("I0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4032 = (t4010).items()?;
        if t4032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4032[0].clone();
        v_v = t4032[1].clone();
        if ({ let t4011 = v_k.clone(); Py::Bool({ let t4012 = v_outputs.clone(); if !py::contains(&t4012, &t4011)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_440(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t442: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t441 = v_inputs.clone(); f_bridge_converter(py::getitem(&t441, &Py::str("topology"))?, py::getitem(&t441, &Py::str("Vin"))?, py::getitem(&t441, &Py::str("Vout"))?, py::getitem(&t441, &Py::str("n"))?, py::getitem(&t441, &Py::str("L"))?, py::getitem(&t441, &Py::str("fs"))?, py::getitem(&t441, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t442 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t443 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t465 = (t443).items()?;
        if t465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t465[0].clone();
        v_v = t465[1].clone();
        if ({ let t444 = v_k.clone(); Py::Bool({ let t445 = v_outputs.clone(); if !py::contains(&t445, &t444)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_486(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t488: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t487 = v_inputs.clone(); f_buck_boost_ccm(py::getitem(&t487, &Py::str("Vin"))?, py::getitem(&t487, &Py::str("Vout"))?, py::getitem(&t487, &Py::str("L"))?, py::getitem(&t487, &Py::str("fs"))?, py::getitem(&t487, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t488 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t489 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t511 = (t489).items()?;
        if t511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t511[0].clone();
        v_v = t511[1].clone();
        if ({ let t490 = v_k.clone(); Py::Bool({ let t491 = v_outputs.clone(); if !py::contains(&t491, &t490)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_538(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t540: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t539 = v_inputs.clone(); f_buck_boost_dcm(py::getitem(&t539, &Py::str("Vin"))?, py::getitem(&t539, &Py::str("L"))?, py::getitem(&t539, &Py::str("fs"))?, py::getitem(&t539, &Py::str("R"))?, py::getitem(&t539, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t540 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t541 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t563 = (t541).items()?;
        if t563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t563[0].clone();
        v_v = t563[1].clone();
        if ({ let t542 = v_k.clone(); Py::Bool({ let t543 = v_outputs.clone(); if !py::contains(&t543, &t542)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_586(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t588: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t587 = v_inputs.clone(); f_buck_ccm(py::getitem(&t587, &Py::str("Vin"))?, py::getitem(&t587, &Py::str("Vout"))?, py::getitem(&t587, &Py::str("L"))?, py::getitem(&t587, &Py::str("fs"))?, py::getitem(&t587, &Py::str("Io"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t588 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t589 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t611 = (t589).items()?;
        if t611.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t611[0].clone();
        v_v = t611[1].clone();
        if ({ let t590 = v_k.clone(); Py::Bool({ let t591 = v_outputs.clone(); if !py::contains(&t591, &t590)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_634(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t636: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t635 = v_inputs.clone(); f_buck_currents(py::getitem(&t635, &Py::str("Vin"))?, py::getitem(&t635, &Py::str("Vout"))?, py::getitem(&t635, &Py::str("Io"))?, py::getitem(&t635, &Py::str("L"))?, py::getitem(&t635, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t636 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t637 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t659 = (t637).items()?;
        if t659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t659[0].clone();
        v_v = t659[1].clone();
        if ({ let t638 = v_k.clone(); Py::Bool({ let t639 = v_outputs.clone(); if !py::contains(&t639, &t638)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_686(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t688: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t687 = v_inputs.clone(); f_buck_dcm(py::getitem(&t687, &Py::str("Vin"))?, py::getitem(&t687, &Py::str("L"))?, py::getitem(&t687, &Py::str("fs"))?, py::getitem(&t687, &Py::str("R"))?, py::getitem(&t687, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t688 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t689 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t711 = (t689).items()?;
        if t711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t711[0].clone();
        v_v = t711[1].clone();
        if ({ let t690 = v_k.clone(); Py::Bool({ let t691 = v_outputs.clone(); if !py::contains(&t691, &t690)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_735(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t737: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t736 = v_inputs.clone(); f_buck_lc_design(py::getitem(&t736, &Py::str("Vin"))?, py::getitem(&t736, &Py::str("Vout"))?, py::getitem(&t736, &Py::str("Io"))?, py::getitem(&t736, &Py::str("fs"))?, py::getitem(&t736, &Py::str("r"))?, py::getitem(&t736, &Py::str("dV"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t737 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t738 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t760 = (t738).items()?;
        if t760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t760[0].clone();
        v_v = t760[1].clone();
        if ({ let t739 = v_k.clone(); Py::Bool({ let t740 = v_outputs.clone(); if !py::contains(&t740, &t739)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_78(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t80: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t79 = v_inputs.clone(); f_bldc_six_step(py::getitem(&t79, &Py::str("Vdc"))?, py::getitem(&t79, &Py::str("D"))?, py::getitem(&t79, &Py::str("Rph"))?, py::getitem(&t79, &Py::str("Keph"))?, py::getitem(&t79, &Py::str("I"))?, py::getitem(&t79, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t80 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t81 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t103 = (t81).items()?;
        if t103.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t103[0].clone();
        v_v = t103[1].clone();
        if ({ let t82 = v_k.clone(); Py::Bool({ let t83 = v_outputs.clone(); if !py::contains(&t83, &t82)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_781(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t783: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t782 = v_inputs.clone(); f_buck_output_ripple(py::getitem(&t782, &Py::str("Vin"))?, py::getitem(&t782, &Py::str("Vout"))?, py::getitem(&t782, &Py::str("L"))?, py::getitem(&t782, &Py::str("C"))?, py::getitem(&t782, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t783 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t784 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t806 = (t784).items()?;
        if t806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t806[0].clone();
        v_v = t806[1].clone();
        if ({ let t785 = v_k.clone(); Py::Bool({ let t786 = v_outputs.clone(); if !py::contains(&t786, &t785)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t838: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t837 = v_inputs.clone(); f_buck_losses(py::getitem(&t837, &Py::str("Vin"))?, py::getitem(&t837, &Py::str("Vout"))?, py::getitem(&t837, &Py::str("Io"))?, py::getitem(&t837, &Py::str("fs"))?, py::getitem(&t837, &Py::str("Rds"))?, py::getitem(&t837, &Py::str("RL"))?, py::getitem(&t837, &Py::str("VF"))?, py::getitem(&t837, &Py::str("tsw"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t838 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t839 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t861 = (t839).items()?;
        if t861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t861[0].clone();
        v_v = t861[1].clone();
        if ({ let t840 = v_k.clone(); Py::Bool({ let t841 = v_outputs.clone(); if !py::contains(&t841, &t840)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t894 = v_inputs.clone(); f_capacitor_filter_exact(py::getitem(&t894, &Py::str("circuit"))?, py::getitem(&t894, &Py::str("Vp"))?, py::getitem(&t894, &Py::str("f"))?, py::getitem(&t894, &Py::str("R"))?, py::getitem(&t894, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t896 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t918 = (t896).items()?;
        if t918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t918[0].clone();
        v_v = t918[1].clone();
        if ({ let t897 = v_k.clone(); Py::Bool({ let t898 = v_outputs.clone(); if !py::contains(&t898, &t897)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t953: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t952 = v_inputs.clone(); f_conversion_ratio(py::getitem(&t952, &Py::str("topology"))?, py::getitem(&t952, &Py::str("Vin"))?, py::getitem(&t952, &Py::str("D"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t953 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t954 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t976 = (t954).items()?;
        if t976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t976[0].clone();
        v_v = t976[1].clone();
        if ({ let t955 = v_k.clone(); Py::Bool({ let t956 = v_outputs.clone(); if !py::contains(&t956, &t955)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
