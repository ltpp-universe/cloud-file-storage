use bin_encode_decode::*;
use hyperlane::*;
use rand::TryRngCore;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs::metadata;

static FILE_DIR: &str = "/home/cloud_storage/file";
static LOG_DIR: &str = "/home/cloud_storage/logs";
static FILE_MAX_SIZE: usize = 1_048_576;
static CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=";
static PROTOCOL_DOMAIN_NAME: &str = "";
static FILE_NAME_KEY: &str = "file_name";
static FILE: &[u8; 11734] = include_bytes!("../index.html");

async fn get_file_full_path(req_file_path: &str) -> String {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let year: String = now.format("%Y").to_string();
    let month: String = now.format("%m").to_string();
    let day: String = now.format("%d").to_string();
    let hour: String = now.format("%H").to_string();
    let minute: String = now.format("%M").to_string();
    let full_dir: String = format!(
        "{}/{}/{}/{}/{}/{}",
        FILE_DIR, year, month, day, hour, minute
    );
    let full_path: String = format!("{}{}", full_dir, req_file_path);
    let dir_path: PathBuf = PathBuf::from(&full_dir);
    let _ = tokio::fs::create_dir_all(&dir_path).await;
    full_path
}

fn replace_prefix_with_hash(input: &str) -> String {
    let (prefix, suffix) = input.rsplit_once('.').unwrap_or((input, ""));
    let mut salt: [u8; 64] = [0u8; 64];
    let _ = rand::rng().try_fill_bytes(&mut salt);
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(salt);
    let hash = hex::encode(hasher.finalize());
    format!("{}.{}", hash, suffix)
}

fn get_json_string(code: usize, msg: &str, data: &str) -> String {
    format!(
        "{{\"code\":{},\"msg\":\"{}\",\"data\":\"{}\"}}",
        code, msg, data
    )
}

async fn resp_json(
    arc_lock_controller_data: &ControllerData,
    content_type: &str,
    code: usize,
    msg: &str,
    data: &str,
) {
    let json_string: String = get_json_string(code, msg, data);
    let host: String = arc_lock_controller_data
        .get_socket_addr()
        .await
        .unwrap_or(DEFAULT_SOCKET_ADDR)
        .to_string();
    let _ = arc_lock_controller_data
        .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_METHODS, GET_POST_OPTIONS)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .await
        .set_response_header(CONTENT_TYPE, format!("{}; {}", content_type, CHARSET_UTF_8))
        .await
        .send_response(200, json_string.clone())
        .await;
    arc_lock_controller_data
        .log_info(
            format!("{} resp_json => {}", host, json_string),
            log_handler,
        )
        .await;
}

async fn resp_bin(
    arc_lock_controller_data: &ControllerData,
    status_code: usize,
    content_type: &str,
    data: Vec<u8>,
) {
    let host: String = arc_lock_controller_data
        .get_socket_addr()
        .await
        .unwrap_or(DEFAULT_SOCKET_ADDR)
        .to_string();
    let _ = arc_lock_controller_data
        .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_METHODS, GET_POST_OPTIONS)
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .await
        .set_response_header(CONTENT_TYPE, format!("{}; {}", content_type, CHARSET_UTF_8))
        .await
        .send_response(status_code, data)
        .await;
    arc_lock_controller_data
        .log_info(
            format!(
                "{} resp_bin => content_type:{} status_code:{}",
                host, content_type, status_code
            ),
            log_handler,
        )
        .await;
}

async fn success_resp_json(
    arc_lock_controller_data: &ControllerData,
    content_type: &str,
    msg: &str,
    data: &str,
) {
    println_success!("success_resp_json", " => ", msg, " ", data);
    resp_json(arc_lock_controller_data, content_type, 1, msg, data).await;
}

async fn error_resp_json(
    arc_lock_controller_data: &ControllerData,
    content_type: &str,
    msg: &str,
    data: &str,
) {
    println_error!("error_resp_json", " => ", msg, " ", data);
    resp_json(arc_lock_controller_data, content_type, 0, msg, data).await;
}

async fn success_resp_bin(
    arc_lock_controller_data: &ControllerData,
    content_type: &str,
    data: Vec<u8>,
) {
    println_success!("success_resp_bin", " => ", content_type);
    resp_bin(arc_lock_controller_data, 200, content_type, data).await;
}

fn encode_file_full_path(path: &str) -> String {
    let (prefix, suffix) = path.rsplit_once('.').unwrap_or((path, ""));
    format!("{}.{}", encode(CHARSET, prefix).unwrap(), suffix)
}

fn decode_file_full_path(path: &str) -> String {
    let (prefix, suffix) = path.rsplit_once('.').unwrap_or((path, ""));
    format!("{}.{}", decode(CHARSET, prefix).unwrap(), suffix)
}

async fn file_middleware(arc_lock_controller_data: ControllerData) {
    let controller_data: InnerControllerData = arc_lock_controller_data.get().await;
    let path: &String = controller_data.get_request().get_path();
    let extension_name: String = FileExtension::get_extension_name(path);
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    if content_type.is_empty() {
        return;
    }
    let file_path: String = decode_file_full_path(path);
    if file_path.contains("?") {
        return;
    }
    if file_path.contains("../") {
        return error_resp_json(
            &arc_lock_controller_data,
            content_type,
            &format!("{} unsafe", FILE_NAME_KEY),
            "",
        )
        .await;
    }
    let body: Vec<u8> = async_read_from_file(&file_path).await.unwrap();
    success_resp_bin(&arc_lock_controller_data, content_type, body).await;
}

async fn index_file(arc_lock_controller_data: ControllerData) {
    let _ = arc_lock_controller_data.send_response(200, FILE).await;
}

async fn add_file(arc_lock_controller_data: ControllerData) {
    let controller_data: InnerControllerData = arc_lock_controller_data.get().await;
    let req: &Request = controller_data.get_request();
    let file_name_opt: Option<String> = req.get_query(FILE_NAME_KEY);
    if file_name_opt.is_none() {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            &format!("missing {}", FILE_NAME_KEY),
            "",
        )
        .await;
    }
    let file_name: String = replace_prefix_with_hash(&file_name_opt.unwrap());
    let file_name_path: String = format!("/{}", file_name);
    if file_name_path.contains("../") {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            &format!("{} unsafe", FILE_NAME_KEY),
            "",
        )
        .await;
    }
    let extension_name: String = FileExtension::get_extension_name(&file_name_path);
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    if content_type.is_empty() {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            "file type not supported",
            "",
        )
        .await;
    }
    let file_data: &Vec<u8> = req.get_body();
    let file_data_len: usize = file_data.len();
    if file_data_len == 0 {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            "file can not empty",
            "",
        )
        .await;
    }
    if file_data_len > FILE_MAX_SIZE {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            &format!("file size over {} bytes", FILE_MAX_SIZE),
            "",
        )
        .await;
    }
    let full_path: String = get_file_full_path(&file_name_path).await;
    if metadata(&full_path).await.is_ok() {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            "file already exist",
            "",
        )
        .await;
    }
    let write_res: Result<(), std::io::Error> = async_write_to_file(&full_path, file_data).await;
    if let Err(err) = write_res {
        return error_resp_json(
            &arc_lock_controller_data,
            APPLICATION_JSON,
            &format!("{:?}", err),
            "",
        )
        .await;
    }
    let encode_full_path: String = encode_file_full_path(&full_path);
    let encode_full_url: String = format!("{}/{}", PROTOCOL_DOMAIN_NAME, encode_full_path);
    success_resp_json(
        &arc_lock_controller_data,
        APPLICATION_JSON,
        "ok",
        &encode_full_url,
    )
    .await;
}

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60006).await;
    server.log_dir(LOG_DIR).await;
    server.request_middleware(file_middleware).await;
    server.route("/", index_file).await;
    server.route("/add_file", add_file).await;
    server.listen().await;
}

#[tokio::main]
async fn main() {
    run_server().await;
}
