fn _encode_text(text: &str, single: bool) -> String {
    let result = urlencoding::encode(text);
    if single {
        result.to_string()
    } else {
        urlencoding::encode(&result).to_string()
    }
}

fn encode_text(text: &str, single: bool) -> String {
    let tmp = _encode_text(text, single);
    tmp.replace("%", "$")
}


/// Returns CloudWatch Logs LogGroup URL.
/// # Arguments
/// - `region` - AWS Region
/// - `log_group_name` - LogGroup Name
///
/// # Examples
/// ```
/// let url = cloudwatch_logs_url::create_url_log_group(
///     "ap-northeast-1",
///     "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM"
/// );
pub fn create_url_log_group(region: &str, log_group_name: &str) -> String {
    format!("https://{0}.console.aws.amazon.com/cloudwatch/home?region={0}#logsV2:log-groups/log-group/{1}", region, encode_text(log_group_name, false))
}

/// Props for cloudwatch_logs_url::create_url_log_stream
pub struct PropsCreateUrlLogEvents {
    /// AWS Region
    pub region: String,
    /// LogGroup Name
    pub log_group_name: String,
    /// LogStream Name. if this is None, show all events.
    pub log_stream_name: Option<String>,
    /// The starting point of the period in milliseconds since UNIX epoch.
    /// To specify a relative time, provide a negative value in milliseconds.
    /// ex) The last 30 minutes = -1800000
    pub start: Option<i64>,
    /// The ending point of the period in milliseconds since UNIX epoch.
    pub end: Option<i64>,
    /// FilterPattern
    pub filter_pattern: Option<String>
}

/// Returns CloudWatch Logs Log Events URL.
///
/// # Examples
/// ```
/// let props = cloudwatch_logs_url::PropsCreateUrlLogEvents {
///     region: "ap-northeast-1".to_string(),
///     log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
///     log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
///     start: Some(-1800000),
///     end: None,
///     filter_pattern: None
/// };
/// let url = cloudwatch_logs_url::create_url_log_events(&props);
/// ```
pub fn create_url_log_events(props: &PropsCreateUrlLogEvents) -> String {
    let mut url = format!("{0}/log-events", create_url_log_group(&props.region, &props.log_group_name));

    if let Some(log_stream_name) = &props.log_stream_name {
        url += format!("/{0}", encode_text(log_stream_name, false)).as_str()
    }

    let equal_encoded = encode_text("=", true);
    let mut query: Vec<String> = Vec::new();

    if let Some(filter_pattern) = &props.filter_pattern {
        query.push(format!("filterPattern{0}{1}", &equal_encoded, encode_text(filter_pattern, false)));
    }
    if let Some(start) = props.start {
        query.push(format!("start{0}{1}", &equal_encoded, start));
    }
    if let Some(end) = props.end {
        query.push(format!("end{0}{1}", &equal_encoded, end));
    }

    if query.len() > 0 {
        url += format!("{0}{1}", encode_text("?", true), query.join(encode_text("&", true).as_str())).as_str()
    }

    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_log_group() {
        let region = "ap-northeast-1";
        let log_group_name = "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM";
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM";
        let actual = create_url_log_group(region, log_group_name);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_01() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
            start: Some(1719759600000),
            end: Some(1722437999000),
            filter_pattern: Some("{ $.level = \"DEBUG\" }".to_string()),
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events/2024$252F06$252F30$252F$255B39$255D3ca88f3a2fff4810b2de52cf027d0a40$3FfilterPattern$3D$257B$2520$2524.level$2520$253D$2520$2522DEBUG$2522$2520$257D$26start$3D1719759600000$26end$3D1722437999000";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_02() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
            start: Some(1719759600000),
            end: Some(1722437999000),
            filter_pattern: None,
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events/2024$252F06$252F30$252F$255B39$255D3ca88f3a2fff4810b2de52cf027d0a40$3Fstart$3D1719759600000$26end$3D1722437999000";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_03() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
            start: Some(-1800000),
            end: None,
            filter_pattern: None,
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events/2024$252F06$252F30$252F$255B39$255D3ca88f3a2fff4810b2de52cf027d0a40$3Fstart$3D-1800000";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_04() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
            start: None,
            end: None,
            filter_pattern: None,
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events/2024$252F06$252F30$252F$255B39$255D3ca88f3a2fff4810b2de52cf027d0a40";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_05() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: None,
            start: None,
            end: None,
            filter_pattern: None,
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_log_events_06() {
        let props = PropsCreateUrlLogEvents {
            region: "ap-northeast-1".to_string(),
            log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
            log_stream_name: None,
            start: Some(1719759600000),
            end: Some(1722437999000),
            filter_pattern: Some("{ $.level = \"DEBUG\" }".to_string()),
        };
        let expected = "https://ap-northeast-1.console.aws.amazon.com/cloudwatch/home?region=ap-northeast-1#logsV2:log-groups/log-group/$252Faws$252Flambda$252Fluciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM/log-events$3FfilterPattern$3D$257B$2520$2524.level$2520$253D$2520$2522DEBUG$2522$2520$257D$26start$3D1719759600000$26end$3D1722437999000";
        let actual = create_url_log_events(&props);
        assert_eq!(actual, expected);
    }
}

