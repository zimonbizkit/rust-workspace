
const OPEN_BRACE: u8 = 91;
const CLOSE_BRACE: u8 = 93;
const OPEN_BRACKET: u8 = 123;
const CLOSE_BRACKET: u8 = 125;
const OPEN_PARENTH: u8 = 40;
const CLOSE_PARENTH: u8 = 41;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack : Vec<u8> = Vec::new();
    let mut inverse_stack : Vec<u8> = Vec::new();
    let mut stack_overflow : bool = false;

    'charcheck: for ch in string.as_bytes() {
        if [OPEN_BRACE,OPEN_BRACKET,OPEN_PARENTH].contains(ch) {
            stack.push(*ch);
                if ch == &OPEN_PARENTH {
                    inverse_stack.push(*ch +1);
                }else{
                    inverse_stack.push(*ch +2);
                }
        }else if [CLOSE_BRACE,CLOSE_BRACKET,CLOSE_PARENTH].contains(ch) {
            if stack.pop().is_none() {
                stack_overflow = true;
                break 'charcheck;        
            }else if inverse_stack.pop().unwrap() != *ch {
                stack_overflow = true;
                break 'charcheck;
            }
        }
    }
    stack.len() == 0 && inverse_stack.len() == 0 && !stack_overflow
}
