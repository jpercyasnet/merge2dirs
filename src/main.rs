use iced::widget::{button, column, row, text_input, text, scrollable, checkbox};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color};
use iced::theme::{self, Theme};
use iced::executor;
use iced::window;

use std::process::Command as stdCommand;
use std::path::{Path};

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

     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho - 20;
         heightxx = heighto - 75;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     Counterx::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
 }

struct Counterx {
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

impl Application for Counterx {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Counterx, iced::Command<Message>) {
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
               size_value: "10".to_string(), mess_color: Color::from([0.0, 0.0, 0.0]), outdir_value: "no directory".to_string(), 
               scrol1_value: " No directory selected \n \
                            ".to_string(),
               scrol2_value: " No directory selected \n \
                            ".to_string(),
               mergescrol_value: " No directory selected \n \
                            ".to_string(), scrolheight: heightxx, screenwidth: widthxx as f32,
          },
          Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Merge two directories -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
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
               Command::none()
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
               Command::none()
            }
            Message::Hhmmss1Changed(value) => { self.hhmmss1_value = value; Command::none() }
            Message::Hhmmss2Changed(value) => { self.hhmmss2_value = value; Command::none() }
            Message::SizeChanged(value) => { self.size_value = value; Command::none() }
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
               Command::none()
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
               Command::none()
            }
            Message::CopyPressed => {
               let (errcode, errstr) = copypress2(self.dir1_value.clone(), self.dir2_value.clone(), self.outdir_value.clone(), self.mergescrol_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Command::perform(Copyx::copyit(self.dir1_value.clone(), self.dir2_value.clone(), self.outdir_value.clone(), self.mergescrol_value.clone()), Message::CopyxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::CopyxFound(Ok(copyx)) => {
               self.msg_value = copyx.errval.clone();
               self.mess_color = copyx.errcolor.clone();
               Command::none()
            }
            Message::CopyxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
            Message::DIN1(picked) => {self.din1_bool = picked; Command::none()}
            Message::DIN2(picked) => {self.din2_bool = picked; Command::none()}

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Dir 1 Button").on_press(Message::Dir1Pressed).style(theme::Button::Secondary),
                 text(&self.dir1_value).size(15), button("Dir 2 Button").on_press(Message::Dir2Pressed).style(theme::Button::Secondary),
                 text(&self.dir2_value).size(15),
            ].align_items(Alignment::Center).spacing(10).padding(10),
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
            row![checkbox("Date in Filename 1", self.din1_bool, Message::DIN1,).width(Length::Fill), checkbox("Date in Filename 2", self.din2_bool, Message::DIN2,).width(Length::Fill),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("date mod value 1(-YY:MM:DD:hh:mm:ss): ").size(20),

                 text_input("No input....", &self.hhmmss1_value)
                            .on_input(Message::Hhmmss1Changed).padding(10).size(20),
                 text("date mod value 2(-YY:MM:DD:hh:mm:ss): ").size(20),
                 text_input("No input....", &self.hhmmss2_value)
                            .on_input(Message::Hhmmss2Changed).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("     Length of File Description: "),
                 text_input("No input....", &self.size_value).on_input(Message::SizeChanged).padding(5).size(15).width(Length::Fixed(50.0)),
              button("outDirectory Button").on_press(Message::OutDirPressed).style(theme::Button::Secondary),
                 text(&self.outdir_value).size(15),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.mergescrol_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(self.scrolheight)),
            row![button("Merge Button").on_press(Message::MergePressed).style(theme::Button::Secondary),
                 button("Copy Button").on_press(Message::CopyPressed).style(theme::Button::Secondary),
            ].align_items(Alignment::Center).spacing(400).padding(10),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
//       Theme::Light
          Theme::custom(theme::Palette {
                        background: Color::from_rgb8(240, 240, 240),
                        text: Color::BLACK,
                        primary: Color::from_rgb8(230, 230, 230),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    })
               
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
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let dirval = lineparse[0].clone();
          if dirval == " 1" {
              str_cur_dirfrom = dir1_value.clone();
          } else if dirval == " 2" {
              str_cur_dirfrom = dir2_value.clone();
          } else {
              bolok = false;
              errstring = format!("invalid directory number in list -{}-", dirval);
              break;
          }
          let filefromx = lineparse[1].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].clone().to_string();
          let filetox = lineparse[3].clone().to_string();
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
