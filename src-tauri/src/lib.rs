/*
 * Copyright 2025. NayamiOR
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */


use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
// use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
mod files;
mod utils;

struct AppData {
    target_dir_path: Option<String>,
    current_dir_path: Option<String>,
    file_list: Vec<PathBuf>,
    chosen_files: Vec<SearchResultEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SearchResultEntry {
    file_path: String,
    file_name: String,
    keyword: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                target_dir_path: None,
                current_dir_path: None,
                file_list: vec![],
                chosen_files: vec![],
            }));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_target_folder,
            set_target_folder,
            set_source_folder,
            search,
            execute,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn open_target_folder(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    let path = match state.target_dir_path.take() {
        Some(path) => path,
        None => {
            return Err("没有设置目标文件夹".to_string());
        }
    };

    files::open_folder(path).map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn set_target_folder(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    target_path: String,
) -> Result<(), String> {
    let mut state = state.lock().await;
    if !files::check_valid_path(&target_path) {
        return Err("目标文件夹不存在".to_string());
    }
    state.target_dir_path = Some(target_path);

    Ok(())
}

#[tauri::command]
async fn set_source_folder(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    source_path: String,
    recursive: String,
) -> Result<(), String> {
    let mut state = state.lock().await;
    if !files::check_valid_path(&source_path) {
        return Err("源文件夹不存在".to_string());
    }
    state.current_dir_path = Some(source_path);

    let recursive = match recursive.as_str() {
        "unrecursive" => false,
        "recursive" => true,
        _ => unreachable!(),
    };

    // Collect files
    let filenames = match files::load_files(&state.current_dir_path.as_ref().unwrap(), recursive) {
        Ok(filenames) => filenames,
        Err(err) => {
            return Err(format!("加载文件失败: {}", err));
        }
    };

    state.file_list = filenames;

    // NOTE: 不知道为什么这里发不出去，浏览器控制台报错
    // UPDATE: 后来突然好了，可能是临时的问题，代码没错
    // Report and show the result
    app.dialog()
        .message(format!(
            "设置成功，内部文件数量为：{}",
            state.file_list.iter().count()
        ))
        .kind(MessageDialogKind::Info)
        .title("设置源文件夹成功")
        .buttons(MessageDialogButtons::OkCustom("确定".to_string()))
        .blocking_show();

    Ok(())
}

#[tauri::command]
async fn search(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    method: String,
    keywords: Vec<String>,
) -> Result<Vec<SearchResultEntry>, String> {
    let mut state = state.lock().await;
    update_progress(app.clone(), 0.0);
    let file_list = &state.file_list;
    let mut counter: Arc<std::sync::Mutex<i32>> = Arc::new(std::sync::Mutex::new(0));
    let file_filter = |x: &PathBuf| {
        let filepath = x.to_str().unwrap().to_string();
        let filename = x.file_name().unwrap().to_str().unwrap().to_string();
        let filestem = x.file_stem().unwrap().to_str().unwrap().to_string();
        let result: Option<SearchResultEntry> = match method.as_str() {
            "same" => {
                if keywords.contains(&filename) {
                    Some(SearchResultEntry {
                        file_path: filepath,
                        file_name: filename.clone(),
                        keyword: filename,
                    })
                } else {
                    None
                }
            }
            "same-name" => {
                let result: Vec<&String> = keywords
                    .iter()
                    .filter(|x: &&String| if filestem == **x { true } else { false })
                    .collect::<Vec<&String>>();
                if result.is_empty() {
                    None
                } else {
                    Some(SearchResultEntry {
                        file_path: filepath,
                        file_name: filename,
                        keyword: result[0].to_string(),
                    })
                }
            }
            "include-name" => {
                let result: Vec<&String> = keywords
                    .iter()
                    .filter(
                        |x: &&String| {
                            if filename.contains(*x) {
                                true
                            } else {
                                false
                            }
                        },
                    )
                    .collect::<Vec<&String>>();
                if result.is_empty() {
                    None
                } else {
                    Some(SearchResultEntry {
                        file_path: filepath,
                        file_name: filename,
                        keyword: result[0].to_string(),
                    })
                }
            }
            _ => unreachable!(),
        };
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        update_progress(app.clone(), *counter as f32 / file_list.len() as f32);

        result
    };

    let result: Vec<SearchResultEntry> = file_list.iter().filter_map(file_filter).collect();
    state.chosen_files = result.clone();

    // 显示完成提示
    app.dialog()
        .message(format!("操作完成，找到 {} 个文件", result.len()))
        .kind(MessageDialogKind::Info)
        .title("完成")
        .buttons(MessageDialogButtons::OkCustom("确定".to_string()))
        .blocking_show();
    Ok(result)
}

#[tauri::command]
async fn execute(
    app: AppHandle,
    state: State<'_, Mutex<AppData>>,
    method: String,
) -> Result<(), String> {
    let state = state.lock().await;
    let target_dir_path = match state.target_dir_path.clone() {
        Some(path) => path,
        None => {
            return Err("没有设置目标文件夹".to_string());
        }
    };
    let chosen_files = state.chosen_files.clone();
    let total_files = chosen_files.len() as f32;
    let counter = Arc::new(std::sync::Mutex::new(0));

    chosen_files.iter().try_for_each(|x| {
        let file_path = std::path::PathBuf::from(&x.file_path);
        let file_name = x.file_name.clone();
        let target_path = std::path::Path::new(&target_dir_path).join(&file_name);

        match method.as_str() {
            "copy" => {
                std::fs::copy(&file_path, &target_path)
                    .map_err(|e| format!("Failed to copy {}: {}", file_path.display(), e))?;
            }
            "move" => {
                std::fs::rename(&file_path, &target_path)
                    .map_err(|e| format!("Failed to move {}: {}", file_path.display(), e))?;
            }
            "delete" => {
                if file_path.is_dir() {
                    std::fs::remove_dir_all(&file_path).map_err(|e| {
                        format!("Failed to delete dir {}: {}", file_path.display(), e)
                    })?;
                } else {
                    std::fs::remove_file(&file_path).map_err(|e| {
                        format!("Failed to delete file {}: {}", file_path.display(), e)
                    })?;
                }
            }
            _ => unreachable!(),
        }

        // 更新进度
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        update_progress(app.clone(), *counter as f32 / total_files);

        Ok::<(), String>(())
    })?;

    // 显示完成提示
    app.dialog()
        .message("操作完成")
        .kind(MessageDialogKind::Info)
        .title("完成")
        .buttons(MessageDialogButtons::OkCustom("确定".to_string()))
        .blocking_show();

    Ok(())
}

fn update_progress(app: AppHandle, percentage: f32) {
    app.emit("update-progress", (percentage * 100.0) as i32)
        .unwrap();
}
