use cpal::{Device, SampleFormat, SupportedFormat};
use neon::prelude::*;

pub fn jsobject_from_device<'a>(
    cx: &mut FunctionContext<'a>,
    device: Device,
) -> JsResult<'a, JsObject> {
    let obj: Handle<JsObject> = JsObject::new(cx);
    let name = cx.string(&device.name());
    obj.set(cx, "name", name)?;
    if let Ok(input_formats) = device.supported_input_formats() {
        let formats: Vec<SupportedFormat> = input_formats.map(|f| f).collect();
        let formats_count = formats.len();
        let js_formats = JsArray::new(cx, formats_count as u32);
        for (i, f) in formats.iter().enumerate() {
            let o = JsObject::new(cx);
            let channels = cx.number(f64::from(f.channels));
            let min_sample_rate = cx.number(f64::from(f.min_sample_rate.0));
            let max_sample_rate = cx.number(f64::from(f.max_sample_rate.0));
            let sample_format = cx.string(&match f.data_type {
                SampleFormat::I16 => "i16",
                SampleFormat::U16 => "u16",
                SampleFormat::F32 => "f32",
            });
            let sample_size = cx.number(f64::from(f.data_type.sample_size() as u32));
            o.set(cx, "channels", channels)?;
            o.set(cx, "min_sample_rate", min_sample_rate)?;
            o.set(cx, "max_sample_rate", max_sample_rate)?;
            o.set(cx, "sample_format", sample_format)?;
            o.set(cx, "sample_size", sample_size)?;

            js_formats.set(cx, i as u32, o)?;
        }

        obj.set(cx, "input_formats", js_formats)?;
    }

    if let Ok(output_formats) = device.supported_output_formats() {
        let formats: Vec<SupportedFormat> = output_formats.map(|f| f).collect();
        let formats_count = formats.len();
        let js_formats = JsArray::new(cx, formats_count as u32);
        for (i, f) in formats.iter().enumerate() {
            let o = JsObject::new(cx);
            let channels = cx.number(f64::from(f.channels));
            let min_sample_rate = cx.number(f64::from(f.min_sample_rate.0));
            let max_sample_rate = cx.number(f64::from(f.max_sample_rate.0));
            let sample_format = cx.string(&match f.data_type {
                SampleFormat::I16 => "i16",
                SampleFormat::U16 => "u16",
                SampleFormat::F32 => "f32",
            });
            let sample_size = cx.number(f64::from(f.data_type.sample_size() as u32));
            o.set(cx, "channels", channels)?;
            o.set(cx, "min_sample_rate", min_sample_rate)?;
            o.set(cx, "max_sample_rate", max_sample_rate)?;
            o.set(cx, "sample_format", sample_format)?;
            o.set(cx, "sample_size", sample_size)?;

            js_formats.set(cx, i as u32, o)?;
        }

        obj.set(cx, "output_formats", js_formats)?;
    }

    if let Ok(f) = device.default_input_format() {
        let o = JsObject::new(cx);
        let channels = cx.number(f64::from(f.channels));
        let sample_rate = cx.number(f64::from(f.sample_rate.0));
        let sample_format = cx.string(&match f.data_type {
            SampleFormat::I16 => "i16",
            SampleFormat::U16 => "u16",
            SampleFormat::F32 => "f32",
        });
        let sample_size = cx.number(f64::from(f.data_type.sample_size() as u32));
        o.set(cx, "channels", channels)?;
        o.set(cx, "sample_rate", sample_rate)?;
        o.set(cx, "sample_format", sample_format)?;
        o.set(cx, "sample_size", sample_size)?;

        obj.set(cx, "default_input_format", o)?;
    }

    if let Ok(f) = device.default_output_format() {
        let o = JsObject::new(cx);
        let channels = cx.number(f64::from(f.channels));
        let sample_rate = cx.number(f64::from(f.sample_rate.0));
        let sample_format = cx.string(&match f.data_type {
            SampleFormat::I16 => "i16",
            SampleFormat::U16 => "u16",
            SampleFormat::F32 => "f32",
        });
        let sample_size = cx.number(f64::from(f.data_type.sample_size() as u32));
        o.set(cx, "channels", channels)?;
        o.set(cx, "sample_rate", sample_rate)?;
        o.set(cx, "sample_format", sample_format)?;
        o.set(cx, "sample_size", sample_size)?;

        obj.set(cx, "default_output_format", o)?;
    }

    Ok(obj)
}

pub fn map_devices_iter<'a>(
    cx: &mut FunctionContext<'a>,
    device_iter: impl Iterator<Item = Device>,
) -> JsResult<'a, JsArray> {
    let devices: Vec<Device> = device_iter.map(|d| d).collect();
    let device_count = devices.len();

    let ret: Handle<JsArray> = JsArray::new(cx, device_count as u32);
    for (i, d) in devices.iter().enumerate() {
        if let Ok(jso) = jsobject_from_device(cx, d.clone()) {
            let _ = ret.set(cx, i as u32, jso)?;
        }
    }

    Ok(ret)
}
