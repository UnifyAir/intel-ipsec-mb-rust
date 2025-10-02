use intel_ipsec_mb::mgr::MbMgr;

fn main() {
    let mut mgr = MbMgr::new().unwrap();
    let hash = mgr.sha1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap();
    println!("{:?}", hash);
}