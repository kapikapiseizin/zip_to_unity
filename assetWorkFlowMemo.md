# アセットワークフロー

## 概要
デザイナーがアセット(3Dモデル、テクスチャ)を制作し、  
Unityプロジェクトに導入するまでの手順についての提案です。

## メリット
- Assetの容量が増えてもGitの**無料枠を使い切らない**
- デザイナーがアセットを更新しても**プログラマーの負担が増えない**

## 方針
リモートリポジトリにはアセットの**metaファイルのみをコミット**します。  
これはgitignoreファイルにて行います。  
実際のアセットファイルは**Googleドライブ**で集中管理します。  
ドライブからUnityプロジェクトへの導入は半自動で行われます。  

## 制作環境
Unityプロジェクトのフォルダ構成
```md
/Assets
└/Model
    ├model.fbx
    └model.fbx.meta
```

リモートリポジトリ上のフォルダ構成
```md
/Assets
└/Model
    <!-- metaファイルのみ -->
    └model.fbx.meta
```

Googleドライブ上のフォルダ構成
```md
/Assets
└/Model
    <!-- fbxファイルのみ -->
    └model.fbx
```

## デザイナーの運用方法
1. プランナーから「**Assets/Model/model.fbx**を納品して」とタスクを受ける
1. モデルを作成する
1. Googleドライブの**Assets/Model/model.fbx**に納品する
<img width="1162" height="626" alt="スクリーンショット 2025-09-14 160500" src="https://github.com/user-attachments/assets/c86ddc11-2d96-4bd5-844b-bd1db33dc73d" />

## プログラマーの運用方法
1. 管理者がgitignoreで特定フォルダ以下で.metaファイルのみコミットされるようにします。
- Assets/Model
- Assets/Texture
- Assets/Sound
- その他
2. 配布されたアプリを開く  
※使用できますがデザイン何もしてないプロトタイプの写真です
<img width="1054" height="962" alt="スクリーンショット 2025-09-14 161404" src="https://github.com/user-attachments/assets/42d8edb3-fb8d-4fe8-b0ff-9add809b316c" />

3. アプリの操作に従う
1. Unityプロジェクトに**最新のアセットがコピー**されます。
1. まだプロジェクトにインポートされていない  
**Assets/Model/model.fbx**をインポートします。
1. 作業します。
1. コミット時にアセットのmetaのみ上がっていることを確認します。
1. 他のメンバーは.metaファイルがリモートリポジトリに上がるため  
**アプリの操作以外にインポートは不要**です。

