pub mod leap_year_judgement{ 
    pub fn is_leap(&year :i32) -> bool {
        if ((year%4 == 0) && (year%100 != 0)) or (year%400 == 0) {
            return true;
        } else {
            return false;
        }
    }
    
}