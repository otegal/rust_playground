// use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page = 7;
    let url = format!("http://openccpm.com/unknown.txt?p={}", page);
    println!("call {}", &url);
    // let res = reqwest::get(&url).await?;
    // match res.status() {
    //     StatusCode::OK => {
    //         let body = res.text().await?;
    //         println!("response is \n{}", body);
    //     },
    //     StatusCode::NOT_FOUND => {
    //         println!("error: 目的のページがありませんでした。");
    //     },
    //     _ => {
    //         println!("error: その他のエラーが発生しました。");
    //     },
    // }

    if let Ok(res) = reqwest::get(&url).await {
        let body = res.text().await?;
        println!("response is \n {}", body);
    } else {
        println!("error: Not found web server.");
    }
    Ok(())
}
