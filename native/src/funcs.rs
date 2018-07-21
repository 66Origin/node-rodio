use neon::js::{error::{JsError, Kind},
               JsArray,
               JsObject};
use neon::vm::{Call, JsResult};
use rodio;

use super::support::*;

pub fn default_input_device<'a>(call: Call<'a>) -> JsResult<JsObject> {
    match rodio::default_input_device() {
        Some(device) => jsobject_from_device(call.scope, device),
        None => JsError::throw(Kind::Error, "No default input device set"),
    }
}

pub fn default_output_device<'a>(call: Call<'a>) -> JsResult<JsObject> {
    match rodio::default_output_device() {
        Some(device) => jsobject_from_device(call.scope, device),
        None => JsError::throw(Kind::Error, "No default output device set"),
    }
}

pub fn devices<'a>(call: Call<'a>) -> JsResult<JsArray> {
    map_devices_iter(call.scope, rodio::devices())
}

pub fn input_devices<'a>(call: Call<'a>) -> JsResult<JsArray> {
    map_devices_iter(call.scope, rodio::input_devices())
}

pub fn output_devices<'a>(call: Call<'a>) -> JsResult<JsArray> {
    map_devices_iter(call.scope, rodio::output_devices())
}
