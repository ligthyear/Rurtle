use super::{Environment, ResultType, RuntimeError, Value};

pub fn head(_: &mut Environment, args: &[Value]) -> ResultType {
    get_args!(args, arg Value::List(ref values), => {
        if values.is_empty() {
            Ok(Value::Nothing)
        } else {
            Ok(values[0].clone())
        }
    })
}

pub fn tail(_: &mut Environment, args: &[Value]) -> ResultType {
    get_args!(args, arg Value::List(ref values), => {
        if values.is_empty() {
            Ok(Value::Nothing)
        } else {
            Ok(Value::List(values[1..].iter().map(|v| v.clone()).collect()))
        }
    })
}

pub fn length(_: &mut Environment, args: &[Value]) -> ResultType {
    get_args!(args, arg Value::List(ref values), => {
        Ok(Value::Number(values.len() as f32))
    })
}

pub fn isempty(_: &mut Environment, args: &[Value]) -> ResultType {
    get_args!(args, arg Value::List(ref values), => {
        Ok(Value::Number(if values.is_empty() { 1. } else { 0. }))
    })
}

pub fn getindex(_: &mut Environment, args: &[Value]) -> ResultType {
    get_args!(args,
              arg Value::List(ref values),
              arg Value::Number(n), =>
    {
        let idx = n as usize;
        if idx >= values.len() {
            Err(RuntimeError(format!("Index out of bounds: {} >= {}", idx, values.len())))
        } else {
            Ok(values[idx].clone())
        }
    })
}

pub fn not(_: &mut Environment, args: &[Value]) -> ResultType {
    let as_boolean = args[0].boolean();
    Ok(Value::Number(if as_boolean { 0. } else { 1. }))
}