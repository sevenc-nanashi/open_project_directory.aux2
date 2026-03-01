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
    #[edit(name = "open_project_directory.aux2\\プロジェクトファイルのフォルダを開く")]
    fn open_project_directory(&mut self) -> aviutl2::AnyResult<()> {
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

        tracing::info!("Opening project directory: {}", project_dir.display());

        std::process::Command::new("explorer")
            .arg(project_dir)
            .spawn()
            .map(|_| ())
            .map_err(|e| {
                anyhow::anyhow!(
                    "エクスプローラーの起動に失敗しました ({}): {}",
                    project_dir.display(),
                    e
                )
            })?;

        tracing::info!(
            "Project directory opened successfully: {}",
            project_dir.display()
        );

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
}

aviutl2::register_generic_plugin!(OpenProjectDirectoryAux2);
