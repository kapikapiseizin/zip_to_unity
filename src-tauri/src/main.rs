// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fs_extra::dir::CopyOptions;
use std::fs;
use std::io;
use std::path::Path;
use webbrowser;
use zip::ZipArchive;

/**
 * @breif フォルダを空にする
 * @param path フォルダのパス
 */
fn reset_folder(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // フォルダを再帰的に削除
    if fs::metadata(path).is_ok() {
        fs::remove_dir_all(path)?;
    }

    // フォルダを作成
    fs::create_dir(path)?;

    return Ok(());
}

/**
 * @param zip_file zipファイルのパス
 * @param dest_folder zipファイルの解凍先フォルダ
 */
#[tauri::command]
fn extract_zip(zip_file: &str, dest_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    // パスを作成
    let dest_path = Path::new(dest_folder);

    // zipファイルを読込
    let file = fs::File::open(zip_file)?;
    // zipArchiveに変換
    let mut archive = ZipArchive::new(file)?;

    // アーカイブ内の各ファイルをループで処理
    for file_index in 0..archive.len() {
        // アーカイブから要素を取り出す
        let mut file = archive.by_index(file_index)?;

        // 解凍先/ファイルパスのパスを安全に取得
        let outpath = match file.enclosed_name() {
            Some(path) => dest_path.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            // フォルダなら作成
            fs::create_dir_all(&outpath)?;
        } else {
            // パスがファイルの場合

            // 親ディレクトリを取得
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    // 親ディレクトリが存在しないなら作成
                    fs::create_dir_all(&p)?;
                }
            }

            // ファイルをコピー
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    return Ok(());
}

/**
 * @breif パスの子要素に特定のフォルダがあるか判定する
 * @param path フォルダパス
 * @param search_name 存在するか探したいフォルダの名前
 * @return trueなら存在する
 *
 */
fn is_exists_folder(path: &str, search_name: &str) -> bool {
    // パス型に変換
    let path_copy = Path::new(path);

    // seach_nameをパス結合
    let join_path = path_copy.join(search_name);

    // パスがフォルダとして存在するか
    return join_path.is_dir();
}

/**
 * @breif zipを解凍してunityフォルダに展開
 * @param zip_file zipファイルの絶対パス
 * @param unity_foler unityフォルダの絶対パス
 * @return (trueなら成功, エラーメッセージ)
 */
#[tauri::command]
fn copy_zip_to_unity(zip_file: &str, unity_folder: &str) -> (bool, String) {
    let temp_folder = "./temp"; // 作業用フォルダ
    let assets_folder_name = "Assets"; // Assetsフォルダ

    // 作業フォルダをリセット
    match reset_folder(temp_folder) {
        Ok(()) => {}
        Err(error) => {
            return (
                false,
                format!("{}{}{}", temp_folder, "のリセット失敗\n", error.to_string()),
            );
        }
    }

    // zipファイルを展開
    match extract_zip(zip_file, temp_folder) {
        Ok(()) => {}
        Err(error) => {
            return (
                false,
                format!("{}{}{}", zip_file, "解凍エラー\n", error.to_string()),
            );
        }
    }

    // zipファイルの直下にAssetsがあるか
    if !is_exists_folder(temp_folder, assets_folder_name) {
        return (false, String::from("zipファイルにAssetsがありません"));
    }

    // Unityファイルの直下にAssetsがあるか
    if !is_exists_folder(unity_folder, assets_folder_name) {
        return (false, String::from("UnityフォルダにAssetsがありません"));
    }

    // temp/Assetsのパス作成
    let temp_folder_path = Path::new(temp_folder);
    let temp_assets_folder = temp_folder_path.join(assets_folder_name);
    if !temp_assets_folder.is_dir() {
        return (
            false,
            String::from("zipからコピーしたtemp/Assetsフォルダがありません"),
        );
    }

    // 展開したzipの中身をUnityプロジェクトフォルダにコピー
    // 上書きを許可
    let copy_option = CopyOptions::new().overwrite(true);
    match fs_extra::dir::copy(temp_assets_folder, unity_folder, &copy_option) {
        Ok(_) => {}
        Err(_) => {
            return (false, String::from("zipからUnityフォルダへのコピー失敗"));
        }
    }

    return (true, String::from("正常に終了しました"));
}

/**
 * @breif URLをデフォルトブラウザで開く
 * @param url 開きたいURL
 */
#[tauri::command]
fn open_url(url: &str) -> bool {
    if webbrowser::open(url).is_ok() {
        return true;
    }
    return false;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![copy_zip_to_unity, open_url])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("runtime error");
}
