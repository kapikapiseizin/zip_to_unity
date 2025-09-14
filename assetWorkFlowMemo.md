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
<iframe
  src="https://drive.google.com/viewer?srcid=1mzjr4GUfMkLGBj0vXrZ1myY_whagT-bd&pid=explorer&efh=false&a=v&chrome=false&embedded=true" 
  style="width:600px; height:500px;" 
  frameborder="0"></iframe>

## プログラマーの運用方法
1. 管理者がgitignoreで特定フォルダ以下でmetaのみコミットされるようにします。
- Assets/Model
- Assets/Texture
- Assets/Sound
- その他
2. 配布されたアプリを開く  
※使用できますがデザイン何もしてないプロトタイプの写真です
<iframe
  src="https://drive.google.com/viewer?srcid=1XTP0EcNUBs0F8Kv9tnVFgQtiY0OtmAh7&pid=explorer&efh=false&a=v&chrome=false&embedded=true" 
  style="width:600px; height:500px;" 
  frameborder="0"></iframe>

3. アプリの操作に従う
1. Unityプロジェクトに**最新のアセットがコピー**されます。
1. まだプロジェクトにインポートされていない  
**Assets/Model/model.fbx**をインポートします。
1. 作業します。
1. コミット時にアセットのmetaのみ上がっていることを確認します。
1. 他のメンバーはmetaファイルがリモートリポジトリに上がるため  
**アプリの操作以外にインポートは不要**です。