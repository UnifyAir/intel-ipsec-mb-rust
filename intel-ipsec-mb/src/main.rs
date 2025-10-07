// use intel_ipsec_mb::mgr::MbMgr;

// fn main() {
//     let mgr = MbMgr::new().unwrap();
//     let mut output = Vec::new();
//     output.resize(20 as usize, 0);
//     let _hash = mgr.sha1(
//         &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//         &mut output,
//     );

//     match _hash {
//         Ok(_) => println!("Hash: {:?}", output),
//         Err(e) => println!("Error: {:}", e),
//     }

//     unsafe {
//         let mut job1 = mgr.get_next_job().unwrap();
//         let mut outputjob1 = Vec::new();
//         outputjob1.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job1,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob1,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!("Hash: {:?}", outputjob1),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job2 = mgr.get_next_job().unwrap();
//         let mut outputjob2 = Vec::new();
//         outputjob2.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job2,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob2,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!("Hash: {:?} {:?}", outputjob2, outputjob1),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job3 = mgr.get_next_job().unwrap();
//         let mut outputjob3 = Vec::new();
//         outputjob3.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job3,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob3,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!("Hash: {:?} {:?} {:?}", outputjob3, outputjob2, outputjob1),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job4 = mgr.get_next_job().unwrap();
//         let mut outputjob4 = Vec::new();
//         outputjob4.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job4,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob4,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?}",
//                 outputjob4, outputjob3, outputjob2, outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job5 = mgr.get_next_job().unwrap();
//         let mut outputjob5 = Vec::new();
//         outputjob5.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job5,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob5,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?}",
//                 outputjob5, outputjob4, outputjob3, outputjob2, outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job6 = mgr.get_next_job().unwrap();
//         let mut outputjob6 = Vec::new();
//         outputjob6.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job6,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob6,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?} {:?}",
//                 outputjob6, outputjob5, outputjob4, outputjob3, outputjob2, outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job7 = mgr.get_next_job().unwrap();
//         let mut outputjob7 = Vec::new();
//         outputjob7.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job7,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob7,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
//                 outputjob7, outputjob6, outputjob5, outputjob4, outputjob3, outputjob2, outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job8 = mgr.get_next_job().unwrap();
//         let mut outputjob8 = Vec::new();
//         outputjob8.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job8,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob8,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
//                 outputjob8,
//                 outputjob7,
//                 outputjob6,
//                 outputjob5,
//                 outputjob4,
//                 outputjob3,
//                 outputjob2,
//                 outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job9 = mgr.get_next_job().unwrap();
//         let mut outputjob9 = Vec::new();
//         outputjob9.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job9,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob9,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
//                 outputjob9,
//                 outputjob8,
//                 outputjob7,
//                 outputjob6,
//                 outputjob5,
//                 outputjob4,
//                 outputjob3,
//                 outputjob2,
//                 outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         let mut job10 = mgr.get_next_job().unwrap();
//         let mut outputjob10 = Vec::new();
//         outputjob10.resize(20 as usize, 0);
//         let _fill = mgr
//             .fill_job_sha1(
//                 &mut job10,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut outputjob10,
//             )
//             .unwrap();
//         let _hash = mgr.submit_job();
//         match _hash {
//             Ok(res) => println!(
//                 "Hash: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
//                 outputjob10,
//                 outputjob9,
//                 outputjob8,
//                 outputjob7,
//                 outputjob6,
//                 outputjob5,
//                 outputjob4,
//                 outputjob3,
//                 outputjob2,
//                 outputjob1
//             ),
//             Err(e) => println!("Error: {:}", e),
//         }

//         println!("After all submissions:");
//         println!("Job 1: {:?}", outputjob1);
//         println!("Job 2: {:?}", outputjob2);
//         println!("Job 3: {:?}", outputjob3);
//         println!("Job 4: {:?}", outputjob4);
//         println!("Job 5: {:?}", outputjob5);
//         println!("Job 6: {:?}", outputjob6);
//         println!("Job 7: {:?}", outputjob7);
//         println!("Job 8: {:?}", outputjob8);
//         println!("Job 9: {:?}", outputjob9);
//         println!("Job 10: {:?}", outputjob10);

//         // loop {
//         //     match mgr.flush_job() {
//         //         Ok(res) => {
//         //             if res.0.is_none() {
//         //                 break;
//         //             }
//         //         }
//         //         Err(e) => println!("Error: {:}", e),
//         //     }
//         //     println!("Flush: {:?} {:?} {:?} {:?} {:?}", outputjob5, outputjob4, outputjob3, outputjob2, outputjob1);
//         // }
//     }
// }

// // mgr.flush_job().unwrap();
// // println!("Hash: {:?}", output1);

// // let mut job1 =mgr.get_next_job().unwrap();
// // let mut output2 = Vec::new();
// // let _fill= mgr.fill_job_sha1(&mut job1, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output2);
// // match _fill {
// //     Ok(_) => println!("Fill:"),
// //     Err(e) => println!("Error: {:}", e),
// // }
// // let _hash = mgr.submit_job();
// // match _hash {
// //     Ok(_) => println!("Hash: {:?}, {:?}", output1, output2),
// //     Err(e) => println!("Error: {:}", e),
// // }

// // }

// // fn main() {
// //     let mut mgr = MbMgr::new().unwrap();

// //     // Test direct SHA1
// //     let mut output = Vec::new();
// //     output.resize(20, 0);
// //     let _hash = mgr.sha1(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output);
// //     match _hash {
// //         Ok(_) => println!("Hash: {:?}", output),
// //         Err(e) => println!("Error: {}", e),
// //     }

// //     // First job
// //     let mut job1 = mgr.get_next_job().unwrap();
// //     let mut output1 = vec![0u8; 20];
// //     mgr.fill_job_sha1(&mut job1, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output1).unwrap();

// //     let result1 = mgr.submit_job();
// //     match result1 {
// //         Ok(res) => {
// //             if res.0.is_none() {
// //                 println!("Job 1: queued");
// //             } else {
// //                 println!("Job 1: completed immediately");
// //             }
// //         }
// //         Err(e) => println!("Job 1 Error: {}", e),
// //     }

// //     // Second job
// //     let mut job2 = mgr.get_next_job().unwrap();
// //     let mut output2 = vec![0u8; 20];

// //     // Make sure fill_job_sha1 succeeded
// //     match mgr.fill_job_sha1(&mut job2, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], &mut output2) {
// //         Ok(_) => println!("Job 2: filled successfully"),
// //         Err(e) => {
// //             println!("Job 2 fill error: {}", e);
// //             return;
// //         }
// //     }

// //     let result2 = mgr.submit_job();
// //     match result2 {
// //         Ok(res) => {
// //             if res.0.is_none() {
// //                 println!("Job 2: queued");
// //             } else {
// //                 println!("Job 2: completed immediately");
// //             }
// //         }
// //         Err(e) => println!("Job 2 Error: {}", e),
// //     }

// //     println!("Outputs before flush: {:?}, {:?}", output1, output2);
// // }

// use intel_ipsec_mb::mgr::MbMgr;

// fn main() {
//     let mgr = MbMgr::new().unwrap();

//     // Test the direct API first
//     let mut output = Vec::new();
//     output.resize(20 as usize, 0);
//     let _hash = mgr.sha1(
//         &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//         &mut output,
//     );
//     match _hash {
//         Ok(_) => println!("Direct SHA1: {:?}\n", output),
//         Err(e) => println!("Error: {:}", e),
//     }

//     unsafe {
//         // Keep ALL output buffers alive
//         let mut outputs: Vec<Vec<u8>> = Vec::new();

//         println!("=== SUBMITTING 20 JOBS ===\n");

//         for i in 0..20 {
//             let mut job = mgr.get_next_job().unwrap();
//             let mut output_buffer = vec![0u8; 20];

//             mgr.fill_job_sha1(
//                 &mut job,
//                 &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
//                 &mut output_buffer,
//             ).unwrap();

//             outputs.push(output_buffer);

//             let _completed = mgr.submit_job().unwrap();

//             // Print status after each submit
//             println!("After submit #{:2}:", i + 1);
//             for (idx, out) in outputs.iter().enumerate() {
//                 let is_filled = out.iter().any(|&b| b != 0);
//                 print!("  Job {:2}: {}", idx + 1, if is_filled { "FILLED" } else { "empty " });
//                 if (idx + 1) % 5 == 0 { println!(); }
//             }
//             if outputs.len() % 5 != 0 { println!(); }
//             println!();
//         }

//         println!("\n=== AFTER ALL SUBMISSIONS (before flush) ===");
//         for (i, out) in outputs.iter().enumerate() {
//             let is_filled = out.iter().any(|&b| b != 0);
//             println!("Job {:2}: {} - {:?}",
//                 i + 1,
//                 if is_filled { "FILLED" } else { "empty " },
//                 if is_filled { &out[..] } else { &[0u8; 0][..] }
//             );
//         }

//         println!("\n=== NOW CALLING FLUSH ===\n");

//         let mut flush_count = 0;
//         loop {
//             match mgr.flush_job() {
//                 Ok(job_result) => {
//                     if job_result.0.is_none() {
//                         println!("Flush returned None - all done");
//                         break;
//                     }
//                     flush_count += 1;
//                     println!("After flush call #{}:", flush_count);
//                     for (idx, out) in outputs.iter().enumerate() {
//                         let is_filled = out.iter().any(|&b| b != 0);
//                         print!("  Job {:2}: {}", idx + 1, if is_filled { "FILLED" } else { "empty " });
//                         if (idx + 1) % 5 == 0 { println!(); }
//                     }
//                     if outputs.len() % 5 != 0 { println!(); }
//                     println!();
//                 }
//                 Err(e) => {
//                     println!("Flush error: {}", e);
//                     break;
//                 }
//             }
//         }

//         println!("\n=== FINAL RESULTS ===");
//         for (i, out) in outputs.iter().enumerate() {
//             let is_filled = out.iter().any(|&b| b != 0);
//             println!("Job {:2}: {} - {:?}",
//                 i + 1,
//                 if is_filled { "FILLED" } else { "empty " },
//                 out
//             );
//         }
//     }
// }

// use intel_ipsec_mb::mgr::MbMgr;
// use intel_ipsec_mb::hash::sha1::Sha1;

// fn main() {
//     let mgr = MbMgr::new().unwrap();
//     let mut output: Vec<u8> = Vec::new();
//     output.resize(20, 0);
//     let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

//     let sha1 = Sha1 {
//         buffer: &input,
//         output: &mut output,
//     };

//     let _hash = mgr.handoff_job(sha1);

//     // Try to drop input - THIS SHOULD FAIL TO COMPILE!
//     // drop(input);  // ❌ ERROR: cannot move out of `input` because it is borrowed

//     unsafe { mgr.flush_job().unwrap(); }
//     drop(_hash);

//     println!("Hash: {:?}", output);

//     drop(input);
// }

use intel_ipsec_mb::hash::sha1::Operation;
use intel_ipsec_mb::hash::sha1::Sha1;
use intel_ipsec_mb::mgr::MbMgr;
use std::pin::Pin;
use std::task::RawWaker;
use std::task::RawWakerVTable;
use std::task::{Context, Poll, Waker};
use std::mem;

// Simple no-op waker for single-threaded polling
// fn noop_waker() -> Waker {
//     fn noop_clone(_: *const ()) -> RawWaker {
//         noop_raw_waker()
//     }
//     fn noop(_: *const ()) {}

//     fn noop_raw_waker() -> RawWaker {
//         RawWaker::new(
//             std::ptr::null(),
//             &RawWakerVTable::new(noop_clone, noop, noop, noop),
//         )
//     }

//     unsafe { Waker::from_raw(noop_raw_waker()) }
// }

fn main() {
    let mgr = MbMgr::new().unwrap();
    let mut output: Vec<u8> = Vec::new();
    output.resize(20, 0);
    let mut output2: Vec<u8> = Vec::new();
    output2.resize(20, 0);
    let mut output3: Vec<u8> = Vec::new();
    output3.resize(20, 0);
    let mut output4: Vec<u8> = Vec::new();
    output4.resize(20, 0);
    let mut output5: Vec<u8> = Vec::new();
    output5.resize(20, 0);
    let mut output6: Vec<u8> = Vec::new();
    output6.resize(20, 0);
    let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let sha = Sha1 {
        buffer: &input,
        output: &mut output,
    };

    let sha2 = Sha1 {
        buffer: &input,
        output: &mut output2,
    };

    let sha3 = Sha1 {
        buffer: &input,
        output: &mut output3,
    };

    let sha4 = Sha1 {
        buffer: &input,
        output: &mut output4,
    };

    let sha5 = Sha1 {
        buffer: &input,
        output: &mut output5,
    };

    let sha6 = Sha1 {
        buffer: &input,
        output: &mut output6,
    };

    unsafe {
        let handle = mgr.handoff_job(sha).unwrap();
        let handle2 = mgr.handoff_job(sha2).unwrap();
        let handle3 = mgr.handoff_job(sha3).unwrap();
        let handle4 = mgr.handoff_job(sha4).unwrap();
        let handle5 = mgr.handoff_job(sha5).unwrap();
        let handle6 = mgr.handoff_job(sha6).unwrap();
        println!("Status: {:?}", handle.get_job_status().unwrap());
        println!("Status: {:?}", handle2.get_job_status().unwrap());
        println!("Status: {:?}", handle3.get_job_status().unwrap());
        println!("Status: {:?}", handle4.get_job_status().unwrap());
        println!("Status: {:?}", handle5.get_job_status().unwrap());
        println!("Status: {:?}", handle6.get_job_status().unwrap());
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        mgr.flush_job().unwrap();
        println!("Status: {:?}", handle.get_job_status().unwrap());
        println!("Status: {:?}", handle2.get_job_status().unwrap());
        println!("Status: {:?}", handle3.get_job_status().unwrap());
        println!("Status: {:?}", handle4.get_job_status().unwrap());
        println!("Status: {:?}", handle5.get_job_status().unwrap());
        println!("Status: {:?}", handle6.get_job_status().unwrap());
        handle.resolve().unwrap();
        handle2.resolve().unwrap();
        handle3.resolve().unwrap();
        handle4.resolve().unwrap();
        handle5.resolve().unwrap();
        handle6.resolve().unwrap();
        println!("Hash: {:?}", output);
        println!("Hash: {:?}", output2);
        println!("Hash: {:?}", output3);
        println!("Hash: {:?}", output4);
        println!("Hash: {:?}", output5);
        println!("Hash: {:?}", output6);
    }
}
