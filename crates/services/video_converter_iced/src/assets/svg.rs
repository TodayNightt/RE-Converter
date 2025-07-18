use crate::assets::Assets;
use iced::widget::Svg;
use iced::widget::svg::Handle;
use std::sync::LazyLock;

macro_rules! svg {
    ($static_name : ident , $file_name : expr, $fn_name : ident) => {
        static $static_name: LazyLock<Handle> = LazyLock::new(|| {
            let svg_data = Assets::get($file_name).unwrap();
            Handle::from_memory(svg_data.data)
        });

        pub fn $fn_name() -> Svg<'static> {
            Svg::new($static_name.clone()).width(iced::Shrink)
        }
    };
}

svg!(BACK_ARROW, "back.svg", back_arrow);
