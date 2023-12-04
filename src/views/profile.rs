use html_to_string_macro::html;

pub fn body() -> String {
    html! {
        <h1>"Hello World!"</h1>
    }
}