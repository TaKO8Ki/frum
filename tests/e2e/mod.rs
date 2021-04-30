use crate::{e2e_test, eq, eq_re};

e2e_test!(use_installed_version, |dir| {
    dir.command().arg("install").arg("2.7.1").output();
    dir.command().arg("local").arg("2.7.1").output();
    eq_re!("^ruby 2.7.1", dir.ruby_version());
    dir.command().arg("global").arg("2.7.1").output();
    eq_re!("^ruby 2.7.1", dir.ruby_version());
});

e2e_test!(use_not_installed_version, |dir| {
    eq!(
        "error: Requested version 2.0.0 is not currently installed\n",
        dir.command().arg("local").arg("2.0.0").stderr()
    );
});

e2e_test!(uninstall_installed_version, |dir| {
    dir.command().arg("install").arg("2.7.1").output();
    assert!(dir.path().join("versions").join("2.7.1").exists());
    dir.command().arg("uninstall").arg("2.7.1").output();
    assert!(!dir.path().join("versions").join("2.7.1").exists());
});

e2e_test!(uninstall_not_installed_version, |dir| {
    dir.command().arg("install").arg("2.7.1").output();
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

e2e_test!(use_configure_opts, |dir| {
    dir.command()
        .arg("install")
        .arg("2.7.1")
        .arg("--disable-werror")
        .arg("--without-gmp")
        .output();
    dir.command().arg("local").arg("2.7.1").output();
    let configure_opts =
        dir.execute_ruby("-r rbconfig -e 'p RbConfig::CONFIG[\"configure_args\"]'");
    eq_re!("--disable-werror", &configure_opts);
    eq_re!("--without-gmp", &configure_opts);
});
