use rodio;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Receiver, Sender};
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

#[derive(Debug)]
pub struct NodeRodioController {
    tx: Sender<NodeRodioCommand>,
    rx_out: Receiver<()>,
}

impl NodeRodioController {
    pub fn new(mut sink: rodio::Sink) -> Self {
        let (tx, rx) = channel();
        let (tx_out, rx_out) = channel();

        let timeout = ::std::time::Duration::from_millis(1);

        thread::spawn(move || {
            let mut added_once = false;
            let mut played_once = false;
            loop {
                if added_once && played_once && sink.empty() {
                    sink.stop();
                    sink.detach();
                    let _ = tx_out.send(());
                    break;
                }

                if let Ok(command) = rx.recv_timeout(timeout) {
                    match command {
                        NodeRodioCommand::Append(path) => {
                            if let Ok(file) = File::open(&path) {
                                if let Ok(decoder) = rodio::Decoder::new(BufReader::new(file)) {
                                    sink.append(decoder);
                                    added_once = true;
                                }
                            }
                        }
                        NodeRodioCommand::Play => {
                            sink.play();
                            played_once = true;
                        }
                        NodeRodioCommand::Pause => sink.pause(),
                        NodeRodioCommand::Stop => {
                            sink.stop();
                            break;
                        }
                        NodeRodioCommand::Volume(vol) => sink.set_volume(vol),
                    }
                }
            }
        });

        NodeRodioController { tx, rx_out }
    }

    pub fn send(&self, cmd: NodeRodioCommand) {
        let _ = self.tx.send(cmd);
    }

    pub fn wait(&self) {
        let _ = self.rx_out.recv();
    }
}
