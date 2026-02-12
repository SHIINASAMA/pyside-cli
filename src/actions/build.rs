use std::time::Instant;

use crate::{
    builder::{builder::Builder, nuitka::NuitkaBuilder, pyinstaller::PyInstallerBuilder},
    cache::{Cache, load_cache, save_cache},
    cli::{Backend, BuildOptions, BuildStage},
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    files::Files,
    pyproject::PyProjectConfig,
    qt::{assets::compile_resources, i18n::compile_i18n_ts_files, ui::convert_ui_files},
    toolchain::Toolchain,
    utils::format_duration,
};

pub fn action(opt: BuildOptions) -> Result<(), Errcode> {
    let toolchain = Toolchain::new();
    let pyproject_config = PyProjectConfig::new("pyproject.toml".into())?;
    let Some(target_path) = &pyproject_config.scripts.get(&opt.target) else {
        return Err(Errcode::GeneralError(GeneralErrorKind::TargetNotFound {
            target: opt.target,
        }));
    };
    let files = Files::new(target_path);
    let mut cache: Cache = if opt.no_cache {
        Cache::default()
    } else {
        load_cache()
    };

    // I18N
    if matches!(
        opt.stage,
        BuildStage::I18n | BuildStage::Rc | BuildStage::All
    ) {
        let lrelease = match &toolchain.lrelease {
            Some(lrelease) => lrelease.clone(),
            None => {
                return Err(Errcode::ToolchainError(
                    ToolchainErrorKind::LReleaseUpdateNotFound,
                ));
            }
        };
        log::info!("Compiling i18n files...");
        let start = Instant::now();
        compile_i18n_ts_files(target_path, &lrelease, &files, &mut cache)?;
        log::info!(
            "I18n files compiled in {}.",
            format_duration(start.elapsed())
        );
    }

    // UI
    if matches!(opt.stage, BuildStage::Ui | BuildStage::Rc | BuildStage::All) {
        let uic = match &toolchain.uic {
            Some(uic) => uic.clone(),
            None => {
                return Err(Errcode::ToolchainError(ToolchainErrorKind::UicNotFound));
            }
        };
        log::info!("Converting ui files...");
        let start = Instant::now();
        convert_ui_files(target_path, &uic, &files, &mut cache)?;
        log::info!(
            "Ui files converted in {}.",
            format_duration(start.elapsed())
        );
    }

    // Assets
    if matches!(
        opt.stage,
        BuildStage::Assets | BuildStage::Rc | BuildStage::All
    ) {
        let rcc = match &toolchain.rcc {
            Some(rcc) => rcc.clone(),
            None => {
                return Err(Errcode::ToolchainError(ToolchainErrorKind::RccNotFound));
            }
        };
        let git = match &toolchain.git {
            Some(git) => git.clone(),
            None => {
                return Err(Errcode::ToolchainError(ToolchainErrorKind::GitNotFound));
            }
        };
        log::info!("Compiling assets...");
        let start = Instant::now();
        compile_resources(target_path, &rcc, &git, &files, &mut cache)?;
        log::info!("Assets compiled in {}.", format_duration(start.elapsed()));
    }

    save_cache(&cache).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::WriteFileFailed {
            path: "Cache".into(),
            source: e,
        })
    })?;

    // Build via backend
    if matches!(opt.stage, BuildStage::Build | BuildStage::All) {
        let build_type = opt.resolve_build_type();
        let backend: Box<dyn Builder> = match &opt.backend {
            Backend::Nuitka => {
                let nuitka_exe = toolchain
                    .nuitka
                    .clone()
                    .ok_or(Errcode::ToolchainError(ToolchainErrorKind::NuitkaNotFound))?;

                let mut extra_opts = opt.backend_args;
                extra_opts.extend(pyproject_config.extra_nuitka_options_list);

                #[cfg(target_os = "macos")]
                {
                    use crate::cli::BuildType;
                    use crate::builder::nuitka::mac::BundleInfo;

                    let bundle_info = if matches!(build_type, BuildType::Bundle) {
                        use crate::qt::assets::get_last_tag;

                        let git_exe = toolchain
                            .git
                            .clone()
                            .ok_or(Errcode::ToolchainError(ToolchainErrorKind::GitNotFound))?;

                        let version = get_last_tag(&git_exe, "0.0.0.0");

                        Some(BundleInfo {
                            name: opt.target.clone(),
                            version,
                        })
                    } else {
                        None
                    };

                    let builder = NuitkaBuilder::new(
                        &opt.target,
                        &target_path.to_string_lossy(),
                        &nuitka_exe,
                        build_type,
                        extra_opts,
                        bundle_info,
                    )?;

                    Box::new(builder)
                }

                #[cfg(not(target_os = "macos"))]
                {
                    let builder = NuitkaBuilder::new(
                        &opt.target,
                        &target_path.to_string_lossy(),
                        &nuitka_exe,
                        build_type,
                        extra_opts,
                    )?;

                    Box::new(builder)
                }
            }

            Backend::Pyinstaller => {
                let pyinstaller_exe = match &toolchain.pyinstaller {
                    Some(pyinstaller) => pyinstaller.clone(),
                    None => {
                        return Err(Errcode::ToolchainError(
                            ToolchainErrorKind::PyInstallerNotFound,
                        ));
                    }
                };
                let mut extra_opts = opt.backend_args;
                extra_opts.extend(pyproject_config.extra_pyinstaller_options_list);

                let builder = PyInstallerBuilder::new(
                    &opt.target,
                    target_path.to_string_lossy().to_string().as_str(),
                    &pyinstaller_exe,
                    build_type,
                    extra_opts,
                )?;

                Box::new(builder)
            }
        };

        log::info!("Building ...");
        let start = Instant::now();
        backend.pre_build()?;
        backend.build()?;
        backend.post_build()?;
        log::info!("Build completed in {}.", format_duration(start.elapsed()));
    }

    Ok(())
}
