pub fn split_message(content: &str, max_length: usize) -> Vec<String> {
    let is_code_block = content.starts_with("```") && content.ends_with("```");
    
    let (language, inner_content) = if is_code_block {
        let first_newline = content.find('\n').unwrap_or(3);
        let lang_str = content[3..first_newline].trim().to_string();
        let inner = &content[first_newline+1..content.len()-3];
        (lang_str, inner)
    } else {
        (String::new(), content)
    };
    
    let wrapper_length = if is_code_block {
        language.len() + 7
    } else {
        0
    };
    
    let adjusted_max_length = if max_length > wrapper_length {
        max_length - wrapper_length
    } else {
        max_length / 2
    };
    
    let mut messages = Vec::new();
    let mut remaining = inner_content;
    
    while !remaining.is_empty() {
        let split_point = if remaining.len() <= adjusted_max_length {
            remaining.len()
        } else {
            let potential_end = remaining[..adjusted_max_length].rfind('\n')
                .unwrap_or_else(|| {
                    let mut pos = adjusted_max_length;
                    while pos > 0 && !remaining.is_char_boundary(pos) {
                        pos -= 1;
                    }
                    pos
                });
            
            if potential_end == 0 {
                let mut pos = adjusted_max_length;
                while pos > 0 && !remaining.is_char_boundary(pos) {
                    pos -= 1;
                }
                pos
            } else {
                if remaining.as_bytes().get(potential_end) == Some(&b'\n') {
                    potential_end + 1
                } else {
                    potential_end
                }
            }
        };
        
        let chunk = &remaining[..split_point];
        
        let formatted_chunk = if is_code_block {
            format!("```{}\n{}\n```", language, chunk)
        } else {
            chunk.to_string()
        };
        
        messages.push(formatted_chunk);
        
        remaining = &remaining[split_point..];
    }
    
    messages
}

pub fn detect_language(command: &str) -> &str {
    if command.starts_with("echo") || command.starts_with("ls") || command.starts_with("pwd") {
        "sh"
    } else if command.starts_with("lscpu") || command.starts_with("uname") {
        "bash"
    } else if command.starts_with("systemctl") || command.starts_with("journalctl") {
        "bash"
    } else if command.starts_with("ps") || command.starts_with("grep") {
        "sh" 
    } else {
        ""
    }
}
