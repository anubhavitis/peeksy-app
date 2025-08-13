use std::process::Command;

pub fn get_finder_selection() -> Option<Vec<String>> {
    let script = r#"
        tell application "Finder"
            set theSelection to selection as alias list
            set pathList to {}
            repeat with theItem in theSelection
                set end of pathList to POSIX path of (theItem as text)
            end repeat
            
            -- Join paths with newlines
            set AppleScript's text item delimiters to ASCII character 10
            set pathString to pathList as string
            set AppleScript's text item delimiters to ""
            
            return pathString
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;

    if output.status.success() {
        let output = String::from_utf8_lossy(&output.stdout);
        let output_str = output.trim().to_string();
        // println!("output_str: {}", output_str);

        if output_str.is_empty() {
            return None;
        }

        let paths: Vec<String> = output_str
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        // println!("Selected paths: {:?}", paths);

        if !paths.is_empty() {
            Some(paths)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_finder_selection_single() -> Option<String> {
    get_finder_selection()?.into_iter().next()
}
