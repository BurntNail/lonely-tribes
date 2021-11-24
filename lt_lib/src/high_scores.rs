use crate::{
    config::LTConfig,
    paths::get_directory,
    states_util::{get_levels, levels_len, LevelType},
};
use ron::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir, read_to_string, write},
    path::PathBuf,
};

///Struct to score High Scores
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighScores {
    ///HashMap of high scores, with the key being the level index, and the i32 being the number of moves
    pub scores: Vec<i32>,
}
impl Default for HighScores {
    fn default() -> Self {
        Self {
            scores: Vec::with_capacity(
                get_levels()
                    .into_iter()
                    .filter(|(_, is_normal)| is_normal == &LevelType::Developer)
                    .count(),
            ),
        }
    }
}

fn high_scores_path() -> PathBuf {
    get_directory(false).join("high_scores.ron")
}

impl HighScores {
    pub fn new() -> Self {
        let file = read_to_string(high_scores_path()).unwrap_or_else(|_| "".to_string());

        let mut scores_from_file: Vec<i32> = from_str(file.as_str()).unwrap_or_default();
        let mut scores = Vec::new();
        scores.append(&mut scores_from_file);

        Self { scores }
    }

    ///Function to add a score, check whether it is better than the written down one, and if so write it to a file
    ///
    /// Returns an option
    /// If it is None, then the high score was beaten
    /// If Some, then the i32 is the old high score
    pub fn add_score_and_write(&mut self, path: String, score: i32) -> Option<i32> {
        let replaced = path.replace("lvl-", "").replace(".ron", "");
        let index = replaced.parse::<usize>().unwrap_or_else(|err| {
            log::warn!("Unable to parse {} into usize: {}", replaced, err);
            usize::MAX
        }) - 1;
        if index == usize::MAX {
            return None;
        }

        let mut new_high_score = false;
        let current = {
            if self.scores.len() > index {
                //to avoid panicking on overflow
                self.scores.remove(index)
            } else {
                i32::MAX
            }
        };

        let mut insert = |el: i32| {
            if index >= self.scores.len() {
                self.scores.push(el);
            } else {
                self.scores.insert(index, el);
            }
        };

        if score < current {
            new_high_score = true;
            insert(score);

            self.write_self_to_file();
        } else {
            insert(current);
        }

        if new_high_score {
            None
        } else {
            Some(current)
        }
    }

    ///Function to check whether or not a level has been beaten yet
    ///
    /// Simple function, but better for reading code later
    pub fn get_high_score(&self, level: usize) -> Option<i32> {
        self.scores.get(level).copied()
    }

    ///Function to find what the next level to be played is
    pub fn find_next_level(&self) -> usize {
        if LTConfig::new().flags.debug {
            return levels_len();
        }

        let mut i = 0;
        let mut an_unfinished_level_exists = false;
        for level in 0..levels_len() {
            if self.get_high_score(level).is_some() {
                i = level + 1;
            } else {
                an_unfinished_level_exists = true;
            }
        }
        if an_unfinished_level_exists {
            i
        } else {
            i - 1
        }
    }

    ///Function to serialise the scores to a file
    fn write_self_to_file(&self) {
        let text = to_string(&self.scores);
        if let Ok(text) = text {
            write(high_scores_path(), &text).unwrap_or_else(|_| {
                create_dir(get_directory(false))
                    .unwrap_or_else(|err| log::error!("Unable to create data directory: {}", err));
                write(high_scores_path(), &text)
                    .unwrap_or_else(|err| log::error!("Unable to write high scores: {}", err));
            });
        }
    }
}
