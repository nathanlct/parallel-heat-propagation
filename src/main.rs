
extern crate fps_counter;
extern crate image;
extern crate piston_window;

use piston_window::*;

/// Ouvre une fenêtre pour afficher une image. L'image est mise à jour entre chaque
/// affichage en appelant la fonction `step`.
pub fn display<F>(title: &str, height: usize, width: usize, mut step: F)
        where F: FnMut(&mut image::RgbaImage) {
    // Création de la fenêtre.
    let glutin_window = WindowSettings::new(title, (width as u32, height as u32))
        .exit_on_esc(true)
        .resizable(false)
        .srgb(false) // Necessary due to issue #139 of piston_window.
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    let mut window: PistonWindow = PistonWindow::new(OpenGL::V3_2, 0, glutin_window);
    // Création de l'image.
    let black_pixel = image::Rgba { data: [0, 0, 0, 255] };
    let mut img = image::RgbaImage::from_pixel(width as u32, height as u32, black_pixel);
    let tex_settings = TextureSettings::new();
    let mut tex_factory = window.factory.clone();
    // Création du conteur de FPS.
    let mut fps_counter = fps_counter::FPSCounter::new();
    let font = "assets/FiraMono-Regular.ttf";
    let glyph_settings = TextureSettings::new();
    let mut glyphs = Glyphs::new(font, window.factory.clone(), glyph_settings).unwrap();

    // Boucle de traitement des évenements.
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            // Affichage d'un pas de calcul.
            step(&mut img);
            let tex = Texture::from_image(&mut tex_factory, &img, &tex_settings).unwrap();
            image(&tex, c.transform, g);
            // Affichage du compteur de fps.
            let fps = format!("{} fps", fps_counter.tick());
            let transform = c.transform.trans((width-100) as f64, 30.0);
            text([1.0, 1.0, 1.0, 1.0], 16, &fps, &mut glyphs, transform, g);
        });
        e.idle(|_| {
            fps_counter.tick();
            step(&mut img);
        });
    }
}

/// Maps values between -1 and 1 to RGB colors.
fn map_color(value: f64) -> (u8, u8, u8) {
    // Express as HSL with S=1 and L=.5, and H between 0(red) and 4/6(blue).
    let h = f64::max(0.0, f64::min(1.0, (1.0-value) * 2.0/6.0));
    // Then convert to RGB.
    let x = 1.0 - (((h*6.0) % 2.0) - 1.0).abs();
    let (r, g, b) = if h < 1.0/6.0 {
        (1.0, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, 1.0, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, 1.0, x)
    } else {
        (0.0, x, 1.0)
    };
    ((r*255.0) as u8, (g*255.0) as u8 , (b*255.0) as u8)
}


/// Hauteur de la carte de température.
const HEIGHT: usize = 600;
// Largeur de la carte de température.
const WIDTH: usize = 800;


fn temp_to_image(mat: &Vec<Vec<f64>>, img: &mut [u8]) {
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            let (r, g, b) = map_color(mat[i][j]);
            img[4 * (i * WIDTH + j)] = r;
            img[4 * (i * WIDTH + j) + 1] = g;
            img[4 * (i * WIDTH + j) + 2] = b;
            img[4 * (i * WIDTH + j) + 3] = 255;
        }
    }
}

/// Pas temporel de calcul.
const DT: f64 = 1.0e-1;
/// Pas dimentionel de calcul.
const DX: f64 = 1.0e-1;

use std::f64;

fn u(x: f64, y: f64, t: f64) -> f64 {
    f64::sin((t + x + y) / std::f64::consts::PI)
}

fn main() {

    let mut iter : usize = 0;

    display("Propagation de la chaleur 2D", HEIGHT, WIDTH, |image| {

        let mut distribution = Vec::new();
        for i in 0..HEIGHT {
            let mut line = Vec::new();
            for j in 0..WIDTH {
                line.push(u((j as f64) * DX, (i as f64) * DX, (iter as f64) * DT));
            }
            distribution.push(line);
        }
        iter += 1;

        temp_to_image(&distribution, image);
    });
}
