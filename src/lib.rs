use aviutl2::{anyhow, tracing};

static GLOBAL_EDIT_HANDLE: aviutl2::generic::GlobalEditHandle =
    aviutl2::generic::GlobalEditHandle::new();

#[aviutl2::plugin(GenericPlugin)]
struct OpenProjectDirectoryAux2;

impl aviutl2::generic::GenericPlugin for OpenProjectDirectoryAux2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(if cfg!(debug_assertions) {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            })
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();
        Ok(Self)
    }

    fn plugin_info(&self) -> aviutl2::generic::GenericPluginTable {
        aviutl2::generic::GenericPluginTable {
            name: "open_project_directory.aux2".to_string(),
            information: format!(
                "Open Project Directory / v{} / https://github.com/sevenc-nanashi/open_project_directory.aux2",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {
        GLOBAL_EDIT_HANDLE.init(registry.create_edit_handle());
        registry.register_menus::<Self>();
    }
}

#[aviutl2::generic::menus]
impl OpenProjectDirectoryAux2 {
    #[edit(name = "open_project_directory.aux2\\プロジェクトファイルをエクスプローラーで表示")]
    fn open_project_directory(&mut self) -> aviutl2::AnyResult<()> {
        let project_path = GLOBAL_EDIT_HANDLE
            .call_edit_section(|edit_section| {
                let project_file = edit_section.get_project_file(&GLOBAL_EDIT_HANDLE);
                project_file.get_path()
            })
            .map_err(|e| anyhow::anyhow!("編集中プロジェクト情報の取得に失敗しました: {e}"))?
            .ok_or_else(|| anyhow::anyhow!("プロジェクトファイルを先に保存してください"))?;

        show_path_in_explorer(&project_path).map_err(|e| {
            anyhow::anyhow!(
                "プロジェクトファイルのフォルダをエクスプローラーで開くことができませんでした ({}): {}",
                project_path.display(),
                e
            )
        })?;

        Ok(())
    }
    #[edit(name = "open_project_directory.aux2\\プロジェクトファイルのフォルダのパスをコピー")]
    fn copy_project_directory(&mut self) -> aviutl2::AnyResult<()> {
        let project_path = GLOBAL_EDIT_HANDLE
            .call_edit_section(|edit_section| {
                let project_file = edit_section.get_project_file(&GLOBAL_EDIT_HANDLE);
                project_file.get_path()
            })
            .map_err(|e| anyhow::anyhow!("編集中プロジェクト情報の取得に失敗しました: {e}"))?
            .ok_or_else(|| anyhow::anyhow!("プロジェクトファイルを先に保存してください"))?;

        let project_dir = project_path.parent().ok_or_else(|| {
            anyhow::anyhow!("プロジェクトファイルの親フォルダを取得できませんでした")
        })?;

        arboard::Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(project_dir.to_string_lossy().to_string()))
            .map_err(|e| {
                anyhow::anyhow!(
                    "クリップボードへのコピーに失敗しました ({}): {}",
                    project_dir.display(),
                    e
                )
            })?;
        tracing::info!(
            "Project directory path copied to clipboard: {}",
            project_dir.display()
        );
        Ok(())
    }

    #[edit(name = "open_project_directory.aux2\\プロジェクトファイルのパスをコピー")]
    fn copy_project_file_path(&mut self) -> aviutl2::AnyResult<()> {
        let project_path = GLOBAL_EDIT_HANDLE
            .call_edit_section(|edit_section| {
                let project_file = edit_section.get_project_file(&GLOBAL_EDIT_HANDLE);
                project_file.get_path()
            })
            .map_err(|e| anyhow::anyhow!("編集中プロジェクト情報の取得に失敗しました: {e}"))?
            .ok_or_else(|| anyhow::anyhow!("プロジェクトファイルを先に保存してください"))?;

        arboard::Clipboard::new()
            .and_then(|mut clipboard| {
                clipboard.set_text(project_path.to_string_lossy().to_string())
            })
            .map_err(|e| {
                anyhow::anyhow!(
                    "クリップボードへのコピーに失敗しました ({}): {}",
                    project_path.display(),
                    e
                )
            })?;
        tracing::info!(
            "Project file path copied to clipboard: {}",
            project_path.display()
        );
        Ok(())
    }

    #[object_item(name = "open_project_directory.aux2\\ファイルのフォルダを開く")]
    fn open_file_directory(
        &mut self,
        object: aviutl2::generic::ObjectHandle,
        effect: &str,
        index: usize,
        item: &str,
    ) -> aviutl2::AnyResult<()> {
        let path = GLOBAL_EDIT_HANDLE
            .call_read_section(|read| read.get_object_effect_item(object, effect, index, item))??;
        let path: std::path::PathBuf = path.into();
        show_path_in_explorer(&path).map_err(|e| {
            anyhow::anyhow!(
                "オブジェクトのファイルのフォルダをエクスプローーで開くことができませんでした ({}): {}",
                path.display(),
                e
            )
        })?;

        Ok(())
    }

    #[object_item(name = "open_project_directory.aux2\\ファイルのフォルダのパスをコピー")]
    fn copy_file_directory(
        &mut self,
        object: aviutl2::generic::ObjectHandle,
        effect: &str,
        index: usize,
        item: &str,
    ) -> aviutl2::AnyResult<()> {
        let path = GLOBAL_EDIT_HANDLE
            .call_read_section(|read| read.get_object_effect_item(object, effect, index, item))??;
        let path: std::path::PathBuf = path.into();
        let dir = path.parent().ok_or_else(|| {
            anyhow::anyhow!(
                "オブジェクトのファイルの親フォルダを取得できませんでした ({}): {}",
                path.display(),
                path.display()
            )
        })?;
        arboard::Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(dir.to_string_lossy().to_string()))
            .map_err(|e| {
                anyhow::anyhow!(
                    "クリップボードへのコピーに失敗しました ({}): {}",
                    dir.display(),
                    e
                )
            })?;
        tracing::info!("File directory path copied to clipboard: {}", dir.display());
        Ok(())
    }

    #[object_item(name = "open_project_directory.aux2\\ファイルのパスをコピー")]
    fn copy_file_path(
        &mut self,
        object: aviutl2::generic::ObjectHandle,
        effect: &str,
        index: usize,
        item: &str,
    ) -> aviutl2::AnyResult<()> {
        let path = GLOBAL_EDIT_HANDLE
            .call_read_section(|read| read.get_object_effect_item(object, effect, index, item))??;
        let path: std::path::PathBuf = path.into();
        arboard::Clipboard::new()
            .and_then(|mut clipboard| clipboard.set_text(path.to_string_lossy().to_string()))
            .map_err(|e| {
                anyhow::anyhow!(
                    "クリップボードへのコピーに失敗しました ({}): {}",
                    path.display(),
                    e
                )
            })?;
        tracing::info!("File path copied to clipboard: {}", path.display());
        Ok(())
    }
}

fn show_path_in_explorer(path: &std::path::Path) -> aviutl2::AnyResult<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "指定されたパスが存在しません: {}",
            path.display()
        ));
    }

    tracing::info!("Opening path in explorer: {}", path.display());

    std::process::Command::new("explorer")
        .arg("/select,")
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|e| {
            anyhow::anyhow!(
                "エクスプローラーの起動に失敗しました ({}): {}",
                path.display(),
                e
            )
        })?;

    tracing::info!("Path opened successfully in explorer: {}", path.display());

    Ok(())
}

aviutl2::register_generic_plugin!(OpenProjectDirectoryAux2);
