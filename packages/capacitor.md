---
kind: Package
id: package:capacitor
name: Capacitor
version: "6.2"
purpose: Document Capacitor — a cross-platform native runtime for building mobile (iOS, Android) and desktop (Electron) applications with web technologies, providing a Native API plugin system, live reload, and app store deployment.
problem_solved: Enables web developers to build native mobile and desktop apps using their existing HTML/CSS/JS codebase, providing access to native device APIs (camera, geolocation, filesystem, push notifications, biometrics) through a JavaScript bridge without writing platform-specific code.
install: npm install @capacitor/core @capacitor/cli
dependencies:
  - concept:web-development
  - concept:mobile-development
  - concept:native-apis
concepts:
  - name: Core Runtime
    id: concept:capacitor/runtime
    description: The Capacitor runtime bridges web code with native APIs via a WebView. Each platform (iOS, Android, Web) has its own runtime implementation that provides the same JavaScript API surface. The runtime handles plugin registration, permissions, and lifecycle events.
  - name: Plugins
    id: concept:capacitor/plugins
    description: Native API modules callable from JavaScript — Camera, Geolocation, Filesystem, Share, Push Notifications, Local Notifications, Storage (Preferences), Clipboard, Network, Haptics, StatusBar, SplashScreen, Browser, Toast, Dialog, Motion, Device, App, ScreenOrientation, Keyboard.
  - name: Capabilities
    id: concept:capacitor/capabilities
    description: Per-platform configuration for native features — iOS capabilities (Push Notifications, Sign In with Apple, iCloud), Android permissions (Camera, Location, Storage), and entitlements. Configured in capacitor.config.ts under ios/capabilities or AndroidManifest.xml overlay.
  - name: Hooks / Lifecycle
    id: concept:capacitor/hooks
    description: App lifecycle events — appStateChange (active/inactive), appUrlOpen (deep links), appRestoredResult (after app restart), backButton (Android back press), pause, resume. Listen via Capacitor.Plugins.App.addListener('appStateChange', handler).
  - name: Deep Links
    id: concept:capacitor/deeplinks
    description: Handle universal links (iOS) and app links (Android) to navigate users into specific app screens from URLs, push notifications, or QR codes. Configured in capacitor.config.ts and the platform-specific project files.
  - name: Live Reload
    id: concept:capacitor/livereload
    description: npx cap run ios --livereload — connects the native app to your dev server for instant code updates without rebuilding the native project. Changes to web code reflect immediately. Requires dev server running (npm run dev).
  - name: Splash Screen
    id: concept:capacitor/splash
    description: Native splash screen shown while the app loads. Configure backgroundColor, showDuration, launchAutoHide, and custom drawable in capacitor.config.ts. Use SplashScreen.hide() from the web code after initialization is complete.
  - name: App Store Deployment
    id: concept:capacitor/deployment
    description: Build process for stores — npx cap build ios (produces .xcarchive for TestFlight/App Store), npx cap build android (produces .aab for Play Store). Code signing and provisioning profiles handled via Xcode/Android Studio. Automated via Fastlane or GitHub Actions.
apis:
  - name: npx cap init [appName] [appId]
    id: api:capacitor/init
    signature: "npx cap init MyApp com.example.myapp"
    returns: Initialized capacitor.config.ts.
    description: Initializes Capacitor in a web project. Creates capacitor.config.ts with appId (reverse-domain), appName, and webDir (build output directory). Must be run once at project setup.
  - name: npx cap add [platform]
    id: api:capacitor/add
    signature: "npx cap add ios"
    returns: Native platform project directory.
    description: Adds a native platform to the project. Creates ios/ or android/ directory with full Xcode/Android Studio project files. Run after initialization and after building the web app.
  - name: npx cap copy
    id: api:capacitor/copy
    signature: "npx cap copy"
    returns: Updated native project web assets.
    description: Copies the web build output (from webDir) to the native project. Run after every web build to update the native app. Also copies capacitor.config.ts changes to native project config.
  - name: npx cap open [platform]
    id: api:capacitor/open
    signature: "npx cap open ios"
    returns: Opens Xcode or Android Studio.
    description: Opens the native platform project in its IDE. Run cap copy first to sync web assets. Use Xcode for iOS builds, testing, and App Store submission. Android Studio for Android builds.
  - name: Camera.getPhoto(opts)
    id: api:capacitor/camera
    signature: "import { Camera, CameraResultType } from '@capacitor/camera'; const photo = await Camera.getPhoto({ resultType: CameraResultType.Uri, source: CameraSource.Prompt, quality: 90 })"
    returns: "{ path, webPath, base64String, format, exif }"
    description: Opens the device camera or photo library. Configure source (Camera, Photos, Prompt), quality (0-100), allowEditing, saveToGallery. Returns image URI, base64, or file path. Requires Camera permission plugin.
  - name: Geolocation.getCurrentPosition(opts)
    id: api:capacitor/geolocation
    signature: "import { Geolocation } from '@capacitor/geolocation'; const pos = await Geolocation.getCurrentPosition({ enableHighAccuracy: true, timeout: 10000 })"
    returns: "{ coords: { latitude, longitude, accuracy, altitude, speed }, timestamp }"
    description: Gets the device's current position. enableHighAccuracy uses GPS (slower, more accurate). watchPosition for continuous tracking. Requires location permissions on both platforms.
failures:
  - id: failure:capacitor/white-screen
    symptom: App shows white screen on device but works in browser.
    cause: Web build output path mismatch — capacitor.config.ts webDir does not match the actual build output directory (dist/, build/, www/).
    fix: Verify webDir in capacitor.config.ts matches the actual build output folder. Run npm run build && npx cap copy. Check the console in Safari/Chrome DevTools connected to the device for errors.
  - id: failure:capacitor/plugin-not-found
    symptom: JavaScript import of a Capacitor plugin returns undefined or throws.
    cause: Plugin native code not synced to the native project (need cap sync) or plugin not installed via npm.
    fix: Install plugin npm package. Run npx cap sync to copy native code. Verify the plugin is in package.json and node_modules. For iOS, run pod install in ios/App. For Android, check gradle sync.
  - id: failure:capacitor/permission-denied
    symptom: Plugin API call rejects with permission error even though the user granted permission.
    cause: Permissions declared in code but not in the platform project config (Info.plist for iOS, AndroidManifest.xml for Android).
    fix: Add required permission descriptions to Info.plist (iOS) and AndroidManifest.xml (Android). Use @capacitor/app-launcher to request permissions before calling the plugin. Check the Capacitor plugin docs for required permission strings.
  - id: failure:capacitor/http-requests-blocked
    symptom: Fetch requests fail on Android but work in browser.
    cause: Android blocks cleartext HTTP traffic by default (requires HTTPS) and Network Security Config may block custom CAs.
    fix: Use HTTPS URLs for all API requests. For local development, add android:usesCleartextTraffic="true" to AndroidManifest.xml. Configure Network Security Config for custom certificates. For iOS, add App Transport Security exception for HTTP dev servers.
extends: []
implements: []
uses:
  - concept:web-development
  - concept:mobile-development
part_of: concept:cross-platform-development
solves:
  - problem:native-mobile-apps-from-web-code
  - problem:device-api-access-from-web
  - problem:app-store-deployment-for-web-apps
alternatives:
  - package:react-native
  - package:flutter
  - package:expo
  - package:ionic
---
Capacitor treats the web app as the primary codebase and native platforms as deployment targets. Unlike React Native (which replaces the web rendering stack) or Flutter (which uses its own engine), Capacitor runs your existing web application in a full-screen WebView with native API bridges. This means your React, Vue, Svelte, or vanilla JS app works in Capacitor with minimal changes — add the runtime, configure plugins, and build for each platform.

The plugin architecture is a bridge pattern. Each plugin has TypeScript definitions on the JavaScript side and native implementations for iOS (Swift), Android (Java/Kotlin), and Web (fallback). Plugins can be installed from npm (public plugins) or developed in-house for custom native functionality. The plugin API handles serialization, permission requests, and error propagation — the JavaScript side just awaits promise-based calls.

The development workflow is web-first: develop and test in the browser with live reload, then deploy to devices when ready. Capacitor's live reload connects the native WebView to your dev server, reflecting code changes immediately on the device without rebuilding the native project. For production, the web build is bundled into the native app binary, making the app fully self-contained for App Store and Play Store distribution.
