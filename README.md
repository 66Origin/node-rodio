# node-rodio

[Rodio](https://github.com/tomaka/rodio) (Rust audio playback library) bindings for Node.js, built with [Neon](https://www.neon-bindings.com/)

## Installation

`npm install @yellowinnovation/node-rodio`

## Usage

```javascript

const rodio = require('node-rodio');

console.log(rodio.defaultInputDevice()); // { name: "Your default microphone" }
console.log(rodio.defaultOutputDevice()); // { name: "Your default speakers/headphones" }
const player = new rodio.Player(); // Initializes a new player

player.append("./samples/music.mp3"); // Loads a file in the queue
player.append("./samples/beel.wav"); // Another one that will play after the music.mp3
// If you'd like to get sounds in parallel, just create another player and make them .play(); at the same time!
player.play() // Starts playback
player.volume(0.5); // Sets volume to 50%;
player.pause(); // Pauses playback
player.play(); // Resumes playback
player.volume(1.0); // Sets the volume to 100%
player.wait(); // blocks current thread until queue is finished

player.stop(); // Stops playback completely and empties queue.
// player is not usable at this point since we killed the background thread.

```

## License

Licensed under:

* Apache License, Version 2.0, ([LICENSE](LICENSE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))

## Credits

* Huge props to @tomaka for his amazing work on [rodio](https://github.com/tomaka/rodio) / [cpal](https://github.com/tomaka/cpal)

## Yellow Innovation

Yellow Innovation is the innovation laboratory of the French postal service: La Poste.

We create innovative user experiences and journeys through services with a focus on IoT lately.

[Yellow Innovation's website and works](http://yellowinnovation.fr/en/)
