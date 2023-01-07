//! List of nasty parameters to be removed
//!
//! The original source of this data is from the source code of
//! `tracking-params` npm package by [dczysz](https://github.com/dczysz):
//!
//! [`https://github.com/dczysz/tracking-params/blob/5ccb3f8e3d4d6f3dfb88abe85a304fb78cfa41ce/src/params.ts`]
//! More sources:
//! * [`https://maxchadwick.xyz/tracking-query-params-registry/`]
//!
use url::Url;

use crate::{
    Rule,
    M::{self, *},
};

lazy_static::lazy_static! {
    pub(crate) static ref  GLOBAL_PARAMS: Vec<Rule> = vec![
        Rule {
            domains: vec![Any],
            params: UNIVERSAL_PARAMS.to_vec(),
            handler: None
        },
        Rule {
            domains: vec![Contains("amazon")],
            params: vec![
                Exact("_encoding"),
                Exact("creative"),
                Exact("creativeASIN"),
                Exact("dchild"),
                Exact("ie"),
                Exact("linkCode"),
                Exact("linkId"),
                Exact("orig"),
                Exact("psc"),
                Exact("qid"),
                Exact("ref"),
                Exact("refRID"),
                Exact("sr"),
                Exact("tag"),
            ],
            handler: Some(Box::new(|url| extract_link_from_query_string(url, vec!["U"], Some(vec!["gp/r.html" ]))))
        },
        Rule {
            domains: vec![Contains("bing")],
            params: vec![
                Exact("cvid"),
                Exact("form"),
                Exact("pq"),
                Exact("qs"),
                Exact("sc"),
                Exact("sk"),
                Exact("sp"),
            ],
            handler: None
        },
        Rule {
            domains: vec![Contains("google")],
            params: vec![
                Exact("cvid"),
                Exact("ei"),
                Exact("gws_rd"),
                Exact("sei"),
                Exact("ved"),
            ],
            handler: None
        },
        Rule {
            domains: vec![ContainsAll(vec!["google", "/url"])],
            params: vec![
                Exact("usg"),
            ],
            handler: Some(Box::new(|url| extract_link_from_query_string(url, vec!["q", "url"], None))),
        },

        Rule {
            domains: vec![Contains("instagram")],
            params: vec![
                Exact("igshid"),
            ],
            handler: None
        },
        Rule {
            domains: vec![Contains("nytimes")],
            params: vec![
                Exact("emc"),
                Exact("partner"),
            ],
            handler: None
        },
        Rule {
            domains: vec![Contains("reddit")],
            params: vec![
                Exact("context"),
                Exact("ref"),
                Exact("ref_source"),
                Exact("st"),
            ],
            handler: None
        },
        Rule {
            domains: vec![Contains("twitter")],
            params: vec![
                Exact("context"),
                Exact("vertical"),
                Exact("src"),
                Exact("s"),
                Exact("ref_src"),
                Exact("ref_url"),
            ],
            handler: None
        },
        Rule {
            domains: vec![Contains("youtube")],
            params: vec![
                Contains("ab_channel"),
                Contains("attr_tag"),
                Contains("feature"),
                Contains("kw"),
            ],
            handler: Some(Box::new(|url| extract_link_from_query_string(url, vec!["q"], Some(vec!["redirect" ]))))
        },
        // https://community.spotify.com/t5/Desktop-Windows/si-Parameter-in-Spotify-URL-s/td-p/4538290
        Rule {
            domains: vec![Contains("spotify")],
            params: vec![
                Exact("si")
            ],
            handler: None
        },
        // https://partnerhelp.ebay.com/helpcenter/s/article/What-are-the-parameters-of-an-EPN-link#tracking-link-format
        Rule {
            domains: vec![Contains("ebay")],
            params: vec![
                Exact("mkevt"),
                Exact("mkcid"),
                Exact("mkrid"),
                Exact("campid"),
                Exact("toolid"),
                Exact("customid"),
            ],
            handler: None
        },
    ];

    static ref UNIVERSAL_PARAMS: Vec<M> = vec![
        Exact("__twitter_impression"),
        Exact("_hsenc"),
        Exact("_openstat"),
        Exact("action_object_map"),
        Exact("action_ref_map"),
        Exact("action_type_map"),
        Exact("adgroupid"),
        Exact("amp"),
        Exact("campaignid"),
        Exact("CNDID"),
        Exact("fb_action_ids"),
        Exact("fb_action_types"),
        Exact("fb_ref"),
        Exact("fb_source"),
        Exact("fbclid"),
        Exact("feeditemid"),
        Exact("ga_campaign"),
        Exact("ga_content"),
        Exact("ga_medium"),
        Exact("ga_place"),
        Exact("ga_source"),
        Exact("ga_term"),
        Exact("gclid"),
        Exact("gs_l"),
        Exact("hmb_campaign"),
        Exact("hmb_medium"),
        Exact("hmb_source"),
        Exact("mbid"),
        Exact("mc_cid"),
        Exact("mc_eid"),
        Exact("mkt_tok"),
        Exact("referrer"),
        Exact("spJobID"),
        Exact("spMailingID"),
        Exact("spReportId"),
        Exact("spUserID"),
        Exact("wt_mc_o"),
        Exact("WT.mc_ev"),
        Exact("WT.mc_id"),
        Exact("WT.srch"),
        Exact("yclid"),

        StartsWith("pd_rd"),
        StartsWith("pf_rd"),
        StartsWith("utm_"),
        // Matomo: https://matomo.org/docs/tracking-campaigns/
        StartsWith("mtm_"),
        StartsWith("matomo_"),
        // Hubspot: https://knowledge.hubspot.com/ads/ad-tracking-in-hubspot
        StartsWith("hsa_"),
        //Piwik
        StartsWith("pk_"),
        //Listrak
        StartsWith("trk_"),
        // Microsoft Advertising: https://help.ads.microsoft.com/apex/index/3/en/60000
        Exact("msclkid"),
        // Google advertising:
        Exact("_ga"),
        Exact("gclid"),
        Exact("gclsrc"),


    ];

}

/// Given a `url` extract a valid link from the query string `query`.
///
/// Optionally specify list of path components that much mach
/// before extracting the links.
fn extract_link_from_query_string(
    url: Url,
    queries: Vec<&'static str>,
    path_match: Option<Vec<&'static str>>,
) -> Url {
    if let Some(path_match) = path_match {
        if !path_match.iter().all(|p| url.path().contains(p)) {
            return url;
        }
    }
    for query in queries {
        for (_, possible_url) in url.query_pairs().filter(|(k, _)| k.eq(query)) {
            if let Ok(found_url) = urlencoding::decode(&possible_url)
                .map_err(anyhow::Error::from)
                .and_then(|decoded| Url::parse(&decoded).map_err(anyhow::Error::from))
            // must be valid url,
            {
                return found_url;
            }
        }
    }

    url
}
