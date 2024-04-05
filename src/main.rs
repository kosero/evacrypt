use std::io;
use rand::{rngs::ThreadRng, thread_rng, Rng};

fn main() {
    println!("Şifreniz için karakter sayısını giriniz:\n(Önerilen 12)");

    let mut girdi: String = String::new(); 
    
    io::stdin().read_line(&mut girdi)
        .expect("\x1b[41mOkuma hatası!\x1b[0m");

    let belirlenen_sayi: usize = match girdi.trim().parse() { 
        Ok(sayi) => sayi,
        Err(_) => {
            eprintln!("Geçersiz sayı!"); 
            return;
        }
    };

    let sifre: String = sifre_uret(belirlenen_sayi);

    println!("Şifreniz: {}", sifre);
}

fn sifre_uret(uzunluk: usize) -> String {
    let mut rng: ThreadRng = thread_rng();
    let karakterler: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[{]};:',<.>/?";

    let sifre: String = (0..uzunluk)
        .map(|_| {
            let idx = rng.gen_range(0..karakterler.len());
            karakterler[idx] as char
        })
        .collect();

    sifre
}