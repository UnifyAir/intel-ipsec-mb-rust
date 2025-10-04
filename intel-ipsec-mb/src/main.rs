use intel_ipsec_mb::mgr::MbMgr;
use intel_ipsec_mb::hash::sha1::Sha1;

fn main() {
    let mut mgr = MbMgr::new().unwrap();
    let mut output = Vec::new();
    output.resize(20 as usize, 0);
    let _hash = mgr.sha1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output);

    match _hash {
        Ok(_) => println!("Hash: {:?}", output),
        Err(e) => println!("Error: {:}", e),
    }


    let mut job =mgr.get_next_job().unwrap();
    let mut output1 = Vec::new();
    output1.resize(20 as usize, 0);
    let _fill= mgr.fill_job(&mut job, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output1);
    let _hash = mgr.submit_job();
    match _hash {
        Ok(_) => println!("Hash: {:?}", output1),
        Err(e) => println!("Error: {:}", e),
    }
    let _hash = mgr.flush_job();
    match _hash {
        Ok(_) => println!("Hash: {:?}", output1),
        Err(e) => println!("Error: {:}", e),
    }

}