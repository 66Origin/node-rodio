#[macro_use]
extern crate neon;
extern crate arcmutex;
extern crate cpal;
extern crate rodio;

use neon::{
    js::class::{Class, JsClass}, js::error::{JsError, Kind},
    js::{JsFunction, JsNumber, JsUndefined, Object, Value}, mem::Handle, scope::Scope, task::Task,
    vm::{JsResult, Lock},
};

use arcmutex::*;

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
        sink.pause();
        let controller = NodeRodioController::new(sink);

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
        match self.controller.lock() {
            Ok(controller) => match controller.wait() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{}", e)),
            },
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn complete<'a, T: Scope<'a>>(
        self,
        _: &'a mut T,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(_) => Ok(JsUndefined::new()),
            Err(e) => JsError::throw(Kind::Error, &e),
        }
    }
}

declare_types! {
    pub class JsRodio for NodeRodio {
        init(_) {
            if let Some(rodio) = NodeRodio::new() {
                Ok(rodio)
            } else {
                JsError::throw(Kind::Error, "No default output device set")
            }
        }

        method play(call) {
            match call.arguments.this(call.scope).grab(|nrodio| {
                let cmd = NodeRodioCommand::Play;
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }

        method pause(call) {
            match call.arguments.this(call.scope).grab(|nrodio| {
                let cmd = NodeRodioCommand::Pause;
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }

        method stop(call) {
            match call.arguments.this(call.scope).grab(|nrodio| {
                let cmd = NodeRodioCommand::Stop;
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }

        method append(call) {
            let scope = call.scope;
            let path = call.arguments.require(scope, 0)?.to_string(scope)?.value();

            match call.arguments.this(scope).grab(|nrodio| {
                let cmd = NodeRodioCommand::Append(path);
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }

        method volume(call) {
            let scope = call.scope;
            let vol: f64 = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value();

            match call.arguments.this(scope).grab(|nrodio| {
                let cmd = NodeRodioCommand::Volume(vol as f32);
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(cmd),
                    Err(_) => Err(SendError(cmd))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }

        method wait(call) {
            let scope = call.scope;
            let f = call.arguments.require(scope, 0)?.check::<JsFunction>()?;
            let t = WaitTask {
                controller: call.arguments.this(scope).grab(|nrodio| {
                    nrodio.controller.clone()
                })
            };
            t.schedule(f);
            Ok(JsUndefined::new().upcast())
        }

         method send(call) {
            let scope = call.scope;
            let command_str = call.arguments.require(scope, 0)?.to_string(scope)?.value();
            let command = match command_str.as_str() {
                "play" => NodeRodioCommand::Play,
                "pause" => NodeRodioCommand::Pause,
                "stop" => NodeRodioCommand::Stop,
                "append" => NodeRodioCommand::Append(call.arguments.require(scope, 1)?.to_string(scope)?.value()),
                "volume" => NodeRodioCommand::Volume(call.arguments.require(scope, 1)?.check::<JsNumber>()?.value() as f32),
                _ => return JsError::throw(Kind::Error, "Invalid command given to controller")
            };

            match call.arguments.this(scope).grab(|nrodio| {
                match nrodio.controller.lock() {
                    Ok(controller) => controller.send(command),
                    Err(_) => Err(SendError(command))
                }
            }) {
                Ok(_) => Ok(JsUndefined::new().upcast()),
                Err(_) => JsError::throw(Kind::Error, "The internal rodio thread is busy or has been already killed")
            }
        }
    }
}

register_module!(m, {
    let class: Handle<JsClass<JsRodio>> = JsRodio::class(m.scope)?;
    let ctor: Handle<JsFunction<JsRodio>> = class.constructor(m.scope)?;
    m.exports.set("Player", ctor)?;

    m.export("defaultInputDevice", funcs::default_input_device)?;
    m.export("defaultOutputDevice", funcs::default_output_device)?;
    m.export("devices", funcs::devices)?;
    m.export("outputDevices", funcs::output_devices)?;
    m.export("inputDevices", funcs::input_devices)?;

    Ok(())
});
