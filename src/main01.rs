use iced::widget::{button, column, row, text_input, text, scrollable, checkbox, Space};
use iced::{Alignment, Element, Task, Length, Color, Theme};

use std::process::Command as stdCommand;
use std::path::Path;

mod get_dirlist;
mod dirpress2;
mod diroutpress2;
mod parse_moddate;
mod dump_file;
mod get_strvector;
mod mergepress2;
mod copypress2;
mod get_winsize;

use get_dirlist::get_dirlist;
use dirpress2::dirpress2;
use diroutpress2::diroutpress2;
use mergepress2::mergepress2;
use copypress2::copypress2;
use get_winsize::get_winsize;

pub fn main() -> iced::Result {

     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(Convert2dirs::title, Convert2dirs::update, Convert2dirs::view)
        .window_size((widthxx, heightxx))
        .theme(Convert2dirs::theme)
        .run_with(Convert2dirs::new)
 }

struct Convert2dirs {
    mess_color: Color,
    msg_value: String,
    dir1_value: String,
    scrol1_value: String,
    hhmmss1_value: String,
    din1_bool: bool,
    dir2_value: String,
    scrol2_value: String,
    hhmmss2_value: String,
    din2_bool: bool,
    size_value: String,
    outdir_value: String,
    mergescrol_value: String,
    scrolheight: f32,
    screenwidth: f32,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Dir1Pressed,
    Hhmmss1Changed(String),
    Dir2Pressed,
    Hhmmss2Changed(String),
    OutDirPressed,
    SizeChanged(String),
    MergePressed,
    CopyPressed,
    CopyxFound(Result<Copyx, Error>),
    DIN1(bool),
    DIN2(bool),
}

impl Convert2dirs {
    fn new() -> (Self, Task<Message>) {
        let mut heightxx: f32 = 170.0;
        let mut widthxx: f32 = 500.0;
        let (errcode, errstring, widtho, heighto) = get_winsize();
        if errcode == 0 {
            heightxx = 170.0 + ((heighto as f32 - 768.0) / 2.0);
            widthxx = 650.0 + ((widtho as f32 - 1366.0) / 2.0);
            println!("{}", errstring);
        } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
        }
        ( Self { msg_value: "no message".to_string(), dir1_value: "no directory".to_string(), hhmmss1_value: "-00:00:00:00:00:00".to_string(),
               dir2_value: "no directory".to_string(), hhmmss2_value: "-00:00:00:00:00:00".to_string(), din1_bool: false, din2_bool: false,
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 1.0]), outdir_value: "no directory".to_string(), 
               scrol1_value: " No directory selected \n \
                            ".to_string(),
               scrol2_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(), scrolheight: heightxx, screenwidth: widthxx as f32,
               theme: Theme::Dracula,
          },
          Task::none()
        )
    }

    fn title(&self) -> String {
        String::from("Merge two directories -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {
            Message::Dir1Pressed => {
               let (errcode, errstr, newdir, newliststr) = dirpress2(self.dir1_value.clone());
               if errcode == 0 {
                   self.scrol1_value  = newliststr.to_string();
                   self.dir1_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   self.msg_value = "got Dir 1 ".to_string();
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   self.msg_value = errstr.to_string();
               }
               Task::none()
            }
            Message::Dir2Pressed => {
               let (errcode, errstr, newdir, newliststr) = dirpress2(self.dir1_value.clone());
               if errcode == 0 {
                   self.scrol2_value  = newliststr.to_string();
                   self.dir2_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   self.msg_value = "got Dir 2 ".to_string();
               } else {
                   self.msg_value = errstr.to_string();
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::Hhmmss1Changed(value) => { self.hhmmss1_value = value; Task::none() }
            Message::Hhmmss2Changed(value) => { self.hhmmss2_value = value; Task::none() }
            Message::SizeChanged(value) => { self.size_value = value; Task::none() }
            Message::OutDirPressed => {
               let (errcode, errstr, newdir) = diroutpress2(self.dir1_value.clone());
               if errcode == 0 {
                   self.outdir_value = newdir.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   self.msg_value = "got out Directory".to_string();
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   self.msg_value = errstr.to_string();
               }
               Task::none()
            }
            Message::MergePressed => {
               let (errcode, errstr, newliststr) = mergepress2(self.dir1_value.clone(), self.hhmmss1_value.clone(), self.din1_bool.clone(),
                                                            self.dir2_value.clone(), self.hhmmss2_value.clone(), self.din2_bool.clone(), self.size_value.clone(),);
               if errcode == 0 {
                   self.mergescrol_value  = newliststr;
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               self.msg_value = errstr.to_string();
               Task::none()
            }
            Message::CopyPressed => {
               let (errcode, errstr) = copypress2(self.dir1_value.clone(), self.dir2_value.clone(), self.outdir_value.clone(), self.mergescrol_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Task::perform(Copyx::copyit(self.dir1_value.clone(), self.dir2_value.clone(), self.outdir_value.clone(), self.mergescrol_value.clone()), Message::CopyxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Task::none()
               }
            }
            Message::CopyxFound(Ok(copyx)) => {
               self.msg_value = copyx.errval.clone();
               self.mess_color = copyx.errcolor.clone();
               Task::none()
            }
            Message::CopyxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
            Message::DIN1(picked) => {self.din1_bool = picked; Task::none()}
            Message::DIN2(picked) => {self.din2_bool = picked; Task::none()}

        }
    }

    fn view(&self) -> Element<Message> {
        let mut dirspace = 5.0;
        if &self.dir1_value.len()*8 < 700 {
            dirspace = 700.0 - 8.0*self.dir1_value.len() as f32;
        }
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Dir 1 Button").on_press(Message::Dir1Pressed),
                 text(&self.dir1_value).size(20),
                 Space::with_width(dirspace),
                 button("Dir 2 Button").on_press(Message::Dir2Pressed),
                 text(&self.dir2_value).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![scrollable(
                column![
                        text(format!("{}",&self.scrol1_value))
                ].width(Length::Fixed(self.screenwidth)),
                ).height(Length::Fixed(self.scrolheight)),
                scrollable(
                column![
                        text(format!("{}",&self.scrol2_value))
                ].width(Length::Fixed(self.screenwidth)),
                ).height(Length::Fixed(self.scrolheight)),],
            row![checkbox("Date in Filename 1", self.din1_bool).on_toggle(Message::DIN1,).width(Length::Fill),
                 checkbox("Date in Filename 2", self.din2_bool).on_toggle(Message::DIN2,).width(Length::Fill),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![text("date mod value 1(-YY:MM:DD:hh:mm:ss): ").size(20),

                 text_input("No input....", &self.hhmmss1_value)
                            .on_input(Message::Hhmmss1Changed).padding(10).size(20),
                 text("date mod value 2(-YY:MM:DD:hh:mm:ss): ").size(20),
                 text_input("No input....", &self.hhmmss2_value)
                            .on_input(Message::Hhmmss2Changed).padding(10).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![text("     Length of File Description: "),
                 text_input("No input....", &self.size_value).on_input(Message::SizeChanged).padding(5).size(15).width(Length::Fixed(50.0)),
              button("outDirectory Button").on_press(Message::OutDirPressed),
                 text(&self.outdir_value).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.mergescrol_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(self.scrolheight)),
            row![button("Merge Button").on_press(Message::MergePressed),
                 button("Copy Button").on_press(Message::CopyPressed),
            ].align_y(Alignment::Center).spacing(400).padding(10),
         ]
        .padding(10)
        .align_x(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

}
#[derive(Debug, Clone)]
struct Copyx {
    errcolor: Color,
    errval: String,
}

impl Copyx {

    async fn copyit(dir1_value: String, dir2_value: String, outdir_value: String, mergescrol_value: String) -> Result<Copyx, Error> {
     let mut errstring  = " ".to_string();
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mut str_cur_dirfrom;
     let colorx: Color;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     for indl in 0..lenmg1 {
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let dirval = lineparse[0];
          if dirval == " 1" {
              str_cur_dirfrom = dir1_value.clone();
          } else if dirval == " 2" {
              str_cur_dirfrom = dir2_value.clone();
          } else {
              bolok = false;
              errstring = format!("invalid directory number in list -{}-", dirval);
              break;
          }
          let filefromx = lineparse[1].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].to_string();
          let filetox = lineparse[3].to_string();
          let fullto = str_cur_dirout.clone() + "/" + &fileprex + "_" + &filetox;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              bolok = false;
              break;
          }
          if numprocess < 4 {
              stdCommand::new("cp")
                           .arg("-p")
                           .arg(&fullfrom)
                           .arg(&fullto)
                           .spawn()
                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let _output = stdCommand::new("cp")
                                         .arg("-p")
                                         .arg(&fullfrom)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");
              numprocess = 0;
          }


          numrow = numrow + 1;
     }
     if bolok {
         errstring = format!("merge 2 dirs copied {} files", lenmg1);
         colorx = Color::from([0.0, 1.0, 0.0]);
     } else {
         colorx = Color::from([1.0, 0.0, 0.0]);
     }
     Ok(Copyx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
//    APIError,
//    LanguageError,
}
