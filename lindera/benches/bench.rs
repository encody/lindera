#[cfg(any(
    feature = "ipadic",
    feature = "unidic",
    feature = "ko-dic",
    feature = "cc-cedict"
))]
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(any(
    feature = "ipadic",
    feature = "unidic",
    feature = "ko-dic",
    feature = "cc-cedict"
))]
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig, UserDictionaryConfig},
    DictionaryKind,
};

#[allow(unused_variables)]
fn bench_constructor(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        c.bench_function("bench-constructor-ipadic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::IPADIC),
                    path: None,
                };

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary: None,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "unidic")]
    {
        c.bench_function("bench-constructor-unidic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::UniDic),
                    path: None,
                };

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary: None,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "ko-dic")]
    {
        c.bench_function("bench-constructor-ko-dic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::KoDic),
                    path: None,
                };

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary: None,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "cc-cedict")]
    {
        c.bench_function("bench-constructor-cc-cedict", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::CcCedict),
                    path: None,
                };

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary: None,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }
}

#[allow(unused_variables)]
fn bench_constructor_with_simple_userdic(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        c.bench_function("bench-constructor-simple-userdic-ipadic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::IPADIC),
                    path: None,
                };

                let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("ipadic_simple_userdic.csv");

                let user_dictionary = Some(UserDictionaryConfig {
                    kind: Some(DictionaryKind::IPADIC),
                    path: userdic_file,
                });

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "unidic")]
    {
        c.bench_function("bench-constructor-simple-userdic-unidic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::UniDic),
                    path: None,
                };

                let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("unidic_simple_userdic.csv");

                let user_dictionary = Some(UserDictionaryConfig {
                    kind: Some(DictionaryKind::UniDic),
                    path: userdic_file,
                });

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "ko-dic")]
    {
        c.bench_function("bench-constructor-simple-userdic-ko-dic", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::KoDic),
                    path: None,
                };

                let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("ko-dic_simple_userdic.csv");

                let user_dictionary = Some(UserDictionaryConfig {
                    kind: Some(DictionaryKind::KoDic),
                    path: userdic_file,
                });

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }

    #[cfg(feature = "cc-cedict")]
    {
        c.bench_function("bench-constructor-simple-userdic-cc-cedict", |b| {
            b.iter(|| {
                let dictionary = DictionaryConfig {
                    kind: Some(DictionaryKind::CcCedict),
                    path: None,
                };

                let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("cc-cedict_simple_userdic.csv");

                let user_dictionary = Some(UserDictionaryConfig {
                    kind: Some(DictionaryKind::CcCedict),
                    path: userdic_file,
                });

                let config = TokenizerConfig {
                    dictionary,
                    user_dictionary,
                    mode: Mode::Normal,
                };
                Tokenizer::with_config(config).unwrap()
            })
        });
    }
}

#[allow(unused_variables)]
fn bench_tokenize(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();

        c.bench_function("bench-tokenize-ipadic", |b| {
            b.iter(|| tokenizer.tokenize("検索エンジン（けんさくエンジン、英語: search engine）は、狭義にはインターネットに存在する情報（ウェブページ、ウェブサイト、画像ファイル、ネットニュースなど）を検索する機能およびそのプログラム。"))
        });
    }

    #[cfg(feature = "unidic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();

        c.bench_function("bench-tokenize-unidic", |b| {
            b.iter(|| tokenizer.tokenize("検索エンジン（けんさくエンジン、英語: search engine）は、狭義にはインターネットに存在する情報（ウェブページ、ウェブサイト、画像ファイル、ネットニュースなど）を検索する機能およびそのプログラム。"))
        });
    }

    #[cfg(feature = "ko-dic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::KoDic),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();

        c.bench_function("bench-tokenize-ko-dic", |b| {
            b.iter(|| tokenizer.tokenize("검색엔진(search engine)은컴퓨터시스템에저장된정보를찾아주거나웹검색(web search query)을도와주도록설계된정보검색시스템또는컴퓨터프로그램이다. 이러한검색결과는목록으로표시되는것이보통이다."))
        });
    }

    #[cfg(feature = "cc-cedict")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::CcCedict),
            path: None,
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();

        c.bench_function("bench-tokenize-cc-cedict", |b| {
            b.iter(|| tokenizer.tokenize("搜索引擎（英語：search engine）是一种信息检索系统，旨在协助搜索存储在计算机系统中的信息。搜索结果一般被称为“hits”，通常会以表单的形式列出。网络搜索引擎是最常见、公开的一种搜索引擎，其功能为搜索万维网上储存的信息。"))
        });
    }
}

#[allow(unused_variables)]
fn bench_tokenize_with_simple_userdic(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("ipadic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();
        c.bench_function("bench-tokenize-with-simple-userdic-ipadic", |b| {
            b.iter(|| {
                tokenizer.tokenize("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")
            })
        });
    }

    #[cfg(feature = "unidic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("unidic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();
        c.bench_function("bench-tokenize-with-simple-userdic-unidic", |b| {
            b.iter(|| {
                tokenizer.tokenize("東京スカイツリーの最寄り駅はとうきょうスカイツリー駅です")
            })
        });
    }

    #[cfg(feature = "ko-dic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::KoDic),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("ko-dic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::KoDic),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();
        c.bench_function("bench-tokenize-with-simple-userdic-ko-dic", |b| {
            b.iter(|| tokenizer.tokenize("하네다공항한정토트백."))
        });
    }

    #[cfg(feature = "cc-cedict")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::CcCedict),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("cc-cedict_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::CcCedict),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };

        let tokenizer = Tokenizer::with_config(config).unwrap();
        c.bench_function("bench-tokenize-with-simple-userdic-cc-cedict", |b| {
            b.iter(|| tokenizer.tokenize("羽田机场限定托特包。"))
        });
    }
}

#[allow(unused_variables)]
fn bench_tokenize_long_text(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        let mut long_text_file = BufReader::new(
            File::open(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("bocchan.txt"),
            )
            .unwrap(),
        );
        let mut long_text = String::new();
        let _size = long_text_file.read_to_string(&mut long_text).unwrap();

        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("ipadic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // Using benchmark_group for changing sample_size
        let mut group = c.benchmark_group("tokenize-long-text-ipadic");
        group.sample_size(20);
        group.bench_function("bench-tokenize-long-text-ipadic", |b| {
            b.iter(|| tokenizer.tokenize(long_text.as_str()));
        });
        group.finish();
    }

    #[cfg(feature = "unidic")]
    {
        let mut long_text_file = BufReader::new(
            File::open(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("bocchan.txt"),
            )
            .unwrap(),
        );
        let mut long_text = String::new();
        let _size = long_text_file.read_to_string(&mut long_text).unwrap();

        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("unidic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // Using benchmark_group for changing sample_size
        let mut group = c.benchmark_group("tokenize-long-text-unidic");
        group.sample_size(20);
        group.bench_function("bench-tokenize-long-text-unidic", |b| {
            b.iter(|| tokenizer.tokenize(long_text.as_str()));
        });
        group.finish();
    }
}

#[allow(unused_variables)]
fn bench_tokenize_details_long_text(c: &mut Criterion) {
    #[cfg(feature = "ipadic")]
    {
        let mut long_text_file = BufReader::new(
            File::open(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("bocchan.txt"),
            )
            .unwrap(),
        );
        let mut long_text = String::new();
        let _size = long_text_file.read_to_string(&mut long_text).unwrap();

        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("ipadic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // Using benchmark_group for changing sample_size
        let mut group = c.benchmark_group("tokenize-details-long-text-ipadic");
        group.sample_size(20);
        group.bench_function("bench-tokenize-details-long-text-ipadic", |b| {
            b.iter(|| tokenizer.tokenize_with_details(long_text.as_str()));
        });
        group.finish();
    }

    #[cfg(feature = "unidic")]
    {
        let mut long_text_file = BufReader::new(
            File::open(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("../resources")
                    .join("bocchan.txt"),
            )
            .unwrap(),
        );
        let mut long_text = String::new();
        let _size = long_text_file.read_to_string(&mut long_text).unwrap();

        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: None,
        };

        let userdic_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../resources")
            .join("unidic_simple_userdic.csv");

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::UniDic),
            path: userdic_file,
        });

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // Using benchmark_group for changing sample_size
        let mut group = c.benchmark_group("tokenize-details-long-text-unidic");
        group.sample_size(20);
        group.bench_function("bench-tokenize-details-long-text-unidic", |b| {
            b.iter(|| tokenizer.tokenize_with_details(long_text.as_str()));
        });
        group.finish();
    }
}

criterion_group!(
    benches,
    bench_constructor,
    bench_constructor_with_simple_userdic,
    bench_tokenize,
    bench_tokenize_with_simple_userdic,
    bench_tokenize_long_text,
    bench_tokenize_details_long_text,
);
criterion_main!(benches);
