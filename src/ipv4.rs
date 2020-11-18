pub fn validate(pattern: &str) -> bool {
    
    let mut valid: bool = true;
    'outer: loop {
        
        let lambda = |c| c == '.' || c == '/';
        
        let slices: Vec<&str> = pattern.split(lambda).collect();
        if slices.len() != 5 {valid=false; break;}

        // handle subnet mask
        let subnet_mask: &str = slices[4];
        if subnet_mask == "" {valid=false; break;}
        if subnet_mask.len() > 2 {valid=false; break;}
        if !is_number(subnet_mask) {valid=false; break;}
        let subnet_mask: i32 = subnet_mask.parse().unwrap();
        if subnet_mask > 32 || subnet_mask < 0 {valid=false; break;}
       
        // handle octets
        let octets = &slices[..=3];
        for octet in octets {
            if octet == &"" {valid=false; break 'outer;}
            if octet.len() > 3 {valid=false; break 'outer;}
            if !is_number(octet) {valid=false; break 'outer;}
            let octet: i32 = octet.parse().unwrap();
            if octet > 255 || octet < 0 {valid=false; break 'outer;}
        }

        // handle separators
        let sep: String = pattern.matches(lambda).collect();
        if sep.len() != 4 {valid=false; break;}
        if sep != ".../" {valid=false; break;}
        
        break;
    }
    valid
}


fn is_number(octet: &str) -> bool {
    let mut check = true;
    for character in octet.chars() {
        if !character.is_numeric() {check=false; break}
    }
    check
}

