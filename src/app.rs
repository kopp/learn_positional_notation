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
                <button onclick={on_click_number2} style="font-size: 2em; width: 40%; margin: 0 10px; background-color: blue;">{ numbers.number2 }</button>
            </div>
            {
                if *show_numbers_as_images {
                    html! {
                        <div style="display: flex; justify-content: center; align-items: center; width: 80%; margin: 0 auto;">
                            { number_as_svg(numbers.number1, "yellow".to_string() ) }
                            { number_as_svg(numbers.number2, "blue".to_string() ) }
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
    /*
    let pick_new_numbers = {
        let numbers_to_compare = numbers_to_compare.clone();
        move |_| {
            numbers_to_compare.set(NumbersToCompare::new());
        }
    };
    */

    let question_state = use_state(|| QuestionState::AskUser);

    let update_question_state = {
        let question_state = question_state.clone();
        Callback::from(move |new_state: QuestionState| {
            question_state.set(new_state);
        })
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
                                        <h1>{ "Richtig!" }</h1>
                                        { correct_answer }
                                    </>
                                }
                            }
                            Result::IncorrectNoHelp => {
                                html! {
                                    <>
                                        <h1>{ "Falsch!" }</h1>
                                        { correct_answer }
                                        <p>{ "Zeige dir nächstes mal die Bilder an, wenn du unsicher bist." }</p>
                                    </>
                                }
                            }
                            Result::IncorrectWithHelp => {
                                html! {
                                    <>
                                        <h1>{ "Falsch!" }</h1>
                                        { correct_answer }
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
