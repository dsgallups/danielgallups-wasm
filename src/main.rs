use yew::prelude::*;
mod canvas;
mod space;
use space::Space;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <div>{"This is a test"}</div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
