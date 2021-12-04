use std::collections::HashMap;
use macroquad::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key {
    x: usize,
    y: usize,
}

enum Difficult {
    SuperEasy,
    Easy,
    Medium,
    Hard,
}


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
    let mut user_matrix: HashMap<Key, u32> = HashMap::default();
    let mut empties: HashMap<Key, bool> = HashMap::default();
    let mut marked_coord: Vec<[usize; 2]> = vec![];
    let current_difficult = Difficult::SuperEasy;

    match current_difficult {
        Difficult::SuperEasy => {
            for y in (0..9).step_by(3) {
                for x in (0..9).step_by(3) {
                    empties.insert(Key{x: x + rand::RandomRange::gen_range(0, 1), y: y}, false);
                }
            }
        }
        Difficult::Easy => {}
        Difficult::Medium => {}
        Difficult::Hard => {}
    }

    loop {
        clear_background(WHITE);
        draw_form(offset, start_y, end_y, start_x, end_x);

        let (mouse_x, mouse_y) = mouse_position();
        if !in_window(mouse_x, mouse_y, start_x as f32, start_y, end_x as f32, end_y) {
            set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &marked_coord, &user_matrix, &empties);
            if is_mouse_button_down(MouseButton::Left) {
                marked_coord = vec![];
            }
            next_frame().await;
            continue;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = coord_by_position(mouse_x, mouse_y, start_x as f32, start_y, offset as f32);
            clear_background(WHITE);
            draw_form(offset, start_y, end_y, start_x, end_x);
            let mut num = matrix[y][x];
            let key = Key { x, y };
            if user_matrix.contains_key( &key){
                num = user_matrix.get(&key).unwrap().clone() - 1
            } else {
                if empties.contains_key(&key) {
                    marked_coord = vec!([x, y]);
                    set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &marked_coord, &user_matrix, &empties);
                    next_frame().await;
                    continue;
                }
            }
            let need_mark = coord_by_num(&matrix, num);
            set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &need_mark, &user_matrix, &empties);
            marked_coord = vec!([x, y]);
            next_frame().await;
            continue;
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = coord_by_position(mouse_x, mouse_y, start_x as f32, start_y, offset as f32);
            clear_background(WHITE);
            draw_form(offset, start_y, end_y, start_x, end_x);
            let mut num = matrix[y][x];
            let key = Key { x, y };
            if user_matrix.contains_key( &key){
                num = user_matrix.get(&key).unwrap().clone() - 1
            } else {
                if empties.contains_key(&key) {
                    set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &marked_coord, &user_matrix, &empties);
                    next_frame().await;
                    continue;
                }
            }
            let need_mark = coord_by_num(&matrix, num);
            set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &need_mark, &user_matrix, &empties);
            next_frame().await;
            continue;
        }

        match get_last_key_pressed() {
            None => {}
            Some(code) => {
                if marked_coord.is_empty() {
                    next_frame().await;
                    continue;
                }
                let key = Key { x: marked_coord[0][0], y: marked_coord[0][1] };
                if !empties.contains_key(&key) {
                    next_frame().await;
                    continue
                }
                match code {
                    KeyCode::Key1 => { user_matrix.insert(key, 1); }
                    KeyCode::Key2 => { user_matrix.insert(key, 2); }
                    KeyCode::Key3 => { user_matrix.insert(key, 3); }
                    KeyCode::Key4 => { user_matrix.insert(key, 4); }
                    KeyCode::Key5 => { user_matrix.insert(key, 5); }
                    KeyCode::Key6 => { user_matrix.insert(key, 6); }
                    KeyCode::Key7 => { user_matrix.insert(key, 7); }
                    KeyCode::Key8 => { user_matrix.insert(key, 8); }
                    KeyCode::Key9 => { user_matrix.insert(key, 9); }
                    KeyCode::Kp1 => { user_matrix.insert(key, 1); }
                    KeyCode::Kp2 => { user_matrix.insert(key, 2); }
                    KeyCode::Kp3 => { user_matrix.insert(key, 3); }
                    KeyCode::Kp4 => { user_matrix.insert(key, 4); }
                    KeyCode::Kp5 => { user_matrix.insert(key, 5); }
                    KeyCode::Kp6 => { user_matrix.insert(key, 6); }
                    KeyCode::Kp7 => { user_matrix.insert(key, 7); }
                    KeyCode::Kp8 => { user_matrix.insert(key, 8); }
                    KeyCode::Kp9 => { user_matrix.insert(key, 9); }
                    _ => {}
                }
            }
        }

        set_numbers(offset, start_y, start_x, end_x, font_size, &matrix, &marked_coord, &user_matrix, &empties);
        next_frame().await
    }
}

fn coord_by_num(matrix: &Vec<[u32; 9]>, num: u32) -> Vec<[usize; 2]> {
    let mut data: Vec<[usize; 2]> = vec!();
    for y in (0..9).step_by(1) {
        for x in (0..9).step_by(1) {
            if num != matrix[y][x] {
                continue;
            }
            data.push([x, y]);
        }
    }
    return data;
}

fn coord_by_position(mouse_x: f32, mouse_y: f32, start_x: f32, start_y: f32, offset: f32) -> (usize, usize) {
    let mut x = ((mouse_x - start_x as f32) / offset as f32) as usize;
    let mut y = ((mouse_y - start_y as f32) / offset as f32) as usize;
    if x > 8 {
        x = 8;
    }
    if y > 8 {
        y = 8;
    }
    return (x, y);
}

fn in_window(mouse_x: f32, mouse_y: f32, start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> bool {
    if mouse_x < start_x {
        return false;
    }
    if mouse_x > end_x {
        return false;
    }
    if mouse_y > end_y {
        return false;
    }
    if mouse_y < start_y {
        return false;
    }
    return true;
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

fn set_numbers(
    offset: usize,
    start_y: f32,
    start_x: usize,
    end_x: usize,
    font_size: f32,
    matrix: &Vec<[u32; 9]>,
    need_mark: &Vec<[usize; 2]>,
    user_matrix: &HashMap<Key, u32>,
    empties: &HashMap<Key, bool>) {
    let mut y = start_y;
    let mut counter: usize = 0;
    for _ in (start_x..end_x).step_by(offset) {
        for i in (0..9).step_by(1) {
            let mut val = get_char_code(matrix[counter][i]);
            let mut color = BLACK;
            for coord in need_mark {
                if coord[0] == i && coord[1] == counter {
                    color = RED;
                }
            }
            for (key, v) in empties {
                if key.x == i && key.y == counter {
                    color = BLUE;
                    val = "".to_owned();
                }
            }
            for (key, v) in user_matrix {
                if key.x == i && key.y == counter {
                    color = BLUE;
                    val = v.to_string();
                }
            }
            let text_start_x: f32 = start_x as f32 + (offset * i) as f32 + offset as f32 / 2.0 - font_size / 4.0;
            let text_start_y: f32 = y + offset as f32 - offset as f32 / 2.0 + font_size / 4.0;
            draw_text(&val, text_start_x, text_start_y, font_size, color);
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