use rand::seq::SliceRandom; // 0.7.2

pub fn get_thumbnails() -> String {
    let thumbnails = vec![
        "https://c.tenor.com/0PP9WiddZWcAAAAC/anime-sleepy.gif",
        "https://thumbs.gfycat.com/FearlessAnyAlaskanmalamute-size_restricted.gif",
        "https://c.tenor.com/Bx_eD4jd8jQAAAAC/anime-sleepy.gif",
        "https://c.tenor.com/re9a71mA5xwAAAAC/nogamenolife-shiro.gif",
    ];

    return thumbnails.choose(&mut rand::thread_rng()).unwrap().to_string();
}
