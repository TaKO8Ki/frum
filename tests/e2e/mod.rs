use crate::{e2e_test, eq, eq_re};

e2e_test!(init, |dir| {
    dir.command().arg("init").output();
    let stdout = dir.command().arg("init").stdout();
    eq_re!("PATH", stdout);
    eq_re!("FRUM_MULTISHELL_PATH", stdout);
    eq_re!("FRUM_DIR", stdout);
    eq_re!("FRUM_LOGLEVEL", stdout);
    eq_re!("FRUM_RUBY_BUILD_MIRROR", stdout);
    eq_re!("frum --log-level quiet local", stdout);
});

e2e_test!(use_installed_version, |dir| {
    dir.command().arg("install").arg("2.7.0").output();
    dir.command().arg("local").arg("2.7.0").output();
    eq_re!("^ruby 2.7.0", dir.ruby_version());
    dir.command().arg("global").arg("2.7.0").output();
    eq_re!("^ruby 2.7.0", dir.ruby_version());
});

e2e_test!(use_not_installed_version, |dir| {
    eq!(
        "error: Requested version 2.7.0 is not currently installed\n",
        dir.command().arg("local").arg("2.7.0").stderr()
    );
});

e2e_test!(uninstall_installed_version, |dir| {
    dir.command().arg("install").arg("3.0.1").output();
    assert!(dir.path().join("versions").join("3.0.1").exists());
    dir.command().arg("uninstall").arg("3.0.1").output();
    assert!(!dir.path().join("versions").join("3.0.1").exists());
});

e2e_test!(uninstall_not_installed_version, |dir| {
    dir.command().arg("install").arg("2.7.0").output();
    eq!(
        "error: Can't find version: 2.6.5\n",
        dir.command().arg("uninstall").arg("2.6.5").stderr()
    );
});

e2e_test!(use_version_specified_in_ruby_version_file, |dir| {
    eq!(
        "error: Can't find version in dotfiles. Please provide a version manually to the command.\n",
        dir.command().arg("local").stderr()
    );
    eq!(
        "",
        dir.command()
            .arg("--log-level")
            .arg("quiet")
            .arg("local")
            .stderr()
    );
});

e2e_test!(install_ruby_in_specific_base_dir, |dir| {
    let base_dir = dir.path().join("foo");
    dir.command()
        .arg("--frum-dir")
        .arg(&base_dir)
        .arg("install")
        .arg("2.7.0")
        .output();
    dir.command()
        .arg("--frum-dir")
        .arg(&base_dir)
        .arg("local")
        .arg("2.7.0")
        .output();
    assert!(base_dir.join("versions").exists());
});

e2e_test!(use_configure_opts, |dir| {
    dir.command()
        .arg("install")
        .arg("2.7.1")
        .arg("--disable-werror")
        .arg("--without-gmp")
        .output();
    dir.command().arg("local").arg("2.7.1").output();
    let configure_opts = dir.ruby_configure_options();
    eq_re!("--disable-werror", configure_opts);
    eq_re!("--with-openssl-dir", configure_opts);
});
