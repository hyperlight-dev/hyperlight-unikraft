// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! WebAssembly values as JSON, both ways: a call's arguments and result,
//! and what a host function takes and returns.  The type decides how JSON
//! reads, so the host writes plain JSON:
//!
//! | WebAssembly | JSON |
//! |---|---|
//! | integers, floats | numbers |
//! | `bool` | `true`/`false` |
//! | `char`, `string` | strings |
//! | `list<T>`, `tuple<…>` | arrays |
//! | `record` | an object of its fields |
//! | `option<T>` | `null`, or the value |
//! | `result<T, E>` | `{"ok": T}` or `{"err": E}` |
//! | `enum` | the case's name |
//! | `variant` | `{"case": payload}`, or the name of a case without one |
//! | `flags` | an array of the names set |

use serde_json::{Map, Number, Value as Json};
use wasmtime::component::{Type, Val};
use wasmtime::{Result, ValType, bail, format_err};

// ── Core module values ──────────────────────────────────────────────

pub fn core_from_json(ty: &ValType, j: &Json) -> Result<wasmtime::Val> {
    let n = |what| {
        j.as_f64()
            .ok_or_else(|| format_err!("expected a number for {what}, got {j}"))
    };
    Ok(match ty {
        // An i32 or i64 is signless: its bits take either a signed or
        // an unsigned number, but nothing wider is cut down to fit.
        ValType::I32 => wasmtime::Val::I32(match int(j)? {
            v @ 0..=0xffff_ffff => v as u32 as i32,
            v if (i32::MIN as i64..0).contains(&v) => v as i32,
            v => bail!("{v} does not fit an i32"),
        }),
        ValType::I64 => wasmtime::Val::I64(match j.as_u64() {
            Some(u) => u as i64,
            None => int(j)?,
        }),
        ValType::F32 => wasmtime::Val::F32((n("f32")? as f32).to_bits()),
        ValType::F64 => wasmtime::Val::F64(n("f64")?.to_bits()),
        other => bail!("a {other} parameter cannot be passed as JSON"),
    })
}

pub fn core_to_json(v: &wasmtime::Val) -> Result<Json> {
    Ok(match v {
        wasmtime::Val::I32(x) => Json::from(*x),
        wasmtime::Val::I64(x) => Json::from(*x),
        wasmtime::Val::F32(x) => float(f32::from_bits(*x) as f64),
        wasmtime::Val::F64(x) => float(f64::from_bits(*x)),
        _ => bail!("only numbers can be returned as JSON from a module"),
    })
}

/// A signed 64-bit integer as JSON gives it, or a float with no fraction
/// that one holds exactly; anything else is an error, never a wrapped or
/// saturated value.
fn int(j: &Json) -> Result<i64> {
    if let Some(v) = j.as_i64() {
        return Ok(v);
    }
    // Every i64 from -2^63 up to (not including) 2^63.
    let range = (i64::MIN as f64)..-(i64::MIN as f64);
    match j.as_f64() {
        Some(f) if f.fract() == 0.0 && range.contains(&f) => Ok(f as i64),
        _ => bail!("expected an integer that fits 64 bits, got {j}"),
    }
}

fn float(f: f64) -> Json {
    Number::from_f64(f).map_or(Json::Null, Json::Number)
}

// ── Component values ────────────────────────────────────────────────

pub fn from_json(ty: &Type, j: &Json) -> Result<Val> {
    let int_in = |lo: i64, hi: i64| -> Result<i64> {
        let v = int(j)?;
        if v < lo || v > hi {
            bail!("{v} is out of range for {ty:?}");
        }
        Ok(v)
    };
    Ok(match ty {
        Type::Bool => Val::Bool(
            j.as_bool()
                .ok_or_else(|| format_err!("expected a bool, got {j}"))?,
        ),
        Type::S8 => Val::S8(int_in(i8::MIN.into(), i8::MAX.into())? as i8),
        Type::U8 => Val::U8(int_in(0, u8::MAX.into())? as u8),
        Type::S16 => Val::S16(int_in(i16::MIN.into(), i16::MAX.into())? as i16),
        Type::U16 => Val::U16(int_in(0, u16::MAX.into())? as u16),
        Type::S32 => Val::S32(int_in(i32::MIN.into(), i32::MAX.into())? as i32),
        Type::U32 => Val::U32(int_in(0, u32::MAX.into())? as u32),
        Type::S64 => Val::S64(int(j)?),
        Type::U64 => Val::U64(match j.as_u64() {
            Some(u) => u,
            // An integral float, as the other integer types take one.
            None => match j.as_f64() {
                Some(f) if f.fract() == 0.0 && (0.0..18_446_744_073_709_551_616.0).contains(&f) => {
                    f as u64
                }
                _ => bail!("expected an unsigned 64-bit integer, got {j}"),
            },
        }),
        Type::Float32 => Val::Float32(number(j)? as f32),
        Type::Float64 => Val::Float64(number(j)?),
        Type::Char => {
            let s = string(j)?;
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Val::Char(c),
                _ => bail!("expected a one-character string, got {j}"),
            }
        }
        Type::String => Val::String(string(j)?.to_string()),
        Type::List(list) => {
            let elem = list.ty();
            Val::List(
                array(j)?
                    .iter()
                    .map(|e| from_json(&elem, e))
                    .collect::<Result<_>>()?,
            )
        }
        Type::Tuple(tuple) => {
            let items = array(j)?;
            let types: Vec<Type> = tuple.types().collect();
            if items.len() != types.len() {
                bail!("expected a {}-element array, got {j}", types.len());
            }
            Val::Tuple(
                types
                    .iter()
                    .zip(items)
                    .map(|(t, e)| from_json(t, e))
                    .collect::<Result<_>>()?,
            )
        }
        Type::Record(record) => {
            let obj = j
                .as_object()
                .ok_or_else(|| format_err!("expected an object, got {j}"))?;
            Val::Record(
                record
                    .fields()
                    .map(|f| {
                        // An absent field is `null`, which reads as `none`
                        // for an option and fails for anything else.
                        let v = obj.get(f.name).unwrap_or(&Json::Null);
                        Ok((f.name.to_string(), from_json(&f.ty, v)?))
                    })
                    .collect::<Result<_>>()?,
            )
        }
        Type::Option(opt) => match j {
            Json::Null => Val::Option(None),
            v => Val::Option(Some(Box::new(from_json(&opt.ty(), v)?))),
        },
        Type::Result(res) => {
            let obj = j
                .as_object()
                .filter(|o| o.len() == 1)
                .ok_or_else(|| format_err!("expected {{\"ok\": …}} or {{\"err\": …}}, got {j}"))?;
            let payload = |t: Option<Type>, v: &Json| -> Result<Option<Box<Val>>> {
                t.map(|t| from_json(&t, v).map(Box::new)).transpose()
            };
            match obj.iter().next().unwrap() {
                (k, v) if k == "ok" => Val::Result(Ok(payload(res.ok(), v)?)),
                (k, v) if k == "err" => Val::Result(Err(payload(res.err(), v)?)),
                _ => bail!("expected {{\"ok\": …}} or {{\"err\": …}}, got {j}"),
            }
        }
        Type::Enum(e) => {
            let name = string(j)?;
            if !e.names().any(|n| n == name) {
                bail!("{name:?} is not a case of the enum");
            }
            Val::Enum(name.to_string())
        }
        Type::Variant(variant) => {
            let (name, payload) = match j {
                Json::String(s) => (s.as_str(), None),
                Json::Object(o) if o.len() == 1 => {
                    let (k, v) = o.iter().next().unwrap();
                    (k.as_str(), Some(v))
                }
                _ => bail!("expected a case name or {{\"case\": payload}}, got {j}"),
            };
            let case = variant
                .cases()
                .find(|c| c.name == name)
                .ok_or_else(|| format_err!("{name:?} is not a case of the variant"))?;
            let payload = match (case.ty, payload) {
                (Some(t), Some(v)) => Some(Box::new(from_json(&t, v)?)),
                (None, None) => None,
                (Some(_), None) => bail!("case {name:?} takes a payload"),
                (None, Some(_)) => bail!("case {name:?} takes no payload"),
            };
            Val::Variant(name.to_string(), payload)
        }
        Type::Flags(flags) => {
            let set: Vec<String> = array(j)?
                .iter()
                .map(|n| string(n).map(str::to_string))
                .collect::<Result<_>>()?;
            if let Some(bad) = set.iter().find(|n| !flags.names().any(|f| f == n.as_str())) {
                bail!("{bad:?} is not a flag");
            }
            Val::Flags(set)
        }
        other => bail!("a {other:?} cannot be passed as JSON"),
    })
}

pub fn to_json(v: &Val) -> Result<Json> {
    Ok(match v {
        Val::Bool(b) => Json::Bool(*b),
        Val::S8(x) => Json::from(*x),
        Val::U8(x) => Json::from(*x),
        Val::S16(x) => Json::from(*x),
        Val::U16(x) => Json::from(*x),
        Val::S32(x) => Json::from(*x),
        Val::U32(x) => Json::from(*x),
        Val::S64(x) => Json::from(*x),
        Val::U64(x) => Json::from(*x),
        Val::Float32(x) => float(*x as f64),
        Val::Float64(x) => float(*x),
        Val::Char(c) => Json::String(c.to_string()),
        Val::String(s) => Json::String(s.clone()),
        Val::List(items) | Val::Tuple(items) | Val::FixedLengthList(items) => {
            Json::Array(items.iter().map(to_json).collect::<Result<_>>()?)
        }
        Val::Record(fields) => Json::Object(
            fields
                .iter()
                .map(|(k, v)| Ok((k.clone(), to_json(v)?)))
                .collect::<Result<Map<_, _>>>()?,
        ),
        Val::Option(None) => Json::Null,
        Val::Option(Some(v)) => to_json(v)?,
        Val::Result(r) => {
            let (key, payload) = match r {
                Ok(p) => ("ok", p),
                Err(p) => ("err", p),
            };
            let payload = match payload {
                Some(v) => to_json(v)?,
                None => Json::Null,
            };
            Json::Object(Map::from_iter([(key.to_string(), payload)]))
        }
        Val::Enum(name) => Json::String(name.clone()),
        Val::Variant(name, None) => Json::String(name.clone()),
        Val::Variant(name, Some(v)) => Json::Object(Map::from_iter([(name.clone(), to_json(v)?)])),
        Val::Flags(names) => Json::Array(names.iter().cloned().map(Json::String).collect()),
        other => bail!("a {other:?} cannot be returned as JSON"),
    })
}

fn number(j: &Json) -> Result<f64> {
    j.as_f64()
        .ok_or_else(|| format_err!("expected a number, got {j}"))
}

fn string(j: &Json) -> Result<&str> {
    j.as_str()
        .ok_or_else(|| format_err!("expected a string, got {j}"))
}

fn array(j: &Json) -> Result<&Vec<Json>> {
    j.as_array()
        .ok_or_else(|| format_err!("expected an array, got {j}"))
}

/// A function's results as one JSON value: none is no result at all, one
/// is that value, several an array.
pub fn results(values: Vec<Json>) -> Option<Json> {
    match values.len() {
        0 => None,
        1 => values.into_iter().next(),
        _ => Some(Json::Array(values)),
    }
}

/// The arguments of a call: a JSON array, one element per parameter
/// (empty input for none).
pub fn arguments(input: &str, count: usize) -> Result<Vec<Json>> {
    if input.trim().is_empty() && count == 0 {
        return Ok(Vec::new());
    }
    match serde_json::from_str(input) {
        Ok(Json::Array(items)) if items.len() == count => Ok(items),
        Ok(other) => bail!("expected a JSON array of {count} arguments, got {other}"),
        Err(e) => bail!("the input is not JSON: {e}"),
    }
}
