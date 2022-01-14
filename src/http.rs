use std::io::{ Write, Read };
use std::time::{ Duration };
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::{ Sender };
use std::sync::atomic::{ AtomicBool, Ordering };
use url::{ Url };
use rand::seq::SliceRandom;
use rand::{ Rng };
use reqwest::header;
use crate::data::{ GraphData, DownloadData };

// BUFFER_SIZE = 1Kb
const BUFFER_SIZE: usize = 1 << 10;

fn http_get(url: &Url) -> Result<reqwest::blocking::Response, String> {
    let u: Url = url.clone();
    //user agent
    let user_agent = random_user_agent();

    // headers
    let mut headers = header::HeaderMap::new();

    // *experimental
    // experimenting to avoid block by instagram's rate limiter

    // experimental
    headers.insert("Referer", header::HeaderValue::from_static("https://www.google.co.uk/"));

    // experimental
    // proxy
    let proxy = match reqwest::Proxy::http("51.195.188.28:9090") {
        Ok(p) => p,
        Err(_) => return Err(String::from("error initialize proxy"))
    };

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .connection_verbose(true)
        .default_headers(headers)
        .user_agent(user_agent)
        .proxy(proxy)
        .build() {
            Ok(client) => client,
            Err(_) => return Err(String::from("error initialize client"))
        };

    let response = match client.get(u).send() {
        Ok(response) => response,
        Err(e) => {
            println!("{}", e);
            return Err(String::from("error performing request"));
        }
    };

    Ok(response)
}

// get_download_data
// a helper function for getting image or video URL
fn get_download_data(url: &Url) -> Result<DownloadData, String> {
    let response = match http_get(url) {
        Ok(response) => response,
        Err(_) => return Err(String::from("error performing request"))
    };

    if response.status().as_u16() != 200 {
        return Err(String::from(format!("error performing request, request return {}", 
            response.status().as_u16())));
    }

    let graph_data = match response.json::<GraphData>() {
        Ok(graph_data) => graph_data,
        Err(e) => {
            println!("{}", e);
            return Err(String::from("error parsing response data"));
        }
    };

    // download
    let download_data = match graph_data.download_url() {
        Ok(download_data) => download_data,
        Err(_) => return Err(String::from("error getting download url"))
    };

    Ok(download_data)
}

// download to channel's Sender
// a download function that support multithreading
// this function will send buffer chunk each (with BUFFER_SIZE size)
// to : out Sender<Vec<u8>> channel
pub fn download_to_tx(url: &Url, out: Sender<Vec<u8>>, 
    file_type: Sender<String>, 
    done: Arc<AtomicBool>) -> Result<(), String> {

    let download_data = match get_download_data(url) {
        Ok(download_data) => download_data,
        Err(_) => return Err(String::from("error getting download url"))
    };

    // send file type
    if download_data.is_video {
        let file_type_s = String::from(".mp4");
        if let Err(e) = file_type.send(file_type_s) {
            return Err(format!("error send file type {}", e));
        }
    } else {
        let file_type_s = String::from(".jpg");
        if let Err(e) = file_type.send(file_type_s) {
            return Err(format!("error send file type {}", e));
        }
    }

    let parsed_download_url = match Url::parse(&download_data.download_url) {
        Ok(parsed_download_url) => parsed_download_url,
        Err(_) => return Err(String::from("error parsing download url"))
    };

    let mut response_download = match http_get(&parsed_download_url) {
        Ok(response) => response,
        Err(_) => return Err(String::from("error performing download request"))
    };

    if response_download.status().as_u16() != 200 {
        return Err(String::from(format!("error performing download request, request return {}", 
            response_download.status().as_u16())));
    }

    let mut buffer = vec![0 as u8; BUFFER_SIZE];

    loop {
        let line_read = match response_download.read(&mut buffer[..]) {
            Ok(o) => o,
            Err(e) => return Err(format!("error read response_download {}", e))   
        };

        if line_read <= 0 {
            break;
        }
        
        // send chunk
        // create buffer copy from original buffer read from response
        let mut buffer_copy = vec![0 as u8; line_read];

        // copy buffer and send it to the channel sender
        buffer_copy[..line_read].copy_from_slice(&buffer[..line_read]);
        if let Err(e) = out.send(buffer_copy) {
            return Err(format!("error sending buffer to channel sender {}", e));
        }
    }

    done.store(true, Ordering::Relaxed);

    Ok(())
}

// download to io writer
pub fn download_to_writer(url: &Url, out: &mut impl Write, 
    file_type: Arc<Mutex<String>>) -> Result<(), String> {

    let download_data = match get_download_data(url) {
        Ok(download_data) => download_data,
        Err(_) => return Err(String::from("error getting download url"))
    };

    let parsed_download_url = match Url::parse(&download_data.download_url) {
        Ok(parsed_download_url) => parsed_download_url,
        Err(_) => return Err(String::from("error parsing download url"))
    };

    let mut response_download = match http_get(&parsed_download_url) {
        Ok(response) => response,
        Err(_) => return Err(String::from("error performing download request"))
    };

    if response_download.status().as_u16() != 200 {
        return Err(String::from(format!("error performing download request, request return {}", 
            response_download.status().as_u16())));
    }

    let mut buffer = vec![0 as u8; BUFFER_SIZE];

    loop {
        let line_read = match response_download.read(&mut buffer[..]) {
            Ok(o) => o,
            Err(e) => return Err(format!("error read response_download {}", e))   
        };

        if line_read <= 0 {
            break;
        }
        
        if let Err(e) = out.write(&buffer[..line_read]) {
            return Err(format!("error write buffer to out {}", e));
        }
    }

    if download_data.is_video {
        let mut file_type_u = file_type.lock().unwrap();
        file_type_u.clear();
        file_type_u.push_str(".mp4");
    } else {
        let mut file_type_u = file_type.lock().unwrap();
        file_type_u.clear();
        file_type_u.push_str(".jpg");
    }

    Ok(())
}

fn random_user_agent() -> String {
    let browsers = vec!["Firefox", "Safari", "Opera", "Flock", "Internet Explorer", "Seamonkey", "Tor Browser", "GNU IceCat", "CriOS", "TenFourFox",
        "SeaMonkey", "B-l-i-t-z-B-O-T", "Konqueror", "Mobile", "Konqueror", "Netscape", "Chrome", "Dragon", "SeaMonkey", "Maxthon", "IBrowse",
        "K-Meleon", "GoogleBot", "Konqueror", "Minimo", "Googlebot", "WeltweitimnetzBrowser", "SuperBot", "TerrawizBot", "YodaoBot", "Wyzo", "Grail",
        "PycURL", "Galaxy", "EnigmaFox", "008", "ABACHOBot", "Bimbot", "Covario IDS", "iCab", "KKman", "Oregano", "WorldWideWeb", "Wyzo", "GNU IceCat",
        "Vimprobable", "uzbl", "Slim Browser", "Flock", "OmniWeb", "Rockmelt", "Shiira", "Swift", "Pale Moon", "Camino", "Flock", "Galeon", "Sylera"];

    let operating_systems = vec!["Windows 3.1", "Windows 95", "Windows 98", "Windows 2000", "Windows NT", "Linux 2.4.22-10mdk", "FreeBSD",
        "Windows XP", "Windows Vista", "Redhat Linux", "Ubuntu", "Fedora", "AmigaOS", "BackTrack Linux", "iPad", "BlackBerry", "Unix",
        "CentOS Linux", "Debian Linux", "Macintosh", "Android", "iPhone", "Windows NT 6.1", "BeOS", "OS 10.5", "Nokia", "Arch Linux",
        "Ark Linux", "BitLinux", "Conectiva (Mandriva)", "CRUX Linux", "Damn Small Linux", "DeLi Linux", "Ubuntu", "BigLinux", "Edubuntu",
        "Fluxbuntu", "Freespire", "GNewSense", "Gobuntu", "gOS", "Mint Linux", "Kubuntu", "Xubuntu", "ZeVenOS", "Zebuntu", "DemoLinux",
        "Dreamlinux", "DualOS", "eLearnix", "Feather Linux", "Famelix", "FeniX", "Gentoo", "GoboLinux", "GNUstep", "Insigne Linux",
        "Kalango", "KateOS", "Knoppix", "Kurumin", "Dizinha", "TupiServer", "Linspire", "Litrix", "Mandrake", "Mandriva", "MEPIS",
        "Musix GNU Linux", "Musix-BR", "OneBase Go", "openSuSE", "pQui Linux", "PCLinuxOS", "Plaszma OS", "Puppy Linux", "QiLinux",
        "Red Hat Linux", "Red Hat Enterprise Linux", "CentOS", "Fedora", "Resulinux", "Rxart", "Sabayon Linux", "SAM Desktop", "Satux",
        "Slackware", "GoblinX", "Slax", "Zenwalk", "SuSE", "Caixa Mágica", "HP-UX", "IRIX", "OSF/1", "OS-9", "POSYS", "QNX", "Solaris",
        "OpenSolaris", "SunOS", "SCO UNIX", "Tropix", "EROS", "Tru64", "Digital UNIX", "Ultrix", "UniCOS", "UNIflex", "Microsoft Xenix",
        "z/OS", "Xinu", "Research Unix", "InfernoOS"];

    let locals = vec!["cs-CZ", "en-US", "sk-SK", "pt-BR", "sq_AL", "sq", "ar_DZ", "ar_BH", "ar_EG", "ar_IQ", "ar_JO",
        "ar_KW", "ar_LB", "ar_LY", "ar_MA", "ar_OM", "ar_QA", "ar_SA", "ar_SD", "ar_SY", "ar_TN", "ar_AE", "ar_YE", "ar",
        "be_BY", "be", "bg_BG", "bg", "ca_ES", "ca", "zh_CN", "zh_HK", "zh_SG", "zh_TW", "zh", "hr_HR", "hr", "cs_CZ", "cs",
        "da_DK", "da", "nl_BE", "nl_NL", "nl", "en_AU", "en_CA", "en_IN", "en_IE", "en_MT", "en_NZ", "en_PH", "en_SG", "en_ZA",
        "en_GB", "en_US", "en", "et_EE", "et", "fi_FI", "fi", "fr_BE", "fr_CA", "fr_FR", "fr_LU", "fr_CH", "fr", "de_AT", "de_DE",
        "de_LU", "de_CH", "de", "el_CY", "el_GR", "el", "iw_IL", "iw", "hi_IN", "hu_HU", "hu", "is_IS", "is", "in_ID", "in", "ga_IE",
        "ga", "it_IT", "it_CH", "it", "ja_JP", "ja_JP_JP", "ja", "ko_KR", "ko", "lv_LV", "lv", "lt_LT", "lt", "mk_MK", "mk", "ms_MY",
        "ms", "mt_MT", "mt", "no_NO", "no_NO_NY", "no", "pl_PL", "pl", "pt_PT", "pt", "ro_RO", "ro", "ru_RU", "ru", "sr_BA", "sr_ME",
        "sr_CS", "sr_RS", "sr", "sk_SK", "sk", "sl_SI", "sl", "es_AR", "es_BO", "es_CL", "es_CO", "es_CR", "es_DO", "es_EC", "es_SV",
        "es_GT", "es_HN", "es_MX", "es_NI", "es_PA", "es_PY", "es_PE", "es_PR", "es_ES", "es_US", "es_UY", "es_VE", "es", "sv_SE",
        "sv", "th_TH", "th_TH_TH", "th", "tr_TR", "tr", "uk_UA", "uk", "vi_VN", "vi"];

    let user_agent = format!("{}/{}.{} ({} {}.{}; {};)", 
            browsers.choose(&mut rand::thread_rng()).unwrap(), 
            rand::thread_rng().gen_range(1..19), 
            rand::thread_rng().gen_range(0..20), 
            operating_systems.choose(&mut rand::thread_rng()).unwrap(),
            rand::thread_rng().gen_range(1..6),
            rand::thread_rng().gen_range(0..9),
            locals.choose(&mut rand::thread_rng()).unwrap()
        );
    
    user_agent
}