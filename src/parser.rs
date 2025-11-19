fn find_tag_content(s: &str, tag: &str) -> Option<String> {
    let mut i = 0usize;
    let bytes = s.as_bytes();
    let tag_open = format!("<{}>", tag);
    let tag_close = format!("</{}>", tag);
    while i < bytes.len() {
        // find '<'
        if bytes[i] == b'<' {
            // check if this is the opening tag exactly (no attributes)
            if s[i..].starts_with(&tag_open) {
                let start = i + tag_open.len();
                // find closing tag from start
                if let Some(pos) = s[start..].find(&tag_close) {
                    let content = s[start..start + pos].trim().to_string();
                    return Some(content);
                } else {
                    return None;
                }
            }
        }
        i += 1;
    }
    None
}

/// free-reseau.fr return a XML as follow. A invalid input won't return <value><boolean>VAL</boolean></value> but a string directly <value>ERROR STRING</value>
///
/// ```xml
/// <?xml version="1.0"?>
/// <methodResponse>
///   <params>
///     <param>
///       <value><boolean>0</boolean></value>
///     </param>
///   </params>
/// </methodResponse>
/// ```
///
pub fn extract_status_from_xml(s: &str) -> Option<bool> {
    let content = find_tag_content(s, "boolean")?;
    match content.as_str() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}
#[cfg(test)]
pub mod tests {

    use crate::parser::extract_status_from_xml;

    static ONLINE: &str = r#"
<?xml version="1.0"?>
<methodResponse>
  <params>
    <param>
      <value><boolean>1</boolean></value>
    </param>
  </params>
</methodResponse>
"#;
    static OFFLINE: &str = r#"
<?xml version="1.0"?>
<methodResponse>
  <params>
    <param>
      <value><boolean>0</boolean></value>
    </param>
  </params>
</methodResponse>
"#;

    static ERROR: &str = r#"
<?xml version="1.0"?>
<methodResponse>
  <params>
    <param>
      <value>Erreur !</value>
    </param>
  </params>
</methodResponse>
"#;

    #[test]
    fn test_parse_online() {
        assert_eq!(extract_status_from_xml(ONLINE), Some(true))
    }
    #[test]
    fn test_parse_offline() {
        assert_eq!(extract_status_from_xml(OFFLINE), Some(false))
    }
    #[test]
    fn test_parse_error() {
        assert_eq!(extract_status_from_xml(ERROR), None)
    }
    #[test]
    fn test_parse_invalids() {
        assert_eq!(
            extract_status_from_xml(
                r#"
            <?xml version="1.0"?>
            <methodResponse>
            <params>
                <param>
                <value><boolean>1/boolean></value>
                </param>
            </params>
            </methodResponse>
            "#
            ),
            None
        );
        assert_eq!(
            extract_status_from_xml(
                r#"
            <?xml version="1.0"?>
            <methodResponse>
            <params>
                <param>
                <value><boolean>11</boolean></value>
                </param>
            </params>
            </methodResponse>
            "#
            ),
            None
        );
    }
}
