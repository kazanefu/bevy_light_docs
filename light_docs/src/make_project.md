# プロジェクトの作成

先ほどのサンプルゲームのようなものを実際に0から作っていきます！<br>
まずは、プロジェクトを作成するために、ターミナルで`BevyWorks`ディレクトリ(ディレクトリ名は人それぞれなので読み替えてください)にいる状態で以下のコマンドを実行します。
```bash
cargo new game
```

これを実行すると、`BevyWorks`ディレクトリの中に`game`という名前のディレクトリが作成され、その中にRustのプロジェクトの雛形が作成されます。<br>

次に、作成したプロジェクトをVSCodeで開きます。<br>

```bash
code ./game
```

開くと以下のようなディレクトリ構造になっていると思います。<br>

```
game
├── Cargo.lock
├── Cargo.toml
├── target/
└── src/
    └── main.rs
```
それぞれのフォルダの解説を軽くすると、`Cargo.lock`は依存関係のバージョンを固定するためのファイル(人間が直接いじることはほとんどない)、`Cargo.toml`はプロジェクトの設定や依存関係を記述するファイル、`target/`はビルドした際に生成されるファイルが入るディレクトリ、`src/`はソースコードを置くディレクトリで、その中の`main.rs`がエントリーポイントとなるRustのソースコードです。<br>
あれ？呪文に聞こえるって？大丈夫です、慣れればすぐに理解できるようになります。<br>

このままでもRustのプロジェクトとしてはビルド&実行できますが、Bevyを使うために`Cargo.toml`に依存関係を追加していきます。<br>
`Cargo.toml`を開き、以下のように書き換えます。
```toml
[package]
name = "game"
version = "0.1.0"
edition = "2024"

[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 3
[features]
dynamic_linking = ["bevy/dynamic_linking"]

[dependencies]
bevy = "0.18.1"
bevy_rapier2d = "0.33"
serde = { version = "1.0", features = ["derive"] }
toml = "1.1"
anyhow = "1.0"
```
Bevyを使えるようにするために、`[dependencies]`のところに`bevy = "0.18.1"`を追加しました。<br>
また、`bevy_rapier2d`は物理演算を行うためのライブラリで、`serde`はデータのシリアライズ/デシリアライズを行うためのライブラリ、`toml`はTOML形式のデータを扱うためのライブラリ、`anyhow`はエラー処理を簡単にするためのライブラリです。<br>

> この辺はとりあえずそうしておけばいいと思ってれば[profile.dev]のopt-levelはビルドの最適化レベルを指定するもので、デフォルトでは1ですが、リリースビルドでは3に設定しています。<br>
[profile.dev.package."*"]のopt-levelは、すべての依存関係に対して最適化レベルを3に設定するためのものです。<br>
[features]のdynamic_linkingはBevyの動的リンクを有効にするための設定です。<br>

次に、`.cargo/config.toml`を作成し、以下のように書きます。
```toml
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
```

これで、プロジェクトの作成とBevyの依存関係の追加が完了しました。<br>
次のステップでは、実際にゲームを作るためのコードを書いていきます。
