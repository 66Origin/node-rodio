#[macro_use]
extern crate neon;
extern crate cpal;
extern crate rodio;

use neon::prelude::*;

use std::path::Path;
use std::sync::{Arc, RwLock};

mod controller;
mod funcs;
mod support;

use self::controller::{NodeRodioCommand, NodeRodioController};

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
            Ok(controller) => {
                controller.send(NodeRodioCommand::Play)?;
                controller.sink.sleep_until_end();
            }
            Err(e) => return Err(format!("{}", e)),
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
        let res = if let Ok(controller) = nrodio.controller.read() {
            controller.send($cmd)
        } else {
            Err(String::from("controller is locked"))
        };
        drop(this);
        res
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
            match send!(cx, cmd) {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(e) => cx.throw_error(e)
            }
        }

        method pause(mut cx) {
            let cmd = NodeRodioCommand::Pause;
            match send!(cx, cmd) {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(e) => cx.throw_error(e)
            }
        }

        method stop(mut cx) {
            let cmd = NodeRodioCommand::Stop;
            match send!(cx, cmd) {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(e) => cx.throw_error(e)
            }
        }

        method append(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            if !Path::new(&path).exists() {
                return cx.throw_error(format!("{}: File not found", path));
            }
            let cmd = NodeRodioCommand::Append(path);
            match send!(cx, cmd) {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(e) => cx.throw_error(e)
            }
        }

        method volume(mut cx) {
            let vol: f64 = cx.argument::<JsNumber>(0)?.value();
            let res = {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let res_inner = if let Ok(mut controller) = nrodio.controller.try_write() {
                    controller.set_volume(vol as f32);
                    Ok(())
                } else {
                    Err(String::from("controller is locked"))
                };
                drop(this);

                res_inner
            };


            match res {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(e) => cx.throw_error(e)
            }
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
