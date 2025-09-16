// Rustのバックエンドを呼びだす
const { invoke } = window.__TAURI__.core;

// ファイルダイアログを扱う関数
const { open, save } = window.__TAURI__.dialog;

/**
 * @breif zipファイルを開く
*/
function selectZipFile()
{
  open({
    filters: [
        { name: 'ZIP Archive', extensions: ['zip'] },
    ],
    multiple: false,
  }).then(async file => {
    // 要素にパスを格納
    let pathElement = document.getElementById('zip-filepath');
    pathElement.textContent = file;
  });
  
}

/**
 * @breif Unityプロジェクトを開く
*/
function selectUnityFolder()
{
  open({
    directory : true,
    multiple : false,
  }).then(async folder => {
    // 要素にパスを格納
    let folderElement = document.getElementById('unity-folderpath');
    folderElement.textContent = folder;
  });
}

/**
 * @breif zipからUnityプロジェクトへのコピーを実行
 */
async function executeCopy()
{
  // Unityを閉じたか確認
  let checkUnityClose = prompt("本当にUnityを閉じましたか? (Y/N)");
  if(checkUnityClose != "Y")
  {
    alert("Unityを閉じてください");
    return;
  }

  // パスを取得
  let zipFile = document.getElementById('zip-filepath').textContent;
  let unityFolder = document.getElementById('unity-folderpath').textContent; 

  // ボタンを処理中の表示にする
  let button = document.getElementById("copy-button");
  button.textContent = "実行中";
  button.disabled = true;

  // コピーを実行
  const result = await invoke("copy_zip_to_unity", {zipFile: zipFile, unityFolder: unityFolder});

  // 結果を出力
  let flag = result[0];
  let msg = result[1];
  alert(msg);

  // ボタンの表示を戻す
  button.textContent = "コピーを実行";
  button.disabled = false;
}

// htmlの要素とバインディング
window.selectZipFile = selectZipFile;
window.selectUnityFolder = selectUnityFolder;
window.executeCopy = executeCopy;