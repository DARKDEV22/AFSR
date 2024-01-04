// cargo run --release
use std::io;
use std::fs;
use std::process;
use reqwest::header;
use std::io::Cursor;
extern crate reqwest;
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


// fn get_ts(base_url: &str, key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert("authority", "live-global-cdn-v02.afreecatv.com".parse().unwrap());
//     headers.insert("accept", "*/*".parse().unwrap());
//     headers.insert("accept-language", "en-US,en;q=0.8".parse().unwrap());
//     headers.insert("origin", "https://play.afreecatv.com".parse().unwrap());
//     headers.insert("referer", "https://play.afreecatv.com/".parse().unwrap());
//     headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Brave\";v=\"120\"".parse().unwrap());
//     headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
//     headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
//     headers.insert("sec-fetch-dest", "empty".parse().unwrap());
//     headers.insert("sec-fetch-mode", "cors".parse().unwrap());
//     headers.insert("sec-fetch-site", "same-site".parse().unwrap());
//     headers.insert("sec-gpc", "1".parse().unwrap());
//     headers.insert("user-agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".parse().unwrap());

//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let res = client.get(&format!("{}?aid={}", base_url, key))
//         .headers(headers)
//         .send()?
//         .text()?;

//     let ts : Vec<String> = res
//                     .split("1920x1080")
//                     .skip(1)
//                     .flat_map(|z| z.split("#EXTINF").next())
//                     .map(|url| url.replace("\r\n", ""))
//                     .collect();
//     Ok(ts)
// }

// fn page_source(url: &str) -> Result<String, reqwest::Error> {
//     // Replace the URL with the actual URL of the page you want to retrieve
//     // Send an HTTP GET request
//     let response = reqwest::blocking::get(&format!("https://play.afreecatv.com/{}", url))?;
//     let html = response.text().unwrap();
//     let s: Vec<_> = html.split("liveimg.afreecatv.com/m/").collect();
//     let res: Vec<_> = s[1].split("?").collect();
//     let number = res[0];

//     Ok(number.to_string())
// }

// fn page_response(bj_id: &str, number_id: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert("authority", "api.m.afreecatv.com".parse().unwrap());
//     headers.insert("accept", "*/*".parse().unwrap());
//     headers.insert("accept-language", "en-US,en;q=0.8".parse().unwrap());
//     headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());
//     headers.insert("origin", "https://m.afreecatv.com".parse().unwrap());
//     headers.insert("referer", "https://m.afreecatv.com/".parse().unwrap());
//     headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Brave\";v=\"120\"".parse().unwrap());
//     headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
//     headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
//     headers.insert("sec-fetch-dest", "empty".parse().unwrap());
//     headers.insert("sec-fetch-mode", "cors".parse().unwrap());
//     headers.insert("sec-fetch-site", "same-site".parse().unwrap());
//     headers.insert("sec-gpc", "1".parse().unwrap());
//     // headers.insert("user-agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".parse().unwrap());

//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let res = client.post(&format!("https://api.m.afreecatv.com/broad/a/watch?bjid={}", bj_id))
//         .headers(headers)
//         .body(format!("bj_id={}&broad_no={}&agent=web&confirm_adult=false&player_type=webm&mode=live", bj_id, number_id))
//         .send()?
//         .text()?;
//     // let key: Vec<_> = res.split("hls_authentication_key").collect();
//     // let s: Vec<_> = key[1].split(",").collect();
//     // let keya: Vec<_> = s[1].split("\"").collect();
//     // Ok(keya[3].to_string())
//     let key = res
//                             .split("hls_authentication_key")
//                             .nth(1)
//                             .and_then(|s| s.split(",").nth(1))
//                             .and_then(|key| key.split("\"").nth(3))
//                             .map(|key| key.to_string());

//     Ok(key.unwrap_or_else(|| String::new()))

// }

// fn get_base_url(number_id: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert("authority", "livestream-manager.afreecatv.com".parse().unwrap());
//     headers.insert("accept", "*/*".parse().unwrap());
//     headers.insert("accept-language", "en-US,en;q=0.8".parse().unwrap());
//     headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());
//     headers.insert("origin", "https://m.afreecatv.com".parse().unwrap());
//     headers.insert("referer", "https://m.afreecatv.com/".parse().unwrap());
//     headers.insert("sec-ch-ua", "\"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Brave\";v=\"120\"".parse().unwrap());
//     headers.insert("sec-ch-ua-mobile", "?1".parse().unwrap());
//     headers.insert("sec-ch-ua-platform", "\"Android\"".parse().unwrap());
//     headers.insert("sec-fetch-dest", "empty".parse().unwrap());
//     headers.insert("sec-fetch-mode", "cors".parse().unwrap());
//     headers.insert("sec-fetch-site", "same-site".parse().unwrap());
//     headers.insert("sec-gpc", "1".parse().unwrap());
//     // headers.insert("user-agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".parse().unwrap());

//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let res = client.get(&format!("https://livestream-manager.afreecatv.com/broad_stream_assign.html?return_type=gcp_cdn&use_cors=true&cors_origin_url=m.afreecatv.com&broad_key={}-common-hd-hls", number_id))
//         .headers(headers)
//         .send()?
//         .text()?;
//     let response: ApiResponse = serde_json::from_str(&res)?;
//     Ok(response.view_url)
// }


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
    // let bj_id = "hl6260";
    // let m3u8_url = "https://live-global-cdn-v02.afreecatv.com/live-stm-16/auth_playlist.m3u8?aid=.A32.7bbT56vyHM9fKZk.5J9f_GP2vBGChB5WHsB2EcNkA9M0nZFm2u1eWJ11ZnE3LcLr7iTGUBf-mzVfE85-QqlYdO_nxSzaEoeVd5fN1sXmoBrHKHXkAriuMhQX4mGGYy6FOPOz_jhbxNh8keMa".to_string();
    // let number_id = page_source(bj_id).unwrap();
    // println!("{:?}", number_id);
    // let key: String = page_response(bj_id, number_id.as_str()).unwrap();
    // println!("{:?}", key);
    // let base_url = get_base_url(number_id.as_str()).unwrap();
    // let base_url = "https://live-global-cdn-v02.afreecatv.com/live-stmc-37/auth_playlist.m3u8";
    // let key: String = ".A32.7bbT56vyHM9fKZk.NmMjFKcd_SLAfN1ablFtbgdMOKZi8vipQYI16ZIHN2r05cUWkBDaMDhuMdGrnHwlnH0NuP4i9PaJdwBHpmGLQlnLjYJkOlQ4htew3t96WurhwdIoRbKeCFlvo85X2mMU".to_string();

    // get m3u8 url, now just loop til stream end.

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
