use wasm_bindgen::Clamped;

#[allow(dead_code)]
pub enum ShadingType {
    OpacityAndColor,
    OpacityOnly,
    ColorOnly,
}

pub struct MandelbrotBuilder {
    pub height: usize,
    pub width: usize,
    pub max_iterations: u32,
    pub shading_type: ShadingType,
    pub opacity: u8,
    pub zoom: f64,
    pub x_offset: f64,
    pub y_offset: f64,
}

const NUM_SHADES: u32 = 13;
impl MandelbrotBuilder {
    pub fn new() -> Self {
        MandelbrotBuilder {
            height: 0,
            width: 0,
            max_iterations: 0,
            shading_type: ShadingType::OpacityAndColor,
            opacity: 0,
            zoom: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    pub fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn max_iterations(mut self, max_iterations: u32) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn shading_type(mut self, shading_type: ShadingType) -> Self {
        self.shading_type = shading_type;
        self
    }

    pub fn opacity(mut self, opacity: u8) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn zoom(mut self, zoom: f64) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn x_offset(mut self, x_offset: f64) -> Self {
        self.x_offset = x_offset;
        self
    }

    pub fn y_offset(mut self, y_offset: f64) -> Self {
        self.y_offset = y_offset;
        self
    }

    pub fn build(self) -> Mandelbrot {
        Mandelbrot::init_with_props(self)
    }
}

pub struct Mandelbrot {
    pub plot: Vec<Vec<u32>>,
    height: usize,
    width: usize,
    max_iterations: u32,
    shading_type: ShadingType,
    opacity: u8,
    zoom: f64,
    x_offset: f64,
    y_offset: f64,
}

impl Mandelbrot {
    pub fn init_with_props(m: MandelbrotBuilder) -> Self {
        Mandelbrot {
            plot: vec![vec![0; m.height]; m.width],
            height: m.height,
            width: m.width,
            max_iterations: m.max_iterations,
            shading_type: m.shading_type,
            opacity: m.opacity,
            zoom: m.zoom,
            x_offset: m.x_offset,
            y_offset: m.y_offset,
        }
    }

    fn point_in_mandelbrot_set(&self, x: f64, y: f64) -> u32 {
        let mut zx: f64 = 0.0;
        let mut zy: f64 = 0.0;

        let mut i: u32 = 0;
        loop {
            //Z(n+1) = Z(n)^2 + c
            let xt = zx * zy;
            zx = (zx * zx) - (zy * zy) + x;
            zy = 2.0 * xt + y;

            //this is pythagoreans theorum without square root because
            //ya know, power intensive
            //(sqrt of (zx^2 + zy^2)) > 2.0
            //So if
            if (zx * zx) + (zy * zy) > 4.0 || i > self.max_iterations {
                /*if (i >= 1) {
                    println!("Val on break: {}", (zx*zx) + (zy*zy));
                    println!("i = {}", i);
                }*/
                break;
            }
            i += 1;
        }

        return i;
    }

    pub fn draw(&mut self) -> &Vec<Vec<u32>> {
        let mut r: Vec<Vec<u32>> = vec![vec![0; self.height]; self.width];

        let x_start: f64 = (-2.0 * self.zoom) + self.x_offset;
        let x_end: f64 = (0.5 * self.zoom) + self.x_offset;
        let y_start: f64 = (-1.2 * self.zoom) + self.y_offset;
        let y_end: f64 = (1.2 * self.zoom) + self.y_offset;

        let delta_x: f64 = (x_end - x_start) / (self.width as f64);
        let delta_y: f64 = (y_end - y_start) / (self.height as f64);

        for i in 0..self.width {
            for j in 0..self.height {
                let x = x_start + (i as f64 * delta_x);
                let y = y_start + (j as f64 * delta_y);

                r[i][j] = self.point_in_mandelbrot_set(x, y);
            }
        }
        self.plot = r;
        &self.plot
    }

    fn get_image_data(&self) -> Clamped<&[u8]> {
        let mut r: Vec<u8> = Vec::new();

        for i in 0..self.width {
            for j in 0..self.height {
                let shade = self.plot[i][j];
                let opacity = if shade == self.max_iterations {
                    0
                } else {
                    self.opacity
                };
                let color = if shade == self.max_iterations {
                    0
                } else {
                    (shade % NUM_SHADES) * 255 / NUM_SHADES
                };
                let color = vec![color, color, color, opacity];
                /*let color = match self.shading_type {
                    ShadingType::OpacityAndColor => {
                        let opacity = if shade == self.max_iterations {
                            0
                        } else {
                            self.opacity
                        };
                        let color = if shade == self.max_iterations {
                            0
                        } else {
                            (shade % NUM_SHADES) * 255 / NUM_SHADES
                        };
                        vec![color, color, color, opacity]
                    }
                    ShadingType::OpacityOnly => {
                        let opacity = if shade == self.max_iterations {
                            0
                        } else {
                            self.opacity
                        };
                        vec![0, 0, 0, opacity]
                    }
                    ShadingType::ColorOnly => {
                        let color = if shade == self.max_iterations {
                            0
                        } else {
                            (shade % NUM_SHADES) * 255 / NUM_SHADES
                        };
                        vec![color, color, color, 255]
                    }
                };*/
                r.extend(color);
            }
        }
        Clamped(&r)
    }
}
