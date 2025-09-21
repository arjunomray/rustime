use crate::services::clock::SessionType;

pub fn render_art(session_type: &SessionType) {
    match session_type {
        SessionType::SESSION => {
            println!(
                r#"
 ______________
||            ||
||            ||
||            ||
||            ||
||____________||
|______________|        /| ､      
 \\############\\      (°､ ｡ 7    
  \\############\\      |､  ~ヽ   
   \      ____    \     じしf_,)〳
    \_____\___\____\
"#
            );
        }

        SessionType::BREAK => {
            println!(
                r#"
        /| ､      
Zzz~   (~､ ~ 7    
        |､  ~ヽ   
        じしf_,)〳
"#
            );
        }
    }
}
