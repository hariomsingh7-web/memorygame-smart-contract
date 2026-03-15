#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Score {
    pub player: Address,
    pub score: u32,
}

#[contract]
pub struct MemoryGame;

#[contractimpl]
impl MemoryGame {

    // Save a player's score
    pub fn submit_score(env: Env, player: Address, score: u32) {

        player.require_auth();

        let mut scores: Vec<Score> = env
            .storage()
            .instance()
            .get(&Symbol::short("SCORES"))
            .unwrap_or(Vec::new(&env));

        let new_score = Score {
            player,
            score,
        };

        scores.push_back(new_score);

        env.storage().instance().set(&Symbol::short("SCORES"), &scores);
    }

    // Get all scores (leaderboard)
    pub fn get_scores(env: Env) -> Vec<Score> {
        env.storage()
            .instance()
            .get(&Symbol::short("SCORES"))
            .unwrap_or(Vec::new(&env))
    }

    // Get best score
    pub fn best_score(env: Env) -> u32 {

        let scores: Vec<Score> = env
            .storage()
            .instance()
            .get(&Symbol::short("SCORES"))
            .unwrap_or(Vec::new(&env));

        let mut best: u32 = 0;

        for score in scores.iter() {
            if score.score > best {
                best = score.score;
            }
        }

        best
    }
}