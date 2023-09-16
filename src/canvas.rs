use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::{html, Component, Context, Html, NodeRef};

use gloo_console::log;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGlRenderingContext};

use crate::mandelbrot::{Mandelbrot, MandelbrotBuilder, ShadingType};

enum Msg {
    Zoom,
    ScrollX,
    ScrollY,
    Drag,
}

pub struct Canvas {
    node_ref: NodeRef,
    width: u32,
    height: u32,
    mandelbrot: Mandelbrot,
}

impl Component for Canvas {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mandelbrot = MandelbrotBuilder::new()
            .height(300)
            .width(300)
            .max_iterations(12)
            .shading_type(ShadingType::OpacityAndColor)
            .opacity(255)
            .zoom(1.0)
            .x_offset(0.0)
            .y_offset(0.0)
            .build();
        Self {
            node_ref: NodeRef::default(),
            width: 300,
            height: 300,
            mandelbrot,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("test");
        html! {
            <canvas ref={self.node_ref.clone()} width={300} height={300} ></canvas>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        //TODO: This is where we get and then set the viewport height and width for the canvas.

        log!("in rendered");
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration
        if !first_render {
            return;
        }
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        //Generate mandelbrot here
        //let res = self.mandelbrot.draw();

        //let img = web_sys::ImageData::new_with_u8_clamped_array_and_sh(data, sw, sh);

        Self::render_canvas(gl);
    }
}

impl Canvas {
    fn render_canvas(gl: CanvasRenderingContext2d) {}
}
