use yew::prelude::*;
use rand::seq::IteratorRandom;


fn number_as_svg(value: i32, color: String) -> Html {

    let grid_size = 4;

    let circles = (0..value).map(|val| {
        let i = val % 10;
        let j = (val - i) / 10;
        let x = grid_size * i + grid_size / 2;
        let y = grid_size * j + grid_size / 2;
        html! {
            <circle cx={ y.to_string() } cy={ x.to_string() } r={ ((grid_size as f64) / 2.3).floor().to_string() } fill={ color.clone() } />
        }
    }).collect::<Html>();

    html! {
        <svg viewBox={ format!("0 0 {} {}", 11 * grid_size, 11 * grid_size) } xmlns="http://www.w3.org/2000/svg">
        { circles }
        </svg>
    }
}


#[derive(Clone, PartialEq)]
struct NumbersToCompare {
    number1: i32,
    number2: i32,
}

impl NumbersToCompare {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        // ensure that we do not get the same digit twice, e.g. 77
        let digits = (0..10).choose_multiple(&mut rng, 2);
        let number1 = 10 * digits[0] + digits[1];
        let number2 = 10 * digits[1] + digits[0];
        NumbersToCompare { number1, number2 }
    }
}


#[derive(Properties, PartialEq)]
struct TwoNumbersProps {
    numbers: NumbersToCompare,
    on_update: Callback<QuestionState>,
}

#[function_component(TwoNumbers)]
fn two_numbers(TwoNumbersProps { numbers, on_update }: &TwoNumbersProps) -> Html {
    let show_numbers_as_images = use_state(|| false);

    let do_show_numbers_as_images = {
        let show_numbers_as_images = show_numbers_as_images.clone();
        move |_| {
            show_numbers_as_images.set(true);
        }
    };

    let on_click_number1 = {
        let on_update = on_update.clone();
        let numbers = numbers.clone();
        let show_numbers_as_images = show_numbers_as_images.clone();
        move |_| {
            if numbers.number1 > numbers.number2 {
                on_update.emit(QuestionState::DisplayResult(Result::Correct));
            } else {
                if *show_numbers_as_images {
                    on_update.emit(QuestionState::DisplayResult(Result::IncorrectWithHelp));
                } else {
                    on_update.emit(QuestionState::DisplayResult(Result::IncorrectNoHelp));
                }
            }
        }
    };

    let on_click_number2 = {
        let on_update = on_update.clone();
        let numbers = numbers.clone();
        let show_numbers_as_images = show_numbers_as_images.clone();
        move |_| {
            if numbers.number2 > numbers.number1 {
                on_update.emit(QuestionState::DisplayResult(Result::Correct));
            } else {
                if *show_numbers_as_images {
                    on_update.emit(QuestionState::DisplayResult(Result::IncorrectWithHelp));
                } else {
                    on_update.emit(QuestionState::DisplayResult(Result::IncorrectNoHelp));
                }

            }
        }
    };

    html! {
        <>
            <h1>{ "Welche Zahl ist größer?" }</h1>
            <div style="display: flex; justify-content: center; align-items: center; width: 80%; margin: 0 auto;">
                <button onclick={on_click_number1} style="font-size: 2em; width: 40%; margin: 0 10px; background-color: yellow;">{ numbers.number1 }</button>
                <button onclick={on_click_number2} style="font-size: 2em; width: 40%; margin: 0 10px; background-color: green;">{ numbers.number2 }</button>
            </div>
            {
                if *show_numbers_as_images {
                    html! {
                        <div style="display: flex; justify-content: center; align-items: center; width: 80%; margin: 0 auto;">
                            { number_as_svg(numbers.number1, "yellow".to_string() ) }
                            { number_as_svg(numbers.number2, "green".to_string() ) }
                        </div>
                    }
                }
                else {
                    html! {
                        <div style="margin-top: 1em;">
                            <button onclick={do_show_numbers_as_images} style="font-size: 2em; width: 80%;">{ "Zeige als Bilder" }</button>
                        </div>
                    }
                }
            }
        </>
    }

}


#[derive(Copy, Clone, PartialEq)]
enum Result {
    Correct,
    IncorrectNoHelp,
    IncorrectWithHelp,
}

#[derive(Copy, Clone, PartialEq)]
enum QuestionState {
    AskUser,
    DisplayResult(Result),
}


fn format_correct_answer(numbers: &NumbersToCompare) -> String {
    let min = numbers.number1.min(numbers.number2);
    let max = numbers.number1.max(numbers.number2);
    format!("{} ist größer als {}", max, min)
}


#[function_component(App)]
pub fn app() -> Html {
    let numbers_to_compare = use_state(|| NumbersToCompare::new());

    let history = use_state(|| Vec::<Result>::new());

    let question_state = use_state(|| QuestionState::AskUser);

    let update_question_state = {
        let question_state = question_state.clone();
        Callback::from(move |new_state: QuestionState| {
            question_state.set(new_state);
        })
    };

    let ask_next_question = {
        let numbers_to_compare = numbers_to_compare.clone();
        let question_state = question_state.clone();
        let history = history.clone();
        move |_| {
            match (*question_state).clone() {
                QuestionState::DisplayResult(result) => {
                    let mut new_history = (*history).clone();
                    new_history.push(result);
                    history.set(new_history);
                    // history.set((*history).clone().into_iter().chain(std::iter::once(result)).collect());
                }
                _ => {}
            }
            numbers_to_compare.set(NumbersToCompare::new());
            question_state.set(QuestionState::AskUser);
        }
    };

    let next_question_button = html! {
        <button onclick={ask_next_question} style="font-size: 2em; width: 80%;">{ "Nächste Frage" }</button>
    };

    let formatted_history_items = (*history)
        .clone()
        .into_iter()
        .chain(
            match *question_state {
                QuestionState::DisplayResult(result) => Some(result),
                _ => None,
            }
            .into_iter(),
        )
        .enumerate()
        .map(|(i, result)| {
            let text = match result {
                Result::Correct => "👍",
                Result::IncorrectNoHelp | Result::IncorrectWithHelp => "🤔",
            };
            html! {
                <li key={i.to_string()} style="display: inline;">{ text }</li>
            }
        })
        .collect::<Html>();

    let formatted_history = html! {
        <div style="margin-top: 1em;">
            <ul>
                { formatted_history_items }
            </ul>
        </div>
    };

    html! {
        <main>
            {
                match (*question_state).clone() {
                    QuestionState::AskUser => {
                        html! {
                            <TwoNumbers numbers={ (*numbers_to_compare).clone() } on_update={update_question_state.clone()} />
                        }
                    }
                    QuestionState::DisplayResult(result) => {
                        let correct_answer = html! {
                            <p>{ format_correct_answer(&(*numbers_to_compare).clone()) }</p>
                        };

                        match result {
                            Result::Correct => {
                                html! {
                                    <>
                                        <h1>{ "Das stimmt 🥳, weiter so!" }</h1>
                                        { correct_answer }
                                        { next_question_button }
                                        { formatted_history }
                                    </>
                                }
                            }
                            Result::IncorrectNoHelp => {
                                html! {
                                    <>
                                        <h1>{ "Leider nicht richtig." }</h1>
                                        { correct_answer }
                                        <p>{ "Zeige dir nächstes mal die Bilder an, wenn du unsicher bist." }</p>
                                        { next_question_button }
                                        { formatted_history }
                                    </>
                                }
                            }
                            Result::IncorrectWithHelp => {
                                html! {
                                    <>
                                        <h1>{ "Leider nicht richtig!" }</h1>
                                        { correct_answer }
                                        { next_question_button }
                                        { formatted_history }
                                    </>
                                }
                            }
                        }

                    }
                }
            }
        </main>
    }
}
