use rand::Rng;

const BRAINROT: &[&str] = &[
    "ඞyou are gඞy",
    "𝖜𝖔𝖒𝖕 𝖜𝖔𝖒𝖕",
    "𝗖𝗘𝗢 𝗢𝗙 𝗢𝗛𝗜𝗢",
    "Skibidisigm🐺🥶",
    "🍆🍑😩👉👌💦",
    "Ɑ͞ ̶͞ ̶͞ ̶͞ لں͞",
    "😫FASTER💦𓂺",
    "𝓱𝓮𝔂 𝓶𝓸𝓶𝓶𝔂😫",
    "𝕾𝖐𝖎𝖇𝖎𝖉𝖎 𝕿𝖔𝖎𝖑𝖊𝖙",
    "🛩🏢🏢 9/11",
    "🥛 not milk",
    "Hawk Tuah and spit on that thang",
    "tung tung tung sahur"
];

pub fn random() -> &'static str {
    let brainrot_len = BRAINROT.len();
    let x = rand::rng().random_range(0..brainrot_len);
    BRAINROT[x]
}

