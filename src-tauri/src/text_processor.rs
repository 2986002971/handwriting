use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TextSettings {
    font_sizes: Vec<f32>,
    font_names: Vec<String>,
    line_spacing: i32,
    position_offset: f32,
    spacing_offset: f32,
}

#[tauri::command]
pub fn process_text(text: &str, settings: TextSettings) -> String {
    let mut rng = rand::thread_rng();
    let mut html_parts = Vec::new();

    // 处理文本内容
    for c in text.chars() {
        // 随机选择字体和大小
        let font_name = select_font_for_char(&c, &settings.font_names, &mut rng);
        let font_size = settings.font_sizes[rng.gen_range(0..settings.font_sizes.len())];

        // 随机样式
        let is_italic = rng.gen_ratio(1, 13);
        let is_bold = rng.gen_ratio(1, 27);

        // 随机偏移量
        let position_offset = rng.gen_range(-1.0..1.0) + settings.position_offset;
        let spacing_offset = rng.gen_range(-1.0..1.0) + settings.spacing_offset;

        // 生成带样式的HTML
        let span = format!(
            r#"<span style="
                display: inline-block;
                font-family: '{}';
                font-size: {}px;
                font-style: {};
                font-weight: {};
                position: relative;
                top: {}px;
                letter-spacing: {}px;
            ">{}</span>"#,
            font_name,
            font_size,
            if is_italic { "italic" } else { "normal" },
            if is_bold { "bold" } else { "normal" },
            position_offset,
            spacing_offset,
            c
        );

        html_parts.push(span);
    }

    // 处理段落格式
    let paragraphs: Vec<&str> = text.split('\n').collect();
    let mut formatted_paragraphs = Vec::new();

    for p in paragraphs {
        let mut html_parts = Vec::new();

        // 为每个段落的字符生成样式
        for c in p.chars() {
            let font_name = select_font_for_char(&c, &settings.font_names, &mut rng);
            let font_size = settings.font_sizes[rng.gen_range(0..settings.font_sizes.len())];

            // 随机样式
            let is_italic = rng.gen_ratio(1, 13);
            let is_bold = rng.gen_ratio(1, 27);

            // 随机偏移量
            let position_offset = rng.gen_range(-1.0..1.0) + settings.position_offset;
            let spacing_offset = rng.gen_range(-1.0..1.0) + settings.spacing_offset;

            // 生成带样式的HTML
            let span = format!(
                r#"<span style="
                    display: inline-block;
                    font-family: '{}';
                    font-size: {}px;
                    font-style: {};
                    font-weight: {};
                    position: relative;
                    top: {}px;
                    letter-spacing: {}px;
                ">{}</span>"#,
                font_name,
                font_size,
                if is_italic { "italic" } else { "normal" },
                if is_bold { "bold" } else { "normal" },
                position_offset,
                spacing_offset,
                c
            );

            html_parts.push(span);
        }

        let line_height = rng.gen_range(0..5) as f32 + settings.line_spacing as f32;
        let paragraph = format!(
            r#"<p style="
                line-height: {}px;
                text-indent: 2em;
                margin: 0;
                padding: 0;
            ">{}</p>"#,
            line_height,
            html_parts.join("")
        );
        formatted_paragraphs.push(paragraph);
    }

    // 返回完整的HTML
    format!(
        r#"<div style="
            font-size: 18px;
            white-space: pre-wrap;
            word-break: break-all;
        ">{}</div>"#,
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
