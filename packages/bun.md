---
kind: Package
id: package:bun
name: "bun Knowledge Package"
version: "0.1.0"
purpose: |
  Auto-generated knowledge package crawled from https://bun.sh/docs.
  Covers 5 pages of documentation.
problem_solved: |
  Provides structured knowledge extracted from the official bun.sh documentation
  for use in AI agent decision-making.
install: |
  ```bash
  atlas install bun.md
  ```
concepts:
  - name: "Welcome to Bun"
    id: concept:page_0_bun
    description: |
      Extracted from documentation: Welcome to Bun
  - name: "Bun Runtime"
    id: concept:page_1_bun
    description: |
      Extracted from documentation: Bun Runtime
  - name: "Bun Package Manager"
    id: concept:page_2_bun
    description: |
      Extracted from documentation: Bun Package Manager
  - name: "Bun Test Runner"
    id: concept:page_3_bun
    description: |
      Extracted from documentation: Bun Test Runner
  - name: "Bun Bundler"
    id: concept:page_4_bun
    description: |
      Extracted from documentation: Bun Bundler
  - name: "​Get Started"
    id: concept:page_5_bun
    description: |
      Extracted from documentation: ​Get Started
  - name: "Install Bun"
    id: concept:page_6_bun
    description: |
      Extracted from documentation: Install Bun
  - name: "Quickstart"
    id: concept:page_7_bun
    description: |
      Extracted from documentation: Quickstart
  - name: "​What’s Inside"
    id: concept:page_8_bun
    description: |
      Extracted from documentation: ​What’s Inside
  - name: "​What is Bun?"
    id: concept:page_9_bun
    description: |
      Extracted from documentation: ​What is Bun?
  - name: "​What is a runtime?"
    id: concept:page_10_bun
    description: |
      Extracted from documentation: ​What is a runtime?
  - name: "​Design goals"
    id: concept:page_11_bun
    description: |
      Extracted from documentation: ​Design goals
  - name: "​Browsers"
    id: concept:page_12_bun
    description: |
      Extracted from documentation: ​Browsers
  - name: "​Node.js"
    id: concept:page_13_bun
    description: |
      Extracted from documentation: ​Node.js
  - name: "Bun"
    id: concept:page_14_bun
    description: |
      Extracted from documentation: Bun
  - name: "Docs"
    id: concept:page_15_bun
    description: |
      Extracted from documentation: Docs
  - name: "Optional"
    id: concept:page_16_bun
    description: |
      Extracted from documentation: Optional
  - name: "Bytecode Caching"
    id: concept:page_17_bun
    description: |
      Extracted from documentation: Bytecode Caching
  - name: "Usage"
    id: concept:page_18_bun
    description: |
      Extracted from documentation: Usage
  - name: "Basic usage (CommonJS)"
    id: concept:page_19_bun
    description: |
      Extracted from documentation: Basic usage (CommonJS)
  - name: "With standalone executables"
    id: concept:page_20_bun
    description: |
      Extracted from documentation: With standalone executables
  - name: "ESM (requires --compile)"
    id: concept:page_21_bun
    description: |
      Extracted from documentation: ESM (requires --compile)
  - name: "CommonJS (works with or without --compile)"
    id: concept:page_22_bun
    description: |
      Extracted from documentation: CommonJS (works with or without --compile)
  - name: "ESM bytecode"
    id: concept:page_23_bun
    description: |
      Extracted from documentation: ESM bytecode
  - name: "Combining with other optimizations"
    id: concept:page_24_bun
    description: |
      Extracted from documentation: Combining with other optimizations
  - name: "Performance impact"
    id: concept:page_25_bun
    description: |
      Extracted from documentation: Performance impact
  - name: "When to use bytecode"
    id: concept:page_26_bun
    description: |
      Extracted from documentation: When to use bytecode
  - name: "Great for:"
    id: concept:page_27_bun
    description: |
      Extracted from documentation: Great for:
  - name: "CLI tools"
    id: concept:page_28_bun
    description: |
      Extracted from documentation: CLI tools
  - name: "Build tools and task runners"
    id: concept:page_29_bun
    description: |
      Extracted from documentation: Build tools and task runners
  - name: "Standalone executables"
    id: concept:page_30_bun
    description: |
      Extracted from documentation: Standalone executables
  - name: "Skip it for:"
    id: concept:page_31_bun
    description: |
      Extracted from documentation: Skip it for:
  - name: "Limitations"
    id: concept:page_32_bun
    description: |
      Extracted from documentation: Limitations
  - name: "Version compatibility"
    id: concept:page_33_bun
    description: |
      Extracted from documentation: Version compatibility
  - name: "After updating Bun"
    id: concept:page_34_bun
    description: |
      Extracted from documentation: After updating Bun
  - name: "Source code still required"
    id: concept:page_35_bun
    description: |
      Extracted from documentation: Source code still required
  - name: "Bytecode is not obfuscation"
    id: concept:page_36_bun
    description: |
      Extracted from documentation: Bytecode is not obfuscation
  - name: "Production deployment"
    id: concept:page_37_bun
    description: |
      Extracted from documentation: Production deployment
  - name: "Docker"
    id: concept:page_38_bun
    description: |
      Extracted from documentation: Docker
  - name: "CI/CD"
    id: concept:page_39_bun
    description: |
      Extracted from documentation: CI/CD
  - name: "GitHub Actions"
    id: concept:page_40_bun
    description: |
      Extracted from documentation: GitHub Actions
  - name: "Debugging"
    id: concept:page_41_bun
    description: |
      Extracted from documentation: Debugging
  - name: "Verify bytecode is being used"
    id: concept:page_42_bun
    description: |
      Extracted from documentation: Verify bytecode is being used
  - name: "Common issues"
    id: concept:page_43_bun
    description: |
      Extracted from documentation: Common issues
  - name: "What is bytecode?"
    id: concept:page_44_bun
    description: |
      Extracted from documentation: What is bytecode?
  - name: "Why lazy parsing makes this even better"
    id: concept:page_45_bun
    description: |
      Extracted from documentation: Why lazy parsing makes this even better
  - name: "The bytecode format"
    id: concept:page_46_bun
    description: |
      Extracted from documentation: The bytecode format
  - name: "Inside a .jsc file"
    id: concept:page_47_bun
    description: |
      Extracted from documentation: Inside a .jsc file
  - name: "What bytecode does NOT contain"
    id: concept:page_48_bun
    description: |
      Extracted from documentation: What bytecode does NOT contain
  - name: "The tradeoff: file size"
    id: concept:page_49_bun
    description: |
      Extracted from documentation: The tradeoff: file size
  - name: "Why is bytecode so much larger?"
    id: concept:page_50_bun
    description: |
      Extracted from documentation: Why is bytecode so much larger?
  - name: "Mitigation strategies"
    id: concept:page_51_bun
    description: |
      Extracted from documentation: Mitigation strategies
  - name: "Versioning and portability"
    id: concept:page_52_bun
    description: |
      Extracted from documentation: Versioning and portability
  - name: "Cross-architecture portability: ✅"
    id: concept:page_53_bun
    description: |
      Extracted from documentation: Cross-architecture portability: ✅
  - name: "Cross-version portability: ❌"
    id: concept:page_54_bun
    description: |
      Extracted from documentation: Cross-version portability: ❌
  - name: "Unlinked vs. linked bytecode"
    id: concept:page_55_bun
    description: |
      Extracted from documentation: Unlinked vs. linked bytecode
  - name: "Unlinked bytecode (what's cached)"
    id: concept:page_56_bun
    description: |
      Extracted from documentation: Unlinked bytecode (what's cached)
  - name: "Linked bytecode (runtime execution)"
    id: concept:page_57_bun
    description: |
      Extracted from documentation: Linked bytecode (runtime execution)
  - name: "CSS"
    id: concept:page_58_bun
    description: |
      Extracted from documentation: CSS
  - name: "Transpiling"
    id: concept:page_59_bun
    description: |
      Extracted from documentation: Transpiling
  - name: "Browser Compatibility"
    id: concept:page_60_bun
    description: |
      Extracted from documentation: Browser Compatibility
  - name: "Syntax Lowering"
    id: concept:page_61_bun
    description: |
      Extracted from documentation: Syntax Lowering
  - name: "Nesting"
    id: concept:page_62_bun
    description: |
      Extracted from documentation: Nesting
  - name: "Color mix"
    id: concept:page_63_bun
    description: |
      Extracted from documentation: Color mix
  - name: "Relative colors"
    id: concept:page_64_bun
    description: |
      Extracted from documentation: Relative colors
  - name: "LAB colors"
    id: concept:page_65_bun
    description: |
      Extracted from documentation: LAB colors
  - name: "Color function"
    id: concept:page_66_bun
    description: |
      Extracted from documentation: Color function
  - name: "HWB colors"
    id: concept:page_67_bun
    description: |
      Extracted from documentation: HWB colors
  - name: "Color notation"
    id: concept:page_68_bun
    description: |
      Extracted from documentation: Color notation
  - name: "light-dark() color function"
    id: concept:page_69_bun
    description: |
      Extracted from documentation: light-dark() color function
  - name: "Logical properties"
    id: concept:page_70_bun
    description: |
      Extracted from documentation: Logical properties
  - name: ":dir() selector"
    id: concept:page_71_bun
    description: |
      Extracted from documentation: :dir() selector
  - name: ":lang() selector"
    id: concept:page_72_bun
    description: |
      Extracted from documentation: :lang() selector
  - name: ":is() selector"
    id: concept:page_73_bun
    description: |
      Extracted from documentation: :is() selector
  - name: ":not() selector"
    id: concept:page_74_bun
    description: |
      Extracted from documentation: :not() selector
  - name: "Math functions"
    id: concept:page_75_bun
    description: |
      Extracted from documentation: Math functions
  - name: "Media query ranges"
    id: concept:page_76_bun
    description: |
      Extracted from documentation: Media query ranges
  - name: "Shorthands"
    id: concept:page_77_bun
    description: |
      Extracted from documentation: Shorthands
  - name: "Double position gradients"
    id: concept:page_78_bun
    description: |
      Extracted from documentation: Double position gradients
  - name: "4caf50 0% 25%,"
    id: concept:page_79_bun
    description: |
      Extracted from documentation: 4caf50 0% 25%,
  - name: "4caf50 0%,"
    id: concept:page_80_bun
    description: |
      Extracted from documentation: 4caf50 0%,
  - name: "4caf50 25%,"
    id: concept:page_81_bun
    description: |
      Extracted from documentation: 4caf50 25%,
  - name: "ffc107 50%,"
    id: concept:page_82_bun
    description: |
      Extracted from documentation: ffc107 50%,
  - name: "2196f3 75%,"
    id: concept:page_83_bun
    description: |
      Extracted from documentation: 2196f3 75%,
  - name: "9c27b0 100% /* Two stops for purple section */"
    id: concept:page_84_bun
    description: |
      Extracted from documentation: 9c27b0 100% /* Two stops for purple section */
  - name: "system-ui font"
    id: concept:page_85_bun
    description: |
      Extracted from documentation: system-ui font
  - name: "CSS Modules"
    id: concept:page_86_bun
    description: |
      Extracted from documentation: CSS Modules
  - name: "Getting started"
    id: concept:page_87_bun
    description: |
      Extracted from documentation: Getting started
  - name: "Composition"
    id: concept:page_88_bun
    description: |
      Extracted from documentation: Composition
  - name: "button {"
    id: concept:page_89_bun
    description: |
      Extracted from documentation: button {
  - name: "Composing from a separate CSS module file"
    id: concept:page_90_bun
    description: |
      Extracted from documentation: Composing from a separate CSS module file
  - name: "esbuild"
    id: concept:page_91_bun
    description: |
      Extracted from documentation: esbuild
  - name: "Performance"
    id: concept:page_92_bun
    description: |
      Extracted from documentation: Performance
  - name: "CLI API"
    id: concept:page_93_bun
    description: |
      Extracted from documentation: CLI API
  - name: "bun"
    id: concept:page_94_bun
    description: |
      Extracted from documentation: bun
  - name: "JavaScript API"
    id: concept:page_95_bun
    description: |
      Extracted from documentation: JavaScript API
  - name: "Plugin API"
    id: concept:page_96_bun
    description: |
      Extracted from documentation: Plugin API
  - name: "onResolve"
    id: concept:page_97_bun
    description: |
      Extracted from documentation: onResolve
  - name: "onLoad"
    id: concept:page_98_bun
    description: |
      Extracted from documentation: onLoad
  - name: "Single-file executable"
    id: concept:page_99_bun
    description: |
      Extracted from documentation: Single-file executable
  - name: "Cross-compile to other platforms"
    id: concept:page_100_bun
    description: |
      Extracted from documentation: Cross-compile to other platforms
  - name: "To support CPUs from before 2013, use the baseline version (nehalem)"
    id: concept:page_101_bun
    description: |
      Extracted from documentation: To support CPUs from before 2013, use the baseline version (nehalem)
  - name: "To explicitly only support CPUs from 2013 and later, use the modern version (haswell)"
    id: concept:page_102_bun
    description: |
      Extracted from documentation: To explicitly only support CPUs from 2013 and later, use the modern version (haswell)
  - name: "modern is faster, but baseline is more compatible."
    id: concept:page_103_bun
    description: |
      Extracted from documentation: modern is faster, but baseline is more compatible.
  - name: "Note: the default architecture is x64 if no architecture is specified."
    id: concept:page_104_bun
    description: |
      Extracted from documentation: Note: the default architecture is x64 if no architecture is specified.
  - name: "note: if no .exe extension is provided, Bun adds it automatically for Windows executables"
    id: concept:page_105_bun
    description: |
      Extracted from documentation: note: if no .exe extension is provided, Bun adds it automatically for Windows executables
  - name: "Supported targets"
    id: concept:page_106_bun
    description: |
      Extracted from documentation: Supported targets
  - name: "Build-time constants"
    id: concept:page_107_bun
    description: |
      Extracted from documentation: Build-time constants
  - name: "Deploying to production"
    id: concept:page_108_bun
    description: |
      Extracted from documentation: Deploying to production
  - name: "Bytecode compilation"
    id: concept:page_109_bun
    description: |
      Extracted from documentation: Bytecode compilation
  - name: "What do these flags do?"
    id: concept:page_110_bun
    description: |
      Extracted from documentation: What do these flags do?
  - name: "Embedding runtime arguments"
    id: concept:page_111_bun
    description: |
      Extracted from documentation: Embedding runtime arguments
  - name: "Runtime arguments via `BUN_OPTIONS`"
    id: concept:page_112_bun
    description: |
      Extracted from documentation: Runtime arguments via `BUN_OPTIONS`
  - name: "Enable CPU profiling on a compiled executable"
    id: concept:page_113_bun
    description: |
      Extracted from documentation: Enable CPU profiling on a compiled executable
  - name: "Enable heap profiling with markdown output"
    id: concept:page_114_bun
    description: |
      Extracted from documentation: Enable heap profiling with markdown output
  - name: "Combine multiple flags"
    id: concept:page_115_bun
    description: |
      Extracted from documentation: Combine multiple flags
  - name: "Automatic config loading"
    id: concept:page_116_bun
    description: |
      Extracted from documentation: Automatic config loading
  - name: "Enabling config loading at runtime"
    id: concept:page_117_bun
    description: |
      Extracted from documentation: Enabling config loading at runtime
  - name: "Enable runtime loading of tsconfig.json"
    id: concept:page_118_bun
    description: |
      Extracted from documentation: Enable runtime loading of tsconfig.json
  - name: "Enable runtime loading of package.json"
    id: concept:page_119_bun
    description: |
      Extracted from documentation: Enable runtime loading of package.json
  - name: "Enable both"
    id: concept:page_120_bun
    description: |
      Extracted from documentation: Enable both
  - name: "Disabling config loading at runtime"
    id: concept:page_121_bun
    description: |
      Extracted from documentation: Disabling config loading at runtime
  - name: "Disable .env loading"
    id: concept:page_122_bun
    description: |
      Extracted from documentation: Disable .env loading
  - name: "Disable bunfig.toml loading"
    id: concept:page_123_bun
    description: |
      Extracted from documentation: Disable bunfig.toml loading
  - name: "Disable all config loading"
    id: concept:page_124_bun
    description: |
      Extracted from documentation: Disable all config loading
  - name: "Act as the Bun CLI"
    id: concept:page_125_bun
    description: |
      Extracted from documentation: Act as the Bun CLI
  - name: "Executable runs its own entrypoint by default"
    id: concept:page_126_bun
    description: |
      Extracted from documentation: Executable runs its own entrypoint by default
  - name: "With the env var, the executable acts like the `bun` CLI"
    id: concept:page_127_bun
    description: |
      Extracted from documentation: With the env var, the executable acts like the `bun` CLI
  - name: "Full-stack executables"
    id: concept:page_128_bun
    description: |
      Extracted from documentation: Full-stack executables
  - name: "Worker"
    id: concept:page_129_bun
    description: |
      Extracted from documentation: Worker
  - name: "SQLite"
    id: concept:page_130_bun
    description: |
      Extracted from documentation: SQLite
  - name: "Embed assets & files"
    id: concept:page_131_bun
    description: |
      Extracted from documentation: Embed assets & files
  - name: "How it works"
    id: concept:page_132_bun
    description: |
      Extracted from documentation: How it works
  - name: "Reading embedded files with Bun.file()"
    id: concept:page_133_bun
    description: |
      Extracted from documentation: Reading embedded files with Bun.file()
  - name: "Reading embedded files with Node.js fs"
    id: concept:page_134_bun
    description: |
      Extracted from documentation: Reading embedded files with Node.js fs
  - name: "Practical examples"
    id: concept:page_135_bun
    description: |
      Extracted from documentation: Practical examples
  - name: "Embedding a JSON config file"
    id: concept:page_136_bun
    description: |
      Extracted from documentation: Embedding a JSON config file
  - name: "Serving static assets in an HTTP server"
    id: concept:page_137_bun
    description: |
      Extracted from documentation: Serving static assets in an HTTP server
  - name: "Embedding templates"
    id: concept:page_138_bun
    description: |
      Extracted from documentation: Embedding templates
  - name: "Embedding binary files"
    id: concept:page_139_bun
    description: |
      Extracted from documentation: Embedding binary files
  - name: "Embed SQLite databases"
    id: concept:page_140_bun
    description: |
      Extracted from documentation: Embed SQLite databases
  - name: "Embed N-API Addons"
    id: concept:page_141_bun
    description: |
      Extracted from documentation: Embed N-API Addons
  - name: "Embed directories"
    id: concept:page_142_bun
    description: |
      Extracted from documentation: Embed directories
  - name: "Detecting standalone mode at runtime"
    id: concept:page_143_bun
    description: |
      Extracted from documentation: Detecting standalone mode at runtime
  - name: "Listing embedded files"
    id: concept:page_144_bun
    description: |
      Extracted from documentation: Listing embedded files
  - name: "Content hash"
    id: concept:page_145_bun
    description: |
      Extracted from documentation: Content hash
  - name: "Minification"
    id: concept:page_146_bun
    description: |
      Extracted from documentation: Minification
  - name: "Windows-specific flags"
    id: concept:page_147_bun
    description: |
      Extracted from documentation: Windows-specific flags
  - name: "Custom icon"
    id: concept:page_148_bun
    description: |
      Extracted from documentation: Custom icon
  - name: "Hide console window (for GUI apps)"
    id: concept:page_149_bun
    description: |
      Extracted from documentation: Hide console window (for GUI apps)
  - name: "Code signing on macOS"
    id: concept:page_150_bun
    description: |
      Extracted from documentation: Code signing on macOS
  - name: "Code splitting"
    id: concept:page_151_bun
    description: |
      Extracted from documentation: Code splitting
  - name: "Using plugins"
    id: concept:page_152_bun
    description: |
      Extracted from documentation: Using plugins
  - name: "Unsupported CLI arguments"
    id: concept:page_153_bun
    description: |
      Extracted from documentation: Unsupported CLI arguments
  - name: "API reference"
    id: concept:page_154_bun
    description: |
      Extracted from documentation: API reference
  - name: "Complete example"
    id: concept:page_155_bun
    description: |
      Extracted from documentation: Complete example
  - name: "Fullstack dev server"
    id: concept:page_156_bun
    description: |
      Extracted from documentation: Fullstack dev server
  - name: "HTML Routes"
    id: concept:page_157_bun
    description: |
      Extracted from documentation: HTML Routes
  - name: "HTML Imports as Routes"
    id: concept:page_158_bun
    description: |
      Extracted from documentation: HTML Imports as Routes
  - name: "HTML Processing Example"
    id: concept:page_159_bun
    description: |
      Extracted from documentation: HTML Processing Example
  - name: "React Integration"
    id: concept:page_160_bun
    description: |
      Extracted from documentation: React Integration
  - name: "Development Mode"
    id: concept:page_161_bun
    description: |
      Extracted from documentation: Development Mode
  - name: "Development Mode Features"
    id: concept:page_162_bun
    description: |
      Extracted from documentation: Development Mode Features
  - name: "Advanced Development Configuration"
    id: concept:page_163_bun
    description: |
      Extracted from documentation: Advanced Development Configuration
  - name: "Development vs Production"
    id: concept:page_164_bun
    description: |
      Extracted from documentation: Development vs Production
  - name: "Production Mode"
    id: concept:page_165_bun
    description: |
      Extracted from documentation: Production Mode
  - name: "Ahead of Time Bundling (Recommended)"
    id: concept:page_166_bun
    description: |
      Extracted from documentation: Ahead of Time Bundling (Recommended)
  - name: "Runtime Bundling"
    id: concept:page_167_bun
    description: |
      Extracted from documentation: Runtime Bundling
  - name: "API Routes"
    id: concept:page_168_bun
    description: |
      Extracted from documentation: API Routes
  - name: "HTTP Method Handlers"
    id: concept:page_169_bun
    description: |
      Extracted from documentation: HTTP Method Handlers
  - name: "Dynamic Routes"
    id: concept:page_170_bun
    description: |
      Extracted from documentation: Dynamic Routes
  - name: "Request Handling"
    id: concept:page_171_bun
    description: |
      Extracted from documentation: Request Handling
  - name: "Plugins"
    id: concept:page_172_bun
    description: |
      Extracted from documentation: Plugins
  - name: "TailwindCSS Plugin"
    id: concept:page_173_bun
    description: |
      Extracted from documentation: TailwindCSS Plugin
  - name: "Custom Plugins"
    id: concept:page_174_bun
    description: |
      Extracted from documentation: Custom Plugins
  - name: "Inline Environment Variables"
    id: concept:page_175_bun
    description: |
      Extracted from documentation: Inline Environment Variables
  - name: "env = \"inline\"  # inline all environment variables"
    id: concept:page_176_bun
    description: |
      Extracted from documentation: env = "inline"  # inline all environment variables
  - name: "env = \"disable\" # disable env var replacement (default)"
    id: concept:page_177_bun
    description: |
      Extracted from documentation: env = "disable" # disable env var replacement (default)
  - name: "How It Works"
    id: concept:page_178_bun
    description: |
      Extracted from documentation: How It Works
  - name: "Processing Pipeline"
    id: concept:page_179_bun
    description: |
      Extracted from documentation: Processing Pipeline
  - name: "Complete Example"
    id: concept:page_180_bun
    description: |
      Extracted from documentation: Complete Example
  - name: "Best Practices"
    id: concept:page_181_bun
    description: |
      Extracted from documentation: Best Practices
  - name: "Project Structure"
    id: concept:page_182_bun
    description: |
      Extracted from documentation: Project Structure
  - name: "Environment-Based Configuration"
    id: concept:page_183_bun
    description: |
      Extracted from documentation: Environment-Based Configuration
  - name: "Error Handling"
    id: concept:page_184_bun
    description: |
      Extracted from documentation: Error Handling
  - name: "API Response Helpers"
    id: concept:page_185_bun
    description: |
      Extracted from documentation: API Response Helpers
  - name: "Type Safety"
    id: concept:page_186_bun
    description: |
      Extracted from documentation: Type Safety
  - name: "Deployment"
    id: concept:page_187_bun
    description: |
      Extracted from documentation: Deployment
  - name: "Production Build"
    id: concept:page_188_bun
    description: |
      Extracted from documentation: Production Build
  - name: "Build for production"
    id: concept:page_189_bun
    description: |
      Extracted from documentation: Build for production
  - name: "Run production server"
    id: concept:page_190_bun
    description: |
      Extracted from documentation: Run production server
  - name: "Docker Deployment"
    id: concept:page_191_bun
    description: |
      Extracted from documentation: Docker Deployment
  - name: "Install dependencies"
    id: concept:page_192_bun
    description: |
      Extracted from documentation: Install dependencies
  - name: "Copy source code"
    id: concept:page_193_bun
    description: |
      Extracted from documentation: Copy source code
  - name: "Build application"
    id: concept:page_194_bun
    description: |
      Extracted from documentation: Build application
  - name: "Production stage"
    id: concept:page_195_bun
    description: |
      Extracted from documentation: Production stage
  - name: "Environment Variables"
    id: concept:page_196_bun
    description: |
      Extracted from documentation: Environment Variables
  - name: "Migration from Other Frameworks"
    id: concept:page_197_bun
    description: |
      Extracted from documentation: Migration from Other Frameworks
  - name: "From Express + Webpack"
    id: concept:page_198_bun
    description: |
      Extracted from documentation: From Express + Webpack
  - name: "From Next.js API Routes"
    id: concept:page_199_bun
    description: |
      Extracted from documentation: From Next.js API Routes
  - name: "Limitations and Future Plans"
    id: concept:page_200_bun
    description: |
      Extracted from documentation: Limitations and Future Plans
  - name: "Current Limitations"
    id: concept:page_201_bun
    description: |
      Extracted from documentation: Current Limitations
  - name: "Planned Features"
    id: concept:page_202_bun
    description: |
      Extracted from documentation: Planned Features
  - name: "Hot reloading"
    id: concept:page_203_bun
    description: |
      Extracted from documentation: Hot reloading
  - name: "`import.meta.hot` API Reference"
    id: concept:page_204_bun
    description: |
      Extracted from documentation: `import.meta.hot` API Reference
  - name: "API Methods"
    id: concept:page_205_bun
    description: |
      Extracted from documentation: API Methods
  - name: "import.meta.hot.accept()"
    id: concept:page_206_bun
    description: |
      Extracted from documentation: import.meta.hot.accept()
  - name: "With callback"
    id: concept:page_207_bun
    description: |
      Extracted from documentation: With callback
  - name: "Accepting other modules"
    id: concept:page_208_bun
    description: |
      Extracted from documentation: Accepting other modules
  - name: "With multiple dependencies"
    id: concept:page_209_bun
    description: |
      Extracted from documentation: With multiple dependencies
  - name: "import.meta.hot.data"
    id: concept:page_210_bun
    description: |
      Extracted from documentation: import.meta.hot.data
  - name: "import.meta.hot.dispose()"
    id: concept:page_211_bun
    description: |
      Extracted from documentation: import.meta.hot.dispose()
  - name: "import.meta.hot.prune()"
    id: concept:page_212_bun
    description: |
      Extracted from documentation: import.meta.hot.prune()
  - name: "import.meta.hot.on() and off()"
    id: concept:page_213_bun
    description: |
      Extracted from documentation: import.meta.hot.on() and off()
  - name: "Built-in events"
    id: concept:page_214_bun
    description: |
      Extracted from documentation: Built-in events
  - name: "HTML & static sites"
    id: concept:page_215_bun
    description: |
      Extracted from documentation: HTML & static sites
  - name: "Single Page Apps (SPA)"
    id: concept:page_216_bun
    description: |
      Extracted from documentation: Single Page Apps (SPA)
  - name: "Multi-page apps (MPA)"
    id: concept:page_217_bun
    description: |
      Extracted from documentation: Multi-page apps (MPA)
  - name: "Glob patterns"
    id: concept:page_218_bun
    description: |
      Extracted from documentation: Glob patterns
  - name: "Path normalization"
    id: concept:page_219_bun
    description: |
      Extracted from documentation: Path normalization
  - name: "JavaScript, TypeScript, and JSX"
    id: concept:page_220_bun
    description: |
      Extracted from documentation: JavaScript, TypeScript, and JSX
  - name: "ES Modules & CommonJS"
    id: concept:page_221_bun
    description: |
      Extracted from documentation: ES Modules & CommonJS
  - name: "Referencing local assets in CSS"
    id: concept:page_222_bun
    description: |
      Extracted from documentation: Referencing local assets in CSS
  - name: "Importing CSS in JavaScript"
    id: concept:page_223_bun
    description: |
      Extracted from documentation: Importing CSS in JavaScript
  - name: "Tailwind CSS"
    id: concept:page_224_bun
    description: |
      Extracted from documentation: Tailwind CSS
  - name: "Or any npm client"
    id: concept:page_225_bun
    description: |
      Extracted from documentation: Or any npm client
  - name: "Inline environment variables"
    id: concept:page_226_bun
    description: |
      Extracted from documentation: Inline environment variables
  - name: "Dev server (runtime)"
    id: concept:page_227_bun
    description: |
      Extracted from documentation: Dev server (runtime)
  - name: "Inline all environment variables"
    id: concept:page_228_bun
    description: |
      Extracted from documentation: Inline all environment variables
  - name: "Only inline env vars with a specific prefix (recommended)"
    id: concept:page_229_bun
    description: |
      Extracted from documentation: Only inline env vars with a specific prefix (recommended)
  - name: "Example"
    id: concept:page_230_bun
    description: |
      Extracted from documentation: Example
  - name: "Echo console logs from browser to terminal"
    id: concept:page_231_bun
    description: |
      Extracted from documentation: Echo console logs from browser to terminal
  - name: "Edit files in the browser"
    id: concept:page_232_bun
    description: |
      Extracted from documentation: Edit files in the browser
  - name: "Keyboard Shortcuts"
    id: concept:page_233_bun
    description: |
      Extracted from documentation: Keyboard Shortcuts
  - name: "Build for Production"
    id: concept:page_234_bun
    description: |
      Extracted from documentation: Build for Production
  - name: "Watch Mode"
    id: concept:page_235_bun
    description: |
      Extracted from documentation: Watch Mode
  - name: "What Gets Processed?"
    id: concept:page_236_bun
    description: |
      Extracted from documentation: What Gets Processed?
  - name: "How this works"
    id: concept:page_237_bun
    description: |
      Extracted from documentation: How this works
  - name: "Standalone HTML"
    id: concept:page_238_bun
    description: |
      Extracted from documentation: Standalone HTML
  - name: "Adding a backend to your frontend"
    id: concept:page_239_bun
    description: |
      Extracted from documentation: Adding a backend to your frontend
  - name: "Bundler"
    id: concept:page_240_bun
    description: |
      Extracted from documentation: Bundler
  - name: "At a Glance"
    id: concept:page_241_bun
    description: |
      Extracted from documentation: At a Glance
  - name: "Why bundle?"
    id: concept:page_242_bun
    description: |
      Extracted from documentation: Why bundle?
  - name: "Basic example"
    id: concept:page_243_bun
    description: |
      Extracted from documentation: Basic example
  - name: "Watch mode"
    id: concept:page_244_bun
    description: |
      Extracted from documentation: Watch mode
  - name: "Content types"
    id: concept:page_245_bun
    description: |
      Extracted from documentation: Content types
  - name: "Assets"
    id: concept:page_246_bun
    description: |
      Extracted from documentation: Assets
  - name: "API"
    id: concept:page_247_bun
    description: |
      Extracted from documentation: API
  - name: "entrypoints"
    id: concept:page_248_bun
    description: |
      Extracted from documentation: entrypoints
  - name: "files"
    id: concept:page_249_bun
    description: |
      Extracted from documentation: files
  - name: "Bundle entirely from memory"
    id: concept:page_250_bun
    description: |
      Extracted from documentation: Bundle entirely from memory
  - name: "Override files on disk"
    id: concept:page_251_bun
    description: |
      Extracted from documentation: Override files on disk
  - name: "Mix disk and virtual files"
    id: concept:page_252_bun
    description: |
      Extracted from documentation: Mix disk and virtual files
  - name: "outdir"
    id: concept:page_253_bun
    description: |
      Extracted from documentation: outdir
  - name: "target"
    id: concept:page_254_bun
    description: |
      Extracted from documentation: target
  - name: "format"
    id: concept:page_255_bun
    description: |
      Extracted from documentation: format
  - name: "format: \"esm\" - ES Module"
    id: concept:page_256_bun
    description: |
      Extracted from documentation: format: "esm" - ES Module
  - name: "format: \"cjs\" - CommonJS"
    id: concept:page_257_bun
    description: |
      Extracted from documentation: format: "cjs" - CommonJS
  - name: "format: \"iife\" - IIFE"
    id: concept:page_258_bun
    description: |
      Extracted from documentation: format: "iife" - IIFE
  - name: "`jsx`"
    id: concept:page_259_bun
    description: |
      Extracted from documentation: `jsx`
  - name: "JSX configuration is handled via bunfig.toml or tsconfig.json"
    id: concept:page_260_bun
    description: |
      Extracted from documentation: JSX configuration is handled via bunfig.toml or tsconfig.json
  - name: "splitting"
    id: concept:page_261_bun
    description: |
      Extracted from documentation: splitting
  - name: "plugins"
    id: concept:page_262_bun
    description: |
      Extracted from documentation: plugins
  - name: "env"
    id: concept:page_263_bun
    description: |
      Extracted from documentation: env
  - name: "env: \"inline\""
    id: concept:page_264_bun
    description: |
      Extracted from documentation: env: "inline"
  - name: "env: \"PUBLIC\\_\\*\" (prefix)"
    id: concept:page_265_bun
    description: |
      Extracted from documentation: env: "PUBLIC\_\*" (prefix)
  - name: "env: \"disable\""
    id: concept:page_266_bun
    description: |
      Extracted from documentation: env: "disable"
  - name: "sourcemap"
    id: concept:page_267_bun
    description: |
      Extracted from documentation: sourcemap
  - name: "minify"
    id: concept:page_268_bun
    description: |
      Extracted from documentation: minify
  - name: "external"
    id: concept:page_269_bun
    description: |
      Extracted from documentation: external
  - name: "packages"
    id: concept:page_270_bun
    description: |
      Extracted from documentation: packages
  - name: "naming"
    id: concept:page_271_bun
    description: |
      Extracted from documentation: naming
  - name: "root"
    id: concept:page_272_bun
    description: |
      Extracted from documentation: root
  - name: "publicPath"
    id: concept:page_273_bun
    description: |
      Extracted from documentation: publicPath
  - name: "define"
    id: concept:page_274_bun
    description: |
      Extracted from documentation: define
  - name: "loader"
    id: concept:page_275_bun
    description: |
      Extracted from documentation: loader
  - name: "banner"
    id: concept:page_276_bun
    description: |
      Extracted from documentation: banner
  - name: "footer"
    id: concept:page_277_bun
    description: |
      Extracted from documentation: footer
  - name: "drop"
    id: concept:page_278_bun
    description: |
      Extracted from documentation: drop
  - name: "features"
    id: concept:page_279_bun
    description: |
      Extracted from documentation: features
  - name: "optimizeImports"
    id: concept:page_280_bun
    description: |
      Extracted from documentation: optimizeImports
  - name: "metafile"
    id: concept:page_281_bun
    description: |
      Extracted from documentation: metafile
  - name: "Markdown metafile"
    id: concept:page_282_bun
    description: |
      Extracted from documentation: Markdown metafile
  - name: "`metafile` option formats"
    id: concept:page_283_bun
    description: |
      Extracted from documentation: `metafile` option formats
  - name: "Outputs"
    id: concept:page_284_bun
    description: |
      Extracted from documentation: Outputs
  - name: "Bytecode"
    id: concept:page_285_bun
    description: |
      Extracted from documentation: Bytecode
  - name: "CommonJS bytecode"
    id: concept:page_286_bun
    description: |
      Extracted from documentation: CommonJS bytecode
  - name: "ESM bytecode (requires --compile)"
    id: concept:page_287_bun
    description: |
      Extracted from documentation: ESM bytecode (requires --compile)
  - name: "Executables"
    id: concept:page_288_bun
    description: |
      Extracted from documentation: Executables
  - name: "Logs and errors"
    id: concept:page_289_bun
    description: |
      Extracted from documentation: Logs and errors
  - name: "Reference"
    id: concept:page_290_bun
    description: |
      Extracted from documentation: Reference
  - name: "CLI Usage"
    id: concept:page_291_bun
    description: |
      Extracted from documentation: CLI Usage
  - name: "General Configuration"
    id: concept:page_292_bun
    description: |
      Extracted from documentation: General Configuration
  - name: "Output & File Handling"
    id: concept:page_293_bun
    description: |
      Extracted from documentation: Output & File Handling
  - name: "File Naming"
    id: concept:page_294_bun
    description: |
      Extracted from documentation: File Naming
  - name: "Bundling Options"
    id: concept:page_295_bun
    description: |
      Extracted from documentation: Bundling Options
  - name: "Minification & Optimization"
    id: concept:page_296_bun
    description: |
      Extracted from documentation: Minification & Optimization
  - name: "Development Features"
    id: concept:page_297_bun
    description: |
      Extracted from documentation: Development Features
  - name: "Standalone Executables"
    id: concept:page_298_bun
    description: |
      Extracted from documentation: Standalone Executables
  - name: "Windows Executable Details"
    id: concept:page_299_bun
    description: |
      Extracted from documentation: Windows Executable Details
  - name: "Experimental & App Building"
    id: concept:page_300_bun
    description: |
      Extracted from documentation: Experimental & App Building
  - name: "Loaders"
    id: concept:page_301_bun
    description: |
      Extracted from documentation: Loaders
  - name: "Built-in loaders"
    id: concept:page_302_bun
    description: |
      Extracted from documentation: Built-in loaders
  - name: "`js`"
    id: concept:page_303_bun
    description: |
      Extracted from documentation: `js`
  - name: "`ts`"
    id: concept:page_304_bun
    description: |
      Extracted from documentation: `ts`
  - name: "`tsx`"
    id: concept:page_305_bun
    description: |
      Extracted from documentation: `tsx`
  - name: "`json`"
    id: concept:page_306_bun
    description: |
      Extracted from documentation: `json`
  - name: "`jsonc`"
    id: concept:page_307_bun
    description: |
      Extracted from documentation: `jsonc`
  - name: "`toml`"
    id: concept:page_308_bun
    description: |
      Extracted from documentation: `toml`
  - name: "`yaml`"
    id: concept:page_309_bun
    description: |
      Extracted from documentation: `yaml`
  - name: "`text`"
    id: concept:page_310_bun
    description: |
      Extracted from documentation: `text`
  - name: "`napi`"
    id: concept:page_311_bun
    description: |
      Extracted from documentation: `napi`
  - name: "`sqlite`"
    id: concept:page_312_bun
    description: |
      Extracted from documentation: `sqlite`
  - name: "`html`"
    id: concept:page_313_bun
    description: |
      Extracted from documentation: `html`
  - name: "`css`"
    id: concept:page_314_bun
    description: |
      Extracted from documentation: `css`
  - name: "`sh`"
    id: concept:page_315_bun
    description: |
      Extracted from documentation: `sh`
  - name: "`file`"
    id: concept:page_316_bun
    description: |
      Extracted from documentation: `file`
  - name: "Output: /path/to/project/logo.svg"
    id: concept:page_317_bun
    description: |
      Extracted from documentation: Output: /path/to/project/logo.svg
  - name: "Macros"
    id: concept:page_318_bun
    description: |
      Extracted from documentation: Macros
  - name: "When to use macros"
    id: concept:page_319_bun
    description: |
      Extracted from documentation: When to use macros
  - name: "Import attributes"
    id: concept:page_320_bun
    description: |
      Extracted from documentation: Import attributes
  - name: "Security considerations"
    id: concept:page_321_bun
    description: |
      Extracted from documentation: Security considerations
  - name: "Export condition \"macro\""
    id: concept:page_322_bun
    description: |
      Extracted from documentation: Export condition "macro"
  - name: "Execution"
    id: concept:page_323_bun
    description: |
      Extracted from documentation: Execution
  - name: "Dead code elimination"
    id: concept:page_324_bun
    description: |
      Extracted from documentation: Dead code elimination
  - name: "Serializability"
    id: concept:page_325_bun
    description: |
      Extracted from documentation: Serializability
  - name: "Arguments"
    id: concept:page_326_bun
    description: |
      Extracted from documentation: Arguments
  - name: "Examples"
    id: concept:page_327_bun
    description: |
      Extracted from documentation: Examples
  - name: "Embed latest git commit hash"
    id: concept:page_328_bun
    description: |
      Extracted from documentation: Embed latest git commit hash
  - name: "Make fetch() requests at bundle-time"
    id: concept:page_329_bun
    description: |
      Extracted from documentation: Make fetch() requests at bundle-time
  - name: "Minifier"
    id: concept:page_330_bun
    description: |
      Extracted from documentation: Minifier
  - name: "Enable all minification"
    id: concept:page_331_bun
    description: |
      Extracted from documentation: Enable all minification
  - name: "Production mode"
    id: concept:page_332_bun
    description: |
      Extracted from documentation: Production mode
  - name: "Granular control"
    id: concept:page_333_bun
    description: |
      Extracted from documentation: Granular control
  - name: "Only remove whitespace"
    id: concept:page_334_bun
    description: |
      Extracted from documentation: Only remove whitespace
  - name: "Only minify syntax"
    id: concept:page_335_bun
    description: |
      Extracted from documentation: Only minify syntax
  - name: "Only minify identifiers"
    id: concept:page_336_bun
    description: |
      Extracted from documentation: Only minify identifiers
  - name: "Combine specific modes"
    id: concept:page_337_bun
    description: |
      Extracted from documentation: Combine specific modes
  - name: "Minification Modes"
    id: concept:page_338_bun
    description: |
      Extracted from documentation: Minification Modes
  - name: "Whitespace minification (`--minify-whitespace`)"
    id: concept:page_339_bun
    description: |
      Extracted from documentation: Whitespace minification (`--minify-whitespace`)
  - name: "Syntax minification (`--minify-syntax`)"
    id: concept:page_340_bun
    description: |
      Extracted from documentation: Syntax minification (`--minify-syntax`)
  - name: "Identifier minification (`--minify-identifiers`)"
    id: concept:page_341_bun
    description: |
      Extracted from documentation: Identifier minification (`--minify-identifiers`)
  - name: "All Transformations"
    id: concept:page_342_bun
    description: |
      Extracted from documentation: All Transformations
  - name: "Boolean literal shortening"
    id: concept:page_343_bun
    description: |
      Extracted from documentation: Boolean literal shortening
  - name: "Boolean algebra optimizations"
    id: concept:page_344_bun
    description: |
      Extracted from documentation: Boolean algebra optimizations
  - name: "Undefined shortening"
    id: concept:page_345_bun
    description: |
      Extracted from documentation: Undefined shortening
  - name: "Undefined equality optimization"
    id: concept:page_346_bun
    description: |
      Extracted from documentation: Undefined equality optimization
  - name: "Infinity shortening"
    id: concept:page_347_bun
    description: |
      Extracted from documentation: Infinity shortening
  - name: "Typeof optimizations"
    id: concept:page_348_bun
    description: |
      Extracted from documentation: Typeof optimizations
  - name: "Number formatting"
    id: concept:page_349_bun
    description: |
      Extracted from documentation: Number formatting
  - name: "Arithmetic constant folding"
    id: concept:page_350_bun
    description: |
      Extracted from documentation: Arithmetic constant folding
  - name: "Bitwise constant folding"
    id: concept:page_351_bun
    description: |
      Extracted from documentation: Bitwise constant folding
  - name: "String concatenation"
    id: concept:page_352_bun
    description: |
      Extracted from documentation: String concatenation
  - name: "String indexing"
    id: concept:page_353_bun
    description: |
      Extracted from documentation: String indexing
  - name: "Template literal folding"
    id: concept:page_354_bun
    description: |
      Extracted from documentation: Template literal folding
  - name: "Template literal to string conversion"
    id: concept:page_355_bun
    description: |
      Extracted from documentation: Template literal to string conversion
  - name: "String quote optimization"
    id: concept:page_356_bun
    description: |
      Extracted from documentation: String quote optimization
  - name: "Array spread inlining"
    id: concept:page_357_bun
    description: |
      Extracted from documentation: Array spread inlining
  - name: "Array indexing"
    id: concept:page_358_bun
    description: |
      Extracted from documentation: Array indexing
  - name: "Property access optimization"
    id: concept:page_359_bun
    description: |
      Extracted from documentation: Property access optimization
  - name: "Comparison folding"
    id: concept:page_360_bun
    description: |
      Extracted from documentation: Comparison folding
  - name: "Logical operation folding"
    id: concept:page_361_bun
    description: |
      Extracted from documentation: Logical operation folding
  - name: "Nullish coalescing folding"
    id: concept:page_362_bun
    description: |
      Extracted from documentation: Nullish coalescing folding
  - name: "Comma expression simplification"
    id: concept:page_363_bun
    description: |
      Extracted from documentation: Comma expression simplification
  - name: "Ternary conditional folding"
    id: concept:page_364_bun
    description: |
      Extracted from documentation: Ternary conditional folding
  - name: "Unary expression folding"
    id: concept:page_365_bun
    description: |
      Extracted from documentation: Unary expression folding
  - name: "Double negation removal"
    id: concept:page_366_bun
    description: |
      Extracted from documentation: Double negation removal
  - name: "If statement optimization"
    id: concept:page_367_bun
    description: |
      Extracted from documentation: If statement optimization
  - name: "Unreachable branch removal"
    id: concept:page_368_bun
    description: |
      Extracted from documentation: Unreachable branch removal
  - name: "Empty block removal"
    id: concept:page_369_bun
    description: |
      Extracted from documentation: Empty block removal
  - name: "Single statement block unwrapping"
    id: concept:page_370_bun
    description: |
      Extracted from documentation: Single statement block unwrapping
  - name: "TypeScript enum inlining"
    id: concept:page_371_bun
    description: |
      Extracted from documentation: TypeScript enum inlining
  - name: "Pure annotation support"
    id: concept:page_372_bun
    description: |
      Extracted from documentation: Pure annotation support
  - name: "Identifier renaming"
    id: concept:page_373_bun
    description: |
      Extracted from documentation: Identifier renaming
  - name: "Whitespace removal"
    id: concept:page_374_bun
    description: |
      Extracted from documentation: Whitespace removal
  - name: "Semicolon optimization"
    id: concept:page_375_bun
    description: |
      Extracted from documentation: Semicolon optimization
  - name: "Operator spacing removal"
    id: concept:page_376_bun
    description: |
      Extracted from documentation: Operator spacing removal
  - name: "Comment removal"
    id: concept:page_377_bun
    description: |
      Extracted from documentation: Comment removal
  - name: "Object and array formatting"
    id: concept:page_378_bun
    description: |
      Extracted from documentation: Object and array formatting
  - name: "Control flow formatting"
    id: concept:page_379_bun
    description: |
      Extracted from documentation: Control flow formatting
  - name: "Function formatting"
    id: concept:page_380_bun
    description: |
      Extracted from documentation: Function formatting
  - name: "Parentheses minimization"
    id: concept:page_381_bun
    description: |
      Extracted from documentation: Parentheses minimization
  - name: "Template literal value folding"
    id: concept:page_382_bun
    description: |
      Extracted from documentation: Template literal value folding
  - name: "String length constant folding"
    id: concept:page_383_bun
    description: |
      Extracted from documentation: String length constant folding
  - name: "Constructor call simplification"
    id: concept:page_384_bun
    description: |
      Extracted from documentation: Constructor call simplification
  - name: "Single property object inlining"
    id: concept:page_385_bun
    description: |
      Extracted from documentation: Single property object inlining
  - name: "String charCodeAt constant folding"
    id: concept:page_386_bun
    description: |
      Extracted from documentation: String charCodeAt constant folding
  - name: "Void 0 equality to null equality"
    id: concept:page_387_bun
    description: |
      Extracted from documentation: Void 0 equality to null equality
  - name: "Negation operator optimization"
    id: concept:page_388_bun
    description: |
      Extracted from documentation: Negation operator optimization
  - name: "Import.meta property inlining"
    id: concept:page_389_bun
    description: |
      Extracted from documentation: Import.meta property inlining
  - name: "Variable declaration merging"
    id: concept:page_390_bun
    description: |
      Extracted from documentation: Variable declaration merging
  - name: "Expression statement merging"
    id: concept:page_391_bun
    description: |
      Extracted from documentation: Expression statement merging
  - name: "Return statement merging"
    id: concept:page_392_bun
    description: |
      Extracted from documentation: Return statement merging
  - name: "Throw statement merging"
    id: concept:page_393_bun
    description: |
      Extracted from documentation: Throw statement merging
  - name: "TypeScript enum cross-module inlining"
    id: concept:page_394_bun
    description: |
      Extracted from documentation: TypeScript enum cross-module inlining
  - name: "Computed property enum inlining"
    id: concept:page_395_bun
    description: |
      Extracted from documentation: Computed property enum inlining
  - name: "Arrow function body shortening"
    id: concept:page_396_bun
    description: |
      Extracted from documentation: Arrow function body shortening
  - name: "Object property shorthand"
    id: concept:page_397_bun
    description: |
      Extracted from documentation: Object property shorthand
  - name: "Drop debugger statements"
    id: concept:page_398_bun
    description: |
      Extracted from documentation: Drop debugger statements
  - name: "Drop console calls"
    id: concept:page_399_bun
    description: |
      Extracted from documentation: Drop console calls
  - name: "Drop custom function calls"
    id: concept:page_400_bun
    description: |
      Extracted from documentation: Drop custom function calls
  - name: "Keep Names"
    id: concept:page_401_bun
    description: |
      Extracted from documentation: Keep Names
  - name: "Combined Example"
    id: concept:page_402_bun
    description: |
      Extracted from documentation: Combined Example
  - name: "When to Use Minification"
    id: concept:page_403_bun
    description: |
      Extracted from documentation: When to Use Minification
  - name: "Lifecycle hooks"
    id: concept:page_404_bun
    description: |
      Extracted from documentation: Lifecycle hooks
  - name: "Plugin lifecycle"
    id: concept:page_405_bun
    description: |
      Extracted from documentation: Plugin lifecycle
  - name: "Namespaces"
    id: concept:page_406_bun
    description: |
      Extracted from documentation: Namespaces
  - name: "onStart"
    id: concept:page_407_bun
    description: |
      Extracted from documentation: onStart
  - name: ".defer()"
    id: concept:page_408_bun
    description: |
      Extracted from documentation: .defer()
  - name: "Native plugins"
    id: concept:page_409_bun
    description: |
      Extracted from documentation: Native plugins
  - name: "Creating a native plugin in Rust"
    id: concept:page_410_bun
    description: |
      Extracted from documentation: Creating a native plugin in Rust
  - name: "[bun]"
    id: concept:page_411_bun
    description: |
      Extracted from documentation: [bun]
  - name: "onBeforeParse"
    id: concept:page_412_bun
    description: |
      Extracted from documentation: onBeforeParse
  - name: "onEnd"
    id: concept:page_413_bun
    description: |
      Extracted from documentation: onEnd
  - name: "One file. Upload anywhere."
    id: concept:page_414_bun
    description: |
      Extracted from documentation: One file. Upload anywhere.
  - name: "Truly one file"
    id: concept:page_415_bun
    description: |
      Extracted from documentation: Truly one file
  - name: "Quick start"
    id: concept:page_416_bun
    description: |
      Extracted from documentation: Quick start
  - name: "Everything gets inlined"
    id: concept:page_417_bun
    description: |
      Extracted from documentation: Everything gets inlined
  - name: "What gets inlined"
    id: concept:page_418_bun
    description: |
      Extracted from documentation: What gets inlined
  - name: "Using with React"
    id: concept:page_419_bun
    description: |
      Extracted from documentation: Using with React
  - name: "Using with Tailwind CSS"
    id: concept:page_420_bun
    description: |
      Extracted from documentation: Using with Tailwind CSS
  - name: "Multiple HTML files"
    id: concept:page_421_bun
    description: |
      Extracted from documentation: Multiple HTML files
  - name: "Environment variables"
    id: concept:page_422_bun
    description: |
      Extracted from documentation: Environment variables
  - name: "Feedback"
    id: concept:page_423_bun
    description: |
      Extracted from documentation: Feedback
  - name: "Reporting Issues"
    id: concept:page_424_bun
    description: |
      Extracted from documentation: Reporting Issues
  - name: "To revert to the stable release"
    id: concept:page_425_bun
    description: |
      Extracted from documentation: To revert to the stable release
  - name: "Use `bun feedback`"
    id: concept:page_426_bun
    description: |
      Extracted from documentation: Use `bun feedback`
  - name: "Convert an ArrayBuffer to an array of numbers"
    id: concept:page_427_bun
    description: |
      Extracted from documentation: Convert an ArrayBuffer to an array of numbers
  - name: "Convert an ArrayBuffer to a Blob"
    id: concept:page_428_bun
    description: |
      Extracted from documentation: Convert an ArrayBuffer to a Blob
  - name: "Convert an ArrayBuffer to a Buffer"
    id: concept:page_429_bun
    description: |
      Extracted from documentation: Convert an ArrayBuffer to a Buffer
  - name: "Convert an ArrayBuffer to a string"
    id: concept:page_430_bun
    description: |
      Extracted from documentation: Convert an ArrayBuffer to a string
  - name: "Convert an ArrayBuffer to a Uint8Array"
    id: concept:page_431_bun
    description: |
      Extracted from documentation: Convert an ArrayBuffer to a Uint8Array
  - name: "Convert a Blob to an ArrayBuffer"
    id: concept:page_432_bun
    description: |
      Extracted from documentation: Convert a Blob to an ArrayBuffer
  - name: "Convert a Blob to a DataView"
    id: concept:page_433_bun
    description: |
      Extracted from documentation: Convert a Blob to a DataView
  - name: "Convert a Blob to a ReadableStream"
    id: concept:page_434_bun
    description: |
      Extracted from documentation: Convert a Blob to a ReadableStream
  - name: "Convert a Blob to a string"
    id: concept:page_435_bun
    description: |
      Extracted from documentation: Convert a Blob to a string
  - name: "Convert a Blob to a Uint8Array"
    id: concept:page_436_bun
    description: |
      Extracted from documentation: Convert a Blob to a Uint8Array
  - name: "Convert a Buffer to an ArrayBuffer"
    id: concept:page_437_bun
    description: |
      Extracted from documentation: Convert a Buffer to an ArrayBuffer
  - name: "Convert a Buffer to a blob"
    id: concept:page_438_bun
    description: |
      Extracted from documentation: Convert a Buffer to a blob
  - name: "Convert a Buffer to a ReadableStream"
    id: concept:page_439_bun
    description: |
      Extracted from documentation: Convert a Buffer to a ReadableStream
  - name: "Convert a Buffer to a string"
    id: concept:page_440_bun
    description: |
      Extracted from documentation: Convert a Buffer to a string
  - name: "Convert a Buffer to a Uint8Array"
    id: concept:page_441_bun
    description: |
      Extracted from documentation: Convert a Buffer to a Uint8Array
  - name: "Convert a DataView to a string"
    id: concept:page_442_bun
    description: |
      Extracted from documentation: Convert a DataView to a string
  - name: "Convert a Uint8Array to an ArrayBuffer"
    id: concept:page_443_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to an ArrayBuffer
  - name: "Convert a Uint8Array to a Blob"
    id: concept:page_444_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to a Blob
  - name: "Convert a Uint8Array to a Buffer"
    id: concept:page_445_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to a Buffer
  - name: "Convert a Uint8Array to a DataView"
    id: concept:page_446_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to a DataView
  - name: "Convert a Uint8Array to a ReadableStream"
    id: concept:page_447_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to a ReadableStream
  - name: "Convert a Uint8Array to a string"
    id: concept:page_448_bun
    description: |
      Extracted from documentation: Convert a Uint8Array to a string
  - name: "Deploy a Bun application on AWS Lambda"
    id: concept:page_449_bun
    description: |
      Extracted from documentation: Deploy a Bun application on AWS Lambda
  - name: "Use the official AWS Lambda adapter image to handle the Lambda runtime"
    id: concept:page_450_bun
    description: |
      Extracted from documentation: Use the official AWS Lambda adapter image to handle the Lambda runtime
  - name: "Use the official Bun image to run the application"
    id: concept:page_451_bun
    description: |
      Extracted from documentation: Use the official Bun image to run the application
  - name: "Copy the Lambda adapter into the container"
    id: concept:page_452_bun
    description: |
      Extracted from documentation: Copy the Lambda adapter into the container
  - name: "Set the port to 8080. This is required for the AWS Lambda adapter."
    id: concept:page_453_bun
    description: |
      Extracted from documentation: Set the port to 8080. This is required for the AWS Lambda adapter.
  - name: "Set the work directory to `/var/task`. This is the default work directory for Lambda."
    id: concept:page_454_bun
    description: |
      Extracted from documentation: Set the work directory to `/var/task`. This is the default work directory for Lambda.
  - name: "Copy the package.json and bun.lock into the container"
    id: concept:page_455_bun
    description: |
      Extracted from documentation: Copy the package.json and bun.lock into the container
  - name: "Install the dependencies"
    id: concept:page_456_bun
    description: |
      Extracted from documentation: Install the dependencies
  - name: "Copy the rest of the application into the container"
    id: concept:page_457_bun
    description: |
      Extracted from documentation: Copy the rest of the application into the container
  - name: "Run the application."
    id: concept:page_458_bun
    description: |
      Extracted from documentation: Run the application.
  - name: "Any other files or directories you want to exclude"
    id: concept:page_459_bun
    description: |
      Extracted from documentation: Any other files or directories you want to exclude
  - name: "cd /path/to/your/app"
    id: concept:page_460_bun
    description: |
      Extracted from documentation: cd /path/to/your/app
  - name: "Deploy a Bun application on DigitalOcean"
    id: concept:page_461_bun
    description: |
      Extracted from documentation: Deploy a Bun application on DigitalOcean
  - name: "Set the work directory to `/app`"
    id: concept:page_462_bun
    description: |
      Extracted from documentation: Set the work directory to `/app`
  - name: "Expose the port (DigitalOcean will set PORT env var)"
    id: concept:page_463_bun
    description: |
      Extracted from documentation: Expose the port (DigitalOcean will set PORT env var)
  - name: "Run the application"
    id: concept:page_464_bun
    description: |
      Extracted from documentation: Run the application
  - name: "Deploy a Bun application on Google Cloud Run"
    id: concept:page_465_bun
    description: |
      Extracted from documentation: Deploy a Bun application on Google Cloud Run
  - name: "Deploy a Bun application on Railway"
    id: concept:page_466_bun
    description: |
      Extracted from documentation: Deploy a Bun application on Railway
  - name: "Method 1: Deploy via CLI"
    id: concept:page_467_bun
    description: |
      Extracted from documentation: Method 1: Deploy via CLI
  - name: "Add PostgreSQL database. Make sure to add this first!"
    id: concept:page_468_bun
    description: |
      Extracted from documentation: Add PostgreSQL database. Make sure to add this first!
  - name: "Add your application service."
    id: concept:page_469_bun
    description: |
      Extracted from documentation: Add your application service.
  - name: "Deploy your application"
    id: concept:page_470_bun
    description: |
      Extracted from documentation: Deploy your application
  - name: "Generate public domain"
    id: concept:page_471_bun
    description: |
      Extracted from documentation: Generate public domain
  - name: "Method 2: Deploy via Dashboard"
    id: concept:page_472_bun
    description: |
      Extracted from documentation: Method 2: Deploy via Dashboard
  - name: "Configuration (Optional)"
    id: concept:page_473_bun
    description: |
      Extracted from documentation: Configuration (Optional)
  - name: "Deploy a Bun application on Render"
    id: concept:page_474_bun
    description: |
      Extracted from documentation: Deploy a Bun application on Render
  - name: "Deploy a Bun application on Vercel"
    id: concept:page_475_bun
    description: |
      Extracted from documentation: Deploy a Bun application on Vercel
  - name: "Using bunx (no global install)"
    id: concept:page_476_bun
    description: |
      Extracted from documentation: Using bunx (no global install)
  - name: "Build an app with Astro and Bun"
    id: concept:page_477_bun
    description: |
      Extracted from documentation: Build an app with Astro and Bun
  - name: "Create a Discord bot"
    id: concept:page_478_bun
    description: |
      Extracted from documentation: Create a Discord bot
  - name: "Containerize a Bun application with Docker"
    id: concept:page_479_bun
    description: |
      Extracted from documentation: Containerize a Bun application with Docker
  - name: "use the official Bun image"
    id: concept:page_480_bun
    description: |
      Extracted from documentation: use the official Bun image
  - name: "see all versions at https://hub.docker.com/r/oven/bun/tags"
    id: concept:page_481_bun
    description: |
      Extracted from documentation: see all versions at https://hub.docker.com/r/oven/bun/tags
  - name: "install dependencies into temp directory"
    id: concept:page_482_bun
    description: |
      Extracted from documentation: install dependencies into temp directory
  - name: "this will cache them and speed up future builds"
    id: concept:page_483_bun
    description: |
      Extracted from documentation: this will cache them and speed up future builds
  - name: "install with --production (exclude devDependencies)"
    id: concept:page_484_bun
    description: |
      Extracted from documentation: install with --production (exclude devDependencies)
  - name: "copy node_modules from temp directory"
    id: concept:page_485_bun
    description: |
      Extracted from documentation: copy node_modules from temp directory
  - name: "then copy all (non-ignored) project files into the image"
    id: concept:page_486_bun
    description: |
      Extracted from documentation: then copy all (non-ignored) project files into the image
  - name: "[optional] tests & build"
    id: concept:page_487_bun
    description: |
      Extracted from documentation: [optional] tests & build
  - name: "copy production dependencies and source code into final image"
    id: concept:page_488_bun
    description: |
      Extracted from documentation: copy production dependencies and source code into final image
  - name: "run the app"
    id: concept:page_489_bun
    description: |
      Extracted from documentation: run the app
  - name: "...lots of commands..."
    id: concept:page_490_bun
    description: |
      Extracted from documentation: ...lots of commands...
  - name: "Use Drizzle ORM with Bun"
    id: concept:page_491_bun
    description: |
      Extracted from documentation: Use Drizzle ORM with Bun
  - name: "Build an HTTP server using Elysia and Bun"
    id: concept:page_492_bun
    description: |
      Extracted from documentation: Build an HTTP server using Elysia and Bun
  - name: "Build an HTTP server using Express and Bun"
    id: concept:page_493_bun
    description: |
      Extracted from documentation: Build an HTTP server using Express and Bun
  - name: "Use Gel with Bun"
    id: concept:page_494_bun
    description: |
      Extracted from documentation: Use Gel with Bun
  - name: "Build an HTTP server using Hono and Bun"
    id: concept:page_495_bun
    description: |
      Extracted from documentation: Build an HTTP server using Hono and Bun
  - name: "Read and write data to MongoDB using Mongoose and Bun"
    id: concept:page_496_bun
    description: |
      Extracted from documentation: Read and write data to MongoDB using Mongoose and Bun
  - name: "Use Neon Postgres through Drizzle ORM"
    id: concept:page_497_bun
    description: |
      Extracted from documentation: Use Neon Postgres through Drizzle ORM
  - name: "Use Neon's Serverless Postgres with Bun"
    id: concept:page_498_bun
    description: |
      Extracted from documentation: Use Neon's Serverless Postgres with Bun
  - name: "Build an app with Next.js and Bun"
    id: concept:page_499_bun
    description: |
      Extracted from documentation: Build an app with Next.js and Bun
  - name: "Hosting"
    id: concept:page_500_bun
    description: |
      Extracted from documentation: Hosting
  - name: "Templates"
    id: concept:page_501_bun
    description: |
      Extracted from documentation: Templates
  - name: "Build an app with Nuxt and Bun"
    id: concept:page_502_bun
    description: |
      Extracted from documentation: Build an app with Nuxt and Bun
  - name: "Run Bun as a daemon with PM2"
    id: concept:page_503_bun
    description: |
      Extracted from documentation: Run Bun as a daemon with PM2
  - name: "With `--interpreter`"
    id: concept:page_504_bun
    description: |
      Extracted from documentation: With `--interpreter`
  - name: "With a configuration file"
    id: concept:page_505_bun
    description: |
      Extracted from documentation: With a configuration file
  - name: "Use Prisma with Bun"
    id: concept:page_506_bun
    description: |
      Extracted from documentation: Use Prisma with Bun
  - name: "Use Prisma Postgres with Bun"
    id: concept:page_507_bun
    description: |
      Extracted from documentation: Use Prisma Postgres with Bun
  - name: "Build an app with Qwik and Bun"
    id: concept:page_508_bun
    description: |
      Extracted from documentation: Build an app with Qwik and Bun
  - name: "Build a React app with Bun"
    id: concept:page_509_bun
    description: |
      Extracted from documentation: Build a React app with Bun
  - name: "Create a new React app"
    id: concept:page_510_bun
    description: |
      Extracted from documentation: Create a new React app
  - name: "Run the app in development mode"
    id: concept:page_511_bun
    description: |
      Extracted from documentation: Run the app in development mode
  - name: "Build as a static site for production"
    id: concept:page_512_bun
    description: |
      Extracted from documentation: Build as a static site for production
  - name: "Run the server in production"
    id: concept:page_513_bun
    description: |
      Extracted from documentation: Run the server in production
  - name: "Hot Reloading"
    id: concept:page_514_bun
    description: |
      Extracted from documentation: Hot Reloading
  - name: "Full-Stack App"
    id: concept:page_515_bun
    description: |
      Extracted from documentation: Full-Stack App
  - name: "Static Site"
    id: concept:page_516_bun
    description: |
      Extracted from documentation: Static Site
  - name: "Build an app with Remix and Bun"
    id: concept:page_517_bun
    description: |
      Extracted from documentation: Build an app with Remix and Bun
  - name: "Add Sentry to a Bun app"
    id: concept:page_518_bun
    description: |
      Extracted from documentation: Add Sentry to a Bun app
  - name: "Build an app with SolidStart and Bun"
    id: concept:page_519_bun
    description: |
      Extracted from documentation: Build an app with SolidStart and Bun
  - name: "Server-side render (SSR) a React component"
    id: concept:page_520_bun
    description: |
      Extracted from documentation: Server-side render (SSR) a React component
  - name: "Any package manager can be used"
    id: concept:page_521_bun
    description: |
      Extracted from documentation: Any package manager can be used
  - name: "Build an HTTP server using StricJS and Bun"
    id: concept:page_522_bun
    description: |
      Extracted from documentation: Build an HTTP server using StricJS and Bun
  - name: "Build an app with SvelteKit and Bun"
    id: concept:page_523_bun
    description: |
      Extracted from documentation: Build an app with SvelteKit and Bun
  - name: "Run Bun as a daemon with systemd"
    id: concept:page_524_bun
    description: |
      Extracted from documentation: Run Bun as a daemon with systemd
  - name: "describe the app"
    id: concept:page_525_bun
    description: |
      Extracted from documentation: describe the app
  - name: "start the app after the network is available"
    id: concept:page_526_bun
    description: |
      Extracted from documentation: start the app after the network is available
  - name: "usually you'll use 'simple'"
    id: concept:page_527_bun
    description: |
      Extracted from documentation: usually you'll use 'simple'
  - name: "one of https://www.freedesktop.org/software/systemd/man/systemd.service.html#Type="
    id: concept:page_528_bun
    description: |
      Extracted from documentation: one of https://www.freedesktop.org/software/systemd/man/systemd.service.html#Type=
  - name: "which user to use when starting the app"
    id: concept:page_529_bun
    description: |
      Extracted from documentation: which user to use when starting the app
  - name: "path to your application's root directory"
    id: concept:page_530_bun
    description: |
      Extracted from documentation: path to your application's root directory
  - name: "the command to start the app"
    id: concept:page_531_bun
    description: |
      Extracted from documentation: the command to start the app
  - name: "requires absolute paths"
    id: concept:page_532_bun
    description: |
      Extracted from documentation: requires absolute paths
  - name: "restart policy"
    id: concept:page_533_bun
    description: |
      Extracted from documentation: restart policy
  - name: "one of {no|on-success|on-failure|on-abnormal|on-watchdog|on-abort|always}"
    id: concept:page_534_bun
    description: |
      Extracted from documentation: one of {no|on-success|on-failure|on-abnormal|on-watchdog|on-abort|always}
  - name: "start the app automatically"
    id: concept:page_535_bun
    description: |
      Extracted from documentation: start the app automatically
  - name: "Use TanStack Start with Bun"
    id: concept:page_536_bun
    description: |
      Extracted from documentation: Use TanStack Start with Bun
  - name: "Bun Redis with Upstash"
    id: concept:page_537_bun
    description: |
      Extracted from documentation: Bun Redis with Upstash
  - name: "Build a frontend using Vite and Bun"
    id: concept:page_538_bun
    description: |
      Extracted from documentation: Build a frontend using Vite and Bun
  - name: "Extract links from a webpage using HTMLRewriter"
    id: concept:page_539_bun
    description: |
      Extracted from documentation: Extract links from a webpage using HTMLRewriter
  - name: "Extract links from a webpage"
    id: concept:page_540_bun
    description: |
      Extracted from documentation: Extract links from a webpage
  - name: "Convert relative URLs to absolute"
    id: concept:page_541_bun
    description: |
      Extracted from documentation: Convert relative URLs to absolute
  - name: "Extract social share images and Open Graph tags"
    id: concept:page_542_bun
    description: |
      Extracted from documentation: Extract social share images and Open Graph tags
  - name: "Start a cluster of HTTP servers"
    id: concept:page_543_bun
    description: |
      Extracted from documentation: Start a cluster of HTTP servers
  - name: "Send an HTTP request using fetch"
    id: concept:page_544_bun
    description: |
      Extracted from documentation: Send an HTTP request using fetch
  - name: "fetch with unix domain sockets in Bun"
    id: concept:page_545_bun
    description: |
      Extracted from documentation: fetch with unix domain sockets in Bun
  - name: "Upload files via HTTP using FormData"
    id: concept:page_546_bun
    description: |
      Extracted from documentation: Upload files via HTTP using FormData
  - name: "Hot reload an HTTP server"
    id: concept:page_547_bun
    description: |
      Extracted from documentation: Hot reload an HTTP server
  - name: "Proxy HTTP requests using fetch()"
    id: concept:page_548_bun
    description: |
      Extracted from documentation: Proxy HTTP requests using fetch()
  - name: "Custom proxy headers"
    id: concept:page_549_bun
    description: |
      Extracted from documentation: Custom proxy headers
  - name: "Common HTTP server usage"
    id: concept:page_550_bun
    description: |
      Extracted from documentation: Common HTTP server usage
  - name: "Write a simple HTTP server"
    id: concept:page_551_bun
    description: |
      Extracted from documentation: Write a simple HTTP server
  - name: "Server-Sent Events (SSE) with Bun"
    id: concept:page_552_bun
    description: |
      Extracted from documentation: Server-Sent Events (SSE) with Bun
  - name: "Using an async generator"
    id: concept:page_553_bun
    description: |
      Extracted from documentation: Using an async generator
  - name: "Using a `ReadableStream`"
    id: concept:page_554_bun
    description: |
      Extracted from documentation: Using a `ReadableStream`
  - name: "Stream a file as an HTTP Response"
    id: concept:page_555_bun
    description: |
      Extracted from documentation: Stream a file as an HTTP Response
  - name: "Streaming HTTP Server with Async Iterators"
    id: concept:page_556_bun
    description: |
      Extracted from documentation: Streaming HTTP Server with Async Iterators
  - name: "Streaming HTTP Server with Node.js Streams"
    id: concept:page_557_bun
    description: |
      Extracted from documentation: Streaming HTTP Server with Node.js Streams
  - name: "Configure TLS on an HTTP server"
    id: concept:page_558_bun
    description: |
      Extracted from documentation: Configure TLS on an HTTP server
  - name: "Guides"
    id: concept:page_559_bun
    description: |
      Extracted from documentation: Guides
  - name: "Add a dependency"
    id: concept:page_560_bun
    description: |
      Extracted from documentation: Add a dependency
  - name: "Add a development dependency"
    id: concept:page_561_bun
    description: |
      Extracted from documentation: Add a development dependency
  - name: "Add a Git dependency"
    id: concept:page_562_bun
    description: |
      Extracted from documentation: Add a Git dependency
  - name: "Add an optional dependency"
    id: concept:page_563_bun
    description: |
      Extracted from documentation: Add an optional dependency
  - name: "Add a peer dependency"
    id: concept:page_564_bun
    description: |
      Extracted from documentation: Add a peer dependency
  - name: "Add a tarball dependency"
    id: concept:page_565_bun
    description: |
      Extracted from documentation: Add a tarball dependency
  - name: "Using bun install with an Azure Artifacts npm registry"
    id: concept:page_566_bun
    description: |
      Extracted from documentation: Using bun install with an Azure Artifacts npm registry
  - name: "Configure with bunfig.toml"
    id: concept:page_567_bun
    description: |
      Extracted from documentation: Configure with bunfig.toml
  - name: "You can use an environment variable here"
    id: concept:page_568_bun
    description: |
      Extracted from documentation: You can use an environment variable here
  - name: "Configure with environment variables"
    id: concept:page_569_bun
    description: |
      Extracted from documentation: Configure with environment variables
  - name: "Don't base64 encode the password"
    id: concept:page_570_bun
    description: |
      Extracted from documentation: Don't base64 encode the password
  - name: "Install dependencies with Bun in GitHub Actions"
    id: concept:page_571_bun
    description: |
      Extracted from documentation: Install dependencies with Bun in GitHub Actions
  - name: "..."
    id: concept:page_572_bun
    description: |
      Extracted from documentation: ...
  - name: "run any `bun` or `bunx` command"
    id: concept:page_573_bun
    description: |
      Extracted from documentation: run any `bun` or `bunx` command
  - name: "Override the default npm registry for bun install"
    id: concept:page_574_bun
    description: |
      Extracted from documentation: Override the default npm registry for bun install
  - name: "set default registry as a string"
    id: concept:page_575_bun
    description: |
      Extracted from documentation: set default registry as a string
  - name: "if needed, set a token"
    id: concept:page_576_bun
    description: |
      Extracted from documentation: if needed, set a token
  - name: "if needed, set a username/password"
    id: concept:page_577_bun
    description: |
      Extracted from documentation: if needed, set a username/password
  - name: "Migrate from npm install to bun install"
    id: concept:page_578_bun
    description: |
      Extracted from documentation: Migrate from npm install to bun install
  - name: "It only takes one command to migrate"
    id: concept:page_579_bun
    description: |
      Extracted from documentation: It only takes one command to migrate
  - name: "To add dependencies:"
    id: concept:page_580_bun
    description: |
      Extracted from documentation: To add dependencies:
  - name: "To add devDependencies:"
    id: concept:page_581_bun
    description: |
      Extracted from documentation: To add devDependencies:
  - name: "To remove a dependency:"
    id: concept:page_582_bun
    description: |
      Extracted from documentation: To remove a dependency:
  - name: "Run package.json scripts faster"
    id: concept:page_583_bun
    description: |
      Extracted from documentation: Run package.json scripts faster
  - name: "Run a package.json script:"
    id: concept:page_584_bun
    description: |
      Extracted from documentation: Run a package.json script:
  - name: "Run an executable in node_modules/.bin:"
    id: concept:page_585_bun
    description: |
      Extracted from documentation: Run an executable in node_modules/.bin:
  - name: "Run a JavaScript/TypeScript file:"
    id: concept:page_586_bun
    description: |
      Extracted from documentation: Run a JavaScript/TypeScript file:
  - name: "Workspaces? Yes."
    id: concept:page_587_bun
    description: |
      Extracted from documentation: Workspaces? Yes.
  - name: "Filter scripts by workspace name"
    id: concept:page_588_bun
    description: |
      Extracted from documentation: Filter scripts by workspace name
  - name: "instead of:"
    id: concept:page_589_bun
    description: |
      Extracted from documentation: instead of:
  - name: "npm run --workspace lib-foo --workspace lib-bar my-script"
    id: concept:page_590_bun
    description: |
      Extracted from documentation: npm run --workspace lib-foo --workspace lib-bar my-script
  - name: "Update dependencies"
    id: concept:page_591_bun
    description: |
      Extracted from documentation: Update dependencies
  - name: "Update a single dependency"
    id: concept:page_592_bun
    description: |
      Extracted from documentation: Update a single dependency
  - name: "Update all dependencies"
    id: concept:page_593_bun
    description: |
      Extracted from documentation: Update all dependencies
  - name: "Ignore semver, update to the latest version"
    id: concept:page_594_bun
    description: |
      Extracted from documentation: Ignore semver, update to the latest version
  - name: "Update a dependency to a specific version"
    id: concept:page_595_bun
    description: |
      Extracted from documentation: Update a dependency to a specific version
  - name: "Update all dependencies to the latest versions"
    id: concept:page_596_bun
    description: |
      Extracted from documentation: Update all dependencies to the latest versions
  - name: "View outdated dependencies"
    id: concept:page_597_bun
    description: |
      Extracted from documentation: View outdated dependencies
  - name: "List installed packages"
    id: concept:page_598_bun
    description: |
      Extracted from documentation: List installed packages
  - name: "List top-level installed packages:"
    id: concept:page_599_bun
    description: |
      Extracted from documentation: List top-level installed packages:
  - name: "List all installed packages:"
    id: concept:page_600_bun
    description: |
      Extracted from documentation: List all installed packages:
  - name: "Create a package tarball"
    id: concept:page_601_bun
    description: |
      Extracted from documentation: Create a package tarball
  - name: "Create a tarball"
    id: concept:page_602_bun
    description: |
      Extracted from documentation: Create a tarball
  - name: "Shebang"
    id: concept:page_603_bun
    description: |
      Extracted from documentation: Shebang
  - name: "Force using Bun's runtime instead of node"
    id: concept:page_604_bun
    description: |
      Extracted from documentation: Force using Bun's runtime instead of node
  - name: "This also works:"
    id: concept:page_605_bun
    description: |
      Extracted from documentation: This also works:
  - name: "Global installs"
    id: concept:page_606_bun
    description: |
      Extracted from documentation: Global installs
  - name: "Install a package globally"
    id: concept:page_607_bun
    description: |
      Extracted from documentation: Install a package globally
  - name: "Run a globally-installed package without the `bun run` prefix"
    id: concept:page_608_bun
    description: |
      Extracted from documentation: Run a globally-installed package without the `bun run` prefix
  - name: "Configure git to diff Bun's lockb lockfile"
    id: concept:page_609_bun
    description: |
      Extracted from documentation: Configure git to diff Bun's lockb lockfile
  - name: "Using bun install with Artifactory"
    id: concept:page_610_bun
    description: |
      Extracted from documentation: Using bun install with Artifactory
  - name: "url = \"$NPM_CONFIG_REGISTRY\""
    id: concept:page_611_bun
    description: |
      Extracted from documentation: url = "$NPM_CONFIG_REGISTRY"
  - name: "Configure with `$NPM_CONFIG_REGISTRY`"
    id: concept:page_612_bun
    description: |
      Extracted from documentation: Configure with `$NPM_CONFIG_REGISTRY`
  - name: "Install a package under a different name"
    id: concept:page_613_bun
    description: |
      Extracted from documentation: Install a package under a different name
  - name: "Configure a private registry for an organization scope with bun install"
    id: concept:page_614_bun
    description: |
      Extracted from documentation: Configure a private registry for an organization scope with bun install
  - name: "as a string"
    id: concept:page_615_bun
    description: |
      Extracted from documentation: as a string
  - name: "as an object with username/password"
    id: concept:page_616_bun
    description: |
      Extracted from documentation: as an object with username/password
  - name: "you can reference environment variables"
    id: concept:page_617_bun
    description: |
      Extracted from documentation: you can reference environment variables
  - name: "as an object with token"
    id: concept:page_618_bun
    description: |
      Extracted from documentation: as an object with token
  - name: "Add a trusted dependency"
    id: concept:page_619_bun
    description: |
      Extracted from documentation: Add a trusted dependency
  - name: "Configuring a monorepo using workspaces"
    id: concept:page_620_bun
    description: |
      Extracted from documentation: Configuring a monorepo using workspaces
  - name: "Generate a yarn-compatible lockfile"
    id: concept:page_621_bun
    description: |
      Extracted from documentation: Generate a yarn-compatible lockfile
  - name: "THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY."
    id: concept:page_622_bun
    description: |
      Extracted from documentation: THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.
  - name: "yarn lockfile v1"
    id: concept:page_623_bun
    description: |
      Extracted from documentation: yarn lockfile v1
  - name: "bun ./bun.lockb --hash: 9BFBF11D86084AAB-9418b03ff880c569-390CE6459EACEC9A..."
    id: concept:page_624_bun
    description: |
      Extracted from documentation: bun ./bun.lockb --hash: 9BFBF11D86084AAB-9418b03ff880c569-390CE6459EACEC9A...
  - name: "Parse command-line arguments"
    id: concept:page_625_bun
    description: |
      Extracted from documentation: Parse command-line arguments
  - name: "Listen for CTRL+C"
    id: concept:page_626_bun
    description: |
      Extracted from documentation: Listen for CTRL+C
  - name: "Spawn a child process and communicate using IPC"
    id: concept:page_627_bun
    description: |
      Extracted from documentation: Spawn a child process and communicate using IPC
  - name: "Get the process uptime in nanoseconds"
    id: concept:page_628_bun
    description: |
      Extracted from documentation: Get the process uptime in nanoseconds
  - name: "Listen to OS signals"
    id: concept:page_629_bun
    description: |
      Extracted from documentation: Listen to OS signals
  - name: "Spawn a child process"
    id: concept:page_630_bun
    description: |
      Extracted from documentation: Spawn a child process
  - name: "Read stderr from a child process"
    id: concept:page_631_bun
    description: |
      Extracted from documentation: Read stderr from a child process
  - name: "Read stdout from a child process"
    id: concept:page_632_bun
    description: |
      Extracted from documentation: Read stdout from a child process
  - name: "Read from stdin"
    id: concept:page_633_bun
    description: |
      Extracted from documentation: Read from stdin
  - name: "Read a file to an ArrayBuffer"
    id: concept:page_634_bun
    description: |
      Extracted from documentation: Read a file to an ArrayBuffer
  - name: "Read a file to a Buffer"
    id: concept:page_635_bun
    description: |
      Extracted from documentation: Read a file to a Buffer
  - name: "Check if a file exists"
    id: concept:page_636_bun
    description: |
      Extracted from documentation: Check if a file exists
  - name: "Read a JSON file"
    id: concept:page_637_bun
    description: |
      Extracted from documentation: Read a JSON file
  - name: "Get the MIME type of a file"
    id: concept:page_638_bun
    description: |
      Extracted from documentation: Get the MIME type of a file
  - name: "Read a file as a ReadableStream"
    id: concept:page_639_bun
    description: |
      Extracted from documentation: Read a file as a ReadableStream
  - name: "Read a file as a string"
    id: concept:page_640_bun
    description: |
      Extracted from documentation: Read a file as a string
  - name: "Read a file to a Uint8Array"
    id: concept:page_641_bun
    description: |
      Extracted from documentation: Read a file to a Uint8Array
  - name: "Watch a directory for changes"
    id: concept:page_642_bun
    description: |
      Extracted from documentation: Watch a directory for changes
  - name: "Build-time constants with --define"
    id: concept:page_643_bun
    description: |
      Extracted from documentation: Build-time constants with --define
  - name: "Why use build-time constants?"
    id: concept:page_644_bun
    description: |
      Extracted from documentation: Why use build-time constants?
  - name: "Basic usage"
    id: concept:page_645_bun
    description: |
      Extracted from documentation: Basic usage
  - name: "With `bun build`"
    id: concept:page_646_bun
    description: |
      Extracted from documentation: With `bun build`
  - name: "Bundle with build-time constants"
    id: concept:page_647_bun
    description: |
      Extracted from documentation: Bundle with build-time constants
  - name: "With `bun build --compile`"
    id: concept:page_648_bun
    description: |
      Extracted from documentation: With `bun build --compile`
  - name: "Compile to executable with build-time constants"
    id: concept:page_649_bun
    description: |
      Extracted from documentation: Compile to executable with build-time constants
  - name: "Common use cases"
    id: concept:page_650_bun
    description: |
      Extracted from documentation: Common use cases
  - name: "Version information"
    id: concept:page_651_bun
    description: |
      Extracted from documentation: Version information
  - name: "Feature flags"
    id: concept:page_652_bun
    description: |
      Extracted from documentation: Feature flags
  - name: "Production build - analytics enabled, debug disabled"
    id: concept:page_653_bun
    description: |
      Extracted from documentation: Production build - analytics enabled, debug disabled
  - name: "Development build - both enabled"
    id: concept:page_654_bun
    description: |
      Extracted from documentation: Development build - both enabled
  - name: "Configuration"
    id: concept:page_655_bun
    description: |
      Extracted from documentation: Configuration
  - name: "Advanced patterns"
    id: concept:page_656_bun
    description: |
      Extracted from documentation: Advanced patterns
  - name: "Environment-specific builds"
    id: concept:page_657_bun
    description: |
      Extracted from documentation: Environment-specific builds
  - name: "Using shell commands for dynamic values"
    id: concept:page_658_bun
    description: |
      Extracted from documentation: Using shell commands for dynamic values
  - name: "Use git to get current commit and timestamp"
    id: concept:page_659_bun
    description: |
      Extracted from documentation: Use git to get current commit and timestamp
  - name: "Build automation script"
    id: concept:page_660_bun
    description: |
      Extracted from documentation: Build automation script
  - name: "Important considerations"
    id: concept:page_661_bun
    description: |
      Extracted from documentation: Important considerations
  - name: "Value format"
    id: concept:page_662_bun
    description: |
      Extracted from documentation: Value format
  - name: "✅ Strings must be JSON-quoted"
    id: concept:page_663_bun
    description: |
      Extracted from documentation: ✅ Strings must be JSON-quoted
  - name: "✅ Numbers are JSON literals"
    id: concept:page_664_bun
    description: |
      Extracted from documentation: ✅ Numbers are JSON literals
  - name: "✅ Booleans are JSON literals"
    id: concept:page_665_bun
    description: |
      Extracted from documentation: ✅ Booleans are JSON literals
  - name: "✅ Objects and arrays (use single quotes to wrap the JSON)"
    id: concept:page_666_bun
    description: |
      Extracted from documentation: ✅ Objects and arrays (use single quotes to wrap the JSON)
  - name: "✅ Arrays work too"
    id: concept:page_667_bun
    description: |
      Extracted from documentation: ✅ Arrays work too
  - name: "❌ This won't work - missing quotes around string"
    id: concept:page_668_bun
    description: |
      Extracted from documentation: ❌ This won't work - missing quotes around string
  - name: "Property keys"
    id: concept:page_669_bun
    description: |
      Extracted from documentation: Property keys
  - name: "✅ Replace process.env.NODE_ENV with \"production\""
    id: concept:page_670_bun
    description: |
      Extracted from documentation: ✅ Replace process.env.NODE_ENV with "production"
  - name: "✅ Replace process.env.API_KEY with the actual key"
    id: concept:page_671_bun
    description: |
      Extracted from documentation: ✅ Replace process.env.API_KEY with the actual key
  - name: "✅ Replace nested properties"
    id: concept:page_672_bun
    description: |
      Extracted from documentation: ✅ Replace nested properties
  - name: "TypeScript declarations"
    id: concept:page_673_bun
    description: |
      Extracted from documentation: TypeScript declarations
  - name: "Cross-platform compatibility"
    id: concept:page_674_bun
    description: |
      Extracted from documentation: Cross-platform compatibility
  - name: "Linux"
    id: concept:page_675_bun
    description: |
      Extracted from documentation: Linux
  - name: "macOS"
    id: concept:page_676_bun
    description: |
      Extracted from documentation: macOS
  - name: "Windows"
    id: concept:page_677_bun
    description: |
      Extracted from documentation: Windows
  - name: "Related"
    id: concept:page_678_bun
    description: |
      Extracted from documentation: Related
  - name: "Install and run Bun in GitHub Actions"
    id: concept:page_679_bun
    description: |
      Extracted from documentation: Install and run Bun in GitHub Actions
  - name: "Codesign a single-file JavaScript executable on macOS"
    id: concept:page_680_bun
    description: |
      Extracted from documentation: Codesign a single-file JavaScript executable on macOS
  - name: "Define and replace static globals & constants"
    id: concept:page_681_bun
    description: |
      Extracted from documentation: Define and replace static globals & constants
  - name: "What types of values are supported?"
    id: concept:page_682_bun
    description: |
      Extracted from documentation: What types of values are supported?
  - name: "Replace global identifiers"
    id: concept:page_683_bun
    description: |
      Extracted from documentation: Replace global identifiers
  - name: "Replace values with JSON"
    id: concept:page_684_bun
    description: |
      Extracted from documentation: Replace values with JSON
  - name: "JSON"
    id: concept:page_685_bun
    description: |
      Extracted from documentation: JSON
  - name: "Replace values with other properties"
    id: concept:page_686_bun
    description: |
      Extracted from documentation: Replace values with other properties
  - name: "How is this different than setting a variable?"
    id: concept:page_687_bun
    description: |
      Extracted from documentation: How is this different than setting a variable?
  - name: "How is this different than find-and-replace or string replacement?"
    id: concept:page_688_bun
    description: |
      Extracted from documentation: How is this different than find-and-replace or string replacement?
  - name: "Delete directories"
    id: concept:page_689_bun
    description: |
      Extracted from documentation: Delete directories
  - name: "Delete files"
    id: concept:page_690_bun
    description: |
      Extracted from documentation: Delete files
  - name: "Inspect memory usage using V8 heap snapshots"
    id: concept:page_691_bun
    description: |
      Extracted from documentation: Inspect memory usage using V8 heap snapshots
  - name: "Inspect memory in Chrome DevTools"
    id: concept:page_692_bun
    description: |
      Extracted from documentation: Inspect memory in Chrome DevTools
  - name: "Import a HTML file as text"
    id: concept:page_693_bun
    description: |
      Extracted from documentation: Import a HTML file as text
  - name: "Import a JSON file"
    id: concept:page_694_bun
    description: |
      Extracted from documentation: Import a JSON file
  - name: "Import a JSON5 file"
    id: concept:page_695_bun
    description: |
      Extracted from documentation: Import a JSON5 file
  - name: "Import a TOML file"
    id: concept:page_696_bun
    description: |
      Extracted from documentation: Import a TOML file
  - name: "Import a YAML file"
    id: concept:page_697_bun
    description: |
      Extracted from documentation: Import a YAML file
  - name: "TypeScript Support"
    id: concept:page_698_bun
    description: |
      Extracted from documentation: TypeScript Support
  - name: "Read environment variables"
    id: concept:page_699_bun
    description: |
      Extracted from documentation: Read environment variables
  - name: "Set environment variables"
    id: concept:page_700_bun
    description: |
      Extracted from documentation: Set environment variables
  - name: "Using CMD"
    id: concept:page_701_bun
    description: |
      Extracted from documentation: Using CMD
  - name: "Using PowerShell"
    id: concept:page_702_bun
    description: |
      Extracted from documentation: Using PowerShell
  - name: "Run a Shell Command"
    id: concept:page_703_bun
    description: |
      Extracted from documentation: Run a Shell Command
  - name: "Set a time zone in Bun"
    id: concept:page_704_bun
    description: |
      Extracted from documentation: Set a time zone in Bun
  - name: "Re-map import paths"
    id: concept:page_705_bun
    description: |
      Extracted from documentation: Re-map import paths
  - name: "Install TypeScript declarations for Bun"
    id: concept:page_706_bun
    description: |
      Extracted from documentation: Install TypeScript declarations for Bun
  - name: "Debugging Bun with the VS Code extension"
    id: concept:page_707_bun
    description: |
      Extracted from documentation: Debugging Bun with the VS Code extension
  - name: "Debugging Bun with the web debugger"
    id: concept:page_708_bun
    description: |
      Extracted from documentation: Debugging Bun with the web debugger
  - name: "Convert a Node.js Readable to an ArrayBuffer"
    id: concept:page_709_bun
    description: |
      Extracted from documentation: Convert a Node.js Readable to an ArrayBuffer
  - name: "Convert a Node.js Readable to a Blob"
    id: concept:page_710_bun
    description: |
      Extracted from documentation: Convert a Node.js Readable to a Blob
  - name: "Convert a Node.js Readable to JSON"
    id: concept:page_711_bun
    description: |
      Extracted from documentation: Convert a Node.js Readable to JSON
  - name: "Convert a Node.js Readable to a string"
    id: concept:page_712_bun
    description: |
      Extracted from documentation: Convert a Node.js Readable to a string
  - name: "Convert a Node.js Readable to an Uint8Array"
    id: concept:page_713_bun
    description: |
      Extracted from documentation: Convert a Node.js Readable to an Uint8Array
  - name: "Convert a ReadableStream to an array of chunks"
    id: concept:page_714_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to an array of chunks
  - name: "Convert a ReadableStream to an ArrayBuffer"
    id: concept:page_715_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to an ArrayBuffer
  - name: "Convert a ReadableStream to a Blob"
    id: concept:page_716_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to a Blob
  - name: "Convert a ReadableStream to a Buffer"
    id: concept:page_717_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to a Buffer
  - name: "Convert a ReadableStream to JSON"
    id: concept:page_718_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to JSON
  - name: "Convert a ReadableStream to a string"
    id: concept:page_719_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to a string
  - name: "Convert a ReadableStream to a Uint8Array"
    id: concept:page_720_bun
    description: |
      Extracted from documentation: Convert a ReadableStream to a Uint8Array
  - name: "Bail early with the Bun test runner"
    id: concept:page_721_bun
    description: |
      Extracted from documentation: Bail early with the Bun test runner
  - name: "bail after 10 failures"
    id: concept:page_722_bun
    description: |
      Extracted from documentation: bail after 10 failures
  - name: "Selectively run tests concurrently with glob patterns"
    id: concept:page_723_bun
    description: |
      Extracted from documentation: Selectively run tests concurrently with glob patterns
  - name: "Run all test files with \"concurrent-\" prefix concurrently"
    id: concept:page_724_bun
    description: |
      Extracted from documentation: Run all test files with "concurrent-" prefix concurrently
  - name: "Test Files"
    id: concept:page_725_bun
    description: |
      Extracted from documentation: Test Files
  - name: "Unit Test (Sequential)"
    id: concept:page_726_bun
    description: |
      Extracted from documentation: Unit Test (Sequential)
  - name: "Integration Test (Concurrent)"
    id: concept:page_727_bun
    description: |
      Extracted from documentation: Integration Test (Concurrent)
  - name: "Running Tests"
    id: concept:page_728_bun
    description: |
      Extracted from documentation: Running Tests
  - name: "Run all tests - concurrent-*.test.ts files will run concurrently"
    id: concept:page_729_bun
    description: |
      Extracted from documentation: Run all tests - concurrent-*.test.ts files will run concurrently
  - name: "Override: Force ALL tests to run concurrently"
    id: concept:page_730_bun
    description: |
      Extracted from documentation: Override: Force ALL tests to run concurrently
  - name: "Note: This overrides bunfig.toml and runs all tests concurrently, regardless of glob"
    id: concept:page_731_bun
    description: |
      Extracted from documentation: Note: This overrides bunfig.toml and runs all tests concurrently, regardless of glob
  - name: "Run only unit tests (sequential)"
    id: concept:page_732_bun
    description: |
      Extracted from documentation: Run only unit tests (sequential)
  - name: "Run only integration tests (concurrent due to glob pattern)"
    id: concept:page_733_bun
    description: |
      Extracted from documentation: Run only integration tests (concurrent due to glob pattern)
  - name: "Benefits"
    id: concept:page_734_bun
    description: |
      Extracted from documentation: Benefits
  - name: "Migration Strategy"
    id: concept:page_735_bun
    description: |
      Extracted from documentation: Migration Strategy
  - name: "Tips"
    id: concept:page_736_bun
    description: |
      Extracted from documentation: Tips
  - name: "Multiple Patterns"
    id: concept:page_737_bun
    description: |
      Extracted from documentation: Multiple Patterns
  - name: "Generate code coverage reports with the Bun test runner"
    id: concept:page_738_bun
    description: |
      Extracted from documentation: Generate code coverage reports with the Bun test runner
  - name: "Set a code coverage threshold with the Bun test runner"
    id: concept:page_739_bun
    description: |
      Extracted from documentation: Set a code coverage threshold with the Bun test runner
  - name: "to require 90% line-level and function-level coverage"
    id: concept:page_740_bun
    description: |
      Extracted from documentation: to require 90% line-level and function-level coverage
  - name: "to set different thresholds for lines and functions"
    id: concept:page_741_bun
    description: |
      Extracted from documentation: to set different thresholds for lines and functions
  - name: "Write browser DOM tests with Bun and happy-dom"
    id: concept:page_742_bun
    description: |
      Extracted from documentation: Write browser DOM tests with Bun and happy-dom
  - name: "Migrate from Jest to Bun's test runner"
    id: concept:page_743_bun
    description: |
      Extracted from documentation: Migrate from Jest to Bun's test runner
  - name: "Set the system time in Bun's test runner"
    id: concept:page_744_bun
    description: |
      Extracted from documentation: Set the system time in Bun's test runner
  - name: "Mock functions in `bun test`"
    id: concept:page_745_bun
    description: |
      Extracted from documentation: Mock functions in `bun test`
  - name: "Re-run tests multiple times with the Bun test runner"
    id: concept:page_746_bun
    description: |
      Extracted from documentation: Re-run tests multiple times with the Bun test runner
  - name: "re-run each test 10 times"
    id: concept:page_747_bun
    description: |
      Extracted from documentation: re-run each test 10 times
  - name: "Run your tests with the Bun test runner"
    id: concept:page_748_bun
    description: |
      Extracted from documentation: Run your tests with the Bun test runner
  - name: "Skip tests with the Bun test runner"
    id: concept:page_749_bun
    description: |
      Extracted from documentation: Skip tests with the Bun test runner
  - name: "Use snapshot testing in `bun test`"
    id: concept:page_750_bun
    description: |
      Extracted from documentation: Use snapshot testing in `bun test`
  - name: "Spy on methods in `bun test`"
    id: concept:page_751_bun
    description: |
      Extracted from documentation: Spy on methods in `bun test`
  - name: "import, require, and test Svelte components with bun test"
    id: concept:page_752_bun
    description: |
      Extracted from documentation: import, require, and test Svelte components with bun test
  - name: "Tell Bun to load this plugin before your tests run"
    id: concept:page_753_bun
    description: |
      Extracted from documentation: Tell Bun to load this plugin before your tests run
  - name: "test.preload = [\"./svelte-loader.ts\"]"
    id: concept:page_754_bun
    description: |
      Extracted from documentation: test.preload = ["./svelte-loader.ts"]
  - name: "Using Testing Library with Bun"
    id: concept:page_755_bun
    description: |
      Extracted from documentation: Using Testing Library with Bun
  - name: "Set a per-test timeout with the Bun test runner"
    id: concept:page_756_bun
    description: |
      Extracted from documentation: Set a per-test timeout with the Bun test runner
  - name: "Mark a test as a \"todo\" with the Bun test runner"
    id: concept:page_757_bun
    description: |
      Extracted from documentation: Mark a test as a "todo" with the Bun test runner
  - name: "Update snapshots in `bun test`"
    id: concept:page_758_bun
    description: |
      Extracted from documentation: Update snapshots in `bun test`
  - name: "Run tests in watch mode with Bun"
    id: concept:page_759_bun
    description: |
      Extracted from documentation: Run tests in watch mode with Bun
  - name: "Encode and decode base64 data"
    id: concept:page_760_bun
    description: |
      Extracted from documentation: Encode and decode base64 data
  - name: "Check if two objects are deeply equal"
    id: concept:page_761_bun
    description: |
      Extracted from documentation: Check if two objects are deeply equal
  - name: "Compress and decompress data with DEFLATE"
    id: concept:page_762_bun
    description: |
      Extracted from documentation: Compress and decompress data with DEFLATE
  - name: "Detect when code is executed with Bun"
    id: concept:page_763_bun
    description: |
      Extracted from documentation: Detect when code is executed with Bun
  - name: "Check if the current file is the entrypoint"
    id: concept:page_764_bun
    description: |
      Extracted from documentation: Check if the current file is the entrypoint
  - name: "Escape an HTML string"
    id: concept:page_765_bun
    description: |
      Extracted from documentation: Escape an HTML string
  - name: "Convert a file URL to an absolute path"
    id: concept:page_766_bun
    description: |
      Extracted from documentation: Convert a file URL to an absolute path
  - name: "Compress and decompress data with gzip"
    id: concept:page_767_bun
    description: |
      Extracted from documentation: Compress and decompress data with gzip
  - name: "Hash a password"
    id: concept:page_768_bun
    description: |
      Extracted from documentation: Hash a password
  - name: "Get the directory of the current file"
    id: concept:page_769_bun
    description: |
      Extracted from documentation: Get the directory of the current file
  - name: "Get the file name of the current file"
    id: concept:page_770_bun
    description: |
      Extracted from documentation: Get the file name of the current file
  - name: "Get the absolute path of the current file"
    id: concept:page_771_bun
    description: |
      Extracted from documentation: Get the absolute path of the current file
  - name: "Generate a UUID"
    id: concept:page_772_bun
    description: |
      Extracted from documentation: Generate a UUID
  - name: "Get the absolute path to the current entrypoint"
    id: concept:page_773_bun
    description: |
      Extracted from documentation: Get the absolute path to the current entrypoint
  - name: "Convert an absolute path to a file URL"
    id: concept:page_774_bun
    description: |
      Extracted from documentation: Convert an absolute path to a file URL
  - name: "Sleep for a fixed number of milliseconds"
    id: concept:page_775_bun
    description: |
      Extracted from documentation: Sleep for a fixed number of milliseconds
  - name: "Upgrade Bun to the latest version"
    id: concept:page_776_bun
    description: |
      Extracted from documentation: Upgrade Bun to the latest version
  - name: "Verify the upgrade"
    id: concept:page_777_bun
    description: |
      Extracted from documentation: Verify the upgrade
  - name: "Output: 1.x.y"
    id: concept:page_778_bun
    description: |
      Extracted from documentation: Output: 1.x.y
  - name: "See the exact commit of the Bun binary"
    id: concept:page_779_bun
    description: |
      Extracted from documentation: See the exact commit of the Bun binary
  - name: "Output: 1.x.y+abc123def"
    id: concept:page_780_bun
    description: |
      Extracted from documentation: Output: 1.x.y+abc123def
  - name: "Upgrade to canary builds"
    id: concept:page_781_bun
    description: |
      Extracted from documentation: Upgrade to canary builds
  - name: "Switch back to stable"
    id: concept:page_782_bun
    description: |
      Extracted from documentation: Switch back to stable
  - name: "Install a specific version"
    id: concept:page_783_bun
    description: |
      Extracted from documentation: Install a specific version
  - name: "Package manager users"
    id: concept:page_784_bun
    description: |
      Extracted from documentation: Package manager users
  - name: "See also"
    id: concept:page_785_bun
    description: |
      Extracted from documentation: See also
  - name: "Get the current Bun version"
    id: concept:page_786_bun
    description: |
      Extracted from documentation: Get the current Bun version
  - name: "Get the path to an executable bin file"
    id: concept:page_787_bun
    description: |
      Extracted from documentation: Get the path to an executable bin file
  - name: "Enable compression for WebSocket messages"
    id: concept:page_788_bun
    description: |
      Extracted from documentation: Enable compression for WebSocket messages
  - name: "Set per-socket contextual data on a WebSocket"
    id: concept:page_789_bun
    description: |
      Extracted from documentation: Set per-socket contextual data on a WebSocket
  - name: "Build a publish-subscribe WebSocket server"
    id: concept:page_790_bun
    description: |
      Extracted from documentation: Build a publish-subscribe WebSocket server
  - name: "Build a simple WebSocket server"
    id: concept:page_791_bun
    description: |
      Extracted from documentation: Build a simple WebSocket server
  - name: "Append content to a file"
    id: concept:page_792_bun
    description: |
      Extracted from documentation: Append content to a file
  - name: "Write a string to a file"
    id: concept:page_793_bun
    description: |
      Extracted from documentation: Write a string to a file
  - name: "Write a Blob to a file"
    id: concept:page_794_bun
    description: |
      Extracted from documentation: Write a Blob to a file
  - name: "Write a file to stdout"
    id: concept:page_795_bun
    description: |
      Extracted from documentation: Write a file to stdout
  - name: "Copy a file to another location"
    id: concept:page_796_bun
    description: |
      Extracted from documentation: Copy a file to another location
  - name: "Write a file incrementally"
    id: concept:page_797_bun
    description: |
      Extracted from documentation: Write a file incrementally
  - name: "Write a Response to a file"
    id: concept:page_798_bun
    description: |
      Extracted from documentation: Write a Response to a file
  - name: "Write to stdout"
    id: concept:page_799_bun
    description: |
      Extracted from documentation: Write to stdout
  - name: "Write a ReadableStream to a file"
    id: concept:page_800_bun
    description: |
      Extracted from documentation: Write a ReadableStream to a file
  - name: "Delete a file"
    id: concept:page_801_bun
    description: |
      Extracted from documentation: Delete a file
  - name: "Get Started"
    id: concept:page_802_bun
    description: |
      Extracted from documentation: Get Started
  - name: "What's Inside"
    id: concept:page_803_bun
    description: |
      Extracted from documentation: What's Inside
  - name: "What is Bun?"
    id: concept:page_804_bun
    description: |
      Extracted from documentation: What is Bun?
  - name: "What is a runtime?"
    id: concept:page_805_bun
    description: |
      Extracted from documentation: What is a runtime?
  - name: "Browsers"
    id: concept:page_806_bun
    description: |
      Extracted from documentation: Browsers
  - name: "Node.js"
    id: concept:page_807_bun
    description: |
      Extracted from documentation: Node.js
  - name: "Design goals"
    id: concept:page_808_bun
    description: |
      Extracted from documentation: Design goals
  - name: "Installation"
    id: concept:page_809_bun
    description: |
      Extracted from documentation: Installation
  - name: "Overview"
    id: concept:page_810_bun
    description: |
      Extracted from documentation: Overview
  - name: "Image Variants"
    id: concept:page_811_bun
    description: |
      Extracted from documentation: Image Variants
  - name: "See the precise commit of `oven-sh/bun` that you're using"
    id: concept:page_812_bun
    description: |
      Extracted from documentation: See the precise commit of `oven-sh/bun` that you're using
  - name: "Output: 1.x.y+b7982ac13189"
    id: concept:page_813_bun
    description: |
      Extracted from documentation: Output: 1.x.y+b7982ac13189
  - name: "/bin/zsh  or /bin/bash or /bin/fish"
    id: concept:page_814_bun
    description: |
      Extracted from documentation: /bin/zsh  or /bin/bash or /bin/fish
  - name: "Upgrading"
    id: concept:page_815_bun
    description: |
      Extracted from documentation: Upgrading
  - name: "Canary Builds"
    id: concept:page_816_bun
    description: |
      Extracted from documentation: Canary Builds
  - name: "Upgrade to latest canary"
    id: concept:page_817_bun
    description: |
      Extracted from documentation: Upgrade to latest canary
  - name: "Installing Older Versions"
    id: concept:page_818_bun
    description: |
      Extracted from documentation: Installing Older Versions
  - name: "Direct Downloads"
    id: concept:page_819_bun
    description: |
      Extracted from documentation: Direct Downloads
  - name: "Latest Version Downloads"
    id: concept:page_820_bun
    description: |
      Extracted from documentation: Latest Version Downloads
  - name: "Musl Binaries"
    id: concept:page_821_bun
    description: |
      Extracted from documentation: Musl Binaries
  - name: "CPU Requirements"
    id: concept:page_822_bun
    description: |
      Extracted from documentation: CPU Requirements
  - name: "Uninstall"
    id: concept:page_823_bun
    description: |
      Extracted from documentation: Uninstall
  - name: "bunx"
    id: concept:page_824_bun
    description: |
      Extracted from documentation: bunx
  - name: "!/usr/bin/env node"
    id: concept:page_825_bun
    description: |
      Extracted from documentation: !/usr/bin/env node
  - name: "Arguments and flags"
    id: concept:page_826_bun
    description: |
      Extracted from documentation: Arguments and flags
  - name: "Shebangs"
    id: concept:page_827_bun
    description: |
      Extracted from documentation: Shebangs
  - name: "Package flag"
    id: concept:page_828_bun
    description: |
      Extracted from documentation: Package flag
  - name: "!/usr/bin/env bun"
    id: concept:page_829_bun
    description: |
      Extracted from documentation: !/usr/bin/env bun
  - name: "Flags"
    id: concept:page_830_bun
    description: |
      Extracted from documentation: Flags
  - name: "Run Prisma migrations"
    id: concept:page_831_bun
    description: |
      Extracted from documentation: Run Prisma migrations
  - name: "Format a file with Prettier"
    id: concept:page_832_bun
    description: |
      Extracted from documentation: Format a file with Prettier
  - name: "Run a specific version of a package"
    id: concept:page_833_bun
    description: |
      Extracted from documentation: Run a specific version of a package
  - name: "Use --package when binary name differs from package name"
    id: concept:page_834_bun
    description: |
      Extracted from documentation: Use --package when binary name differs from package name
  - name: "Force running with Bun instead of Node.js, even if the executable contains a Node shebang"
    id: concept:page_835_bun
    description: |
      Extracted from documentation: Force running with Bun instead of Node.js, even if the executable contains a Node shebang
  - name: "Catalogs"
    id: concept:page_836_bun
    description: |
      Extracted from documentation: Catalogs
  - name: "How to Use Catalogs"
    id: concept:page_837_bun
    description: |
      Extracted from documentation: How to Use Catalogs
  - name: "Directory Structure Example"
    id: concept:page_838_bun
    description: |
      Extracted from documentation: Directory Structure Example
  - name: "1. Define Catalogs in Root package.json"
    id: concept:page_839_bun
    description: |
      Extracted from documentation: 1. Define Catalogs in Root package.json
  - name: "2. Reference Catalog Versions in Workspace Packages"
    id: concept:page_840_bun
    description: |
      Extracted from documentation: 2. Reference Catalog Versions in Workspace Packages
  - name: "3. Run Bun Install"
    id: concept:page_841_bun
    description: |
      Extracted from documentation: 3. Run Bun Install
  - name: "Catalog vs Catalogs"
    id: concept:page_842_bun
    description: |
      Extracted from documentation: Catalog vs Catalogs
  - name: "Benefits of Using Catalogs"
    id: concept:page_843_bun
    description: |
      Extracted from documentation: Benefits of Using Catalogs
  - name: "Real-World Example"
    id: concept:page_844_bun
    description: |
      Extracted from documentation: Real-World Example
  - name: "Updating Versions"
    id: concept:page_845_bun
    description: |
      Extracted from documentation: Updating Versions
  - name: "Lockfile Integration"
    id: concept:page_846_bun
    description: |
      Extracted from documentation: Lockfile Integration
  - name: "Limitations and Edge Cases"
    id: concept:page_847_bun
    description: |
      Extracted from documentation: Limitations and Edge Cases
  - name: "Publishing"
    id: concept:page_848_bun
    description: |
      Extracted from documentation: Publishing
  - name: "bun add"
    id: concept:page_849_bun
    description: |
      Extracted from documentation: bun add
  - name: "`--dev`"
    id: concept:page_850_bun
    description: |
      Extracted from documentation: `--dev`
  - name: "`--optional`"
    id: concept:page_851_bun
    description: |
      Extracted from documentation: `--optional`
  - name: "`--peer`"
    id: concept:page_852_bun
    description: |
      Extracted from documentation: `--peer`
  - name: "`--exact`"
    id: concept:page_853_bun
    description: |
      Extracted from documentation: `--exact`
  - name: "`--global`"
    id: concept:page_854_bun
    description: |
      Extracted from documentation: `--global`
  - name: "where `bun add --global` installs packages"
    id: concept:page_855_bun
    description: |
      Extracted from documentation: where `bun add --global` installs packages
  - name: "where globally-installed package bins are linked"
    id: concept:page_856_bun
    description: |
      Extracted from documentation: where globally-installed package bins are linked
  - name: "Trusted dependencies"
    id: concept:page_857_bun
    description: |
      Extracted from documentation: Trusted dependencies
  - name: "Git dependencies"
    id: concept:page_858_bun
    description: |
      Extracted from documentation: Git dependencies
  - name: "Tarball dependencies"
    id: concept:page_859_bun
    description: |
      Extracted from documentation: Tarball dependencies
  - name: "Dependency Management"
    id: concept:page_860_bun
    description: |
      Extracted from documentation: Dependency Management
  - name: "Project Files & Lockfiles"
    id: concept:page_861_bun
    description: |
      Extracted from documentation: Project Files & Lockfiles
  - name: "Installation Control"
    id: concept:page_862_bun
    description: |
      Extracted from documentation: Installation Control
  - name: "Network & Registry"
    id: concept:page_863_bun
    description: |
      Extracted from documentation: Network & Registry
  - name: "Performance & Resource"
    id: concept:page_864_bun
    description: |
      Extracted from documentation: Performance & Resource
  - name: "Caching"
    id: concept:page_865_bun
    description: |
      Extracted from documentation: Caching
  - name: "Output & Logging"
    id: concept:page_866_bun
    description: |
      Extracted from documentation: Output & Logging
  - name: "Global Configuration & Context"
    id: concept:page_867_bun
    description: |
      Extracted from documentation: Global Configuration & Context
  - name: "Help"
    id: concept:page_868_bun
    description: |
      Extracted from documentation: Help
  - name: "bun audit"
    id: concept:page_869_bun
    description: |
      Extracted from documentation: bun audit
  - name: "Filtering options"
    id: concept:page_870_bun
    description: |
      Extracted from documentation: Filtering options
  - name: "`--json`"
    id: concept:page_871_bun
    description: |
      Extracted from documentation: `--json`
  - name: "Exit code"
    id: concept:page_872_bun
    description: |
      Extracted from documentation: Exit code
  - name: "bun info"
    id: concept:page_873_bun
    description: |
      Extracted from documentation: bun info
  - name: "Viewing specific versions"
    id: concept:page_874_bun
    description: |
      Extracted from documentation: Viewing specific versions
  - name: "Viewing specific properties"
    id: concept:page_875_bun
    description: |
      Extracted from documentation: Viewing specific properties
  - name: "JSON output"
    id: concept:page_876_bun
    description: |
      Extracted from documentation: JSON output
  - name: "Alias"
    id: concept:page_877_bun
    description: |
      Extracted from documentation: Alias
  - name: "View basic package information"
    id: concept:page_878_bun
    description: |
      Extracted from documentation: View basic package information
  - name: "View a specific version"
    id: concept:page_879_bun
    description: |
      Extracted from documentation: View a specific version
  - name: "View all available versions"
    id: concept:page_880_bun
    description: |
      Extracted from documentation: View all available versions
  - name: "View package dependencies"
    id: concept:page_881_bun
    description: |
      Extracted from documentation: View package dependencies
  - name: "View package homepage"
    id: concept:page_882_bun
    description: |
      Extracted from documentation: View package homepage
  - name: "Get JSON output"
    id: concept:page_883_bun
    description: |
      Extracted from documentation: Get JSON output
  - name: "bun install"
    id: concept:page_884_bun
    description: |
      Extracted from documentation: bun install
  - name: "Basic Usage"
    id: concept:page_885_bun
    description: |
      Extracted from documentation: Basic Usage
  - name: "Logging"
    id: concept:page_886_bun
    description: |
      Extracted from documentation: Logging
  - name: "Lifecycle scripts"
    id: concept:page_887_bun
    description: |
      Extracted from documentation: Lifecycle scripts
  - name: "Workspaces"
    id: concept:page_888_bun
    description: |
      Extracted from documentation: Workspaces
  - name: "Installing dependencies for specific packages"
    id: concept:page_889_bun
    description: |
      Extracted from documentation: Installing dependencies for specific packages
  - name: "Install dependencies for all workspaces except `pkg-c`"
    id: concept:page_890_bun
    description: |
      Extracted from documentation: Install dependencies for all workspaces except `pkg-c`
  - name: "Install dependencies for only `pkg-a` in `./packages/pkg-a`"
    id: concept:page_891_bun
    description: |
      Extracted from documentation: Install dependencies for only `pkg-a` in `./packages/pkg-a`
  - name: "Overrides and resolutions"
    id: concept:page_892_bun
    description: |
      Extracted from documentation: Overrides and resolutions
  - name: "Global packages"
    id: concept:page_893_bun
    description: |
      Extracted from documentation: Global packages
  - name: "Omitting dependencies"
    id: concept:page_894_bun
    description: |
      Extracted from documentation: Omitting dependencies
  - name: "Exclude \"devDependencies\" from the installation. This will apply to the"
    id: concept:page_895_bun
    description: |
      Extracted from documentation: Exclude "devDependencies" from the installation. This will apply to the
  - name: "root package and workspaces if they exist. Transitive dependencies will"
    id: concept:page_896_bun
    description: |
      Extracted from documentation: root package and workspaces if they exist. Transitive dependencies will
  - name: "not have \"devDependencies\"."
    id: concept:page_897_bun
    description: |
      Extracted from documentation: not have "devDependencies".
  - name: "Install only dependencies from \"dependencies\""
    id: concept:page_898_bun
    description: |
      Extracted from documentation: Install only dependencies from "dependencies"
  - name: "Dry run"
    id: concept:page_899_bun
    description: |
      Extracted from documentation: Dry run
  - name: "Non-npm dependencies"
    id: concept:page_900_bun
    description: |
      Extracted from documentation: Non-npm dependencies
  - name: "Installation strategies"
    id: concept:page_901_bun
    description: |
      Extracted from documentation: Installation strategies
  - name: "Hoisted installs"
    id: concept:page_902_bun
    description: |
      Extracted from documentation: Hoisted installs
  - name: "Isolated installs"
    id: concept:page_903_bun
    description: |
      Extracted from documentation: Isolated installs
  - name: "Default strategy"
    id: concept:page_904_bun
    description: |
      Extracted from documentation: Default strategy
  - name: "Minimum release age"
    id: concept:page_905_bun
    description: |
      Extracted from documentation: Minimum release age
  - name: "Only install package versions published at least 3 days ago"
    id: concept:page_906_bun
    description: |
      Extracted from documentation: Only install package versions published at least 3 days ago
  - name: "Exclude trusted packages from the age gate"
    id: concept:page_907_bun
    description: |
      Extracted from documentation: Exclude trusted packages from the age gate
  - name: "Configuring `bun install` with `bunfig.toml`"
    id: concept:page_908_bun
    description: |
      Extracted from documentation: Configuring `bun install` with `bunfig.toml`
  - name: "whether to install optionalDependencies"
    id: concept:page_909_bun
    description: |
      Extracted from documentation: whether to install optionalDependencies
  - name: "whether to install devDependencies"
    id: concept:page_910_bun
    description: |
      Extracted from documentation: whether to install devDependencies
  - name: "whether to install peerDependencies"
    id: concept:page_911_bun
    description: |
      Extracted from documentation: whether to install peerDependencies
  - name: "equivalent to `--production` flag"
    id: concept:page_912_bun
    description: |
      Extracted from documentation: equivalent to `--production` flag
  - name: "equivalent to `--save-text-lockfile` flag"
    id: concept:page_913_bun
    description: |
      Extracted from documentation: equivalent to `--save-text-lockfile` flag
  - name: "equivalent to `--frozen-lockfile` flag"
    id: concept:page_914_bun
    description: |
      Extracted from documentation: equivalent to `--frozen-lockfile` flag
  - name: "equivalent to `--dry-run` flag"
    id: concept:page_915_bun
    description: |
      Extracted from documentation: equivalent to `--dry-run` flag
  - name: "equivalent to `--concurrent-scripts` flag"
    id: concept:page_916_bun
    description: |
      Extracted from documentation: equivalent to `--concurrent-scripts` flag
  - name: "installation strategy: \"hoisted\" or \"isolated\""
    id: concept:page_917_bun
    description: |
      Extracted from documentation: installation strategy: "hoisted" or "isolated"
  - name: "default depends on lockfile configVersion and workspaces:"
    id: concept:page_918_bun
    description: |
      Extracted from documentation: default depends on lockfile configVersion and workspaces:
  - name: "- configVersion = 1: \"isolated\" if using workspaces, otherwise \"hoisted\""
    id: concept:page_919_bun
    description: |
      Extracted from documentation: - configVersion = 1: "isolated" if using workspaces, otherwise "hoisted"
  - name: "- configVersion = 0: \"hoisted\""
    id: concept:page_920_bun
    description: |
      Extracted from documentation: - configVersion = 0: "hoisted"
  - name: "minimum age config"
    id: concept:page_921_bun
    description: |
      Extracted from documentation: minimum age config
  - name: "Configuring with environment variables"
    id: concept:page_922_bun
    description: |
      Extracted from documentation: Configuring with environment variables
  - name: "Platform-specific dependencies?"
    id: concept:page_923_bun
    description: |
      Extracted from documentation: Platform-specific dependencies?
  - name: "`--cpu` and `--os` flags"
    id: concept:page_924_bun
    description: |
      Extracted from documentation: `--cpu` and `--os` flags
  - name: "Peer dependencies?"
    id: concept:page_925_bun
    description: |
      Extracted from documentation: Peer dependencies?
  - name: "Lockfile"
    id: concept:page_926_bun
    description: |
      Extracted from documentation: Lockfile
  - name: "Cache"
    id: concept:page_927_bun
    description: |
      Extracted from documentation: Cache
  - name: "or"
    id: concept:page_928_bun
    description: |
      Extracted from documentation: or
  - name: "Platform-specific backends"
    id: concept:page_929_bun
    description: |
      Extracted from documentation: Platform-specific backends
  - name: "npm registry metadata"
    id: concept:page_930_bun
    description: |
      Extracted from documentation: npm registry metadata
  - name: "pnpm migration"
    id: concept:page_931_bun
    description: |
      Extracted from documentation: pnpm migration
  - name: "Lockfile Migration"
    id: concept:page_932_bun
    description: |
      Extracted from documentation: Lockfile Migration
  - name: "Workspace Configuration"
    id: concept:page_933_bun
    description: |
      Extracted from documentation: Workspace Configuration
  - name: "Catalog Dependencies"
    id: concept:page_934_bun
    description: |
      Extracted from documentation: Catalog Dependencies
  - name: "Configuration Migration"
    id: concept:page_935_bun
    description: |
      Extracted from documentation: Configuration Migration
  - name: "Requirements"
    id: concept:page_936_bun
    description: |
      Extracted from documentation: Requirements
  - name: "Dependency Scope & Management"
    id: concept:page_937_bun
    description: |
      Extracted from documentation: Dependency Scope & Management
  - name: "Dependency Type & Versioning"
    id: concept:page_938_bun
    description: |
      Extracted from documentation: Dependency Type & Versioning
  - name: "Lockfile Control"
    id: concept:page_939_bun
    description: |
      Extracted from documentation: Lockfile Control
  - name: "Network & Registry Settings"
    id: concept:page_940_bun
    description: |
      Extracted from documentation: Network & Registry Settings
  - name: "Installation Process Control"
    id: concept:page_941_bun
    description: |
      Extracted from documentation: Installation Process Control
  - name: "Caching Options"
    id: concept:page_942_bun
    description: |
      Extracted from documentation: Caching Options
  - name: "Security & Integrity"
    id: concept:page_943_bun
    description: |
      Extracted from documentation: Security & Integrity
  - name: "Concurrency & Performance"
    id: concept:page_944_bun
    description: |
      Extracted from documentation: Concurrency & Performance
  - name: "Lifecycle Script Management"
    id: concept:page_945_bun
    description: |
      Extracted from documentation: Lifecycle Script Management
  - name: "Help Information"
    id: concept:page_946_bun
    description: |
      Extracted from documentation: Help Information
  - name: "bun link"
    id: concept:page_947_bun
    description: |
      Extracted from documentation: bun link
  - name: "Unlinking"
    id: concept:page_948_bun
    description: |
      Extracted from documentation: Unlinking
  - name: "Installation Scope"
    id: concept:page_949_bun
    description: |
      Extracted from documentation: Installation Scope
  - name: "Platform Targeting"
    id: concept:page_950_bun
    description: |
      Extracted from documentation: Platform Targeting
  - name: "bun outdated"
    id: concept:page_951_bun
    description: |
      Extracted from documentation: bun outdated
  - name: "Version Information"
    id: concept:page_952_bun
    description: |
      Extracted from documentation: Version Information
  - name: "Dependency Filters"
    id: concept:page_953_bun
    description: |
      Extracted from documentation: Dependency Filters
  - name: "Workspace Filters"
    id: concept:page_954_bun
    description: |
      Extracted from documentation: Workspace Filters
  - name: "General Options"
    id: concept:page_955_bun
    description: |
      Extracted from documentation: General Options
  - name: "Dependency Scope & Target"
    id: concept:page_956_bun
    description: |
      Extracted from documentation: Dependency Scope & Target
  - name: "Lockfile & Package.json"
    id: concept:page_957_bun
    description: |
      Extracted from documentation: Lockfile & Package.json
  - name: "Execution Behavior"
    id: concept:page_958_bun
    description: |
      Extracted from documentation: Execution Behavior
  - name: "bun patch"
    id: concept:page_959_bun
    description: |
      Extracted from documentation: bun patch
  - name: "Step 1. Prepare the package for patching"
    id: concept:page_960_bun
    description: |
      Extracted from documentation: Step 1. Prepare the package for patching
  - name: "you can supply the package name"
    id: concept:page_961_bun
    description: |
      Extracted from documentation: you can supply the package name
  - name: "...and a precise version in case multiple versions are installed"
    id: concept:page_962_bun
    description: |
      Extracted from documentation: ...and a precise version in case multiple versions are installed
  - name: "or the path to the package"
    id: concept:page_963_bun
    description: |
      Extracted from documentation: or the path to the package
  - name: "Step 2. Test your changes locally"
    id: concept:page_964_bun
    description: |
      Extracted from documentation: Step 2. Test your changes locally
  - name: "Step 3. Commit your changes"
    id: concept:page_965_bun
    description: |
      Extracted from documentation: Step 3. Commit your changes
  - name: "you can supply the path to the patched package"
    id: concept:page_966_bun
    description: |
      Extracted from documentation: you can supply the path to the patched package
  - name: "... or the package name and optionally the version"
    id: concept:page_967_bun
    description: |
      Extracted from documentation: ... or the package name and optionally the version
  - name: "choose the directory to store the patch files"
    id: concept:page_968_bun
    description: |
      Extracted from documentation: choose the directory to store the patch files
  - name: "`patch-commit` is available for compatibility with pnpm"
    id: concept:page_969_bun
    description: |
      Extracted from documentation: `patch-commit` is available for compatibility with pnpm
  - name: "Patch Generation"
    id: concept:page_970_bun
    description: |
      Extracted from documentation: Patch Generation
  - name: "bun pm"
    id: concept:page_971_bun
    description: |
      Extracted from documentation: bun pm
  - name: "pack"
    id: concept:page_972_bun
    description: |
      Extracted from documentation: pack
  - name: "Creates my-package-1.0.0.tgz in current directory"
    id: concept:page_973_bun
    description: |
      Extracted from documentation: Creates my-package-1.0.0.tgz in current directory
  - name: "Saves tarball in ./dist/ directory"
    id: concept:page_974_bun
    description: |
      Extracted from documentation: Saves tarball in ./dist/ directory
  - name: "Options"
    id: concept:page_975_bun
    description: |
      Extracted from documentation: Options
  - name: "Output Modes"
    id: concept:page_976_bun
    description: |
      Extracted from documentation: Output Modes
  - name: "bin"
    id: concept:page_977_bun
    description: |
      Extracted from documentation: bin
  - name: "ls"
    id: concept:page_978_bun
    description: |
      Extracted from documentation: ls
  - name: "whoami"
    id: concept:page_979_bun
    description: |
      Extracted from documentation: whoami
  - name: "hash"
    id: concept:page_980_bun
    description: |
      Extracted from documentation: hash
  - name: "cache"
    id: concept:page_981_bun
    description: |
      Extracted from documentation: cache
  - name: "migrate"
    id: concept:page_982_bun
    description: |
      Extracted from documentation: migrate
  - name: "untrusted"
    id: concept:page_983_bun
    description: |
      Extracted from documentation: untrusted
  - name: "trust"
    id: concept:page_984_bun
    description: |
      Extracted from documentation: trust
  - name: "default-trusted"
    id: concept:page_985_bun
    description: |
      Extracted from documentation: default-trusted
  - name: "version"
    id: concept:page_986_bun
    description: |
      Extracted from documentation: version
  - name: "pkg"
    id: concept:page_987_bun
    description: |
      Extracted from documentation: pkg
  - name: "get"
    id: concept:page_988_bun
    description: |
      Extracted from documentation: get
  - name: "set"
    id: concept:page_989_bun
    description: |
      Extracted from documentation: set
  - name: "delete"
    id: concept:page_990_bun
    description: |
      Extracted from documentation: delete
  - name: "fix"
    id: concept:page_991_bun
    description: |
      Extracted from documentation: fix
  - name: "bun publish"
    id: concept:page_992_bun
    description: |
      Extracted from documentation: bun publish
  - name: "Publishing the package from the current working directory"
    id: concept:page_993_bun
    description: |
      Extracted from documentation: Publishing the package from the current working directory
  - name: "`--access`"
    id: concept:page_994_bun
    description: |
      Extracted from documentation: `--access`
  - name: "`--tag`"
    id: concept:page_995_bun
    description: |
      Extracted from documentation: `--tag`
  - name: "`--dry-run`"
    id: concept:page_996_bun
    description: |
      Extracted from documentation: `--dry-run`
  - name: "`--tolerate-republish`"
    id: concept:page_997_bun
    description: |
      Extracted from documentation: `--tolerate-republish`
  - name: "`--gzip-level`"
    id: concept:page_998_bun
    description: |
      Extracted from documentation: `--gzip-level`
  - name: "`--auth-type`"
    id: concept:page_999_bun
    description: |
      Extracted from documentation: `--auth-type`
  - name: "`--otp`"
    id: concept:page_1000_bun
    description: |
      Extracted from documentation: `--otp`
  - name: "Publishing Options"
    id: concept:page_1001_bun
    description: |
      Extracted from documentation: Publishing Options
  - name: "Registry Configuration"
    id: concept:page_1002_bun
    description: |
      Extracted from documentation: Registry Configuration
  - name: "Custom Registry"
    id: concept:page_1003_bun
    description: |
      Extracted from documentation: Custom Registry
  - name: "SSL Certificates"
    id: concept:page_1004_bun
    description: |
      Extracted from documentation: SSL Certificates
  - name: "Script Control"
    id: concept:page_1005_bun
    description: |
      Extracted from documentation: Script Control
  - name: "File Management"
    id: concept:page_1006_bun
    description: |
      Extracted from documentation: File Management
  - name: "Output Control"
    id: concept:page_1007_bun
    description: |
      Extracted from documentation: Output Control
  - name: "bun remove"
    id: concept:page_1008_bun
    description: |
      Extracted from documentation: bun remove
  - name: "General Information"
    id: concept:page_1009_bun
    description: |
      Extracted from documentation: General Information
  - name: "Package.json Interaction"
    id: concept:page_1010_bun
    description: |
      Extracted from documentation: Package.json Interaction
  - name: "Lockfile Behavior"
    id: concept:page_1011_bun
    description: |
      Extracted from documentation: Lockfile Behavior
  - name: "Dependency Filtering"
    id: concept:page_1012_bun
    description: |
      Extracted from documentation: Dependency Filtering
  - name: "Execution Control & Validation"
    id: concept:page_1013_bun
    description: |
      Extracted from documentation: Execution Control & Validation
  - name: "Script Execution"
    id: concept:page_1014_bun
    description: |
      Extracted from documentation: Script Execution
  - name: "Scope & Path"
    id: concept:page_1015_bun
    description: |
      Extracted from documentation: Scope & Path
  - name: "Advanced & Performance"
    id: concept:page_1016_bun
    description: |
      Extracted from documentation: Advanced & Performance
  - name: "bun update"
    id: concept:page_1017_bun
    description: |
      Extracted from documentation: bun update
  - name: "`--interactive`"
    id: concept:page_1018_bun
    description: |
      Extracted from documentation: `--interactive`
  - name: "Interactive Interface"
    id: concept:page_1019_bun
    description: |
      Extracted from documentation: Interactive Interface
  - name: "Keyboard Controls"
    id: concept:page_1020_bun
    description: |
      Extracted from documentation: Keyboard Controls
  - name: "Visual Indicators"
    id: concept:page_1021_bun
    description: |
      Extracted from documentation: Visual Indicators
  - name: "Package Grouping"
    id: concept:page_1022_bun
    description: |
      Extracted from documentation: Package Grouping
  - name: "`--recursive`"
    id: concept:page_1023_bun
    description: |
      Extracted from documentation: `--recursive`
  - name: "`--latest`"
    id: concept:page_1024_bun
    description: |
      Extracted from documentation: `--latest`
  - name: "Update Strategy"
    id: concept:page_1025_bun
    description: |
      Extracted from documentation: Update Strategy
  - name: "Dependency Scope"
    id: concept:page_1026_bun
    description: |
      Extracted from documentation: Dependency Scope
  - name: "Project File Management"
    id: concept:page_1027_bun
    description: |
      Extracted from documentation: Project File Management
  - name: "Installation Controls"
    id: concept:page_1028_bun
    description: |
      Extracted from documentation: Installation Controls
  - name: "General & Environment"
    id: concept:page_1029_bun
    description: |
      Extracted from documentation: General & Environment
  - name: "bun why"
    id: concept:page_1030_bun
    description: |
      Extracted from documentation: bun why
  - name: "Understanding the Output"
    id: concept:page_1031_bun
    description: |
      Extracted from documentation: Understanding the Output
  - name: "bun --filter"
    id: concept:page_1032_bun
    description: |
      Extracted from documentation: bun --filter
  - name: "Matching"
    id: concept:page_1033_bun
    description: |
      Extracted from documentation: Matching
  - name: "Package Name `--filter <pattern>`"
    id: concept:page_1034_bun
    description: |
      Extracted from documentation: Package Name `--filter <pattern>`
  - name: "Package Path `--filter ./<glob>`"
    id: concept:page_1035_bun
    description: |
      Extracted from documentation: Package Path `--filter ./<glob>`
  - name: "`bun install` and `bun outdated`"
    id: concept:page_1036_bun
    description: |
      Extracted from documentation: `bun install` and `bun outdated`
  - name: "Install dependencies for packages in `./packages` (`pkg-a`, `pkg-b`, `pkg-c`)"
    id: concept:page_1037_bun
    description: |
      Extracted from documentation: Install dependencies for packages in `./packages` (`pkg-a`, `pkg-b`, `pkg-c`)
  - name: "Save as above, but exclude the root package.json"
    id: concept:page_1038_bun
    description: |
      Extracted from documentation: Save as above, but exclude the root package.json
  - name: "Display outdated dependencies for workspaces starting with `pkg-`"
    id: concept:page_1039_bun
    description: |
      Extracted from documentation: Display outdated dependencies for workspaces starting with `pkg-`
  - name: "Display outdated dependencies for only the root package.json"
    id: concept:page_1040_bun
    description: |
      Extracted from documentation: Display outdated dependencies for only the root package.json
  - name: "Running scripts with `--filter`"
    id: concept:page_1041_bun
    description: |
      Extracted from documentation: Running scripts with `--filter`
  - name: "in another terminal"
    id: concept:page_1042_bun
    description: |
      Extracted from documentation: in another terminal
  - name: "Running scripts in workspaces"
    id: concept:page_1043_bun
    description: |
      Extracted from documentation: Running scripts in workspaces
  - name: "Packages"
    id: concept:page_1044_bun
    description: |
      Extracted from documentation: Packages
  - name: "src/foo"
    id: concept:page_1045_bun
    description: |
      Extracted from documentation: src/foo
  - name: "src/bar"
    id: concept:page_1046_bun
    description: |
      Extracted from documentation: src/bar
  - name: "in src/bar: runs myscript in src/foo, no need to cd!"
    id: concept:page_1047_bun
    description: |
      Extracted from documentation: in src/bar: runs myscript in src/foo, no need to cd!
  - name: "Parallel and sequential mode"
    id: concept:page_1048_bun
    description: |
      Extracted from documentation: Parallel and sequential mode
  - name: "Run \"build\" in all matching packages concurrently"
    id: concept:page_1049_bun
    description: |
      Extracted from documentation: Run "build" in all matching packages concurrently
  - name: "Run \"build\" in all workspace packages sequentially"
    id: concept:page_1050_bun
    description: |
      Extracted from documentation: Run "build" in all workspace packages sequentially
  - name: "Run glob-matched scripts across all packages"
    id: concept:page_1051_bun
    description: |
      Extracted from documentation: Run glob-matched scripts across all packages
  - name: "Continue running even if one package's script fails"
    id: concept:page_1052_bun
    description: |
      Extracted from documentation: Continue running even if one package's script fails
  - name: "Run multiple scripts across all packages"
    id: concept:page_1053_bun
    description: |
      Extracted from documentation: Run multiple scripts across all packages
  - name: "Dependency Order"
    id: concept:page_1054_bun
    description: |
      Extracted from documentation: Dependency Order
  - name: "Global cache"
    id: concept:page_1055_bun
    description: |
      Extracted from documentation: Global cache
  - name: "the directory to use for the cache"
    id: concept:page_1056_bun
    description: |
      Extracted from documentation: the directory to use for the cache
  - name: "when true, don't load from the global cache."
    id: concept:page_1057_bun
    description: |
      Extracted from documentation: when true, don't load from the global cache.
  - name: "Bun may still write to node_modules/.cache"
    id: concept:page_1058_bun
    description: |
      Extracted from documentation: Bun may still write to node_modules/.cache
  - name: "when true, always resolve the latest versions from the registry"
    id: concept:page_1059_bun
    description: |
      Extracted from documentation: when true, always resolve the latest versions from the registry
  - name: "Minimizing re-downloads"
    id: concept:page_1060_bun
    description: |
      Extracted from documentation: Minimizing re-downloads
  - name: "Fast copying"
    id: concept:page_1061_bun
    description: |
      Extracted from documentation: Fast copying
  - name: "Saving disk space"
    id: concept:page_1062_bun
    description: |
      Extracted from documentation: Saving disk space
  - name: "Global virtual store"
    id: concept:page_1063_bun
    description: |
      Extracted from documentation: Global virtual store
  - name: "Enabling"
    id: concept:page_1064_bun
    description: |
      Extracted from documentation: Enabling
  - name: "Why it's fast"
    id: concept:page_1065_bun
    description: |
      Extracted from documentation: Why it's fast
  - name: "Benchmarks"
    id: concept:page_1066_bun
    description: |
      Extracted from documentation: Benchmarks
  - name: "Disk"
    id: concept:page_1067_bun
    description: |
      Extracted from documentation: Disk
  - name: "Real-world cold→warm"
    id: concept:page_1068_bun
    description: |
      Extracted from documentation: Real-world cold→warm
  - name: "Directory structure"
    id: concept:page_1069_bun
    description: |
      Extracted from documentation: Directory structure
  - name: "What stays project-local"
    id: concept:page_1070_bun
    description: |
      Extracted from documentation: What stays project-local
  - name: "Peer dependencies"
    id: concept:page_1071_bun
    description: |
      Extracted from documentation: Peer dependencies
  - name: "Tradeoffs"
    id: concept:page_1072_bun
    description: |
      Extracted from documentation: Tradeoffs
  - name: "Phantom-dependency fallback"
    id: concept:page_1073_bun
    description: |
      Extracted from documentation: Phantom-dependency fallback
  - name: "`node_modules` is mostly symlinks"
    id: concept:page_1074_bun
    description: |
      Extracted from documentation: `node_modules` is mostly symlinks
  - name: "Disk usage"
    id: concept:page_1075_bun
    description: |
      Extracted from documentation: Disk usage
  - name: "Concurrency"
    id: concept:page_1076_bun
    description: |
      Extracted from documentation: Concurrency
  - name: "Related documentation"
    id: concept:page_1077_bun
    description: |
      Extracted from documentation: Related documentation
  - name: "What are isolated installs?"
    id: concept:page_1078_bun
    description: |
      Extracted from documentation: What are isolated installs?
  - name: "Key benefits"
    id: concept:page_1079_bun
    description: |
      Extracted from documentation: Key benefits
  - name: "Using isolated installs"
    id: concept:page_1080_bun
    description: |
      Extracted from documentation: Using isolated installs
  - name: "Command line"
    id: concept:page_1081_bun
    description: |
      Extracted from documentation: Command line
  - name: "Use isolated installs"
    id: concept:page_1082_bun
    description: |
      Extracted from documentation: Use isolated installs
  - name: "Use traditional hoisted installs"
    id: concept:page_1083_bun
    description: |
      Extracted from documentation: Use traditional hoisted installs
  - name: "Configuration file"
    id: concept:page_1084_bun
    description: |
      Extracted from documentation: Configuration file
  - name: "Default behavior"
    id: concept:page_1085_bun
    description: |
      Extracted from documentation: Default behavior
  - name: "How isolated installs work"
    id: concept:page_1086_bun
    description: |
      Extracted from documentation: How isolated installs work
  - name: "Resolution algorithm"
    id: concept:page_1087_bun
    description: |
      Extracted from documentation: Resolution algorithm
  - name: "Workspace handling"
    id: concept:page_1088_bun
    description: |
      Extracted from documentation: Workspace handling
  - name: "Comparison with hoisted installs"
    id: concept:page_1089_bun
    description: |
      Extracted from documentation: Comparison with hoisted installs
  - name: "Advanced features"
    id: concept:page_1090_bun
    description: |
      Extracted from documentation: Advanced features
  - name: "Peer dependency handling"
    id: concept:page_1091_bun
    description: |
      Extracted from documentation: Peer dependency handling
  - name: "Package with peer dependencies creates specialized paths"
    id: concept:page_1092_bun
    description: |
      Extracted from documentation: Package with peer dependencies creates specialized paths
  - name: "Backend strategies"
    id: concept:page_1093_bun
    description: |
      Extracted from documentation: Backend strategies
  - name: "Debugging isolated installs"
    id: concept:page_1094_bun
    description: |
      Extracted from documentation: Debugging isolated installs
  - name: "Troubleshooting"
    id: concept:page_1095_bun
    description: |
      Extracted from documentation: Troubleshooting
  - name: "Compatibility issues"
    id: concept:page_1096_bun
    description: |
      Extracted from documentation: Compatibility issues
  - name: "Performance considerations"
    id: concept:page_1097_bun
    description: |
      Extracted from documentation: Performance considerations
  - name: "Migration guide"
    id: concept:page_1098_bun
    description: |
      Extracted from documentation: Migration guide
  - name: "From npm/Yarn"
    id: concept:page_1099_bun
    description: |
      Extracted from documentation: From npm/Yarn
  - name: "Remove existing node_modules and lockfiles"
    id: concept:page_1100_bun
    description: |
      Extracted from documentation: Remove existing node_modules and lockfiles
  - name: "Install with isolated linker"
    id: concept:page_1101_bun
    description: |
      Extracted from documentation: Install with isolated linker
  - name: "From pnpm"
    id: concept:page_1102_bun
    description: |
      Extracted from documentation: From pnpm
  - name: "Remove pnpm files"
    id: concept:page_1103_bun
    description: |
      Extracted from documentation: Remove pnpm files
  - name: "Install with Bun's isolated linker"
    id: concept:page_1104_bun
    description: |
      Extracted from documentation: Install with Bun's isolated linker
  - name: "When to use isolated installs"
    id: concept:page_1105_bun
    description: |
      Extracted from documentation: When to use isolated installs
  - name: "`postinstall`"
    id: concept:page_1106_bun
    description: |
      Extracted from documentation: `postinstall`
  - name: "`trustedDependencies`"
    id: concept:page_1107_bun
    description: |
      Extracted from documentation: `trustedDependencies`
  - name: "Behavior of the `trustedDependencies` field"
    id: concept:page_1108_bun
    description: |
      Extracted from documentation: Behavior of the `trustedDependencies` field
  - name: "`--ignore-scripts`"
    id: concept:page_1109_bun
    description: |
      Extracted from documentation: `--ignore-scripts`
  - name: "Should it be committed to git?"
    id: concept:page_1110_bun
    description: |
      Extracted from documentation: Should it be committed to git?
  - name: "Generate a lockfile without installing?"
    id: concept:page_1111_bun
    description: |
      Extracted from documentation: Generate a lockfile without installing?
  - name: "Can I opt out?"
    id: concept:page_1112_bun
    description: |
      Extracted from documentation: Can I opt out?
  - name: "whether to save a non-Bun lockfile alongside bun.lock"
    id: concept:page_1113_bun
    description: |
      Extracted from documentation: whether to save a non-Bun lockfile alongside bun.lock
  - name: "only \"yarn\" is supported"
    id: concept:page_1114_bun
    description: |
      Extracted from documentation: only "yarn" is supported
  - name: "Text-based lockfile"
    id: concept:page_1115_bun
    description: |
      Extracted from documentation: Text-based lockfile
  - name: "Automatic lockfile migration"
    id: concept:page_1116_bun
    description: |
      Extracted from documentation: Automatic lockfile migration
  - name: ".npmrc support"
    id: concept:page_1117_bun
    description: |
      Extracted from documentation: .npmrc support
  - name: "Supported options"
    id: concept:page_1118_bun
    description: |
      Extracted from documentation: Supported options
  - name: "Set the default registry"
    id: concept:page_1119_bun
    description: |
      Extracted from documentation: Set the default registry
  - name: "Set the registry for a specific scope"
    id: concept:page_1120_bun
    description: |
      Extracted from documentation: Set the registry for a specific scope
  - name: "Configure options for a specific registry"
    id: concept:page_1121_bun
    description: |
      Extracted from documentation: Configure options for a specific registry
  - name: "set an auth token for the registry"
    id: concept:page_1122_bun
    description: |
      Extracted from documentation: set an auth token for the registry
  - name: "${...} is a placeholder for environment variables"
    id: concept:page_1123_bun
    description: |
      Extracted from documentation: ${...} is a placeholder for environment variables
  - name: "or you could set a username and password"
    id: concept:page_1124_bun
    description: |
      Extracted from documentation: or you could set a username and password
  - name: "note that the password is base64 encoded"
    id: concept:page_1125_bun
    description: |
      Extracted from documentation: note that the password is base64 encoded
  - name: "or use _auth, which is your username and password"
    id: concept:page_1126_bun
    description: |
      Extracted from documentation: or use _auth, which is your username and password
  - name: "combined into a single string, which is then base 64 encoded"
    id: concept:page_1127_bun
    description: |
      Extracted from documentation: combined into a single string, which is then base 64 encoded
  - name: "`link-workspace-packages`: Control workspace package installation"
    id: concept:page_1128_bun
    description: |
      Extracted from documentation: `link-workspace-packages`: Control workspace package installation
  - name: "`save-exact`: Save exact versions"
    id: concept:page_1129_bun
    description: |
      Extracted from documentation: `save-exact`: Save exact versions
  - name: "`ignore-scripts`: Skip lifecycle scripts"
    id: concept:page_1130_bun
    description: |
      Extracted from documentation: `ignore-scripts`: Skip lifecycle scripts
  - name: "`dry-run`: Preview changes without installing"
    id: concept:page_1131_bun
    description: |
      Extracted from documentation: `dry-run`: Preview changes without installing
  - name: "`cache`: Configure cache directory"
    id: concept:page_1132_bun
    description: |
      Extracted from documentation: `cache`: Configure cache directory
  - name: "set a custom cache directory"
    id: concept:page_1133_bun
    description: |
      Extracted from documentation: set a custom cache directory
  - name: "or disable caching"
    id: concept:page_1134_bun
    description: |
      Extracted from documentation: or disable caching
  - name: "`ca` and `cafile`: Configure CA certificates"
    id: concept:page_1135_bun
    description: |
      Extracted from documentation: `ca` and `cafile`: Configure CA certificates
  - name: "single CA certificate"
    id: concept:page_1136_bun
    description: |
      Extracted from documentation: single CA certificate
  - name: "multiple CA certificates"
    id: concept:page_1137_bun
    description: |
      Extracted from documentation: multiple CA certificates
  - name: "or specify a path to a CA file"
    id: concept:page_1138_bun
    description: |
      Extracted from documentation: or specify a path to a CA file
  - name: "`omit` and `include`: Control dependency types"
    id: concept:page_1139_bun
    description: |
      Extracted from documentation: `omit` and `include`: Control dependency types
  - name: "omit dev dependencies"
    id: concept:page_1140_bun
    description: |
      Extracted from documentation: omit dev dependencies
  - name: "omit multiple types"
    id: concept:page_1141_bun
    description: |
      Extracted from documentation: omit multiple types
  - name: "include specific types (overrides omit)"
    id: concept:page_1142_bun
    description: |
      Extracted from documentation: include specific types (overrides omit)
  - name: "`install-strategy` and `node-linker`: Installation strategy"
    id: concept:page_1143_bun
    description: |
      Extracted from documentation: `install-strategy` and `node-linker`: Installation strategy
  - name: "flat node_modules structure (default)"
    id: concept:page_1144_bun
    description: |
      Extracted from documentation: flat node_modules structure (default)
  - name: "symlinked structure"
    id: concept:page_1145_bun
    description: |
      Extracted from documentation: symlinked structure
  - name: "symlinked/isolated mode"
    id: concept:page_1146_bun
    description: |
      Extracted from documentation: symlinked/isolated mode
  - name: "flat/hoisted mode"
    id: concept:page_1147_bun
    description: |
      Extracted from documentation: flat/hoisted mode
  - name: "`public-hoist-pattern` and `hoist-pattern`: Control hoisting"
    id: concept:page_1148_bun
    description: |
      Extracted from documentation: `public-hoist-pattern` and `hoist-pattern`: Control hoisting
  - name: "packages matching this pattern will be hoisted to the root"
    id: concept:page_1149_bun
    description: |
      Extracted from documentation: packages matching this pattern will be hoisted to the root
  - name: "multiple patterns"
    id: concept:page_1150_bun
    description: |
      Extracted from documentation: multiple patterns
  - name: "control general hoisting behavior"
    id: concept:page_1151_bun
    description: |
      Extracted from documentation: control general hoisting behavior
  - name: "`\"overrides\"`"
    id: concept:page_1152_bun
    description: |
      Extracted from documentation: `"overrides"`
  - name: "`\"resolutions\"`"
    id: concept:page_1153_bun
    description: |
      Extracted from documentation: `"resolutions"`
  - name: "Scopes and registries"
    id: concept:page_1154_bun
    description: |
      Extracted from documentation: Scopes and registries
  - name: "set a token"
    id: concept:page_1155_bun
    description: |
      Extracted from documentation: set a token
  - name: "set a username/password"
    id: concept:page_1156_bun
    description: |
      Extracted from documentation: set a username/password
  - name: "registry as string"
    id: concept:page_1157_bun
    description: |
      Extracted from documentation: registry as string
  - name: "registry with username/password"
    id: concept:page_1158_bun
    description: |
      Extracted from documentation: registry with username/password
  - name: "registry with token"
    id: concept:page_1159_bun
    description: |
      Extracted from documentation: registry with token
  - name: "`.npmrc`"
    id: concept:page_1160_bun
    description: |
      Extracted from documentation: `.npmrc`
  - name: "Security Scanner API"
    id: concept:page_1161_bun
    description: |
      Extracted from documentation: Security Scanner API
  - name: "Quick Start"
    id: concept:page_1162_bun
    description: |
      Extracted from documentation: Quick Start
  - name: "Security Levels"
    id: concept:page_1163_bun
    description: |
      Extracted from documentation: Security Levels
  - name: "Using Pre-built Scanners"
    id: concept:page_1164_bun
    description: |
      Extracted from documentation: Using Pre-built Scanners
  - name: "Installing a Scanner"
    id: concept:page_1165_bun
    description: |
      Extracted from documentation: Installing a Scanner
  - name: "Configuring the Scanner"
    id: concept:page_1166_bun
    description: |
      Extracted from documentation: Configuring the Scanner
  - name: "Enterprise Configuration"
    id: concept:page_1167_bun
    description: |
      Extracted from documentation: Enterprise Configuration
  - name: "This might go in ~/.bashrc, for example"
    id: concept:page_1168_bun
    description: |
      Extracted from documentation: This might go in ~/.bashrc, for example
  - name: "The scanner will now use these credentials automatically"
    id: concept:page_1169_bun
    description: |
      Extracted from documentation: The scanner will now use these credentials automatically
  - name: "Authoring your own scanner"
    id: concept:page_1170_bun
    description: |
      Extracted from documentation: Authoring your own scanner
  - name: "Install dependencies for all workspaces starting with `pkg-` except for `pkg-c`"
    id: concept:page_1171_bun
    description: |
      Extracted from documentation: Install dependencies for all workspaces starting with `pkg-` except for `pkg-c`
  - name: "Paths can also be used. This is equivalent to the command above."
    id: concept:page_1172_bun
    description: |
      Extracted from documentation: Paths can also be used. This is equivalent to the command above.
  - name: "Share versions with Catalogs"
    id: concept:page_1173_bun
    description: |
      Extracted from documentation: Share versions with Catalogs
  - name: "Benchmarking"
    id: concept:page_1174_bun
    description: |
      Extracted from documentation: Benchmarking
  - name: "Measuring time"
    id: concept:page_1175_bun
    description: |
      Extracted from documentation: Measuring time
  - name: "Benchmarking tools"
    id: concept:page_1176_bun
    description: |
      Extracted from documentation: Benchmarking tools
  - name: "Measuring memory usage"
    id: concept:page_1177_bun
    description: |
      Extracted from documentation: Measuring memory usage
  - name: "JavaScript heap stats"
    id: concept:page_1178_bun
    description: |
      Extracted from documentation: JavaScript heap stats
  - name: "Native heap stats"
    id: concept:page_1179_bun
    description: |
      Extracted from documentation: Native heap stats
  - name: "CPU profiling"
    id: concept:page_1180_bun
    description: |
      Extracted from documentation: CPU profiling
  - name: "Markdown output"
    id: concept:page_1181_bun
    description: |
      Extracted from documentation: Markdown output
  - name: "Heap profiling"
    id: concept:page_1182_bun
    description: |
      Extracted from documentation: Heap profiling
  - name: "Bindgen"
    id: concept:page_1183_bun
    description: |
      Extracted from documentation: Bindgen
  - name: "Creating JS Functions in Rust"
    id: concept:page_1184_bun
    description: |
      Extracted from documentation: Creating JS Functions in Rust
  - name: "Strings"
    id: concept:page_1185_bun
    description: |
      Extracted from documentation: Strings
  - name: "Function Variants"
    id: concept:page_1186_bun
    description: |
      Extracted from documentation: Function Variants
  - name: "`t.dictionary`"
    id: concept:page_1187_bun
    description: |
      Extracted from documentation: `t.dictionary`
  - name: "Enumerations"
    id: concept:page_1188_bun
    description: |
      Extracted from documentation: Enumerations
  - name: "[repr(u8)]"
    id: concept:page_1189_bun
    description: |
      Extracted from documentation: [repr(u8)]
  - name: "[derive(Copy, Clone, Eq, PartialEq)]"
    id: concept:page_1190_bun
    description: |
      Extracted from documentation: [derive(Copy, Clone, Eq, PartialEq)]
  - name: "`implNamespace`"
    id: concept:page_1191_bun
    description: |
      Extracted from documentation: `implNamespace`
  - name: "`t.oneOf`"
    id: concept:page_1192_bun
    description: |
      Extracted from documentation: `t.oneOf`
  - name: "Attributes"
    id: concept:page_1193_bun
    description: |
      Extracted from documentation: Attributes
  - name: "Integer Attributes"
    id: concept:page_1194_bun
    description: |
      Extracted from documentation: Integer Attributes
  - name: "Callbacks"
    id: concept:page_1195_bun
    description: |
      Extracted from documentation: Callbacks
  - name: "Classes"
    id: concept:page_1196_bun
    description: |
      Extracted from documentation: Classes
  - name: "Building Windows"
    id: concept:page_1197_bun
    description: |
      Extracted from documentation: Building Windows
  - name: "Prerequisites"
    id: concept:page_1198_bun
    description: |
      Extracted from documentation: Prerequisites
  - name: "Enable Scripts"
    id: concept:page_1199_bun
    description: |
      Extracted from documentation: Enable Scripts
  - name: "System Dependencies"
    id: concept:page_1200_bun
    description: |
      Extracted from documentation: System Dependencies
  - name: "scoop seems to be buggy if you install llvm and the rest at the same time"
    id: concept:page_1201_bun
    description: |
      Extracted from documentation: scoop seems to be buggy if you install llvm and the rest at the same time
  - name: "Download and install LLVM for ARM64"
    id: concept:page_1202_bun
    description: |
      Extracted from documentation: Download and install LLVM for ARM64
  - name: "Building"
    id: concept:page_1203_bun
    description: |
      Extracted from documentation: Building
  - name: "after the initial `bun run build` you can use the following to build"
    id: concept:page_1204_bun
    description: |
      Extracted from documentation: after the initial `bun run build` you can use the following to build
  - name: "Extra paths"
    id: concept:page_1205_bun
    description: |
      Extracted from documentation: Extra paths
  - name: "Tests"
    id: concept:page_1206_bun
    description: |
      Extracted from documentation: Tests
  - name: "Run the entire test suite with reporter"
    id: concept:page_1207_bun
    description: |
      Extracted from documentation: Run the entire test suite with reporter
  - name: "the package.json script \"test\" uses \"build/debug/bun-debug.exe\" by default"
    id: concept:page_1208_bun
    description: |
      Extracted from documentation: the package.json script "test" uses "build/debug/bun-debug.exe" by default
  - name: "Run an individual test file:"
    id: concept:page_1209_bun
    description: |
      Extracted from documentation: Run an individual test file:
  - name: ".rc file fails to build"
    id: concept:page_1210_bun
    description: |
      Extracted from documentation: .rc file fails to build
  - name: "failed to write output 'bun-debug.exe': permission denied"
    id: concept:page_1211_bun
    description: |
      Extracted from documentation: failed to write output 'bun-debug.exe': permission denied
  - name: "Cross-compiling from Linux"
    id: concept:page_1212_bun
    description: |
      Extracted from documentation: Cross-compiling from Linux
  - name: "clang-cl/lld-link look up SDK paths as \"Include\"/\"Lib\"; the splat writes"
    id: concept:page_1213_bun
    description: |
      Extracted from documentation: clang-cl/lld-link look up SDK paths as "Include"/"Lib"; the splat writes
  - name: "them lowercase, so alias both spellings (needs the same privileges as the"
    id: concept:page_1214_bun
    description: |
      Extracted from documentation: them lowercase, so alias both spellings (needs the same privileges as the
  - name: "splat — configure creates these itself when the directory is writable)."
    id: concept:page_1215_bun
    description: |
      Extracted from documentation: splat — configure creates these itself when the directory is writable).
  - name: "Debug builds"
    id: concept:page_1216_bun
    description: |
      Extracted from documentation: Debug builds
  - name: "Release builds"
    id: concept:page_1217_bun
    description: |
      Extracted from documentation: Release builds
  - name: "LTO"
    id: concept:page_1218_bun
    description: |
      Extracted from documentation: LTO
  - name: "Contributing"
    id: concept:page_1219_bun
    description: |
      Extracted from documentation: Contributing
  - name: "Using Nix (Alternative)"
    id: concept:page_1220_bun
    description: |
      Extracted from documentation: Using Nix (Alternative)
  - name: "Install Dependencies (Manual)"
    id: concept:page_1221_bun
    description: |
      Extracted from documentation: Install Dependencies (Manual)
  - name: "Optional: Install `ccache`"
    id: concept:page_1222_bun
    description: |
      Extracted from documentation: Optional: Install `ccache`
  - name: "For macOS"
    id: concept:page_1223_bun
    description: |
      Extracted from documentation: For macOS
  - name: "For Ubuntu/Debian"
    id: concept:page_1224_bun
    description: |
      Extracted from documentation: For Ubuntu/Debian
  - name: "For Arch"
    id: concept:page_1225_bun
    description: |
      Extracted from documentation: For Arch
  - name: "For Fedora"
    id: concept:page_1226_bun
    description: |
      Extracted from documentation: For Fedora
  - name: "For openSUSE"
    id: concept:page_1227_bun
    description: |
      Extracted from documentation: For openSUSE
  - name: "Install LLVM"
    id: concept:page_1228_bun
    description: |
      Extracted from documentation: Install LLVM
  - name: "LLVM has an automatic installation script that is compatible with all versions of Ubuntu"
    id: concept:page_1229_bun
    description: |
      Extracted from documentation: LLVM has an automatic installation script that is compatible with all versions of Ubuntu
  - name: "use fish_add_path if you're using fish"
    id: concept:page_1230_bun
    description: |
      Extracted from documentation: use fish_add_path if you're using fish
  - name: "use path+=\"$(brew --prefix llvm@21)/bin\" if you are using zsh"
    id: concept:page_1231_bun
    description: |
      Extracted from documentation: use path+="$(brew --prefix llvm@21)/bin" if you are using zsh
  - name: "Building Bun"
    id: concept:page_1232_bun
    description: |
      Extracted from documentation: Building Bun
  - name: "VSCode"
    id: concept:page_1233_bun
    description: |
      Extracted from documentation: VSCode
  - name: "Running debug builds"
    id: concept:page_1234_bun
    description: |
      Extracted from documentation: Running debug builds
  - name: "Code generation scripts"
    id: concept:page_1235_bun
    description: |
      Extracted from documentation: Code generation scripts
  - name: "Modifying ESM modules"
    id: concept:page_1236_bun
    description: |
      Extracted from documentation: Modifying ESM modules
  - name: "Release build"
    id: concept:page_1237_bun
    description: |
      Extracted from documentation: Release build
  - name: "Download release build from pull requests"
    id: concept:page_1238_bun
    description: |
      Extracted from documentation: Download release build from pull requests
  - name: "Viewing CI failures from the terminal"
    id: concept:page_1239_bun
    description: |
      Extracted from documentation: Viewing CI failures from the terminal
  - name: "AddressSanitizer"
    id: concept:page_1240_bun
    description: |
      Extracted from documentation: AddressSanitizer
  - name: "Building WebKit locally + Debug mode of JSC"
    id: concept:page_1241_bun
    description: |
      Extracted from documentation: Building WebKit locally + Debug mode of JSC
  - name: "Clone WebKit into ./vendor/WebKit"
    id: concept:page_1242_bun
    description: |
      Extracted from documentation: Clone WebKit into ./vendor/WebKit
  - name: "Check out the commit hash specified in WEBKIT_VERSION in scripts/build/deps/webkit.ts"
    id: concept:page_1243_bun
    description: |
      Extracted from documentation: Check out the commit hash specified in WEBKIT_VERSION in scripts/build/deps/webkit.ts
  - name: "Build bun with the local JSC build — this automatically configures and builds JSC"
    id: concept:page_1244_bun
    description: |
      Extracted from documentation: Build bun with the local JSC build — this automatically configures and builds JSC
  - name: "'span' file not found on Ubuntu"
    id: concept:page_1245_bun
    description: |
      Extracted from documentation: 'span' file not found on Ubuntu
  - name: "include <span>"
    id: concept:page_1246_bun
    description: |
      Extracted from documentation: include <span>
  - name: "If the above command fails with `Unable to locate package gcc-11` we need"
    id: concept:page_1247_bun
    description: |
      Extracted from documentation: If the above command fails with `Unable to locate package gcc-11` we need
  - name: "to add the APT repository"
    id: concept:page_1248_bun
    description: |
      Extracted from documentation: to add the APT repository
  - name: "Now run `apt install` again"
    id: concept:page_1249_bun
    description: |
      Extracted from documentation: Now run `apt install` again
  - name: "libarchive"
    id: concept:page_1250_bun
    description: |
      Extracted from documentation: libarchive
  - name: "macOS `library not found for -lSystem`"
    id: concept:page_1251_bun
    description: |
      Extracted from documentation: macOS `library not found for -lSystem`
  - name: "Cannot find `libatomic.a`"
    id: concept:page_1252_bun
    description: |
      Extracted from documentation: Cannot find `libatomic.a`
  - name: "Using bun-debug"
    id: concept:page_1253_bun
    description: |
      Extracted from documentation: Using bun-debug
  - name: "License"
    id: concept:page_1254_bun
    description: |
      Extracted from documentation: License
  - name: "JavaScriptCore"
    id: concept:page_1255_bun
    description: |
      Extracted from documentation: JavaScriptCore
  - name: "Linked libraries"
    id: concept:page_1256_bun
    description: |
      Extracted from documentation: Linked libraries
  - name: "Polyfills"
    id: concept:page_1257_bun
    description: |
      Extracted from documentation: Polyfills
  - name: "Additional credits"
    id: concept:page_1258_bun
    description: |
      Extracted from documentation: Additional credits
  - name: "Roadmap"
    id: concept:page_1259_bun
    description: |
      Extracted from documentation: Roadmap
  - name: "Run a script"
    id: concept:page_1260_bun
    description: |
      Extracted from documentation: Run a script
  - name: "Archive"
    id: concept:page_1261_bun
    description: |
      Extracted from documentation: Archive
  - name: "Creating Archives"
    id: concept:page_1262_bun
    description: |
      Extracted from documentation: Creating Archives
  - name: "Writing Archives to Disk"
    id: concept:page_1263_bun
    description: |
      Extracted from documentation: Writing Archives to Disk
  - name: "Getting Archive Bytes"
    id: concept:page_1264_bun
    description: |
      Extracted from documentation: Getting Archive Bytes
  - name: "Extracting Archives"
    id: concept:page_1265_bun
    description: |
      Extracted from documentation: Extracting Archives
  - name: "From Existing Archive Data"
    id: concept:page_1266_bun
    description: |
      Extracted from documentation: From Existing Archive Data
  - name: "Extracting to Disk"
    id: concept:page_1267_bun
    description: |
      Extracted from documentation: Extracting to Disk
  - name: "Filtering Extracted Files"
    id: concept:page_1268_bun
    description: |
      Extracted from documentation: Filtering Extracted Files
  - name: "Reading Archive Contents"
    id: concept:page_1269_bun
    description: |
      Extracted from documentation: Reading Archive Contents
  - name: "Get All Files"
    id: concept:page_1270_bun
    description: |
      Extracted from documentation: Get All Files
  - name: "Filtering with Glob Patterns"
    id: concept:page_1271_bun
    description: |
      Extracted from documentation: Filtering with Glob Patterns
  - name: "Compression"
    id: concept:page_1272_bun
    description: |
      Extracted from documentation: Compression
  - name: "Bundle Project Files"
    id: concept:page_1273_bun
    description: |
      Extracted from documentation: Bundle Project Files
  - name: "Extract and Process npm Package"
    id: concept:page_1274_bun
    description: |
      Extracted from documentation: Extract and Process npm Package
  - name: "Create Archive from Directory"
    id: concept:page_1275_bun
    description: |
      Extracted from documentation: Create Archive from Directory
  - name: "Auto-install"
    id: concept:page_1276_bun
    description: |
      Extracted from documentation: Auto-install
  - name: "Version resolution"
    id: concept:page_1277_bun
    description: |
      Extracted from documentation: Version resolution
  - name: "Cache behavior"
    id: concept:page_1278_bun
    description: |
      Extracted from documentation: Cache behavior
  - name: "Version specifiers"
    id: concept:page_1279_bun
    description: |
      Extracted from documentation: Version specifiers
  - name: "FAQ"
    id: concept:page_1280_bun
    description: |
      Extracted from documentation: FAQ
  - name: "Binary Data"
    id: concept:page_1281_bun
    description: |
      Extracted from documentation: Binary Data
  - name: "`ArrayBuffer` and views"
    id: concept:page_1282_bun
    description: |
      Extracted from documentation: `ArrayBuffer` and views
  - name: "`DataView`"
    id: concept:page_1283_bun
    description: |
      Extracted from documentation: `DataView`
  - name: "`TypedArray`"
    id: concept:page_1284_bun
    description: |
      Extracted from documentation: `TypedArray`
  - name: "`Uint8Array`"
    id: concept:page_1285_bun
    description: |
      Extracted from documentation: `Uint8Array`
  - name: "`Buffer`"
    id: concept:page_1286_bun
    description: |
      Extracted from documentation: `Buffer`
  - name: "`Blob`"
    id: concept:page_1287_bun
    description: |
      Extracted from documentation: `Blob`
  - name: "`BunFile`"
    id: concept:page_1288_bun
    description: |
      Extracted from documentation: `BunFile`
  - name: "`File`"
    id: concept:page_1289_bun
    description: |
      Extracted from documentation: `File`
  - name: "Streams"
    id: concept:page_1290_bun
    description: |
      Extracted from documentation: Streams
  - name: "Conversion"
    id: concept:page_1291_bun
    description: |
      Extracted from documentation: Conversion
  - name: "From `ArrayBuffer`"
    id: concept:page_1292_bun
    description: |
      Extracted from documentation: From `ArrayBuffer`
  - name: "To `TypedArray`"
    id: concept:page_1293_bun
    description: |
      Extracted from documentation: To `TypedArray`
  - name: "To `DataView`"
    id: concept:page_1294_bun
    description: |
      Extracted from documentation: To `DataView`
  - name: "To `Buffer`"
    id: concept:page_1295_bun
    description: |
      Extracted from documentation: To `Buffer`
  - name: "To `string`"
    id: concept:page_1296_bun
    description: |
      Extracted from documentation: To `string`
  - name: "To `number[]`"
    id: concept:page_1297_bun
    description: |
      Extracted from documentation: To `number[]`
  - name: "To `Blob`"
    id: concept:page_1298_bun
    description: |
      Extracted from documentation: To `Blob`
  - name: "To `ReadableStream`"
    id: concept:page_1299_bun
    description: |
      Extracted from documentation: To `ReadableStream`
  - name: "From `TypedArray`"
    id: concept:page_1300_bun
    description: |
      Extracted from documentation: From `TypedArray`
  - name: "To `ArrayBuffer`"
    id: concept:page_1301_bun
    description: |
      Extracted from documentation: To `ArrayBuffer`
  - name: "From `DataView`"
    id: concept:page_1302_bun
    description: |
      Extracted from documentation: From `DataView`
  - name: "From `Buffer`"
    id: concept:page_1303_bun
    description: |
      Extracted from documentation: From `Buffer`
  - name: "From `Blob`"
    id: concept:page_1304_bun
    description: |
      Extracted from documentation: From `Blob`
  - name: "From `ReadableStream`"
    id: concept:page_1305_bun
    description: |
      Extracted from documentation: From `ReadableStream`
  - name: "To `Uint8Array`"
    id: concept:page_1306_bun
    description: |
      Extracted from documentation: To `Uint8Array`
  - name: "Bun APIs"
    id: concept:page_1307_bun
    description: |
      Extracted from documentation: Bun APIs
  - name: "bunfig.toml"
    id: concept:page_1308_bun
    description: |
      Extracted from documentation: bunfig.toml
  - name: "Global vs. local"
    id: concept:page_1309_bun
    description: |
      Extracted from documentation: Global vs. local
  - name: "Runtime"
    id: concept:page_1310_bun
    description: |
      Extracted from documentation: Runtime
  - name: "`preload`"
    id: concept:page_1311_bun
    description: |
      Extracted from documentation: `preload`
  - name: "scripts to run before `bun run`-ing a file or script"
    id: concept:page_1312_bun
    description: |
      Extracted from documentation: scripts to run before `bun run`-ing a file or script
  - name: "register plugins by adding them to this list"
    id: concept:page_1313_bun
    description: |
      Extracted from documentation: register plugins by adding them to this list
  - name: "`smol`"
    id: concept:page_1314_bun
    description: |
      Extracted from documentation: `smol`
  - name: "Reduce memory usage at the cost of performance"
    id: concept:page_1315_bun
    description: |
      Extracted from documentation: Reduce memory usage at the cost of performance
  - name: "`logLevel`"
    id: concept:page_1316_bun
    description: |
      Extracted from documentation: `logLevel`
  - name: "`define`"
    id: concept:page_1317_bun
    description: |
      Extracted from documentation: `define`
  - name: "Replace any usage of \"process.env.bagel\" with the string `lox`."
    id: concept:page_1318_bun
    description: |
      Extracted from documentation: Replace any usage of "process.env.bagel" with the string `lox`.
  - name: "The values are parsed as JSON, except single-quoted strings are supported and `'undefined'` becomes `undefined` in JS."
    id: concept:page_1319_bun
    description: |
      Extracted from documentation: The values are parsed as JSON, except single-quoted strings are supported and `'undefined'` becomes `undefined` in JS.
  - name: "This will probably change in a future release to be just regular TOML instead. It is a holdover from the CLI argument parsing."
    id: concept:page_1320_bun
    description: |
      Extracted from documentation: This will probably change in a future release to be just regular TOML instead. It is a holdover from the CLI argument parsing.
  - name: "`loader`"
    id: concept:page_1321_bun
    description: |
      Extracted from documentation: `loader`
  - name: "when a .bagel file is imported, treat it like a tsx file"
    id: concept:page_1322_bun
    description: |
      Extracted from documentation: when a .bagel file is imported, treat it like a tsx file
  - name: "`telemetry`"
    id: concept:page_1323_bun
    description: |
      Extracted from documentation: `telemetry`
  - name: "`env`"
    id: concept:page_1324_bun
    description: |
      Extracted from documentation: `env`
  - name: "Disable automatic .env file loading"
    id: concept:page_1325_bun
    description: |
      Extracted from documentation: Disable automatic .env file loading
  - name: "`console`"
    id: concept:page_1326_bun
    description: |
      Extracted from documentation: `console`
  - name: "`console.depth`"
    id: concept:page_1327_bun
    description: |
      Extracted from documentation: `console.depth`
  - name: "Serve"
    id: concept:page_1328_bun
    description: |
      Extracted from documentation: Serve
  - name: "`serve.port`"
    id: concept:page_1329_bun
    description: |
      Extracted from documentation: `serve.port`
  - name: "Test runner"
    id: concept:page_1330_bun
    description: |
      Extracted from documentation: Test runner
  - name: "configuration goes here"
    id: concept:page_1331_bun
    description: |
      Extracted from documentation: configuration goes here
  - name: "`test.root`"
    id: concept:page_1332_bun
    description: |
      Extracted from documentation: `test.root`
  - name: "`test.preload`"
    id: concept:page_1333_bun
    description: |
      Extracted from documentation: `test.preload`
  - name: "`test.pathIgnorePatterns`"
    id: concept:page_1334_bun
    description: |
      Extracted from documentation: `test.pathIgnorePatterns`
  - name: "`test.smol`"
    id: concept:page_1335_bun
    description: |
      Extracted from documentation: `test.smol`
  - name: "`test.coverage`"
    id: concept:page_1336_bun
    description: |
      Extracted from documentation: `test.coverage`
  - name: "`test.coverageThreshold`"
    id: concept:page_1337_bun
    description: |
      Extracted from documentation: `test.coverageThreshold`
  - name: "`test.coverageSkipTestFiles`"
    id: concept:page_1338_bun
    description: |
      Extracted from documentation: `test.coverageSkipTestFiles`
  - name: "`test.coverageIgnoreSourcemaps`"
    id: concept:page_1339_bun
    description: |
      Extracted from documentation: `test.coverageIgnoreSourcemaps`
  - name: "`test.coveragePathIgnorePatterns`"
    id: concept:page_1340_bun
    description: |
      Extracted from documentation: `test.coveragePathIgnorePatterns`
  - name: "Single pattern"
    id: concept:page_1341_bun
    description: |
      Extracted from documentation: Single pattern
  - name: "Multiple patterns"
    id: concept:page_1342_bun
    description: |
      Extracted from documentation: Multiple patterns
  - name: "`test.coverageReporter`"
    id: concept:page_1343_bun
    description: |
      Extracted from documentation: `test.coverageReporter`
  - name: "`test.coverageDir`"
    id: concept:page_1344_bun
    description: |
      Extracted from documentation: `test.coverageDir`
  - name: "`test.randomize`"
    id: concept:page_1345_bun
    description: |
      Extracted from documentation: `test.randomize`
  - name: "`test.seed`"
    id: concept:page_1346_bun
    description: |
      Extracted from documentation: `test.seed`
  - name: "`test.rerunEach`"
    id: concept:page_1347_bun
    description: |
      Extracted from documentation: `test.rerunEach`
  - name: "`test.retry`"
    id: concept:page_1348_bun
    description: |
      Extracted from documentation: `test.retry`
  - name: "`test.concurrentTestGlob`"
    id: concept:page_1349_bun
    description: |
      Extracted from documentation: `test.concurrentTestGlob`
  - name: "`test.onlyFailures`"
    id: concept:page_1350_bun
    description: |
      Extracted from documentation: `test.onlyFailures`
  - name: "`test.reporter`"
    id: concept:page_1351_bun
    description: |
      Extracted from documentation: `test.reporter`
  - name: "`test.reporter.dots`"
    id: concept:page_1352_bun
    description: |
      Extracted from documentation: `test.reporter.dots`
  - name: "`test.reporter.junit`"
    id: concept:page_1353_bun
    description: |
      Extracted from documentation: `test.reporter.junit`
  - name: "Package manager"
    id: concept:page_1354_bun
    description: |
      Extracted from documentation: Package manager
  - name: "configuration here"
    id: concept:page_1355_bun
    description: |
      Extracted from documentation: configuration here
  - name: "`install.optional`"
    id: concept:page_1356_bun
    description: |
      Extracted from documentation: `install.optional`
  - name: "`install.dev`"
    id: concept:page_1357_bun
    description: |
      Extracted from documentation: `install.dev`
  - name: "`install.peer`"
    id: concept:page_1358_bun
    description: |
      Extracted from documentation: `install.peer`
  - name: "`install.production`"
    id: concept:page_1359_bun
    description: |
      Extracted from documentation: `install.production`
  - name: "`install.exact`"
    id: concept:page_1360_bun
    description: |
      Extracted from documentation: `install.exact`
  - name: "`install.ignoreScripts`"
    id: concept:page_1361_bun
    description: |
      Extracted from documentation: `install.ignoreScripts`
  - name: "`install.concurrentScripts`"
    id: concept:page_1362_bun
    description: |
      Extracted from documentation: `install.concurrentScripts`
  - name: "`install.saveTextLockfile`"
    id: concept:page_1363_bun
    description: |
      Extracted from documentation: `install.saveTextLockfile`
  - name: "`install.auto`"
    id: concept:page_1364_bun
    description: |
      Extracted from documentation: `install.auto`
  - name: "`install.prefer`"
    id: concept:page_1365_bun
    description: |
      Extracted from documentation: `install.prefer`
  - name: "`install.frozenLockfile`"
    id: concept:page_1366_bun
    description: |
      Extracted from documentation: `install.frozenLockfile`
  - name: "`install.dryRun`"
    id: concept:page_1367_bun
    description: |
      Extracted from documentation: `install.dryRun`
  - name: "`install.globalDir`"
    id: concept:page_1368_bun
    description: |
      Extracted from documentation: `install.globalDir`
  - name: "where `bun install --global` installs packages"
    id: concept:page_1369_bun
    description: |
      Extracted from documentation: where `bun install --global` installs packages
  - name: "`install.globalBinDir`"
    id: concept:page_1370_bun
    description: |
      Extracted from documentation: `install.globalBinDir`
  - name: "`install.registry`"
    id: concept:page_1371_bun
    description: |
      Extracted from documentation: `install.registry`
  - name: "`install.linkWorkspacePackages`"
    id: concept:page_1372_bun
    description: |
      Extracted from documentation: `install.linkWorkspacePackages`
  - name: "`install.scopes`"
    id: concept:page_1373_bun
    description: |
      Extracted from documentation: `install.scopes`
  - name: "`install.ca` and `install.cafile`"
    id: concept:page_1374_bun
    description: |
      Extracted from documentation: `install.ca` and `install.cafile`
  - name: "The CA certificate as a string"
    id: concept:page_1375_bun
    description: |
      Extracted from documentation: The CA certificate as a string
  - name: "A path to a CA certificate file. The file can contain multiple certificates."
    id: concept:page_1376_bun
    description: |
      Extracted from documentation: A path to a CA certificate file. The file can contain multiple certificates.
  - name: "`install.cache`"
    id: concept:page_1377_bun
    description: |
      Extracted from documentation: `install.cache`
  - name: "`install.lockfile`"
    id: concept:page_1378_bun
    description: |
      Extracted from documentation: `install.lockfile`
  - name: "`install.linker`"
    id: concept:page_1379_bun
    description: |
      Extracted from documentation: `install.linker`
  - name: "`install.globalStore`"
    id: concept:page_1380_bun
    description: |
      Extracted from documentation: `install.globalStore`
  - name: "`install.publicHoistPattern`"
    id: concept:page_1381_bun
    description: |
      Extracted from documentation: `install.publicHoistPattern`
  - name: "`install.hoistPattern`"
    id: concept:page_1382_bun
    description: |
      Extracted from documentation: `install.hoistPattern`
  - name: "`install.logLevel`"
    id: concept:page_1383_bun
    description: |
      Extracted from documentation: `install.logLevel`
  - name: "`install.security.scanner`"
    id: concept:page_1384_bun
    description: |
      Extracted from documentation: `install.security.scanner`
  - name: "`install.minimumReleaseAge`"
    id: concept:page_1385_bun
    description: |
      Extracted from documentation: `install.minimumReleaseAge`
  - name: "`install.minimumReleaseAgeExcludes`"
    id: concept:page_1386_bun
    description: |
      Extracted from documentation: `install.minimumReleaseAgeExcludes`
  - name: "These packages will bypass the 3-day minimum age requirement"
    id: concept:page_1387_bun
    description: |
      Extracted from documentation: These packages will bypass the 3-day minimum age requirement
  - name: "`bun run`"
    id: concept:page_1388_bun
    description: |
      Extracted from documentation: `bun run`
  - name: "`run.shell` - use the system shell or Bun's shell"
    id: concept:page_1389_bun
    description: |
      Extracted from documentation: `run.shell` - use the system shell or Bun's shell
  - name: "default outside of Windows"
    id: concept:page_1390_bun
    description: |
      Extracted from documentation: default outside of Windows
  - name: "default on Windows"
    id: concept:page_1391_bun
    description: |
      Extracted from documentation: default on Windows
  - name: "`run.bun` - auto alias `node` to `bun`"
    id: concept:page_1392_bun
    description: |
      Extracted from documentation: `run.bun` - auto alias `node` to `bun`
  - name: "equivalent to `bun --bun` for all `bun run` commands"
    id: concept:page_1393_bun
    description: |
      Extracted from documentation: equivalent to `bun --bun` for all `bun run` commands
  - name: "`run.silent` - suppress reporting the command being run"
    id: concept:page_1394_bun
    description: |
      Extracted from documentation: `run.silent` - suppress reporting the command being run
  - name: "`run.elide-lines` - truncate filtered output"
    id: concept:page_1395_bun
    description: |
      Extracted from documentation: `run.elide-lines` - truncate filtered output
  - name: "`run.noOrphans` - don't leave orphan processes behind"
    id: concept:page_1396_bun
    description: |
      Extracted from documentation: `run.noOrphans` - don't leave orphan processes behind
  - name: "C Compiler"
    id: concept:page_1397_bun
    description: |
      Extracted from documentation: C Compiler
  - name: "Usage (cc in `bun:ffi`)"
    id: concept:page_1398_bun
    description: |
      Extracted from documentation: Usage (cc in `bun:ffi`)
  - name: "Primitive types"
    id: concept:page_1399_bun
    description: |
      Extracted from documentation: Primitive types
  - name: "Strings, objects, and non-primitive types"
    id: concept:page_1400_bun
    description: |
      Extracted from documentation: Strings, objects, and non-primitive types
  - name: "Returning a C string to JavaScript"
    id: concept:page_1401_bun
    description: |
      Extracted from documentation: Returning a C string to JavaScript
  - name: "include <node/node_api.h>"
    id: concept:page_1402_bun
    description: |
      Extracted from documentation: include <node/node_api.h>
  - name: "`cc` Reference"
    id: concept:page_1403_bun
    description: |
      Extracted from documentation: `cc` Reference
  - name: "`library: string[]`"
    id: concept:page_1404_bun
    description: |
      Extracted from documentation: `library: string[]`
  - name: "`symbols`"
    id: concept:page_1405_bun
    description: |
      Extracted from documentation: `symbols`
  - name: "`source`"
    id: concept:page_1406_bun
    description: |
      Extracted from documentation: `source`
  - name: "`flags: string | string[]`"
    id: concept:page_1407_bun
    description: |
      Extracted from documentation: `flags: string | string[]`
  - name: "`define: Record<string, string>`"
    id: concept:page_1408_bun
    description: |
      Extracted from documentation: `define: Record<string, string>`
  - name: "Spawn"
    id: concept:page_1409_bun
    description: |
      Extracted from documentation: Spawn
  - name: "Spawn a process (`Bun.spawn()`)"
    id: concept:page_1410_bun
    description: |
      Extracted from documentation: Spawn a process (`Bun.spawn()`)
  - name: "Input stream"
    id: concept:page_1411_bun
    description: |
      Extracted from documentation: Input stream
  - name: "Output streams"
    id: concept:page_1412_bun
    description: |
      Extracted from documentation: Output streams
  - name: "Exit handling"
    id: concept:page_1413_bun
    description: |
      Extracted from documentation: Exit handling
  - name: "Resource usage"
    id: concept:page_1414_bun
    description: |
      Extracted from documentation: Resource usage
  - name: "Using AbortSignal"
    id: concept:page_1415_bun
    description: |
      Extracted from documentation: Using AbortSignal
  - name: "Using timeout and killSignal"
    id: concept:page_1416_bun
    description: |
      Extracted from documentation: Using timeout and killSignal
  - name: "Using maxBuffer"
    id: concept:page_1417_bun
    description: |
      Extracted from documentation: Using maxBuffer
  - name: "Inter-process communication (IPC)"
    id: concept:page_1418_bun
    description: |
      Extracted from documentation: Inter-process communication (IPC)
  - name: "IPC between Bun & Node.js"
    id: concept:page_1419_bun
    description: |
      Extracted from documentation: IPC between Bun & Node.js
  - name: "Terminal (PTY) support"
    id: concept:page_1420_bun
    description: |
      Extracted from documentation: Terminal (PTY) support
  - name: "Terminal options"
    id: concept:page_1421_bun
    description: |
      Extracted from documentation: Terminal options
  - name: "Terminal methods"
    id: concept:page_1422_bun
    description: |
      Extracted from documentation: Terminal methods
  - name: "Reusable Terminal"
    id: concept:page_1423_bun
    description: |
      Extracted from documentation: Reusable Terminal
  - name: "Platform differences"
    id: concept:page_1424_bun
    description: |
      Extracted from documentation: Platform differences
  - name: "Blocking API (`Bun.spawnSync()`)"
    id: concept:page_1425_bun
    description: |
      Extracted from documentation: Blocking API (`Bun.spawnSync()`)
  - name: "Color"
    id: concept:page_1426_bun
    description: |
      Extracted from documentation: Color
  - name: "Flexible input"
    id: concept:page_1427_bun
    description: |
      Extracted from documentation: Flexible input
  - name: "Format colors as CSS"
    id: concept:page_1428_bun
    description: |
      Extracted from documentation: Format colors as CSS
  - name: "Format colors as ANSI (for terminals)"
    id: concept:page_1429_bun
    description: |
      Extracted from documentation: Format colors as ANSI (for terminals)
  - name: "24-bit ANSI colors (`ansi-16m`)"
    id: concept:page_1430_bun
    description: |
      Extracted from documentation: 24-bit ANSI colors (`ansi-16m`)
  - name: "256 ANSI colors (`ansi-256`)"
    id: concept:page_1431_bun
    description: |
      Extracted from documentation: 256 ANSI colors (`ansi-256`)
  - name: "16 ANSI colors (`ansi-16`)"
    id: concept:page_1432_bun
    description: |
      Extracted from documentation: 16 ANSI colors (`ansi-16`)
  - name: "Format colors as numbers"
    id: concept:page_1433_bun
    description: |
      Extracted from documentation: Format colors as numbers
  - name: "Get the red, green, blue, and alpha channels"
    id: concept:page_1434_bun
    description: |
      Extracted from documentation: Get the red, green, blue, and alpha channels
  - name: "`{rgba}` object"
    id: concept:page_1435_bun
    description: |
      Extracted from documentation: `{rgba}` object
  - name: "`[rgba]` array"
    id: concept:page_1436_bun
    description: |
      Extracted from documentation: `[rgba]` array
  - name: "Format colors as hex strings"
    id: concept:page_1437_bun
    description: |
      Extracted from documentation: Format colors as hex strings
  - name: "Bundle-time client-side color formatting"
    id: concept:page_1438_bun
    description: |
      Extracted from documentation: Bundle-time client-side color formatting
  - name: "Console"
    id: concept:page_1439_bun
    description: |
      Extracted from documentation: Console
  - name: "Object inspection depth"
    id: concept:page_1440_bun
    description: |
      Extracted from documentation: Object inspection depth
  - name: "Reading from stdin"
    id: concept:page_1441_bun
    description: |
      Extracted from documentation: Reading from stdin
  - name: "Cookies"
    id: concept:page_1442_bun
    description: |
      Extracted from documentation: Cookies
  - name: "CookieMap class"
    id: concept:page_1443_bun
    description: |
      Extracted from documentation: CookieMap class
  - name: "In HTTP servers"
    id: concept:page_1444_bun
    description: |
      Extracted from documentation: In HTTP servers
  - name: "Methods"
    id: concept:page_1445_bun
    description: |
      Extracted from documentation: Methods
  - name: "`get(name: string): string | null`"
    id: concept:page_1446_bun
    description: |
      Extracted from documentation: `get(name: string): string | null`
  - name: "`has(name: string): boolean`"
    id: concept:page_1447_bun
    description: |
      Extracted from documentation: `has(name: string): boolean`
  - name: "`set(name: string, value: string): void`"
    id: concept:page_1448_bun
    description: |
      Extracted from documentation: `set(name: string, value: string): void`
  - name: "`set(options: CookieInit): void`"
    id: concept:page_1449_bun
    description: |
      Extracted from documentation: `set(options: CookieInit): void`
  - name: "`set(cookie: Cookie): void`"
    id: concept:page_1450_bun
    description: |
      Extracted from documentation: `set(cookie: Cookie): void`
  - name: "`delete(name: string): void`"
    id: concept:page_1451_bun
    description: |
      Extracted from documentation: `delete(name: string): void`
  - name: "`delete(options: CookieStoreDeleteOptions): void`"
    id: concept:page_1452_bun
    description: |
      Extracted from documentation: `delete(options: CookieStoreDeleteOptions): void`
  - name: "`toJSON(): Record<string, string>`"
    id: concept:page_1453_bun
    description: |
      Extracted from documentation: `toJSON(): Record<string, string>`
  - name: "`toSetCookieHeaders(): string[]`"
    id: concept:page_1454_bun
    description: |
      Extracted from documentation: `toSetCookieHeaders(): string[]`
  - name: "Iteration"
    id: concept:page_1455_bun
    description: |
      Extracted from documentation: Iteration
  - name: "Properties"
    id: concept:page_1456_bun
    description: |
      Extracted from documentation: Properties
  - name: "`size: number`"
    id: concept:page_1457_bun
    description: |
      Extracted from documentation: `size: number`
  - name: "Cookie class"
    id: concept:page_1458_bun
    description: |
      Extracted from documentation: Cookie class
  - name: "Constructors"
    id: concept:page_1459_bun
    description: |
      Extracted from documentation: Constructors
  - name: "`isExpired(): boolean`"
    id: concept:page_1460_bun
    description: |
      Extracted from documentation: `isExpired(): boolean`
  - name: "`serialize(): string`"
    id: concept:page_1461_bun
    description: |
      Extracted from documentation: `serialize(): string`
  - name: "`toString(): string`"
    id: concept:page_1462_bun
    description: |
      Extracted from documentation: `toString(): string`
  - name: "`toJSON(): CookieInit`"
    id: concept:page_1463_bun
    description: |
      Extracted from documentation: `toJSON(): CookieInit`
  - name: "Static methods"
    id: concept:page_1464_bun
    description: |
      Extracted from documentation: Static methods
  - name: "`Cookie.parse(cookieString: string): Cookie`"
    id: concept:page_1465_bun
    description: |
      Extracted from documentation: `Cookie.parse(cookieString: string): Cookie`
  - name: "`Cookie.from(name: string, value: string, options?: CookieInit): Cookie`"
    id: concept:page_1466_bun
    description: |
      Extracted from documentation: `Cookie.from(name: string, value: string, options?: CookieInit): Cookie`
  - name: "Types"
    id: concept:page_1467_bun
    description: |
      Extracted from documentation: Types
  - name: "Cron"
    id: concept:page_1468_bun
    description: |
      Extracted from documentation: Cron
  - name: "`Bun.cron.parse()`"
    id: concept:page_1469_bun
    description: |
      Extracted from documentation: `Bun.cron.parse()`
  - name: "Parameters"
    id: concept:page_1470_bun
    description: |
      Extracted from documentation: Parameters
  - name: "Returns"
    id: concept:page_1471_bun
    description: |
      Extracted from documentation: Returns
  - name: "Chaining calls"
    id: concept:page_1472_bun
    description: |
      Extracted from documentation: Chaining calls
  - name: "Cron expression syntax"
    id: concept:page_1473_bun
    description: |
      Extracted from documentation: Cron expression syntax
  - name: "Special characters"
    id: concept:page_1474_bun
    description: |
      Extracted from documentation: Special characters
  - name: "Named values"
    id: concept:page_1475_bun
    description: |
      Extracted from documentation: Named values
  - name: "Predefined nicknames"
    id: concept:page_1476_bun
    description: |
      Extracted from documentation: Predefined nicknames
  - name: "Time zone"
    id: concept:page_1477_bun
    description: |
      Extracted from documentation: Time zone
  - name: "Day-of-month and day-of-week interaction"
    id: concept:page_1478_bun
    description: |
      Extracted from documentation: Day-of-month and day-of-week interaction
  - name: "`Bun.cron(schedule, handler)` — in-process"
    id: concept:page_1479_bun
    description: |
      Extracted from documentation: `Bun.cron(schedule, handler)` — in-process
  - name: "No-overlap guarantee"
    id: concept:page_1480_bun
    description: |
      Extracted from documentation: No-overlap guarantee
  - name: "Error handling"
    id: concept:page_1481_bun
    description: |
      Extracted from documentation: Error handling
  - name: "`bun --hot`"
    id: concept:page_1482_bun
    description: |
      Extracted from documentation: `bun --hot`
  - name: "The `CronJob` handle"
    id: concept:page_1483_bun
    description: |
      Extracted from documentation: The `CronJob` handle
  - name: "Fake timers"
    id: concept:page_1484_bun
    description: |
      Extracted from documentation: Fake timers
  - name: "`Bun.cron(path, schedule, title)` — OS-level"
    id: concept:page_1485_bun
    description: |
      Extracted from documentation: `Bun.cron(path, schedule, title)` — OS-level
  - name: "The `scheduled()` handler"
    id: concept:page_1486_bun
    description: |
      Extracted from documentation: The `scheduled()` handler
  - name: "How it works per platform"
    id: concept:page_1487_bun
    description: |
      Extracted from documentation: How it works per platform
  - name: "systemd-based (Ubuntu, Fedora, Arch, etc.)"
    id: concept:page_1488_bun
    description: |
      Extracted from documentation: systemd-based (Ubuntu, Fedora, Arch, etc.)
  - name: "syslog-based (older systems)"
    id: concept:page_1489_bun
    description: |
      Extracted from documentation: syslog-based (older systems)
  - name: "Edit your crontab and remove the \"# bun-cron: <title>\" comment"
    id: concept:page_1490_bun
    description: |
      Extracted from documentation: Edit your crontab and remove the "# bun-cron: <title>" comment
  - name: "and the command line below it"
    id: concept:page_1491_bun
    description: |
      Extracted from documentation: and the command line below it
  - name: "Or remove ALL bun cron jobs at once by filtering them out:"
    id: concept:page_1492_bun
    description: |
      Extracted from documentation: Or remove ALL bun cron jobs at once by filtering them out:
  - name: "Unload the job from launchd"
    id: concept:page_1493_bun
    description: |
      Extracted from documentation: Unload the job from launchd
  - name: "Delete the plist file"
    id: concept:page_1494_bun
    description: |
      Extracted from documentation: Delete the plist file
  - name: "Example for a job titled \"weekly-report\":"
    id: concept:page_1495_bun
    description: |
      Extracted from documentation: Example for a job titled "weekly-report":
  - name: "User context"
    id: concept:page_1496_bun
    description: |
      Extracted from documentation: User context
  - name: "Trigger limit"
    id: concept:page_1497_bun
    description: |
      Extracted from documentation: Trigger limit
  - name: "Windows containers"
    id: concept:page_1498_bun
    description: |
      Extracted from documentation: Windows containers
  - name: "List all bun cron tasks"
    id: concept:page_1499_bun
    description: |
      Extracted from documentation: List all bun cron tasks
  - name: "Example:"
    id: concept:page_1500_bun
    description: |
      Extracted from documentation: Example:
  - name: "`Bun.cron.remove()`"
    id: concept:page_1501_bun
    description: |
      Extracted from documentation: `Bun.cron.remove()`
  - name: "CSRF Protection"
    id: concept:page_1502_bun
    description: |
      Extracted from documentation: CSRF Protection
  - name: "`Bun.CSRF.generate()`"
    id: concept:page_1503_bun
    description: |
      Extracted from documentation: `Bun.CSRF.generate()`
  - name: "`Bun.CSRF.verify()`"
    id: concept:page_1504_bun
    description: |
      Extracted from documentation: `Bun.CSRF.verify()`
  - name: "Using with `Bun.serve()`"
    id: concept:page_1505_bun
    description: |
      Extracted from documentation: Using with `Bun.serve()`
  - name: "Default secret"
    id: concept:page_1506_bun
    description: |
      Extracted from documentation: Default secret
  - name: "TypeScript"
    id: concept:page_1507_bun
    description: |
      Extracted from documentation: TypeScript
  - name: "Debugging JavaScript and TypeScript"
    id: concept:page_1508_bun
    description: |
      Extracted from documentation: Debugging JavaScript and TypeScript
  - name: "`--inspect`"
    id: concept:page_1509_bun
    description: |
      Extracted from documentation: `--inspect`
  - name: "`--inspect-brk`"
    id: concept:page_1510_bun
    description: |
      Extracted from documentation: `--inspect-brk`
  - name: "`--inspect-wait`"
    id: concept:page_1511_bun
    description: |
      Extracted from documentation: `--inspect-wait`
  - name: "Setting a port or URL for the debugger"
    id: concept:page_1512_bun
    description: |
      Extracted from documentation: Setting a port or URL for the debugger
  - name: "Debuggers"
    id: concept:page_1513_bun
    description: |
      Extracted from documentation: Debuggers
  - name: "`debug.bun.sh`"
    id: concept:page_1514_bun
    description: |
      Extracted from documentation: `debug.bun.sh`
  - name: "Visual Studio Code Debugger"
    id: concept:page_1515_bun
    description: |
      Extracted from documentation: Visual Studio Code Debugger
  - name: "Debugging Network Requests"
    id: concept:page_1516_bun
    description: |
      Extracted from documentation: Debugging Network Requests
  - name: "Print fetch & node:http requests as curl commands"
    id: concept:page_1517_bun
    description: |
      Extracted from documentation: Print fetch & node:http requests as curl commands
  - name: "Stacktraces & sourcemaps"
    id: concept:page_1518_bun
    description: |
      Extracted from documentation: Stacktraces & sourcemaps
  - name: "Syntax-highlighted source code preview"
    id: concept:page_1519_bun
    description: |
      Extracted from documentation: Syntax-highlighted source code preview
  - name: "V8 Stack Traces"
    id: concept:page_1520_bun
    description: |
      Extracted from documentation: V8 Stack Traces
  - name: "V8 Stack Trace API"
    id: concept:page_1521_bun
    description: |
      Extracted from documentation: V8 Stack Trace API
  - name: "`Error.prepareStackTrace`"
    id: concept:page_1522_bun
    description: |
      Extracted from documentation: `Error.prepareStackTrace`
  - name: "`Error.captureStackTrace(error, startFn)`"
    id: concept:page_1523_bun
    description: |
      Extracted from documentation: `Error.captureStackTrace(error, startFn)`
  - name: "Setting environment variables"
    id: concept:page_1524_bun
    description: |
      Extracted from documentation: Setting environment variables
  - name: "Manually specifying `.env` files"
    id: concept:page_1525_bun
    description: |
      Extracted from documentation: Manually specifying `.env` files
  - name: "Disabling automatic `.env` loading"
    id: concept:page_1526_bun
    description: |
      Extracted from documentation: Disabling automatic `.env` loading
  - name: "Disable loading .env files"
    id: concept:page_1527_bun
    description: |
      Extracted from documentation: Disable loading .env files
  - name: "Quotation marks"
    id: concept:page_1528_bun
    description: |
      Extracted from documentation: Quotation marks
  - name: "Expansion"
    id: concept:page_1529_bun
    description: |
      Extracted from documentation: Expansion
  - name: "`dotenv`"
    id: concept:page_1530_bun
    description: |
      Extracted from documentation: `dotenv`
  - name: "Reading environment variables"
    id: concept:page_1531_bun
    description: |
      Extracted from documentation: Reading environment variables
  - name: "Configuring Bun"
    id: concept:page_1532_bun
    description: |
      Extracted from documentation: Configuring Bun
  - name: "Runtime transpiler caching"
    id: concept:page_1533_bun
    description: |
      Extracted from documentation: Runtime transpiler caching
  - name: "Disable the runtime transpiler cache"
    id: concept:page_1534_bun
    description: |
      Extracted from documentation: Disable the runtime transpiler cache
  - name: "What does it cache?"
    id: concept:page_1535_bun
    description: |
      Extracted from documentation: What does it cache?
  - name: "FFI"
    id: concept:page_1536_bun
    description: |
      Extracted from documentation: FFI
  - name: "dlopen usage (`bun:ffi`)"
    id: concept:page_1537_bun
    description: |
      Extracted from documentation: dlopen usage (`bun:ffi`)
  - name: "Zig"
    id: concept:page_1538_bun
    description: |
      Extracted from documentation: Zig
  - name: "Rust"
    id: concept:page_1539_bun
    description: |
      Extracted from documentation: Rust
  - name: "[no_mangle]"
    id: concept:page_1540_bun
    description: |
      Extracted from documentation: [no_mangle]
  - name: "C++"
    id: concept:page_1541_bun
    description: |
      Extracted from documentation: C++
  - name: "include <cstdint>"
    id: concept:page_1542_bun
    description: |
      Extracted from documentation: include <cstdint>
  - name: "FFI types"
    id: concept:page_1543_bun
    description: |
      Extracted from documentation: FFI types
  - name: "Function pointers"
    id: concept:page_1544_bun
    description: |
      Extracted from documentation: Function pointers
  - name: "Experimental thread-safe callbacks"
    id: concept:page_1545_bun
    description: |
      Extracted from documentation: Experimental thread-safe callbacks
  - name: "Pointers"
    id: concept:page_1546_bun
    description: |
      Extracted from documentation: Pointers
  - name: "Memory management"
    id: concept:page_1547_bun
    description: |
      Extracted from documentation: Memory management
  - name: "From JavaScript"
    id: concept:page_1548_bun
    description: |
      Extracted from documentation: From JavaScript
  - name: "From C, Rust, Zig, etc"
    id: concept:page_1549_bun
    description: |
      Extracted from documentation: From C, Rust, Zig, etc
  - name: "Memory safety"
    id: concept:page_1550_bun
    description: |
      Extracted from documentation: Memory safety
  - name: "Pointer alignment"
    id: concept:page_1551_bun
    description: |
      Extracted from documentation: Pointer alignment
  - name: "Passing a pointer"
    id: concept:page_1552_bun
    description: |
      Extracted from documentation: Passing a pointer
  - name: "Reading pointers"
    id: concept:page_1553_bun
    description: |
      Extracted from documentation: Reading pointers
  - name: "File I/O"
    id: concept:page_1554_bun
    description: |
      Extracted from documentation: File I/O
  - name: "Reading files (`Bun.file()`)"
    id: concept:page_1555_bun
    description: |
      Extracted from documentation: Reading files (`Bun.file()`)
  - name: "Deleting files (`file.delete()`)"
    id: concept:page_1556_bun
    description: |
      Extracted from documentation: Deleting files (`file.delete()`)
  - name: "Writing files (`Bun.write()`)"
    id: concept:page_1557_bun
    description: |
      Extracted from documentation: Writing files (`Bun.write()`)
  - name: "Incremental writing with `FileSink`"
    id: concept:page_1558_bun
    description: |
      Extracted from documentation: Incremental writing with `FileSink`
  - name: "Directories"
    id: concept:page_1559_bun
    description: |
      Extracted from documentation: Directories
  - name: "Reading directories (readdir)"
    id: concept:page_1560_bun
    description: |
      Extracted from documentation: Reading directories (readdir)
  - name: "Reading directories recursively"
    id: concept:page_1561_bun
    description: |
      Extracted from documentation: Reading directories recursively
  - name: "Creating directories (mkdir)"
    id: concept:page_1562_bun
    description: |
      Extracted from documentation: Creating directories (mkdir)
  - name: "File System Router"
    id: concept:page_1563_bun
    description: |
      Extracted from documentation: File System Router
  - name: "Next.js-style"
    id: concept:page_1564_bun
    description: |
      Extracted from documentation: Next.js-style
  - name: "File Types"
    id: concept:page_1565_bun
    description: |
      Extracted from documentation: File Types
  - name: "`json5`"
    id: concept:page_1566_bun
    description: |
      Extracted from documentation: `json5`
  - name: "`sh` loader"
    id: concept:page_1567_bun
    description: |
      Extracted from documentation: `sh` loader
  - name: "Glob"
    id: concept:page_1568_bun
    description: |
      Extracted from documentation: Glob
  - name: "Supported Glob Patterns"
    id: concept:page_1569_bun
    description: |
      Extracted from documentation: Supported Glob Patterns
  - name: "`?` - Match any single character"
    id: concept:page_1570_bun
    description: |
      Extracted from documentation: `?` - Match any single character
  - name: "`*` - Matches zero or more characters, except for path separators (`/` or `\\`)"
    id: concept:page_1571_bun
    description: |
      Extracted from documentation: `*` - Matches zero or more characters, except for path separators (`/` or `\`)
  - name: "`**` - Match any number of characters including `/`"
    id: concept:page_1572_bun
    description: |
      Extracted from documentation: `**` - Match any number of characters including `/`
  - name: "`[ab]` - Matches one of the characters contained in the brackets, as well as character ranges"
    id: concept:page_1573_bun
    description: |
      Extracted from documentation: `[ab]` - Matches one of the characters contained in the brackets, as well as character ranges
  - name: "`{a,b,c}` - Match any of the given patterns"
    id: concept:page_1574_bun
    description: |
      Extracted from documentation: `{a,b,c}` - Match any of the given patterns
  - name: "`!` - Negates the result at the start of a pattern"
    id: concept:page_1575_bun
    description: |
      Extracted from documentation: `!` - Negates the result at the start of a pattern
  - name: "`\\` - Escapes any of the special characters above"
    id: concept:page_1576_bun
    description: |
      Extracted from documentation: `\` - Escapes any of the special characters above
  - name: "Node.js `fs.glob()` compatibility"
    id: concept:page_1577_bun
    description: |
      Extracted from documentation: Node.js `fs.glob()` compatibility
  - name: "Globals"
    id: concept:page_1578_bun
    description: |
      Extracted from documentation: Globals
  - name: "Hashing"
    id: concept:page_1579_bun
    description: |
      Extracted from documentation: Hashing
  - name: "`Bun.password`"
    id: concept:page_1580_bun
    description: |
      Extracted from documentation: `Bun.password`
  - name: "Salt"
    id: concept:page_1581_bun
    description: |
      Extracted from documentation: Salt
  - name: "bcrypt - Modular Crypt Format"
    id: concept:page_1582_bun
    description: |
      Extracted from documentation: bcrypt - Modular Crypt Format
  - name: "argon2 - PHC format"
    id: concept:page_1583_bun
    description: |
      Extracted from documentation: argon2 - PHC format
  - name: "`Bun.hash`"
    id: concept:page_1584_bun
    description: |
      Extracted from documentation: `Bun.hash`
  - name: "`Bun.CryptoHasher`"
    id: concept:page_1585_bun
    description: |
      Extracted from documentation: `Bun.CryptoHasher`
  - name: "HMAC in `Bun.CryptoHasher`"
    id: concept:page_1586_bun
    description: |
      Extracted from documentation: HMAC in `Bun.CryptoHasher`
  - name: "HTMLRewriter"
    id: concept:page_1587_bun
    description: |
      Extracted from documentation: HTMLRewriter
  - name: "Input types"
    id: concept:page_1588_bun
    description: |
      Extracted from documentation: Input types
  - name: "Element Handlers"
    id: concept:page_1589_bun
    description: |
      Extracted from documentation: Element Handlers
  - name: "CSS Selector Support"
    id: concept:page_1590_bun
    description: |
      Extracted from documentation: CSS Selector Support
  - name: "Element Operations"
    id: concept:page_1591_bun
    description: |
      Extracted from documentation: Element Operations
  - name: "Text Operations"
    id: concept:page_1592_bun
    description: |
      Extracted from documentation: Text Operations
  - name: "Comment Operations"
    id: concept:page_1593_bun
    description: |
      Extracted from documentation: Comment Operations
  - name: "Document Handlers"
    id: concept:page_1594_bun
    description: |
      Extracted from documentation: Document Handlers
  - name: "Response Handling"
    id: concept:page_1595_bun
    description: |
      Extracted from documentation: Response Handling
  - name: "Reading cookies"
    id: concept:page_1596_bun
    description: |
      Extracted from documentation: Reading cookies
  - name: "Setting cookies"
    id: concept:page_1597_bun
    description: |
      Extracted from documentation: Setting cookies
  - name: "Deleting cookies"
    id: concept:page_1598_bun
    description: |
      Extracted from documentation: Deleting cookies
  - name: "`error` callback"
    id: concept:page_1599_bun
    description: |
      Extracted from documentation: `error` callback
  - name: "Metrics"
    id: concept:page_1600_bun
    description: |
      Extracted from documentation: Metrics
  - name: "`server.pendingRequests` and `server.pendingWebSockets`"
    id: concept:page_1601_bun
    description: |
      Extracted from documentation: `server.pendingRequests` and `server.pendingWebSockets`
  - name: "`server.subscriberCount(topic)`"
    id: concept:page_1602_bun
    description: |
      Extracted from documentation: `server.subscriberCount(topic)`
  - name: "Routing"
    id: concept:page_1603_bun
    description: |
      Extracted from documentation: Routing
  - name: "Basic Setup"
    id: concept:page_1604_bun
    description: |
      Extracted from documentation: Basic Setup
  - name: "Asynchronous Routes"
    id: concept:page_1605_bun
    description: |
      Extracted from documentation: Asynchronous Routes
  - name: "Async/await"
    id: concept:page_1606_bun
    description: |
      Extracted from documentation: Async/await
  - name: "Promise"
    id: concept:page_1607_bun
    description: |
      Extracted from documentation: Promise
  - name: "Route precedence"
    id: concept:page_1608_bun
    description: |
      Extracted from documentation: Route precedence
  - name: "Type-safe route parameters"
    id: concept:page_1609_bun
    description: |
      Extracted from documentation: Type-safe route parameters
  - name: "Static responses"
    id: concept:page_1610_bun
    description: |
      Extracted from documentation: Static responses
  - name: "File Responses vs Static Responses"
    id: concept:page_1611_bun
    description: |
      Extracted from documentation: File Responses vs Static Responses
  - name: "Streaming files"
    id: concept:page_1612_bun
    description: |
      Extracted from documentation: Streaming files
  - name: "`fetch` request handler"
    id: concept:page_1613_bun
    description: |
      Extracted from documentation: `fetch` request handler
  - name: "Server"
    id: concept:page_1614_bun
    description: |
      Extracted from documentation: Server
  - name: "HTML imports"
    id: concept:page_1615_bun
    description: |
      Extracted from documentation: HTML imports
  - name: "Changing the `port` and `hostname`"
    id: concept:page_1616_bun
    description: |
      Extracted from documentation: Changing the `port` and `hostname`
  - name: "Configuring a default port"
    id: concept:page_1617_bun
    description: |
      Extracted from documentation: Configuring a default port
  - name: "Unix domain sockets"
    id: concept:page_1618_bun
    description: |
      Extracted from documentation: Unix domain sockets
  - name: "Abstract namespace sockets"
    id: concept:page_1619_bun
    description: |
      Extracted from documentation: Abstract namespace sockets
  - name: "HTTP/3 (QUIC)"
    id: concept:page_1620_bun
    description: |
      Extracted from documentation: HTTP/3 (QUIC)
  - name: "idleTimeout"
    id: concept:page_1621_bun
    description: |
      Extracted from documentation: idleTimeout
  - name: "export default syntax"
    id: concept:page_1622_bun
    description: |
      Extracted from documentation: export default syntax
  - name: "Hot Route Reloading"
    id: concept:page_1623_bun
    description: |
      Extracted from documentation: Hot Route Reloading
  - name: "Server Lifecycle Methods"
    id: concept:page_1624_bun
    description: |
      Extracted from documentation: Server Lifecycle Methods
  - name: "`server.stop()`"
    id: concept:page_1625_bun
    description: |
      Extracted from documentation: `server.stop()`
  - name: "`server.ref()` and `server.unref()`"
    id: concept:page_1626_bun
    description: |
      Extracted from documentation: `server.ref()` and `server.unref()`
  - name: "`server.reload()`"
    id: concept:page_1627_bun
    description: |
      Extracted from documentation: `server.reload()`
  - name: "Per-Request Controls"
    id: concept:page_1628_bun
    description: |
      Extracted from documentation: Per-Request Controls
  - name: "`server.timeout(Request, seconds)`"
    id: concept:page_1629_bun
    description: |
      Extracted from documentation: `server.timeout(Request, seconds)`
  - name: "`server.requestIP(Request)`"
    id: concept:page_1630_bun
    description: |
      Extracted from documentation: `server.requestIP(Request)`
  - name: "Server Metrics"
    id: concept:page_1631_bun
    description: |
      Extracted from documentation: Server Metrics
  - name: "Practical example: REST API"
    id: concept:page_1632_bun
    description: |
      Extracted from documentation: Practical example: REST API
  - name: "TLS"
    id: concept:page_1633_bun
    description: |
      Extracted from documentation: TLS
  - name: "Passphrase"
    id: concept:page_1634_bun
    description: |
      Extracted from documentation: Passphrase
  - name: "CA Certificates"
    id: concept:page_1635_bun
    description: |
      Extracted from documentation: CA Certificates
  - name: "Diffie-Hellman"
    id: concept:page_1636_bun
    description: |
      Extracted from documentation: Diffie-Hellman
  - name: "Server name indication (SNI)"
    id: concept:page_1637_bun
    description: |
      Extracted from documentation: Server name indication (SNI)
  - name: "WebSockets"
    id: concept:page_1638_bun
    description: |
      Extracted from documentation: WebSockets
  - name: "Start a WebSocket server"
    id: concept:page_1639_bun
    description: |
      Extracted from documentation: Start a WebSocket server
  - name: "Sending messages"
    id: concept:page_1640_bun
    description: |
      Extracted from documentation: Sending messages
  - name: "Headers"
    id: concept:page_1641_bun
    description: |
      Extracted from documentation: Headers
  - name: "Contextual data"
    id: concept:page_1642_bun
    description: |
      Extracted from documentation: Contextual data
  - name: "Pub/Sub"
    id: concept:page_1643_bun
    description: |
      Extracted from documentation: Pub/Sub
  - name: "Backpressure"
    id: concept:page_1644_bun
    description: |
      Extracted from documentation: Backpressure
  - name: "Timeouts and limits"
    id: concept:page_1645_bun
    description: |
      Extracted from documentation: Timeouts and limits
  - name: "Connect to a `Websocket` server"
    id: concept:page_1646_bun
    description: |
      Extracted from documentation: Connect to a `Websocket` server
  - name: "Image"
    id: concept:page_1647_bun
    description: |
      Extracted from documentation: Image
  - name: "Input"
    id: concept:page_1648_bun
    description: |
      Extracted from documentation: Input
  - name: "Metadata"
    id: concept:page_1649_bun
    description: |
      Extracted from documentation: Metadata
  - name: "Resize"
    id: concept:page_1650_bun
    description: |
      Extracted from documentation: Resize
  - name: "Rotate · flip"
    id: concept:page_1651_bun
    description: |
      Extracted from documentation: Rotate · flip
  - name: "Modulate"
    id: concept:page_1652_bun
    description: |
      Extracted from documentation: Modulate
  - name: "Output formats"
    id: concept:page_1653_bun
    description: |
      Extracted from documentation: Output formats
  - name: "Terminals"
    id: concept:page_1654_bun
    description: |
      Extracted from documentation: Terminals
  - name: "Placeholders"
    id: concept:page_1655_bun
    description: |
      Extracted from documentation: Placeholders
  - name: "`Bun.serve` integration"
    id: concept:page_1656_bun
    description: |
      Extracted from documentation: `Bun.serve` integration
  - name: "Clipboard"
    id: concept:page_1657_bun
    description: |
      Extracted from documentation: Clipboard
  - name: "Platform backends"
    id: concept:page_1658_bun
    description: |
      Extracted from documentation: Platform backends
  - name: "Run a file"
    id: concept:page_1659_bun
    description: |
      Extracted from documentation: Run a file
  - name: "`--watch`"
    id: concept:page_1660_bun
    description: |
      Extracted from documentation: `--watch`
  - name: "Run a `package.json` script"
    id: concept:page_1661_bun
    description: |
      Extracted from documentation: Run a `package.json` script
  - name: "`--bun`"
    id: concept:page_1662_bun
    description: |
      Extracted from documentation: `--bun`
  - name: "Filtering"
    id: concept:page_1663_bun
    description: |
      Extracted from documentation: Filtering
  - name: "`bun run -` to pipe code from stdin"
    id: concept:page_1664_bun
    description: |
      Extracted from documentation: `bun run -` to pipe code from stdin
  - name: "`bun run --console-depth`"
    id: concept:page_1665_bun
    description: |
      Extracted from documentation: `bun run --console-depth`
  - name: "`bun run --smol`"
    id: concept:page_1666_bun
    description: |
      Extracted from documentation: `bun run --smol`
  - name: "Resolution order"
    id: concept:page_1667_bun
    description: |
      Extracted from documentation: Resolution order
  - name: "General Execution Options"
    id: concept:page_1668_bun
    description: |
      Extracted from documentation: General Execution Options
  - name: "Workspace Management"
    id: concept:page_1669_bun
    description: |
      Extracted from documentation: Workspace Management
  - name: "Runtime & Process Control"
    id: concept:page_1670_bun
    description: |
      Extracted from documentation: Runtime & Process Control
  - name: "Development Workflow"
    id: concept:page_1671_bun
    description: |
      Extracted from documentation: Development Workflow
  - name: "Dependency & Module Resolution"
    id: concept:page_1672_bun
    description: |
      Extracted from documentation: Dependency & Module Resolution
  - name: "Transpilation & Language Features"
    id: concept:page_1673_bun
    description: |
      Extracted from documentation: Transpilation & Language Features
  - name: "Networking & Security"
    id: concept:page_1674_bun
    description: |
      Extracted from documentation: Networking & Security
  - name: "JSON5"
    id: concept:page_1675_bun
    description: |
      Extracted from documentation: JSON5
  - name: "Conformance"
    id: concept:page_1676_bun
    description: |
      Extracted from documentation: Conformance
  - name: "Runtime API"
    id: concept:page_1677_bun
    description: |
      Extracted from documentation: Runtime API
  - name: "`Bun.JSON5.parse()`"
    id: concept:page_1678_bun
    description: |
      Extracted from documentation: `Bun.JSON5.parse()`
  - name: "Supported JSON5 Features"
    id: concept:page_1679_bun
    description: |
      Extracted from documentation: Supported JSON5 Features
  - name: "`Bun.JSON5.stringify()`"
    id: concept:page_1680_bun
    description: |
      Extracted from documentation: `Bun.JSON5.stringify()`
  - name: "Pretty Printing"
    id: concept:page_1681_bun
    description: |
      Extracted from documentation: Pretty Printing
  - name: "Special Values"
    id: concept:page_1682_bun
    description: |
      Extracted from documentation: Special Values
  - name: "Module Import"
    id: concept:page_1683_bun
    description: |
      Extracted from documentation: Module Import
  - name: "ES Modules"
    id: concept:page_1684_bun
    description: |
      Extracted from documentation: ES Modules
  - name: "Default Import"
    id: concept:page_1685_bun
    description: |
      Extracted from documentation: Default Import
  - name: "Named Imports"
    id: concept:page_1686_bun
    description: |
      Extracted from documentation: Named Imports
  - name: "CommonJS"
    id: concept:page_1687_bun
    description: |
      Extracted from documentation: CommonJS
  - name: "Hot Reloading with JSON5"
    id: concept:page_1688_bun
    description: |
      Extracted from documentation: Hot Reloading with JSON5
  - name: "Bundler Integration"
    id: concept:page_1689_bun
    description: |
      Extracted from documentation: Bundler Integration
  - name: "Dynamic Imports"
    id: concept:page_1690_bun
    description: |
      Extracted from documentation: Dynamic Imports
  - name: "JSONL"
    id: concept:page_1691_bun
    description: |
      Extracted from documentation: JSONL
  - name: "`Bun.JSONL.parse()`"
    id: concept:page_1692_bun
    description: |
      Extracted from documentation: `Bun.JSONL.parse()`
  - name: "`Bun.JSONL.parseChunk()`"
    id: concept:page_1693_bun
    description: |
      Extracted from documentation: `Bun.JSONL.parseChunk()`
  - name: "Return value"
    id: concept:page_1694_bun
    description: |
      Extracted from documentation: Return value
  - name: "Streaming example"
    id: concept:page_1695_bun
    description: |
      Extracted from documentation: Streaming example
  - name: "Byte offsets with `Uint8Array`"
    id: concept:page_1696_bun
    description: |
      Extracted from documentation: Byte offsets with `Uint8Array`
  - name: "Error recovery"
    id: concept:page_1697_bun
    description: |
      Extracted from documentation: Error recovery
  - name: "Supported value types"
    id: concept:page_1698_bun
    description: |
      Extracted from documentation: Supported value types
  - name: "Performance notes"
    id: concept:page_1699_bun
    description: |
      Extracted from documentation: Performance notes
  - name: "JSX"
    id: concept:page_1700_bun
    description: |
      Extracted from documentation: JSX
  - name: "[`jsx`](https://www.typescriptlang.org/tsconfig#jsx)"
    id: concept:page_1701_bun
    description: |
      Extracted from documentation: [`jsx`](https://www.typescriptlang.org/tsconfig#jsx)
  - name: "[`jsxFactory`](https://www.typescriptlang.org/tsconfig#jsxFactory)"
    id: concept:page_1702_bun
    description: |
      Extracted from documentation: [`jsxFactory`](https://www.typescriptlang.org/tsconfig#jsxFactory)
  - name: "[`jsxFragmentFactory`](https://www.typescriptlang.org/tsconfig#jsxFragmentFactory)"
    id: concept:page_1703_bun
    description: |
      Extracted from documentation: [`jsxFragmentFactory`](https://www.typescriptlang.org/tsconfig#jsxFragmentFactory)
  - name: "[`jsxImportSource`](https://www.typescriptlang.org/tsconfig#jsxImportSource)"
    id: concept:page_1704_bun
    description: |
      Extracted from documentation: [`jsxImportSource`](https://www.typescriptlang.org/tsconfig#jsxImportSource)
  - name: "JSX pragma"
    id: concept:page_1705_bun
    description: |
      Extracted from documentation: JSX pragma
  - name: "Prop punning"
    id: concept:page_1706_bun
    description: |
      Extracted from documentation: Prop punning
  - name: "Markdown"
    id: concept:page_1707_bun
    description: |
      Extracted from documentation: Markdown
  - name: "`Bun.markdown.html()`"
    id: concept:page_1708_bun
    description: |
      Extracted from documentation: `Bun.markdown.html()`
  - name: "Autolinks"
    id: concept:page_1709_bun
    description: |
      Extracted from documentation: Autolinks
  - name: "Heading IDs"
    id: concept:page_1710_bun
    description: |
      Extracted from documentation: Heading IDs
  - name: "`Bun.markdown.render()`"
    id: concept:page_1711_bun
    description: |
      Extracted from documentation: `Bun.markdown.render()`
  - name: "Callback signature"
    id: concept:page_1712_bun
    description: |
      Extracted from documentation: Callback signature
  - name: "Block callbacks"
    id: concept:page_1713_bun
    description: |
      Extracted from documentation: Block callbacks
  - name: "List item meta"
    id: concept:page_1714_bun
    description: |
      Extracted from documentation: List item meta
  - name: "Inline callbacks"
    id: concept:page_1715_bun
    description: |
      Extracted from documentation: Inline callbacks
  - name: "Custom HTML with classes"
    id: concept:page_1716_bun
    description: |
      Extracted from documentation: Custom HTML with classes
  - name: "Stripping all formatting"
    id: concept:page_1717_bun
    description: |
      Extracted from documentation: Stripping all formatting
  - name: "Omitting elements"
    id: concept:page_1718_bun
    description: |
      Extracted from documentation: Omitting elements
  - name: "ANSI terminal output"
    id: concept:page_1719_bun
    description: |
      Extracted from documentation: ANSI terminal output
  - name: "Nested list numbering"
    id: concept:page_1720_bun
    description: |
      Extracted from documentation: Nested list numbering
  - name: "Code block syntax highlighting"
    id: concept:page_1721_bun
    description: |
      Extracted from documentation: Code block syntax highlighting
  - name: "Parser options"
    id: concept:page_1722_bun
    description: |
      Extracted from documentation: Parser options
  - name: "`Bun.markdown.react()`"
    id: concept:page_1723_bun
    description: |
      Extracted from documentation: `Bun.markdown.react()`
  - name: "Server-side rendering"
    id: concept:page_1724_bun
    description: |
      Extracted from documentation: Server-side rendering
  - name: "Component overrides"
    id: concept:page_1725_bun
    description: |
      Extracted from documentation: Component overrides
  - name: "Available overrides"
    id: concept:page_1726_bun
    description: |
      Extracted from documentation: Available overrides
  - name: "React 18 and older"
    id: concept:page_1727_bun
    description: |
      Extracted from documentation: React 18 and older
  - name: "Module Resolution"
    id: concept:page_1728_bun
    description: |
      Extracted from documentation: Module Resolution
  - name: "Syntax"
    id: concept:page_1729_bun
    description: |
      Extracted from documentation: Syntax
  - name: "Module systems"
    id: concept:page_1730_bun
    description: |
      Extracted from documentation: Module systems
  - name: "Using `require()`"
    id: concept:page_1731_bun
    description: |
      Extracted from documentation: Using `require()`
  - name: "Using `import`"
    id: concept:page_1732_bun
    description: |
      Extracted from documentation: Using `import`
  - name: "Using `import` and `require()` together"
    id: concept:page_1733_bun
    description: |
      Extracted from documentation: Using `import` and `require()` together
  - name: "Top level await"
    id: concept:page_1734_bun
    description: |
      Extracted from documentation: Top level await
  - name: "Importing packages"
    id: concept:page_1735_bun
    description: |
      Extracted from documentation: Importing packages
  - name: "NODE\\_PATH"
    id: concept:page_1736_bun
    description: |
      Extracted from documentation: NODE\_PATH
  - name: "Custom conditions"
    id: concept:page_1737_bun
    description: |
      Extracted from documentation: Custom conditions
  - name: "Use it with bun build:"
    id: concept:page_1738_bun
    description: |
      Extracted from documentation: Use it with bun build:
  - name: "Use it with bun's runtime:"
    id: concept:page_1739_bun
    description: |
      Extracted from documentation: Use it with bun's runtime:
  - name: "Path re-mapping"
    id: concept:page_1740_bun
    description: |
      Extracted from documentation: Path re-mapping
  - name: "`import.meta`"
    id: concept:page_1741_bun
    description: |
      Extracted from documentation: `import.meta`
  - name: "DNS"
    id: concept:page_1742_bun
    description: |
      Extracted from documentation: DNS
  - name: "DNS caching in Bun"
    id: concept:page_1743_bun
    description: |
      Extracted from documentation: DNS caching in Bun
  - name: "When should I prefetch a DNS entry?"
    id: concept:page_1744_bun
    description: |
      Extracted from documentation: When should I prefetch a DNS entry?
  - name: "`dns.prefetch`"
    id: concept:page_1745_bun
    description: |
      Extracted from documentation: `dns.prefetch`
  - name: "`dns.getCacheStats()`"
    id: concept:page_1746_bun
    description: |
      Extracted from documentation: `dns.getCacheStats()`
  - name: "Configuring DNS cache TTL"
    id: concept:page_1747_bun
    description: |
      Extracted from documentation: Configuring DNS cache TTL
  - name: "Why is 30 seconds the default?"
    id: concept:page_1748_bun
    description: |
      Extracted from documentation: Why is 30 seconds the default?
  - name: "Fetch"
    id: concept:page_1749_bun
    description: |
      Extracted from documentation: Fetch
  - name: "Sending an HTTP request"
    id: concept:page_1750_bun
    description: |
      Extracted from documentation: Sending an HTTP request
  - name: "Sending a POST request"
    id: concept:page_1751_bun
    description: |
      Extracted from documentation: Sending a POST request
  - name: "Proxying requests"
    id: concept:page_1752_bun
    description: |
      Extracted from documentation: Proxying requests
  - name: "Custom headers"
    id: concept:page_1753_bun
    description: |
      Extracted from documentation: Custom headers
  - name: "Response bodies"
    id: concept:page_1754_bun
    description: |
      Extracted from documentation: Response bodies
  - name: "Streaming response bodies"
    id: concept:page_1755_bun
    description: |
      Extracted from documentation: Streaming response bodies
  - name: "Streaming request bodies"
    id: concept:page_1756_bun
    description: |
      Extracted from documentation: Streaming request bodies
  - name: "Fetching a URL with a timeout"
    id: concept:page_1757_bun
    description: |
      Extracted from documentation: Fetching a URL with a timeout
  - name: "Canceling a request"
    id: concept:page_1758_bun
    description: |
      Extracted from documentation: Canceling a request
  - name: "Custom TLS Validation"
    id: concept:page_1759_bun
    description: |
      Extracted from documentation: Custom TLS Validation
  - name: "Disable TLS validation"
    id: concept:page_1760_bun
    description: |
      Extracted from documentation: Disable TLS validation
  - name: "Request options"
    id: concept:page_1761_bun
    description: |
      Extracted from documentation: Request options
  - name: "Protocol support"
    id: concept:page_1762_bun
    description: |
      Extracted from documentation: Protocol support
  - name: "S3 URLs - `s3://`"
    id: concept:page_1763_bun
    description: |
      Extracted from documentation: S3 URLs - `s3://`
  - name: "File URLs - `file://`"
    id: concept:page_1764_bun
    description: |
      Extracted from documentation: File URLs - `file://`
  - name: "Data URLs - `data:`"
    id: concept:page_1765_bun
    description: |
      Extracted from documentation: Data URLs - `data:`
  - name: "Blob URLs - `blob:`"
    id: concept:page_1766_bun
    description: |
      Extracted from documentation: Blob URLs - `blob:`
  - name: "Content-Type handling"
    id: concept:page_1767_bun
    description: |
      Extracted from documentation: Content-Type handling
  - name: "DNS prefetching"
    id: concept:page_1768_bun
    description: |
      Extracted from documentation: DNS prefetching
  - name: "DNS caching"
    id: concept:page_1769_bun
    description: |
      Extracted from documentation: DNS caching
  - name: "Preconnect to a host"
    id: concept:page_1770_bun
    description: |
      Extracted from documentation: Preconnect to a host
  - name: "Preconnect at startup"
    id: concept:page_1771_bun
    description: |
      Extracted from documentation: Preconnect at startup
  - name: "Connection pooling & HTTP keep-alive"
    id: concept:page_1772_bun
    description: |
      Extracted from documentation: Connection pooling & HTTP keep-alive
  - name: "Simultaneous connection limit"
    id: concept:page_1773_bun
    description: |
      Extracted from documentation: Simultaneous connection limit
  - name: "Response buffering"
    id: concept:page_1774_bun
    description: |
      Extracted from documentation: Response buffering
  - name: "Implementation details"
    id: concept:page_1775_bun
    description: |
      Extracted from documentation: Implementation details
  - name: "TCP"
    id: concept:page_1776_bun
    description: |
      Extracted from documentation: TCP
  - name: "Start a server (`Bun.listen()`)"
    id: concept:page_1777_bun
    description: |
      Extracted from documentation: Start a server (`Bun.listen()`)
  - name: "Create a connection (`Bun.connect()`)"
    id: concept:page_1778_bun
    description: |
      Extracted from documentation: Create a connection (`Bun.connect()`)
  - name: "Buffering"
    id: concept:page_1779_bun
    description: |
      Extracted from documentation: Buffering
  - name: "UDP"
    id: concept:page_1780_bun
    description: |
      Extracted from documentation: UDP
  - name: "Bind a UDP socket (`Bun.udpSocket()`)"
    id: concept:page_1781_bun
    description: |
      Extracted from documentation: Bind a UDP socket (`Bun.udpSocket()`)
  - name: "Send a datagram"
    id: concept:page_1782_bun
    description: |
      Extracted from documentation: Send a datagram
  - name: "Receive datagrams"
    id: concept:page_1783_bun
    description: |
      Extracted from documentation: Receive datagrams
  - name: "Connections"
    id: concept:page_1784_bun
    description: |
      Extracted from documentation: Connections
  - name: "Send many packets at once using `sendMany()`"
    id: concept:page_1785_bun
    description: |
      Extracted from documentation: Send many packets at once using `sendMany()`
  - name: "Handle backpressure"
    id: concept:page_1786_bun
    description: |
      Extracted from documentation: Handle backpressure
  - name: "Socket options"
    id: concept:page_1787_bun
    description: |
      Extracted from documentation: Socket options
  - name: "Multicast"
    id: concept:page_1788_bun
    description: |
      Extracted from documentation: Multicast
  - name: "Node-API"
    id: concept:page_1789_bun
    description: |
      Extracted from documentation: Node-API
  - name: "Node.js Compatibility"
    id: concept:page_1790_bun
    description: |
      Extracted from documentation: Node.js Compatibility
  - name: "Built-in Node.js modules"
    id: concept:page_1791_bun
    description: |
      Extracted from documentation: Built-in Node.js modules
  - name: "[`node:assert`](https://nodejs.org/api/assert.html)"
    id: concept:page_1792_bun
    description: |
      Extracted from documentation: [`node:assert`](https://nodejs.org/api/assert.html)
  - name: "[`node:buffer`](https://nodejs.org/api/buffer.html)"
    id: concept:page_1793_bun
    description: |
      Extracted from documentation: [`node:buffer`](https://nodejs.org/api/buffer.html)
  - name: "[`node:console`](https://nodejs.org/api/console.html)"
    id: concept:page_1794_bun
    description: |
      Extracted from documentation: [`node:console`](https://nodejs.org/api/console.html)
  - name: "[`node:dgram`](https://nodejs.org/api/dgram.html)"
    id: concept:page_1795_bun
    description: |
      Extracted from documentation: [`node:dgram`](https://nodejs.org/api/dgram.html)
  - name: "[`node:diagnostics_channel`](https://nodejs.org/api/diagnostics_channel.html)"
    id: concept:page_1796_bun
    description: |
      Extracted from documentation: [`node:diagnostics_channel`](https://nodejs.org/api/diagnostics_channel.html)
  - name: "[`node:dns`](https://nodejs.org/api/dns.html)"
    id: concept:page_1797_bun
    description: |
      Extracted from documentation: [`node:dns`](https://nodejs.org/api/dns.html)
  - name: "[`node:events`](https://nodejs.org/api/events.html)"
    id: concept:page_1798_bun
    description: |
      Extracted from documentation: [`node:events`](https://nodejs.org/api/events.html)
  - name: "[`node:fs`](https://nodejs.org/api/fs.html)"
    id: concept:page_1799_bun
    description: |
      Extracted from documentation: [`node:fs`](https://nodejs.org/api/fs.html)
  - name: "[`node:http`](https://nodejs.org/api/http.html)"
    id: concept:page_1800_bun
    description: |
      Extracted from documentation: [`node:http`](https://nodejs.org/api/http.html)
  - name: "[`node:https`](https://nodejs.org/api/https.html)"
    id: concept:page_1801_bun
    description: |
      Extracted from documentation: [`node:https`](https://nodejs.org/api/https.html)
  - name: "[`node:os`](https://nodejs.org/api/os.html)"
    id: concept:page_1802_bun
    description: |
      Extracted from documentation: [`node:os`](https://nodejs.org/api/os.html)
  - name: "[`node:path`](https://nodejs.org/api/path.html)"
    id: concept:page_1803_bun
    description: |
      Extracted from documentation: [`node:path`](https://nodejs.org/api/path.html)
  - name: "[`node:punycode`](https://nodejs.org/api/punycode.html)"
    id: concept:page_1804_bun
    description: |
      Extracted from documentation: [`node:punycode`](https://nodejs.org/api/punycode.html)
  - name: "[`node:querystring`](https://nodejs.org/api/querystring.html)"
    id: concept:page_1805_bun
    description: |
      Extracted from documentation: [`node:querystring`](https://nodejs.org/api/querystring.html)
  - name: "[`node:readline`](https://nodejs.org/api/readline.html)"
    id: concept:page_1806_bun
    description: |
      Extracted from documentation: [`node:readline`](https://nodejs.org/api/readline.html)
  - name: "[`node:stream`](https://nodejs.org/api/stream.html)"
    id: concept:page_1807_bun
    description: |
      Extracted from documentation: [`node:stream`](https://nodejs.org/api/stream.html)
  - name: "[`node:string_decoder`](https://nodejs.org/api/string_decoder.html)"
    id: concept:page_1808_bun
    description: |
      Extracted from documentation: [`node:string_decoder`](https://nodejs.org/api/string_decoder.html)
  - name: "[`node:timers`](https://nodejs.org/api/timers.html)"
    id: concept:page_1809_bun
    description: |
      Extracted from documentation: [`node:timers`](https://nodejs.org/api/timers.html)
  - name: "[`node:tty`](https://nodejs.org/api/tty.html)"
    id: concept:page_1810_bun
    description: |
      Extracted from documentation: [`node:tty`](https://nodejs.org/api/tty.html)
  - name: "[`node:url`](https://nodejs.org/api/url.html)"
    id: concept:page_1811_bun
    description: |
      Extracted from documentation: [`node:url`](https://nodejs.org/api/url.html)
  - name: "[`node:zlib`](https://nodejs.org/api/zlib.html)"
    id: concept:page_1812_bun
    description: |
      Extracted from documentation: [`node:zlib`](https://nodejs.org/api/zlib.html)
  - name: "[`node:async_hooks`](https://nodejs.org/api/async_hooks.html)"
    id: concept:page_1813_bun
    description: |
      Extracted from documentation: [`node:async_hooks`](https://nodejs.org/api/async_hooks.html)
  - name: "[`node:child_process`](https://nodejs.org/api/child_process.html)"
    id: concept:page_1814_bun
    description: |
      Extracted from documentation: [`node:child_process`](https://nodejs.org/api/child_process.html)
  - name: "[`node:cluster`](https://nodejs.org/api/cluster.html)"
    id: concept:page_1815_bun
    description: |
      Extracted from documentation: [`node:cluster`](https://nodejs.org/api/cluster.html)
  - name: "[`node:crypto`](https://nodejs.org/api/crypto.html)"
    id: concept:page_1816_bun
    description: |
      Extracted from documentation: [`node:crypto`](https://nodejs.org/api/crypto.html)
  - name: "[`node:domain`](https://nodejs.org/api/domain.html)"
    id: concept:page_1817_bun
    description: |
      Extracted from documentation: [`node:domain`](https://nodejs.org/api/domain.html)
  - name: "[`node:http2`](https://nodejs.org/api/http2.html)"
    id: concept:page_1818_bun
    description: |
      Extracted from documentation: [`node:http2`](https://nodejs.org/api/http2.html)
  - name: "[`node:module`](https://nodejs.org/api/module.html)"
    id: concept:page_1819_bun
    description: |
      Extracted from documentation: [`node:module`](https://nodejs.org/api/module.html)
  - name: "[`node:net`](https://nodejs.org/api/net.html)"
    id: concept:page_1820_bun
    description: |
      Extracted from documentation: [`node:net`](https://nodejs.org/api/net.html)
  - name: "[`node:perf_hooks`](https://nodejs.org/api/perf_hooks.html)"
    id: concept:page_1821_bun
    description: |
      Extracted from documentation: [`node:perf_hooks`](https://nodejs.org/api/perf_hooks.html)
  - name: "[`node:process`](https://nodejs.org/api/process.html)"
    id: concept:page_1822_bun
    description: |
      Extracted from documentation: [`node:process`](https://nodejs.org/api/process.html)
  - name: "[`node:sys`](https://nodejs.org/api/util.html)"
    id: concept:page_1823_bun
    description: |
      Extracted from documentation: [`node:sys`](https://nodejs.org/api/util.html)
  - name: "[`node:tls`](https://nodejs.org/api/tls.html)"
    id: concept:page_1824_bun
    description: |
      Extracted from documentation: [`node:tls`](https://nodejs.org/api/tls.html)
  - name: "[`node:util`](https://nodejs.org/api/util.html)"
    id: concept:page_1825_bun
    description: |
      Extracted from documentation: [`node:util`](https://nodejs.org/api/util.html)
  - name: "[`node:v8`](https://nodejs.org/api/v8.html)"
    id: concept:page_1826_bun
    description: |
      Extracted from documentation: [`node:v8`](https://nodejs.org/api/v8.html)
  - name: "[`node:vm`](https://nodejs.org/api/vm.html)"
    id: concept:page_1827_bun
    description: |
      Extracted from documentation: [`node:vm`](https://nodejs.org/api/vm.html)
  - name: "[`node:wasi`](https://nodejs.org/api/wasi.html)"
    id: concept:page_1828_bun
    description: |
      Extracted from documentation: [`node:wasi`](https://nodejs.org/api/wasi.html)
  - name: "[`node:worker_threads`](https://nodejs.org/api/worker_threads.html)"
    id: concept:page_1829_bun
    description: |
      Extracted from documentation: [`node:worker_threads`](https://nodejs.org/api/worker_threads.html)
  - name: "[`node:inspector`](https://nodejs.org/api/inspector.html)"
    id: concept:page_1830_bun
    description: |
      Extracted from documentation: [`node:inspector`](https://nodejs.org/api/inspector.html)
  - name: "[`node:repl`](https://nodejs.org/api/repl.html)"
    id: concept:page_1831_bun
    description: |
      Extracted from documentation: [`node:repl`](https://nodejs.org/api/repl.html)
  - name: "[`node:sqlite`](https://nodejs.org/api/sqlite.html)"
    id: concept:page_1832_bun
    description: |
      Extracted from documentation: [`node:sqlite`](https://nodejs.org/api/sqlite.html)
  - name: "[`node:test`](https://nodejs.org/api/test.html)"
    id: concept:page_1833_bun
    description: |
      Extracted from documentation: [`node:test`](https://nodejs.org/api/test.html)
  - name: "[`node:trace_events`](https://nodejs.org/api/tracing.html)"
    id: concept:page_1834_bun
    description: |
      Extracted from documentation: [`node:trace_events`](https://nodejs.org/api/tracing.html)
  - name: "Node.js globals"
    id: concept:page_1835_bun
    description: |
      Extracted from documentation: Node.js globals
  - name: "[`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)"
    id: concept:page_1836_bun
    description: |
      Extracted from documentation: [`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)
  - name: "[`AbortSignal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)"
    id: concept:page_1837_bun
    description: |
      Extracted from documentation: [`AbortSignal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)
  - name: "[`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob)"
    id: concept:page_1838_bun
    description: |
      Extracted from documentation: [`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
  - name: "[`Buffer`](https://nodejs.org/api/buffer.html#class-buffer)"
    id: concept:page_1839_bun
    description: |
      Extracted from documentation: [`Buffer`](https://nodejs.org/api/buffer.html#class-buffer)
  - name: "[`ByteLengthQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)"
    id: concept:page_1840_bun
    description: |
      Extracted from documentation: [`ByteLengthQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)
  - name: "[`__dirname`](https://nodejs.org/api/globals.html#__dirname)"
    id: concept:page_1841_bun
    description: |
      Extracted from documentation: [`__dirname`](https://nodejs.org/api/globals.html#__dirname)
  - name: "[`__filename`](https://nodejs.org/api/globals.html#__filename)"
    id: concept:page_1842_bun
    description: |
      Extracted from documentation: [`__filename`](https://nodejs.org/api/globals.html#__filename)
  - name: "[`atob()`](https://developer.mozilla.org/en-US/docs/Web/API/atob)"
    id: concept:page_1843_bun
    description: |
      Extracted from documentation: [`atob()`](https://developer.mozilla.org/en-US/docs/Web/API/atob)
  - name: "[`Atomics`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics)"
    id: concept:page_1844_bun
    description: |
      Extracted from documentation: [`Atomics`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Atomics)
  - name: "[`BroadcastChannel`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)"
    id: concept:page_1845_bun
    description: |
      Extracted from documentation: [`BroadcastChannel`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)
  - name: "[`btoa()`](https://developer.mozilla.org/en-US/docs/Web/API/btoa)"
    id: concept:page_1846_bun
    description: |
      Extracted from documentation: [`btoa()`](https://developer.mozilla.org/en-US/docs/Web/API/btoa)
  - name: "[`clearImmediate()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearImmediate)"
    id: concept:page_1847_bun
    description: |
      Extracted from documentation: [`clearImmediate()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearImmediate)
  - name: "[`clearInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)"
    id: concept:page_1848_bun
    description: |
      Extracted from documentation: [`clearInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)
  - name: "[`clearTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)"
    id: concept:page_1849_bun
    description: |
      Extracted from documentation: [`clearTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)
  - name: "[`CompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream)"
    id: concept:page_1850_bun
    description: |
      Extracted from documentation: [`CompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream)
  - name: "[`console`](https://developer.mozilla.org/en-US/docs/Web/API/console)"
    id: concept:page_1851_bun
    description: |
      Extracted from documentation: [`console`](https://developer.mozilla.org/en-US/docs/Web/API/console)
  - name: "[`CountQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)"
    id: concept:page_1852_bun
    description: |
      Extracted from documentation: [`CountQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)
  - name: "[`Crypto`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)"
    id: concept:page_1853_bun
    description: |
      Extracted from documentation: [`Crypto`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)
  - name: "[`SubtleCrypto (crypto)`](https://developer.mozilla.org/en-US/docs/Web/API/crypto)"
    id: concept:page_1854_bun
    description: |
      Extracted from documentation: [`SubtleCrypto (crypto)`](https://developer.mozilla.org/en-US/docs/Web/API/crypto)
  - name: "[`CryptoKey`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)"
    id: concept:page_1855_bun
    description: |
      Extracted from documentation: [`CryptoKey`](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)
  - name: "[`CustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)"
    id: concept:page_1856_bun
    description: |
      Extracted from documentation: [`CustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
  - name: "[`DecompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)"
    id: concept:page_1857_bun
    description: |
      Extracted from documentation: [`DecompressionStream`](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)
  - name: "[`Event`](https://developer.mozilla.org/en-US/docs/Web/API/Event)"
    id: concept:page_1858_bun
    description: |
      Extracted from documentation: [`Event`](https://developer.mozilla.org/en-US/docs/Web/API/Event)
  - name: "[`EventTarget`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)"
    id: concept:page_1859_bun
    description: |
      Extracted from documentation: [`EventTarget`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)
  - name: "[`exports`](https://nodejs.org/api/globals.html#exports)"
    id: concept:page_1860_bun
    description: |
      Extracted from documentation: [`exports`](https://nodejs.org/api/globals.html#exports)
  - name: "[`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/fetch)"
    id: concept:page_1861_bun
    description: |
      Extracted from documentation: [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/fetch)
  - name: "[`FormData`](https://developer.mozilla.org/en-US/docs/Web/API/FormData)"
    id: concept:page_1862_bun
    description: |
      Extracted from documentation: [`FormData`](https://developer.mozilla.org/en-US/docs/Web/API/FormData)
  - name: "[`global`](https://nodejs.org/api/globals.html#global)"
    id: concept:page_1863_bun
    description: |
      Extracted from documentation: [`global`](https://nodejs.org/api/globals.html#global)
  - name: "[`globalThis`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis)"
    id: concept:page_1864_bun
    description: |
      Extracted from documentation: [`globalThis`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis)
  - name: "[`Headers`](https://developer.mozilla.org/en-US/docs/Web/API/Headers)"
    id: concept:page_1865_bun
    description: |
      Extracted from documentation: [`Headers`](https://developer.mozilla.org/en-US/docs/Web/API/Headers)
  - name: "[`MessageChannel`](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)"
    id: concept:page_1866_bun
    description: |
      Extracted from documentation: [`MessageChannel`](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)
  - name: "[`MessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)"
    id: concept:page_1867_bun
    description: |
      Extracted from documentation: [`MessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)
  - name: "[`MessagePort`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)"
    id: concept:page_1868_bun
    description: |
      Extracted from documentation: [`MessagePort`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)
  - name: "[`module`](https://nodejs.org/api/globals.html#module)"
    id: concept:page_1869_bun
    description: |
      Extracted from documentation: [`module`](https://nodejs.org/api/globals.html#module)
  - name: "[`PerformanceEntry`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)"
    id: concept:page_1870_bun
    description: |
      Extracted from documentation: [`PerformanceEntry`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)
  - name: "[`PerformanceMark`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)"
    id: concept:page_1871_bun
    description: |
      Extracted from documentation: [`PerformanceMark`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)
  - name: "[`PerformanceMeasure`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)"
    id: concept:page_1872_bun
    description: |
      Extracted from documentation: [`PerformanceMeasure`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)
  - name: "[`PerformanceObserver`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)"
    id: concept:page_1873_bun
    description: |
      Extracted from documentation: [`PerformanceObserver`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)
  - name: "[`PerformanceObserverEntryList`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)"
    id: concept:page_1874_bun
    description: |
      Extracted from documentation: [`PerformanceObserverEntryList`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)
  - name: "[`PerformanceResourceTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)"
    id: concept:page_1875_bun
    description: |
      Extracted from documentation: [`PerformanceResourceTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)
  - name: "[`performance`](https://developer.mozilla.org/en-US/docs/Web/API/performance)"
    id: concept:page_1876_bun
    description: |
      Extracted from documentation: [`performance`](https://developer.mozilla.org/en-US/docs/Web/API/performance)
  - name: "[`process`](https://nodejs.org/api/process.html)"
    id: concept:page_1877_bun
    description: |
      Extracted from documentation: [`process`](https://nodejs.org/api/process.html)
  - name: "[`queueMicrotask()`](https://developer.mozilla.org/en-US/docs/Web/API/queueMicrotask)"
    id: concept:page_1878_bun
    description: |
      Extracted from documentation: [`queueMicrotask()`](https://developer.mozilla.org/en-US/docs/Web/API/queueMicrotask)
  - name: "[`ReadableByteStreamController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)"
    id: concept:page_1879_bun
    description: |
      Extracted from documentation: [`ReadableByteStreamController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)
  - name: "[`ReadableStream`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)"
    id: concept:page_1880_bun
    description: |
      Extracted from documentation: [`ReadableStream`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)
  - name: "[`ReadableStreamBYOBReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)"
    id: concept:page_1881_bun
    description: |
      Extracted from documentation: [`ReadableStreamBYOBReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)
  - name: "[`ReadableStreamBYOBRequest`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)"
    id: concept:page_1882_bun
    description: |
      Extracted from documentation: [`ReadableStreamBYOBRequest`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)
  - name: "[`ReadableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)"
    id: concept:page_1883_bun
    description: |
      Extracted from documentation: [`ReadableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)
  - name: "[`ReadableStreamDefaultReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)"
    id: concept:page_1884_bun
    description: |
      Extracted from documentation: [`ReadableStreamDefaultReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)
  - name: "[`require()`](https://nodejs.org/api/globals.html#require)"
    id: concept:page_1885_bun
    description: |
      Extracted from documentation: [`require()`](https://nodejs.org/api/globals.html#require)
  - name: "[`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response)"
    id: concept:page_1886_bun
    description: |
      Extracted from documentation: [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response)
  - name: "[`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request)"
    id: concept:page_1887_bun
    description: |
      Extracted from documentation: [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request)
  - name: "[`setImmediate()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setImmediate)"
    id: concept:page_1888_bun
    description: |
      Extracted from documentation: [`setImmediate()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setImmediate)
  - name: "[`setInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"
    id: concept:page_1889_bun
    description: |
      Extracted from documentation: [`setInterval()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
  - name: "[`setTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"
    id: concept:page_1890_bun
    description: |
      Extracted from documentation: [`setTimeout()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)
  - name: "[`structuredClone()`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone)"
    id: concept:page_1891_bun
    description: |
      Extracted from documentation: [`structuredClone()`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone)
  - name: "[`SubtleCrypto`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)"
    id: concept:page_1892_bun
    description: |
      Extracted from documentation: [`SubtleCrypto`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)
  - name: "[`DOMException`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)"
    id: concept:page_1893_bun
    description: |
      Extracted from documentation: [`DOMException`](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)
  - name: "[`TextDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)"
    id: concept:page_1894_bun
    description: |
      Extracted from documentation: [`TextDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
  - name: "[`TextDecoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream)"
    id: concept:page_1895_bun
    description: |
      Extracted from documentation: [`TextDecoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream)
  - name: "[`TextEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)"
    id: concept:page_1896_bun
    description: |
      Extracted from documentation: [`TextEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
  - name: "[`TextEncoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream)"
    id: concept:page_1897_bun
    description: |
      Extracted from documentation: [`TextEncoderStream`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream)
  - name: "[`TransformStream`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"
    id: concept:page_1898_bun
    description: |
      Extracted from documentation: [`TransformStream`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)
  - name: "[`TransformStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)"
    id: concept:page_1899_bun
    description: |
      Extracted from documentation: [`TransformStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)
  - name: "[`URL`](https://developer.mozilla.org/en-US/docs/Web/API/URL)"
    id: concept:page_1900_bun
    description: |
      Extracted from documentation: [`URL`](https://developer.mozilla.org/en-US/docs/Web/API/URL)
  - name: "[`URLSearchParams`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)"
    id: concept:page_1901_bun
    description: |
      Extracted from documentation: [`URLSearchParams`](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)
  - name: "[`WebAssembly`](https://nodejs.org/api/globals.html#webassembly)"
    id: concept:page_1902_bun
    description: |
      Extracted from documentation: [`WebAssembly`](https://nodejs.org/api/globals.html#webassembly)
  - name: "[`WritableStream`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"
    id: concept:page_1903_bun
    description: |
      Extracted from documentation: [`WritableStream`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)
  - name: "[`WritableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)"
    id: concept:page_1904_bun
    description: |
      Extracted from documentation: [`WritableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)
  - name: "[`WritableStreamDefaultWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)"
    id: concept:page_1905_bun
    description: |
      Extracted from documentation: [`WritableStreamDefaultWriter`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)
  - name: "`onStart`"
    id: concept:page_1906_bun
    description: |
      Extracted from documentation: `onStart`
  - name: "`onResolve`"
    id: concept:page_1907_bun
    description: |
      Extracted from documentation: `onResolve`
  - name: "`onLoad`"
    id: concept:page_1908_bun
    description: |
      Extracted from documentation: `onLoad`
  - name: "`.defer()`"
    id: concept:page_1909_bun
    description: |
      Extracted from documentation: `.defer()`
  - name: "Example: tracking and reporting unused exports"
    id: concept:page_1910_bun
    description: |
      Extracted from documentation: Example: tracking and reporting unused exports
  - name: "`onBeforeParse`"
    id: concept:page_1911_bun
    description: |
      Extracted from documentation: `onBeforeParse`
  - name: "Redis"
    id: concept:page_1912_bun
    description: |
      Extracted from documentation: Redis
  - name: "Getting Started"
    id: concept:page_1913_bun
    description: |
      Extracted from documentation: Getting Started
  - name: "Connection Lifecycle"
    id: concept:page_1914_bun
    description: |
      Extracted from documentation: Connection Lifecycle
  - name: "Basic Operations"
    id: concept:page_1915_bun
    description: |
      Extracted from documentation: Basic Operations
  - name: "String Operations"
    id: concept:page_1916_bun
    description: |
      Extracted from documentation: String Operations
  - name: "Numeric Operations"
    id: concept:page_1917_bun
    description: |
      Extracted from documentation: Numeric Operations
  - name: "Hash Operations"
    id: concept:page_1918_bun
    description: |
      Extracted from documentation: Hash Operations
  - name: "Set Operations"
    id: concept:page_1919_bun
    description: |
      Extracted from documentation: Set Operations
  - name: "Subscriptions"
    id: concept:page_1920_bun
    description: |
      Extracted from documentation: Subscriptions
  - name: "Advanced Usage"
    id: concept:page_1921_bun
    description: |
      Extracted from documentation: Advanced Usage
  - name: "Command Execution and Pipelining"
    id: concept:page_1922_bun
    description: |
      Extracted from documentation: Command Execution and Pipelining
  - name: "Raw Commands"
    id: concept:page_1923_bun
    description: |
      Extracted from documentation: Raw Commands
  - name: "Connection Events"
    id: concept:page_1924_bun
    description: |
      Extracted from documentation: Connection Events
  - name: "Connection Status and Monitoring"
    id: concept:page_1925_bun
    description: |
      Extracted from documentation: Connection Status and Monitoring
  - name: "Type Conversion"
    id: concept:page_1926_bun
    description: |
      Extracted from documentation: Type Conversion
  - name: "Connection Options"
    id: concept:page_1927_bun
    description: |
      Extracted from documentation: Connection Options
  - name: "Reconnection Behavior"
    id: concept:page_1928_bun
    description: |
      Extracted from documentation: Reconnection Behavior
  - name: "Supported URL Formats"
    id: concept:page_1929_bun
    description: |
      Extracted from documentation: Supported URL Formats
  - name: "Example Use Cases"
    id: concept:page_1930_bun
    description: |
      Extracted from documentation: Example Use Cases
  - name: "Rate Limiting"
    id: concept:page_1931_bun
    description: |
      Extracted from documentation: Rate Limiting
  - name: "Session Storage"
    id: concept:page_1932_bun
    description: |
      Extracted from documentation: Session Storage
  - name: "Implementation Notes"
    id: concept:page_1933_bun
    description: |
      Extracted from documentation: Implementation Notes
  - name: "REPL"
    id: concept:page_1934_bun
    description: |
      Extracted from documentation: REPL
  - name: "Features"
    id: concept:page_1935_bun
    description: |
      Extracted from documentation: Features
  - name: "Special variables"
    id: concept:page_1936_bun
    description: |
      Extracted from documentation: Special variables
  - name: "Top-level `await`"
    id: concept:page_1937_bun
    description: |
      Extracted from documentation: Top-level `await`
  - name: "Importing modules"
    id: concept:page_1938_bun
    description: |
      Extracted from documentation: Importing modules
  - name: "Multi-line input"
    id: concept:page_1939_bun
    description: |
      Extracted from documentation: Multi-line input
  - name: "REPL commands"
    id: concept:page_1940_bun
    description: |
      Extracted from documentation: REPL commands
  - name: "Keybindings"
    id: concept:page_1941_bun
    description: |
      Extracted from documentation: Keybindings
  - name: "History"
    id: concept:page_1942_bun
    description: |
      Extracted from documentation: History
  - name: "Non-interactive mode"
    id: concept:page_1943_bun
    description: |
      Extracted from documentation: Non-interactive mode
  - name: "42"
    id: concept:page_1944_bun
    description: |
      Extracted from documentation: 42
  - name: "200"
    id: concept:page_1945_bun
    description: |
      Extracted from documentation: 200
  - name: "{ a: 1, b: 2 }"
    id: concept:page_1946_bun
    description: |
      Extracted from documentation: { a: 1, b: 2 }
  - name: "S3"
    id: concept:page_1947_bun
    description: |
      Extracted from documentation: S3
  - name: "Bun's S3 API is fast"
    id: concept:page_1948_bun
    description: |
      Extracted from documentation: Bun's S3 API is fast
  - name: "`Bun.S3Client` & `Bun.s3`"
    id: concept:page_1949_bun
    description: |
      Extracted from documentation: `Bun.S3Client` & `Bun.s3`
  - name: "Working with S3 Files"
    id: concept:page_1950_bun
    description: |
      Extracted from documentation: Working with S3 Files
  - name: "Reading files from S3"
    id: concept:page_1951_bun
    description: |
      Extracted from documentation: Reading files from S3
  - name: "Memory optimization"
    id: concept:page_1952_bun
    description: |
      Extracted from documentation: Memory optimization
  - name: "Writing & uploading files to S3"
    id: concept:page_1953_bun
    description: |
      Extracted from documentation: Writing & uploading files to S3
  - name: "Working with large files (streams)"
    id: concept:page_1954_bun
    description: |
      Extracted from documentation: Working with large files (streams)
  - name: "Presigning URLs"
    id: concept:page_1955_bun
    description: |
      Extracted from documentation: Presigning URLs
  - name: "Setting ACLs"
    id: concept:page_1956_bun
    description: |
      Extracted from documentation: Setting ACLs
  - name: "Expiring URLs"
    id: concept:page_1957_bun
    description: |
      Extracted from documentation: Expiring URLs
  - name: "`method`"
    id: concept:page_1958_bun
    description: |
      Extracted from documentation: `method`
  - name: "`new Response(S3File)`"
    id: concept:page_1959_bun
    description: |
      Extracted from documentation: `new Response(S3File)`
  - name: "Support for S3-Compatible Services"
    id: concept:page_1960_bun
    description: |
      Extracted from documentation: Support for S3-Compatible Services
  - name: "Using Bun's S3Client with AWS S3"
    id: concept:page_1961_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with AWS S3
  - name: "Using Bun's S3Client with Google Cloud Storage"
    id: concept:page_1962_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with Google Cloud Storage
  - name: "Using Bun's S3Client with Cloudflare R2"
    id: concept:page_1963_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with Cloudflare R2
  - name: "Using Bun's S3Client with DigitalOcean Spaces"
    id: concept:page_1964_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with DigitalOcean Spaces
  - name: "Using Bun's S3Client with MinIO"
    id: concept:page_1965_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with MinIO
  - name: "Using Bun's S3Client with supabase"
    id: concept:page_1966_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with supabase
  - name: "Using Bun's S3Client with S3 Virtual Hosted-Style endpoints"
    id: concept:page_1967_bun
    description: |
      Extracted from documentation: Using Bun's S3Client with S3 Virtual Hosted-Style endpoints
  - name: "Credentials"
    id: concept:page_1968_bun
    description: |
      Extracted from documentation: Credentials
  - name: "`S3Client` objects"
    id: concept:page_1969_bun
    description: |
      Extracted from documentation: `S3Client` objects
  - name: "`S3Client.prototype.write`"
    id: concept:page_1970_bun
    description: |
      Extracted from documentation: `S3Client.prototype.write`
  - name: "`S3Client.prototype.delete`"
    id: concept:page_1971_bun
    description: |
      Extracted from documentation: `S3Client.prototype.delete`
  - name: "`S3Client.prototype.exists`"
    id: concept:page_1972_bun
    description: |
      Extracted from documentation: `S3Client.prototype.exists`
  - name: "`S3File`"
    id: concept:page_1973_bun
    description: |
      Extracted from documentation: `S3File`
  - name: "Partial reads with `slice`"
    id: concept:page_1974_bun
    description: |
      Extracted from documentation: Partial reads with `slice`
  - name: "Deleting files from S3"
    id: concept:page_1975_bun
    description: |
      Extracted from documentation: Deleting files from S3
  - name: "Error codes"
    id: concept:page_1976_bun
    description: |
      Extracted from documentation: Error codes
  - name: "`S3Client` static methods"
    id: concept:page_1977_bun
    description: |
      Extracted from documentation: `S3Client` static methods
  - name: "`S3Client.write` (static)"
    id: concept:page_1978_bun
    description: |
      Extracted from documentation: `S3Client.write` (static)
  - name: "`S3Client.presign` (static)"
    id: concept:page_1979_bun
    description: |
      Extracted from documentation: `S3Client.presign` (static)
  - name: "`S3Client.list` (static)"
    id: concept:page_1980_bun
    description: |
      Extracted from documentation: `S3Client.list` (static)
  - name: "`S3Client.exists` (static)"
    id: concept:page_1981_bun
    description: |
      Extracted from documentation: `S3Client.exists` (static)
  - name: "`S3Client.size` (static)"
    id: concept:page_1982_bun
    description: |
      Extracted from documentation: `S3Client.size` (static)
  - name: "`S3Client.stat` (static)"
    id: concept:page_1983_bun
    description: |
      Extracted from documentation: `S3Client.stat` (static)
  - name: "`S3Client.delete` (static)"
    id: concept:page_1984_bun
    description: |
      Extracted from documentation: `S3Client.delete` (static)
  - name: "`s3://` protocol"
    id: concept:page_1985_bun
    description: |
      Extracted from documentation: `s3://` protocol
  - name: "UTF-8, UTF-16, and BOM (byte order mark)"
    id: concept:page_1986_bun
    description: |
      Extracted from documentation: UTF-8, UTF-16, and BOM (byte order mark)
  - name: "Secrets"
    id: concept:page_1987_bun
    description: |
      Extracted from documentation: Secrets
  - name: "`Bun.secrets.get(options)`"
    id: concept:page_1988_bun
    description: |
      Extracted from documentation: `Bun.secrets.get(options)`
  - name: "`Bun.secrets.set(options)`"
    id: concept:page_1989_bun
    description: |
      Extracted from documentation: `Bun.secrets.set(options)`
  - name: "`Bun.secrets.delete(options)`"
    id: concept:page_1990_bun
    description: |
      Extracted from documentation: `Bun.secrets.delete(options)`
  - name: "Storing CLI Tool Credentials"
    id: concept:page_1991_bun
    description: |
      Extracted from documentation: Storing CLI Tool Credentials
  - name: "Migrating from Plaintext Config Files"
    id: concept:page_1992_bun
    description: |
      Extracted from documentation: Migrating from Plaintext Config Files
  - name: "Updating Credentials"
    id: concept:page_1993_bun
    description: |
      Extracted from documentation: Updating Credentials
  - name: "Platform Behavior"
    id: concept:page_1994_bun
    description: |
      Extracted from documentation: Platform Behavior
  - name: "macOS (Keychain)"
    id: concept:page_1995_bun
    description: |
      Extracted from documentation: macOS (Keychain)
  - name: "Linux (libsecret)"
    id: concept:page_1996_bun
    description: |
      Extracted from documentation: Linux (libsecret)
  - name: "Windows (Credential Manager)"
    id: concept:page_1997_bun
    description: |
      Extracted from documentation: Windows (Credential Manager)
  - name: "Security Considerations"
    id: concept:page_1998_bun
    description: |
      Extracted from documentation: Security Considerations
  - name: "Comparison with Environment Variables"
    id: concept:page_1999_bun
    description: |
      Extracted from documentation: Comparison with Environment Variables
  - name: "Semver"
    id: concept:page_2000_bun
    description: |
      Extracted from documentation: Semver
  - name: "`Bun.semver.satisfies(version: string, range: string): boolean`"
    id: concept:page_2001_bun
    description: |
      Extracted from documentation: `Bun.semver.satisfies(version: string, range: string): boolean`
  - name: "`Bun.semver.order(versionA: string, versionB: string): 0 | 1 | -1`"
    id: concept:page_2002_bun
    description: |
      Extracted from documentation: `Bun.semver.order(versionA: string, versionB: string): 0 | 1 | -1`
  - name: "Shell"
    id: concept:page_2003_bun
    description: |
      Extracted from documentation: Shell
  - name: "Redirection"
    id: concept:page_2004_bun
    description: |
      Extracted from documentation: Redirection
  - name: "Example: Redirect output to JavaScript objects (`>`)"
    id: concept:page_2005_bun
    description: |
      Extracted from documentation: Example: Redirect output to JavaScript objects (`>`)
  - name: "Example: Redirect input from JavaScript objects (`<`)"
    id: concept:page_2006_bun
    description: |
      Extracted from documentation: Example: Redirect input from JavaScript objects (`<`)
  - name: "Example: Redirect stdin -> file"
    id: concept:page_2007_bun
    description: |
      Extracted from documentation: Example: Redirect stdin -> file
  - name: "Example: Redirect stdout -> file"
    id: concept:page_2008_bun
    description: |
      Extracted from documentation: Example: Redirect stdout -> file
  - name: "Example: Redirect stderr -> file"
    id: concept:page_2009_bun
    description: |
      Extracted from documentation: Example: Redirect stderr -> file
  - name: "Example: Redirect stderr -> stdout"
    id: concept:page_2010_bun
    description: |
      Extracted from documentation: Example: Redirect stderr -> stdout
  - name: "Example: Redirect stdout -> stderr"
    id: concept:page_2011_bun
    description: |
      Extracted from documentation: Example: Redirect stdout -> stderr
  - name: "Piping (`|`)"
    id: concept:page_2012_bun
    description: |
      Extracted from documentation: Piping (`|`)
  - name: "Command substitution (`$(...)`)"
    id: concept:page_2013_bun
    description: |
      Extracted from documentation: Command substitution (`$(...)`)
  - name: "Changing the environment variables"
    id: concept:page_2014_bun
    description: |
      Extracted from documentation: Changing the environment variables
  - name: "Changing the working directory"
    id: concept:page_2015_bun
    description: |
      Extracted from documentation: Changing the working directory
  - name: "Reading output"
    id: concept:page_2016_bun
    description: |
      Extracted from documentation: Reading output
  - name: "Reading output as JSON"
    id: concept:page_2017_bun
    description: |
      Extracted from documentation: Reading output as JSON
  - name: "Reading output line-by-line"
    id: concept:page_2018_bun
    description: |
      Extracted from documentation: Reading output line-by-line
  - name: "Reading output as a Blob"
    id: concept:page_2019_bun
    description: |
      Extracted from documentation: Reading output as a Blob
  - name: "Builtin Commands"
    id: concept:page_2020_bun
    description: |
      Extracted from documentation: Builtin Commands
  - name: "Utilities"
    id: concept:page_2021_bun
    description: |
      Extracted from documentation: Utilities
  - name: "`$.braces` (brace expansion)"
    id: concept:page_2022_bun
    description: |
      Extracted from documentation: `$.braces` (brace expansion)
  - name: "`$.escape` (escape strings)"
    id: concept:page_2023_bun
    description: |
      Extracted from documentation: `$.escape` (escape strings)
  - name: "`.sh` file loader"
    id: concept:page_2024_bun
    description: |
      Extracted from documentation: `.sh` file loader
  - name: "Implementation notes"
    id: concept:page_2025_bun
    description: |
      Extracted from documentation: Implementation notes
  - name: "Security in the Bun shell"
    id: concept:page_2026_bun
    description: |
      Extracted from documentation: Security in the Bun shell
  - name: "Argument injection"
    id: concept:page_2027_bun
    description: |
      Extracted from documentation: Argument injection
  - name: "Credits"
    id: concept:page_2028_bun
    description: |
      Extracted from documentation: Credits
  - name: "SQL"
    id: concept:page_2029_bun
    description: |
      Extracted from documentation: SQL
  - name: "Database Support"
    id: concept:page_2030_bun
    description: |
      Extracted from documentation: Database Support
  - name: "PostgreSQL"
    id: concept:page_2031_bun
    description: |
      Extracted from documentation: PostgreSQL
  - name: "MySQL"
    id: concept:page_2032_bun
    description: |
      Extracted from documentation: MySQL
  - name: "Inserting data"
    id: concept:page_2033_bun
    description: |
      Extracted from documentation: Inserting data
  - name: "Bulk Insert"
    id: concept:page_2034_bun
    description: |
      Extracted from documentation: Bulk Insert
  - name: "Picking columns to insert"
    id: concept:page_2035_bun
    description: |
      Extracted from documentation: Picking columns to insert
  - name: "Query Results"
    id: concept:page_2036_bun
    description: |
      Extracted from documentation: Query Results
  - name: "`sql``.values()` format"
    id: concept:page_2037_bun
    description: |
      Extracted from documentation: `sql``.values()` format
  - name: "`sql``.raw()` format"
    id: concept:page_2038_bun
    description: |
      Extracted from documentation: `sql``.raw()` format
  - name: "SQL Fragments"
    id: concept:page_2039_bun
    description: |
      Extracted from documentation: SQL Fragments
  - name: "Dynamic Table Names"
    id: concept:page_2040_bun
    description: |
      Extracted from documentation: Dynamic Table Names
  - name: "Conditional Queries"
    id: concept:page_2041_bun
    description: |
      Extracted from documentation: Conditional Queries
  - name: "Dynamic columns in updates"
    id: concept:page_2042_bun
    description: |
      Extracted from documentation: Dynamic columns in updates
  - name: "Dynamic values and `where in`"
    id: concept:page_2043_bun
    description: |
      Extracted from documentation: Dynamic values and `where in`
  - name: "`sql.array` helper"
    id: concept:page_2044_bun
    description: |
      Extracted from documentation: `sql.array` helper
  - name: "`sql``.simple()`"
    id: concept:page_2045_bun
    description: |
      Extracted from documentation: `sql``.simple()`
  - name: "Queries in files"
    id: concept:page_2046_bun
    description: |
      Extracted from documentation: Queries in files
  - name: "Unsafe Queries"
    id: concept:page_2047_bun
    description: |
      Extracted from documentation: Unsafe Queries
  - name: "Execute and Cancelling Queries"
    id: concept:page_2048_bun
    description: |
      Extracted from documentation: Execute and Cancelling Queries
  - name: "Database Environment Variables"
    id: concept:page_2049_bun
    description: |
      Extracted from documentation: Database Environment Variables
  - name: "Automatic Database Detection"
    id: concept:page_2050_bun
    description: |
      Extracted from documentation: Automatic Database Detection
  - name: "MySQL Auto-Detection"
    id: concept:page_2051_bun
    description: |
      Extracted from documentation: MySQL Auto-Detection
  - name: "SQLite Auto-Detection"
    id: concept:page_2052_bun
    description: |
      Extracted from documentation: SQLite Auto-Detection
  - name: "PostgreSQL Auto-Detection"
    id: concept:page_2053_bun
    description: |
      Extracted from documentation: PostgreSQL Auto-Detection
  - name: "PostgreSQL is detected for these patterns"
    id: concept:page_2054_bun
    description: |
      Extracted from documentation: PostgreSQL is detected for these patterns
  - name: "Or any URL that doesn't match MySQL or SQLite patterns"
    id: concept:page_2055_bun
    description: |
      Extracted from documentation: Or any URL that doesn't match MySQL or SQLite patterns
  - name: "MySQL Environment Variables"
    id: concept:page_2056_bun
    description: |
      Extracted from documentation: MySQL Environment Variables
  - name: "Primary connection URL (checked first)"
    id: concept:page_2057_bun
    description: |
      Extracted from documentation: Primary connection URL (checked first)
  - name: "Alternative: DATABASE_URL with MySQL protocol"
    id: concept:page_2058_bun
    description: |
      Extracted from documentation: Alternative: DATABASE_URL with MySQL protocol
  - name: "PostgreSQL Environment Variables"
    id: concept:page_2059_bun
    description: |
      Extracted from documentation: PostgreSQL Environment Variables
  - name: "SQLite Environment Variables"
    id: concept:page_2060_bun
    description: |
      Extracted from documentation: SQLite Environment Variables
  - name: "These are all recognized as SQLite"
    id: concept:page_2061_bun
    description: |
      Extracted from documentation: These are all recognized as SQLite
  - name: "Runtime Preconnection"
    id: concept:page_2062_bun
    description: |
      Extracted from documentation: Runtime Preconnection
  - name: "Enable PostgreSQL preconnection"
    id: concept:page_2063_bun
    description: |
      Extracted from documentation: Enable PostgreSQL preconnection
  - name: "Works with DATABASE_URL environment variable"
    id: concept:page_2064_bun
    description: |
      Extracted from documentation: Works with DATABASE_URL environment variable
  - name: "Can be combined with other runtime flags"
    id: concept:page_2065_bun
    description: |
      Extracted from documentation: Can be combined with other runtime flags
  - name: "MySQL Options"
    id: concept:page_2066_bun
    description: |
      Extracted from documentation: MySQL Options
  - name: "PostgreSQL Options"
    id: concept:page_2067_bun
    description: |
      Extracted from documentation: PostgreSQL Options
  - name: "SQLite Options"
    id: concept:page_2068_bun
    description: |
      Extracted from documentation: SQLite Options
  - name: "Dynamic passwords"
    id: concept:page_2069_bun
    description: |
      Extracted from documentation: Dynamic passwords
  - name: "SQLite-Specific Features"
    id: concept:page_2070_bun
    description: |
      Extracted from documentation: SQLite-Specific Features
  - name: "Query Execution"
    id: concept:page_2071_bun
    description: |
      Extracted from documentation: Query Execution
  - name: "SQLite Pragmas"
    id: concept:page_2072_bun
    description: |
      Extracted from documentation: SQLite Pragmas
  - name: "Data Type Differences"
    id: concept:page_2073_bun
    description: |
      Extracted from documentation: Data Type Differences
  - name: "Transactions"
    id: concept:page_2074_bun
    description: |
      Extracted from documentation: Transactions
  - name: "Basic Transactions"
    id: concept:page_2075_bun
    description: |
      Extracted from documentation: Basic Transactions
  - name: "Savepoints"
    id: concept:page_2076_bun
    description: |
      Extracted from documentation: Savepoints
  - name: "Distributed Transactions"
    id: concept:page_2077_bun
    description: |
      Extracted from documentation: Distributed Transactions
  - name: "Authentication"
    id: concept:page_2078_bun
    description: |
      Extracted from documentation: Authentication
  - name: "SSL Modes Overview"
    id: concept:page_2079_bun
    description: |
      Extracted from documentation: SSL Modes Overview
  - name: "Using With Connection Strings"
    id: concept:page_2080_bun
    description: |
      Extracted from documentation: Using With Connection Strings
  - name: "Connection Pooling"
    id: concept:page_2081_bun
    description: |
      Extracted from documentation: Connection Pooling
  - name: "Reserved Connections"
    id: concept:page_2082_bun
    description: |
      Extracted from documentation: Reserved Connections
  - name: "Prepared Statements"
    id: concept:page_2083_bun
    description: |
      Extracted from documentation: Prepared Statements
  - name: "Error Classes"
    id: concept:page_2084_bun
    description: |
      Extracted from documentation: Error Classes
  - name: "PostgreSQL Connection Errors"
    id: concept:page_2085_bun
    description: |
      Extracted from documentation: PostgreSQL Connection Errors
  - name: "Authentication Errors"
    id: concept:page_2086_bun
    description: |
      Extracted from documentation: Authentication Errors
  - name: "Query Errors"
    id: concept:page_2087_bun
    description: |
      Extracted from documentation: Query Errors
  - name: "Data Type Errors"
    id: concept:page_2088_bun
    description: |
      Extracted from documentation: Data Type Errors
  - name: "Protocol Errors"
    id: concept:page_2089_bun
    description: |
      Extracted from documentation: Protocol Errors
  - name: "Transaction Errors"
    id: concept:page_2090_bun
    description: |
      Extracted from documentation: Transaction Errors
  - name: "SQLite-Specific Errors"
    id: concept:page_2091_bun
    description: |
      Extracted from documentation: SQLite-Specific Errors
  - name: "Numbers and BigInt"
    id: concept:page_2092_bun
    description: |
      Extracted from documentation: Numbers and BigInt
  - name: "BigInt Instead of Strings"
    id: concept:page_2093_bun
    description: |
      Extracted from documentation: BigInt Instead of Strings
  - name: "Database-Specific Features"
    id: concept:page_2094_bun
    description: |
      Extracted from documentation: Database-Specific Features
  - name: "Authentication Methods"
    id: concept:page_2095_bun
    description: |
      Extracted from documentation: Authentication Methods
  - name: "Prepared Statements & Performance"
    id: concept:page_2096_bun
    description: |
      Extracted from documentation: Prepared Statements & Performance
  - name: "Multiple Result Sets"
    id: concept:page_2097_bun
    description: |
      Extracted from documentation: Multiple Result Sets
  - name: "Character Sets & Collations"
    id: concept:page_2098_bun
    description: |
      Extracted from documentation: Character Sets & Collations
  - name: "Connection Attributes"
    id: concept:page_2099_bun
    description: |
      Extracted from documentation: Connection Attributes
  - name: "Type Handling"
    id: concept:page_2100_bun
    description: |
      Extracted from documentation: Type Handling
  - name: "Differences from PostgreSQL"
    id: concept:page_2101_bun
    description: |
      Extracted from documentation: Differences from PostgreSQL
  - name: "MySQL-Specific Features"
    id: concept:page_2102_bun
    description: |
      Extracted from documentation: MySQL-Specific Features
  - name: "PostgreSQL-Specific Features"
    id: concept:page_2103_bun
    description: |
      Extracted from documentation: PostgreSQL-Specific Features
  - name: "Common Patterns & Best Practices"
    id: concept:page_2104_bun
    description: |
      Extracted from documentation: Common Patterns & Best Practices
  - name: "Working with MySQL Result Sets"
    id: concept:page_2105_bun
    description: |
      Extracted from documentation: Working with MySQL Result Sets
  - name: "MySQL Error Handling"
    id: concept:page_2106_bun
    description: |
      Extracted from documentation: MySQL Error Handling
  - name: "Performance Tips for MySQL"
    id: concept:page_2107_bun
    description: |
      Extracted from documentation: Performance Tips for MySQL
  - name: "Frequently Asked Questions"
    id: concept:page_2108_bun
    description: |
      Extracted from documentation: Frequently Asked Questions
  - name: "Why not just use an existing library?"
    id: concept:page_2109_bun
    description: |
      Extracted from documentation: Why not just use an existing library?
  - name: "Database"
    id: concept:page_2110_bun
    description: |
      Extracted from documentation: Database
  - name: "Strict mode"
    id: concept:page_2111_bun
    description: |
      Extracted from documentation: Strict mode
  - name: "Load via ES module import"
    id: concept:page_2112_bun
    description: |
      Extracted from documentation: Load via ES module import
  - name: "`.close(throwOnError: boolean = false)`"
    id: concept:page_2113_bun
    description: |
      Extracted from documentation: `.close(throwOnError: boolean = false)`
  - name: "`using` statement"
    id: concept:page_2114_bun
    description: |
      Extracted from documentation: `using` statement
  - name: "`.serialize()`"
    id: concept:page_2115_bun
    description: |
      Extracted from documentation: `.serialize()`
  - name: "`.query()`"
    id: concept:page_2116_bun
    description: |
      Extracted from documentation: `.query()`
  - name: "WAL mode"
    id: concept:page_2117_bun
    description: |
      Extracted from documentation: WAL mode
  - name: "WAL sidecar file cleanup"
    id: concept:page_2118_bun
    description: |
      Extracted from documentation: WAL sidecar file cleanup
  - name: "Statements"
    id: concept:page_2119_bun
    description: |
      Extracted from documentation: Statements
  - name: "Binding values"
    id: concept:page_2120_bun
    description: |
      Extracted from documentation: Binding values
  - name: "`strict: true` lets you bind values without prefixes"
    id: concept:page_2121_bun
    description: |
      Extracted from documentation: `strict: true` lets you bind values without prefixes
  - name: "`.all()`"
    id: concept:page_2122_bun
    description: |
      Extracted from documentation: `.all()`
  - name: "`.get()`"
    id: concept:page_2123_bun
    description: |
      Extracted from documentation: `.get()`
  - name: "`.run()`"
    id: concept:page_2124_bun
    description: |
      Extracted from documentation: `.run()`
  - name: "`.as(Class)` - Map query results to a class"
    id: concept:page_2125_bun
    description: |
      Extracted from documentation: `.as(Class)` - Map query results to a class
  - name: "`.iterate()` (`@@iterator`)"
    id: concept:page_2126_bun
    description: |
      Extracted from documentation: `.iterate()` (`@@iterator`)
  - name: "`.values()`"
    id: concept:page_2127_bun
    description: |
      Extracted from documentation: `.values()`
  - name: "`.finalize()`"
    id: concept:page_2128_bun
    description: |
      Extracted from documentation: `.finalize()`
  - name: "`.toString()`"
    id: concept:page_2129_bun
    description: |
      Extracted from documentation: `.toString()`
  - name: "Integers"
    id: concept:page_2130_bun
    description: |
      Extracted from documentation: Integers
  - name: "`safeIntegers: true`"
    id: concept:page_2131_bun
    description: |
      Extracted from documentation: `safeIntegers: true`
  - name: "`safeIntegers: false` (default)"
    id: concept:page_2132_bun
    description: |
      Extracted from documentation: `safeIntegers: false` (default)
  - name: "`.loadExtension()`"
    id: concept:page_2133_bun
    description: |
      Extracted from documentation: `.loadExtension()`
  - name: "`.fileControl(cmd: number, value: any)`"
    id: concept:page_2134_bun
    description: |
      Extracted from documentation: `.fileControl(cmd: number, value: any)`
  - name: "Datatypes"
    id: concept:page_2135_bun
    description: |
      Extracted from documentation: Datatypes
  - name: "Direct `ReadableStream`"
    id: concept:page_2136_bun
    description: |
      Extracted from documentation: Direct `ReadableStream`
  - name: "Handling backpressure"
    id: concept:page_2137_bun
    description: |
      Extracted from documentation: Handling backpressure
  - name: "Async generator streams"
    id: concept:page_2138_bun
    description: |
      Extracted from documentation: Async generator streams
  - name: "`Bun.ArrayBufferSink`"
    id: concept:page_2139_bun
    description: |
      Extracted from documentation: `Bun.ArrayBufferSink`
  - name: "bun create"
    id: concept:page_2140_bun
    description: |
      Extracted from documentation: bun create
  - name: "From a React component"
    id: concept:page_2141_bun
    description: |
      Extracted from documentation: From a React component
  - name: "Using TailwindCSS with Bun"
    id: concept:page_2142_bun
    description: |
      Extracted from documentation: Using TailwindCSS with Bun
  - name: "Using `shadcn/ui` with Bun"
    id: concept:page_2143_bun
    description: |
      Extracted from documentation: Using `shadcn/ui` with Bun
  - name: "Assuming bun detected imports to @/components/ui/accordion and @/components/ui/button"
    id: concept:page_2144_bun
    description: |
      Extracted from documentation: Assuming bun detected imports to @/components/ui/accordion and @/components/ui/button
  - name: "From `npm`"
    id: concept:page_2145_bun
    description: |
      Extracted from documentation: From `npm`
  - name: "From GitHub"
    id: concept:page_2146_bun
    description: |
      Extracted from documentation: From GitHub
  - name: "From a local template"
    id: concept:page_2147_bun
    description: |
      Extracted from documentation: From a local template
  - name: "Setup logic"
    id: concept:page_2148_bun
    description: |
      Extracted from documentation: Setup logic
  - name: "CLI flags"
    id: concept:page_2149_bun
    description: |
      Extracted from documentation: CLI flags
  - name: "bun init"
    id: concept:page_2150_bun
    description: |
      Extracted from documentation: bun init
  - name: "Initialization Options"
    id: concept:page_2151_bun
    description: |
      Extracted from documentation: Initialization Options
  - name: "Project Templates"
    id: concept:page_2152_bun
    description: |
      Extracted from documentation: Project Templates
  - name: "Output & Files"
    id: concept:page_2153_bun
    description: |
      Extracted from documentation: Output & Files
  - name: "TOML"
    id: concept:page_2154_bun
    description: |
      Extracted from documentation: TOML
  - name: "`Bun.TOML.parse()`"
    id: concept:page_2155_bun
    description: |
      Extracted from documentation: `Bun.TOML.parse()`
  - name: "Supported TOML Features"
    id: concept:page_2156_bun
    description: |
      Extracted from documentation: Supported TOML Features
  - name: "Application config"
    id: concept:page_2157_bun
    description: |
      Extracted from documentation: Application config
  - name: "Import Attributes"
    id: concept:page_2158_bun
    description: |
      Extracted from documentation: Import Attributes
  - name: "Hot Reloading with TOML"
    id: concept:page_2159_bun
    description: |
      Extracted from documentation: Hot Reloading with TOML
  - name: "Transpiler"
    id: concept:page_2160_bun
    description: |
      Extracted from documentation: Transpiler
  - name: "`.transformSync()`"
    id: concept:page_2161_bun
    description: |
      Extracted from documentation: `.transformSync()`
  - name: "`.transform()`"
    id: concept:page_2162_bun
    description: |
      Extracted from documentation: `.transform()`
  - name: "`.scan()`"
    id: concept:page_2163_bun
    description: |
      Extracted from documentation: `.scan()`
  - name: "`.scanImports()`"
    id: concept:page_2164_bun
    description: |
      Extracted from documentation: `.scanImports()`
  - name: "Utils"
    id: concept:page_2165_bun
    description: |
      Extracted from documentation: Utils
  - name: "`Bun.version`"
    id: concept:page_2166_bun
    description: |
      Extracted from documentation: `Bun.version`
  - name: "`Bun.revision`"
    id: concept:page_2167_bun
    description: |
      Extracted from documentation: `Bun.revision`
  - name: "`Bun.env`"
    id: concept:page_2168_bun
    description: |
      Extracted from documentation: `Bun.env`
  - name: "`Bun.main`"
    id: concept:page_2169_bun
    description: |
      Extracted from documentation: `Bun.main`
  - name: "`Bun.sleep()`"
    id: concept:page_2170_bun
    description: |
      Extracted from documentation: `Bun.sleep()`
  - name: "`Bun.sleepSync()`"
    id: concept:page_2171_bun
    description: |
      Extracted from documentation: `Bun.sleepSync()`
  - name: "`Bun.which()`"
    id: concept:page_2172_bun
    description: |
      Extracted from documentation: `Bun.which()`
  - name: "`Bun.randomUUIDv7()`"
    id: concept:page_2173_bun
    description: |
      Extracted from documentation: `Bun.randomUUIDv7()`
  - name: "`Bun.peek()`"
    id: concept:page_2174_bun
    description: |
      Extracted from documentation: `Bun.peek()`
  - name: "`Bun.openInEditor()`"
    id: concept:page_2175_bun
    description: |
      Extracted from documentation: `Bun.openInEditor()`
  - name: "`Bun.deepEquals()`"
    id: concept:page_2176_bun
    description: |
      Extracted from documentation: `Bun.deepEquals()`
  - name: "`Bun.escapeHTML()`"
    id: concept:page_2177_bun
    description: |
      Extracted from documentation: `Bun.escapeHTML()`
  - name: "`Bun.stringWidth()`"
    id: concept:page_2178_bun
    description: |
      Extracted from documentation: `Bun.stringWidth()`
  - name: "`Bun.fileURLToPath()`"
    id: concept:page_2179_bun
    description: |
      Extracted from documentation: `Bun.fileURLToPath()`
  - name: "`Bun.pathToFileURL()`"
    id: concept:page_2180_bun
    description: |
      Extracted from documentation: `Bun.pathToFileURL()`
  - name: "`Bun.gzipSync()`"
    id: concept:page_2181_bun
    description: |
      Extracted from documentation: `Bun.gzipSync()`
  - name: "`Bun.gunzipSync()`"
    id: concept:page_2182_bun
    description: |
      Extracted from documentation: `Bun.gunzipSync()`
  - name: "`Bun.deflateSync()`"
    id: concept:page_2183_bun
    description: |
      Extracted from documentation: `Bun.deflateSync()`
  - name: "`Bun.inflateSync()`"
    id: concept:page_2184_bun
    description: |
      Extracted from documentation: `Bun.inflateSync()`
  - name: "`Bun.zstdCompress()` / `Bun.zstdCompressSync()`"
    id: concept:page_2185_bun
    description: |
      Extracted from documentation: `Bun.zstdCompress()` / `Bun.zstdCompressSync()`
  - name: "`Bun.zstdDecompress()` / `Bun.zstdDecompressSync()`"
    id: concept:page_2186_bun
    description: |
      Extracted from documentation: `Bun.zstdDecompress()` / `Bun.zstdDecompressSync()`
  - name: "`Bun.inspect()`"
    id: concept:page_2187_bun
    description: |
      Extracted from documentation: `Bun.inspect()`
  - name: "`Bun.inspect.custom`"
    id: concept:page_2188_bun
    description: |
      Extracted from documentation: `Bun.inspect.custom`
  - name: "`Bun.inspect.table(tabularData, properties, options)`"
    id: concept:page_2189_bun
    description: |
      Extracted from documentation: `Bun.inspect.table(tabularData, properties, options)`
  - name: "`Bun.nanoseconds()`"
    id: concept:page_2190_bun
    description: |
      Extracted from documentation: `Bun.nanoseconds()`
  - name: "`Bun.readableStreamTo*()`"
    id: concept:page_2191_bun
    description: |
      Extracted from documentation: `Bun.readableStreamTo*()`
  - name: "`Bun.resolveSync()`"
    id: concept:page_2192_bun
    description: |
      Extracted from documentation: `Bun.resolveSync()`
  - name: "`Bun.stripANSI()`"
    id: concept:page_2193_bun
    description: |
      Extracted from documentation: `Bun.stripANSI()`
  - name: "`Bun.wrapAnsi()`"
    id: concept:page_2194_bun
    description: |
      Extracted from documentation: `Bun.wrapAnsi()`
  - name: "`serialize` & `deserialize` in `bun:jsc`"
    id: concept:page_2195_bun
    description: |
      Extracted from documentation: `serialize` & `deserialize` in `bun:jsc`
  - name: "`estimateShallowMemoryUsageOf` in `bun:jsc`"
    id: concept:page_2196_bun
    description: |
      Extracted from documentation: `estimateShallowMemoryUsageOf` in `bun:jsc`
  - name: "`--watch` mode"
    id: concept:page_2197_bun
    description: |
      Extracted from documentation: `--watch` mode
  - name: "`--hot` mode"
    id: concept:page_2198_bun
    description: |
      Extracted from documentation: `--hot` mode
  - name: "HTTP servers"
    id: concept:page_2199_bun
    description: |
      Extracted from documentation: HTTP servers
  - name: "Web APIs"
    id: concept:page_2200_bun
    description: |
      Extracted from documentation: Web APIs
  - name: "WebView"
    id: concept:page_2201_bun
    description: |
      Extracted from documentation: WebView
  - name: "Creating a view"
    id: concept:page_2202_bun
    description: |
      Extracted from documentation: Creating a view
  - name: "Automatic cleanup with `using`"
    id: concept:page_2203_bun
    description: |
      Extracted from documentation: Automatic cleanup with `using`
  - name: "Persistent storage"
    id: concept:page_2204_bun
    description: |
      Extracted from documentation: Persistent storage
  - name: "Backends"
    id: concept:page_2205_bun
    description: |
      Extracted from documentation: Backends
  - name: "How the WebKit backend works"
    id: concept:page_2206_bun
    description: |
      Extracted from documentation: How the WebKit backend works
  - name: "How the Chrome backend works"
    id: concept:page_2207_bun
    description: |
      Extracted from documentation: How the Chrome backend works
  - name: "Finding the Chrome executable"
    id: concept:page_2208_bun
    description: |
      Extracted from documentation: Finding the Chrome executable
  - name: "Launch flags"
    id: concept:page_2209_bun
    description: |
      Extracted from documentation: Launch flags
  - name: "Subprocess output"
    id: concept:page_2210_bun
    description: |
      Extracted from documentation: Subprocess output
  - name: "Navigation"
    id: concept:page_2211_bun
    description: |
      Extracted from documentation: Navigation
  - name: "Navigation callbacks"
    id: concept:page_2212_bun
    description: |
      Extracted from documentation: Navigation callbacks
  - name: "Evaluating JavaScript"
    id: concept:page_2213_bun
    description: |
      Extracted from documentation: Evaluating JavaScript
  - name: "Screenshots"
    id: concept:page_2214_bun
    description: |
      Extracted from documentation: Screenshots
  - name: "Image format"
    id: concept:page_2215_bun
    description: |
      Extracted from documentation: Image format
  - name: "Return type"
    id: concept:page_2216_bun
    description: |
      Extracted from documentation: Return type
  - name: "Shared memory for terminal graphics"
    id: concept:page_2217_bun
    description: |
      Extracted from documentation: Shared memory for terminal graphics
  - name: "Input simulation"
    id: concept:page_2218_bun
    description: |
      Extracted from documentation: Input simulation
  - name: "Clicking"
    id: concept:page_2219_bun
    description: |
      Extracted from documentation: Clicking
  - name: "Clicking by selector"
    id: concept:page_2220_bun
    description: |
      Extracted from documentation: Clicking by selector
  - name: "Typing text"
    id: concept:page_2221_bun
    description: |
      Extracted from documentation: Typing text
  - name: "Pressing keys"
    id: concept:page_2222_bun
    description: |
      Extracted from documentation: Pressing keys
  - name: "Scrolling"
    id: concept:page_2223_bun
    description: |
      Extracted from documentation: Scrolling
  - name: "Resizing"
    id: concept:page_2224_bun
    description: |
      Extracted from documentation: Resizing
  - name: "Console capture"
    id: concept:page_2225_bun
    description: |
      Extracted from documentation: Console capture
  - name: "Mirror to Bun's console"
    id: concept:page_2226_bun
    description: |
      Extracted from documentation: Mirror to Bun's console
  - name: "Custom handler"
    id: concept:page_2227_bun
    description: |
      Extracted from documentation: Custom handler
  - name: "Sending commands"
    id: concept:page_2228_bun
    description: |
      Extracted from documentation: Sending commands
  - name: "Subscribing to events"
    id: concept:page_2229_bun
    description: |
      Extracted from documentation: Subscribing to events
  - name: "Lifecycle"
    id: concept:page_2230_bun
    description: |
      Extracted from documentation: Lifecycle
  - name: "Closing a view"
    id: concept:page_2231_bun
    description: |
      Extracted from documentation: Closing a view
  - name: "Killing all browsers"
    id: concept:page_2232_bun
    description: |
      Extracted from documentation: Killing all browsers
  - name: "Event-loop behavior"
    id: concept:page_2233_bun
    description: |
      Extracted from documentation: Event-loop behavior
  - name: "Subprocess death"
    id: concept:page_2234_bun
    description: |
      Extracted from documentation: Subprocess death
  - name: "Concurrency model"
    id: concept:page_2235_bun
    description: |
      Extracted from documentation: Concurrency model
  - name: "`new Bun.WebView(options?)`"
    id: concept:page_2236_bun
    description: |
      Extracted from documentation: `new Bun.WebView(options?)`
  - name: "`backend` object form"
    id: concept:page_2237_bun
    description: |
      Extracted from documentation: `backend` object form
  - name: "Instance properties"
    id: concept:page_2238_bun
    description: |
      Extracted from documentation: Instance properties
  - name: "Instance methods"
    id: concept:page_2239_bun
    description: |
      Extracted from documentation: Instance methods
  - name: "`click()` options"
    id: concept:page_2240_bun
    description: |
      Extracted from documentation: `click()` options
  - name: "`press()` options"
    id: concept:page_2241_bun
    description: |
      Extracted from documentation: `press()` options
  - name: "`scrollTo()` options"
    id: concept:page_2242_bun
    description: |
      Extracted from documentation: `scrollTo()` options
  - name: "`screenshot()` options"
    id: concept:page_2243_bun
    description: |
      Extracted from documentation: `screenshot()` options
  - name: "Workers"
    id: concept:page_2244_bun
    description: |
      Extracted from documentation: Workers
  - name: "Creating a `Worker`"
    id: concept:page_2245_bun
    description: |
      Extracted from documentation: Creating a `Worker`
  - name: "From the main thread"
    id: concept:page_2246_bun
    description: |
      Extracted from documentation: From the main thread
  - name: "Worker thread"
    id: concept:page_2247_bun
    description: |
      Extracted from documentation: Worker thread
  - name: "`preload` - load modules before the worker starts"
    id: concept:page_2248_bun
    description: |
      Extracted from documentation: `preload` - load modules before the worker starts
  - name: "`blob:` URLs"
    id: concept:page_2249_bun
    description: |
      Extracted from documentation: `blob:` URLs
  - name: "`\"open\"`"
    id: concept:page_2250_bun
    description: |
      Extracted from documentation: `"open"`
  - name: "Messages with `postMessage`"
    id: concept:page_2251_bun
    description: |
      Extracted from documentation: Messages with `postMessage`
  - name: "Performance optimizations"
    id: concept:page_2252_bun
    description: |
      Extracted from documentation: Performance optimizations
  - name: "Terminating a worker"
    id: concept:page_2253_bun
    description: |
      Extracted from documentation: Terminating a worker
  - name: "`process.exit()`"
    id: concept:page_2254_bun
    description: |
      Extracted from documentation: `process.exit()`
  - name: "`\"close\"`"
    id: concept:page_2255_bun
    description: |
      Extracted from documentation: `"close"`
  - name: "Managing lifetime"
    id: concept:page_2256_bun
    description: |
      Extracted from documentation: Managing lifetime
  - name: "`worker.unref()`"
    id: concept:page_2257_bun
    description: |
      Extracted from documentation: `worker.unref()`
  - name: "`worker.ref()`"
    id: concept:page_2258_bun
    description: |
      Extracted from documentation: `worker.ref()`
  - name: "Memory usage with `smol`"
    id: concept:page_2259_bun
    description: |
      Extracted from documentation: Memory usage with `smol`
  - name: "Environment Data"
    id: concept:page_2260_bun
    description: |
      Extracted from documentation: Environment Data
  - name: "Worker Events"
    id: concept:page_2261_bun
    description: |
      Extracted from documentation: Worker Events
  - name: "`Bun.isMainThread`"
    id: concept:page_2262_bun
    description: |
      Extracted from documentation: `Bun.isMainThread`
  - name: "YAML"
    id: concept:page_2263_bun
    description: |
      Extracted from documentation: YAML
  - name: "`Bun.YAML.parse()`"
    id: concept:page_2264_bun
    description: |
      Extracted from documentation: `Bun.YAML.parse()`
  - name: "Multi-document YAML"
    id: concept:page_2265_bun
    description: |
      Extracted from documentation: Multi-document YAML
  - name: "Supported YAML Features"
    id: concept:page_2266_bun
    description: |
      Extracted from documentation: Supported YAML Features
  - name: "Employee record"
    id: concept:page_2267_bun
    description: |
      Extracted from documentation: Employee record
  - name: "Hot Reloading with YAML"
    id: concept:page_2268_bun
    description: |
      Extracted from documentation: Hot Reloading with YAML
  - name: "Configuration Hot Reloading"
    id: concept:page_2269_bun
    description: |
      Extracted from documentation: Configuration Hot Reloading
  - name: "Configuration Management"
    id: concept:page_2270_bun
    description: |
      Extracted from documentation: Configuration Management
  - name: "Feature Flags Configuration"
    id: concept:page_2271_bun
    description: |
      Extracted from documentation: Feature Flags Configuration
  - name: "Database Configuration"
    id: concept:page_2272_bun
    description: |
      Extracted from documentation: Database Configuration
  - name: "Code coverage"
    id: concept:page_2273_bun
    description: |
      Extracted from documentation: Code coverage
  - name: "Enabling Coverage"
    id: concept:page_2274_bun
    description: |
      Extracted from documentation: Enabling Coverage
  - name: "Enable by Default"
    id: concept:page_2275_bun
    description: |
      Extracted from documentation: Enable by Default
  - name: "Always enable coverage"
    id: concept:page_2276_bun
    description: |
      Extracted from documentation: Always enable coverage
  - name: "Coverage Thresholds"
    id: concept:page_2277_bun
    description: |
      Extracted from documentation: Coverage Thresholds
  - name: "Simple Threshold"
    id: concept:page_2278_bun
    description: |
      Extracted from documentation: Simple Threshold
  - name: "To require 90% line-level and function-level coverage"
    id: concept:page_2279_bun
    description: |
      Extracted from documentation: To require 90% line-level and function-level coverage
  - name: "Detailed Thresholds"
    id: concept:page_2280_bun
    description: |
      Extracted from documentation: Detailed Thresholds
  - name: "To set different thresholds for lines and functions"
    id: concept:page_2281_bun
    description: |
      Extracted from documentation: To set different thresholds for lines and functions
  - name: "Coverage Reporters"
    id: concept:page_2282_bun
    description: |
      Extracted from documentation: Coverage Reporters
  - name: "Available Reporters"
    id: concept:page_2283_bun
    description: |
      Extracted from documentation: Available Reporters
  - name: "LCOV Coverage Reporter"
    id: concept:page_2284_bun
    description: |
      Extracted from documentation: LCOV Coverage Reporter
  - name: "Or via CLI"
    id: concept:page_2285_bun
    description: |
      Extracted from documentation: Or via CLI
  - name: "Using LCOV with GitHub Actions"
    id: concept:page_2286_bun
    description: |
      Extracted from documentation: Using LCOV with GitHub Actions
  - name: "Excluding Files from Coverage"
    id: concept:page_2287_bun
    description: |
      Extracted from documentation: Excluding Files from Coverage
  - name: "Skip Test Files"
    id: concept:page_2288_bun
    description: |
      Extracted from documentation: Skip Test Files
  - name: "Ignore Specific Paths and Patterns"
    id: concept:page_2289_bun
    description: |
      Extracted from documentation: Ignore Specific Paths and Patterns
  - name: "Common Use Cases"
    id: concept:page_2290_bun
    description: |
      Extracted from documentation: Common Use Cases
  - name: "Exclude utility files"
    id: concept:page_2291_bun
    description: |
      Extracted from documentation: Exclude utility files
  - name: "Exclude configuration files"
    id: concept:page_2292_bun
    description: |
      Extracted from documentation: Exclude configuration files
  - name: "Exclude specific test patterns"
    id: concept:page_2293_bun
    description: |
      Extracted from documentation: Exclude specific test patterns
  - name: "Exclude build artifacts"
    id: concept:page_2294_bun
    description: |
      Extracted from documentation: Exclude build artifacts
  - name: "Exclude generated files"
    id: concept:page_2295_bun
    description: |
      Extracted from documentation: Exclude generated files
  - name: "Exclude vendor/third-party code"
    id: concept:page_2296_bun
    description: |
      Extracted from documentation: Exclude vendor/third-party code
  - name: "Sourcemaps"
    id: concept:page_2297_bun
    description: |
      Extracted from documentation: Sourcemaps
  - name: "Coverage Defaults"
    id: concept:page_2298_bun
    description: |
      Extracted from documentation: Coverage Defaults
  - name: "Advanced Configuration"
    id: concept:page_2299_bun
    description: |
      Extracted from documentation: Advanced Configuration
  - name: "Custom Coverage Directory"
    id: concept:page_2300_bun
    description: |
      Extracted from documentation: Custom Coverage Directory
  - name: "Multiple Reporters"
    id: concept:page_2301_bun
    description: |
      Extracted from documentation: Multiple Reporters
  - name: "Coverage with Specific Test Patterns"
    id: concept:page_2302_bun
    description: |
      Extracted from documentation: Coverage with Specific Test Patterns
  - name: "Run coverage only on specific test files"
    id: concept:page_2303_bun
    description: |
      Extracted from documentation: Run coverage only on specific test files
  - name: "Run coverage with name pattern"
    id: concept:page_2304_bun
    description: |
      Extracted from documentation: Run coverage with name pattern
  - name: "CI/CD Integration"
    id: concept:page_2305_bun
    description: |
      Extracted from documentation: CI/CD Integration
  - name: "GitHub Actions Example"
    id: concept:page_2306_bun
    description: |
      Extracted from documentation: GitHub Actions Example
  - name: "GitLab CI Example"
    id: concept:page_2307_bun
    description: |
      Extracted from documentation: GitLab CI Example
  - name: "Interpreting Coverage Reports"
    id: concept:page_2308_bun
    description: |
      Extracted from documentation: Interpreting Coverage Reports
  - name: "Text Output Explanation"
    id: concept:page_2309_bun
    description: |
      Extracted from documentation: Text Output Explanation
  - name: "What to Aim For"
    id: concept:page_2310_bun
    description: |
      Extracted from documentation: What to Aim For
  - name: "Focus on Quality, Not Just Quantity"
    id: concept:page_2311_bun
    description: |
      Extracted from documentation: Focus on Quality, Not Just Quantity
  - name: "Test Edge Cases"
    id: concept:page_2312_bun
    description: |
      Extracted from documentation: Test Edge Cases
  - name: "Use Coverage to Find Missing Tests"
    id: concept:page_2313_bun
    description: |
      Extracted from documentation: Use Coverage to Find Missing Tests
  - name: "Run coverage to identify untested code"
    id: concept:page_2314_bun
    description: |
      Extracted from documentation: Run coverage to identify untested code
  - name: "Look at specific files that need attention"
    id: concept:page_2315_bun
    description: |
      Extracted from documentation: Look at specific files that need attention
  - name: "Combine with Other Quality Metrics"
    id: concept:page_2316_bun
    description: |
      Extracted from documentation: Combine with Other Quality Metrics
  - name: "Coverage Not Showing for Some Files"
    id: concept:page_2317_bun
    description: |
      Extracted from documentation: Coverage Not Showing for Some Files
  - name: "False Coverage Reports"
    id: concept:page_2318_bun
    description: |
      Extracted from documentation: False Coverage Reports
  - name: "Performance Issues with Large Codebases"
    id: concept:page_2319_bun
    description: |
      Extracted from documentation: Performance Issues with Large Codebases
  - name: "Exclude large directories you don't need coverage for"
    id: concept:page_2320_bun
    description: |
      Extracted from documentation: Exclude large directories you don't need coverage for
  - name: "Test configuration"
    id: concept:page_2321_bun
    description: |
      Extracted from documentation: Test configuration
  - name: "Configuration File"
    id: concept:page_2322_bun
    description: |
      Extracted from documentation: Configuration File
  - name: "Options go here"
    id: concept:page_2323_bun
    description: |
      Extracted from documentation: Options go here
  - name: "Test Discovery"
    id: concept:page_2324_bun
    description: |
      Extracted from documentation: Test Discovery
  - name: "Only run tests in the src directory"
    id: concept:page_2325_bun
    description: |
      Extracted from documentation: Only run tests in the src directory
  - name: "Run tests in a specific test directory"
    id: concept:page_2326_bun
    description: |
      Extracted from documentation: Run tests in a specific test directory
  - name: "Run tests in multiple specific directories (not currently supported - use patterns instead)"
    id: concept:page_2327_bun
    description: |
      Extracted from documentation: Run tests in multiple specific directories (not currently supported - use patterns instead)
  - name: "root = [\"src\", \"lib\"]  # This syntax is not supported"
    id: concept:page_2328_bun
    description: |
      Extracted from documentation: root = ["src", "lib"]  # This syntax is not supported
  - name: "Preload Scripts"
    id: concept:page_2329_bun
    description: |
      Extracted from documentation: Preload Scripts
  - name: "Common Preload Use Cases"
    id: concept:page_2330_bun
    description: |
      Extracted from documentation: Common Preload Use Cases
  - name: "Path Ignore Patterns"
    id: concept:page_2331_bun
    description: |
      Extracted from documentation: Path Ignore Patterns
  - name: "Git submodules with their own test suites"
    id: concept:page_2332_bun
    description: |
      Extracted from documentation: Git submodules with their own test suites
  - name: "Vendored dependencies"
    id: concept:page_2333_bun
    description: |
      Extracted from documentation: Vendored dependencies
  - name: "Test fixtures that look like tests but aren't"
    id: concept:page_2334_bun
    description: |
      Extracted from documentation: Test fixtures that look like tests but aren't
  - name: "Integration / E2E tests you want to run separately"
    id: concept:page_2335_bun
    description: |
      Extracted from documentation: Integration / E2E tests you want to run separately
  - name: "Reporters"
    id: concept:page_2336_bun
    description: |
      Extracted from documentation: Reporters
  - name: "JUnit Reporter"
    id: concept:page_2337_bun
    description: |
      Extracted from documentation: JUnit Reporter
  - name: "Equivalent command line usage"
    id: concept:page_2338_bun
    description: |
      Extracted from documentation: Equivalent command line usage
  - name: "CLI approach"
    id: concept:page_2339_bun
    description: |
      Extracted from documentation: CLI approach
  - name: "Config file approach"
    id: concept:page_2340_bun
    description: |
      Extracted from documentation: Config file approach
  - name: "Also enable coverage reporting"
    id: concept:page_2341_bun
    description: |
      Extracted from documentation: Also enable coverage reporting
  - name: "Memory Usage"
    id: concept:page_2342_bun
    description: |
      Extracted from documentation: Memory Usage
  - name: "smol Mode"
    id: concept:page_2343_bun
    description: |
      Extracted from documentation: smol Mode
  - name: "Test execution"
    id: concept:page_2344_bun
    description: |
      Extracted from documentation: Test execution
  - name: "concurrentTestGlob"
    id: concept:page_2345_bun
    description: |
      Extracted from documentation: concurrentTestGlob
  - name: "randomize"
    id: concept:page_2346_bun
    description: |
      Extracted from documentation: randomize
  - name: "seed"
    id: concept:page_2347_bun
    description: |
      Extracted from documentation: seed
  - name: "retry"
    id: concept:page_2348_bun
    description: |
      Extracted from documentation: retry
  - name: "rerunEach"
    id: concept:page_2349_bun
    description: |
      Extracted from documentation: rerunEach
  - name: "Coverage Options"
    id: concept:page_2350_bun
    description: |
      Extracted from documentation: Coverage Options
  - name: "Basic Coverage Settings"
    id: concept:page_2351_bun
    description: |
      Extracted from documentation: Basic Coverage Settings
  - name: "Enable coverage by default"
    id: concept:page_2352_bun
    description: |
      Extracted from documentation: Enable coverage by default
  - name: "Set coverage reporter"
    id: concept:page_2353_bun
    description: |
      Extracted from documentation: Set coverage reporter
  - name: "Set coverage output directory"
    id: concept:page_2354_bun
    description: |
      Extracted from documentation: Set coverage output directory
  - name: "Skip Test Files from Coverage"
    id: concept:page_2355_bun
    description: |
      Extracted from documentation: Skip Test Files from Coverage
  - name: "Simple threshold - applies to lines, functions, and statements"
    id: concept:page_2356_bun
    description: |
      Extracted from documentation: Simple threshold - applies to lines, functions, and statements
  - name: "Detailed thresholds"
    id: concept:page_2357_bun
    description: |
      Extracted from documentation: Detailed thresholds
  - name: "Threshold Examples"
    id: concept:page_2358_bun
    description: |
      Extracted from documentation: Threshold Examples
  - name: "Require 90% coverage across the board"
    id: concept:page_2359_bun
    description: |
      Extracted from documentation: Require 90% coverage across the board
  - name: "Different requirements for different metrics"
    id: concept:page_2360_bun
    description: |
      Extracted from documentation: Different requirements for different metrics
  - name: "Coverage Path Ignore Patterns"
    id: concept:page_2361_bun
    description: |
      Extracted from documentation: Coverage Path Ignore Patterns
  - name: "Common Ignore Patterns"
    id: concept:page_2362_bun
    description: |
      Extracted from documentation: Common Ignore Patterns
  - name: "Test files"
    id: concept:page_2363_bun
    description: |
      Extracted from documentation: Test files
  - name: "Configuration files"
    id: concept:page_2364_bun
    description: |
      Extracted from documentation: Configuration files
  - name: "Build output"
    id: concept:page_2365_bun
    description: |
      Extracted from documentation: Build output
  - name: "Generated code"
    id: concept:page_2366_bun
    description: |
      Extracted from documentation: Generated code
  - name: "Vendor/third-party"
    id: concept:page_2367_bun
    description: |
      Extracted from documentation: Vendor/third-party
  - name: "Utilities that don't need testing"
    id: concept:page_2368_bun
    description: |
      Extracted from documentation: Utilities that don't need testing
  - name: "Sourcemap Handling"
    id: concept:page_2369_bun
    description: |
      Extracted from documentation: Sourcemap Handling
  - name: "Install Settings Inheritance"
    id: concept:page_2370_bun
    description: |
      Extracted from documentation: Install Settings Inheritance
  - name: "These settings are inherited by bun test"
    id: concept:page_2371_bun
    description: |
      Extracted from documentation: These settings are inherited by bun test
  - name: "Test-specific configuration"
    id: concept:page_2372_bun
    description: |
      Extracted from documentation: Test-specific configuration
  - name: "Complete Configuration Example"
    id: concept:page_2373_bun
    description: |
      Extracted from documentation: Complete Configuration Example
  - name: "Install settings inherited by tests"
    id: concept:page_2374_bun
    description: |
      Extracted from documentation: Install settings inherited by tests
  - name: "Test discovery"
    id: concept:page_2375_bun
    description: |
      Extracted from documentation: Test discovery
  - name: "Execution settings"
    id: concept:page_2376_bun
    description: |
      Extracted from documentation: Execution settings
  - name: "Coverage configuration"
    id: concept:page_2377_bun
    description: |
      Extracted from documentation: Coverage configuration
  - name: "Advanced coverage settings"
    id: concept:page_2378_bun
    description: |
      Extracted from documentation: Advanced coverage settings
  - name: "Reporter configuration"
    id: concept:page_2379_bun
    description: |
      Extracted from documentation: Reporter configuration
  - name: "CLI Override Behavior"
    id: concept:page_2380_bun
    description: |
      Extracted from documentation: CLI Override Behavior
  - name: "This CLI flag overrides the config file"
    id: concept:page_2381_bun
    description: |
      Extracted from documentation: This CLI flag overrides the config file
  - name: "coverage will be enabled"
    id: concept:page_2382_bun
    description: |
      Extracted from documentation: coverage will be enabled
  - name: "Dates and times"
    id: concept:page_2383_bun
    description: |
      Extracted from documentation: Dates and times
  - name: "setSystemTime"
    id: concept:page_2384_bun
    description: |
      Extracted from documentation: setSystemTime
  - name: "Reset the system time"
    id: concept:page_2385_bun
    description: |
      Extracted from documentation: Reset the system time
  - name: "Get mocked time with jest.now()"
    id: concept:page_2386_bun
    description: |
      Extracted from documentation: Get mocked time with jest.now()
  - name: "Set the time zone"
    id: concept:page_2387_bun
    description: |
      Extracted from documentation: Set the time zone
  - name: "Finding tests"
    id: concept:page_2388_bun
    description: |
      Extracted from documentation: Finding tests
  - name: "Default Discovery Logic"
    id: concept:page_2389_bun
    description: |
      Extracted from documentation: Default Discovery Logic
  - name: "Exclusions"
    id: concept:page_2390_bun
    description: |
      Extracted from documentation: Exclusions
  - name: "Customizing Test Discovery"
    id: concept:page_2391_bun
    description: |
      Extracted from documentation: Customizing Test Discovery
  - name: "Position Arguments as Filters"
    id: concept:page_2392_bun
    description: |
      Extracted from documentation: Position Arguments as Filters
  - name: "Specifying Exact File Paths"
    id: concept:page_2393_bun
    description: |
      Extracted from documentation: Specifying Exact File Paths
  - name: "Filter by Test Name"
    id: concept:page_2394_bun
    description: |
      Extracted from documentation: Filter by Test Name
  - name: "run all tests with \"addition\" in the name"
    id: concept:page_2395_bun
    description: |
      Extracted from documentation: run all tests with "addition" in the name
  - name: "Changing the Root Directory"
    id: concept:page_2396_bun
    description: |
      Extracted from documentation: Changing the Root Directory
  - name: "Execution Order"
    id: concept:page_2397_bun
    description: |
      Extracted from documentation: Execution Order
  - name: "DOM testing"
    id: concept:page_2398_bun
    description: |
      Extracted from documentation: DOM testing
  - name: "happy-dom"
    id: concept:page_2399_bun
    description: |
      Extracted from documentation: happy-dom
  - name: "React Testing Library"
    id: concept:page_2400_bun
    description: |
      Extracted from documentation: React Testing Library
  - name: "Advanced DOM Testing"
    id: concept:page_2401_bun
    description: |
      Extracted from documentation: Advanced DOM Testing
  - name: "Custom Elements"
    id: concept:page_2402_bun
    description: |
      Extracted from documentation: Custom Elements
  - name: "Event Testing"
    id: concept:page_2403_bun
    description: |
      Extracted from documentation: Event Testing
  - name: "Configuration Tips"
    id: concept:page_2404_bun
    description: |
      Extracted from documentation: Configuration Tips
  - name: "Global Setup"
    id: concept:page_2405_bun
    description: |
      Extracted from documentation: Global Setup
  - name: "Common Issues"
    id: concept:page_2406_bun
    description: |
      Extracted from documentation: Common Issues
  - name: "Performance Considerations"
    id: concept:page_2407_bun
    description: |
      Extracted from documentation: Performance Considerations
  - name: "Run tests"
    id: concept:page_2408_bun
    description: |
      Extracted from documentation: Run tests
  - name: "run all tests or test suites with \"addition\" in the name"
    id: concept:page_2409_bun
    description: |
      Extracted from documentation: run all tests or test suites with "addition" in the name
  - name: "CI/CD integration"
    id: concept:page_2410_bun
    description: |
      Extracted from documentation: CI/CD integration
  - name: "How to install `bun` in a GitHub Actions workflow"
    id: concept:page_2411_bun
    description: |
      Extracted from documentation: How to install `bun` in a GitHub Actions workflow
  - name: "JUnit XML reports (GitLab, etc.)"
    id: concept:page_2412_bun
    description: |
      Extracted from documentation: JUnit XML reports (GitLab, etc.)
  - name: "Timeouts"
    id: concept:page_2413_bun
    description: |
      Extracted from documentation: Timeouts
  - name: "default value is 5000"
    id: concept:page_2414_bun
    description: |
      Extracted from documentation: default value is 5000
  - name: "Concurrent test execution"
    id: concept:page_2415_bun
    description: |
      Extracted from documentation: Concurrent test execution
  - name: "`--concurrent` flag"
    id: concept:page_2416_bun
    description: |
      Extracted from documentation: `--concurrent` flag
  - name: "`--max-concurrency` flag"
    id: concept:page_2417_bun
    description: |
      Extracted from documentation: `--max-concurrency` flag
  - name: "Limit to 4 concurrent tests"
    id: concept:page_2418_bun
    description: |
      Extracted from documentation: Limit to 4 concurrent tests
  - name: "Default: 20"
    id: concept:page_2419_bun
    description: |
      Extracted from documentation: Default: 20
  - name: "`test.concurrent`"
    id: concept:page_2420_bun
    description: |
      Extracted from documentation: `test.concurrent`
  - name: "`test.serial`"
    id: concept:page_2421_bun
    description: |
      Extracted from documentation: `test.serial`
  - name: "Retry failed tests"
    id: concept:page_2422_bun
    description: |
      Extracted from documentation: Retry failed tests
  - name: "Rerun tests"
    id: concept:page_2423_bun
    description: |
      Extracted from documentation: Rerun tests
  - name: "Randomize test execution order"
    id: concept:page_2424_bun
    description: |
      Extracted from documentation: Randomize test execution order
  - name: "... test output ..."
    id: concept:page_2425_bun
    description: |
      Extracted from documentation: ... test output ...
  - name: "Reproducible random order with `--seed`"
    id: concept:page_2426_bun
    description: |
      Extracted from documentation: Reproducible random order with `--seed`
  - name: "Reproduce a previous randomized run"
    id: concept:page_2427_bun
    description: |
      Extracted from documentation: Reproduce a previous randomized run
  - name: "Bail out with `--bail`"
    id: concept:page_2428_bun
    description: |
      Extracted from documentation: Bail out with `--bail`
  - name: "bail after 1 failure"
    id: concept:page_2429_bun
    description: |
      Extracted from documentation: bail after 1 failure
  - name: "bail after 10 failure"
    id: concept:page_2430_bun
    description: |
      Extracted from documentation: bail after 10 failure
  - name: "Mocks"
    id: concept:page_2431_bun
    description: |
      Extracted from documentation: Mocks
  - name: "Snapshot testing"
    id: concept:page_2432_bun
    description: |
      Extracted from documentation: Snapshot testing
  - name: "UI & DOM testing"
    id: concept:page_2433_bun
    description: |
      Extracted from documentation: UI & DOM testing
  - name: "AI Agent Integration"
    id: concept:page_2434_bun
    description: |
      Extracted from documentation: AI Agent Integration
  - name: "Behavior"
    id: concept:page_2435_bun
    description: |
      Extracted from documentation: Behavior
  - name: "Example: Enable quiet output for Claude Code"
    id: concept:page_2436_bun
    description: |
      Extracted from documentation: Example: Enable quiet output for Claude Code
  - name: "Still shows failures and summary, but hides verbose passing test output"
    id: concept:page_2437_bun
    description: |
      Extracted from documentation: Still shows failures and summary, but hides verbose passing test output
  - name: "Execution Control"
    id: concept:page_2438_bun
    description: |
      Extracted from documentation: Execution Control
  - name: "Test Filtering"
    id: concept:page_2439_bun
    description: |
      Extracted from documentation: Test Filtering
  - name: "Reporting"
    id: concept:page_2440_bun
    description: |
      Extracted from documentation: Reporting
  - name: "Coverage"
    id: concept:page_2441_bun
    description: |
      Extracted from documentation: Coverage
  - name: "Snapshots"
    id: concept:page_2442_bun
    description: |
      Extracted from documentation: Snapshots
  - name: "Per-Test Setup and Teardown"
    id: concept:page_2443_bun
    description: |
      Extracted from documentation: Per-Test Setup and Teardown
  - name: "Per-Scope Setup and Teardown"
    id: concept:page_2444_bun
    description: |
      Extracted from documentation: Per-Scope Setup and Teardown
  - name: "Scoped to a Describe Block"
    id: concept:page_2445_bun
    description: |
      Extracted from documentation: Scoped to a Describe Block
  - name: "Scoped to a Test File"
    id: concept:page_2446_bun
    description: |
      Extracted from documentation: Scoped to a Test File
  - name: "`onTestFinished`"
    id: concept:page_2447_bun
    description: |
      Extracted from documentation: `onTestFinished`
  - name: "Global Setup and Teardown"
    id: concept:page_2448_bun
    description: |
      Extracted from documentation: Global Setup and Teardown
  - name: "Practical Examples"
    id: concept:page_2449_bun
    description: |
      Extracted from documentation: Practical Examples
  - name: "Database Setup"
    id: concept:page_2450_bun
    description: |
      Extracted from documentation: Database Setup
  - name: "API Server Setup"
    id: concept:page_2451_bun
    description: |
      Extracted from documentation: API Server Setup
  - name: "Mock Setup"
    id: concept:page_2452_bun
    description: |
      Extracted from documentation: Mock Setup
  - name: "Async Lifecycle Hooks"
    id: concept:page_2453_bun
    description: |
      Extracted from documentation: Async Lifecycle Hooks
  - name: "Nested Hooks"
    id: concept:page_2454_bun
    description: |
      Extracted from documentation: Nested Hooks
  - name: "Keep Hooks Simple"
    id: concept:page_2455_bun
    description: |
      Extracted from documentation: Keep Hooks Simple
  - name: "Use Appropriate Scope"
    id: concept:page_2456_bun
    description: |
      Extracted from documentation: Use Appropriate Scope
  - name: "Clean Up Resources"
    id: concept:page_2457_bun
    description: |
      Extracted from documentation: Clean Up Resources
  - name: "Basic Function Mocks"
    id: concept:page_2458_bun
    description: |
      Extracted from documentation: Basic Function Mocks
  - name: "Jest Compatibility"
    id: concept:page_2459_bun
    description: |
      Extracted from documentation: Jest Compatibility
  - name: "Mock Function Properties"
    id: concept:page_2460_bun
    description: |
      Extracted from documentation: Mock Function Properties
  - name: "Available Properties and Methods"
    id: concept:page_2461_bun
    description: |
      Extracted from documentation: Available Properties and Methods
  - name: "Basic Mock Usage"
    id: concept:page_2462_bun
    description: |
      Extracted from documentation: Basic Mock Usage
  - name: "Dynamic Mock Implementations"
    id: concept:page_2463_bun
    description: |
      Extracted from documentation: Dynamic Mock Implementations
  - name: "Async Mocks"
    id: concept:page_2464_bun
    description: |
      Extracted from documentation: Async Mocks
  - name: "Spies with spyOn()"
    id: concept:page_2465_bun
    description: |
      Extracted from documentation: Spies with spyOn()
  - name: "Advanced Spy Usage"
    id: concept:page_2466_bun
    description: |
      Extracted from documentation: Advanced Spy Usage
  - name: "Module Mocks with mock.module()"
    id: concept:page_2467_bun
    description: |
      Extracted from documentation: Module Mocks with mock.module()
  - name: "Overriding Already Imported Modules"
    id: concept:page_2468_bun
    description: |
      Extracted from documentation: Overriding Already Imported Modules
  - name: "Hoisting & Preloading"
    id: concept:page_2469_bun
    description: |
      Extracted from documentation: Hoisting & Preloading
  - name: "Load these modules before running tests."
    id: concept:page_2470_bun
    description: |
      Extracted from documentation: Load these modules before running tests.
  - name: "Module Mock Best Practices"
    id: concept:page_2471_bun
    description: |
      Extracted from documentation: Module Mock Best Practices
  - name: "When to Use Preload"
    id: concept:page_2472_bun
    description: |
      Extracted from documentation: When to Use Preload
  - name: "Practical Module Mock Examples"
    id: concept:page_2473_bun
    description: |
      Extracted from documentation: Practical Module Mock Examples
  - name: "Mocking External Dependencies"
    id: concept:page_2474_bun
    description: |
      Extracted from documentation: Mocking External Dependencies
  - name: "Global Mock Functions"
    id: concept:page_2475_bun
    description: |
      Extracted from documentation: Global Mock Functions
  - name: "Clear All Mocks"
    id: concept:page_2476_bun
    description: |
      Extracted from documentation: Clear All Mocks
  - name: "Reset All Mocks"
    id: concept:page_2477_bun
    description: |
      Extracted from documentation: Reset All Mocks
  - name: "Restore All Mocks"
    id: concept:page_2478_bun
    description: |
      Extracted from documentation: Restore All Mocks
  - name: "Vitest Compatibility"
    id: concept:page_2479_bun
    description: |
      Extracted from documentation: Vitest Compatibility
  - name: "Implementation Details"
    id: concept:page_2480_bun
    description: |
      Extracted from documentation: Implementation Details
  - name: "Cache Interaction"
    id: concept:page_2481_bun
    description: |
      Extracted from documentation: Cache Interaction
  - name: "Lazy Evaluation"
    id: concept:page_2482_bun
    description: |
      Extracted from documentation: Lazy Evaluation
  - name: "Path Resolution"
    id: concept:page_2483_bun
    description: |
      Extracted from documentation: Path Resolution
  - name: "Import Timing Effects"
    id: concept:page_2484_bun
    description: |
      Extracted from documentation: Import Timing Effects
  - name: "Live Bindings"
    id: concept:page_2485_bun
    description: |
      Extracted from documentation: Live Bindings
  - name: "Advanced Patterns"
    id: concept:page_2486_bun
    description: |
      Extracted from documentation: Advanced Patterns
  - name: "Factory Functions"
    id: concept:page_2487_bun
    description: |
      Extracted from documentation: Factory Functions
  - name: "Conditional Mocking"
    id: concept:page_2488_bun
    description: |
      Extracted from documentation: Conditional Mocking
  - name: "Mock Cleanup Patterns"
    id: concept:page_2489_bun
    description: |
      Extracted from documentation: Mock Cleanup Patterns
  - name: "Keep Mocks Simple"
    id: concept:page_2490_bun
    description: |
      Extracted from documentation: Keep Mocks Simple
  - name: "Use Type-Safe Mocks"
    id: concept:page_2491_bun
    description: |
      Extracted from documentation: Use Type-Safe Mocks
  - name: "Test Mock Behavior"
    id: concept:page_2492_bun
    description: |
      Extracted from documentation: Test Mock Behavior
  - name: "Notes"
    id: concept:page_2493_bun
    description: |
      Extracted from documentation: Notes
  - name: "Auto-mocking"
    id: concept:page_2494_bun
    description: |
      Extracted from documentation: Auto-mocking
  - name: "ESM vs CommonJS"
    id: concept:page_2495_bun
    description: |
      Extracted from documentation: ESM vs CommonJS
  - name: "Test Reporters"
    id: concept:page_2496_bun
    description: |
      Extracted from documentation: Test Reporters
  - name: "Built-in Reporters"
    id: concept:page_2497_bun
    description: |
      Extracted from documentation: Built-in Reporters
  - name: "Default Console Reporter"
    id: concept:page_2498_bun
    description: |
      Extracted from documentation: Default Console Reporter
  - name: "Dots Reporter"
    id: concept:page_2499_bun
    description: |
      Extracted from documentation: Dots Reporter
  - name: "JUnit XML Reporter"
    id: concept:page_2500_bun
    description: |
      Extracted from documentation: JUnit XML Reporter
  - name: "Using the JUnit Reporter"
    id: concept:page_2501_bun
    description: |
      Extracted from documentation: Using the JUnit Reporter
  - name: "Configuring via bunfig.toml"
    id: concept:page_2502_bun
    description: |
      Extracted from documentation: Configuring via bunfig.toml
  - name: "Environment Variables in JUnit Reports"
    id: concept:page_2503_bun
    description: |
      Extracted from documentation: Environment Variables in JUnit Reports
  - name: "GitHub Actions reporter"
    id: concept:page_2504_bun
    description: |
      Extracted from documentation: GitHub Actions reporter
  - name: "Custom Reporters"
    id: concept:page_2505_bun
    description: |
      Extracted from documentation: Custom Reporters
  - name: "Inspector Protocol for Testing"
    id: concept:page_2506_bun
    description: |
      Extracted from documentation: Inspector Protocol for Testing
  - name: "Key Events"
    id: concept:page_2507_bun
    description: |
      Extracted from documentation: Key Events
  - name: "Runtime behavior"
    id: concept:page_2508_bun
    description: |
      Extracted from documentation: Runtime behavior
  - name: "NODE\\_ENV"
    id: concept:page_2509_bun
    description: |
      Extracted from documentation: NODE\_ENV
  - name: "TZ (Timezone)"
    id: concept:page_2510_bun
    description: |
      Extracted from documentation: TZ (Timezone)
  - name: "Test Timeouts"
    id: concept:page_2511_bun
    description: |
      Extracted from documentation: Test Timeouts
  - name: "Global Timeout"
    id: concept:page_2512_bun
    description: |
      Extracted from documentation: Global Timeout
  - name: "Per-Test Timeout"
    id: concept:page_2513_bun
    description: |
      Extracted from documentation: Per-Test Timeout
  - name: "Infinite Timeout"
    id: concept:page_2514_bun
    description: |
      Extracted from documentation: Infinite Timeout
  - name: "Unhandled Errors"
    id: concept:page_2515_bun
    description: |
      Extracted from documentation: Unhandled Errors
  - name: "Promise Rejections"
    id: concept:page_2516_bun
    description: |
      Extracted from documentation: Promise Rejections
  - name: "Custom Error Handling"
    id: concept:page_2517_bun
    description: |
      Extracted from documentation: Custom Error Handling
  - name: "CLI Flags Integration"
    id: concept:page_2518_bun
    description: |
      Extracted from documentation: CLI Flags Integration
  - name: "Reduces memory usage for the test runner VM"
    id: concept:page_2519_bun
    description: |
      Extracted from documentation: Reduces memory usage for the test runner VM
  - name: "Attaches the debugger to the test runner process"
    id: concept:page_2520_bun
    description: |
      Extracted from documentation: Attaches the debugger to the test runner process
  - name: "Module Loading"
    id: concept:page_2521_bun
    description: |
      Extracted from documentation: Module Loading
  - name: "Runs scripts before test files (useful for global setup/mocks)"
    id: concept:page_2522_bun
    description: |
      Extracted from documentation: Runs scripts before test files (useful for global setup/mocks)
  - name: "Sets compile-time constants"
    id: concept:page_2523_bun
    description: |
      Extracted from documentation: Sets compile-time constants
  - name: "Maps file extensions to built-in loaders"
    id: concept:page_2524_bun
    description: |
      Extracted from documentation: Maps file extensions to built-in loaders
  - name: "Uses a different tsconfig"
    id: concept:page_2525_bun
    description: |
      Extracted from documentation: Uses a different tsconfig
  - name: "Sets package.json conditions for module resolution"
    id: concept:page_2526_bun
    description: |
      Extracted from documentation: Sets package.json conditions for module resolution
  - name: "Loads environment variables for tests"
    id: concept:page_2527_bun
    description: |
      Extracted from documentation: Loads environment variables for tests
  - name: "Installation-related Flags"
    id: concept:page_2528_bun
    description: |
      Extracted from documentation: Installation-related Flags
  - name: "Affect any network requests or auto-installs during test execution"
    id: concept:page_2529_bun
    description: |
      Extracted from documentation: Affect any network requests or auto-installs during test execution
  - name: "Watch and Hot Reloading"
    id: concept:page_2530_bun
    description: |
      Extracted from documentation: Watch and Hot Reloading
  - name: "Global Variables"
    id: concept:page_2531_bun
    description: |
      Extracted from documentation: Global Variables
  - name: "Process Integration"
    id: concept:page_2532_bun
    description: |
      Extracted from documentation: Process Integration
  - name: "Exit Codes"
    id: concept:page_2533_bun
    description: |
      Extracted from documentation: Exit Codes
  - name: "Signal Handling"
    id: concept:page_2534_bun
    description: |
      Extracted from documentation: Signal Handling
  - name: "Gracefully stops test execution"
    id: concept:page_2535_bun
    description: |
      Extracted from documentation: Gracefully stops test execution
  - name: "Immediately stops test execution"
    id: concept:page_2536_bun
    description: |
      Extracted from documentation: Immediately stops test execution
  - name: "Environment Detection"
    id: concept:page_2537_bun
    description: |
      Extracted from documentation: Environment Detection
  - name: "Single Process"
    id: concept:page_2538_bun
    description: |
      Extracted from documentation: Single Process
  - name: "Memory Management"
    id: concept:page_2539_bun
    description: |
      Extracted from documentation: Memory Management
  - name: "Monitor memory usage"
    id: concept:page_2540_bun
    description: |
      Extracted from documentation: Monitor memory usage
  - name: "For large test suites, consider splitting files"
    id: concept:page_2541_bun
    description: |
      Extracted from documentation: For large test suites, consider splitting files
  - name: "Test Isolation"
    id: concept:page_2542_bun
    description: |
      Extracted from documentation: Test Isolation
  - name: "Basic Snapshots"
    id: concept:page_2543_bun
    description: |
      Extracted from documentation: Basic Snapshots
  - name: "Snapshot Files"
    id: concept:page_2544_bun
    description: |
      Extracted from documentation: Snapshot Files
  - name: "Updating Snapshots"
    id: concept:page_2545_bun
    description: |
      Extracted from documentation: Updating Snapshots
  - name: "Inline Snapshots"
    id: concept:page_2546_bun
    description: |
      Extracted from documentation: Inline Snapshots
  - name: "Using Inline Snapshots"
    id: concept:page_2547_bun
    description: |
      Extracted from documentation: Using Inline Snapshots
  - name: "Error Snapshots"
    id: concept:page_2548_bun
    description: |
      Extracted from documentation: Error Snapshots
  - name: "Advanced Snapshot Usage"
    id: concept:page_2549_bun
    description: |
      Extracted from documentation: Advanced Snapshot Usage
  - name: "Complex Objects"
    id: concept:page_2550_bun
    description: |
      Extracted from documentation: Complex Objects
  - name: "Array Snapshots"
    id: concept:page_2551_bun
    description: |
      Extracted from documentation: Array Snapshots
  - name: "Function Output Snapshots"
    id: concept:page_2552_bun
    description: |
      Extracted from documentation: Function Output Snapshots
  - name: "React Component Snapshots"
    id: concept:page_2553_bun
    description: |
      Extracted from documentation: React Component Snapshots
  - name: "Property Matchers"
    id: concept:page_2554_bun
    description: |
      Extracted from documentation: Property Matchers
  - name: "Keep Snapshots Small"
    id: concept:page_2555_bun
    description: |
      Extracted from documentation: Keep Snapshots Small
  - name: "Use Descriptive Test Names"
    id: concept:page_2556_bun
    description: |
      Extracted from documentation: Use Descriptive Test Names
  - name: "Group Related Snapshots"
    id: concept:page_2557_bun
    description: |
      Extracted from documentation: Group Related Snapshots
  - name: "Handle Dynamic Data"
    id: concept:page_2558_bun
    description: |
      Extracted from documentation: Handle Dynamic Data
  - name: "Managing Snapshots"
    id: concept:page_2559_bun
    description: |
      Extracted from documentation: Managing Snapshots
  - name: "Reviewing Snapshot Changes"
    id: concept:page_2560_bun
    description: |
      Extracted from documentation: Reviewing Snapshot Changes
  - name: "See what changed"
    id: concept:page_2561_bun
    description: |
      Extracted from documentation: See what changed
  - name: "Update if changes are intentional"
    id: concept:page_2562_bun
    description: |
      Extracted from documentation: Update if changes are intentional
  - name: "Commit the updated snapshots"
    id: concept:page_2563_bun
    description: |
      Extracted from documentation: Commit the updated snapshots
  - name: "Organizing Large Snapshot Files"
    id: concept:page_2564_bun
    description: |
      Extracted from documentation: Organizing Large Snapshot Files
  - name: "Snapshot Failures"
    id: concept:page_2565_bun
    description: |
      Extracted from documentation: Snapshot Failures
  - name: "Platform Differences"
    id: concept:page_2566_bun
    description: |
      Extracted from documentation: Platform Differences
  - name: "Writing tests"
    id: concept:page_2567_bun
    description: |
      Extracted from documentation: Writing tests
  - name: "Grouping Tests"
    id: concept:page_2568_bun
    description: |
      Extracted from documentation: Grouping Tests
  - name: "Async Tests"
    id: concept:page_2569_bun
    description: |
      Extracted from documentation: Async Tests
  - name: "Retries and Repeats"
    id: concept:page_2570_bun
    description: |
      Extracted from documentation: Retries and Repeats
  - name: "test.retry"
    id: concept:page_2571_bun
    description: |
      Extracted from documentation: test.retry
  - name: "test.repeats"
    id: concept:page_2572_bun
    description: |
      Extracted from documentation: test.repeats
  - name: "🧟 Zombie Process Killer"
    id: concept:page_2573_bun
    description: |
      Extracted from documentation: 🧟 Zombie Process Killer
  - name: "Test Modifiers"
    id: concept:page_2574_bun
    description: |
      Extracted from documentation: Test Modifiers
  - name: "test.skip"
    id: concept:page_2575_bun
    description: |
      Extracted from documentation: test.skip
  - name: "test.todo"
    id: concept:page_2576_bun
    description: |
      Extracted from documentation: test.todo
  - name: "test.only"
    id: concept:page_2577_bun
    description: |
      Extracted from documentation: test.only
  - name: "test.if"
    id: concept:page_2578_bun
    description: |
      Extracted from documentation: test.if
  - name: "test.skipIf"
    id: concept:page_2579_bun
    description: |
      Extracted from documentation: test.skipIf
  - name: "test.todoIf"
    id: concept:page_2580_bun
    description: |
      Extracted from documentation: test.todoIf
  - name: "test.failing"
    id: concept:page_2581_bun
    description: |
      Extracted from documentation: test.failing
  - name: "Conditional Tests for Describe Blocks"
    id: concept:page_2582_bun
    description: |
      Extracted from documentation: Conditional Tests for Describe Blocks
  - name: "Parametrized Tests"
    id: concept:page_2583_bun
    description: |
      Extracted from documentation: Parametrized Tests
  - name: "`test.each` and `describe.each`"
    id: concept:page_2584_bun
    description: |
      Extracted from documentation: `test.each` and `describe.each`
  - name: "Argument Passing"
    id: concept:page_2585_bun
    description: |
      Extracted from documentation: Argument Passing
  - name: "Format Specifiers"
    id: concept:page_2586_bun
    description: |
      Extracted from documentation: Format Specifiers
  - name: "Assertion Counting"
    id: concept:page_2587_bun
    description: |
      Extracted from documentation: Assertion Counting
  - name: "expect.hasAssertions()"
    id: concept:page_2588_bun
    description: |
      Extracted from documentation: expect.hasAssertions()
  - name: "expect.assertions(count)"
    id: concept:page_2589_bun
    description: |
      Extracted from documentation: expect.assertions(count)
  - name: "Type Testing"
    id: concept:page_2590_bun
    description: |
      Extracted from documentation: Type Testing
  - name: "expectTypeOf"
    id: concept:page_2591_bun
    description: |
      Extracted from documentation: expectTypeOf
  - name: "Matchers"
    id: concept:page_2592_bun
    description: |
      Extracted from documentation: Matchers
  - name: "Basic Matchers"
    id: concept:page_2593_bun
    description: |
      Extracted from documentation: Basic Matchers
  - name: "String and Array Matchers"
    id: concept:page_2594_bun
    description: |
      Extracted from documentation: String and Array Matchers
  - name: "Object Matchers"
    id: concept:page_2595_bun
    description: |
      Extracted from documentation: Object Matchers
  - name: "Number Matchers"
    id: concept:page_2596_bun
    description: |
      Extracted from documentation: Number Matchers
  - name: "Function and Class Matchers"
    id: concept:page_2597_bun
    description: |
      Extracted from documentation: Function and Class Matchers
  - name: "Promise Matchers"
    id: concept:page_2598_bun
    description: |
      Extracted from documentation: Promise Matchers
  - name: "Mock Function Matchers"
    id: concept:page_2599_bun
    description: |
      Extracted from documentation: Mock Function Matchers
  - name: "Snapshot Matchers"
    id: concept:page_2600_bun
    description: |
      Extracted from documentation: Snapshot Matchers
  - name: "Utility Matchers"
    id: concept:page_2601_bun
    description: |
      Extracted from documentation: Utility Matchers
  - name: "Not Yet Implemented"
    id: concept:page_2602_bun
    description: |
      Extracted from documentation: Not Yet Implemented
  - name: "Group Related Tests"
    id: concept:page_2603_bun
    description: |
      Extracted from documentation: Group Related Tests
  - name: "Use Appropriate Matchers"
    id: concept:page_2604_bun
    description: |
      Extracted from documentation: Use Appropriate Matchers
  - name: "Test Error Conditions"
    id: concept:page_2605_bun
    description: |
      Extracted from documentation: Test Error Conditions
  - name: "Use Setup and Teardown"
    id: concept:page_2606_bun
    description: |
      Extracted from documentation: Use Setup and Teardown
  - name: "Suggested `compilerOptions`"
    id: concept:page_2607_bun
    description: |
      Extracted from documentation: Suggested `compilerOptions`
  - name: "TypeScript 6 and 7"
    id: concept:page_2608_bun
    description: |
      Extracted from documentation: TypeScript 6 and 7
  - name: "What changed"
    id: concept:page_2609_bun
    description: |
      Extracted from documentation: What changed
  - name: "Add `\"types\": [\"bun\"]` to your tsconfig"
    id: concept:page_2610_bun
    description: |
      Extracted from documentation: Add `"types": ["bun"]` to your tsconfig
  - name: "Full recommended tsconfig.json"
    id: concept:page_2611_bun
    description: |
      Extracted from documentation: Full recommended tsconfig.json
  - name: "Does this apply to TypeScript 7?"
    id: concept:page_2612_bun
    description: |
      Extracted from documentation: Does this apply to TypeScript 7?
  - name: "On this page"
    id: concept:page_2613_bun
    description: |
      Extracted from documentation: On this page
  - name: "​Basic Usage"
    id: concept:page_2614_bun
    description: |
      Extracted from documentation: ​Basic Usage
  - name: "​Logging"
    id: concept:page_2615_bun
    description: |
      Extracted from documentation: ​Logging
  - name: "​Lifecycle scripts"
    id: concept:page_2616_bun
    description: |
      Extracted from documentation: ​Lifecycle scripts
  - name: "​Workspaces"
    id: concept:page_2617_bun
    description: |
      Extracted from documentation: ​Workspaces
  - name: "​Installing dependencies for specific packages"
    id: concept:page_2618_bun
    description: |
      Extracted from documentation: ​Installing dependencies for specific packages
  - name: "​Overrides and resolutions"
    id: concept:page_2619_bun
    description: |
      Extracted from documentation: ​Overrides and resolutions
  - name: "​Global packages"
    id: concept:page_2620_bun
    description: |
      Extracted from documentation: ​Global packages
  - name: "​Production mode"
    id: concept:page_2621_bun
    description: |
      Extracted from documentation: ​Production mode
  - name: "​Omitting dependencies"
    id: concept:page_2622_bun
    description: |
      Extracted from documentation: ​Omitting dependencies
  - name: "​Dry run"
    id: concept:page_2623_bun
    description: |
      Extracted from documentation: ​Dry run
  - name: "​Non-npm dependencies"
    id: concept:page_2624_bun
    description: |
      Extracted from documentation: ​Non-npm dependencies
  - name: "​Installation strategies"
    id: concept:page_2625_bun
    description: |
      Extracted from documentation: ​Installation strategies
  - name: "​Minimum release age"
    id: concept:page_2626_bun
    description: |
      Extracted from documentation: ​Minimum release age
  - name: "​Configuration"
    id: concept:page_2627_bun
    description: |
      Extracted from documentation: ​Configuration
  - name: "​CI/CD"
    id: concept:page_2628_bun
    description: |
      Extracted from documentation: ​CI/CD
  - name: "​Platform-specific dependencies?"
    id: concept:page_2629_bun
    description: |
      Extracted from documentation: ​Platform-specific dependencies?
  - name: "​Peer dependencies?"
    id: concept:page_2630_bun
    description: |
      Extracted from documentation: ​Peer dependencies?
  - name: "​Lockfile"
    id: concept:page_2631_bun
    description: |
      Extracted from documentation: ​Lockfile
  - name: "​Cache"
    id: concept:page_2632_bun
    description: |
      Extracted from documentation: ​Cache
  - name: "​Platform-specific backends"
    id: concept:page_2633_bun
    description: |
      Extracted from documentation: ​Platform-specific backends
  - name: "​npm registry metadata"
    id: concept:page_2634_bun
    description: |
      Extracted from documentation: ​npm registry metadata
  - name: "​pnpm migration"
    id: concept:page_2635_bun
    description: |
      Extracted from documentation: ​pnpm migration
  - name: "​CLI Usage"
    id: concept:page_2636_bun
    description: |
      Extracted from documentation: ​CLI Usage
  - name: "​Hoisted installs"
    id: concept:page_2637_bun
    description: |
      Extracted from documentation: ​Hoisted installs
  - name: "​Isolated installs"
    id: concept:page_2638_bun
    description: |
      Extracted from documentation: ​Isolated installs
  - name: "​Default strategy"
    id: concept:page_2639_bun
    description: |
      Extracted from documentation: ​Default strategy
  - name: "​Configuring bun install with bunfig.toml"
    id: concept:page_2640_bun
    description: |
      Extracted from documentation: ​Configuring bun install with bunfig.toml
  - name: "​Configuring with environment variables"
    id: concept:page_2641_bun
    description: |
      Extracted from documentation: ​Configuring with environment variables
  - name: "​--cpu and --os flags"
    id: concept:page_2642_bun
    description: |
      Extracted from documentation: ​--cpu and --os flags
  - name: "​Lockfile Migration"
    id: concept:page_2643_bun
    description: |
      Extracted from documentation: ​Lockfile Migration
  - name: "​Workspace Configuration"
    id: concept:page_2644_bun
    description: |
      Extracted from documentation: ​Workspace Configuration
  - name: "​Catalog Dependencies"
    id: concept:page_2645_bun
    description: |
      Extracted from documentation: ​Catalog Dependencies
  - name: "​Configuration Migration"
    id: concept:page_2646_bun
    description: |
      Extracted from documentation: ​Configuration Migration
  - name: "​Requirements"
    id: concept:page_2647_bun
    description: |
      Extracted from documentation: ​Requirements
  - name: "​General Configuration"
    id: concept:page_2648_bun
    description: |
      Extracted from documentation: ​General Configuration
  - name: "​Dependency Scope & Management"
    id: concept:page_2649_bun
    description: |
      Extracted from documentation: ​Dependency Scope & Management
  - name: "​Dependency Type & Versioning"
    id: concept:page_2650_bun
    description: |
      Extracted from documentation: ​Dependency Type & Versioning
  - name: "​Lockfile Control"
    id: concept:page_2651_bun
    description: |
      Extracted from documentation: ​Lockfile Control
  - name: "​Network & Registry Settings"
    id: concept:page_2652_bun
    description: |
      Extracted from documentation: ​Network & Registry Settings
  - name: "​Installation Process Control"
    id: concept:page_2653_bun
    description: |
      Extracted from documentation: ​Installation Process Control
  - name: "​Caching Options"
    id: concept:page_2654_bun
    description: |
      Extracted from documentation: ​Caching Options
  - name: "​Output & Logging"
    id: concept:page_2655_bun
    description: |
      Extracted from documentation: ​Output & Logging
  - name: "​Security & Integrity"
    id: concept:page_2656_bun
    description: |
      Extracted from documentation: ​Security & Integrity
  - name: "​Concurrency & Performance"
    id: concept:page_2657_bun
    description: |
      Extracted from documentation: ​Concurrency & Performance
  - name: "​Lifecycle Script Management"
    id: concept:page_2658_bun
    description: |
      Extracted from documentation: ​Lifecycle Script Management
  - name: "​Help Information"
    id: concept:page_2659_bun
    description: |
      Extracted from documentation: ​Help Information
apis:
  - name: "bun run index.tsx  # TS and JSX supported by default"
    id: api:crawl_0_bun
    signature: |
      bun run index.tsx  # TS and JSX supported by default
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun run start                 # run the `start` script"
    id: api:crawl_1_bun
    signature: |
      bun run start                 # run the `start` script
      bun install <pkg>             # install a package
      bun build ./index.tsx         # bundle a project for browsers
      bun test                      # run tests
      bunx cowsay 'Hello, world!'   # execute a package
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install react"
    id: api:crawl_2_bun
    signature: |
      bun install react
      bun install react@19.1.1 # specific version
      bun install react@latest # specific tag
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --verbose # debug logging"
    id: api:crawl_3_bun
    signature: |
      bun install --verbose # debug logging
      bun install --silent  # no logging
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_4_bun
    signature: |
      {
        "name": "my-app",
        "version": "1.0.0",
        "trustedDependencies": ["my-trusted-package"] 
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --concurrent-scripts 5"
    id: api:crawl_5_bun
    signature: |
      bun install --concurrent-scripts 5
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "BUN_FEATURE_FLAG_DISABLE_NATIVE_DEPENDENCY_LINKER=1 bun inst"
    id: api:crawl_6_bun
    signature: |
      BUN_FEATURE_FLAG_DISABLE_NATIVE_DEPENDENCY_LINKER=1 bun install
      BUN_FEATURE_FLAG_DISABLE_IGNORE_SCRIPTS=1 bun install
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_7_bun
    signature: |
      {
        "name": "my-app",
        "version": "1.0.0",
        "workspaces": ["packages/*"], 
        "dependencies": {
          "preact": "^10.5.13"
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "# Install dependencies for all workspaces except `pkg-c`"
    id: api:crawl_8_bun
    signature: |
      # Install dependencies for all workspaces except `pkg-c`
      bun install --filter '!pkg-c'
      
      # Install dependencies for only `pkg-a` in `./packages/pkg-a`
      bun install --filter './packages/pkg-a'
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_9_bun
    signature: |
      {
        "name": "my-app",
        "dependencies": {
          "foo": "^2.0.0"
        },
        "overrides": { 
          "bar": "~4.4.0"
        } 
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --global cowsay # or `bun install -g cowsay`"
    id: api:crawl_10_bun
    signature: |
      bun install --global cowsay # or `bun install -g cowsay`
      cowsay "Bun!"
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "______"
    id: api:crawl_11_bun
    signature: |
      ______
      < Bun! >
       ------
              \   ^__^
               \  (oo)\_______
                  (__)\       )\/\
                      ||----w |
                      ||     ||
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --production"
    id: api:crawl_12_bun
    signature: |
      bun install --production
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --frozen-lockfile"
    id: api:crawl_13_bun
    signature: |
      bun install --frozen-lockfile
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "# Exclude \"devDependencies\" from the installation. This will"
    id: api:crawl_14_bun
    signature: |
      # Exclude "devDependencies" from the installation. This will apply to the
      # root package and workspaces if they exist. Transitive dependencies will
      # not have "devDependencies".
      bun install --omit dev
      
      # Install only dependencies from "dependencies"
      bun install --omit=dev --omit=peer --omit=optional
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --dry-run"
    id: api:crawl_15_bun
    signature: |
      bun install --dry-run
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_16_bun
    signature: |
      {
        "dependencies": {
          "dayjs": "git+https://github.com/iamkun/dayjs.git",
          "lodash": "git+ssh://github.com/lodash/lodash.git#4.17.21",
          "moment": "git@github.com:moment/moment.git",
          "zod": "github:colinhacks/zod",
          "react": "https://registry.npmjs.org/react/-/react-18.2.0.tgz",
          "bun-types": "npm:@types/bun"
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --linker hoisted"
    id: api:crawl_17_bun
    signature: |
      bun install --linker hoisted
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --linker isolated"
    id: api:crawl_18_bun
    signature: |
      bun install --linker isolated
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "# Only install package versions published at least 3 days ag"
    id: api:crawl_19_bun
    signature: |
      # Only install package versions published at least 3 days ago
      bun add @types/bun --minimum-release-age 259200 # seconds
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "[install]"
    id: api:crawl_20_bun
    signature: |
      [install]
      # Only install package versions published at least 3 days ago
      minimumReleaseAge = 259200 # seconds
      
      # Exclude trusted packages from the age gate
      minimumReleaseAgeExcludes = ["@types/node", "typescript"]
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$XDG_CONFIG_HOME/.bunfig.toml"
    id: api:crawl_21_bun
    signature: |
      $XDG_CONFIG_HOME/.bunfig.toml
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "BUN_CONFIG_YARN_LOCKFILE"
    id: api:crawl_23_bun
    signature: |
      BUN_CONFIG_YARN_LOCKFILE
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "BUN_CONFIG_SKIP_SAVE_LOCKFILE"
    id: api:crawl_24_bun
    signature: |
      BUN_CONFIG_SKIP_SAVE_LOCKFILE
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "BUN_CONFIG_SKIP_LOAD_LOCKFILE"
    id: api:crawl_25_bun
    signature: |
      BUN_CONFIG_SKIP_LOAD_LOCKFILE
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "BUN_CONFIG_SKIP_INSTALL_PACKAGES"
    id: api:crawl_26_bun
    signature: |
      BUN_CONFIG_SKIP_INSTALL_PACKAGES
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "~/.bun/install/cache/${name}@${version}"
    id: api:crawl_27_bun
    signature: |
      ~/.bun/install/cache/${name}@${version}
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "name: bun-types"
    id: api:crawl_28_bun
    signature: |
      name: bun-types
      jobs:
        build:
          name: build-app
          runs-on: ubuntu-latest
          steps:
            - name: Checkout repo
              uses: actions/checkout@v4
            - name: Install bun
              uses: oven-sh/setup-bun@v2
            - name: Install dependencies
              run: bun install
            - name: Build app
              run: bun run build
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "name: bun-types"
    id: api:crawl_29_bun
    signature: |
      name: bun-types
      jobs:
        build:
          name: build-app
          runs-on: ubuntu-latest
          steps:
            - name: Checkout repo
              uses: actions/checkout@v4
            - name: Install bun
              uses: oven-sh/setup-bun@v2
            - name: Install dependencies
              run: bun ci
            - name: Build app
              run: bun run build
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --cpu=x64 --os=linux"
    id: api:crawl_30_bun
    signature: |
      bun install --cpu=x64 --os=linux
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install --save-text-lockfile --frozen-lockfile --lockfil"
    id: api:crawl_31_bun
    signature: |
      bun install --save-text-lockfile --frozen-lockfile --lockfile-only
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun pm cache rm"
    id: api:crawl_32_bun
    signature: |
      bun pm cache rm
      # or
      rm -rf ~/.bun/install/cache
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "rm -rf node_modules"
    id: api:crawl_33_bun
    signature: |
      rm -rf node_modules
      bun install --backend hardlink
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "rm -rf node_modules"
    id: api:crawl_34_bun
    signature: |
      rm -rf node_modules
      bun install --backend clonefile
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "rm -rf node_modules"
    id: api:crawl_35_bun
    signature: |
      rm -rf node_modules
      bun install --backend clonefile_each_dir
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "rm -rf node_modules"
    id: api:crawl_36_bun
    signature: |
      rm -rf node_modules
      bun install --backend copyfile
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "rm -rf node_modules"
    id: api:crawl_37_bun
    signature: |
      rm -rf node_modules
      bun install --backend symlink
      bun --preserve-symlinks ./my-file.js
      node --preserve-symlinks ./my-file.js # https://nodejs.org/api/cli.html#--preserve-symlinks
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "~/.bun/install/cache/*.npm"
    id: api:crawl_38_bun
    signature: |
      ~/.bun/install/cache/*.npm
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "${hash(packageName)}.npm"
    id: api:crawl_39_bun
    signature: |
      ${hash(packageName)}.npm
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "packages:"
    id: api:crawl_40_bun
    signature: |
      packages:
        - "apps/*"
        - "packages/*"
      
      catalog:
        react: ^18.0.0
        typescript: ^5.0.0
      
      catalogs:
        build:
          webpack: ^5.0.0
          babel: ^7.0.0
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_41_bun
    signature: |
      {
        "workspaces": {
          "packages": ["apps/*", "packages/*"],
          "catalog": {
            "react": "^18.0.0",
            "typescript": "^5.0.0"
          },
          "catalogs": {
            "build": {
              "webpack": "^5.0.0",
              "babel": "^7.0.0"
            }
          }
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_42_bun
    signature: |
      {
        "dependencies": {
          "react": "catalog:",
          "webpack": "catalog:build"
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "pnpm.patchedDependencies"
    id: api:crawl_43_bun
    signature: |
      pnpm.patchedDependencies
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "bun install <name>@<version>"
    id: api:crawl_44_bun
    signature: |
      bun install <name>@<version>
    returns: See documentation
    description: |
      Extracted code example from documentation.
failures:
  - id: failure:crawl_0_bun
    symptom: |
      Documentation IndexFetch the complete documentation index at: /docs/llms.txtUse
    cause: |
      sr-only issue
    fix: |
      Refer to official documentation for resolution.
---


_Content truncated at 400KB._

