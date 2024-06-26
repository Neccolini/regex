## 正規表現エンジンを実装する


### メモ
今回実装すること

- 正規表現を構文解析してASTを生成
- ASTからNFAを構築
  - Thompsonの構築法
- NFAをDFAに変換
  - Subset Construction法
- DFAを最適化
  - on-the-fly構成法
  - Hopcroftの最適化法

今回文章に書くこと
- 正規表現とは 1ページ
- 正規表現の実装について 1ページ
- 数学的な説明 7ページ
- Thompsonの構築法 5ページ
- Subset Construction法 5ページ
- on-the-fly構成法 2ページ
- Hopcroftの最適化法 2ページ
- ReDoSについて 2ページ


参考にしたファイル
/home/naga/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-syntax-0.6.29/src/ast/parse.rs
/home/naga/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-syntax-0.6.29/src/ast/mod.rs
/home/naga/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.1.10/src/nfa/compiler.rs
/home/naga/.cargo/registry/src/index.crates.io-6f17d22bba15001f/regex-automata-0.1.10/src/nfa/mod.rs