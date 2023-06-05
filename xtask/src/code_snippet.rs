use serde::Deserialize;
use serde_flat_path::flat_path;

#[flat_path]
#[derive(Deserialize)]
struct CodeSnippetResponse {
    #[flat_path("data.question.codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
}
#[derive(Deserialize)]
struct CodeSnippet {
    lang: String,
    code: String,
}

pub fn get_code_snippet(title_slug: &str) -> String {
    let code_snippets_res = ureq::get("https://leetcode.com/graphql/")
        .send_json(ureq::json!({
            "query": r#"query questionEditorData($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        codeSnippets {
                            lang
                            code
                        }
                    }
                }"#,
            "variables":{"titleSlug": title_slug},
            "operationName":"questionEditorData"
        }))
        .unwrap()
        .into_json::<CodeSnippetResponse>()
        .unwrap();
    code_snippets_res
        .code_snippets
        .into_iter()
        .find_map(|cs| (cs.lang == "Rust").then(|| cs.code))
        .unwrap()
}
