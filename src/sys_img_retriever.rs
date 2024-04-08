use minidom::Element;
use ureq::get;

/*
const LINK1: &str = "https://dl.google.com/android/repository/sys-img/android/sys-img2-1.xml";
const LINK2: &str = "https://dl.google.com/android/repository/sys-img/android/sys-img2-2.xml";
const LINK3: &str = "https://dl.google.com/android/repository/sys-img/android/sys-img2-3.xml";
*/

const MICROG: &str = "https://microg.org/sdk/sys-img.xml";

pub async fn download_sdk_lists() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Create new client");
    let file1 = download_sdk_list(MICROG).await?;
    if !file1.starts_with("ERROR") {
        let element: Element = file1.parse().unwrap();
        println!("{}", element.name());
    }

    Ok(())
}

async fn download_sdk_list(link: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("Creating res");
    let res = get(link)
        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
        .call()?;
    println!("Created res");

    let out = res.into_string()?;

    /*
    let total_size = res
        .header("Content-Length")
        .ok_or(format!("ERROR: Failed to get content length from '{}'", &link))?;
    let total_size = total_size.parse::<u64>()?;

    let base_path = link.split('/').last().unwrap();
    let path = temp_dir().as_path().to_str().unwrap().to_owned() + "/" + base_path;

    println!("{}: {}", path, total_size);

    let mut file = File::create(&path)
        .or(Err(format!("ERROR: Failed to create file '{}'", &path)))?;
    file.write_all(out.as_bytes())
        .or(Err(format!("ERROR: Failed while writing to file")))?;
    */

    Ok(out)
}
