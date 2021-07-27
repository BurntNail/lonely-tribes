use std::collections::HashMap;
use ron::{from_str, to_string};
use std::fs::{read_to_string, write};
use serde::{Deserialize, Serialize};

///The path to the high scores
const HIGH_SCORES_PATH: &str = "assets/data/high_scores.ron";

///Struct to score High Scores
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HighScores {
	///HashMap of high scores, with the key being the level index, and the i32 being the number of moves
	pub scores: HashMap<usize, i32>
}
impl Default for HighScores {
	fn default() -> Self {
		let file = read_to_string(HIGH_SCORES_PATH).unwrap_or("".to_string());
		let scores: HashMap<usize, i32> = from_str(file.as_str()).unwrap_or_else(|err| {
			log::error!("Error deserialising {}", err);
			HashMap::new()
		});

        Self {
			scores
		}
    }
}
impl HighScores {
	///Function to add a score, check whether it is better than the written down one, and if so write it to a file
	pub fn add_score_and_write (&mut self, index: usize, score: i32) -> bool {
		let mut new_high_score = false;
		let current = self.scores.remove(&index).unwrap_or(i32::MAX);
		if score < current {
			new_high_score = true;
			self.scores.insert(index, score);

			self.write_self_to_file();
		} else {
			self.scores.insert(index, current);
		}


		new_high_score
	}

	///Function to serialise the scores to a file
	fn write_self_to_file (&self) {
		let text = to_string(&self.scores);
		if let Ok(text) = text {
			write(HIGH_SCORES_PATH, text).unwrap_or_else(|err| log::error!("Unable to write high score down - {:?}", err) );
		}
	}
}