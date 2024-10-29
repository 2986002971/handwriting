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
    line_spacing_random: f32,
    spacing_offset: f32,
    spacing_offset_random: f32,
    italic_probability: i32,
    bold_probability: i32,
    indent_size: f32,
}

#[derive(Serialize)]
pub struct FontData {
    name: String,
    data: Vec<u8>,
}

#[tauri::command]
pub fn process_text(text: &str, settings: TextSettings) -> String {
    let mut rng = rand::thread_rng();

    let paragraphs: Vec<&str> = text.split('\n').collect();
    let formatted_paragraphs: Vec<String> = paragraphs
        .iter()
        .map(|p| {
            let chars: String = p.chars()
                .map(|c| {
                    let font_name = select_font_for_char(&c, &settings.font_names, &mut rng);
                    let font_size = settings.font_sizes[rng.gen_range(0..settings.font_sizes.len())];
                    let is_italic = rng.gen_ratio(1, settings.italic_probability as u32);
                    let is_bold = rng.gen_ratio(1, settings.bold_probability as u32);
                    
                    let random_spacing = rng.gen_range(-settings.spacing_offset_random..=settings.spacing_offset_random);
                    let final_spacing = settings.spacing_offset + random_spacing;
                    
                    format!(
                        r#"<span style="font-family:'{}';font-size:{}px;font-style:{};font-weight:{};letter-spacing:{}px">{}</span>"#,
                        font_name,
                        font_size,
                        if is_italic { "italic" } else { "normal" },
                        if is_bold { "bold" } else { "normal" },
                        final_spacing,
                        c
                    )
                })
                .collect();

            let random_line_spacing = rng.gen_range(-settings.line_spacing_random..=settings.line_spacing_random);
            let final_line_spacing = (settings.line_spacing as f32 + random_line_spacing).max(0.0);
            
            format!(
                r#"<p style="line-height:{}px;text-indent:{}em;margin:0;padding:0">{}</p>"#,
                final_line_spacing,
                settings.indent_size,
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
