use base64::{engine::general_purpose::STANDARD, Engine as _};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct TextSettings {
    font_sizes: Vec<f32>,
    font_names: Vec<String>,
    line_spacing: i32,
    position_offset: f32,
    spacing_offset: f32,
}

#[derive(Serialize)]
pub struct FontData {
    name: String,
    data: Vec<u8>,
}

#[tauri::command]
pub fn process_text(text: &str, settings: TextSettings) -> String {
    let mut rng = rand::thread_rng();

    // 处理段落
    let paragraphs: Vec<&str> = text.split('\n').collect();
    let formatted_paragraphs: Vec<String> = paragraphs
        .iter()
        .map(|p| {
            // 处理每个字符
            let chars: String = p.chars()
                .map(|c| {
                    let font_name = select_font_for_char(&c, &settings.font_names, &mut rng);
                    let font_size = settings.font_sizes[rng.gen_range(0..settings.font_sizes.len())];
                    let is_italic = rng.gen_ratio(1, 13);
                    let is_bold = rng.gen_ratio(1, 27);
                    let position_offset = rng.gen_range(-1.0..1.0) + settings.position_offset;
                    let spacing_offset = rng.gen_range(-1.0..1.0) + settings.spacing_offset;

                    format!(
                        r#"<span style="display:inline-block;font-family:'{}';font-size:{}px;font-style:{};font-weight:{};position:relative;top:{}px;letter-spacing:{}px">{}</span>"#,
                        font_name,
                        font_size,
                        if is_italic { "italic" } else { "normal" },
                        if is_bold { "bold" } else { "normal" },
                        position_offset,
                        spacing_offset,
                        c
                    )
                })
                .collect();

            let line_height = rng.gen_range(0..5) as f32 + settings.line_spacing as f32;
            format!(
                r#"<p style="line-height:{}px;text-indent:2em;margin:0;padding:0">{}</p>"#,
                line_height,
                chars
            )
        })
        .collect();

    format!(
        r#"<div style="font-size:18px;white-space:pre-wrap;word-break:break-all">{}</div>"#,
        formatted_paragraphs.join("\n")
    )
}

fn select_font_for_char(c: &char, fonts: &[String], rng: &mut impl rand::Rng) -> String {
    if "。，；''\"\"！？、：".contains(*c) {
        fonts[fonts.len() - 1].clone()
    } else if c.is_ascii_digit() {
        fonts[0].clone()
    } else if c.is_ascii_alphabetic()
        || *c == '.'
        || *c == '('
        || *c == ')'
        || *c == '（'
        || *c == '）'
    {
        fonts[fonts.len() - 1].clone()
    } else {
        fonts[rng.gen_range(0..fonts.len())].clone()
    }
}

#[tauri::command]
pub fn get_embedded_fonts() -> Vec<String> {
    let font_dir = PathBuf::from("src/fonts");

    fs::read_dir(font_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            
            if path.extension()?.to_str()? != "ttf" {
                return None;
            }
            
            let font_data = fs::read(&path).ok()?;
            let base64_font = STANDARD.encode(&font_data);
            let font_name = path.file_stem()?.to_string_lossy();
            
            Some(format!(
                "@font-face {{
                    font-family: '{}';
                    src: url(data:font/ttf;base64,{}) format('truetype');
                    font-display: block;
                }}",
                font_name, base64_font
            ))
        })
        .collect()
}
