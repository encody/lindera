#[cfg(feature = "ipadic")]
use std::path::PathBuf;

#[cfg(feature = "ipadic")]
use lindera::analyzer::Analyzer;
use lindera::LinderaResult;

fn main() -> LinderaResult<()> {
    #[cfg(feature = "ipadic")]
    {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("lindera_ipadic_conf.json");

        let analyzer = Analyzer::from_file(&path).unwrap();

        let mut text =
            "Ｌｉｎｄｅｒａは形態素解析ｴﾝｼﾞﾝです。ユーザー辞書も利用可能です。".to_string();
        println!("text: {}", text);

        // tokenize the text
        let tokens = analyzer.analyze(&mut text)?;

        // output the tokens
        for token in tokens {
            println!(
                "token: {}, start: {}, end: {}, details: {:?}",
                token.text, token.byte_start, token.byte_end, token.details
            );
        }
    }

    Ok(())
}
