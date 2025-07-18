#[macro_export]
macro_rules! radios {
    ($enum_type:expr, $current_value:expr, $message_constructor:expr, $layout:ident, $spacing : expr) => {{
        container(
            $layout(
                $enum_type
                    .into_iter()
                    .map(|item| radio(item, item, $current_value, |val| $message_constructor(val)))
                    .map(Element::from),
            )
            .spacing($spacing)
            .wrap(),
        )
    }};
}
