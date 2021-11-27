use macroquad::ui::{hash, root_ui, widgets};
use macroquad::prelude::*;

#[macroquad::main("Sudoku")]
async fn main() {
    let font_size: f32 = 30.0;
    let steps = 9.0;
    let offset: usize = 40;
    let start_y: f32 = screen_height() / 2.0 - offset as f32 * steps / 2.0;
    let end_y: f32 = start_y as f32 + offset as f32 * steps;
    let start_x: usize = (screen_width() / 2.0 - offset as f32 * steps / 2.0) as usize;
    let end_x: usize = (start_x as f32 + offset as f32 * steps) as usize;
    let matrix = create_matrix();
    loop {
        clear_background(WHITE);
        draw_form(offset, start_y, end_y, start_x, end_x);
        set_numbers(offset, start_y, start_x, end_x, font_size, &matrix);

        let (mouse_x, mouse_y) = mouse_position();
        if !in_window(mouse_x, mouse_y, start_x as f32, start_y, end_x as f32, end_y) {
            next_frame().await;
            continue
        }
        if is_mouse_button_down(MouseButton::Left) {
            clear_background(WHITE);
            draw_form(offset, start_y, end_y, start_x, end_x);
            let num = num_by_position(mouse_x, mouse_y, start_x as f32, start_y, offset as f32, &matrix);
            println!("click: {}", get_char_code(num));
            let need_mark = coord_by_num(start_x as f32, start_y, offset as f32, &matrix, num);
            for coord in need_mark {
                draw_text(&get_char_code(num), coord[0], coord[1], font_size, RED);
            }
        }
        next_frame().await
    }
}

fn coord_by_num(start_x: f32, start_y: f32, offset: f32, matrix: &Vec<[u32; 9]>, num: u32) -> Vec<[f32; 2]> {
    let mut data: Vec<[f32; 2]> = vec!();
    for y in (0..9).step_by(1) {
        for x in (0..9).step_by(1) {
            if num != matrix[y][x] {
                continue
            }
            data.push([x as f32 * offset + start_x, y as f32 * offset + start_y]);
        }
    }
    return data;
}

fn num_by_position(mouse_x: f32, mouse_y: f32, start_x: f32, start_y: f32, offset: f32, matrix: &Vec<[u32; 9]>) -> u32 {
    let mut x = ((mouse_x - start_x as f32) / offset as f32) as usize;
    let mut y = ((mouse_y - start_y as f32) / offset as f32) as usize;
    if x > 8 {
        x = 8;
    }
    if y > 8 {
        y = 8;
    }
    if x < 0 {
        x = 0;
    }
    if y < 0 {
        y = 0;
    }
    return matrix[y][x];
}

fn in_window(mouse_x:f32, mouse_y:f32, start_x:f32, start_y:f32, end_x:f32, end_y:f32) -> bool {
    if mouse_x < start_x {
        return false;
    }
    if mouse_x > end_x {
        return false;
    }
    if mouse_y > end_y{
        return false;
    }
    if mouse_y < start_y {
        return false;
    }
    return true
}


fn draw_form(offset: usize, start_y: f32, end_y: f32, start_x: usize, end_x: usize) {
    let mut y = start_y;
    let mut counter: usize = 0;
    let def_thickness: f32 = 1.0;
    let def_color = GRAY;
    for x in (start_x..end_x).step_by(offset) {
        let mut thickness = def_thickness;
        let mut color = def_color;
        if x == start_x || x == end_x || counter % 3 == 0 {
            thickness *= 2.0;
            color = BLACK;
        }
        draw_line(start_x as f32, y, end_x as f32, y, thickness, color);
        draw_line(x as f32, start_y, x as f32, end_y as f32, thickness, color);
        y += offset as f32;
        counter += 1;
    }
    draw_line(start_x as f32, end_y as f32, end_x as f32, end_y as f32, 2.0, BLACK);
    draw_line(end_x as f32, start_y, end_x as f32, end_y as f32, 2.0, BLACK);
}

fn set_numbers(offset: usize, start_y: f32, start_x: usize, end_x: usize, font_size: f32, matrix: &Vec<[u32; 9]>) {
    let mut y = start_y;
    let mut counter: usize = 0;
    for x in (start_x..end_x).step_by(offset) {
        for i in (0..9).step_by(1) {
            let text_start_x: f32 = start_x as f32 + (offset * i) as f32 + offset as f32 / 2.0 - font_size / 4.0;
            let text_start_y: f32 = y + offset as f32 - offset as f32 / 2.0 + font_size / 4.0;
            draw_text(&get_char_code(matrix[counter][i]), text_start_x,
                      text_start_y, font_size, BLACK);
        }
        y += offset as f32;
        counter += 1;
    }
}

fn create_matrix() -> Vec<[u32; 9]> {
    let mut data: Vec<[u32; 9]> = vec!();
    for _ in (0..9).step_by(1) {
        let mut new_line: [u32; 9] = [0; 9];
        for i in (0..9).step_by(1) {
            new_line[i] = rand::RandomRange::gen_range(0, 8);
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