use leptos::*;
use leptos_meta::*;

use stylers::style_sheet_str;

#[component]
pub fn Multimedia() -> impl IntoView {
    let (style_scope, style_str) = style_sheet_str!("./styles/multimedia.css");

    view! { class=style_scope,
        <Style>{style_str}</Style>

        <div class="container">
            <img src="http://i.giphy.com/wTgYlmxctT2O4.webp" />
            <br />
            <video controls autoplay>
                // see https://www.webmfiles.org/demo-files/
                <source src="http://dl11.webmfiles.org/big-buck-bunny_trailer.webm" type="video/webm"></source>
            </video>
        </div>
    }
}
