#[macro_use]
extern crate neon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate neon_serde;
extern crate rodio;

#[allow(unused_imports)]
use neon::js::{JsArray, JsBoolean, JsFunction, JsInteger, JsNull, JsNumber, JsObject, JsString,
               JsUndefined, Object};
use neon::mem::Handle;
use neon::vm::{Call, JsResult};

use std::fs::File;
use std::io::BufReader;
use std::thread;

#[derive(Serialize, Deserialize)]
struct NodeRodio {
    path: String,
}

impl NodeRodio {
    pub fn new(path: String) -> Self {
        NodeRodio { path }
    }

    pub fn play(&self) {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        let file = File::open(&self.path).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        thread::spawn(move || {
            sink.sleep_until_end();
        });
        println!("success, should be playing now");
    }
}

export! {
    fn play(path: String) -> bool {
        let nr = NodeRodio::new(path);
        nr.play();
        true
    }
}

/*fn default_input_device(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let obj: Handle<JsObject> = JsObject::new(scope);
    if let Some(device) = rodio::default_input_device() {
        obj.set("name", JsString::new(scope, &device.name()).unwrap())?;
    }
    Ok(obj)
}

fn default_output_device(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    let obj: Handle<JsObject> = JsObject::new(scope);
    if let Some(device) = rodio::default_output_device() {
        obj.set("name", JsString::new(scope, &device.name()).unwrap())?;
    }
    Ok(obj)
}

fn play_sound_with_default_output_device(call: Call) -> JsResult<

register_module!(m, {
    m.export("defaultInputDevice", default_input_device)?;
    m.export("defaultOutputDevice", default_output_device)?;
    Ok(())
});*/
