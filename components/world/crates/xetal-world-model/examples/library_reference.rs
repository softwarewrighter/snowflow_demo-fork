//! Generate the live API/source cross-reference from the actual imported modules.
use xetal_snow_libraries::LIBRARIES;
use xetal_world_model::highlighted_text;

fn main() {
    let revision = std::env::args().nth(1).unwrap_or_else(|| "main".into());
    assert!(
        revision
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_/".contains(&b))
    );
    println!(
        "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width\"><title>Snowfield library reference</title><style>@font-face{{font-family:JuliaMono;src:url(https://cdnjs.cloudflare.com/ajax/libs/juliamono/0.63.2/JuliaMono-Regular.woff2)}}body{{max-width:1100px;margin:32px auto;padding:0 20px;background:#0b1421;color:#dce8f5;font:15px system-ui;line-height:1.6}}a{{color:#87cbe5}}pre{{overflow:auto;padding:20px;background:#08101a;font:13px/1.7 JuliaMono,monospace}}section{{padding:20px 0;border-top:1px solid #294258}}code{{font-family:JuliaMono,monospace}}.c-builtin{{color:#82b4ff}}.c-userfunc,.c-libfunc{{color:#7cdb95}}.c-number,.c-exponent{{color:#f0ce72}}.c-string{{color:#e8d89b}}.c-comment,.c-doc{{color:#9fc3ad}}.c-symbol{{color:#a1cfff}}.c-error{{color:#ff9999}}</style><body><a href=\"./\">Back to demo</a><h1>X_eTaL libraries used by the snowfield</h1><p>This page is generated from the exact imported modules. The program owns pure state updates and emits typed render/audio data; Rust supplies physics facts, execution scheduling, WebAudio and GPU resources.</p><p><a href=\"https://github.com/softwarewrighter/snowflow_demo-fork/blob/{revision}/docs/libraries.md\">Architecture and native backend cross-reference</a></p><nav>"
    );
    for library in LIBRARIES {
        println!(
            "<a href=\"#{}\">{}</a> &middot; ",
            library.anchor, library.name
        );
    }
    println!("</nav>");
    for library in LIBRARIES {
        section(library, &revision);
    }
    println!(
        "<footer>Copyright (c) 2026 Michael A Wright &middot; MIT License &middot; <a href=\"https://github.com/softwarewrighter/snowflow_demo-fork\">Repository</a> &middot; Source {revision}</footer></body></html>"
    );
}

fn section(library: &xetal_snow_libraries::Library, revision: &str) {
    let base = format!(
        "https://github.com/softwarewrighter/snowflow_demo-fork/blob/{revision}/{}",
        library.path
    );
    println!(
        "<section id=\"{}\"><h2>{}</h2><p><a href=\"{base}\">X_eTaL source</a> &middot; alias <code>{}:</code></p><h3>Public functions</h3><ul>",
        library.anchor, library.name, library.alias
    );
    for (line, text) in library.source.lines().enumerate() {
        if let Some(function) = text
            .strip_prefix("l:")
            .and_then(|line| line.split_once(" :=").map(|pair| pair.0))
        {
            println!(
                "<li><a href=\"{base}#L{}\"><code>{}:{function}</code></a></li>",
                line + 1,
                library.alias
            );
        }
    }
    println!(
        "</ul><h3>Source as rendered</h3><pre>{}</pre></section>",
        highlighted_text(library.source)
    );
}
