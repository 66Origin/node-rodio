use cpal::{Device, SampleFormat, SupportedFormat};
use neon::{js::{JsArray, JsNumber, JsObject, JsString, Object},
           mem::Handle,
           scope::Scope,
           vm::JsResult};

pub fn jsobject_from_device<'a>(
    scope: &mut impl Scope<'a>,
    device: Device,
) -> JsResult<'a, JsObject> {
    let obj: Handle<JsObject> = JsObject::new(scope);
    obj.set("name", JsString::new(scope, &device.name()).unwrap())?;
    if let Ok(input_formats) = device.supported_input_formats() {
        let formats: Vec<SupportedFormat> = input_formats.map(|f| f).collect();
        let formats_count = formats.len();
        let js_formats = JsArray::new(scope, formats_count as u32);
        for (i, f) in formats.iter().enumerate() {
            let o = JsObject::new(scope);
            o.set("channels", JsNumber::new(scope, f64::from(f.channels)))
                .unwrap();
            o.set(
                "min_sample_rate",
                JsNumber::new(scope, f64::from(f.min_sample_rate.0)),
            ).unwrap();
            o.set(
                "max_sample_rate",
                JsNumber::new(scope, f64::from(f.max_sample_rate.0)),
            ).unwrap();
            o.set(
                "sample_format",
                JsString::new(
                    scope,
                    &match f.data_type {
                        SampleFormat::I16 => "i16",
                        SampleFormat::U16 => "u16",
                        SampleFormat::F32 => "f32",
                    },
                ).unwrap(),
            ).unwrap();
            o.set(
                "sample_size",
                JsNumber::new(scope, f64::from(f.data_type.sample_size() as u32)),
            ).unwrap();

            let _ = js_formats.set(i as u32, o);
        }

        obj.set("input_formats", js_formats)?;
    }

    if let Ok(output_formats) = device.supported_output_formats() {
        let formats: Vec<SupportedFormat> = output_formats.map(|f| f).collect();
        let formats_count = formats.len();
        let js_formats = JsArray::new(scope, formats_count as u32);
        for (i, f) in formats.iter().enumerate() {
            let o = JsObject::new(scope);
            o.set("channels", JsNumber::new(scope, f64::from(f.channels)))
                .unwrap();
            o.set(
                "min_sample_rate",
                JsNumber::new(scope, f64::from(f.min_sample_rate.0)),
            ).unwrap();
            o.set(
                "max_sample_rate",
                JsNumber::new(scope, f64::from(f.max_sample_rate.0)),
            ).unwrap();
            o.set(
                "sample_format",
                JsString::new(
                    scope,
                    &match f.data_type {
                        SampleFormat::I16 => "i16",
                        SampleFormat::U16 => "u16",
                        SampleFormat::F32 => "f32",
                    },
                ).unwrap(),
            ).unwrap();
            o.set(
                "sample_size",
                JsNumber::new(scope, f64::from(f.data_type.sample_size() as u32)),
            ).unwrap();

            let _ = js_formats.set(i as u32, o);
        }

        obj.set("output_formats", js_formats)?;
    }

    if let Ok(f) = device.default_input_format() {
        let o = JsObject::new(scope);
        o.set("channels", JsNumber::new(scope, f64::from(f.channels)))
            .unwrap();
        o.set(
            "sample_rate",
            JsNumber::new(scope, f64::from(f.sample_rate.0)),
        ).unwrap();
        o.set(
            "sample_format",
            JsString::new(
                scope,
                &match f.data_type {
                    SampleFormat::I16 => "i16",
                    SampleFormat::U16 => "u16",
                    SampleFormat::F32 => "f32",
                },
            ).unwrap(),
        ).unwrap();
        o.set(
            "sample_size",
            JsNumber::new(scope, f64::from(f.data_type.sample_size() as u32)),
        ).unwrap();

        obj.set("default_input_format", o)?;
    }

    if let Ok(f) = device.default_output_format() {
        let o = JsObject::new(scope);
        o.set("channels", JsNumber::new(scope, f64::from(f.channels)))
            .unwrap();
        o.set(
            "sample_rate",
            JsNumber::new(scope, f64::from(f.sample_rate.0)),
        ).unwrap();
        o.set(
            "sample_format",
            JsString::new(
                scope,
                &match f.data_type {
                    SampleFormat::I16 => "i16",
                    SampleFormat::U16 => "u16",
                    SampleFormat::F32 => "f32",
                },
            ).unwrap(),
        ).unwrap();
        o.set(
            "sample_size",
            JsNumber::new(scope, f64::from(f.data_type.sample_size() as u32)),
        ).unwrap();

        obj.set("default_output_format", o)?;
    }

    Ok(obj)
}

pub fn map_devices_iter<'a>(
    scope: &mut impl Scope<'a>,
    device_iter: impl Iterator<Item = Device>,
) -> JsResult<'a, JsArray> {
    let devices: Vec<Device> = device_iter.map(|d| d).collect();
    let device_count = devices.len();

    let ret: Handle<JsArray> = JsArray::new(scope, device_count as u32);
    for (i, d) in devices.iter().enumerate() {
        if let Ok(jso) = jsobject_from_device(scope, d.clone()) {
            let _ = ret.set(i as u32, jso)?;
        }
    }

    Ok(ret)
}
