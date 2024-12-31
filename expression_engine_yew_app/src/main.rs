mod draw_layer;

use draw_layer::{DrawLayer, GraphLayer, LayerDrawing, XAxisLayer, YAxisLayer};
use std::cell::RefCell;
use std::rc::Rc;

use expression_engine::{parse, ExpressionNode, ExpressionNodeType, Operator};
use implicit_clone::unsync::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{
    console, window, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL,
    WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};
use web_sys::{HtmlInputElement, InputEvent};
use yew::prelude::*;

pub enum Msg {
    InputExpression(String),
    InputVariable(String),
    InputNone(String),
    ZoomIn(),
    ZoomOut(),
}

pub struct Graphing {
    node_ref: NodeRef,
    expression_text: IString,
    expression: Option<ExpressionNode>,
    expression2: Option<ExpressionNode>,
    variable_text: IString,
    result: f64,
    width: u32,
    height: u32,
    zoom_level: f64,
    color: Option<WebGlUniformLocation>,
}

impl Graphing {
    pub fn get_exp_str(&self) -> String {
        match parse(&self.expression_text) {
            Ok(exp) => {
                format!("Expression : {}", exp)
            }
            Err(error) => match error.kind {
                expression_engine::ErrorKind::Empty => String::from("Empty expression"),
                expression_engine::ErrorKind::InvalidOpenCloseParantheses => {
                    String::from("Incorrect number of open/close parantheses")
                }
                expression_engine::ErrorKind::InvalidNumberParsed => {
                    String::from("Failed to parse number(s) in expression")
                }
                expression_engine::ErrorKind::InvalidExpression => {
                    String::from("Expression is invalid")
                }
            },
        }
    }
    pub fn get_exp_res(&self) -> String {
        match parse(&self.expression_text) {
            Ok(exp) => {
                let var = self.variable_text.parse::<f64>().unwrap();
                let eval_res = exp.evaluate(&self.variable_text, var);
                match eval_res {
                    Ok(eval) => {
                        format!("Result : {}", eval)
                    }
                    Err(_) => ("".to_string()),
                }
            }
            Err(error) => match error.kind {
                expression_engine::ErrorKind::Empty => String::from("Empty expression"),
                expression_engine::ErrorKind::InvalidOpenCloseParantheses => {
                    String::from("Incorrect number of open/close parantheses")
                }
                expression_engine::ErrorKind::InvalidNumberParsed => {
                    String::from("Failed to parse number(s) in expression")
                }
                expression_engine::ErrorKind::InvalidExpression => {
                    String::from("Expression is invalid")
                }
            },
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Properties)]
pub struct GraphingProps {
    expression_str: IString,
}

impl Component for Graphing {
    type Message = Msg;
    type Properties = GraphingProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            expression_text: String::from("").into(),
            expression: None,
            expression2: Some(parse("x^4").unwrap()),
            variable_text: "0.0".into(),
            result: 0.0,
            width: 0,
            height: 0,
            zoom_level: 1.0,
            color: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputExpression(exp_str) => {
                self.expression_text = exp_str.into();
                match parse(self.expression_text.as_str()) {
                    Ok(exp) => self.expression = Some(exp),
                    Err(_) => (),
                }

                true
            }
            Msg::InputNone(exp_str) => false,
            Msg::InputVariable(var_str) => {
                self.variable_text = var_str.into();
                true
            }
            Msg::ZoomIn() => {
                self.zoom_level = self.zoom_level * 1.1;
                true
            }
            Msg::ZoomOut() => {
                self.zoom_level = self.zoom_level / 1.1;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();
        let canvas_ref = use_node_ref();

        let expression_str = self.expression_text.clone();

        let exp_oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::InputExpression(target.value())
        });

        let var_oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::InputVariable(target.value())
        });

        let zoom_in = link.callback(|e: MouseEvent| Msg::ZoomIn());
        let zoom_out = link.callback(|e: MouseEvent| Msg::ZoomOut());

        let zoom_in_out = link.callback(|e: WheelEvent| {
            if e.delta_y() > 0.0 {
                Msg::ZoomOut()
            } else {
                Msg::ZoomIn()
            }
        });

        html! {
            <section>
                <div class="container expression-panel">
                    <div class="row">
                        <label class="col-6" for="expression-input">
                        { "Expression : " }
                        <input oninput={exp_oninput} value={&self.expression_text}/>
                        </label>
                        <label class="col-6" for="expression-input">
                        { "Variable (x) : " }
                        <input oninput={var_oninput} value={&self.variable_text}/>
                        </label>
                    </div>
                    <div class="row">
                        <p class="col-6"> { self.get_exp_str() } </p>
                        <p class="col-6"> { self.get_exp_res() } </p>
                    </div>
                </div>
            <canvas ref={self.node_ref.clone()} style="width: 100vw; height: 80vh;" onwheel={zoom_in_out}/>
            <div style="position: absolute; top: 10px; right: 10px;">
                <button class="btn btn-primary mx-1" onclick={zoom_in}>{ "Zoom In" }</button>
                <button class="btn btn-primary mx-1" onclick={zoom_out}>{ "Zoom Out" }</button>
            </div>
            </section>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration

        //if !first_render {
        //    return;
        //}

        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        self.render_gl(gl, _ctx);
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

impl Graphing {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn prepare_vertices(&self) -> (Vec<DrawLayer>, Vec<f32>) {
        // Generate grid vertices
        let mut vertices = vec![];
        let mut draw_layers = vec![];

        let mut current_offset: i32 = 0;

        let width = self.width;
        let height = self.height;

        let c_level_3 = (0.85, 0.85, 0.85, 1.0);
        let c_level_2 = (0.9125, 0.9125, 0.9125, 1.0);
        let c_level_1 = (0.975, 0.975, 0.975, 1.0);

        let mut drawings: Vec<Box<dyn LayerDrawing>> = vec![
            Box::new(YAxisLayer {
                width: width,
                height: height,
                color: c_level_1,
                scale: 25,
            }),
            Box::new(XAxisLayer {
                width: width,
                height: height,
                color: c_level_1,
                scale: 25,
            }),
            Box::new(YAxisLayer {
                width: width,
                height: height,
                color: c_level_2,
                scale: 50,
            }),
            Box::new(XAxisLayer {
                width: width,
                height: height,
                color: c_level_2,
                scale: 50,
            }),
            Box::new(XAxisLayer {
                width: width,
                height: height,
                color: c_level_3,
                scale: 100,
            }),
            Box::new(YAxisLayer {
                width: width,
                height: height,
                color: c_level_3,
                scale: 100,
            }),
        ];

        match &self.expression {
            Some(exp) => {
                drawings.push(Box::new(GraphLayer {
                    expression: exp,
                    width: width,
                    height: height,
                    scale: self.zoom_level,
                    color: (0.0, 0.0, 1.0, 1.0),
                }));
            }
            None => (),
        };

        match &self.expression2 {
            Some(exp) => {
                drawings.push(Box::new(GraphLayer {
                    expression: exp,
                    width: width,
                    height: height,
                    scale: self.zoom_level,
                    color: (1.0, 0.0, 1.0, 1.0),
                }));
            }
            None => (),
        };

        for drawing in drawings {
            let draw = drawing.draw(&mut vertices, current_offset);
            current_offset = draw.vertices_offset + draw.vertices_length;
            draw_layers.push(draw);
        }

        return (draw_layers, vertices);
    }

    fn render_gl(&mut self, gl: WebGlRenderingContext, ctx: &Context<Self>) {
        // This should log only once -- not once per frame

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        self.width = canvas.offset_width() as u32;
        self.height = canvas.offset_height() as u32;

        canvas.set_width(self.width);
        canvas.set_height(self.height);

        // Update WebGL viewport
        gl.viewport(0, 0, self.width as i32, self.height as i32);

        //gl.viewport(0, 0, vp_width as i32, vp_height as i32);

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Generate grid vertices
        let (layers, vertices) = self.prepare_vertices();

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Initial transform
        //update_transform(self.zoom_level);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                if (vertices.len() > 0) {
                    // Attach the time as a uniform for the GL context.
                    let color = gl.get_uniform_location(&shader_program, "u_color");

                    for layer in layers.clone() {
                        gl.uniform4f(
                            color.as_ref(),
                            layer.color.0,
                            layer.color.1,
                            layer.color.2,
                            layer.color.3,
                        ); // color for layer
                        gl.draw_arrays(layer.mode, layer.vertices_offset, layer.vertices_length);
                        // draw layer
                    }

                    Graphing::request_animation_frame(cb.borrow().as_ref().unwrap());
                }
            }
        }) as Box<dyn FnMut()>));

        Graphing::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}

fn main() {
    yew::Renderer::<Graphing>::new().render();
}
