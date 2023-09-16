use yew::{html, Component, Context, Html, NodeRef};

pub struct Canvas {
    node_ref: NodeRef,
}

impl Component for Canvas {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()}></canvas>
        }
    }
}
