# Misskey EmojiBot-R

Misskey の絵文字とデコレーションの更新を通知する Bot です。API として、絵文字一覧取得ではなくモデレーションログ取得を利用しているのが特徴です。<br>

Python で作成した Bot を Rust で書き直したものであり、整理した部分はありますが挙動は概ね同じです。<br>
https://github.com/9min-packup/misskey-emojibot/tree/main<br><br>


Misskey 2024.3.1 での動作を確認していますが、API の変更が起きることがあり、それ以外のバージョンでの動作の保証はできません。<br><br>

この Bot は一般ユーザーが作成するものではなく、サーバー管理者が作成してモデレーションに活用することを想定しています。

-   Bot アカウントに管理者権限を付与する必要があります。
-   アクセストークンに適切な権限を与える必要があります。<br><br>

アクセストークンに必要な権限は以下の通りです。

```
・アカウントの情報を見る
・ノートを作成・削除する
・モデレーションログを見る
```
## ビルド

cargo と rustc のバージョンは以下の通りです。
```bash
$ cargo --version
cargo 1.77.2 (e52e36006 2024-03-26)
$ rustc --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

リポジトリを Clone します。
```bash
$ git clone https://github.com/9min-packup/misskey-emojibot-r.git
```
ディレクトリに移動します。
```bash
$ cd misskey-emojibot-r
```
config ディレクトリを作成します。
```bash
$ mkdir config
```

以下のファイルを `.example` ディレクトリからコピーします。
```bash
$ cp .example/.env ./
$ cp .example/emojibot.toml config/
```
`.env` ファイルを vi などで開き、 `CONFIG_FILE_PATH` 環境変数に `emojibot.toml` へのパスを記載します。
```bash
# path to config file
# like this : CONFIG_FILE_PATH=config/emojibot.toml
CONFIG_FILE_PATH=config/emojibot.toml

```
`RUST_LOG` には出力するログのレベルを指定します。 特にこだわりがなければ `info` のままで Ok です。 
<br><br>
cargo でビルドします。
```bash
$ cargo build --release
```

エラーが出なければビルド完了です。<br><br>

## 使い方


`emojibot.toml` に設定内容を記載します。 `emojibot.toml` の例は以下の通りです。

```toml
[bot]
# host : your server url - like this, "https://example.tld"
host = "https://example.tld"
# token : your bot's access token
token = "XXXXXXXXXXXXXXXX"
# moderation_logs_limit : number of moderation logs that be got at one time (max : 100)
moderation_logs_limit = 5
# running_interval_seconds : running interval (seconds)
running_interval_seconds = 60

[notify]
# visibility : visibility of notes
# any of "public" "home" "followers" "specified"
# "specified" means direct note
visibility.add ="public"
visibility.update = "home"
visibility.delete = "home"

# visible_usernames : when visibility is "specified", emojibot sends a direct note to them
visible_usernames = []

# use_cw : either false or true
use_cw.add = false
use_cw.update = false
use_cw.delete = false

# local_only : either false or true
local_only = false

# reaction_acceptance : reaction acceptance setting
# any of "all" "likeOnly" "likeOnlyForRemote" "nonSensitiveOnly" "nonSensitiveOnlyForLocalLikeOnlyForRemote"
reaction_acceptance = "all"

# use_mention : either false or true
# when it is true, emojibot notify who added custom emoji
use_mention = true

# if you want to customize messages, change following
[messages]
emoji_add = "新しい絵文字が追加されました。"
emoji_add_user = "追加したユーザー"
emoji_update = "絵文字が更新されました。"
emoji_update_user = "更新したユーザー"
emoji_delete = "絵文字が削除されました。"
emoji_delete_user = "削除したユーザー"
decoration_add = "新しいアバターデコレーションが追加されました。"
decoration_add_user = "追加したユーザー"
decoration_update = "アバターデコレーションが更新されました。"
decoration_update_user = "更新したユーザー"
decoration_delete = "アバターデコレーションが削除されました。"
decoration_delete_user = "削除したユーザー"
```

-   `host` には Misskey サーバーの URL を記載してください。
-   `token` には取得したアクセストークンを記載してください。
-   `moderation_logs_limit` は一度に取得するモデレーションログの数で、最大値は 100 です。適宜調節してください。（数値を増やしたとしても、取得するのは絵文字 Bot 起動後に作成されたログのみです）
-   `running_interval_seconds` は Bot が動作する時間間隔（秒）です。お好みで調節してください。
-   `visibility` は絵文字またはデコレーションが更新されたときのノートの公開範囲です。`public`, `home`, `followers`,`specified` を指定できます。(`specified` はダイレクトのことです。)`add` は追加時の公開範囲、`update` は更新時の公開範囲、`delete` は削除時の公開範囲です。
-   `visible_usernames` には公開範囲がダイレクトのときの宛先を文字列配列で記述してください。@ を除いて記述する必要があります。ダイレクトを使用しない場合は空の配列でも構いません。存在しないアカウント名を記載するとエラーになります。
-   `use_cw` は絵文字またはデコレーションが更新されたときのノートに CW をかけるかの設定です。`true` か `false` かを指定できます。`add` は追加時、`update` は追加時、`delete` は削除時の指定です。
-   `local_only` は絵文字またはデコレーションが更新されたときのノートの連合をオフにするかどうかの設定です。`true` か `false` かを設定でき、`true` にすると連合オフになります。
-   `reaction_acceptance` は Bot のノートにつけられるリアクションの種類の指定です。`all`（全て許可）, `likeOnly`（いいねのみ）, `likeOnlyForRemote`（リモートからはいいねのみ）, `nonSensitiveOnly`（非センシティブのみ）, `nonSensitiveOnlyForLocalLikeOnlyForRemote`（リモートからは非センシティブのみ）のいずれかを指定できます。
-   `use_mention` は絵文字またはデコレーションを操作したユーザーにメンションするかの設定です。 `true` にするとメンションし、 `false` にするとメンションされません。
-   `messages` は各種メッセージの設定です。お好みで変更してください。

<br>
設定が完了したら、以下のコマンドで Bot を実行します。運が良ければ動きます。

```
$ ./target/release/misskey-emojibot-r 
```

<br>
動作確認ができたら、あとは Linux サーバーなどでサービスに登録して動作させてください。<br><br>

## 雑記

管理者権限を与えた Bot ですので、改造すれば各種モデレーションに活用できそうです。ユーザーからの通報を通知したり、絵文字を申請する機能を追加したり。通知先は discord などの外部から見えない場所にしたほうがいいかもしれませんね。