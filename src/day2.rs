use crate::day2::GameResult::{DRAW, LOSS, WIN};
use crate::day2::Play::{PAPER, ROCK, SCISSORS};

#[derive(Clone, Copy, PartialEq)]
enum Play {ROCK, PAPER, SCISSORS}
#[derive(Clone, Copy)]
enum GameResult {LOSS, WIN, DRAW}

fn get_score(lines: &str, game_score_function: fn(&str) -> Option<i32>) -> i32 {
    return lines.lines()
        .map(|game_string| game_score_function(game_string).unwrap_or(0))
        .sum();
}

fn get_game_score_1(game_string: &str) -> Option<i32>{
    let mut split = game_string.split(" ");
    let opponent_play = map_to_play(split.next()?)?;
    let your_play = map_to_play(split.next()?)?;

    let (_, _, result) = complete((Some(your_play), Some(opponent_play), None))?;

    return Some(get_score_for_play(your_play) + get_score_for_result(result))
}

fn get_game_score_2(game_string: &str) -> Option<i32>{
    let mut split = game_string.split(" ");
    let opponent_play = map_to_play(split.next()?)?;
    let your_result = map_to_result(split.next()?)?;

    let (your_play, _, _) = complete((None, Some(opponent_play), Some(your_result)))?;

    return Some(get_score_for_play(your_play) + get_score_for_result(your_result))
}

fn map_to_play(play_string: &str) -> Option<Play>{
    return match play_string {
        "A" | "X" => Some(ROCK),
        "B" | "Y" => Some(PAPER),
        "C" | "Z" => Some(SCISSORS),
        _ => None
    }
}

fn map_to_result(result_string: &str) -> Option<GameResult>{
    return match result_string {
        "A" | "X" => Some(LOSS),
        "B" | "Y" => Some(DRAW),
        "C" | "Z" => Some(WIN),
        _ => None
    }
}

fn get_score_for_play(play: Play) -> i32{
    return match play {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    }
}

fn get_score_for_result(game_result: GameResult) -> i32{
    return match game_result {
        LOSS => 0,
        WIN => 6,
        DRAW => 3,
    }
}

fn complete(game: (Option<Play>, Option<Play>, Option<GameResult>)) -> Option<(Play, Play, GameResult)>{
    return match game {
        (None, None, _) | (None, _, None) | (_, None, None) => None,
        (Some(ROCK), Some(SCISSORS), _) | (Some(ROCK), _, Some(WIN)) | (_, Some(SCISSORS), Some(WIN)) => Some((ROCK, SCISSORS, WIN)),
        (Some(SCISSORS), Some(PAPER), _) | (Some(SCISSORS), _, Some(WIN)) | (_, Some(PAPER), Some(WIN)) => Some((SCISSORS, PAPER, WIN)),
        (Some(PAPER), Some(ROCK), _) | (Some(PAPER), _, Some(WIN)) | (_, Some(ROCK), Some(WIN)) => Some((PAPER, ROCK, WIN)),
        (Some(play1), Some(play2), None) if play1 == play2 => Some((play1, play1, DRAW)),
        (Some(play1), None, Some(DRAW)) | (None, Some(play1), Some(DRAW))=> Some((play1, play1, DRAW)),
        (play1, play2, _) => {
            let (reversed_play1, reversed_play2, _) = complete((play2, play1, Some(WIN)))?;
            return Some((reversed_play2, reversed_play1, LOSS))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test_1() {
        let game_string = "A Y
B X
C Z";
        assert_eq!(get_score(game_string, get_game_score_1), 15);
    }

    #[test]
    fn simple_test_2() {
        let game_string = "A Y
B X
C Z";
        assert_eq!(get_score(game_string, get_game_score_2), 12);
    }
}