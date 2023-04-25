use wasm_bindgen::prelude::*;
use sha1::{Sha1, Digest};
use serde::{Deserialize, Serialize};
use base64ct::{Base64, Encoding};
use chrono::{Local};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(inline_js = "export function get_local() {try { return JSON.stringify(localStorage) } catch { return ''; } }")]
extern "C" {
    fn get_local() -> String;
}

#[wasm_bindgen(inline_js = "export function get_browser_local() {try { var result={}; for(var n_attr in navigator) { var n_value = navigator[n_attr]; if(typeof n_value == 'string' || typeof n_value == 'boolean'){ result[n_attr.toLowerCase()]=n_value }}; return JSON.stringify(result); } catch(err) { return ''; } }" )]
extern "C" {
    fn get_browser_local() -> String;
}


#[derive(Serialize, Deserialize)]
struct LocalDataV0 {
    csrf: String,
}


#[derive(Serialize, Deserialize)]
struct LocalData {
    cst: String,
    csrf: String,
    time_diff: String,
}


#[derive(Serialize, Deserialize)]
struct SignResult {
    sign: String,
    timestamp: i64,
}

#[derive(Serialize, Deserialize)]
struct BrowserInfo {
    webdriver: bool,
    appversion: String,
    useragent: String,
}

struct CstResult {
    cst: String,
    version: String,
}


const CHECK_LEN: [usize;3] = [32,33,36];
const CHECK_STR: &str = "god_web";
const VERSION_STR_V1: &str = "1";


fn headless_detect() -> bool {
    let js_local_data1 = get_browser_local();
    let local_data1: BrowserInfo = match serde_json::from_str( &js_local_data1 ) {
        Ok(v) => v,
        Err(_) => return false,
    };

    if local_data1.appversion.find("Headless") != None || local_data1.useragent.find("Headless") != None {
        return true;
    }

    if local_data1.webdriver == true {
        return true;
    }

    return false;
}


fn parse_cst(cst: &str) -> Option<CstResult> {
    const BUF_SIZE: usize = 128;
    let mut dec_buf = [0u8; BUF_SIZE];
    let decoded = match Base64::decode(cst, &mut dec_buf) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let decode_cst = match String::from_utf8( decoded.to_vec() ) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let cst_slice = &decode_cst;
    if cst_slice.len() != CHECK_LEN[2] {
        return None;
    }
    else {
        let check_num = match cst_slice[CHECK_LEN[1]..CHECK_LEN[2]].parse::<usize>() {
            Ok(v) => v,
            Err(_) => return None,
        };
        let s_num: usize = check_num / 100;
        let e_num: usize = check_num % 100;
        if e_num > CHECK_LEN[0] || e_num < s_num {
            return None;
        }
        else {
            let version_str = cst_slice[CHECK_LEN[0]..CHECK_LEN[1]].to_string();
            let cst_str = cst_slice[s_num..e_num].to_string();

            Some(CstResult {
                cst: cst_str,
                version: version_str,
            })
        }
    }
}


fn make_js_result(sign_str: &str, time_stamp: i64) -> JsValue {
    let sign_result: SignResult = SignResult{
        sign: sign_str.to_string(),
        timestamp: time_stamp,
    };

    let sign_result_str = match serde_json::to_string(&sign_result) {
        Ok(v) => v,
        Err(_) => return JsValue::NULL,
    };
    JsValue::from_str(&sign_result_str)
}


fn gen_sign_fake() -> (String, i64) {
    let cur_time_stamp = Local::now().timestamp_millis();
    let mut ori_str = "sign".to_string();
    ori_str += &cur_time_stamp.to_string();

    let mut hasher = Sha1::new();
    hasher.update(&ori_str);
    let sign_str = format!("{:x}", hasher.finalize());

    (sign_str, cur_time_stamp)
}


fn gen_sign_v0(loca_data: &LocalDataV0, js_body_str: &str) -> (String, i64) {
    let cur_time_stamp = Local::now().timestamp_millis();
    let mut ori_str = js_body_str.to_string();
    ori_str += &loca_data.csrf;

    let mut hasher = Sha1::new();
    hasher.update(&ori_str);
    let sign_str = format!("{:x}", hasher.finalize());

    (sign_str, cur_time_stamp)
}


fn gen_sign_v1(local_data: &LocalData, js_body_str: &str, cst_str: &str) -> (String, i64) {
    let time_diff: i64 = local_data.time_diff.parse::<i64>().unwrap_or(0);
    let mut cur_time_stamp = Local::now().timestamp_millis();
    cur_time_stamp -= time_diff;

    let mut ori_str = js_body_str.to_string();
    ori_str += &local_data.csrf;
    ori_str += &CHECK_STR;
    ori_str += &cur_time_stamp.to_string();
    ori_str += &cst_str;

    let mut hasher = Sha1::new();
    hasher.update(&ori_str);
    let sign_str = format!("{:x}", hasher.finalize());

    (sign_str, cur_time_stamp)
}


#[wasm_bindgen]
pub fn gen_sign(js_body: &JsValue) -> JsValue {
    let js_local_data = get_local();

    //生成假的签名fake_sign
    let (sign_str_fake, time_stamp_fake) = gen_sign_fake();
    let sign_fake_js_result = make_js_result(&sign_str_fake, time_stamp_fake);

    //判断是headless则返回假签名
    if headless_detect() == true {
        return sign_fake_js_result;
    }

    let body: String = match js_body.as_string() {
        Some(v) => v,
        None => return sign_fake_js_result, //解析body失败返回假签名
    };

    let local_data_v0: LocalDataV0 = match serde_json::from_str( &js_local_data ) {
        Ok(v) => v,
        Err(_) => return sign_fake_js_result, //获取localstorage的csrf失败，返回假签名
    };

    //生成v0版本的原始签名
    let (sign_str_v0, time_stamp_v0) = gen_sign_v0(&local_data_v0, &body);
    let sign_v0_js_result = make_js_result(&sign_str_v0, time_stamp_v0);

    let local_data: LocalData = match serde_json::from_str( &js_local_data ) {
        Ok(v) => v,
        Err(_) => return sign_v0_js_result,  //获取localstorege的参数失败，返回v0版本的签名
    };

    let cst_result: CstResult = match parse_cst(&local_data.cst) {
        Some(v) => v,
        None => return sign_v0_js_result, //解析cst失败，返回v0版本的签名
    };

    if cst_result.version == VERSION_STR_V1 {
        //生成v1版本的签名
        let (sign_str_v1, time_stamp_v1) = gen_sign_v1(&local_data, &body, &cst_result.cst);
        //结果为v0和v1的拼接
        let sign_str_v0_v1 = format!("{}_{}", sign_str_v0, sign_str_v1);
        return make_js_result(&sign_str_v0_v1, time_stamp_v1);
    }
    else {
        //获取cst版本异常，返回v0版本的签名
        return sign_v0_js_result;
    }
}


#[wasm_bindgen]
pub fn get_version() -> String {
    VERSION_STR_V1.to_string()
}
