#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/blog/";
    println!("call {}", url);
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    println!("response is \n{}", body);
    Ok(())
}
