# cloudwatch_logs_url [![CratesIOBadge]][CratesIO] [![DocsRsBadge]][DocsRs]

[CratesIOBadge]: https://img.shields.io/crates/v/cloudwatch_logs_url.svg
[CratesIO]: https://crates.io/crates/cloudwatch_logs_url
[DocsRsBadge]: https://docs.rs/cloudwatch_logs_url/badge.svg
[DocsRs]: https://docs.rs/cloudwatch_logs_url

**Generate AWS CloudWatch Logs URL**

---

```toml
[dependencies]
cloudwatch_logs_url = "1"
```

## Examples
### create_url_log_group()
```rs
let url = cloudwatch_logs_url::create_url_log_group(
    "ap-northeast-1",
    "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM"
);
```

### create_url_log_events()
```rs
let props = cloudwatch_logs_url::PropsCreateUrlLogEvents {
    region: "ap-northeast-1".to_string(),
    log_group_name: "/aws/lambda/luciferous-devio-index-cl-FunctionCheckIndividualS-qNWf7JYCZBBM".to_string(),
    log_stream_name: Some("2024/06/30/[39]3ca88f3a2fff4810b2de52cf027d0a40".to_string()),
    start: Some(-1800000),
    end: None,
    filter_pattern: None
};
let url = cloudwatch_logs_url::create_url_log_events(&props);
```

Options for URL generation are passed using Struct [PropsCreateUrlLogEvents][PropsCreateUrlLogEvents].
See the Struct [PropsCreateUrlLogEvents][PropsCreateUrlLogEvents] documentation for more information.

[PropsCreateUrlLogEvents]: https://docs.rs/cloudwatch_logs_url/latest/cloudwatch_logs_url/struct.PropsCreateUrlLogEvents.html

## Contribute

PRs accepted.

## License

MIT © sinofseven