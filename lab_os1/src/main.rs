use std::fmt::Display;
use winsafe::GetSystemDirectory;
use winsafe::GetUserName;
use winsafe::PostQuitMessage;
use winsafe::SYSTEM_INFO;
use winsafe::GetNativeSystemInfo;
use winsafe::{HWND, msg::lvm};
use winsafe::{RegisterClassEx, WNDCLASSEX, WString};

fn main() {
    // let buf = WString::from_str();
    // let mut info = SYSTEM_INFO{..};
    let sys_dir = GetSystemDirectory().unwrap();
    let username = GetUserName().unwrap();
    // let ver = GetNativeSystemInfo(&mut info);
    println!("{:?}, {:?}, {:?}", sys_dir, username, ver);
}
