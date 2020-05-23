#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page = 7;
    let url = format!("http://openccpm.com/blog/?p={}", page);
    println!("call {}", &url);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    println!("response is \n{}", body);
    Ok(())
}
