use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT_TAB_TITLES: Vec<&'static str> =
        vec!["Chat", "Channel", "Username", "Server"];
    pub static ref HELP_COLUMN_TITLES: Vec<&'static str> = vec!["Keybind", "Description"];
    pub static ref HELP_NORMAL_MODE: Vec<Vec<&'static str>> = vec![
        vec!["c", "Chat window"],
        vec!["i", "Insert mode"],
        vec!["?", "Bring up this window"],
        vec!["q", "Quit this application"],
        vec!["Esc", "Drop back to previous window layer"],
    ];
    pub static ref HELP_INSERT_MODE: Vec<Vec<&'static str>> = vec![
        vec!["Ctrl + f", "Move cursor to the right"],
        vec!["Ctrl + b", "Move cursor to the left"],
        vec!["Ctrl + a", "Move cursor to the start"],
        vec!["Ctrl + e", "Move cursor to the end"],
        vec!["Alt + f", "Move to the end of the next word"],
        vec!["Alt + b", "Move to the start of the previous word"],
        vec!["Ctrl + t", "Swap previous item with current item"],
        vec!["Alt + t", "Swap previous word with current word"],
        vec!["Ctrl + u", "Remove everything before the cursor"],
        vec!["Ctrl + k", "Remove everything after the cursor"],
        vec!["Ctrl + w", "Remove the previous word"],
        vec!["Ctrl + d", "Remove item to the right"],
        vec!["Esc", "Drop back to previous window layer"],
    ];
    pub static ref COMMANDS: Vec<&'static str> = vec![
        "ban",
        "unban",
        "clear",
        "color",
        "commercial",
        "delete",
        "disconnect",
        "emoteonly",
        "emoteonlyoff",
        "followers",
        "followersoff",
        "help",
        "host",
        "unhost",
        "marker",
        "me",
        "mod",
        "unmod",
        "mods",
        "r9kbeta",
        "r9kbetaoff",
        "raid",
        "unraid",
        "slow",
        "slowoff",
        "subscribers",
        "subscribersoff",
        "timeout",
        "untimeout",
        "vip",
        "unvip",
        "vips",
        "w",
    ];
}
