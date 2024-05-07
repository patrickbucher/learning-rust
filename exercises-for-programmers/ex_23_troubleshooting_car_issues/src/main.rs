use akshually::io::prompt_line;

enum Step {
    Branch {
        question: String,
        yes_step: Box<Step>,
        no_step: Box<Step>,
    },
    Leaf {
        answer: String,
    },
}

fn main() {
    let troubleshooting: Step = Step::Branch {
        question: "Is the car silent when you turn the key?".into(),
        yes_step: Box::new(Step::Branch {
            question: "Are the battery terminals corroded?".into(),
            yes_step: Box::new(Step::Leaf {
                answer: "Clean terminals and try starting again.".into(),
            }),
            no_step: Box::new(Step::Leaf {
                answer: "Replace cables and try again.".into(),
            }),
        }),
        no_step: Box::new(Step::Branch {
            question: "Does the car make a clicking noise?".into(),
            yes_step: Box::new(Step::Leaf {
                answer: "Replace the battery.".into(),
            }),
            no_step: Box::new(Step::Branch {
                question: "Does the car crank up but fail to start?".into(),
                yes_step: Box::new(Step::Leaf {
                    answer: "Check spark plug connections.".into(),
                }),
                no_step: Box::new(Step::Branch {
                    question: "Does the engine start and then die?".into(),
                    yes_step: Box::new(Step::Branch {
                        question: "Does your car have fuel injection?".into(),
                        yes_step: Box::new(Step::Leaf {
                            answer: "Get it in for service.".into(),
                        }),
                        no_step: Box::new(Step::Leaf {
                            answer: "Check to ensure the choke is opening and closing".into(),
                        }),
                    }),
                    no_step: Box::new(Step::Leaf {
                        answer: "Diehl with it…".into(), // NOTE: not given in exercise
                    }),
                }),
            }),
        }),
    };
    process(troubleshooting);
}

fn process(step: Step) {
    match step {
        Step::Branch {
            question,
            yes_step,
            no_step,
        } => {
            let input: char = prompt_line(&format!("{question} ")).expect("enter 'y' or 'n'");
            if input == 'y' {
                process(*yes_step);
            } else {
                process(*no_step);
            }
        }
        Step::Leaf { answer } => {
            println!("{answer}");
        }
    }
}
