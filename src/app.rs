use yew::prelude::*;
use rand::seq::IteratorRandom;

#[function_component(App)]
pub fn app() -> Html {
    let mut rng = rand::thread_rng();
    // ensure that we do not get the same digit twice, e.g. 77
    let digits = (0..10).choose_multiple(&mut rng, 2);
    let number1 = 10 * digits[0] + digits[1];
    let number2 = 10 * digits[1] + digits[0];

    html! {
        <main>
            <h1>{ "Welche Zahl ist größer?" }</h1>
            <div style="display: flex; justify-content: center; align-items: center; width: 80%; margin: 0 auto;">
                <button style="font-size: 2em; width: 40%; margin: 0 10px;">{ number1 }</button>
                <button style="font-size: 2em; width: 40%; margin: 0 10px;">{ number2 }</button>
            </div>
            <div style="margin-top: 1em;">
                <button style="font-size: 2em; width: 80%;">{ "Zeige als Bilder" }</button>
            </div>
        </main>
    }
}
