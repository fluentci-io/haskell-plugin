use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx",
            "install",
            "haskell.org/cabal",
            &format!("haskell.org@{}", version),
        ])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "build", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "test", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn install(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "install", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn check(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "check", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn clean(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "clean", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn sdist(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "sdist", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn upload(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "upload", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn report(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "cabal", "report", &args])?
        .stdout()?;

    Ok(stdout)
}
