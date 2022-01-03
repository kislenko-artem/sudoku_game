use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::ui::{root_ui};
use sudoku::Sudoku;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
    key: KeyCode,
}

#[derive(Clone, Copy)]
pub enum Difficult {
    SuperEasy,
    Easy,
    Medium,
    Hard,
}


pub enum Screens {
    Start,
    Game,
}

pub struct Game {
    pub font_size: f32,
    pub start_y: f32,
    pub start_x: usize,
    pub current_difficult: Difficult,
    pub current_screen: Screens,
    end_y: f32,
    offset: usize,
    end_x: usize,
    user_matrix: HashMap<Key, u8>,
    empties: HashMap<Key, bool>,
    marked_coord: Vec<[usize; 2]>,
    matrix: Vec<[u8; 9]>,
    no_valid: Vec<[usize; 2]>,
    textures: HashMap<String, Texture2D>,
    numbers_coord: Vec<Circle>,
}

impl Game {
    pub async fn new(screen_height: f32, screen_width: f32, current_difficult: Difficult) -> Self {
        let steps = 9.0;
        let offset: usize = 40;
        let start_y: f32 = screen_height / 2.0 - offset as f32 * steps / 1.5;
        let end_y: f32 = start_y as f32 + offset as f32 * steps;
        let start_x: usize = (screen_width / 2.0 - offset as f32 * steps / 2.0) as usize;
        let end_x: usize = (start_x as f32 + offset as f32 * steps) as usize;
        let sudoku = Sudoku::generate_solved();
        let mut empties = Default::default();
        Game::fill_empties(&sudoku, &mut empties, current_difficult);
        
        let color_circle: Texture2D = load_texture("assets/color_circle.png").await.unwrap();
        let textures: HashMap<String, Texture2D> = HashMap::from([
            ("color_circle".to_string(), color_circle),
        ]);

        return Game {
            font_size: 25.0,
            start_y,
            end_y,
            offset,
            start_x,
            end_x,
            user_matrix: Default::default(),
            empties,
            marked_coord: vec![],
            current_difficult,
            matrix: Game::create_matrix(&sudoku),
            no_valid: vec![],
            current_screen: Screens::Start,
            textures,
            numbers_coord: vec![],
        };
    }

    pub fn regenerate(&mut self) {
        let sudoku = Sudoku::generate_solved();
        let mut empties = Default::default();
        Game::fill_empties(&sudoku, &mut empties, self.current_difficult);
        self.user_matrix = Default::default();
        self.empties = empties;
        self.marked_coord = vec![];
        self.matrix = Game::create_matrix(&sudoku);
        self.no_valid = vec![];
    }

    pub fn is_win(&self) -> bool {
        for (key, _) in &self.empties {
            match self.user_matrix.get(&key) {
                None => { return false; }
                Some(v) => {
                    if self.matrix[key.y][key.x] != v.clone() - 1 {
                        return false;
                    }
                    continue;
                }
            }
        }
        return true;
    }

    fn in_numbers_coord(&self, mouse_x: f32, mouse_y: f32) -> Option<KeyCode> {
        for c in &self.numbers_coord {
            if mouse_x < c.x - c.r {
                continue;
            }
            if mouse_x > c.x + c.r {
                continue;
            }
            if mouse_y < c.y - c.r {
                continue;
            }
            if mouse_y > c.y + c.r {
                continue;
            }
            return Some(c.key.clone());
        }
        return None;
    }

    fn fill_num(&mut self, code: Option<KeyCode>) {
        match code {
            None => {}
            Some(code) => {
                if self.marked_coord.is_empty() {
                    return;
                }
                let key = Key { x: self.marked_coord[0][0], y: self.marked_coord[0][1] };
                if !self.empties.contains_key(&key) {
                    return;
                }
                match code {
                    KeyCode::Delete => { self.user_matrix.remove(&key); }
                    KeyCode::KpDecimal => { self.user_matrix.remove(&key); }
                    KeyCode::Backspace => { self.user_matrix.remove(&key); }
                    KeyCode::Key1 => { self.user_matrix.insert(key, 1); }
                    KeyCode::Key2 => { self.user_matrix.insert(key, 2); }
                    KeyCode::Key3 => { self.user_matrix.insert(key, 3); }
                    KeyCode::Key4 => { self.user_matrix.insert(key, 4); }
                    KeyCode::Key5 => { self.user_matrix.insert(key, 5); }
                    KeyCode::Key6 => { self.user_matrix.insert(key, 6); }
                    KeyCode::Key7 => { self.user_matrix.insert(key, 7); }
                    KeyCode::Key8 => { self.user_matrix.insert(key, 8); }
                    KeyCode::Key9 => { self.user_matrix.insert(key, 9); }
                    KeyCode::Kp1 => { self.user_matrix.insert(key, 1); }
                    KeyCode::Kp2 => { self.user_matrix.insert(key, 2); }
                    KeyCode::Kp3 => { self.user_matrix.insert(key, 3); }
                    KeyCode::Kp4 => { self.user_matrix.insert(key, 4); }
                    KeyCode::Kp5 => { self.user_matrix.insert(key, 5); }
                    KeyCode::Kp6 => { self.user_matrix.insert(key, 6); }
                    KeyCode::Kp7 => { self.user_matrix.insert(key, 7); }
                    KeyCode::Kp8 => { self.user_matrix.insert(key, 8); }
                    KeyCode::Kp9 => { self.user_matrix.insert(key, 9); }
                    _ => {}
                }
            }
        }
    }

    fn fill_empties(sudoku: &Sudoku, empties: &mut HashMap<Key, bool>, dif: Difficult) {
        let real_sudoku = Sudoku::generate_from(sudoku.clone());

        let mut grid_line: [u8; 81] = [0; 81];


        match dif {
            Difficult::SuperEasy => {
                let mut start_num = rand::RandomRange::gen_range(0, 10);
                for (i, _) in real_sudoku.iter().enumerate() {
                    match start_num%5 {
                        0 => {
                            let result = rand::RandomRange::gen_range(0, 3);
                            match result {
                                1 => {
                                    grid_line[i] = 1
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            grid_line[i] = 1
                        }
                    }
                    start_num += 1;
                }
            }
            Difficult::Easy => {
                let mut start_num = rand::RandomRange::gen_range(0, 10);
                for (i, _) in real_sudoku.iter().enumerate() {
                    match start_num%2 {
                        0 => {
                            let result = rand::RandomRange::gen_range(0, 3);
                            match result {
                                1 => {
                                    grid_line[i] = 1
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            grid_line[i] = 1
                        }
                    }
                    start_num += 1;
                }
            }
            Difficult::Medium => {
                for (i, num) in real_sudoku.iter().enumerate() {
                    match num {
                        None => {
                            let result = rand::RandomRange::gen_range(0, 4);
                            match result {
                                1 => {
                                    grid_line[i] = 1
                                }
                                _ => {}
                            }
                        }
                        Some(_n) => {
                            grid_line[i] = 1
                        }
                    }
                }
            }
            Difficult::Hard => {
                for (i, num) in real_sudoku.iter().enumerate() {
                    match num {
                        None => {
                            grid_line[i] = 0
                        }
                        Some(_n) => {
                            grid_line[i] = 1
                        }
                    }
                }
            }
        }


        for y in (0..9).step_by(1) {
            for x in (0..9).step_by(1) {
                let v = grid_line.get(x + (y * 9));
                match v {
                    None => {}
                    Some(val) => {
                        match val {
                            0 => {
                                empties.insert(Key { x, y }, true);
                            }
                            _ => {}
                        }
                    }
                };
            }
        }
    }

    fn coord_by_num(&self, num: u8) -> Vec<[usize; 2]> {
        let mut data: Vec<[usize; 2]> = vec!();
        for y in (0..9).step_by(1) {
            for x in (0..9).step_by(1) {
                if num != self.matrix[y][x] {
                    continue;
                }
                data.push([x, y]);
            }
        }
        return data;
    }

    fn coord_by_position(&self, mouse_x: f32, mouse_y: f32) -> (usize, usize) {
        let mut x = ((mouse_x - self.start_x as f32) / self.offset as f32) as usize;
        let mut y = ((mouse_y - self.start_y as f32) / self.offset as f32) as usize;
        if x > 8 {
            x = 8;
        }
        if y > 8 {
            y = 8;
        }
        return (x, y);
    }

    fn in_window(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if mouse_x < self.start_x as f32 {
            return false;
        }
        if mouse_x > self.end_x as f32 {
            return false;
        }
        if mouse_y > self.end_y as f32 {
            return false;
        }
        if mouse_y < self.start_y {
            return false;
        }
        return true;
    }

    fn hint(&mut self) {
        let mut need_mark: [usize; 2] = [9, 9];
        if self.marked_coord.len() == 1 {
            need_mark = self.marked_coord[0]
        } else {
            for (key, _) in &self.empties {
                if self.user_matrix.contains_key(&key) {
                    continue;
                }
                need_mark = [key.x, key.y];
                break;
            }
        }
        if need_mark[0] == 9 {
            return;
        }
        let num = self.matrix[need_mark[1]][need_mark[0]];

        self.user_matrix.insert(Key { x: need_mark[0], y: need_mark[1] }, num + 1);

        return;
    }

    fn validate(&mut self) {
        self.no_valid = vec![];
        for y in (0..9).step_by(1) {
            for x in (0..9).step_by(1) {
                let key = Key { x, y };
                if !self.user_matrix.contains_key(&key) {
                    continue;
                }
                let num = self.user_matrix.get(&Key { x, y });
                match num {
                    None => {
                        continue;
                    }
                    Some(v) => {
                        if self.matrix[y][x] == v.clone() - 1 {
                            continue;
                        }
                        self.no_valid.push([x, y])
                    }
                }
            }
        }
    }

    fn create_matrix(grid: &sudoku::Sudoku) -> Vec<[u8; 9]> {
        let mut data: Vec<[u8; 9]> = vec!();
        let mut grid_line: [u8; 81] = [0; 81];

        for (i, num) in grid.iter().enumerate() {
            match num {
                None => {}
                Some(n) => {
                    grid_line[i] = n
                }
            }
        }
        for y in (0..9).step_by(1) {
            let mut new_line: [u8; 9] = [0; 9];
            for x in (0..9).step_by(1) {
                new_line[x] = grid_line[x + (y * 9)] - 1
            }
            data.push(new_line);
        }
        return data;
    }

    fn get_char_code(c: u8) -> String {
        let chars: [String; 9] = ["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned(),
            "5".to_owned(), "6".to_owned(), "7".to_owned(), "8".to_owned(), "9".to_owned()];
        let d = chars.get(c as usize).unwrap();
        return d.clone();
    }
}

pub trait UI {
    fn draw_form(&self);
    fn draw_number_buttons(&mut self);
    fn draw_hit_buttons(&mut self);
    fn draw_game_screen(&mut self, font: Font, mouse_x: f32, mouse_y: f32);
    fn draw_numbers(&self, need_mark: Vec<[usize; 2]>, font: Font);
}

impl UI for Game {
    fn draw_form(&self) {
        let mut y = self.start_y;
        let mut counter: usize = 0;
        let def_thickness: f32 = 1.0;
        let def_color = Color::from_rgba(219, 219, 219, 255);
        for x in (self.start_x..self.end_x).step_by(self.offset) {
            if x == self.start_x {
                y += self.offset as f32;
                counter += 1;
                continue;
            }
            let mut thickness = def_thickness;
            let mut color = def_color;
            if x == self.start_x || x == self.end_x || counter % 3 == 0 {
                thickness *= 2.0;
                color = Color::from_rgba(125, 208, 255, 255);

                draw_line(self.start_x as f32, y, self.end_x as f32, y, thickness, color);
                draw_line(x as f32, self.start_y, x as f32, self.end_y as f32, thickness, color);
                y += self.offset as f32;
                counter += 1;
                continue;
            }

            for i in 0..9 {
                let offset = self.offset as f32 * i as f32;
                let x1 = self.start_x as f32 + offset + 5.;
                let x2 = self.start_x as f32 + offset as f32 + self.offset as f32 - 5.;
                draw_line(x1, y, x2, y, thickness, color);

                let y1 = self.start_y as f32 + offset + 5.;
                let y2 = self.start_y as f32 + offset as f32 + self.offset as f32 - 5.;
                draw_line(x as f32, y1, x as f32, y2, thickness, color);
            }

            y += self.offset as f32;
            counter += 1;
        }
    }

    fn draw_number_buttons(&mut self) {
        let mut y = self.end_y as f32 + self.offset as f32 * 1.8;
        let offset = self.offset as f32 / 1.7;
        let r = 26.;
        let circle_x_offset = 13.;
        let circle_y_offset = 20.;
        let circle_color = Color::from_rgba(166, 166, 166, 255);
        let first_x = self.start_x as f32 + r;
        let key_val: HashMap<i32, KeyCode> = HashMap::from([
            (1, KeyCode::Key1),
            (2, KeyCode::Key2),
            (3, KeyCode::Key3),
            (4, KeyCode::Key4),
            (5, KeyCode::Key5),
            (6, KeyCode::Key6),
            (7, KeyCode::Key7),
            (8, KeyCode::Key8),
            (9, KeyCode::Key9),
        ]);

        self.numbers_coord.push(Circle { x: first_x + 12., y: y + circle_y_offset, r, key: KeyCode::Key1 });
        draw_circle_lines(first_x + 12., y + circle_y_offset, r, 1.0, circle_color);
        if root_ui().button(Vec2::new(first_x + 3., y), "1") {
            self.fill_num(Option::Some(KeyCode::Key1));
        }

        let mut multi = 3.;
        for i in 2..6 {
            let new_offset = offset as f32 * multi;
            draw_circle_lines(first_x + new_offset + circle_x_offset, y + circle_y_offset, r, 1.0, circle_color);
            self.numbers_coord.push(Circle { x: first_x + new_offset + circle_x_offset, y: y + circle_y_offset, r, key: key_val.get(&i).cloned().unwrap() });
            if root_ui().button(Vec2::new(first_x + new_offset, y), i.to_string()) {
                self.fill_num(key_val.get(&i).cloned());
            }
            multi += 3.;
        }
        y += r * 2.2;
        multi = 0.;
        for i in 6..10 {
            let new_offset = offset as f32 * multi;
            draw_circle_lines(first_x + new_offset + circle_x_offset, y + circle_y_offset, r, 1.0, circle_color);
            self.numbers_coord.push(Circle { x: first_x + new_offset + circle_x_offset, y: y + circle_y_offset, r, key: key_val.get(&i).cloned().unwrap() });
            if root_ui().button(Vec2::new(first_x + new_offset, y), i.to_string()) {
                self.fill_num(key_val.get(&i).cloned());
            }
            multi += 3.;
        }

        let new_offset = offset as f32 * multi;
        draw_circle_lines(first_x + new_offset + circle_x_offset, y + circle_y_offset, r, 1.0, circle_color);
        self.numbers_coord.push(Circle { x: first_x + new_offset + circle_x_offset, y: y + circle_y_offset, r, key: KeyCode::Delete });
        if root_ui().button(Vec2::new(first_x + new_offset, y), "X") {
            self.fill_num(Option::Some(KeyCode::Delete));
        }
    }

    fn draw_hit_buttons(&mut self) {
        let y = self.end_y as f32 - self.offset as f32 / 3.0;
        let offset = self.offset as f32 / 1.7;
        let first_x = self.start_x as f32;

        if root_ui().button(Vec2::new(first_x, y + offset), "Проверить") {
            self.validate();
        }
        if root_ui().button(Vec2::new(first_x + offset as f32 * 8., y + offset), "Подсказка") {
            self.hint();
        }
    }


    fn draw_game_screen(&mut self, font: Font, mut mouse_x: f32, mut mouse_y: f32) {
        self.draw_form();

        let mut is_left = false;
        let mut is_right = false;
        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    is_left = true;
                }
                TouchPhase::Stationary => {
                    info!("Stationary");
                }
                TouchPhase::Moved => {}
                TouchPhase::Ended => {
                    is_left = false;
                    mouse_x = touch.position[0];
                    mouse_y = touch.position[1];
                }
                TouchPhase::Cancelled => {}
            };
        }

        if is_mouse_button_down(MouseButton::Left) {
            is_left = true
        }
        if is_mouse_button_down(MouseButton::Left) {
            is_right = true
        }

        match self.in_numbers_coord(mouse_x, mouse_y) {
            None => {}
            Some(key_kode) => {
                if is_left {
                    self.fill_num(Option::Some(key_kode));
                }
            }
        }

        if !self.in_window(mouse_x, mouse_y) {
            self.draw_numbers(vec![], font.clone());
            if is_mouse_button_down(MouseButton::Left) {
                self.marked_coord = vec![];
            }
            return;
        }
        if is_left || is_right {
            self.no_valid = vec![]
        }


        let (x, y) = self.coord_by_position(mouse_x, mouse_y);

        if is_left || is_right {
            if is_left {
                self.marked_coord = vec!([x, y]);
            }
            self.draw_form();
            let key = Key { x, y };

            let mut num = self.matrix[y][x];
            if self.user_matrix.contains_key(&key) {
                num = self.user_matrix.get(&key).unwrap().clone() - 1
            }

            if self.empties.contains_key(&key) && !self.user_matrix.contains_key(&key) {
                self.draw_numbers(self.marked_coord.clone(), font.clone());
                return;
            }

            let need_mark = self.coord_by_num(num);

            self.draw_numbers(need_mark, font.clone());

            return;
        }


        self.fill_num(get_last_key_pressed());

        self.draw_numbers(vec![], font.clone());
    }

    fn draw_numbers(&self, mut need_mark: Vec<[usize; 2]>, font: Font) {
        let mut y = self.start_y;
        if need_mark.len() == 0 {
            need_mark = self.marked_coord.clone()
        }
        let mut counter: usize = 0;
        for _ in (self.start_x..self.end_x).step_by(self.offset) {
            for i in (0..9).step_by(1) {
                let mut val = Game::get_char_code(self.matrix[counter][i]);
                let mut color = Color::from_rgba(100, 100, 100, 255);
                for coord in &need_mark {
                    if coord[0] == i && coord[1] == counter {
                        let x: f32 = self.start_x as f32 + (self.offset * i) as f32 + 3.;
                        let key = Key { x: coord[0], y: coord[1] };
                        match self.empties.get(&key) {
                            None => { draw_texture(self.textures.get("color_circle").unwrap().clone(), x, y + 3., WHITE) }
                            Some(_) => {}
                        }
                        match self.user_matrix.get(&key) {
                            None => {}
                            Some(_) => { draw_texture(self.textures.get("color_circle").unwrap().clone(), x, y + 3., WHITE) }
                        }
                        color = Color::from_rgba(255, 255, 255, 255);
                        //color = RED;
                    }
                }
                for coord in &self.marked_coord {
                    if coord[0] == i && coord[1] == counter {
                        let x = self.start_x as f32 + (self.offset * i) as f32;
                        draw_texture(self.textures.get("color_circle").unwrap().clone(), x + 3., y + 3., WHITE);
                        // draw_rectangle(self.start_x as f32 + (self.offset * i) as f32, y, self.offset as f32, self.offset as f32, GREEN);
                    }
                }
                for (key, _) in &self.empties {
                    if key.x == i && key.y == counter {
                        color = Color::from_rgba(125, 208, 255, 255);
                        val = "".to_owned();
                    }
                }
                for (key, v) in &self.user_matrix {
                    if key.x == i && key.y == counter {
                        color = Color::from_rgba(125, 208, 255, 255);
                        val = v.to_string();
                        for coord in &self.marked_coord {
                            if coord[0] == key.x && coord[1] == key.y {
                                color = WHITE;
                            }
                        }
                    }
                }
                for coord in &self.no_valid {
                    if coord[0] == i && coord[1] == counter {
                        color = RED;
                    }
                }
                let text_start_x: f32 = self.start_x as f32 + (self.offset * i) as f32 + self.offset as f32 / 2.0 - self.font_size / 3.5;
                let text_start_y: f32 = y + self.offset as f32 - self.offset as f32 / 2.0 + self.font_size / 4.0;
                draw_text_ex(&val, text_start_x, text_start_y, TextParams {
                    font_size: self.font_size as u16,
                    font,
                    color,
                    ..Default::default()
                });
            }
            y += self.offset as f32;
            counter += 1;
        }
    }
}