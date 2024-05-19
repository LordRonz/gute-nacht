use rand::seq::SliceRandom; // 0.7.2

pub fn get_gifs() -> String {
    let gifs = vec![
        "https://c.tenor.com/Aj3tsu_ttp0AAAAC/catfish-goodnight.gif",
        "https://c.tenor.com/yIHvGcTjna0AAAAC/good-night-sweet-dreams.gif",
        "https://c.tenor.com/jWcZxjoan_IAAAAC/phone-line-sleep.gif",
        "https://c.tenor.com/TgxtjkjNCYYAAAAC/good-night-rest-well.gif",
        "https://c.tenor.com/ikuTzYmzV0YAAAAC/sleepy-asleep.gif",
        "https://c.tenor.com/I22QzcmoVBAAAAAC/lullaby-tom-and-jerry.gif",
        "https://c.tenor.com/f0_PPe6lZu4AAAAC/sweet-dreams-goodnight.gif",
        "https://c.tenor.com/QUE9WJnrC3cAAAAC/tonton-friends.gif",
        "https://c.tenor.com/XWUtNSTuNosAAAAC/woodstock-snoopy.gif",
        "https://c.tenor.com/pArd7iXfErcAAAAC/cat-cute.gif",
        "https://c.tenor.com/2LqFJHE4D8wAAAAC/mochi-peach.gif",
        "https://c.tenor.com/2wFMB_AKFEEAAAAC/mochi-peach.gif",
        "https://c.tenor.com/Nez70Tzn_WMAAAAC/goodnight-beautiful.gif",
        "https://c.tenor.com/UyC29aJKbq0AAAAC/mochi-peach-cat-cici.gif",
        "https://c.tenor.com/JFYNEBulItMAAAAC/good-night-siesta.gif",
        "https://c.tenor.com/yeDuJ-25Mr0AAAAC/goodnight-malina.gif",
        "https://c.tenor.com/sUgACjnsxpIAAAAC/milk-and.gif",
        "https://c.tenor.com/G5FScV4As7wAAAAC/good-night.gif",
        "https://c.tenor.com/CX3_RmzsQfwAAAAC/milk-and-mocha-bear-relax.gif",
        "https://c.tenor.com/lNl2wecq1H8AAAAi/good-night.gif",
        "https://c.tenor.com/Rae3dbdO7vYAAAAC/cudd-milk-and-mocha.gif",
        "https://c.tenor.com/ctEcPhAMNoQAAAAC/i-miss-you.gif",
        "https://c.tenor.com/RGqF8XGZJfkAAAAC/peach-goma.gif",
        "https://c.tenor.com/zIUIJgtOnpEAAAAC/bubba-love.gif",
        "https://c.tenor.com/RLApYkDEeMMAAAAC/kiss-goodnight.gif",
        "https://c.tenor.com/ZGkluEMgEiMAAAAC/good-night-komal.gif",
        "https://c.tenor.com/fNNVXDHYmrgAAAAC/snuggle-cute.gif",
        "https://c.tenor.com/EYtKgyz0to0AAAAC/night-crypto.gif",
        "https://c.tenor.com/AyMq-80B-ZUAAAAC/boa-noite.gif",
        "https://c.tenor.com/8lphpsGT5vMAAAAC/sleep.gif",
        "https://c.tenor.com/Cspt4CCdhyEAAAAC/mi-amor.gif",
        "https://c.tenor.com/Fycn8nr-2AEAAAAd/goodnight-good.gif",
        "https://c.tenor.com/Nf0owRYJg68AAAAC/penguin-love.gif",
        "https://c.tenor.com/NyJnFmEXi9cAAAAC/peach-goma.gif",
        "https://c.tenor.com/o_qXrxHPmnUAAAAC/good-night-bedtime.gif",
        "https://c.tenor.com/Z1FrbmloHeAAAAAC/kristigocouple.gif",
    ];

    return gifs.choose(&mut rand::thread_rng()).unwrap().to_string();
}
