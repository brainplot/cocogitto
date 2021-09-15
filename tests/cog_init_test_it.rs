use anyhow::Result;
use assert_cmd::prelude::*;
use cocogitto::CONFIG_PATH;
use helper::*;
use std::process::Command;

mod helper;
use helper::run_test_with_context;

#[test]
#[cfg(not(tarpaulin))]
fn init_empty_repo_in_target_dir() -> Result<()> {
    // Arrange
    run_test_with_context(|context| {
        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo")
            .assert()
            .success();

        // Assert
        let repo_directory = context.test_dir.join("test_repo");
        assert_file_exists(repo_directory);
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn init_existing_repo() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        git_init_with_path("test_repo_existing")?;
        assert_file_exists(context.test_dir.join("test_repo_existing"));
        helper::git_commit("chore: test commit")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo_existing")
            // Assert
            .assert()
            .success();
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn fail_if_config_exist() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        helper::git_init_with_path("test_repo_existing")?;
        std::fs::write(
            &context.test_dir.join("test_repo_existing").join(CONFIG_PATH),
            "[hooks]",
        )?;
        helper::git_commit("chore: test commit")?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            .arg("test_repo_existing")
            // Assert
            .assert()
            .failure();
        Ok(())
    })
}

#[test]
#[cfg(not(tarpaulin))]
fn init_current_dir_with_no_arg() -> Result<()> {
    run_test_with_context(|context| {
        // Arrange
        let path = context.test_dir.join("test_repo_no_args");
        std::fs::create_dir(&path)?;
        std::env::set_current_dir(&path)?;

        // Act
        Command::cargo_bin("cog")?
            .arg("init")
            // Assert
            .assert().success();
        Ok(())
    })

}
