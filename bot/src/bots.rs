mod random_bot;

pub use random_bot::RandomBot;

use crate::Bot;

pub fn get_bot(bot_name: &str) -> impl Bot {
    match bot_name {
        "RandomBot" => RandomBot::new(),
        _ => panic!("Bot with name \"{}\" not found", bot_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_random_bot() {
        let bot_name = "RandomBot";

        let bot = get_bot(bot_name);

        assert_eq!(bot.name(), bot_name);
    }
}
