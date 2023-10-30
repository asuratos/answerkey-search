use itertools::Itertools;
use std::io::{stdin, Read};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Answer {
    A,
    B,
    C,
    D,
}

impl From<char> for Answer {
    fn from(value: char) -> Answer {
        match value {
            'A' => Answer::A,
            'B' => Answer::B,
            'C' => Answer::C,
            'D' => Answer::D,
            _ => panic!("Invalid letter"),
        }
    }
}

#[derive(Debug)]
struct QuizAttempt {
    answers: Vec<Answer>,
    score: i32,
}

#[derive(Debug, Clone)]
struct AnswerKey {
    answers: Vec<Answer>,
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
        // TODO: Panic with score > len
        QuizAttempt {
            answers: string
                .to_uppercase()
                .chars()
                .map(|c| Answer::from(c))
                .collect(),
            score: score,
        }
    }

    fn check(&self, key: &AnswerKey) -> bool {
        if self.answers.len() != key.answers.len() {
            panic!("Unmatched lengths!");
        }
        self.answers
            .iter()
            .zip(&key.answers)
            .map(|(&x, &y)| x == y)
            .map(|r| if r { 1 } else { 0 })
            .sum::<i32>()
            == self.score
    }

    fn generate_valid_set(&self) -> AnswerKeySet {
        // let mut valid_keys: Vec<AnswerKey> = vec![];
        let num_mistakes = self.answers.len() - self.score as usize;

        let mut answers_to_try: Vec<Vec<&Answer>> = [Answer::A, Answer::B, Answer::C, Answer::D]
            .iter()
            .combinations_with_replacement(num_mistakes)
            .flat_map(|comb| comb.into_iter().permutations(num_mistakes))
            .collect();
        answers_to_try.dedup();

        AnswerKeySet::from(
            (0..self.answers.len())
                .combinations(num_mistakes)
                .flat_map(|possible_mistakes| {
                    // println!("{:?}", possible_mistakes);
                    let mut small_key = vec![];
                    // generate 2^n solutions
                    for ans in &answers_to_try {
                        let mut this_key = self.answers.clone();

                        for (&add, &a) in possible_mistakes.clone().iter().zip(ans) {
                            this_key[add] = a.clone();
                        }

                        // println!("{:?}", this_key);
                        small_key.push(AnswerKey::from(this_key));
                    }

                    small_key
                })
                .collect::<Vec<AnswerKey>>(),
        )
    }
}

impl AnswerKeySet {
    fn reduce(mut self, attempt: QuizAttempt) -> AnswerKeySet {
        self.keys = self
            .keys
            .iter()
            .filter(|k| attempt.check(k))
            .cloned()
            .collect();
        self
    }
}

fn main() {
    // TODO: Parse from txt

    // TODO: Sort from highest to lowest score

    let attempt = QuizAttempt::from_string("BCABBDBCAD", 5);

    // TODO: fold(attmpt1, |acc, x| acc.reduce(x))

    let mut answerset = attempt.generate_valid_set();
    answerset = answerset.reduce(QuizAttempt::from_string("CCBBCBABAD", 6));
    answerset = answerset.reduce(QuizAttempt::from_string("BCDBACABAD", 6));
    answerset = answerset.reduce(QuizAttempt::from_string("BCCBCAADAD", 6));
    answerset = answerset.reduce(QuizAttempt::from_string("BCDBCBACAA", 6));
    answerset = answerset.reduce(QuizAttempt::from_string("BCDBBDBBBD", 5));
    answerset = answerset.reduce(QuizAttempt::from_string("CCBBBAACBD", 4));
    answerset = answerset.reduce(QuizAttempt::from_string("CCBBACBBAB", 3));

    println!("{}", answerset.keys.len());

    for ans in answerset.keys {
        println!("{:?}", ans.answers);
    }

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
    }

    #[test]
    #[should_panic]
    fn invalid_answer() {
        Answer::from('X');
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
