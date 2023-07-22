use std::path::Path;
use std::fs;

pub fn copypress2 (dir1_value: String, dir2_value: String, outdir_value: String, mergescrol_value: String) -> (u32, String) {
     let errcode: u32;
     let errstring: String;
     if !Path::new(&dir1_value).exists() {
         errstring = "the directory 1 does not exist".to_string();
         errcode = 1;
     } else {
         if !Path::new(&dir2_value).exists() {
             errstring = "the directory 2 does not exist".to_string();
             errcode = 2;
         } else {
             if dir1_value == dir2_value {
                 errstring = "directory 1 and directory 2 are the same".to_string();
                 errcode = 3;
             } else {
                 if !Path::new(&outdir_value).exists() {
                     errstring = "output directory does not exist".to_string();
                     errcode = 4;
                 } else {
                     let mut bolok = true;
                     for entry1 in fs::read_dir(&outdir_value).unwrap() {
                          let entry = entry1.unwrap();
                          if let Ok(metadata) = entry.metadata() {
                              if let Ok(_file_name) = entry.file_name().into_string() {
                                  if metadata.is_file() {
                                      bolok = false;
                                  }
                              }
                          }
                     }
                     if bolok {
                         let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
                         let lenmg1 = mergelistvec.len();
                         if lenmg1 < 2 {
                             errstring = "no values in merge list".to_string();
                             errcode = 5;
                         } else {
                             errstring = "Copying in Progress".to_string();
                            errcode = 0;
                         }
                     } else {
                         errstring = "the output directory has files in it".to_string();
                         errcode = 6;
                     }
                 } 
             }
         } 
     }
     (errcode, errstring)
}

