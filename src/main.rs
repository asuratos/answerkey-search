use core::fmt;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{stdin, BufWriter, Read, Write};

use itertools::Itertools;

#[derive(PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Debug)]
enum Answer {
    A,
    B,
    C,
    D,
    X,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::A => write!(f, "A"),
            Answer::B => write!(f, "B"),
            Answer::C => write!(f, "C"),
            Answer::D => write!(f, "D"),
            Answer::X => write!(f, "X"),
        }
    }
}

impl From<char> for Answer {
    fn from(value: char) -> Answer {
        match value {
            'A' => Answer::A,
            'B' => Answer::B,
            'C' => Answer::C,
            'D' => Answer::D,
            'X' => Answer::X,
            _ => panic!("Invalid letter: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct QuizAttempt {
    answers: Vec<Answer>,
    score: i32,
}

impl PartialOrd for QuizAttempt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for QuizAttempt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
struct AnswerKey {
    answers: Vec<Answer>,
}

impl AnswerKey {
    fn as_string(&self) -> String {
        self.answers.iter().map(|ans| ans.to_string()).collect()
    }
}

struct AnswerKeySet {
    keys: Vec<AnswerKey>,
}

impl From<Vec<Answer>> for AnswerKey {
    fn from(value: Vec<Answer>) -> AnswerKey {
        AnswerKey { answers: value }
    }
}

impl From<Vec<AnswerKey>> for AnswerKeySet {
    fn from(value: Vec<AnswerKey>) -> AnswerKeySet {
        AnswerKeySet { keys: value }
    }
}

impl QuizAttempt {
    fn from_string(string: &str, score: i32) -> QuizAttempt {
        if string.len() < score as usize {
            panic!(
                "Impossible score: {} with test length {}",
                score,
                string.len()
            )
        }
        QuizAttempt {
            answers: string
                .to_uppercase()
                .chars()
                .map(|c| Answer::from(c))
                .collect(),
            score: score,
        }
    }

    fn from_list(list: &[&str]) -> QuizAttempt {
        QuizAttempt::from_string(
            list[0],
            list[1]
                .parse::<i32>()
                .expect(&format!("Score is not a number! {}", list[1])),
        )
    }

    fn check(&self, key: &AnswerKey) -> bool {
        if self.answers.len() != key.answers.len() {
            panic!("Unmatched lengths!");
        }
        self.answers
            .iter()
            .zip(&key.answers)
            .map(|(&x, &y)| if x == y { 1 } else { 0 })
            .sum::<i32>()
            == self.score
    }

    fn generate_valid_set(&self) -> AnswerKeySet {
        let num_mistakes = self.answers.len() - self.score as usize;

        let mut answers_to_try: Vec<Vec<&Answer>> = [Answer::A, Answer::B, Answer::C, Answer::D]
            .iter()
            .combinations_with_replacement(num_mistakes)
            .flat_map(|comb| comb.into_iter().permutations(num_mistakes))
            .collect();
        answers_to_try.sort();
        answers_to_try.dedup();

        AnswerKeySet::from({
            let mut small_set = (0..self.answers.len())
                .combinations(num_mistakes)
                .flat_map(|possible_mistakes| {
                    let mut small_key = vec![];
                    // generate 2^n solutions
                    for ans in &answers_to_try {
                        let mut this_key = self.answers.clone();

                        for (&add, &a) in possible_mistakes.clone().iter().zip(ans) {
                            this_key[add] = a.clone();
                        }

                        small_key.push(AnswerKey::from(this_key));
                    }

                    small_key
                })
                .filter(|key| !key.answers.contains(&Answer::X))
                .collect::<Vec<AnswerKey>>();
            small_set.sort();
            small_set.dedup();
            small_set
        })
    }
}

impl AnswerKeySet {
    fn reduce(mut self, attempt: &QuizAttempt) -> AnswerKeySet {
        self.keys = self
            .keys
            .iter()
            .filter(|k| attempt.check(k))
            .cloned()
            .collect();

        self
    }

    fn save_to_file(&self, filename: &str) {
        let mut current_path = env::current_exe().expect("Could not get current path");
        current_path.pop();
        current_path.push(filename);

        let f = File::create(current_path).expect("Could not create output file!");
        let mut f = BufWriter::new(f);

        self.keys.iter().for_each(|key| {
            writeln!(f, "{}", key.as_string()).expect("Could not write to file!");
        });
    }
}

fn extract_attempts_from_file(filename: &str) -> Vec<QuizAttempt> {
    let mut current_path = env::current_exe().expect("Could not get current path");
    current_path.pop();
    current_path.push(filename);

    let mut loaded_attempts: Vec<QuizAttempt> = fs::read_to_string(current_path)
        .expect(&format!("Could not read file {}", filename))
        .lines()
        .map(|str| QuizAttempt::from_list(&str.split(",").collect::<Vec<&str>>()))
        .collect();

    // sanity checks
    // quiz attempts must all have the same length
    let lens: Vec<usize> = loaded_attempts
        .iter()
        .map(|att| att.answers.len())
        .collect();

    if &lens.iter().min() != &lens.iter().max() {
        panic!("The lengths of the answers are not all the same!")
    }

    // sort by score
    loaded_attempts.sort();
    loaded_attempts.reverse();

    loaded_attempts
}

fn main() {
    println!("Reading attempts from file: attempts.txt...");
    let base = extract_attempts_from_file("attempts.txt");

    println!(
        "Loaded {} answers of length {}",
        base.len(),
        base[0].answers.len()
    );

    println!("Searching for possible answers (This could take a while)...");

    // TODO: This should probably be implemented in AnswerKeySet
    let highest = base[0].generate_valid_set();

    let answerset = &base[1..base.len()]
        .iter()
        .fold(highest, |ans_set, att| ans_set.reduce(att));

    println!(
        "Found {} possible solutions! Writing to possible_answers.txt...",
        answerset.keys.len()
    );

    answerset.save_to_file("possible_answers.txt");

    println!("Press any key to end...");
    stdin().read(&mut [0]).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{Answer, AnswerKey, QuizAttempt};
    use itertools::Itertools;

    #[test]
    fn answer_from_str() {
        assert_eq!(Answer::from('A'), Answer::A);
        assert_eq!(Answer::from('B'), Answer::B);
        assert_eq!(Answer::from('C'), Answer::C);
        assert_eq!(Answer::from('D'), Answer::D);
        assert_eq!(Answer::from('X'), Answer::X);
    }

    #[test]
    #[should_panic]
    fn invalid_answer() {
        Answer::from('E');
    }

    #[test]
    fn crosscheck_works_for_valid() {
        let key = AnswerKey {
            answers: vec![Answer::A, Answer::B],
        };
        let att = QuizAttempt {
            score: 1,
            answers: vec![Answer::A, Answer::A],
        };
        assert_eq!(att.check(&key), true)
    }

    #[test]
    fn crosscheck_works_for_invalid() {
        let key = AnswerKey {
            answers: vec![Answer::A, Answer::B],
        };
        let att = QuizAttempt {
            score: 2,
            answers: vec![Answer::A, Answer::A],
        };
        att.check(&key);
    }

    #[test]
    #[should_panic]
    fn crosscheck_works_for_invalid_length() {
        let key = AnswerKey {
            answers: vec![Answer::A, Answer::B],
        };
        let att = QuizAttempt {
            score: 2,
            answers: vec![Answer::A],
        };
        assert_eq!(att.check(&key), false)
    }

    #[test]
    fn permutations_checking() {
        let att = QuizAttempt {
            score: 1,
            answers: vec![Answer::A, Answer::B],
        };
        let mut valid_keys: Vec<AnswerKey> = vec![];

        for key in [Answer::A, Answer::B, Answer::C, Answer::D]
            .iter()
            .permutations(2)
        {
            let akey = AnswerKey::from(key.into_iter().copied().collect::<Vec<Answer>>());
            if att.check(&akey) {
                valid_keys.push(akey)
            }
        }

        assert_eq!(valid_keys.len(), 4);

        let att2 = QuizAttempt {
            score: 2,
            answers: vec![Answer::A, Answer::B],
        };
        let mut valid_keys2: Vec<AnswerKey> = vec![];
        for key2 in [Answer::A, Answer::B, Answer::C, Answer::D]
            .iter()
            .permutations(2)
        {
            let akey = AnswerKey::from(key2.into_iter().copied().collect::<Vec<Answer>>());
            if att2.check(&akey) {
                valid_keys2.push(akey)
            }
        }
        assert_eq!(valid_keys2.len(), 1);
    }
}
