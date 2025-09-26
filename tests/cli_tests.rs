    use plecakpkg::{handle_init, handle_install, InitCommand, InstallCommand};

    #[test]
    fn test_handle_init() {
        let init_args = InitCommand {
            name: "TestProject".to_string(),
        };

        let result = handle_init(&init_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_install_with_version() {
        let install_args = InstallCommand {
            url: "https://github.com/fmtlib/fmt".to_string(),
            version: Some("1.2.3".to_string()),
        };

        let result = handle_install(&install_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_install_without_version() {
        let install_args = InstallCommand {
            url: "https://github.com/fmtlib/fmt".to_string(),
            version: None,
        };

        let result = handle_install(&install_args);
        assert!(result.is_ok());
    }
