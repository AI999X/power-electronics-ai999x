//! Power Electronics' own live figures (ax_engine::figkit): every published figure kind of a capability, drawn for the
//! visitor's inputs from what the model computed. A curve, a shaded field or a verdict map is a set of further solves
//! of the capability itself; a figure the inputs make impossible is left out. Written by hand for the Rust build.

use ax_engine::figkit::{self as k, eq, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use std::f64::consts::{FRAC_PI_2, PI};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "pe_boost_pfc" => vec![against(s, "di-max-vs-vac", "Vac", "dI_max", R::Near)],
        "pe_buck_ccm" => vec![against(s, "dil-vs-vin", "Vin", "dIL", R::Near)],
        "pe_flyback_ccm" => vec![against(s, "d-vs-vin", "Vin", "D", R::Near)],
        "pe_peak_current_mode_slope" => vec![against(s, "alpha-vs-vin", "Vin", "alpha", R::Near)],
        "pe_power_factor_distortion" => vec![field(s, "phi", R::Abs(0.0, FRAC_PI_2), "THD", R::Abs(0.02, 0.98), "PF")],
        "pe_pulse_waveform_rms" => vec![field(s, "A", R::Near, "D", R::Abs(0.02, 0.98), "rms")],
        "pe_rectifier_line_harmonics" => vec![field(s, "Id", R::Near, "h", R::Abs(1.0, 13.0), "Ih")],
        "pe_single_phase_controlled_rectifier" => vec![field(s, "Vs", R::Near, "alpha", R::Abs(0.0, FRAC_PI_2), "Vdc")],
        "pe_dc_drive_controlled_rectifier" => vec![verdicts(s, "alpha", R::Abs(0.0, PI), "Vs", R::Scale(0.02, 2.0), "mode")],
        "pe_foster_periodic_pulses" => vec![foster(s, &["dT_peak", "dT_min", "dT_avg", "ripple"])],
        "pe_foster_transient_thermal" => vec![foster(s, &["Zth", "dTj", "Rth_total", "dTj_steady"])],
        "pe_parallel_mosfet_sharing" => vec![sharing(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

/// The range an input is swept over.
#[derive(Clone, Copy)]
enum R {
    /// half to one and a half times the given value
    Near,
    /// from lo to hi times the given value
    Scale(f64, f64),
    /// a fixed range (a fraction 0..1, an angle, a harmonic order)
    Abs(f64, f64),
}

fn range(x0: f64, r: R) -> Option<(f64, f64, bool)> {
    match r {
        R::Near if x0 != 0.0 => Some(((x0 * 0.5).min(x0 * 1.5), (x0 * 0.5).max(x0 * 1.5), false)),
        R::Scale(a, b) if x0 != 0.0 => Some(((x0 * a).min(x0 * b), (x0 * a).max(x0 * b), false)),
        R::Abs(a, b) => Some((a, b, false)),
        _ => None,
    }
}

fn is_int(s: &Solved, sym: &str) -> bool {
    s.p.procedure(s.key).and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).is_some_and(|i| i.ty == "integer")
}

/// A number input as read (an integer input too).
fn given(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| s.val(sym).map(|v| v.num()).filter(|x| x.is_finite()))
}

/// The input value to solve with: an integer input as a whole number.
fn input(s: &Solved, sym: &str, x: f64) -> Input {
    if is_int(s, sym) { Input::Int(x.round() as i64) } else { Input::Float(x) }
}

fn unit_of_kind(s: &Solved, kind: &str) -> String {
    match s.p.kinds.get(kind).map(|x| x.unit.clone()) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

/// (meaning, unit) of an input or output.
fn about(s: &Solved, sym: &str) -> (String, String) {
    let Some(p) = s.p.procedure(s.key) else { return (sym.into(), String::new()) };
    if let Some(i) = p.inputs.iter().find(|i| i.symbol == sym) {
        return (i.meaning.clone(), unit_of_kind(s, &i.kind));
    }
    if let Some(o) = p.outputs.iter().find(|o| o.symbol == sym) {
        return (o.meaning.clone(), unit_of_kind(s, &o.kind));
    }
    (sym.into(), String::new())
}

fn label(sym: &str, unit: &str) -> String {
    let u = if unit == "ohm" { "Ω" } else { unit };
    if u.is_empty() { sym.to_string() } else { format!("{sym} ({u})") }
}

fn name(s: &Solved) -> String {
    s.p.procedure(s.key).map(|p| p.name.clone()).unwrap_or_default()
}

fn cap(t: &str) -> String {
    let mut c = t.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// One output against one input over its range (31 solves of the model), the given point marked.
fn against(s: &Solved, kind: &str, x: &str, y: &str, r: R) -> Option<Figure> {
    let (x0, y0) = (given(s, x)?, s.out(y)?);
    let (lo, hi, log) = range(x0, r)?;
    let mut pts: Vec<(f64, f64)> = vec![];
    for xv in k::span(lo, hi, 31, log) {
        let xv = if is_int(s, x) { xv.round() } else { xv };
        if pts.last().is_some_and(|p| p.0 == xv) {
            continue;
        }
        if let Some(yv) = s.with_input(&[(x, input(s, x, xv))]).and_then(|a| k::num_of(&a, y)) {
            pts.push((xv, yv));
        }
    }
    if pts.len() < 3 {
        return None;
    }
    let (mx, ux) = about(s, x);
    let (my, uy) = about(s, y);
    let n = pts.len();
    let (plo, phi) = (pts[0].0, pts[n - 1].0);
    let body = k::lines_y(&[(y.to_string(), pts)], &label(x, &ux), &label(y, &uy), &eq(y, y0, &uy), Some((x0, y0)),
                          &k::q(y0, &uy), log, true);
    Some(s.chart(kind, &format!("{y} against {x}: {}", name(s)),
                 &format!("{} ({y}) against {} ({x}) from {} to {}, the other inputs as given: {n} points, each computed \
                           by the model (inputs it refuses are left out). Marked: the inputs given, {x} = {}, {y} = {}.",
                          cap(&my), mx, k::q(plo, &ux), k::q(phi, &ux), k::q(x0, &ux), k::q(y0, &uy)), &body))
}

/// The grid of two inputs: (xs, ys, x log, y log, ranges).
fn grid(s: &Solved, x: &str, xr: R, y: &str, yr: R, nx: usize, ny: usize) -> Option<(Vec<f64>, Vec<f64>, (f64, f64), (f64, f64))> {
    let (x0, y0) = (given(s, x)?, given(s, y)?);
    let (xl, xh, _) = range(x0, xr)?;
    let (yl, yh, _) = range(y0, yr)?;
    let mut xs = k::span(xl, xh, nx, false);
    let mut ys = k::span(yl, yh, ny, false);
    for (v, sym) in [(&mut xs, x), (&mut ys, y)] {
        if is_int(s, sym) {
            for t in v.iter_mut() {
                *t = t.round();
            }
            v.dedup();
        }
    }
    Some((xs, ys, (xl, xh), (yl, yh)))
}

/// One output over two inputs, each cell one solve of the model shaded by its value, the given point marked.
fn field(s: &Solved, x: &str, xr: R, y: &str, yr: R, out: &str) -> Option<Figure> {
    let (x0, y0, z0) = (given(s, x)?, given(s, y)?, s.out(out)?);
    let (xs, ys, (xl, xh), (yl, yh)) = grid(s, x, xr, y, yr, 17, 13)?;
    let mut vals = vec![];
    let mut n = 0;
    for xv in &xs {
        let mut col = vec![];
        for yv in &ys {
            let v = s.with_input(&[(x, input(s, x, *xv)), (y, input(s, y, *yv))]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let (mx, ux) = about(s, x);
    let (my, uy) = about(s, y);
    let (mz, uz) = about(s, out);
    let body = k::field(&xs, &ys, &vals, (x0, y0), &label(x, &ux), &label(y, &uy), &eq(out, z0, &uz), (false, false));
    Some(s.chart("field", &format!("{}: {mz} over {} and {}", name(s), short(&mx), short(&my)),
                 &format!("{} ({out}) over {x} from {} to {} and {y} from {} to {}: each of the {n} cells is one \
                           computation of the model, shaded by its value (darker is larger; a blank cell: inputs the \
                           model refuses). Marked: the inputs given, {out} = {}.", cap(&mz), k::q(xl, &ux),
                          k::q(xh, &ux), k::q(yl, &uy), k::q(yh, &uy), k::q(z0, &uz)), &body))
}

/// A meaning without its bracketed note ("duty cycle (a fraction or %)" -> "duty cycle").
fn short(t: &str) -> String {
    t.split(" (").next().unwrap_or(t).to_string()
}

/// A text verdict over two inputs, each cell one solve of the model coloured by the verdict it returned.
fn verdicts(s: &Solved, x: &str, xr: R, y: &str, yr: R, out: &str) -> Option<Figure> {
    let (x0, y0) = (given(s, x)?, given(s, y)?);
    let mine = s.out_text(out)?;
    let (xs, ys, (xl, xh), (yl, yh)) = grid(s, x, xr, y, yr, 19, 15)?;
    let mut cells = vec![];
    let mut names: Vec<String> = vec![mine.clone()];
    let mut counts: Vec<usize> = vec![0];
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            let Some(a) = s.with_input(&[(x, input(s, x, *xv)), (y, input(s, y, *yv))]) else { continue };
            let Some(v) = a.values.iter().find(|e| e.0 == out).and_then(|e| e.1.as_str().map(|t| t.to_string()))
            else { continue };
            match names.iter().position(|n| *n == v) {
                Some(p) => counts[p] += 1,
                None => {
                    names.push(v.clone());
                    counts.push(1);
                }
            }
            cells.push((i, j, v));
        }
    }
    if cells.len() < 4 {
        return None;
    }
    // the verdicts in a fixed order (by name), so a verdict keeps its colour whatever the inputs
    let mut order: Vec<usize> = (0..names.len()).collect();
    order.sort_by(|a, b| names[*a].cmp(&names[*b]));
    let names: Vec<String> = order.iter().filter(|i| counts[**i] > 0).map(|i| names[*i].clone()).collect();
    let counts: Vec<usize> = order.iter().filter(|i| counts[**i] > 0).map(|i| counts[*i]).collect();
    let (mx, ux) = about(s, x);
    let (my, uy) = about(s, y);
    let (mo, _) = about(s, out);
    let tally = names.iter().zip(&counts).map(|(n, c)| format!("{n}: {c} cells")).collect::<Vec<_>>().join(", ");
    let shown: Vec<String> = names.iter().map(|n| short(n)).collect();
    let cells: Vec<(usize, usize, String)> = cells.into_iter().map(|(i, j, v)| (i, j, short(&v))).collect();
    let body = k::decision(&xs, &ys, &cells, &shown, (x0, y0), &label(x, &ux), &label(y, &uy),
                           &format!("{out}: {}", short(&mine)));
    Some(s.chart("decision", &format!("{}: where the verdict changes ({out})", name(s)),
                 &format!("{} ({out}) over {} ({x}) from {} to {} and {} ({y}) from {} to {}: each of the {} cells is \
                           one computation of the model, coloured by the verdict it returned ({tally}). Marked: the \
                           inputs given, {out} = {mine}.", cap(&mo), short(&mx), k::q(xl, &ux), k::q(xh, &ux),
                          short(&my), k::q(yl, &uy), k::q(yh, &uy), cells.len()), &body))
}

/// A Foster network's data (R against tau, as given) with what the model returned for it.
fn foster(s: &Solved, outs: &[&str]) -> Option<Figure> {
    let (r, tau) = (s.vec("R")?, s.vec("tau")?);
    if r.len() != tau.len() || r.is_empty() || tau.iter().any(|t| !(*t > 0.0)) {
        return None;
    }
    let mut got = vec![];
    for o in outs {
        let (_, u) = about(s, o);
        got.push(format!("{o} = {} {}", num(s.out(o)?), u).trim_end().to_string());
    }
    let mut pts: Vec<(f64, f64)> = tau.iter().copied().zip(r.iter().copied()).collect();
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let header = got[..2].join(", ");
    let body = k::lines(&[("data (input)".into(), pts)], "tau: Foster time constant (s)", "R: Foster resistance (K/W)",
                        &header, None, "", true);
    Some(s.chart("xy", &format!("{}: the data and what was returned", name(s)),
                 &format!("The Foster network as given: thermal resistances R against time constants tau ({} terms), \
                           logarithmic horizontal axis. Returned by the model for this network: {}.", r.len(),
                          got.join(", ")), &body))
}

/// Paralleled MOSFETs: the on-resistances as given and the currents and losses the model returned, as plane vectors.
fn sharing(s: &Solved) -> Option<Figure> {
    let r = s.vec("R")?;
    let (i, p) = (s.out_list("currents")?, s.out_list("losses")?);
    if r.len() != 2 || i.len() != 2 || p.len() != 2 {
        return None;
    }
    let v = vec![("R".to_string(), (r[0], r[1]), false), ("currents".to_string(), (i[0], i[1]), true),
                 ("losses".to_string(), (p[0], p[1]), true)];
    let total = s.out("P_total")?;
    let imb = s.out("imbalance")?;
    let body = k::arrows(&v, &format!("P_total = {} W, imbalance = {}", num(total), num(imb)));
    Some(s.chart("vectors", &format!("{}: the vectors", name(s)),
                 &format!("The two-device values as arrows from the origin: the on-resistances R = ({}, {}) as given; \
                           returned by the model: currents = ({}, {}) A and losses = ({}, {}) W; also returned P_total \
                           = {} W, imbalance = {}.", num(r[0]), num(r[1]), num(i[0]), num(i[1]), num(p[0]), num(p[1]),
                          num(total), num(imb)), &body))
}
