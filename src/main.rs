use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("RAPIDAPI_KEY")?;
    let expr = env::args().nth(1).unwrap();

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("https://evaluate-expression.p.rapidapi.com")
        .query(&[("expression", &expr)])
        .header("x-rapidapi-key", api_key)
        .header("useQueryString", "true")
        .send()?
        .text()?;

    println!("{}", alphred::Workflow::new(&[alphred::Item::new(body)]));

    Ok(())
}
