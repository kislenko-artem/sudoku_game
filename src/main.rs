use macroquad::prelude::*;

#[macroquad::main("Sudoku")]
async fn main() {
    let steps = 9.0;
    let offset: usize = 40;
    let start_y: f32 = screen_height() / 2.0 - offset as f32 * steps / 2.0;
    let end_y: f32 = start_y as f32 + offset as f32 * steps;
    let start_x: usize = (screen_width() / 2.0 - offset as f32 * steps / 2.0) as usize;
    let end_x: usize = (start_x as f32 + offset as f32 * steps) as usize;
    loop {
        clear_background(WHITE);
        draw_form(steps, offset, start_y, end_y, start_x, end_x, 30.0);

        next_frame().await
    }
}

fn draw_form(steps: f32, offset: usize, start_y: f32, end_y: f32, start_x: usize, end_x: usize, font_size: f32) {
    let mut y = start_y;
    let matrix = create_matrix();
    let mut counter: usize = 0;
    for x in (start_x..end_x).step_by(offset) {
        for i in (0..9).step_by(1) {
            let text_start_x: f32 = start_x as f32 + (offset * i) as f32 + offset as f32 / 2.0 - font_size/ 4.0;
            let text_start_y: f32 = y + offset as f32 - offset as f32 / 2.0 + font_size/ 4.0;
            draw_text(&get_char_code(matrix[counter][i]), text_start_x,
                      text_start_y, font_size, BLACK);
        }
        draw_line(start_x as f32, y, end_x as f32, y, 1.0, BLACK);
        draw_line(x as f32, start_y, x as f32, end_y as f32, 1.0, BLACK);
        y += offset as f32;
        counter += 1;
    }
    draw_line(start_x as f32, end_y as f32, end_x as f32, end_y as f32, 1.0, BLACK);
    draw_line(end_x as f32, start_y, end_x as f32, end_y as f32, 1.0, BLACK);
}

fn create_matrix() -> Vec<[u32; 9]> {
    let mut data: Vec<[u32; 9]> = vec!();
    for _ in (0..9).step_by(1) {
        let mut new_line: [u32; 9] = [0; 9];
        for i in (0..9).step_by(1) {
            new_line[i] = 0;
        }
        data.push(new_line);
    }
    return data;
}

fn get_char_code(c: u32) -> String {
    let chars: [String; 9] = ["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned(),
        "5".to_owned(), "6".to_owned(), "7".to_owned(), "8".to_owned(), "9".to_owned()];
    let d = chars.get(c as usize).unwrap();
    return d.clone();
}