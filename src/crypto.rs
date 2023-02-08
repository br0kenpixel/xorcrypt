use spinoff::{spinners, Color, Spinner};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

const MAGIC_BYTE: u8 = 0xEA;

pub(crate) fn encrypt_file(target: &PathBuf, output: PathBuf, password: &str) {
    let password_len = password.len();
    let tmp_out = output.with_extension("tmp");

    let spinner = Spinner::new(
        spinners::Dots,
        format!("Encrypting {}", target.display()),
        Color::Yellow,
    );

    {
        let mut reader = File::open(target).unwrap();
        let mut writer = File::create(&tmp_out).unwrap();
        let mut chunk: Vec<u8> = Vec::with_capacity(password_len);

        bufferred_read(&reader, &mut chunk, 1);
        if chunk[0] == MAGIC_BYTE {
            spinner.stop_with_message(&format!(
                "{}: found magic byte, is this file already encrypted?",
                target.display()
            ));
            return;
        }
        chunk.clear();
        reader.rewind().unwrap();

        writer.write_all(&[MAGIC_BYTE]).unwrap();
        while bufferred_read(&reader, &mut chunk, password_len) > 0 {
            xor_chunk(&mut chunk, password);
            writer.write_all(&chunk).unwrap();
        }
    }

    fs::rename(tmp_out, output).unwrap();
    spinner.success(&format!("Encrypted {}", target.display()));
}

pub(crate) fn decrypt_file(target: &PathBuf, output: PathBuf, password: &str) {
    let password_len = password.len();
    let tmp_out = output.with_extension("tmp");

    let spinner = Spinner::new(
        spinners::Dots,
        format!("Decrypting {}", target.display()),
        Color::Yellow,
    );

    {
        let reader = File::open(target).unwrap();
        let mut writer = File::create(&tmp_out).unwrap();
        let mut chunk: Vec<u8> = Vec::with_capacity(password_len);

        bufferred_read(&reader, &mut chunk, 1);
        if chunk[0] != MAGIC_BYTE {
            spinner.stop_with_message(&format!(
                "{}: missing magic byte, is this file encrypted?",
                target.display()
            ));
            return;
        }
        chunk.clear();

        while bufferred_read(&reader, &mut chunk, password_len) > 0 {
            xor_chunk(&mut chunk, password);
            writer.write_all(&chunk).unwrap();
        }
    }

    fs::rename(tmp_out, output).unwrap();
    spinner.success(&format!("Decrypted {}", target.display()));
}

fn bufferred_read(file: &File, buf: &mut Vec<u8>, len: usize) -> usize {
    buf.clear();
    file.take(len as u64).read_to_end(buf).unwrap()
}

fn xor_chunk(chunk: &mut [u8], password: &str) {
    for (index, byte) in chunk.iter_mut().enumerate() {
        let xor_byte: char = password.chars().nth(index).unwrap();
        let xor_byte = xor_byte as u8;

        *byte ^= xor_byte;
    }
}
