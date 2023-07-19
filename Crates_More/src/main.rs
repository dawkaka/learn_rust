use regex::Regex;
use std::collections::HashMap;
use Crates_More::mix;
use Crates_More::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let ms = mix(red, yellow);
    println!("{:?}", ms);
    let original_url = "http://localhost:3000/todos/{{$.stages[0].outputs.todoItem}}".to_string();
    let mut outputs = HashMap::new();
    outputs.insert("todoItem".to_string(), "example_todo".to_string());

    let formatted_url = format_url(&original_url, &outputs);
    println!("Formatted URL: {}", formatted_url);
}

fn format_url(original_url: &String, outputs: &HashMap<String, String>) -> String {
    let re = Regex::new(r"\{\{(.*?)\}\}").unwrap_or_else(|err| panic!("{}", err));
    let mut url = original_url.clone();

    for caps in re.captures_iter(original_url) {
        if let Some(matched_string) = caps.get(1) {
            let st = matched_string.as_str().to_string();
            let elements: Vec<&str> = st.split('.').collect();
            if let Some(output_variable) = elements.last() {
                if let Some(value) = outputs.get(&output_variable.to_string()) {
                    let repl = format!("{{{{{}}}}}", st);
                    url = url.replace(&repl, &value);
                }
            }
        }
    }
    url
}
