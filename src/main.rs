use itertools::Itertools;

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
        let mut valid_keys: Vec<AnswerKey> = vec![];

        for key in [Answer::A, Answer::B, Answer::C, Answer::D]
            .iter()
            .combinations_with_replacement(self.answers.len())
        {
            for p in key.iter().copied().permutations(self.answers.len()) {
                // for x in key.iter().permutations(15) {

                //     println!("{:?}", x);
                // }
                let akey = AnswerKey::from(p.into_iter().copied().collect::<Vec<Answer>>());
                // println!("{:?}", akey);
                if self.check(&akey) {
                    valid_keys.push(akey)
                }
            }
        }
        AnswerKeySet::from(valid_keys)
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
    let attempt = QuizAttempt::from_string("DCBCCADADBDDBBD", 13);
    let mut answerset = attempt.generate_valid_set();

    // answerset = answerset.reduce(QuizAttempt::from_string("CCBCAADADADDBBA", 11));
    // answerset = answerset.reduce(QuizAttempt::from_string("CBBCBADADADDBBB", 11));
    // answerset = answerset.reduce(QuizAttempt::from_string("CBBCBADADADDBAD", 11));
    // answerset = answerset.reduce(QuizAttempt::from_string("CBBCAADADACDCAA", 7));

    println!("{}", answerset.keys.len());
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
