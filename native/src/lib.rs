#[macro_use]
extern crate neon;
extern crate rodio;

use neon::{
    js::class::{Class, JsClass}, js::{JsFunction, JsNumber, JsUndefined, Object, Value},
    mem::Handle, vm::Lock,
};

mod controller;
mod funcs;

use self::controller::{NodeRodioCommand, NodeRodioController};

#[derive(Debug)]
pub struct NodeRodio {
    controller: NodeRodioController,
}

impl NodeRodio {
    pub fn new() -> Self {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        sink.pause();
        let controller = NodeRodioController::new(sink);

        NodeRodio { controller }
    }
}

declare_types! {
    pub class JsRodio for NodeRodio {
        init(_call) {
            Ok(NodeRodio::new())
        }

        method play(call) {
            call.arguments.this(call.scope).grab(|nrodio| {
                nrodio.controller.send(NodeRodioCommand::Play);
            });

            Ok(JsUndefined::new().upcast())
        }

        method pause(call) {
            call.arguments.this(call.scope).grab(|nrodio| {
                nrodio.controller.send(NodeRodioCommand::Pause);
            });

            Ok(JsUndefined::new().upcast())
        }

        method stop(call) {
            call.arguments.this(call.scope).grab(|nrodio| {
                nrodio.controller.send(NodeRodioCommand::Stop);
            });

            Ok(JsUndefined::new().upcast())
        }

        method append(call) {
            let scope = call.scope;
            let path = call.arguments.require(scope, 0)?.to_string(scope)?.value();

            call.arguments.this(scope).grab(|nrodio| {
                nrodio.controller.send(NodeRodioCommand::Append(path));
            });

            Ok(JsUndefined::new().upcast())
        }

        method volume(call) {
            let scope = call.scope;
            let vol: f64 = call.arguments.require(scope, 0)?.check::<JsNumber>()?.value();

            call.arguments.this(scope).grab(|nrodio| {
                nrodio.controller.send(NodeRodioCommand::Volume(vol as f32));
            });

            Ok(JsUndefined::new().upcast())
        }

        method wait(call) {
            let scope = call.scope;

            call.arguments.this(scope).grab(|nrodio| {
                nrodio.controller.wait();
            });

            Ok(JsUndefined::new().upcast())
        }
    }
}

register_module!(m, {
    let class: Handle<JsClass<JsRodio>> = JsRodio::class(m.scope)?;
    let ctor: Handle<JsFunction<JsRodio>> = class.constructor(m.scope)?;
    m.exports.set("Player", ctor)?;

    m.export("defaultInputDevice", funcs::default_input_device)?;
    m.export("defaultOutputDevice", funcs::default_output_device)?;

    Ok(())
});
