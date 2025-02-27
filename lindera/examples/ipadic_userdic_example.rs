#[cfg(feature = "ipadic")]
use std::path::PathBuf;

use lindera::LinderaResult;
#[cfg(feature = "ipadic")]
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig, UserDictionaryConfig},
    DictionaryKind,
};

fn main() -> LinderaResult<()> {
    #[cfg(feature = "ipadic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../resources")
                .join("ipadic_simple_userdic.csv"),
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: user_dictionary,
            mode: Mode::Normal,
        };

        #[allow(unused_variables)]
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // tokenize the text
        let tokens =
            tokenizer.tokenize("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")?;

        // output the tokens
        for token in tokens {
            println!("{}", token.text);
        }
    }

    Ok(())
}
