use rodio;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::thread;

/// Commands that are being sent to the controller
#[derive(Debug, Clone)]
pub enum NodeRodioCommand {
    /// Adds a new sound file to the mixer
    Append(String),
    /// Triggers play. Does nothing if not paused.
    Play,
    /// Triggers pause. Does nothing is already paused
    Pause,
    /// Stops playback. WARNING: you cannot resume playback after that
    Stop,
    /// Changes volume
    Volume(f32),
}

#[derive(Debug, Clone)]
pub struct NodeRodioController {
    tx: Sender<NodeRodioCommand>,
}

impl NodeRodioController {
    pub fn new(mut sink: rodio::Sink) -> Self {
        let (tx, rx) = channel();

        thread::spawn(move || loop {
            if let Ok(command) = rx.recv() {
                match command {
                    NodeRodioCommand::Append(path) => {
                        if let Ok(file) = File::open(&path) {
                            if let Ok(decoder) = rodio::Decoder::new(BufReader::new(file)) {
                                sink.append(decoder);
                            }
                        }
                    }
                    NodeRodioCommand::Play => sink.play(),
                    NodeRodioCommand::Pause => sink.pause(),
                    NodeRodioCommand::Stop => {
                        sink.stop();
                        sink.detach();
                        break;
                    }
                    NodeRodioCommand::Volume(vol) => sink.set_volume(vol),
                }
            }
        });

        NodeRodioController { tx }
    }

    pub fn send(&self, cmd: NodeRodioCommand) {
        let _ = self.tx.send(cmd);
    }
}
