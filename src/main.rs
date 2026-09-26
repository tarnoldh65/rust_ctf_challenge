//This code can be modified to create a reverse engineerring challenge.
fn print_decode(pcode: &str) {
    let cypher: [u8; 33] = [0x0d, 0x16, 0x11, 0x02, 0x07, 0x11, 0x14, 0x1f, 0x01, 0x06, 0x0c, 0x0b, 0x01, 0x17, 0x3b, 0x11, 0x0b, 0x0a, 0x3b, 0x06, 0x1d, 0x09, 0x08, 0x16, 0x01, 0x0d, 0x06, 0x13, 0x10, 0x0c, 0x0c, 0x01, 0x19];
    let mut count = 0;
    let mut decode = 0x00;
    let mut decode_txt = String::new();

    for element in cypher.iter() {
        decode = pcode.chars().nth(count).unwrap() as u8;
        decode_txt.push((decode ^ element).into());
        if count == pcode.len() - 1 {
            count = 0;
        } else {
            count += 1;
        }
    }

    println!("{}", decode_txt);
}
//isrmctf{this_is_not_the_flag_trust_me}

fn main() {
    let mut passcode = String::new();
    println!("Enter passcode: ");
    std::io::stdin().read_line(&mut passcode).expect("Failed to read line");
    let passcode = passcode.trim();
    print_decode(passcode);
}
