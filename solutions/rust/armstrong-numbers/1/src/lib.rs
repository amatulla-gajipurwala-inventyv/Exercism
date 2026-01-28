pub fn is_armstrong_number(num: u32) -> bool {
    let b:u32=num.to_string().len()as u32;
    let mut n=num;
    let mut a:u32=0;
    while n>0{
        let d=n%10;
        a+=d.pow(b);
        n=n/10;
    }
    if a==num{
        return true;
    }
    return false;
    
}
