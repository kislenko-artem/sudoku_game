use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

mod game;
mod skins;

use crate::game::UI;

struct Context {
    user_id: i32,
}

#[no_mangle]
static mut MY_CONTEXT: Option<Context> = None;

fn get_context() -> &'static mut Context {
    unsafe { MY_CONTEXT.as_mut().unwrap_or_else(|| panic!("blabla")) }
}


#[no_mangle]
fn set_user_id(user_id: i32) {
    let ctx = get_context();
    ctx.user_id = user_id;
}

#[no_mangle]
fn init_webassembly() {
    unsafe {
        MY_CONTEXT = Some(Context { user_id: 0 });
    }
}


#[macroquad::main("Sudoku")]
async fn main() {
    let mut g = game::Game::new(screen_height(), screen_width(), game::Difficult::SuperEasy).await;


    let font = load_ttf_font("./assets/ofont.ru_Montserrat.ttf")
        .await
        .unwrap();
    let logo: Texture2D = load_texture("assets/logo.png").await.unwrap();
    let button: Texture2D = load_texture("assets/button.png").await.unwrap();
    let win_texture: Texture2D = load_texture("assets/win.png").await.unwrap();
    let current_skin = skins::DefaultSkin::default(&root_ui());
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    loop {
        clear_background(WHITE);

        let (mouse_x, mouse_y) = mouse_position();
        // debug!("{} {} {} {}", mouse_x, mouse_y, screen_width(), screen_height());
        match g.current_screen {
            game::Screens::Start => {
                draw_texture(
                    logo,
                    center_x - logo.width() / 2.,
                    center_y - logo.height() / 2.,
                    WHITE,
                );
                draw_rectangle(0.0, center_y + logo.height() / 1.5, screen_width(), 60.0, Color::from_rgba(248, 248, 248, 255));

                root_ui().push_skin(&current_skin.right_ar_skin);
                if root_ui().button(vec2(center_x + logo.width(), center_y + 114.), "   ") {
                    match g.current_difficult {
                        game::Difficult::SuperEasy => { g.current_difficult = game::Difficult::Easy }
                        game::Difficult::Easy => { g.current_difficult = game::Difficult::Medium }
                        game::Difficult::Medium => { g.current_difficult = game::Difficult::Hard }
                        game::Difficult::Hard => { g.current_difficult = game::Difficult::SuperEasy }
                    }
                }

                root_ui().pop_skin();
                root_ui().push_skin(&current_skin.left_ar_skin);
                if root_ui().button(vec2(center_x - logo.width(), center_y + 114.), "   ") {
                    match g.current_difficult {
                        game::Difficult::SuperEasy => { g.current_difficult = game::Difficult::Hard }
                        game::Difficult::Easy => { g.current_difficult = game::Difficult::SuperEasy }
                        game::Difficult::Medium => { g.current_difficult = game::Difficult::Easy }
                        game::Difficult::Hard => { g.current_difficult = game::Difficult::Medium }
                    }
                }

                root_ui().pop_skin();
                root_ui().push_skin(&current_skin.start_skin);
                let level_name: String;
                match g.current_difficult {
                    game::Difficult::SuperEasy => { level_name = "Начинающий".to_owned() }
                    game::Difficult::Easy => { level_name = "Легко".to_owned() }
                    game::Difficult::Medium => { level_name = "Средне".to_owned() }
                    game::Difficult::Hard => { level_name = "Сложно".to_owned() }
                }
                let offset = level_name.chars().count() as f32 * (g.font_size - 12.);
                root_ui().label(vec2(center_x - offset / 2., center_y + 110.), &level_name);
                if root_ui().button(vec2(center_x - button.width() / 2., center_y + 200.), "Новая Игра") {
                    g.regenerate();
                    g.current_screen = game::Screens::Game;
                }
            }
            game::Screens::Game => {
                root_ui().push_skin(&current_skin.numbers_button_skin);
                g.draw_number_buttons();
                root_ui().pop_skin();
                root_ui().push_skin(&current_skin.big_button_skin);
                g.draw_hit_buttons();
                root_ui().pop_skin();
                root_ui().push_skin(&current_skin.game_skin);
                g.draw_game_screen(font, mouse_x, mouse_y);

                if g.is_win() {
                    let w_x_size = 400.;
                    let w_y_size = 368.;
                    widgets::Window::new(1, vec2(center_x - w_x_size / 2., center_y - w_y_size / 2.), vec2(w_x_size, w_y_size))
                        .titlebar(false)
                        .movable(false)
                        .ui(&mut *root_ui(), |ui| {
                            ui.texture(
                                win_texture,
                                w_x_size,
                                w_y_size - 2.,
                            );
                            if ui.button(Vec2::new(w_x_size - 100., w_y_size - 50.), "На главную") {
                                g.current_screen = game::Screens::Start;
                            }
                        });
                }


                root_ui().pop_skin();
                root_ui().push_skin(&current_skin.button_arrow_skin);
                if root_ui().button(Vec2::new(g.start_x as f32, g.start_y - 50.), "J") {
                    g.current_screen = game::Screens::Start;
                }
            }
        }

        next_frame().await;
    }
}
