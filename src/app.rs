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
}


#[function_component(TwoNumbers)]
fn two_numbers(TwoNumbersProps { numbers, }: &TwoNumbersProps) -> Html {
    let show_numbers_as_images = use_state(|| false);

    let do_show_numbers_as_images = {
        let show_numbers_as_images = show_numbers_as_images.clone();
        move |_| {
            show_numbers_as_images.set(true);
        }
    };

    html! {
        <>
            <div style="display: flex; justify-content: center; align-items: center; width: 80%; margin: 0 auto;">
                <button style="font-size: 2em; width: 40%; margin: 0 10px; background-color: yellow;">{ numbers.number1 }</button>
                <button style="font-size: 2em; width: 40%; margin: 0 10px; background-color: blue;">{ numbers.number2 }</button>
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



#[function_component(App)]
pub fn app() -> Html {
    let numbers_to_compare = use_state(|| NumbersToCompare::new());

    html! {
        <main>
            <h1>{ "Welche Zahl ist größer?" }</h1>
            <TwoNumbers numbers={ (*numbers_to_compare).clone() } />
        </main>
    }
}
