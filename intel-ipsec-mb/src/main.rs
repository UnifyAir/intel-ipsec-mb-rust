use intel_ipsec_mb::mgr::MbMgr;


fn main() {
    let mgr = MbMgr::new().unwrap();
    let mut output = Vec::new();
    output.resize(20 as usize, 0);
    let _hash = mgr.sha1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output);

    match _hash {
        Ok(_) => println!("Hash: {:?}", output),
        Err(e) => println!("Error: {:}", e),
    }


    // let mut job =mgr.get_next_job().unwrap();
    // let mut output1 = Vec::new();
    // output1.resize(20 as usize, 0);
    // let _fill= mgr.fill_job_sha1(&mut job, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output1);
    // let _hash = mgr.submit_job();
    // match _hash {
    //     Ok(res) => {
    //         if res.0.is_none() {
    //             println!("is none");
    //         }
    //         println!("Hash: {:?}", output1);
    //     }
    //     Err(e) => println!("Error: {:}", e),
    }

    // mgr.flush_job().unwrap();
    // println!("Hash: {:?}", output1);


    // let mut job1 =mgr.get_next_job().unwrap();
    // let mut output2 = Vec::new();
    // let _fill= mgr.fill_job_sha1(&mut job1, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output2);
    // match _fill {
    //     Ok(_) => println!("Fill:"),
    //     Err(e) => println!("Error: {:}", e),
    // }
    // let _hash = mgr.submit_job();
    // match _hash {
    //     Ok(_) => println!("Hash: {:?}, {:?}", output1, output2),
    //     Err(e) => println!("Error: {:}", e),
    // }

// }



// fn main() {
//     let mut mgr = MbMgr::new().unwrap();
    
//     // Test direct SHA1
//     let mut output = Vec::new();
//     output.resize(20, 0);
//     let _hash = mgr.sha1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output);
//     match _hash {
//         Ok(_) => println!("Hash: {:?}", output),
//         Err(e) => println!("Error: {}", e),
//     }
    
//     // First job
//     let mut job1 = mgr.get_next_job().unwrap();
//     let mut output1 = vec![0u8; 20];
//     mgr.fill_job_sha1(&mut job1, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output1).unwrap();
    
//     let result1 = mgr.submit_job();
//     match result1 {
//         Ok(res) => {
//             if res.0.is_none() {
//                 println!("Job 1: queued");
//             } else {
//                 println!("Job 1: completed immediately");
//             }
//         }
//         Err(e) => println!("Job 1 Error: {}", e),
//     }
    
//     // Second job
//     let mut job2 = mgr.get_next_job().unwrap();
//     let mut output2 = vec![0u8; 20];
    
//     // Make sure fill_job_sha1 succeeded
//     match mgr.fill_job_sha1(&mut job2, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output2) {
//         Ok(_) => println!("Job 2: filled successfully"),
//         Err(e) => {
//             println!("Job 2 fill error: {}", e);
//             return;
//         }
//     }
    
//     let result2 = mgr.submit_job();
//     match result2 {
//         Ok(res) => {
//             if res.0.is_none() {
//                 println!("Job 2: queued");
//             } else {
//                 println!("Job 2: completed immediately");
//             }
//         }
//         Err(e) => println!("Job 2 Error: {}", e),
//     }
    
//     println!("Outputs before flush: {:?}, {:?}", output1, output2);
// }