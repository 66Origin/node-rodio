const rodio = require('..');

const p = new rodio.Player();
const p2 = new rodio.Player();
try {
    p.append('./samples/music.wav');
    p2.append('./samples/beep.wav');

    Promise.all([
        new Promise(res => {
            p.play(function () {
                console.log('done music');
                p2.resume();
                res();
            });
        }),
        new Promise(res => {
            p2.play(function () {
                console.log('done beeping');
                res();
            });
        })
    ])
        .then(() => {
            console.log('all done');
            process.exit(0);
        });


    setInterval(() => {
        console.log('waiting & doing stuff in parallel');
    }, 1000);

    setTimeout(() => {
        p2.pause();
    }, 3000);
} catch (e) {
    console.error(e);
}
