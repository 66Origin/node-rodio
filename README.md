# node-rodio

[Rodio](https://github.com/tomaka/rodio) (Rust audio playback library) bindings for Node.js, built with [Neon](https://www.neon-bindings.com/)

## Installation

`npm install @yellowinnovation/node-rodio`

## Usage

```javascript

const rodio = require('node-rodio');

try {
    console.log(rodio.defaultInputDevice()); // { name: "Your default microphone" ... sample rate, format etc }
    console.log(rodio.defaultOutputDevice()); // { name: "Your default speakers/headphones" ... sample rate, format etc }
    console.log(rodio.devices()); // Lists all devices on the machine
    console.log(rodio.inputDevices()); // Lists all input devices on the machine
    console.log(rodio.outputDevices()); // Lists all output devices on the machine

    const player = new rodio.Player(); // Initializes a new player

    player.append("./samples/music.mp3"); // Loads a file in the queue
    player.append("./samples/beep.wav"); // Another one that will play after the music.mp3
    // If you'd like to get sounds in parallel, just create another player and make them .play(); at the same time!
    player.play() // Starts playback
    player.volume(0.5); // Sets volume to 50%;
    player.pause(); // Pauses playback
    player.play(); // Resumes playback
    player.volume(1.0); // Sets the volume to 100%
    player.wait(); // blocks current thread until queue is finished
    player.stop(); // Stops playback completely and empties queue.
    // player is not usable at this point since we killed the background thread.
} catch (e) {
    console.error(e); // all functions can throw in case there's a problem with system configuration or you did something wrong
}
```

If you'd like, there's an other, a bit more raw, API available (useful if you'd like to integrate `Player` in your own API)

```javascript
try {
    const player = new rodio.Player(); // Initializes a new player

    player.send("append", "./samples/music.mp3"); // Loads a file in the queue
    player.send("append", "./samples/beep.wav"); // Another one that will play after the music.mp3
    // If you'd like to get sounds in parallel, just create another player and make them .send("play"); at the same time!
    player.send("play") // Starts playback
    player.send("volume", 0.5); // Sets volume to 50%;
    player.send("pause"); // Pauses playback
    player.send("play"); // Resumes playback
    player.send("volume", 1.0); // Sets the volume to 100%
    player.send("wait"); // blocks current thread until queue is finished
    player.send("stop"); // Stops playback completely and empties queue.
    // player is not usable at this point since we killed the background thread.
} catch (e) {
    console.error(e); // all functions can throw in case there's a problem with system configuration or you did something wrong
}
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
