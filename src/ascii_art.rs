use crate::clock::SessionType;

pub fn render_art(session_type: &SessionType) {
    match session_type {
        SessionType::SESSION => {
            println!(
                r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣀⡤⣴⠞⠛⢒⡷⠾⣽⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⡠⠊⠁⡴⠁⠀⢠⠎⠀⠀⠈⠙⢦⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡞⠀⠀⠈⠀⠀⠀⠀⠀⢀⣀⡀⠀⠀⠹⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⡀⠀⠀⠲⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢼⣷⠀⣶⠄⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠈⠋⠀⢠⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠹⡄⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⢀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠑⠦⠤⠤⠤⠔⣺⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣹⠳⠤⣀⣀⣀⣀⣀⣠⠴⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣴⠷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣠⠎⣸⠀⠘⣦⠀⠀⠀⠀⠀⢐⠒⠀⠀⠀⠀⠀⠀⢸⡦⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀
⠀⢀⠜⠁⠀⡇⠀⠀⠈⢳⡀⠀⠀⠀⣘⡉⠃⠀⠀⠀⠀⣠⣾⢳⠀⠀⠀⠀⢀⣀⣀⡤⣤⣤⣤⣶⠶⠾⠿⠟⢹⡟
⣠⠏⠀⠀⣸⣀⣀⣀⠤⠤⠿⣖⠚⠉⢹⢻⠀⠀⠀⣠⠞⠁⣿⣼⠀⠀⣸⣿⣽⣿⠶⠟⠛⣭⡶⠆⠀⠀⠀⠀⡿⠀
⠛⠚⠉⠉⢹⠉⠀⠀⠀⠀⠀⠈⠳⣄⢸⣼⢀⡤⠚⠁⠀⠀⣿⣧⠀⢠⢧⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠀
⠀⠀⠀⠀⡟⠀⠀⠀⠀⠀⠀⠀⠀⠉⠳⠵⠏⠀⠀⠀⠀⠀⠉⠉⢀⢟⣾⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠀
⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣠⠤⠴⠒⡟⡟⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠏⠀⠀⠀
⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⣀⣀⡤⠤⠖⠒⢉⣉⣥⠤⠒⣻⣿⣾⡽⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡞⠀⠀⠀⠀
⠀⠀⣀⣀⠧⠤⠔⠒⠒⠉⠉⢡⣤⠖⠒⠊⠉⢁⣀⠭⣿⣿⣿⢳⠿⠁⠀⠀⠀⠀⠀⠀⣀⣠⣤⣴⠟⠁⠀⠀⠀⠀
⠈⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠻⢿⣶⣤⣛⡿⣟⣛⣿⢶⠇⠎⠀⠀⠀⣀⣤⣴⣶⠯⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⣿⣿⣯⣦⣤⣶⠾⠟⠛⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#
            );
        }

        SessionType::BREAK => {
            println!(
                r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣶⣶⣤⡀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⡙⢻⣿⣿⡄⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⢿⣿⣤⣜⣿⣿⣿⠇⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠉⣷⠀⠀⠉⠛⠿⠿⠿⠟⠋⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⣠⡤⢶⣖⠚⠁⠒⠇⢹⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢀⣀⣠⣤⣤⣶⣏⠻⣿⡈⠻⠇⠀⠀⠀⠈⢹⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠘⣧⡐⠖⠀⠀⠙⠧⠈⠁⠀⠀⠦⠴⠋⠀⣋⣽⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠈⢻⠀⠀⠀⠀⡀⠀⠠⢖⢀⡀⠀⠀⠀⠈⢩⣿⠛⣶⣤⡀⠀⠀⠀⠀⠀
⠀⠀⢸⡀⠐⠲⠖⠁⠀⠐⠂⠁⠀⠀⠀⠀⢀⡞⠉⠐⠋⣼⢷⡀⠀⠀⠀⠀
⠀⠀⠀⢳⡶⠖⣀⠀⠀⠀⠀⠀⠀⠀⢀⣴⡋⠻⡄⠀⠀⠐⠿⡇⠀⠀⠀⠀
⠀⠀⠀⠀⢹⡟⣧⠤⣀⣀⣀⣤⠴⠚⠛⠀⠉⠀⠀⠀⠀⠀⠈⣳⣦⡀⠀⠀
⠀⠀⠀⠀⠈⠳⠴⣦⣴⣤⣤⢿⣴⣦⣴⣦⠴⠤⠤⠤⠤⠶⠶⣿⠋⣹⣆⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡿⠿⢿⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣷⣶⣾⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡏⠉⢹⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⡇⠉⢹⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣶⣾⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢻⠟⠁
"#
            );
        }
    }
}
