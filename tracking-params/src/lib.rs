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
    /// List of domains for which this rule applies.
    host_path: Vec<M>,
    /// List of query string and fragment params to remove.
    params: Vec<M>,
    /// Handler to run any specific code for this rule.
    ///
    /// When defined, the handler run run before removing the matching
    /// params from the input url (defined in `params` field).
    /// The handler can change or return a completely different Url.
    /// Note: the handler is expected (although not validated for perf reason)
    /// to return url that belongs to the same origin and treated as such because
    /// * any matching params will still be removed later even if it's a different origin.
    /// * any defined rule for that new origin won't be applied
    ///
    /// A common use for handler function is to extract destination url from a query string
    /// from the input url. Consider the following link when click on a google search result:
    ///
    /// `https://www.google.com/url?sa=t&rct=j&esrc=s&source=web&cd=&ved=2ahUKEwi8hMv_nKP8AhWXhFwKHSetARUQFnoECBgQAQ&q=invalid_url&q=https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer`
    ///
    /// We can extract the destination url from the `q` or `url` query string (whichever is present)
    /// and skip sending traffic to `/url` endpoint. For such cases you can use
    /// the defined [`rules::extract_link_from_query_string`] function to extract valid url
    /// from one or many query strings.
    ///
    #[derivative(Debug = "ignore")]
    handler: Option<Box<dyn Fn(Url) -> Url + Sync + Send>>,
}

#[allow(dead_code)]
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
pub struct Cleaned {
    result: Url,
    handlers_used: i32,
}

impl std::ops::Deref for Cleaned {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl Cleaned {
    pub fn number_of_handlers_used(&self) -> i32 {
        self.handlers_used
    }
}

impl ToString for Cleaned {
    fn to_string(&self) -> String {
        self.result.as_ref().trim_end_matches('=').to_string()
    }
}

/// Removes tracking parameters from a given [`Url`] type.
///
/// This owns the input and returns a [`Cleaned`] type.
pub fn clean(url: Url) -> Cleaned {
    let mut handlers_used = 0;
    // Find applicable rules for this hostname
    let host_path = format!(
        "{}/{}",
        url.host_str().unwrap_or_default().trim_end_matches('/'),
        url.path()
    );
    let matched_rules = rules::GLOBAL_PARAMS
        .iter()
        .filter(|r| r.host_path.iter().any(|d| d.matches_str(Some(&host_path))))
        .collect::<Vec<_>>();

    // Run ths url through any rules that has a handler defined
    let rules_with_handles = matched_rules.iter().filter(|r| r.handler.is_some());

    let mut url = url;
    for rule in rules_with_handles {
        if let Some(handler) = &rule.handler {
            url = handler(url);
            handlers_used += 1;
        }
    }

    Cleaned {
        result: clean_hash_params(clean_query_string(url, &matched_rules), &matched_rules),
        handlers_used,
    }
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

/// Same as [`clean_str`] but returns the [`Cleaned`] type
pub fn clean_str_raw(url: &str) -> Result<Cleaned, url::ParseError> {
    let url = Url::parse(url)?;
    let cleaned = clean(url);

    Ok(cleaned)
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
    #[test_case(
        "https://www.google.com/url?q=http://www.capitalfm.com/news/tv-film/netflix/kaleidoscope-episode-order/&sa=D&source=calendar&usd=2&usg=AOvVaw0DUKL0RoiXBhCFMYU_U2jY",
        "http://www.capitalfm.com/news/tv-film/netflix/kaleidoscope-episode-order/"; "google result: no url query string"
    )]
    #[test_case(
        "https://www.google.com/url?sa=t&rct=j&esrc=s&source=web&cd=&ved=2ahUKEwi8hMv_nKP8AhWXhFwKHSetARUQFnoECBgQAQ&q=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FWeb%2FHTTP%2FHeaders%2FReferer&usg=AOvVaw0W8-mEp9kfFnE9c5S1DUp0",
        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer"; "google result: has q query string"
    )]
    #[test_case(
        "https://www.google.com/url?sa=t&rct=j&esrc=s&source=web&cd=&ved=2ahUKEwi8hMv_nKP8AhWXhFwKHSetARUQFnoECBgQAQ&q=invalid_url&q=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FWeb%2FHTTP%2FHeaders%2FReferer&usg=AOvVaw0W8-mEp9kfFnE9c5S1DUp0",
        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer"; "google result: has two q query strings"
    )]
    #[test_case(
        "https://www.google.com/url?sa=t&rct=j&esrc=s&source=web&cd=&ved=2ahUKEwi8hMv_nKP8AhWXhFwKHSetARUQFnoECBgQAQ&q=invalid_url&q=https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer",
        "https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer"; "google result: has two q query strings + unencoded value"
    )]
    #[test_case(
        "https://www.youtube.com/redirect?event=channel_description&redir_token=JWT_TOKEN&q=https%3A%2F%2Fwww.britishairways.com",
        "https://www.britishairways.com/"; "youtube /redirect: parses q"
    )]
    #[test_case(
        "https://www.youtube.com/redirect?event=channel_description&redir_token=JWT_TOKEN&q=invalid_url",
        "https://www.youtube.com/redirect?event=channel_description&redir_token=JWT_TOKEN&q=invalid_url"; "youtube /redirect: ingnores invalid q"
    )]
    #[test_case(
        "https://www.amazon.co.uk/gp/r.html?C=HEX&K=SOMEHEX&M=urn:rtn:msg:NUMBERS&R=SOMETHING&T=C&U=https%3A%2F%2Fwww.amazon.co.uk%2Fgp%2Fyour-account%2Forder-details%3ForderID%3DOREDER_ID%26ref_%3Dpreference&H=TEXT&ref_=pe_ref_with_underscore",
        "https://www.amazon.co.uk/gp/your-account/order-details?orderID=OREDER_ID&ref_=preference"; "amazon: extract from U"
    )]
    #[test_case(
        "https://email.clearscore.com/uni/track?uid=UUID&txnid=UUID&bsft_aaid=UUID&eid=UUID&mid=UUID&bsft_ek=RANDOM&bsft_mime_type=html&bsft_tv=27&bsft_lx=9&a=click&redir=https%3A%2F%2Fapp.clearscore.com%2Freport%3Futm_campaign%3Deml_lc_ca_alerts_2021_02_09%26utm_source%3Dblueshift%26utm_medium%3Demail%26utm_content%3Deml_lc_alerts_new_template_2022_04_01",
        "https://app.clearscore.com/report"; "generic email tracker: with track in path"
    )]
    #[test_case(
        "https://t.lever-analytics.com/email-link?dest=https%3A%2F%2Fwww.wired.co.uk%2F&eid=UUID&idx=1&token=TOKEN",
        "https://www.wired.co.uk/"; "generic email tracker: with dest in path"
    )]
    fn site_specific(input: &str, expected: &str) {
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
