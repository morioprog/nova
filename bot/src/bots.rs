mod dfs_bot;
mod random_bot;

pub use dfs_bot::DfsBot;
use eval::NORMAL_EVALUATOR;
pub use random_bot::RandomBot;

use crate::Bot;

pub fn get_bot(bot_name: &str) -> Box<dyn Bot> {
    match bot_name {
        "RandomBot" => Box::new(RandomBot {}),
        "DfsBot" => Box::new(DfsBot {
            evaluator: *NORMAL_EVALUATOR,
        }),
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

    #[test]
    fn get_dfs_bot() {
        let bot_name = "DfsBot";

        let bot = get_bot(bot_name);

        assert_eq!(bot.name(), bot_name);
    }
}
