use std::path::Path;
use std::path::{PathBuf};
use crate::parse_moddate::parse_moddate;
use crate::get_strvector::get_strvector;

pub fn mergepress2 (dir1_value: String, hhmmss1_value: String, din1_bool: bool, dir2_value: String, hhmmss2_value: String, din2_bool: bool, size_value: String) -> (u32, String, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = " ".to_string();
     let mut new_dirlist: String = " ".to_string();
     let mut dateyr1: i64 = 0;
     let mut datemo1: i64 = 0;
     let mut dateday1: i64 = 0;
     let mut datehr1: i64 = 0;
     let mut datemin1: i64 = 0;
     let mut datesec1: i64 = 0;
     let mut dateyr2: i64 = 0;
     let mut datemo2: i64 = 0;
     let mut dateday2: i64 = 0;
     let mut datehr2: i64 = 0;
     let mut datemin2: i64 = 0;
     let mut datesec2: i64 = 0;
     let mut filesize_int: i32 = 0;

     let mut bolok = true;

     if Path::new(&dir1_value).exists() {
         if Path::new(&dir2_value).exists() {
             if dir1_value == dir2_value {
                 bolok = false;
                 errcode = 1;
                 errstring = "directory 1 and 2 are the same".to_string();
             }
         } else {
             bolok = false;
             errcode = 2;
             errstring = "directory 2 does not exist".to_string();
         }
     } else {
         bolok = false;
         errcode = 3;
         errstring = "directory 1 does not exist".to_string();
     }
     if bolok {
// see if filesize exists and is between 4 and 16
         filesize_int = size_value.parse().unwrap_or(-99);
         if filesize_int > 0 {
             if (filesize_int < 4) | (filesize_int > 16) {
                 errcode = 4;
                 errstring = "Invalid file length. Must be between 4 and 16 ".to_string();
                 bolok = false;
             }
         } else if filesize_int == -99 {
             errcode = 5;
             errstring = "Files length is not an integer ".to_string();
             bolok = false;
         } else {
             errcode = 6;
             errstring = "File length is not positive integer ".to_string();
             bolok = false;
         }
     }
// validate date mod 1 & 2
     if bolok {
         let (baddate1, dateyr, datemo, dateday, datehr, datemin, datesec) = parse_moddate(hhmmss1_value.to_string());
         if baddate1 != 0 {
             errcode = 7;
             errstring = "Date Mod 1 is not formatted correctly ".to_string();
             bolok = false;
         } else {
             dateyr1 = dateyr;
             datemo1 = datemo;
             dateday1 = dateday;
             datehr1 = datehr;
             datemin1 = datemin;
             datesec1 = datesec;
        }
     }
     if bolok {
         let (baddate2, dateyr, datemo, dateday, datehr, datemin, datesec) = parse_moddate(hhmmss2_value.to_string());
         if baddate2 != 0 {
             errcode = 7;
             errstring = "Date Mod 1 is not formatted correctly ".to_string();
             bolok = false;
         } else {
             dateyr2 = dateyr;
             datemo2 = datemo;
             dateday2 = dateday;
             datehr2 = datehr;
             datemin2 = datemin;
             datesec2 = datesec;
         }
     }
     if bolok {
         let current_dir = PathBuf::from(&dir1_value);
         let (errcd1, errstr1, newvector1) = get_strvector(current_dir, 1, filesize_int, din1_bool, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
         let mut newvectormut = newvector1;
         let mut chgseq2 = false;
         if errcd1 != 0 {
             errcode = 8;
             errstring = errstr1.to_string();
             bolok = false;
         } else {
             let current_dir2 = PathBuf::from(&dir2_value);
             let (errcd2, errstr2, newvector2) = get_strvector(current_dir2, 2, filesize_int, din2_bool, dateyr2, dateday2, datemo2, datehr2, datemin2, datesec2);
             if errcd2 != 0 {
                 errcode = 8;
                 errstring = errstr2.to_string();
                 bolok = false;
             } else {
                 let mut newvectormut2 = newvector2;
                 newvectormut.append(&mut newvectormut2);
             }
         }
         if bolok {
             let newvectormutlen = newvectormut.len();
             let newtoi = newvectormutlen as i32 ;
             if newtoi < 2 {
                 errcode = 9;
                 errstring = "Only one entry in both directories".to_string();
                 bolok = false;
             } else {
                 newvectormut.sort();
                 let mut chgx = true;
                 while chgx {
                        let mut listitems: Vec<String> = Vec::new();
                        chgx = false;
                        for indexi in 1..newtoi {
                             let strinput1split: Vec<&str> = newvectormut[(indexi - 1) as usize].split("|").collect();
                             let strinputsplit: Vec<&str> = newvectormut[indexi as usize].split("|").collect();
                             let mut file_prefixdate1;
                             if chgseq2 {
                                 chgseq2 = false; 
                                 let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                 let mut seq2_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                 if seq2_int == -9999 {
                                     bolok = false;
                                     chgx = false;
                                     errcode = 10;
                                     errstring = "seq number not numeric".to_string();
                                     break;
                                 } else {
                                     if seq2_int < 999 {
                                         seq2_int = seq2_int + 1;
                                         chgx = true;
                                     }
                                     file_prefixdate1 = format!("{}_{:03}", prefix1, seq2_int);
                                 }
                             } else {
                                 file_prefixdate1 = format!("{}", strinput1split[0]);
                             }
                             let file_prefixdate2 = format!("{}", strinputsplit[0]);
                             if file_prefixdate1 == file_prefixdate2 {
                                 chgseq2 = true;
                                 let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                 let mut seq1_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                 if seq1_int == -9999 {
                                     bolok = false;
                                     chgx = false;
                                     errcode = 11;
                                     errstring = "seq number not numeric".to_string();
                                     break;
                                 } else {
                                     if seq1_int > 0 {
                                         seq1_int = seq1_int - 1;
                                         chgx = true;
                                     }
                                 }    
                                 file_prefixdate1 = format!("{}_{:03}", prefix1, seq1_int);
                             }
                             let stroutput = format!("{}|{}|{}|{}|{}", file_prefixdate1, strinput1split[1], strinput1split[2], strinput1split[3], strinput1split[4]);
                             listitems.push(stroutput);
                        } // end for 
                        if bolok {
                            let current_dira = PathBuf::from(&dir1_value);
                            let (errcda, errstra, newvectora) = get_strvector(current_dira, 1, filesize_int, din1_bool, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
                            let mut newvectormuta = newvectora;
                            if errcda != 0 {
                                errcode = 12;
                                errstring = errstra.to_string();
                                bolok = false;
                                break;
                            } else {
                                let current_dirb = PathBuf::from(&dir2_value);
                                let (errcdb, errstrb, newvectorb) = get_strvector(current_dirb, 2, filesize_int, din2_bool, dateyr2, dateday2, datemo2, datehr2, datemin2, datesec2);
                                if errcdb != 0 {
                                    errcode = 13;
                                    errstring = errstrb.to_string();
                                    bolok = false;
                                    break;
                                } else {
                                    let mut newvectormutb = newvectorb;
                                    newvectormuta.append(&mut newvectormutb);
                                    newvectormuta.sort();
                                    let strinputxsplit: Vec<&str> = newvectormuta[(newtoi - 1) as usize].split("|").collect();
                                    let file_prefixdatex;
                                    if chgseq2 {
                                        chgseq2 = false; 
                                        let prefixx: String = strinputxsplit[0][0..19].parse().unwrap();
                                        let mut seqx_int: i32 = strinputxsplit[0][20..].parse().unwrap_or(-9999);
                                        if seqx_int == -9999 {
                                            bolok = false;
                                            errcode = 14;
                                            errstring = "org Merge: seq number not numeric".to_string();
                                            break;
                                        } else {
                                            if seqx_int < 999 {
                                                seqx_int = seqx_int + 1;
                                                chgx = true;
                                            }
                                            file_prefixdatex = format!("{}_{:03}", prefixx, seqx_int);
                                        }
                                    } else {
                                        file_prefixdatex = format!("{}", strinputxsplit[0]);
                                    }
                                    let stroutputx = format!("{}|{}|{}|{}|{}", file_prefixdatex, strinputxsplit[1], strinputxsplit[2], strinputxsplit[3], strinputxsplit[4]);
                                    listitems.push(stroutputx);
                                    newvectormut = listitems;
                                    newvectormut.sort();
                                }
                            }
                        }
                 }  // end of while
             } 
         }
         if bolok {
             let newvectormutlen = newvectormut.len();
             let newtoi = newvectormutlen as i32 ;
             for indexi in 0..newtoi {
                  let strinputx = &newvectormut[indexi as usize];
                  let strinputspx: Vec<&str>  = strinputx.split("|").collect();
                  let newlinelist = strinputspx[1].to_owned() + " | " + strinputspx[2] + " | " + strinputspx[0] + " | " + strinputspx[3] + " | " + strinputspx[4];
                  new_dirlist = new_dirlist + &newlinelist + "\n ";
             }
             errstring = format!("merged {} files", newtoi);
         }
     }
     (errcode, errstring, new_dirlist)
}

