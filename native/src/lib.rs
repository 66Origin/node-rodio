#[macro_use]
extern crate neon;
extern crate arcmutex;
extern crate cpal;
extern crate rodio;

use arcmutex::*;
use neon::prelude::*;

use std::sync::mpsc::SendError;

mod controller;
mod funcs;
mod support;

use self::controller::{NodeRodioCommand, NodeRodioController};

#[derive(Debug)]
pub struct NodeRodio {
    controller: ArcMutex<NodeRodioController>,
}

impl NodeRodio {
    pub fn new() -> Option<Self> {
        let device = rodio::default_output_device()?;
        let sink = rodio::Sink::new(&device);
        let controller = NodeRodioController::new(sink);
        let _ = controller.send(NodeRodioCommand::Pause);

        Some(NodeRodio {
            controller: arcmutex(controller),
        })
    }
}

struct WaitTask {
    controller: ArcMutex<NodeRodioController>,
}

impl Task for WaitTask {
    type Output = ();
    type Error = String;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        match self.controller.try_lock() {
            Ok(controller) => match controller.send(NodeRodioCommand::Play) {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            },
            Err(e) => return Err(format!("{}", e)),
        }

        if let Ok(controller) = self.controller.lock() {
            match controller.wait() {
                Ok(_) => {}
                Err(e) => return Err(format!("{}", e)),
            }
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
            match {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let ret = match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                };
                drop(this);
                ret
            } {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(_) => cx.throw_error("The internal rodio thread is busy or has been already killed")
            }
        }

        method pause(mut cx) {
            let cmd = NodeRodioCommand::Pause;
            match {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let ret = match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                };
                drop(this);
                ret
            } {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(_) => cx.throw_error("The internal rodio thread is busy or has been already killed")
            }
        }

        method stop(mut cx) {
            let cmd = NodeRodioCommand::Stop;
            match {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let ret = match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                };
                drop(this);
                ret
            } {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(_) => cx.throw_error("The internal rodio thread is busy or has been already killed")
            }
        }

        method append(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let cmd = NodeRodioCommand::Append(path);
            match {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let ret = match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                };
                drop(this);
                ret
            } {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(_) => cx.throw_error("The internal rodio thread is busy or has been already killed")
            }
        }

        method volume(mut cx) {
            let vol: f64 = cx.argument::<JsNumber>(0)?.value();
            let cmd = NodeRodioCommand::Volume(vol as f32);
            match {
                let this = cx.this();
                let guard = cx.lock();
                let nrodio = this.borrow(&guard);
                let ret = match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                };
                drop(this);
                ret
            } {
                Ok(_) => Ok(cx.undefined().upcast()),
                Err(_) => cx.throw_error("The internal rodio thread is busy or has been already killed")
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
