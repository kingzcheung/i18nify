# Change Log

## Version 0.2.0 (2024-08-30)
- Removed prettyplease

  - Removed prettyplease related features or dependencies.
- Refactor (i18n_codegen):

  - Updated the Cargo.toml file to adjust documentation links and change the version number to 0.2.0.
  - Updated the README.md file to add missing contributors and adjust example code to reflect the latest changes.
  - Refactored example_path.rs and src/lib.rs to optimize code generation logic, ensuring correct handling of file paths and attributes.
  - Fixed error handling logic in src/lib.rs to improve the accuracy of error messages.

BREAKING CHANGE: This commit introduces significant refactoring and version updates that may impact dependency resolution and code generation functionality. Please ensure thorough testing before upgrading.
- Implemented internationalization traits and adjusted locality methods (i18n_codegen):

  - Implemented internationalization traits and adjusted locality methods.
- Optimized the code generator and attribute parsing (i18n):

  - Optimized the code generator and improved attribute parsing.
- Improved error handling and macro parsing (i18n_codegen):
  - Improved error handling and optimized macro parsing.
- Updated dependencies (i18ncodegen):
  - Updated related dependencies.

## 0.1.1 - 2019-09-29

### Fixed

- Fixed typos in docs.

## 0.1.0 - 2019-09-29

Initial release.
