//! List of nasty parameters to be removed
//!
//! The original source of this data is from the source code of
//! `tracking-params` npm package by [dczysz](https://github.com/dczysz):
//!
//! [`https://github.com/dczysz/tracking-params/blob/5ccb3f8e3d4d6f3dfb88abe85a304fb78cfa41ce/src/params.ts`]
use crate::{
    Rule,
    M::{self, Any, Contains, Exact, StartsWith},
};

lazy_static::lazy_static! {
    pub(crate) static ref  GLOBAL_PARAMS: Vec<Rule> = vec![
        Rule {
            domains: vec![Any],
            params: UNIVERSAL_PARAMS.to_vec(),
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
        },
        Rule {
            domains: vec![Contains("instagram")],
            params: vec![
                Exact("igshid"),
            ],
        },
        Rule {
            domains: vec![Contains("nytimes")],
            params: vec![
                Exact("emc"),
                Exact("partner"),
            ],
        },
        Rule {
            domains: vec![Contains("reddit")],
            params: vec![
                Exact("context"),
                Exact("ref"),
                Exact("ref_source"),
                Exact("st"),
            ],
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
        },
        Rule {
            domains: vec![Contains("youtube")],
            params: vec![
                Contains("ab_channel"),
                Contains("attr_tag"),
                Contains("feature"),
                Contains("kw"),
            ],
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
    ];

}
