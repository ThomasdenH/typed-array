use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[wasm_bindgen_test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
