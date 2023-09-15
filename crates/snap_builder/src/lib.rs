extern crate proc_macro;
use proc_macro::TokenStream;

include!(concat!(env!("OUT_DIR"), "/testfiles.rs"));

#[proc_macro]
pub fn build_snaps(_item: TokenStream) -> TokenStream {
    TEST_FILES
        .iter()
        .map(|f| format!("snap!({});", f))
        .collect::<Vec<String>>()
        .join("\n")
        .parse()
        .unwrap()
}
