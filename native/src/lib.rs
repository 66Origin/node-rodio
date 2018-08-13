#[macro_use]
extern crate neon;
extern crate cpal;
extern crate crossbeam_channel;
extern crate rodio;

use neon::prelude::*;

use std::sync::{Arc, RwLock};

mod controller;
mod funcs;
mod support;

use self::controller::{NodeRodioCommand, NodeRodioController};

#[derive(Debug)]
pub struct NodeRodio {
    controller: Arc<RwLock<NodeRodioController>>,
}

impl NodeRodio {
    pub fn new() -> Option<Self> {
        let device = rodio::default_output_device()?;
        let sink = rodio::Sink::new(&device);
        let controller = NodeRodioController::new(sink);
        let _ = controller.send(NodeRodioCommand::Pause);

        Some(NodeRodio {
            controller: Arc::new(RwLock::new(controller)),
        })
    }
}

struct WaitTask {
    controller: Arc<RwLock<NodeRodioController>>,
}

impl Task for WaitTask {
    type Output = ();
    type Error = String;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        match self.controller.read() {
            Ok(controller) => controller.send(NodeRodioCommand::Play),
            Err(e) => return Err(format!("{}", e)),
        }

        if let Ok(controller) = self.controller.read() {
            controller.wait();
        }

        Ok(())
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(_) => Ok(cx.undefined()),
            Err(e) => cx.throw_error(&e),
        }
    }
}

macro_rules! send {
    ($cx:ident, $cmd:ident) => {{
        let this = $cx.this();
        let guard = $cx.lock();
        let nrodio = this.borrow(&guard);
        if let Ok(controller) = nrodio.controller.read() {
            controller.send($cmd);
        }
        drop(this);
    }};
}

declare_types! {
    pub class JsRodio for NodeRodio {
        init(mut cx) {
            if let Some(rodio) = NodeRodio::new() {
                Ok(rodio)
            } else {
                cx.throw_error("No default output device set")
            }
        }

        method play(mut cx) {
            let f = cx.argument::<JsFunction>(0)?;
            let t = {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                WaitTask {
                    controller: nrodio.controller.clone()
                }
            };

            t.schedule(f);
            Ok(cx.undefined().upcast())
        }

        method resume(mut cx) {
            let cmd = NodeRodioCommand::Play;
            send!(cx, cmd);
            Ok(cx.undefined().upcast())
        }

        method pause(mut cx) {
            let cmd = NodeRodioCommand::Pause;
            send!(cx, cmd);
            Ok(cx.undefined().upcast())
        }

        method stop(mut cx) {
            let cmd = NodeRodioCommand::Stop;
            send!(cx, cmd);
            Ok(cx.undefined().upcast())
        }

        method append(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let cmd = NodeRodioCommand::Append(path);
            send!(cx, cmd);
            Ok(cx.undefined().upcast())
        }

        method volume(mut cx) {
            let vol: f64 = cx.argument::<JsNumber>(0)?.value();
            let cmd = NodeRodioCommand::Volume(vol as f32);
            send!(cx, cmd);
            Ok(cx.undefined().upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsRodio>("Player")?;

    cx.export_function("defaultInputDevice", funcs::default_input_device)?;
    cx.export_function("defaultOutputDevice", funcs::default_output_device)?;
    cx.export_function("devices", funcs::devices)?;
    cx.export_function("outputDevices", funcs::output_devices)?;
    cx.export_function("inputDevices", funcs::input_devices)?;

    Ok(())
});
