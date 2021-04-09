use crate::{e2e_test, eq, eq_re};

e2e_test!(use_installed_version, |dir| {
    eq!(
        "==> Installing Ruby 2.7.1\n",
        dir.command().arg("install").arg("2.7.1").stdout()
    );
    eq!("", dir.command().arg("local").arg("2.7.1").stdout());
    eq_re!("^ruby 2.7.1", dir.ruby_version());
    eq!("", dir.command().arg("global").arg("2.7.1").stdout());
    eq_re!("^ruby 2.7.1", dir.ruby_version());
});

e2e_test!(use_not_installed_version, |dir| {
    dir.command().arg("local").arg("2.0.0").assert_err();
});
