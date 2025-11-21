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
 * @param unity_folder unityプロジェクトのフォルダ
 * @return 解凍しなかったファイルのリスト
 */
#[tauri::command]
fn extract_zip(
    zip_file: &str,
    dest_folder: &str,
    unity_folder: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // パスを作成
    let dest_path = Path::new(dest_folder);
    let unity_path = Path::new(unity_folder);

    // zipファイルを読込
    let file = fs::File::open(zip_file)?;
    // zipArchiveに変換
    let mut archive = ZipArchive::new(file)?;

    // 解凍しなかったファイルのリスト
    let mut uncopy_files = Vec::new();

    // アーカイブ内の各ファイルをループで処理
    for file_index in 0..archive.len() {
        // アーカイブから要素を取り出す
        let mut file = archive.by_index(file_index)?;

        // ファイル名を安全に取得
        let filename = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        // 解凍先/ファイルパスのパスを取得
        let outpath = dest_path.join(&filename);

        if file.name().ends_with('/') {
            // フォルダなら作成
            fs::create_dir_all(&outpath)?;
        } else {
            // パスがファイルの場合

            // ファイルの拡張子を取得
            let file_extension = match outpath.extension() {
                Some(ext) => ext,
                None => continue,
            };

            // メタファイルはコピーしない
            if file_extension == "meta" {
                continue;
            }

            // メタファイルが存在しないなら除外
            let meta_str = format!("{}.meta", unity_path.join(&filename).display());
            let meta_path = Path::new(&meta_str);
            if !meta_path.exists() {
                // コピーしなかったファイルを記録
                uncopy_files.push(filename.display().to_string());
                continue;
            }

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

    return Ok(uncopy_files);
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

fn join_uncopy_files(src: &str, uncopy_files: &Vec<String>) -> String {
    let mut text = String::from(src);
    if uncopy_files.is_empty() {
        return text;
    }

    text.push_str("\n");
    text.push_str("====================================\n");
    text.push_str("metaファイルがないため以下のファイルは除外されました\n");
    for file in uncopy_files {
        text.push_str(file);
        text.push('\n');
    }

    text
}

/**
 * @breif zipを解凍してunityフォルダに展開
 * @param zip_file zipファイルの絶対パス
 * @param unity_foler unityフォルダの絶対パス
 * @return (trueなら成功, エラーメッセージ)
 */
#[tauri::command]
fn copy_zip_to_unity(zip_file: &str, unity_folder: &str) -> (bool, String) {
    // Downloads/tempフォルダ
    let temp_folder_buf = match dirs::download_dir() {
        Some(download_path) => download_path.join("zip_to_unity_temp"),
        None => {
            return (false, String::from("ダウンロードフォルダの取得に失敗"));
        }
    };

    // tempフォルダを文字列に変換
    let temp_folder = match temp_folder_buf.to_str() {
        Some(path_str) => path_str,
        None => {
            return (false, String::from("tempフォルダのパス変換に失敗"));
        }
    };

    let assets_folder_name = "Assets"; // Assetsフォルダ

    // 作業フォルダをリセット
    match reset_folder(&temp_folder) {
        Ok(()) => {}
        Err(error) => {
            return (
                false,
                format!("{}{}{}", temp_folder, "のリセット失敗\n", error.to_string()),
            );
        }
    }

    // コピーしなかったファイル

    // zipファイルを展開
    let uncopy_files = match extract_zip(zip_file, temp_folder, unity_folder) {
        Ok(files) => files,
        Err(error) => {
            return (
                false,
                format!("{}{}{}", zip_file, "解凍エラー\n", error.to_string()),
            );
        }
    };

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

    // tempフォルダを削除
    match fs::remove_dir_all(temp_folder_path) {
        Ok(()) => {}
        Err(_) => {
            return (false, String::from("tempフォルダの削除に失敗"));
        }
    }

    return (
        true,
        join_uncopy_files("正常に処理されました\n", &uncopy_files),
    );
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
