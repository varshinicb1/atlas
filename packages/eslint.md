---
kind: Package
id: package:eslint
name: ESLint Patterns
version: "9"
purpose: Document ESLint patterns — flat config, rules, plugins, custom rules, and integration with TypeScript, React, and Prettier for consistent code quality.
problem_solved: Provides a pluggable linting framework that catches programming errors, style issues, and anti-patterns before they reach production, enforcing team conventions automatically without manual code review for trivial issues.
install: npm install --save-dev eslint @eslint/js typescript-eslint
dependencies:
  - concept:javascript
  - concept:typescript
  - concept:code-quality
concepts:
  - name: Flat Config (eslint.config.js)
    id: concept:eslint/flat-config
    description: "ESLint 9's modern configuration system replacing .eslintrc. Flat config exports an array of config objects, each with files, ignores, plugins, rules, languageOptions, and linterOptions. Configs are composable, mergable, and have no inheritance chain confusion. Removes .eslintignore in favor of ignores array."
  - name: Rules
    id: concept:eslint/rules
    description: "Individual linting checks configured as rule-name: severity. Severity: 'off' (0), 'warn' (1), 'error' (2). Rules can be simple booleans or accept options: ['error', { allow: ['console'] }]. Core rules cover possible errors (no-unused-vars), best practices (no-param-reassign), and stylistic choices."
  - name: Plugins
    id: concept:eslint/plugins
    description: "Packages that add custom rules, configs, and processors. @typescript-eslint provides TS-aware rules, eslint-plugin-react for React patterns, eslint-plugin-import for module resolution. Plugins are imported and spread into config objects: ...plugin.tsPlugin.configs.recommended."
  - name: TypeScript Integration
    id: concept:eslint/typescript
    description: "typescript-eslint replaces @typescript-eslint/parser and @typescript-eslint/eslint-plugin. Provides type-aware rules (strict-boolean-expressions, no-floating-promises, no-unnecessary-condition) that require tsconfig path in languageOptions.parserOptions. These rules catch logic errors that simple AST analysis cannot detect."
  - name: Prettier Integration
    id: concept:eslint/prettier
    description: "ESLint handles code quality (bugs, anti-patterns); Prettier handles formatting (spacing, semicolons, quotes). eslint-config-prettier disables ESLint formatting rules that conflict with Prettier. With flat config, use prettier's recommended config: ...prettierConfig."
  - name: Custom Rules
    id: concept:eslint/custom-rules
    description: "Rules implemented as objects with meta (type, docs, fixable, schema) and create(context) returning AST visitor functions. context.report({node, message, fix}) reports violations. Use for project-specific conventions: naming patterns, import ordering, domain-specific validations."
  - name: Auto-fix
    id: concept:eslint/auto-fix
    description: "Rules marked as fixable provide automatic fixes via eslint --fix. VSCode ESLint extension applies fixes on save. Common fixable rules: no-unused-vars (remove), object-shorthand (convert), prefer-const, arrow-body-style. Some rules have suggestions (non-automatic, manual review needed)."
  - name: Ignore Patterns
    id: concept:eslint/ignores
    description: "In flat config, the ignores array in each config object specifies file globs to skip. Global ignores go at the top: { ignores: ['dist/**', 'node_modules/**', '*.config.*'] }. ESLint does not lint files outside the current working directory or files with binary extensions."
  - name: Processors
    id: concept:eslint/processors
    description: "Preprocess files of non-JS types. eslint-plugin-markdown extracts and lints JS from markdown code blocks. eslint-plugin-json for JSON files. eslint-plugin-yaml for YAML. Processors are declared in config: processor: 'plugin/processor-name'."
  - name: Environments
    id: concept:eslint/environments
    description: "Predefined global variables for specific runtimes. browser (window, document), node (process, require), es2025 (modern globals). In flat config, set via languageOptions.globals: { browser: true, es2025: true }. Replaces the env key from legacy configs."
  - name: Report Formats
    id: concept:eslint/formats
    description: "Output formatters: stylish (default, human-readable), json (machine-readable for CI), junit (XML for CI dashboards), html (detailed HTML report), sarif (for GitHub Code Scanning). Use -f format-name or -f ./custom-formatter.js for custom."
apis:
  - name: eslint.config.js array
    id: api:eslint/flat-config-export
    signature: "export default [{ files: ['src/**/*.ts'], ignores: ['**/*.test.ts'], rules: { 'no-console': 'error' }, languageOptions: { parser, globals }, plugins: {} }]"
    returns: An array of config objects.
    description: "The flat config entry point. Each config object has files (positive globs), ignores (negative globs), and configuration. Configs are merged in order; later configs override earlier ones for matching files."
  - name: context.report()
    id: api:eslint/context-report
    signature: "context.report({ node: Node, message: string | { message, data }, fix?: (fixer) => Fix, suggest?: SuggestionReportDescriptor[] }): void"
    returns: void
    description: "Reports a lint violation at the node's location. message supports interpolation with data. fix provides an auto-fix via fixer methods (replaceText, insertTextAfter, remove). suggest offers non-automatic suggestions users can review."
  - name: fixer.replaceText()
    id: api:eslint/fixer-replacetext
    signature: "fixer.replaceText(range: [number, number], text: string): Fix"
    returns: A fix object.
    description: "Replaces text in the specified range with new text. fixer.insertTextAfter(node, text) and fixer.remove(node) are also available. Multiple fixes from the same report are applied together if they do not conflict."
  - name: --fix flag
    id: api:eslint/fix-flag
    signature: "eslint . --fix [--fix-dry-run] [--fix-type problem|suggestion|layout]"
    returns: Exit code 0 (success) or 1 (unfixable errors remain).
    description: "Auto-fixes all fixable violations. --fix-dry-run shows what would be fixed without writing. --fix-type filters which category of fixes to apply (problem fixes for bugs, suggestion for improvements, layout for formatting)."
  - name: RuleTester
    id: api:eslint/rule-tester
    signature: "new RuleTester({ parser, parserOptions, rules }).run(name, rule, { valid: [{code, options}], invalid: [{code, errors, output}] })"
    returns: Test results via assertion framework.
    description: "Built-in test harness for custom ESLint rules. Define valid test cases (should not report) and invalid cases (should report with expected errors and optional auto-fix output). Used inside Jest or Mocha describe blocks."
sections:
  - title: Flat Config with TypeScript
    id: section:eslint/flat-config-ts
    content: |
      ESLint 9 flat config for a TypeScript project with React:

      ```typescript
      import js from '@eslint/js';
      import tseslint from 'typescript-eslint';
      import reactHooks from 'eslint-plugin-react-hooks';
      import reactRefresh from 'eslint-plugin-react-refresh';
      import globals from 'globals';

      export default tseslint.config(
          { ignores: ['dist/**', '*.config.*'] },
          {
              extends: [js.configs.recommended, ...tseslint.configs.recommended],
              files: ['src/**/*.{ts,tsx}'],
              languageOptions: {
                  globals: { ...globals.browser, ...globals.es2025 },
                  parserOptions: {
                      project: './tsconfig.json',
                      tsconfigRootDir: import.meta.dirname,
                  },
              },
              plugins: {
                  'react-hooks': reactHooks,
                  'react-refresh': reactRefresh,
              },
              rules: {
                  ...reactHooks.configs.recommended.rules,
                  'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
                  '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
                  '@typescript-eslint/no-floating-promises': 'error',
                  'no-console': ['warn', { allow: ['warn', 'error'] }],
              },
          }
      );
      ```

      The `tseslint.config()` helper merges configs intelligently, applying TypeScript-specific rules only to files that match the project's tsconfig include.
  - title: Rule Configuration Best Practices
    id: section:eslint/rule-best-practices
    content: |
      Configure rules for maximum bug prevention with minimum noise:

      ```typescript
      rules: {
          // Error — catch these before they reach CI
          'no-unused-vars': 'off', // Delegate to TS
          '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
          '@typescript-eslint/no-floating-promises': 'error',
          '@typescript-eslint/require-await': 'error',

          // Warn — intentional but should be reviewed
          'no-console': 'warn',
          '@typescript-eslint/no-explicit-any': 'warn',
          'react-hooks/exhaustive-deps': 'warn',

          // Off — style decisions handled by Prettier
          'semi': 'off',
          'quotes': 'off',
          'indent': 'off',
      }
      ```

      Use `eslint --quiet` in CI to suppress warnings and only fail on errors. For monorepos, configure per-package overrides in workspace configs that extend the root config.
---
