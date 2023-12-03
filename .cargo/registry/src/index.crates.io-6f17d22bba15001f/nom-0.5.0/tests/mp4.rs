#![allow(dead_code)]

#[macro_use]
extern crate nom;

use nom::{HexDisplay,Needed,IResult,FileProducer,be_u16,be_u32,be_u64,be_f32};
use nom::{Consumer,ConsumerState};
use nom::IResult::*;
use nom::Err::*;

use std::str;
use std::io::SeekFrom;

fn mp4_box(input:&[u8]) -> IResult<&[u8], &[u8]> {
  match be_u32(input) {
    Done(i, offset) => {
      let sz: usize = offset as usize;
      if i.len() >= sz - 4 {
        Done(&i[(sz-4)..], &i[0..(sz-4)])
      } else {
        Incomplete(Needed::Size(offset as usize + 4))
      }
    }
    Error(e)      => Error(e),
    Incomplete(e) => Incomplete(e)
  }
}

#[derive(PartialEq,Eq,Debug)]
struct FileType<'a> {
  major_brand:         &'a str,
  major_brand_version: &'a [u8],
  compatible_brands:   Vec<&'a str>
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Mvhd32 {
  version_flags: u32, // actually:
  // version: u8,
  // flags: u24       // 3 bytes
  created_date:  u32,
  modified_date: u32,
  scale:         u32,
  duration:      u32,
  speed:         f32,
  volume:        u16, // actually a 2 bytes decimal
  /* 10 bytes reserved */
  scaleA:        f32,
  rotateB:       f32,
  angleU:        f32,
  rotateC:       f32,
  scaleD:        f32,
  angleV:        f32,
  positionX:     f32,
  positionY:     f32,
  scaleW:        f32,
  preview:       u64,
  poster:        u32,
  selection:     u64,
  current_time:  u32,
  track_id:      u32
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Mvhd64 {
  version_flags: u32, // actually:
  // version: u8,
  // flags: u24       // 3 bytes
  created_date:  u64,
  modified_date: u64,
  scale:         u32,
  duration:      u64,
  speed:         f32,
  volume:        u16, // actually a 2 bytes decimal
  /* 10 bytes reserved */
  scaleA:        f32,
  rotateB:       f32,
  angleU:        f32,
  rotateC:       f32,
  scaleD:        f32,
  angleV:        f32,
  positionX:     f32,
  positionY:     f32,
  scaleW:        f32,
  preview:       u64,
  poster:        u32,
  selection:     u64,
  current_time:  u32,
  track_id:      u32
}

#[allow(non_snake_case)]
named!(mvhd32 <&[u8], MvhdBox>,
  chain!(
  version_flags: be_u32 ~
  created_date:  be_u32 ~
  modified_date: be_u32 ~
  scale:         be_u32 ~
  duration:      be_u32 ~
  speed:         be_f32 ~
  volume:        be_u16 ~ // actually a 2 bytes decimal
              take!(10) ~
  scale_a:       be_f32 ~
  rotate_b:      be_f32 ~
  angle_u:       be_f32 ~
  rotate_c:      be_f32 ~
  scale_d:       be_f32 ~
  angle_v:       be_f32 ~
  position_x:    be_f32 ~
  position_y:    be_f32 ~
  scale_w:       be_f32 ~
  preview:       be_u64 ~
  poster:        be_u32 ~
  selection:     be_u64 ~
  current_time:  be_u32 ~
  track_id:      be_u32,
  ||{
    MvhdBox::M32(Mvhd32 {
      version_flags: version_flags,
      created_date:  created_date,
      modified_date: modified_date,
      scale:         scale,
      duration:      duration,
      speed:         speed,
      volume:        volume,
      scaleA:        scale_a,
      rotateB:       rotate_b,
      angleU:        angle_u,
      rotateC:       rotate_c,
      scaleD:        scale_d,
      angleV:        angle_v,
      positionX:     position_x,
      positionY:     position_y,
      scaleW:        scale_w,
      preview:       preview,
      poster:        poster,
      selection:     selection,
      current_time:  current_time,
      track_id:      track_id
    })
  }
  )
);

#[allow(non_snake_case)]
named!(mvhd64 <&[u8], MvhdBox>,
  chain!(
  version_flags: be_u32 ~
  created_date:  be_u64 ~
  modified_date: be_u64 ~
  scale:         be_u32 ~
  duration:      be_u64 ~
  speed:         be_f32 ~
  volume:        be_u16 ~ // actually a 2 bytes decimal
              take!(10) ~
  scale_a:       be_f32 ~
  rotate_b:      be_f32 ~
  angle_u:       be_f32 ~
  rotate_c:      be_f32 ~
  scale_d:       be_f32 ~
  angle_v:       be_f32 ~
  position_x:    be_f32 ~
  position_y:    be_f32 ~
  scale_w:       be_f32 ~
  preview:       be_u64 ~
  poster:        be_u32 ~
  selection:     be_u64 ~
  current_time:  be_u32 ~
  track_id:      be_u32,
  ||{
    MvhdBox::M64(Mvhd64 {
      version_flags: version_flags,
      created_date:  created_date,
      modified_date: modified_date,
      scale:         scale,
      duration:      duration,
      speed:         speed,
      volume:        volume,
      scaleA:        scale_a,
      rotateB:       rotate_b,
      angleU:        angle_u,
      rotateC:       rotate_c,
      scaleD:        scale_d,
      angleV:        angle_v,
      positionX:     position_x,
      positionY:     position_y,
      scaleW:        scale_w,
      preview:       preview,
      poster:        poster,
      selection:     selection,
      current_time:  current_time,
      track_id:      track_id
    })
  }
  )
);

#[derive(Debug)]
pub enum MvhdBox {
  M32(Mvhd32),
  M64(Mvhd64)
}

#[derive(Debug)]
pub enum MoovBox {
  Mdra,
  Dref,
  Cmov,
  Rmra,
  Iods,
  Mvhd(MvhdBox),
  Clip,
  Trak,
  Udta
}

#[derive(Debug)]
enum MP4BoxType {
  Ftyp,
  Moov,
  Mdat,
  Free,
  Skip,
  Wide,
  Mdra,
  Dref,
  Cmov,
  Rmra,
  Iods,
  Mvhd,
  Clip,
  Trak,
  Udta,
  Unknown
}

#[derive(Debug)]
struct MP4BoxHeader {
  length: u32,
  tag:    MP4BoxType
}

named!(brand_name<&[u8],&str>, map_res!(take!(4), str::from_utf8));

named!(filetype_parser<&[u8], FileType>,
  chain!(
    m: brand_name          ~
    v: take!(4)            ~
    c: many0!(brand_name)  ,
    ||{ FileType{ major_brand: m, major_brand_version:v, compatible_brands: c } }
  )
);

fn mvhd_box(input:&[u8]) -> IResult<&[u8],MvhdBox> {
  if input.len() < 100 {
    Incomplete(Needed::Size(100))
  } else if input.len() == 100 {
    mvhd32(input)
  } else if input.len() == 112 {
    mvhd64(input)
  } else {
    Error(Position(0,input))
  }
}

fn unknown_box_type(input:&[u8]) -> IResult<&[u8], MP4BoxType> {
  Done(input, MP4BoxType::Unknown)
}

named!(box_type<&[u8], MP4BoxType>,
  alt!(
    tag!("ftyp") => { |_| MP4BoxType::Ftyp } |
    tag!("moov") => { |_| MP4BoxType::Moov } |
    tag!("mdat") => { |_| MP4BoxType::Mdat } |
    tag!("free") => { |_| MP4BoxType::Free } |
    tag!("skip") => { |_| MP4BoxType::Skip } |
    tag!("wide") => { |_| MP4BoxType::Wide } |
    unknown_box_type
  )
);

// warning, an alt combinator with 9 branches containing a tag combinator
// can make the compilation very slow. Use functions as sub parsers,
// or split into multiple alt! parsers if it gets slow
named!(moov_type<&[u8], MP4BoxType>,
  alt!(
    tag!("mdra") => { |_| MP4BoxType::Mdra } |
    tag!("dref") => { |_| MP4BoxType::Dref } |
    tag!("cmov") => { |_| MP4BoxType::Cmov } |
    tag!("rmra") => { |_| MP4BoxType::Rmra } |
    tag!("iods") => { |_| MP4BoxType::Iods } |
    tag!("mvhd") => { |_| MP4BoxType::Mvhd } |
    tag!("clip") => { |_| MP4BoxType::Clip } |
    tag!("trak") => { |_| MP4BoxType::Trak } |
    tag!("udta") => { |_| MP4BoxType::Udta }
  )
);

named!(box_header<&[u8],MP4BoxHeader>,
  chain!(
    length: be_u32 ~
    tag: box_type  ,
    || { MP4BoxHeader{ length: length, tag: tag} }
  )
);

named!(moov_header<&[u8],MP4BoxHeader>,
  chain!(
    length: be_u32 ~
    tag: moov_type  ,
    || { MP4BoxHeader{ length: length, tag: tag} }
  )
);

enum MP4State {
  Main,
  Moov,
  Mvhd
}

pub struct MP4Consumer {
  state:      MP4State,
  moov_bytes: usize
}

impl MP4Consumer {
  fn new() -> MP4Consumer {
    MP4Consumer { state: MP4State::Main, moov_bytes: 0 }
  }

  fn consume_main(&mut self, input: &[u8]) -> ConsumerState {
    //println!("\nparsing box header:\n{}", input.to_hex(8));
    match box_header(input) {
      Done(i, header) => {
        match header.tag {
          MP4BoxType::Ftyp    => {
            println!("-> FTYP");
            match filetype_parser(&i[0..(header.length as usize - 8)]) {
              Done(_, filetype_header) => {
                println!("filetype header: {:?}", filetype_header);
                return ConsumerState::Await(header.length as usize, header.length as usize - 8);
              }
              Error(a) => {
                println!("ftyp parsing error: {:?}", a);
                assert!(false);
                return ConsumerState::ConsumerError(0);
              },
              Incomplete(_) => {
                println!("ftyp incomplete -> await: {}", input.len());
                return ConsumerState::Await(0, input.len() + 100);
              }
            }
          },
          MP4BoxType::Moov    => {
            println!("-> MOOV");
            self.state = MP4State::Moov;
            self.moov_bytes = header.length as usize - 8;
            return ConsumerState::Await(input.len() - i.len(), header.length as usize);
          },
          MP4BoxType::Mdat    => println!("-> MDAT"),
          MP4BoxType::Free    => println!("-> FREE"),
          MP4BoxType::Skip    => println!("-> SKIP"),
          MP4BoxType::Wide    => println!("-> WIDE"),
          MP4BoxType::Unknown => {
            println!("-> UNKNOWN");
            println!("bytes:\n{}", (input).to_hex(8));
          },
          _                   => { println!("invalid"); return ConsumerState::ConsumerError(0); }
        }

        // current producer position is at input.len()
        // I consumed input.len() - i.len() = 8
        // I want to advance from header.length()
        // offset to my new position: -input.len() + (input.len() - i.len()) + header.len() == header.len() - i.len()
        ConsumerState::Seek(input.len() - i.len(), SeekFrom::Current((header.length as usize - i.len() as usize - 8) as i64), 100)
      },
      Error(a) => {
        println!("mp4 parsing error: {:?}", a);
        assert!(false);

        ConsumerState::ConsumerError(0)
      },
      Incomplete(_) => {
        // FIXME: incomplete should send the required size
        println!("mp4 incomplete -> await: {}", input.len());

        ConsumerState::Await(0, input.len() + 100)
      }
    }
  }

  fn consume_moov(&mut self, input: &[u8]) -> ConsumerState {
    //println!("\nparsing moov box(remaining {} bytes):\n{}", self.moov_bytes, input.to_hex(8));
    if self.moov_bytes == 0 {
      //println!("finished parsing moov atom, continuing with main parser");
      self.state = MP4State::Main;
      return ConsumerState::Await(0, input.len());
    }
    match moov_header(input) {
      Done(i, header) => {
        match header.tag {
          MP4BoxType::Mvhd    => {
            println!("-> MVHD");
            self.state = MP4State::Mvhd;
            // TODO: check for overflow here
            self.moov_bytes = self.moov_bytes - (input.len() - i.len());
            return ConsumerState::Await(input.len() - i.len(), header.length as usize - 8);
          },
          MP4BoxType::Wide    => println!("-> WIDE"),
          MP4BoxType::Mdra    => println!("-> MDRA"),
          MP4BoxType::Dref    => println!("-> DREF"),
          MP4BoxType::Cmov    => println!("-> CMOV"),
          MP4BoxType::Rmra    => println!("-> RMRA"),
          MP4BoxType::Iods    => println!("-> IODS"),
          MP4BoxType::Clip    => println!("-> CLIP"),
          MP4BoxType::Trak    => println!("-> TRAK"),
          MP4BoxType::Udta    => println!("-> UDTA"),
          MP4BoxType::Unknown => println!("-> MOOV UNKNOWN"),
          _                   => { println!("invalid header here: {:?}", header.tag); return ConsumerState::ConsumerError(0); }
        }
        // TODO: check for overflow here
        self.moov_bytes = self.moov_bytes - header.length as usize;

        ConsumerState::Seek(input.len() - i.len(), SeekFrom::Current((header.length as usize - i.len() as usize - 8) as i64), 100)
      },
      Error(a) => {
        println!("moov parsing error: {:?}", a);
        assert!(false);

        ConsumerState::ConsumerError(0)
      },
      Incomplete(_) => {
        println!("moov incomplete -> await: {}", input.len());

        ConsumerState::Await(0, input.len() + 100)
      }
    }
  }

  pub fn consume_mvhd(&mut self, input: &[u8]) -> ConsumerState {
    //println!("parsing mvhd box(remaining moov bytes:{}):\n{}", self.moov_bytes, input.to_hex(8));
    match mvhd_box(input) {
      Done(i, movie_header) => {
        println!("correctly parsed movie header: {:?}", movie_header);
        self.state = MP4State::Moov;
        //TODO: check for overflow here
        self.moov_bytes = self.moov_bytes - (input.len() - i.len());

        ConsumerState::Await(input.len() - i.len(), 100)
      },
      Error(a) => {
        println!("mvhd parsing error: {:?}", a);
        assert!(false);

        ConsumerState::ConsumerError(0)
      },
      Incomplete(_) => {
        println!("mvhd incomplete -> await: {}", input.len());

        ConsumerState::Await(0, input.len() + 100)
      }
    }
  }
}

impl Consumer for MP4Consumer {
  fn consume(&mut self, input: &[u8]) -> ConsumerState {
    match self.state {
      MP4State::Main => {
        self.consume_main(input)
      },
      MP4State::Moov => {
        self.consume_moov(input)
      },
      MP4State::Mvhd => {
        self.consume_mvhd(input)
      }
    }
  }

  fn failed(&mut self, error_code: u32) {
    println!("failed with error code: {}", error_code);
  }

  fn end(&mut self) {
    println!("finish!");
  }
}

#[allow(unused_must_use)]
fn explore_mp4_file(filename: &str) {
  FileProducer::new(filename, 400).map(|producer: FileProducer| {
    println!("file producer created for {}", filename);
    let mut p = producer;
    let mut c = MP4Consumer{state: MP4State::Main, moov_bytes: 0};
    c.run(&mut p);

    //assert!(false);
  });
}

/*
#[test]
fn file_test() {
  explore_mp4_file("./small.mp4");
}


#[test]
fn bunny_test() {
  //explore_mp4_file("bigbuckbunny.mp4");
}
*/


