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
// //         for (i, out) in outputs.iter().enumerate() {
// //             let is_filled = out.iter().any(|&b| b != 0);
// //             println!("Job {:2}: {} - {:?}",
// //                 i + 1,
// //                 if is_filled { "FILLED" } else { "empty " },
// //                 out
// //             );
// //         }
// //     }
// // }

// use intel_ipsec_mb::mgr::MbMgr;
// use intel_ipsec_mb::operation::hash::sha::Sha1;
// fn main() {
//     let mgr = MbMgr::new().unwrap();
//     let mut output: Vec<u8> = Vec::new();
//     output.resize(20, 0);
//     let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

//     let mut sha1 = Sha1 {
//         buffer: &input,
//         output: &mut output,
//     };

//     let _hash = mgr.handoff_job(&mut sha1);

//     // Try to drop input - THIS SHOULD FAIL TO COMPILE!
//     // drop(input);  // ❌ ERROR: cannot move out of `input` because it is borrowed

//     unsafe { mgr.flush_job().unwrap(); }
//     _hash.unwrap().0.resolve().unwrap();

//     println!("Hash: {:?}", output);

//     drop(input);
// }

// // use intel_ipsec_mb::hash::sha1::Operation;
// // use intel_ipsec_mb::hash::sha1::Sha1;
// // use intel_ipsec_mb::mgr::MbMgr;
// // use std::pin::Pin;
// // use std::task::RawWaker;
// // use std::task::RawWakerVTable;
// // use std::task::{Context, Poll, Waker};
// // use std::mem;

// // Simple no-op waker for single-threaded polling
// // fn noop_waker() -> Waker {
// //     fn noop_clone(_: *const ()) -> RawWaker {
// //         noop_raw_waker()
// //     }
// //     fn noop(_: *const ()) {}

// //     fn noop_raw_waker() -> RawWaker {
// //         RawWaker::new(
// //             std::ptr::null(),
// //             &RawWakerVTable::new(noop_clone, noop, noop, noop),
// //         )
// //     }

// //     unsafe { Waker::from_raw(noop_raw_waker()) }
// // }

// // fn main() {
// //     let mgr = MbMgr::new().unwrap();
// //     let mut output: Vec<u8> = Vec::new();
// //     output.resize(20, 0);
// //     let mut output2: Vec<u8> = Vec::new();
// //     output2.resize(20, 0);
// //     let mut output3: Vec<u8> = Vec::new();
// //     output3.resize(20, 0);
// //     let mut output4: Vec<u8> = Vec::new();
// //     output4.resize(20, 0);
// //     let mut output5: Vec<u8> = Vec::new();
// //     output5.resize(20, 0);
// //     let mut output6: Vec<u8> = Vec::new();
// //     output6.resize(20, 0);
// //     let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

// //     let sha = Sha1 {
// //         buffer: &input,
// //         output: &mut output,
// //     };

// //     let sha2 = Sha1 {
// //         buffer: &input,
// //         output: &mut output2,
// //     };

// //     let sha3 = Sha1 {
// //         buffer: &input,
// //         output: &mut output3,
// //     };

// //     let sha4 = Sha1 {
// //         buffer: &input,
// //         output: &mut output4,
// //     };

// //     let sha5 = Sha1 {
// //         buffer: &input,
// //         output: &mut output5,
// //     };

// //     let sha6 = Sha1 {
// //         buffer: &input,
// //         output: &mut output6,
// //     };

// //     unsafe {
// //         let handle = mgr.handoff_job(sha).unwrap().0;
// //         let handle2 = mgr.handoff_job(sha2).unwrap().0;
// //         let handle3 = mgr.handoff_job(sha3).unwrap().0;
// //         let handle4 = mgr.handoff_job(sha4).unwrap().0;
// //         let handle5 = mgr.handoff_job(sha5).unwrap().0;
// //         let handle6 = mgr.handoff_job(sha6).unwrap().0;
// //         println!("Status: {:?}", handle.get_job_status().unwrap());
// //         println!("Status: {:?}", handle2.get_job_status().unwrap());
// //         println!("Status: {:?}", handle3.get_job_status().unwrap());
// //         println!("Status: {:?}", handle4.get_job_status().unwrap());
// //         println!("Status: {:?}", handle5.get_job_status().unwrap());
// //         println!("Status: {:?}", handle6.get_job_status().unwrap());
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         mgr.flush_job().unwrap();
// //         println!("Status: {:?}", handle.get_job_status().unwrap());
// //         println!("Status: {:?}", handle2.get_job_status().unwrap());
// //         println!("Status: {:?}", handle3.get_job_status().unwrap());
// //         println!("Status: {:?}", handle4.get_job_status().unwrap());
// //         println!("Status: {:?}", handle5.get_job_status().unwrap());
// //         println!("Status: {:?}", handle6.get_job_status().unwrap());
// //         handle.resolve().unwrap();
// //         handle2.resolve().unwrap();
// //         handle3.resolve().unwrap();
// //         handle4.resolve().unwrap();
// //         handle5.resolve().unwrap();
// //         handle6.resolve().unwrap();
// //         println!("Hash: {:?}", output);
// //         println!("Hash: {:?}", output2);
// //         println!("Hash: {:?}", output3);
// //         println!("Hash: {:?}", output4);
// //         println!("Hash: {:?}", output5);
// //         println!("Hash: {:?}", output6);
// //     }
// // }

// // use intel_ipsec_mb::hash::sha1::Sha1;
// // use intel_ipsec_mb::mgr::MbMgr;
// // use intel_ipsec_mb::runtime::{create_runtime, MbRuntimeHandle, MbJobRequest};
// // use std::sync::Arc;
// // use std::sync::mpsc;
// // use std::thread;
// // use std::time::Instant;

// use intel_ipsec_mb::hash::sha1::Sha1;

// use intel_ipsec_mb::runtime::spawn_runtime;
// use std::thread;

// fn main() {
//     println!("Starting MB Runtime...");
    
//     // Create runtime (uninitialized, Send)
//     let handle = spawn_runtime();


//     let mut output1: Vec<u8> = Vec::new();
//     output1.resize(20, 0);

//     let mut output2: Vec<u8> = output1.clone();
//     let mut output3: Vec<u8> = output1.clone();
//     let mut output4: Vec<u8> = output1.clone();
//     let mut output5: Vec<u8> = output1.clone();
//     let mut output6: Vec<u8> = output1.clone();
//     let mut output7: Vec<u8> = output1.clone();
//     let mut output8: Vec<u8> = output1.clone();
//     let mut output9: Vec<u8> = output1.clone();
//     let mut output10: Vec<u8> = output1.clone();

//     let input1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

//     let input2: Vec<u8> = input1.clone();
//     let input3: Vec<u8> = input1.clone();
//     let input4: Vec<u8> = input1.clone();
//     let input5: Vec<u8> = input1.clone();
//     let input6: Vec<u8> = input1.clone();
//     let input7: Vec<u8> = input1.clone();
//     let input8: Vec<u8> = input1.clone();
//     let input9: Vec<u8> = input1.clone();
//     let input10: Vec<u8> = input1.clone();

//     let sha1 = Sha1 {
//         buffer: &input1,
//         output: &mut output1,
//     };

//     let sha2 = Sha1 {
//         buffer: &input2,
//         output: &mut output2,
//     };

//     let sha3 = Sha1 {
//         buffer: &input3,
//         output: &mut output3,
//     };

//     let sha4 = Sha1 {
//         buffer: &input4,
//         output: &mut output4,
//     };
    
//     let sha5 = Sha1 {
//         buffer: &input5,
//         output: &mut output5,
//     };

//     let sha6 = Sha1 {
//         buffer: &input6,
//         output: &mut output6,
//     };

//     let sha7 = Sha1 {
//         buffer: &input7,
//         output: &mut output7,
//     };

//     let sha8 = Sha1 {
//         buffer: &input8,
//         output: &mut output8,
//     };

//     let sha9 = Sha1 {
//         buffer: &input9,
//         output: &mut output9,
//     };

//     let sha10 = Sha1 {
//         buffer: &input10,
//         output: &mut output10,
//     };

//     let status1 = handle.submit_job(sha1);
//     let status2 = handle.submit_job(sha2);
//     let status3 = handle.submit_job(sha3);
//     let status4 = handle.submit_job(sha4);
//     let status5 = handle.submit_job(sha5);
//     let status6 = handle.submit_job(sha6);
//     let status7 = handle.submit_job(sha7);
//     let status8 = handle.submit_job(sha8);
//     let status9 = handle.submit_job(sha9);
//     let status10 = handle.submit_job(sha10);

//     println!("Status: {:?}", output1);
//     println!("Status: {:?}", output2);
//     println!("Status: {:?}", output3);
//     println!("Status: {:?}", output4);
//     println!("Status: {:?}", output5);
//     println!("Status: {:?}", output6);
//     println!("Status: {:?}", output7);
//     println!("Status: {:?}", output8);
//     println!("Status: {:?}", output9);
//     println!("Status: {:?}", output10);
   
//     handle.join_handle.join().unwrap();

//     // // Spawn runtime on dedicated thread
//     // thread::spawn(move || {
//     //     println!("Runtime thread started, initializing MbMgr...");
//     //     runtime_init.run();  // This initializes MbMgr on this thread
//     //     println!("Runtime thread exiting");
//     // });
    
//     // println!("Runtime spawned, creating 5 worker threads...");
    
//     // // Share handle across threads
//     // let handle = Arc::new(handle);
    
//     // // Spawn 5 worker threads
//     // let mut threads = vec![];
    
//     // for thread_id in 0..5 {
//     //     let handle = Arc::clone(&handle);
        
//     //     let thread = thread::spawn(move || {
//     //         println!("Thread {} started", thread_id);
            
//     //         // Each thread submits 10 SHA-1 jobs
//     //         for job_id in 0..10 {
//     //             let input: Vec<u8> = vec![
//     //                 thread_id as u8, job_id as u8,
//     //                 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
//     //             ];
                
//     //             let mut output = vec![0u8; 20];
                
//     //             // Create completion channel for this job
//     //             let (completion_tx, completion_rx) = mpsc::sync_channel(1);
                
//     //             let start = Instant::now();
                
//     //             // Create job request
//     //             let job = MbJobRequest {
//     //                 operation: Box::new(Sha1 {
//     //                     buffer: input.as_slice(),
//     //                     output: output.as_mut_slice(),
//     //                 }),
//     //                 completion: completion_tx,
//     //             };
                
//     //             // Send job to runtime
//     //             if let Err(e) = handle.send_job(job) {
//     //                 eprintln!("Thread {}, Job {}: Failed to send - {:?}", thread_id, job_id, e);
//     //                 continue;
//     //             }
                
//     //             // Wait for completion
//     //             match completion_rx.recv() {
//     //                 Ok(status) => {
//     //                     let elapsed = start.elapsed();
//     //                     println!(
//     //                         "Thread {}, Job {}: {:?} (took {:?})",
//     //                         thread_id, job_id, status, elapsed
//     //                     );
//     //                     println!("  Hash: {:02x?}", output);
//     //                 }
//     //                 Err(e) => {
//     //                     eprintln!("Thread {}, Job {}: Completion error - {:?}", thread_id, job_id, e);
//     //                 }
//     //             }
                
//     //             // Small delay
//     //             thread::sleep(std::time::Duration::from_millis(10));
//     //         }
            
//     //         println!("Thread {} finished", thread_id);
//     //     });
        
//     //     threads.push(thread);
//     // }
    
//     // println!("All worker threads spawned, waiting for completion...");
    
//     // // Wait for all threads
//     // for (i, thread) in threads.into_iter().enumerate() {
//     //     thread.join().expect(&format!("Thread {} panicked", i));
//     // }
    
//     // println!("All jobs completed!");
    
//     // // Drop handle to signal shutdown
//     // drop(handle);
    
//     // thread::sleep(std::time::Duration::from_millis(100));
//     // println!("Done!");
// }




















// use intel_ipsec_mb::hash::sha1::Sha1;

// use intel_ipsec_mb::runtime::spawn_runtime;
// use std::thread;

// fn main() {
//     println!("Starting MB Runtime...");
    
//     // Create runtime (uninitialized, Send)
//     let handle = spawn_runtime();


//     let mut output1: Vec<u8> = Vec::new();
//     output1.resize(20, 0);

//     // let mut output2: Vec<u8> = output1.clone();
//     // let mut output3: Vec<u8> = output1.clone();
//     // let mut output4: Vec<u8> = output1.clone();
//     // let mut output5: Vec<u8> = output1.clone();
//     // let mut output6: Vec<u8> = output1.clone();
//     // let mut output7: Vec<u8> = output1.clone();
//     // let mut output8: Vec<u8> = output1.clone();
//     // let mut output9: Vec<u8> = output1.clone();
//     // let mut output10: Vec<u8> = output1.clone();

//     let input1: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

//     // let input2: Vec<u8> = input1.clone();
//     // let input3: Vec<u8> = input1.clone();
//     // let input4: Vec<u8> = input1.clone();
//     // let input5: Vec<u8> = input1.clone();
//     // let input6: Vec<u8> = input1.clone();
//     // let input7: Vec<u8> = input1.clone();
//     // let input8: Vec<u8> = input1.clone();
//     // let input9: Vec<u8> = input1.clone();
//     // let input10: Vec<u8> = input1.clone();

//     let sha1 = Sha1 {
//         buffer: &input1,
//         output: &mut output1,
//     };

//     // let sha2 = Sha1 {
//     //     buffer: &input2,
//     //     output: &mut output2,
//     // };

//     // let sha3 = Sha1 {
//     //     buffer: &input3,
//     //     output: &mut output3,
//     // };

//     // let sha4 = Sha1 {
//     //     buffer: &input4,
//     //     output: &mut output4,
//     // };
    
//     // let sha5 = Sha1 {
//     //     buffer: &input5,
//     //     output: &mut output5,
//     // };

//     // let sha6 = Sha1 {
//     //     buffer: &input6,
//     //     output: &mut output6,
//     // };

//     // let sha7 = Sha1 {
//     //     buffer: &input7,
//     //     output: &mut output7,
//     // };

//     // let sha8 = Sha1 {
//     //     buffer: &input8,
//     //     output: &mut output8,
//     // };

//     // let sha9 = Sha1 {
//     //     buffer: &input9,
//     //     output: &mut output9,
//     // };

//     // let sha10 = Sha1 {
//     //     buffer: &input10,
//     //     output: &mut output10,
//     // };

//     let status1 = handle.submit_job(sha1);
//     // let status2 = handle.submit_job(sha2);
//     // let status3 = handle.submit_job(sha3);
//     // let status4 = handle.submit_job(sha4);
//     // let status5 = handle.submit_job(sha5);
//     // let status6 = handle.submit_job(sha6);
//     // let status7 = handle.submit_job(sha7);
//     // let status8 = handle.submit_job(sha8);
//     // let status9 = handle.submit_job(sha9);
//     // let status10 = handle.submit_job(sha10);

//     println!("Status: {:?}", output1);
//     // println!("Status: {:?}", output2);
//     // println!("Status: {:?}", output3);
//     // println!("Status: {:?}", output4);
//     // println!("Status: {:?}", output5);
//     // println!("Status: {:?}", output6);
//     // println!("Status: {:?}", output7);
//     // println!("Status: {:?}", output8);
//     // println!("Status: {:?}", output9);
//     // println!("Status: {:?}", output10);
   
//     handle.join_handle.join().unwrap();
// }





// ===============================================
//Async example

// use intel_ipsec_mb::runtime::spawn_runtime;
// use std::thread;
// use std::sync::Arc;
// use intel_ipsec_mb::operation::hash::sha::{Sha1, Sha1OneBlock};

// fn main() {
//     println!("Starting MB Runtime...");
    
//     // Spawn the runtime thread
//     let handle = spawn_runtime().unwrap();
//     let handle = Arc::new(handle);
    
//     println!("Creating 10 worker threads...");
    
//     let mut thread_handles = vec![];
    
//     // Spawn 10 worker threads
//     for thread_id in 0..10 {
//         let handle = Arc::clone(&handle);
        
//         let thread_handle = thread::spawn(move || {
//             println!("Thread {} started", thread_id);
            
//             // Create input and output buffers
//             // let input: Vec<u8> = vec![
//                 // 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
//             // ];

//             let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
//             let mut output = vec![0u8; 20];
            
//             // Create SHA-1 operation
//             let sha = Sha1OneBlock {
//                 buffer: &input,
//                 output: &mut output,
//             };
            
//             // Submit job and wait for completion
//             match handle.publish_job(sha) {
//                 Ok(status) => {
//                     println!("Thread {}: Status = {:?}", thread_id, status);
//                     println!("Thread {}: Hash = {:?}", thread_id, output);
//                 }
//                 Err(e) => {
//                     eprintln!("Thread {}: Error = {:?}", thread_id, e);
//                 }
//             }
            
//             println!("Thread {} finished", thread_id);
//         });
        
//         thread_handles.push(thread_handle);
//     }
    
//     println!("All worker threads spawned, waiting for completion...");
    
//     // Join all worker threads
//     for (i, handle) in thread_handles.into_iter().enumerate() {
//         handle.join().expect(&format!("Thread {} panicked", i));
//     }
    
//     println!("All worker threads completed!");
  
    
//     // Wait a bit for runtime to exit gracefully
//     thread::sleep(std::time::Duration::from_millis(100));

    
//     println!("Test complete!");
// }



// use intel_ipsec_mb::runtime::spawn_runtime;
// use std::sync::Arc;
// use intel_ipsec_mb::operation::hash::sha::Sha1OneBlock;

// #[tokio::main]
// async fn main() {
//     println!("Starting MB Runtime...");
    
//     // Spawn the runtime thread
//     let handle = spawn_runtime().unwrap();
//     let handle = Arc::new(handle);
    
//     println!("Creating 10 async tasks...");
    
//     let mut tasks = vec![];
    
//     // Spawn 10 async tasks
//     for task_id in 0..100000 {
//         let handle = Arc::clone(&handle);
        
//         let task = tokio::spawn(async move {
//             // println!("Task {} started", task_id);
            
//             // Create input and output buffers
//             let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
//             let mut output = vec![0u8; 20];
            
//             // Create SHA-1 operation
//             let sha = Sha1OneBlock {
//                 buffer: &input,
//                 output: &mut output,
//             };
            
//             // Submit job and AWAIT completion (non-blocking!)
//             match handle.publish_job_async(sha).await {
//                 Ok(status) => {
//                     // println!("Task {}: Status = {:?}", task_id, status);
//                     println!("Task {}: Hash = {:?}", task_id, output);
//                 }
//                 Err(e) => {
//                     eprintln!("Task {}: Error = {:?}", task_id, e);
//                 }
//             }
            
//             // println!("Task {} finished", task_id);
//         });
        
//         tasks.push(task);
//     }
    
//     println!("All async tasks spawned, waiting for completion...");
    
//     // Await all tasks concurrently
//     for (i, task) in tasks.into_iter().enumerate() {
//         task.await.expect(&format!("Task {} panicked", i));
//     }
    
//     println!("All async tasks completed!");
    
//     // Gracefully shutdown the runtime
//     drop(handle);
    
//     println!("Test complete!");
// }


use intel_ipsec_mb::runtime::spawn_runtime;
use std::sync::Arc;
use intel_ipsec_mb::operation::hash::sha::Sha1OneBlock;

#[tokio::main]
async fn main() {
    println!("Starting MB Runtime...");
    
    let handle = spawn_runtime().unwrap();
    let handle = Arc::new(handle);
    
    println!("Creating tasks...");
    
    let mut tasks = vec![];
    
    for task_id in 0..1000000 {  // Start with 100 to debug
        let handle = Arc::clone(&handle);
        
        let task = tokio::spawn(async move {
            // Use Box to ensure buffers stay on heap
            let input = Box::new([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
            let mut output = Box::new([0u8; 20]);
            
            let sha = Sha1OneBlock {
                buffer: &*input,
                output: &mut *output,
            };
            
            match handle.publish_job_async(sha).await {
                Ok(_status) => {
                    println!("Task {}: Hash = {:?}", task_id, &*output);
                }
                Err(e) => {
                    eprintln!("Task {}: Error = {:?}", task_id, e);
                }
            }
        });
        
        tasks.push(task);
    }
    
    println!("All async tasks spawned, waiting for completion...");
    
    for (i, task) in tasks.into_iter().enumerate() {
        task.await.expect(&format!("Task {} panicked", i));
    }
    
    println!("All async tasks completed!");
}