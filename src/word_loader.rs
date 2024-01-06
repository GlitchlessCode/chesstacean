use crate::server::console::success_msg;
use rand::{seq::SliceRandom, Rng};
use tokio::fs;

pub async fn load() -> WordList {
    // Fetch utility files
    let adjectives = fs::read_to_string("./util/word_lists/adjectives.txt")
        .await
        .expect("There should be a file at ./util/word_lists/adjectives.txt");
    let adj_list = adjectives.split_whitespace().map(|s| s.to_string()).collect();
    success_msg("Adjective list successfully loaded!");

    let nouns = fs::read_to_string("./util/word_lists/nouns.txt")
        .await
        .expect("There should be a file at ./util/word_lists/adjectives.txt");
    let noun_list = nouns.split_whitespace().map(|s| s.to_string()).collect();
    success_msg("Noun list successfully loaded!");

    let verbs = fs::read_to_string("./util/word_lists/verbs.txt")
        .await
        .expect("There should be a file at ./util/word_lists/adjectives.txt");
    let verb_list = verbs.split_whitespace().map(|s| s.to_string()).collect();
    success_msg("Verb list successfully loaded!");

    let word_list = WordList::new(adj_list, noun_list, verb_list);
    success_msg("Word lists successfully assembled!");

    word_list
}

pub struct WordList {
    pub adjectives: Vec<String>,
    pub nouns: Vec<String>,
    pub verbs: Vec<String>,
}

impl WordList {
    fn new(adjectives: Vec<String>, nouns: Vec<String>, verbs: Vec<String>) -> Self {
        Self {
            adjectives,
            nouns,
            verbs,
        }
    }

    pub fn combo<R>(&self, rng: &mut R) -> String
    where
        R: Rng + ?Sized,
    {
        let adjective = self.adjectives.choose(rng).unwrap();
        let noun = self.nouns.choose(rng).unwrap();
        let verb = self.verbs.choose(rng).unwrap();

        adjective.to_owned() + noun + verb
    }
}
