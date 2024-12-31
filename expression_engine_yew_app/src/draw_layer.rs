use expression_engine::ExpressionNode;
use web_sys::WebGlRenderingContext as GL;

#[derive(Default, Clone, PartialEq)]
pub struct DrawLayer {
    pub mode: u32,
    pub color: (f32, f32, f32, f32),
    pub vertices_offset: i32,
    pub vertices_length: i32,
}

pub trait LayerDrawing {
    fn draw(&self, vertices: &mut Vec<f32>, current_offset: i32) -> DrawLayer;
}

pub struct XAxisLayer {
    pub width: u32,
    pub height: u32,
    pub color: (f32, f32, f32, f32),
    pub scale: i32,
}
pub struct YAxisLayer {
    pub width: u32,
    pub height: u32,
    pub color: (f32, f32, f32, f32),
    pub scale: i32,
}

impl LayerDrawing for XAxisLayer {
    fn draw(&self, vertices: &mut Vec<f32>, current_offset: i32) -> DrawLayer {
        let current_size = vertices.len();
        for x in (-(self.width as i32)..=(self.width as i32)).filter(|i| i % self.scale == 0) {
            vertices.push(x as f32 / self.width as f32);
            vertices.push(-1.0);
            vertices.push(x as f32 / self.width as f32);
            vertices.push(1.0);
        }

        DrawLayer {
            color: self.color,
            mode: GL::LINES,
            vertices_offset: current_offset,
            vertices_length: (vertices.len() as i32 - current_size as i32) / 2,
        }
    }
}

impl LayerDrawing for YAxisLayer {
    fn draw(&self, vertices: &mut Vec<f32>, current_offset: i32) -> DrawLayer {
        let current_size = vertices.len();
        for y in (-(self.height as i32)..=(self.height as i32)).filter(|i| i % self.scale == 0) {
            vertices.push(-1.0);
            vertices.push(y as f32 / self.height as f32);
            vertices.push(1.0);
            vertices.push(y as f32 / self.height as f32);
        }

        DrawLayer {
            color: self.color,
            mode: GL::LINES,
            vertices_offset: current_offset,
            vertices_length: (vertices.len() as i32 - current_size as i32) / 2,
        }
    }
}

pub struct GraphLayer<'a> {
    pub expression: &'a ExpressionNode,
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub color: (f32, f32, f32, f32),
}

impl LayerDrawing for GraphLayer<'_> {
    fn draw(&self, vertices: &mut Vec<f32>, current_offset: i32) -> DrawLayer {
        let current_size = vertices.len();
        let width = self.width as i32;
        let height = self.height as i32;
        for px in 0..=self.width {
            // Map pixel (px) to normalized device coordinates (NDC) for x
            let x_ndc = (px as f64 / width as f64) * 2.0 - 1.0;

            // Compute y based on the mathematical expression
            match self.expression.evaluate("x", (x_ndc as f64) / self.scale) {
                Ok(eval) => {
                    let y = ((eval * ((width as f64) / height as f64)) * self.scale) as f32;
                    vertices.push(x_ndc as f32);
                    vertices.push(y);
                }
                Err(_) => (),
            };

            // Map y to NDC (already in -1 to 1 range)
        }

        DrawLayer {
            color: self.color,
            mode: GL::LINE_STRIP,
            vertices_offset: current_offset,
            vertices_length: (vertices.len() as i32 - current_size as i32) / 2,
        }
    }
}
