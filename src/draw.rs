use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use rusttype::Font;
use rusttype::Scale;
use imageproc::rect::Rect;
use std::path::Path;
use super::gen::Sudoku;
fn draw_field(s: &[[i32; 9];9], path: &String) {
    let black = Rgb([0u8, 0u8, 0u8]);
    let white = Rgb([255u8, 255u8, 255u8]);
    let mut image = RgbImage::new(500, 500);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(500, 500), white);
    // draw bounds
    for i in 0..10 {
        draw_line_segment_mut(
            &mut image,
            (25f32 + (i as f32) * 50f32, 25f32),
            (25f32 + (i as f32) * 50f32, 475f32),
            black,
        );
        draw_line_segment_mut(
            &mut image,
            (25f32, 25f32 + (i as f32) * 50f32),
            (475f32, 25f32 + (i as f32) * 50f32),
            black,
        );
        if i % 3 == 0 {
            draw_line_segment_mut(
                &mut image,
                (25f32 + (i as f32) * 50f32 + 1f32, 25f32),
                (25f32 + (i as f32) * 50f32 + 1f32, 475f32),
                black,
            );
            draw_line_segment_mut(
                &mut image,
                (25f32, 25f32 + (i as f32) * 50f32 + 1f32),
                (475f32, 25f32 + (i as f32) * 50f32 + 1f32),
                black,
            );
        }
    }
    let font_data: &[u8] = include_bytes!("../firacodenerd.ttf");
    let font_opt = Font::try_from_bytes(font_data);
    match font_opt {
        Some(font) => {
            for i in 0 .. 9 {
                for j in 0 .. 9 {
                    let val = s[i as usize][j as usize];
                    if val != 0 {
                    draw_text_mut(&mut image, black, 25 + i * 50 + 16, 25 + j * 50 + 12, 
                                  Scale::uniform(30f32), &font, &val.to_string());
                    }
                }
            }
        }
        _ => {
            println!("Could not load font!");
        }
    }
    let path = Path::new(path);
    image.save(path).unwrap();
}

pub fn draw_sudoku(sudoku: &Sudoku, draw_sol: bool) {
    let path = "sudoku-".to_owned() + &sudoku.score.to_string();
    draw_field(&sudoku.problem, &(path.to_owned() + ".jpg"));
    if draw_sol {
       draw_field(&sudoku.solution, &(path.to_owned() + "_solution.jpg"));
    }
}
