mod ref;

//returns x if flag is set, otherwhise y
pub fn f1(x: u32, y: u32, flag: bool)
{
    let result = if flag { x } else { y };    
}

pub fn f2