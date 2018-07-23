const rodio = require('..');

const p = new rodio.Player();
try {
    p.append('./samples/music.mp3');
    p.play(function () {
        console.log(arguments);
        console.log('done');
        process.exit(0);
    });

    setInterval(() => {
        console.log('waiting & doing stuff in parallel');
    }, 1000);
} catch (e) {
    console.error(e);
}
