use neon::js::{JsObject, JsString, Object};
use neon::mem::Handle;
use neon::vm::{Call, JsResult};
use rodio;

pub fn default_input_device(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let obj: Handle<JsObject> = JsObject::new(scope);
    if let Some(device) = rodio::default_input_device() {
        obj.set("name", JsString::new(scope, &device.name()).unwrap())?;
    }
    Ok(obj)
}

pub fn default_output_device(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let obj: Handle<JsObject> = JsObject::new(scope);
    if let Some(device) = rodio::default_output_device() {
        obj.set("name", JsString::new(scope, &device.name()).unwrap())?;
    }
    Ok(obj)
}
