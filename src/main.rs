use pulldown_cmark::{html, Options, Parser};
use std::fs::{read_to_string, write};

fn main() {
    // Read the Markdown file
    let markdown_content = read_to_string("input.md").expect("Failed to read the Markdown file");

    // Parse Markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown_content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Create dark mode CSS style
    let dark_mode_style = r#"
        <style id="theme-style">
            body {
                background-color: #1E1E1E; /* Dark background color */
                color: #FFFFFF; /* Light text color */
            }
        </style>
    "#;

    // Create light mode CSS style
    let light_mode_inner = r#"
            body {
                background-color: #FFFFFF; /* Light background color */
                color: #000000; /* Dark text color */
            }
    "#;

     // Create dark mode CSS style
     let dark_mode_inner = r#"
         body {
             background-color: #1E1E1E; /* Dark background color */
             color: #FFFFFF; /* Light text color */
         }
 "#;


    // Generate HTML with dark mode style and JavaScript toggle function
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            {}
        </head>
        <body>
            <button id="toggle-button">Toggle Mode</button>
            {}
            <script>
                document.addEventListener('DOMContentLoaded', function() {{
                    const themeStyle = document.getElementById('theme-style');

                    function setMode(mode) {{
                        themeStyle.innerHTML = mode === 'dark' ? `{}` : `{}`; // Dark and Light mode CSS
                        themeStyle.setAttribute('data-mode', mode);
                    }}

                    function toggleMode() {{
                        const isDarkMode = themeStyle.getAttribute('data-mode') === 'dark';
                        setMode(isDarkMode ? 'light' : 'dark');
                    }}

                    setMode('dark'); // Set the initial mode, you can change this to 'light' if needed

                    document.getElementById('toggle-button').addEventListener('click', toggleMode);
                }});
            </script>
        </body>
        </html>
    "#,
        dark_mode_style,
        html_output,
        dark_mode_inner,
        light_mode_inner,
    );

    // Write the HTML to a file
    write("output.html", html_content).expect("Failed to write the HTML file");
}
