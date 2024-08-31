pub fn get_ascii(digit: i32) -> String {
    match digit {
        0 => String::from(
            r#"
 0000 
00  00
00  00
00  00
 0000 "#,
        ),
        1 => String::from(
            r#"
1111  
  11  
  11  
  11  
111111"#,
        ),
        2 => String::from(
            r#"
 2222 
22  22
   22 
  22  
222222"#,
        ),
        3 => String::from(
            r#"
 3333 
33  33
   333
33  33
 3333 "#,
        ),
        4 => String::from(
            r#"
44  44
44  44
444444
    44
    44"#,
        ),
        5 => String::from(
            r#"
55555 
55    
55555 
    55
55555 "#,
        ),
        6 => String::from(
            r#"
 6666 
66    
66666 
66  66
 6666 "#,
        ),
        7 => String::from(
            r#"
777777
   77 
  77  
 77   
77    "#,
        ),
        8 => String::from(
            r#"
 8888 
88  88
 8888 
88  88
 8888 "#,
        ),
        9 => String::from(
            r#"
 9999 
99  99
 99999
    99
 9999 "#,
        ),

        _ => String::from(" "),
    }
    .to_string()
}
