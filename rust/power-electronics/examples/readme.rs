//! Power Electronics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p power-electronics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&power_electronics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "pe_converter_small_signal", "inputs": {"topology": "boost", "Vin": "12 V", "D": "0.5", "L": "50 uH", "C": "100 uF", "R": "10 ohm"}}));
    println!();
    show(json!({"key": "pe_boost_ccm", "inputs": {"Vin": "12 V", "Vout": "48 V", "L": "100 uH", "fs": "100 kHz", "Io": "1 A"}}));
    println!();
    show(json!({"key": "pe_bootstrap_capacitor", "inputs": {"Qg": "60 nC", "Ileak": "200 uA", "fs": "100 kHz", "Dmax": "0.9", "dV": "0.5 V"}}));
    println!();
    show(json!({"key": "pe_boost_pfc", "inputs": {"Vac": "230 V", "f_line": "50 Hz", "Vo": "400 V", "Po": "500 W", "eff": "0.95", "L": "500 uH", "fs": "65 kHz", "C": "330 uF"}}));
    println!();
    show(json!({"key": "pe_dead_time_error", "inputs": {"Vdc": "600 V", "td": "2000 ns", "fs": "10 kHz", "V1": "250 V"}}));
    println!();
    show(json!({"key": "pe_flyback_ccm", "inputs": {"Vin": "48 V", "Vout": "12 V", "n": "2", "Lm": "200 uH", "fs": "100 kHz", "Io": "2 A"}}));
    println!();
    show(json!({"key": "pe_boost_nonideal_gain", "inputs": {"Vin": "12 V", "D": "0.75", "R": "50 ohm", "RL": "0.5 ohm"}}));
    println!();
    show(json!({"key": "pe_steinmetz_core_loss", "inputs": {"k": "3.2", "alpha": "1.4", "beta": "2.5", "f": "100 kHz", "Bpk": "100 mT", "Vc": "5 cm^3"}}));
    println!();
    show(json!({"key": "pe_dc_motor_armature_ripple", "inputs": {"Vs": "220 V", "D": "0.5", "fs": "1 kHz", "Ra": "1 ohm", "La": "10 mH", "Ea": "100 V"}}));
    println!();
    show(json!({"key": "pe_pulse_waveform_rms", "inputs": {"A": "12", "D": "0.25"}}));
    println!();
    show(json!({"key": "pe_capacitor_filter_exact", "inputs": {"circuit": "half-wave", "Vp": "169.7 V", "f": "0.06 kHz", "R": "500 ohm", "C": "100 uF"}}));
    println!();
    show(json!({"key": "pe_dual_active_bridge_power", "inputs": {"V1": "400 V", "V2": "48 V", "n": "8", "L": "50 uH", "fs": "100 kHz", "phi": "30 deg"}}));
    println!();
    show(json!({"key": "pe_foster_periodic_pulses", "inputs": {"R": [0.05, 0.1, 0.15], "tau": [0.0001, 0.003, 0.05], "P": "500 W", "tp": "2 ms", "T": "10 ms"}}));
    println!();
    show(json!({"key": "pe_peak_current_mode_slope", "inputs": {"Vin": "12 V", "Vout": "8 V", "L": "10 uH", "fs": "300 kHz", "ma": "0.3 A/us"}}));
    println!();
    show(json!({"key": "pe_boost_dcm", "inputs": {"Vin": "12 V", "L": "10 uH", "fs": "100 kHz", "R": "100 ohm", "D": "0.4"}}));
}
