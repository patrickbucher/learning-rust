use akshually::io::prompt_line;
use std::env;
use std::fs;

#[derive(Debug)]
struct Question {
    text: String,
    answers: Vec<Answer>,
}

#[derive(Debug)]
struct Answer {
    text: String,
    correct: bool,
}

fn main() {
    let mut args = env::args();
    let bin = args.next().unwrap();
    let usage = format!("usage: {bin} questions_file");
    let file = args.next().expect(&usage);
    let questions = parse(&file).unwrap();
    let mut correct = 0;
    for question in questions {
        if ask(question) {
            println!("correct");
            correct += 1;
        } else {
            println!("wrong");
            break;
        }
    }
    println!("You answered {correct} questions correctly.");
}

fn ask(question: Question) -> bool {
    println!("{}", question.text);
    for (i, answer) in question.answers.iter().enumerate() {
        println!("{} {}", i + 1, answer.text);
    }
    let choice: usize = prompt_line("Answer: ").map_or(0, |v| v);
    let correct: Vec<_> = question
        .answers
        .iter()
        .enumerate()
        .filter(|(i, a)| i + 1 == choice && a.correct)
        .collect();
    !correct.is_empty()
}

fn parse(path: &str) -> Result<Vec<Question>, String> {
    let mut questions: Vec<Question> = Vec::new();
    let content = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
    let parts = content.split("\n\n");
    for part in parts {
        let mut lines = part.split("\n");
        let question = lines.next();
        let options = lines.map(|s| {
            if s.starts_with("[x] ") {
                Some(Answer {
                    text: s[4..].into(),
                    correct: true,
                })
            } else if s.starts_with("[ ] ") {
                Some(Answer {
                    text: s[4..].into(),
                    correct: false,
                })
            } else {
                None
            }
        });
        if let Some(text) = question {
            let mut answers: Vec<Answer> = Vec::new();
            for option in options {
                if let Some(answer) = option {
                    answers.push(answer);
                }
            }
            let mut question = Question {
                text: text.into(),
                answers,
            };
            questions.push(question);
        }
    }
    Ok(questions)
}
