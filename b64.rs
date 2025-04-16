//use std::io;
#[warn(unused_imports)]
fn main() {
    let hxd = "4a6f686e";
    match hexval(hxd) {
        Ok(bytes) => {
            for byte in bytes {
                println!("Dec: {}, Bin: {:08b}", byte, byte);
            }
        }
        Err(e) => {
            println!("Parsing failed with error: {}", e);
        }
    }
}


fn chartohex(ch:char)->Result<u8,String>{
    match ch{
        '0'..='9'=> Ok(ch as u8 -b'0'),
        'a'..='f'=> Ok(ch as u8 - b'a' +10),
        'A'..='F'=> Ok(ch as u8 - b'A' +10),
        _ =>Err(format!("Err with : {}",ch)),
    }
}



fn hexval(hex: &str)->Result<Vec<u8>,String>{
    let mut val= Vec::new();  
    let hexb= hex.as_bytes();
    if hex.len() %2!=0{
        return Err("Error".to_string());
    }
    else{
        for chunk in hexb.chunks(2){
            if chunk.len()!=2{
                return Err("not long enuf".to_string());
            }
            let high = chartohex(chunk[0]as char)?;
            let low = chartohex(chunk[1]as char)?;
            let binf= high*16 + low;
            val.push(binf);
        }    
        Ok(val)
    }
}
