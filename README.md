# zip_to_unity
オンラインファイルサービスから落としてきたAssetのzipファイルをUnityにコピーするアプリ

## 使用方法
1. カレントディレクトリをプロジェクトフォルダにする
1. src/link.jsonのurlにGoogleドライブの共有リンクを記入
1. 以下のコマンドのいずれかでビルドする

デバッグ  
```
cargo tauri dev
```  
リリース  
```
cargo tauri build
```

## tauri
### フロントエンド
Rust(cargo)
### UI
Vanilla
