use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui};

use std::{ops::DerefMut};
pub struct DefaultSkin {
    pub start_skin: Skin,
    pub left_ar_skin: Skin,
    pub right_ar_skin: Skin,
    pub numbers_button_skin: Skin,
    pub big_button_skin: Skin,
    pub button_arrow_skin: Skin,
    pub game_skin: Skin,
}

impl DefaultSkin {
    pub fn default(root_ui: &impl DerefMut<Target = Ui>) -> Self {

        let left_ar_button_style = root_ui.style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../../assets/left_ar.png"),
                None,
            ))
            .font_size(10)
            .build();

        let right_ar_button_style = root_ui.style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../../assets/right_ar.png"),
                None,
            ))
            .font_size(10)
            .build();

        let start_button_style = root_ui.style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../../assets/button.png"),
                None,
            ))
            .text_color(Color::from_rgba(255, 255, 255, 255))
            .font_size(20)
            .margin(RectOffset::new(20.0, 110.0, 11.0, 11.0))
            .font(include_bytes!("../../assets/ofont.ru_Montserrat.ttf")).unwrap()
            .build();

        let start_label_style = root_ui.style_builder()
            .text_color(Color::from_rgba(125, 208, 255, 255))
            .font_size(20)
            .font(include_bytes!("../../assets/ofont.ru_Montserrat.ttf")).unwrap()
            .build();

        let numbers_button_style = root_ui.style_builder()
            .text_color(Color::from_rgba(166, 166, 166, 255))
            .font_size(40)
            .font(include_bytes!("../../assets/MontserratBold.ttf")).unwrap()
            .build();

        let big_button_style = root_ui.style_builder()
            .background(Image::from_file_with_format(
                include_bytes!("../../assets/button_gray.png"),
                None,
            ))
            .margin(RectOffset::new(45.0, 35.0, 10.0, 18.0))
            .text_color(Color::from_rgba(0, 0, 0, 255))
            .font_size(17)
            .font(include_bytes!("../../assets/ofont.ru_Montserrat.ttf")).unwrap()
            .build();

        let button_arrow_style = root_ui.style_builder()
            .text_color(Color::from_rgba(141, 141, 141, 255))
            .font_size(40)
            .font(include_bytes!("../../assets/arrows.ttf")).unwrap()
            .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .build();


        let button_game_style = root_ui.style_builder()
            .text_color(Color::from_rgba(141, 141, 141, 255))
            .font_size(40)
            .font(include_bytes!("../../assets/arrows.ttf")).unwrap()
            .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .build();

        let window_style = root_ui.style_builder()
            .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .build();

        return DefaultSkin {
            start_skin: Skin {
                button_style: start_button_style,
                label_style: start_label_style,
                ..root_ui.default_skin()
            },
            left_ar_skin: Skin {
                button_style: left_ar_button_style,
                ..root_ui.default_skin()
            },
            right_ar_skin: Skin {
                button_style: right_ar_button_style,
                ..root_ui.default_skin()
            },
            numbers_button_skin: Skin {
                button_style: numbers_button_style,
                ..root_ui.default_skin()
            },
            big_button_skin: Skin {
                button_style: big_button_style,
                ..root_ui.default_skin()
            },
            button_arrow_skin: Skin {
                button_style: button_arrow_style,
                ..root_ui.default_skin()
            },
            game_skin: Skin {
                window_style,
                button_style: button_game_style,
                ..root_ui.default_skin()
            }
        }
    }
}
