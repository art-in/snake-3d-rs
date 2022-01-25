use wasm_bindgen::JsValue;

use crate::models::Size;

pub fn get_canvas_font_string(
    size: Option<u32>,
    family: Option<&str>,
    unit: Option<&str>,
    weight: Option<&str>,
) -> String {
    let size = size.unwrap_or(8);
    let family = family.unwrap_or("Arial");
    let unit = unit.unwrap_or("px");

    let weight_str;

    if weight.is_some() {
        weight_str = format!("{} ", weight.unwrap());
    } else {
        weight_str = "".to_string();
    }

    format!("{weight_str}{size}{unit} {family}")
}

pub fn resize_canvas(
    canvas: &web_sys::HtmlCanvasElement,
    css_size: Size,
    pixel_ratio: f64,
) -> Result<(), JsValue> {
    canvas.set_width((css_size.width * pixel_ratio) as u32);
    canvas.set_height((css_size.height * pixel_ratio) as u32);

    let style = &canvas.style();
    style.set_property("width", &format!("{}px", css_size.width))?;
    style.set_property("height", &format!("{}px", css_size.height))?;

    Ok(())
}

pub fn measure_canvas_text(
    canvas_ctx_2d: &web_sys::CanvasRenderingContext2d,
    text: &str,
) -> Result<Size, JsValue> {
    let text_metrics = canvas_ctx_2d.measure_text(text)?;

    Ok(Size {
        width: text_metrics.width().ceil(),
        height: text_metrics.actual_bounding_box_ascent().ceil(),
    })
}

#[cfg(test)]
mod tests {
    mod get_canvas_font_string {
        use super::super::*;

        #[test]
        fn defaults() {
            let actual = get_canvas_font_string(None, None, None, None);
            let expected = "8px Arial";
            assert_eq!(actual, expected);
        }

        #[test]
        fn basic() {
            let actual = get_canvas_font_string(Some(2), Some("Times"), Some("rem"), Some("bold"));
            let expected = "bold 2rem Times";
            assert_eq!(actual, expected);
        }

        #[test]
        fn with_strings() {
            let family = String::from("Consolas");
            let unit = format!("{}{}", "e", "m");
            let actual = get_canvas_font_string(None, Some(&family), Some(&unit), None);
            let expected = "8em Consolas";
            assert_eq!(actual, expected);
        }
    }
}
