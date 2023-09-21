use crate::support::sys_write;

pub unsafe fn strlen(mut s: *const u8) -> usize {
  let mut count = 0;
  while *s != b'\0' {
      count += 1;
      s = s.add(1);
  }
  count
}

pub fn print_str(s: &[u8]) {
  unsafe {
      sys_write(s.as_ptr(), s.len());
  }
}

pub fn print_num(n: usize) {
  if n > 9 {
      print_num(n / 10);
  }
  let c = b'0' + (n % 10) as u8;
  print_str(&[c]);
}

fn nth(n: u8) -> usize
{
    let mut i:usize = 0;
    for _ in 0..n {
        i = i + 1;
    }
    i
}

pub fn ascii_to_digit(character: u8) -> Option<usize> {
    match character {
        b'0' => Some(nth(0)),
        b'1' => Some(nth(1)),
        b'2' => Some(nth(2)),
        b'3' => Some(nth(3)),
        b'4' => Some(nth(4)),
        b'5' => Some(nth(5)),
        b'6' => Some(nth(6)),
        b'7' => Some(nth(7)),
        b'8' => Some(nth(8)),
        b'9' => Some(nth(9)),
        _ => None,
    }
}

pub fn from_radix_10(text: &[u8]) -> (usize, usize) {
  let mut index:usize = 0;
  let mut number:usize = 0;
  while index != text.len() {
      if let Some(digit) = ascii_to_digit(text[index]) {
          number *= nth(10);
          number += digit;
          index += 1;
      } else {
          break;
      }
  }
  (number, index)
}

