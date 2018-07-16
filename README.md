# node-rodio

Rodio (Rust audio playback library) bindings for Node.js with Neon

## Usage

```javascript

const rodio = require('node-rodio');

console.log(rodio.defaultInputDevice()); // { name: "Your default microphone" }
console.log(rodio.defaultOutputDevice()); // { name: "Your default speakers/headphones" }
const player = new rodio.Player(); // Initializes a new player

player.append("./samples/music.mp3"); // Start playing a file
player.volume(0.5); // Sets volume to 50%;
player.pause(); // Pauses playback
player.play(); // Resumes playback
player.volume(1.0); // Sets the volume to 100%
player.wait(); // blocks current thread until queue is finished

player.stop(); // player is not usable at this point since we killed the process

```
