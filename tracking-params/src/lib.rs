//! # tracking-params
//!
//! Removes unwanted tracking parameters from a given URLs.
//!
//! ```rust
//! let dirty_url = url::Url::parse("https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5EdUmBgUY")?;
//! let clean_url = tracking_params::clean(dirty_url); // returns `Cleaned` which derefs to `url::Url`
//!
//! assert_eq!(
//!     clean_url.to_string(),
//!     "https://twitter.com/elonmusk/status/1608273870901096454".to_string() // No `ref_src` tracking params
//! );
//!
//! # Ok::<_, url::ParseError>(())
//! ```
use derivative::Derivative;
use url::Url;

mod rules;

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Rule {
    domains: Vec<M>,
    params: Vec<M>,
    #[derivative(Debug = "ignore")]
    handler: Option<Box<dyn Fn(Url) -> Url + Sync + Send>>,
}

#[derive(Clone, Debug)]
pub(crate) enum M {
    Any,
    AllBut(&'static str),
    ContainsAll(Vec<&'static str>),
    Exact(&'static str),
    StartsWith(&'static str),
    Contains(&'static str),
}

impl M {
    fn matches_str(&self, input: Option<&str>) -> bool {
        self.matches(input.map(|i| i.as_bytes()))
    }

    fn matches(&self, input: Option<&[u8]>) -> bool {
        match input {
            Some(input) => match self {
                M::Any => true,
                M::Exact(e) => input.eq(e.as_bytes()),
                M::StartsWith(sw) => input.starts_with(sw.as_bytes()),
                M::Contains(c) => input.windows(c.len()).any(|w| w.eq(c.as_bytes())),
                M::ContainsAll(all) => all
                    .iter()
                    .map(|a| M::Contains(a))
                    .collect::<Vec<_>>()
                    .iter()
                    .all(|a| a.matches(Some(input))),

                M::AllBut(c) => !c.as_bytes().eq(input),
            },
            None => match self {
                M::Any => true,

                M::Exact(_)
                | M::StartsWith(_)
                | M::Contains(_)
                | M::ContainsAll(_)
                | M::AllBut(_) => false,
            },
        }
    }
}

/// A cleaned URL.
///
///
/// This is a wrapper around and `Deref` into [`url::Url`] that also overriedes the `ToString`
/// to prevent extra `=` at the end of the URL when the query string does
/// not have any value.
///
/// eg.`https://example.com/?json` turns to `https://example.com/?json=` when
/// `ToString` is called on the Url type.
///

#[derive(Debug, Clone)]
pub struct Cleaned(Url);

impl std::ops::Deref for Cleaned {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for Cleaned {
    fn to_string(&self) -> String {
        self.0.as_ref().trim_end_matches('=').to_string()
    }
}

/// Removes tracking parameters from a given [`Url`] type.
///
/// This owns the input and returns a [`Cleaned`] type.
pub fn clean(url: Url) -> Cleaned {
    // Find applicable rules for this hostname
    let host_path = format!(
        "{}/{}",
        url.host_str().unwrap_or_default().trim_end_matches('/'),
        url.path()
    );
    let matched_rules = rules::GLOBAL_PARAMS
        .iter()
        .filter(|r| r.domains.iter().any(|d| d.matches_str(Some(&host_path))))
        .collect::<Vec<_>>();

    // Run ths url through any rules that has a handler defined
    let rules_with_handles = matched_rules.iter().filter(|r| r.handler.is_some());

    let mut url = url;
    for rule in rules_with_handles {
        if let Some(handler) = &rule.handler {
            url = handler(url);
        }
    }

    Cleaned(clean_hash_params(
        clean_query_string(url, &matched_rules),
        &matched_rules,
    ))
}

/// Removes tracking parameters from a given string reference that is expected to be a valid URL.
///
/// This returns the cleaned URL as String.
/// This returns error when the given input is not a valid URL.
pub fn clean_str(url: &str) -> Result<String, url::ParseError> {
    let url = Url::parse(url)?;
    let url = clean(url);

    Ok(url.to_string())
}

fn clean_query_string(url: Url, rules: &[&Rule]) -> Url {
    let mut url = url;
    if url.query().is_none() {
        return url;
    }

    let queries = url
        .query_pairs()
        .into_iter()
        .filter(|(k, _)| {
            !rules
                .iter()
                .any(|r| r.params.iter().any(|p| p.matches_str(Some(k.as_ref()))))
        })
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<Vec<_>>();

    url.set_query(None); // clear all queries
    if queries.is_empty() {
        return url; // prevents dangling `?` at the end (as a result of `query_pairs_mut` call below)
    }

    let mut params = url.query_pairs_mut();

    for (k, v) in queries {
        params.append_pair(k.as_ref(), v.as_ref());
    }

    params.finish().to_owned()
}

fn clean_hash_params(url: Url, rules: &[&Rule]) -> Url {
    let mut url = url;

    if let Some(f) = url.fragment() {
        let mut fr = String::with_capacity(f.len());

        for item in f.split('&') {
            if let Some(key) = item.split('=').take(1).collect::<Vec<_>>().first() {
                if !rules
                    .iter()
                    .any(|r| r.params.iter().any(|p| p.matches_str(Some(*key))))
                {
                    fr.push_str(item);
                    fr.push('&');
                }
            }
        }
        if fr.ends_with('&') {
            fr.remove(fr.len() - 1);
        }

        if fr.is_empty() {
            url.set_fragment(None); // prevents dangling `#` at the end
        } else {
            url.set_fragment(Some(fr.as_str()));
        }
    }

    url
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    //
    // Query
    //
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5Etfw",
        "https://twitter.com/elonmusk/status/1608273870901096454"; "twitter: single bad query"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home",
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home"; "twitter: single good query"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5Etfw&from=home",
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home"; "twitter: good & bad query"
    )]
    //
    // Query without value
    //
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?ref_src",
        "https://twitter.com/elonmusk/status/1608273870901096454"; "twitter: single bad query without value"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home&ref_src",
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home"; "twitter: bad query without value"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?from",
        "https://twitter.com/elonmusk/status/1608273870901096454?from"; "twitter: single good query without value"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?from&ref_src=abc",
        "https://twitter.com/elonmusk/status/1608273870901096454?from"; "twitter: bad query with value good query without value"
    )]
    fn query(input: &str, expected: &str) {
        test_common(input, expected)
    }

    //
    // Hash Params
    //
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454#ref_src=twsrc%5Etfw",
        "https://twitter.com/elonmusk/status/1608273870901096454"; "twitter: single bad hash param"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454#from=home",
        "https://twitter.com/elonmusk/status/1608273870901096454#from=home"; "twitter: single good hash param"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454#ref_src=twsrc%5Etfw&from=home",
        "https://twitter.com/elonmusk/status/1608273870901096454#from=home"; "twitter: good & bad hash param"
    )]
    fn hash(input: &str, expected: &str) {
        test_common(input, expected)
    }

    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5Etfw&from=home#ref_src=twsrc%5Etfw&from=home",
        "https://twitter.com/elonmusk/status/1608273870901096454?from=home#from=home"; "twitter: good & bad hash param and query"
    )]
    #[test_case(
        "https://twitter.com/elonmusk/status/1608273870901096454?ref_src=twsrc%5Etfw#ref_src=twsrc%5Etfw",
        "https://twitter.com/elonmusk/status/1608273870901096454"; "twitter: all bad hash param and query"
    )]
    fn both(input: &str, expected: &str) {
        test_common(input, expected)
    }

    // Misc
    #[test_case(
        "https://example.com/my-post?utm_xyx=abc&id=12456",
        "https://example.com/my-post?id=12456"; "misc: all utm_ query"
    )]
    #[test_case(
        "https://example.com/my-post?utm_xyx=abc&id=12456&utm_life=asssc",
        "https://example.com/my-post?id=12456"; "misc: all utm_ query (two)"
    )]
    // #[test_case(
    //     "https://href.li/?https://whatsmyreferer.com/?utm_campaign=twsrc^dUmBgUY",
    //     "https://href.li/?https://whatsmyreferer.com/?utm_campaign=twsrc^dUmBgUY"; "misc: href.li"
    // )]
    #[test_case(
        "https://whatsmyreferer.com/?json",
        "https://whatsmyreferer.com/?json"; "misc: no trailing eq ="
    )]
    fn misc(input: &str, expected: &str) {
        test_common(input, expected)
    }

    #[test_case(
        "https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&ved=2ahUKEwi8hMv_nKP8AhWXhFwKHSetARUQFnoECBgQAQ&url=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FWeb%2FHTTP%2FHeaders%2FReferer&usg=AOvVaw0W8-mEp9kfFnE9c5S1DUp0",
        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer"; "google result: parses the url query string"
    )]
    fn google(input: &str, expected: &str) {
        test_common(input, expected)
    }

    fn test_common(input: &str, expected: &str) {
        let result = clean(Url::parse(input).unwrap()).to_string();

        assert_eq!(
            result,
            expected.to_string(),
            "\nExpected: `{}`\n   Found: `{}`",
            expected,
            result
        );
    }

    #[test]
    fn matcher() {
        assert!(M::Any.matches_str(Some("yoyo")), "any");
        assert!(
            M::Contains("utm_").matches_str(Some("abc_utm_")),
            "contains"
        );
        assert!(M::Exact("utm_").matches_str(Some("utm_")), "exact");
        assert!(
            M::StartsWith("utm_").matches_str(Some("utm_abc")),
            "starts_with"
        );
    }
}
