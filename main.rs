extern crate reqwest;
use std::io;
use std::fs;
use std::process;
use reqwest::header;
use std::io::Cursor;
use std::thread;
use std::time::Duration;

fn get_ts(m3u8_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "live-global-cdn-v02.afreecatv.com".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.8".parse().unwrap());
    headers.insert("origin", "https://play.afreecatv.com".parse().unwrap());
    headers.insert("referer", "https://play.afreecatv.com/".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Brave\";v=\"120\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get(m3u8_url)
        .headers(headers)
        .send()?
        .text()?;

    let ts : Vec<String> = res
                    .split("1920x1080")
                    .skip(1)
                    .flat_map(|z| z.split("#EXTINF").next())
                    .map(|url| url.replace("\r\n", ""))
                    .collect();
    Ok(ts)
}

async fn download_file(download_url: &str, save_path: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>{
    let response = reqwest::get(download_url).await?;
    let file_size = response.content_length().unwrap_or(0);
    if file_size > 100000 {
        // if it found video not "not found" page
        let mut file = std::fs::File::create(save_path)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        return Ok(file_size);
    } 
    Ok(0)
}

fn create_dir(dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(dir_name)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("\nAFSR: AfreecaTV shadow recorder. [Github: Darkdev22]");
    let mut bj_id = String::new();
    println!("\nEnter a Bj_id:");
    io::stdin().read_line(&mut bj_id).expect("Failed to read line");
    let bj_id = bj_id.trim();

    let mut m3u8_url = String::new();
    println!("\nEnter m3u8_url:");
    io::stdin().read_line(&mut m3u8_url).expect("Failed to read line");
    let m3u8_url = m3u8_url.replace("\r\n", "");

    match get_ts(m3u8_url.as_str()) {
        Ok(_) => (),
        Err(_) => {
            println!("[Failed] Bj_id or m3u8_url is wrong!");
            thread::sleep(Duration::from_secs(5));
            process::exit(1);
        },
    }

    let _ = create_dir(bj_id);
    println!("\n\nRecording ... {}", bj_id);
    loop {
        let ts_urls = get_ts(m3u8_url.as_str()).unwrap();
        let base_url_ts: Vec<_> = m3u8_url.split("auth_playlist.m3u8").collect();
        let base_ts = base_url_ts[0].trim_end_matches('/');
        for url in ts_urls {
            let d = format!("{base_ts}{url}");
            let file_name: &str = url.split('_').nth(1).unwrap_or_default();
            println!("[{}] {}", file_name, bj_id);
            _ = download_file(&d, &format!("./{}/{}.ts",bj_id, file_name)).await;
        }
    }
}
