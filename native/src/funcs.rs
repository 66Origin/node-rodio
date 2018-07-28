use neon::prelude::*;
use rodio;

use super::support::*;

pub fn default_input_device(mut cx: FunctionContext) -> JsResult<JsObject> {
    match rodio::default_input_device() {
        Some(device) => jsobject_from_device(&mut cx, device),
        None => cx.throw_error("No default input device set"),
    }
}

pub fn default_output_device(mut cx: FunctionContext) -> JsResult<JsObject> {
    match rodio::default_output_device() {
        Some(device) => jsobject_from_device(&mut cx, device),
        None => cx.throw_error("No default output device set"),
    }
}

pub fn devices(mut cx: FunctionContext) -> JsResult<JsArray> {
    map_devices_iter(&mut cx, rodio::devices())
}

pub fn input_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
    map_devices_iter(&mut cx, rodio::input_devices())
}

pub fn output_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
    map_devices_iter(&mut cx, rodio::output_devices())
}
