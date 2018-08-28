const rodio = require('..');

const p = new rodio.Player();
const p2 = new rodio.Player();
const p3 = new rodio.Player();

try {
    console.log('Next should be a file not found error');
    p3.append('./samples/not_found.mp3');
} catch (e) {
    console.error(e);
}

try {
    p.append('./samples/music.wav');
    p2.append('./samples/beep.wav');

    Promise.all([
        new Promise((res, rej) => {
            p.play(err => {
                if (err) {
                    return rej(err);
                }

                console.log('done music');
                p2.resume();
                res();
            });
        }),
        new Promise((res, rej) => {
            p2.play(err => {
                if (err) {
                    return rej(err);
                }

                console.log('done beeping');
                res();
            });
        })
    ])
        .then(() => {
            console.log('all done');
            process.exit(0);
        })
        .catch(err => {
            console.error(err);
            return process.exit(1);
        });


    setInterval(() => {
        console.log('waiting & doing stuff in parallel');
    }, 1000);

    setTimeout(() => {
        p2.pause();
    }, 3000);
} catch (e) {
    console.error(e);
    process.exit(1);
}
