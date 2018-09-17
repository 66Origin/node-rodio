use rodio;
use std::fs::File;
use std::io::BufReader;

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
}

pub struct NodeRodioController {
    pub sink: rodio::Sink,
}

impl NodeRodioController {
    pub fn new(sink: rodio::Sink) -> Self {
        NodeRodioController { sink }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn send(&self, cmd: NodeRodioCommand) -> Result<(), String> {
        match cmd {
            NodeRodioCommand::Append(path) => match File::open(&path) {
                Ok(file) => match rodio::Decoder::new(BufReader::new(file)) {
                    Ok(decoder) => {
                        self.sink.append(decoder);
                    }
                    Err(e) => return Err(format!("{}", e)),
                },
                Err(e) => return Err(format!("{}", e)),
            },
            NodeRodioCommand::Play => {
                self.sink.play();
            }
            NodeRodioCommand::Pause => self.sink.pause(),
            NodeRodioCommand::Stop => {
                self.sink.stop();
            }
        }

        Ok(())
    }
}
