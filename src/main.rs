#![feature(type_ascription)]
#![feature(slice_patterns)]

extern crate flate2;
extern crate byteorder;
extern crate crc;

use std::env;
use std::borrow::BorrowMut;

use std::io::prelude::*;

use std::fs::File;
use std::io::{ Cursor };

use byteorder::{ BigEndian, ReadBytesExt };
use crc::{crc32};
use flate2::{ Decompress, Flush, Status };

enum ChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,
    UNKNOWN
}

fn main() {
    println!("Hello, world!");
    let mut args_iter = env::args();
    args_iter.next();
    let path = args_iter.next();

    if let None = path {
        println!("no lenna!");
        return;
    }

    let lenna_png_res = File::open(path.unwrap());

    if let Result::Err(_) = lenna_png_res {
        println!("couldn't open the file");
        return;
    }

    let mut lenna_png : File = lenna_png_res.unwrap();

//    let lenna_data : Bytes<std::fs::File> = lenna_png.unwrap().bytes();

    {
        let mut header_buf: [u8; 8] = [0; 8];
        let header_read_len = lenna_png.read(&mut header_buf[..]).unwrap();
        assert!(header_read_len == 8);

        println!("{:X} {:X} {:X} {:X} {:02X} {:X} {:X} {:X}", header_buf[0], header_buf[1], header_buf[2], header_buf[3], header_buf[4], header_buf[5], header_buf[6], header_buf[7]);

        if header_buf != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
            println!("it ain't a PNG!");
            return;
        }
    }

    let mut big_data_buf : Vec<u8> = vec![0; 473766];

    loop {
        let len = {
            let mut len_buf: [u8; 4] = [0; 4];
            lenna_png.read(&mut len_buf).unwrap();

            print!("Len_Buf:");
            for x in 0..4  {
               print!("{:?}",len_buf[x]);
            }
            println!("");

            let len_u32: u32 = Cursor::new(&len_buf[..]).read_u32::<BigEndian>().unwrap();
            len_u32 as usize
        };

        if len == 0 {
            break
        }

        println!("len: {}", len);

        let mut data_borrow : &mut [u8] = big_data_buf.borrow_mut();
        let mut data = &mut data_borrow[0..len+4];
        let read_len = lenna_png.read(&mut data).unwrap();
        assert!(read_len == (len + 4) as usize);

        //let concat_buf = vec![0;
        let calc_crc2 = crc32::checksum_ieee(data);

//        let crc : [u8; 4];
        {
            let mut crc_buf : [u8; 4] = [0; 4];
            let read_len = lenna_png.read(&mut crc_buf).unwrap();
            assert!(read_len == 4);
            let crc = Cursor::new(&crc_buf[..]).read_u32::<BigEndian>().unwrap();

            assert_eq!(calc_crc2, crc);
        }

        let chunk_type = {
            let chunk_type_buf: &[u8] = &data[0..4];
            println!("chunk_type: {:?}", chunk_type_buf);

            match chunk_type_buf {
                b"IHDR" => ChunkType::IHDR,
                b"PLTE" => ChunkType::PLTE,
                b"IDAT" => ChunkType::IDAT,
                b"IEND" => ChunkType::IEND,
                _ => ChunkType::UNKNOWN
            }

        };
        let chunk_data: &[u8] = &data[4..];

        match chunk_type {
            ChunkType::IHDR => {

            },
            ChunkType::IDAT => {
                let mut decompressed_chunk_data = vec![0; len * 100];
                let mut decompressor = Decompress::new(true);

                println!("chunk_data len: {}", chunk_data.len());
                let status = decompressor.decompress_vec(chunk_data,&mut decompressed_chunk_data,Flush::Sync).unwrap();

                match status {
                    Status::Ok => println!("OK!"),
                    Status::StreamEnd => println!("Stream is OVER"),
                    Status::BufError => println!("buf err")
                }
                println!("decompressed data: {:?}", decompressed_chunk_data.len());
            }
            _ => {

            }
        }
        /*

        */

    }



}
