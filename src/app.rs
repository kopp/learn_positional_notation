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
            <p>{ number1.to_string() }</p>
            <p>{ number2.to_string() }</p>
        </main>
    }
}
