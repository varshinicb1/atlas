---
kind: Package
id: package:three/js
name: three.js 3D Graphics
version: r170
purpose: r3f/three.js patterns for 3D web graphics — scene setup, cameras, lights, materials, geometries, animations, and WebGL rendering
problem_solved: Standardises three.js usage patterns — scene graph structure, render loop lifecycle, material selection, camera rigging, and performance optimisation — reducing WebGL boilerplate, shadow artifacts, memory leaks from uncleaned resources, and animation jank.
install: npm install three @react-three/fiber @react-three/drei
dependencies:
  - package:react
  - package:react-dom
concepts:
  - name: Scene Graph
    id: concept:three/scene-graph
    description: A tree of Object3D nodes (scene, group, mesh, light, camera) with inherited world transforms via matrix multiplication.
  - name: Rendering Pipeline
    id: concept:three/renderer
    description: WebGL or WebGPU context managing draw calls, shader compilation, buffer swaps, and GPU resource lifecycles.
  - name: Camera & Projection
    id: concept:three/camera
    description: View frustum defined by FOV, aspect, near/far planes (perspective) or 6 clipping planes (orthographic); determines what is visible.
  - name: Geometry
    id: concept:three/geometry
    description: Vertex buffers (position, normal, UV, index) defining mesh shape via triangles or lines.
  - name: Materials
    id: concept:three/materials
    description: Surface shading functions — MeshStandardMaterial (PBR), MeshPhysicalMaterial (clearcoat, ior), ShaderMaterial (custom GLSL).
  - name: Lighting
    id: concept:three/lighting
    description: Physically-based illumination via DirectionalLight, PointLight, SpotLight, AmbientLight, and LightProbe.
  - name: Animation
    id: concept:three/animation
    description: Keyframe tracks, skeletal animation, morph targets, and procedural updates via requestAnimationFrame.
  - name: Controls
    id: concept:three/controls
    description: User-interaction handlers — OrbitControls, FlyControls, PointerLockControls, FirstPersonControls for camera navigation.
  - name: Shadows
    id: concept:three/shadows
    description: Shadow-map rendering with PCF soft shadows, variance shadow maps, and cascade shadow maps for large scenes.
  - name: Post-Processing
    id: concept:three/postprocessing
    description: Render-to-texture pipelines with EffectComposer for bloom, depth-of-field, SSAO, tone-mapping, and custom passes.
  - name: Performance
    id: concept:three/performance
    description: Draw-call batching, geometry instancing, LODs, frustum culling, and GPU memory management for 60fps delivery.
  - name: Loading & Assets
    id: concept:three/loading
    description: Async asset loading via TextureLoader, GLTFLoader, FBXLoader, DRACOLoader with progress tracking and caching.
  - name: React Three Fiber
    id: concept:three/r3f
    description: Declarative React bindings for three.js — JSX scene composition, hooks (useFrame, useThree), and automatic disposal on unmount.
apis:
  - name: Scene
    id: api:three/scene
    signature: "const scene = new THREE.Scene()"
    returns: A Scene object (extends Object3D).
    description: Root container for the scene graph — holds children, fog, background color/texture, and environment map.
  - name: PerspectiveCamera
    id: api:three/perspectivecamera
    signature: "new THREE.PerspectiveCamera(fov, aspect, near, far)"
    returns: A perspective projection camera.
    description: Standard game/archviz camera with field-of-view frustum; position and lookAt control the view.
  - name: BoxGeometry
    id: api:three/boxgeometry
    signature: "new THREE.BoxGeometry(width, height, depth, widthSegments, heightSegments, depthSegments)"
    returns: A BufferGeometry with box vertices, normals, UVs, and indices.
    description: Creates a rectangular cuboid mesh geometry; the most common primitive for prototyping and blockout.
  - name: MeshStandardMaterial
    id: api:three/meshstandardmaterial
    signature: "new THREE.MeshStandardMaterial({ color, roughness, metalness, map })"
    returns: A PBR material.
    description: Physically-based material using roughness/metalness workflow; responds to lights and environment maps.
  - name: Mesh
    id: api:three/mesh
    signature: "new THREE.Mesh(geometry, material)"
    returns: A Mesh object (extends Object3D).
    description: Combines geometry and material into a renderable object in the scene graph.
  - name: WebGLRenderer
    id: api:three/webglrenderer
    signature: "new THREE.WebGLRenderer({ antialias, alpha, powerPreference })"
    returns: A WebGL rendering context.
    description: Creates the WebGL context, manages the canvas, and drives rendering via render(scene, camera).
  - name: setAnimationLoop
    id: api:three/animationloop
    signature: "renderer.setAnimationLoop(callback)"
    returns: void
    description: Registers a per-frame callback receiving a Clock delta; replaces manual requestAnimationFrame with built-in loop management.
  - name: OrbitControls
    id: api:three/orbitcontrols
    signature: "new OrbitControls(camera, domElement)"
    returns: An OrbitControls instance.
    description: Damped orbit, pan, and zoom controls for camera inspection; auto-rotates, min/max polar angles, and target locking.
  - name: Raycaster
    id: api:three/raycaster
    signature: "new THREE.Raycaster(origin, direction, near, far)"
    returns: A Raycaster for intersection testing.
    description: Casts a ray from screen coordinates into the scene and returns intersected objects with face data.
  - name: TextureLoader
    id: api:three/textureloader
    signature: "new THREE.TextureLoader().load(url, onLoad, onProgress, onError)"
    returns: A loaded THREE.Texture.
    description: Loads image textures (jpg, png, webp) asynchronously with optional mipmap generation and anisotropy.
  - name: GLTFLoader
    id: api:three/gltfloader
    signature: "new GLTFLoader().load(url, onLoad, onProgress, onError)"
    returns: A GLTF object containing scene, animations, cameras, and materials.
    description: Loads glTF/GLB models with PBR materials, animations, and scene hierarchy; the recommended 3D asset format.
  - name: Group
    id: api:three/group
    signature: "new THREE.Group()"
    returns: An empty Object3D container.
    description: Parent node for grouping children without rendering itself; useful for transform hierarchies and modular assembly.
  - name: BufferGeometry
    id: api:three/buffergeometry
    signature: "new THREE.BufferGeometry().setAttribute('position', new BufferAttribute(array, 3))"
    returns: A custom geometry.
    description: Defines custom vertex data via typed arrays — position, normal, UV, color, and index attributes.
  - name: ShaderMaterial
    id: api:three/shadermaterial
    signature: "new THREE.ShaderMaterial({ vertexShader, fragmentShader, uniforms })"
    returns: A custom GLSL material.
    description: Full control over the rendering pipeline with custom vertex/fragment shaders and uniform management.
  - name: Fog
    id: api:three/fog
    signature: "scene.fog = new THREE.Fog(color, near, far)"
    returns: void (set on scene).
    description: Exponential or linear fog that fades distant objects into a background colour for depth cues.
  - name: Clock
    id: api:three/clock
    signature: "const clock = new THREE.Clock(); clock.getDelta()"
    returns: Per-frame time deltas and elapsed time.
    description: Provides stable time values for animations — compensates for frame-rate variance and pause events.
examples:
  - id: example:three/scene-setup
    language: js
    description: Minimal scene, camera, renderer, and animated rotating cube.
  - id: example:three/orbit-controls
    language: js
    description: OrbitControls with damping for interactive camera inspection.
  - id: example:three/gltf-load
    language: js
    description: Load a glTF model with DRACO decompression and play its animations.
  - id: example:three/shadows
    language: js
    description: DirectionalLight shadow mapping with PCF soft shadows on a plane.
failures:
  - id: failure:three/gpu-memory-leak
    symptom: Tab memory grows indefinitely; Chrome DevTools shows increasing GPU buffer count.
    cause: Geometry, texture, and material dispose() not called when removing objects from the scene.
    fix: Call mesh.geometry.dispose(), mesh.material.dispose(), and texture.dispose() on removal; use R3F's automatic cleanup.
  - id: failure:three/shadow-acne
    symptom: Dark stripe artifacts on shadowed surfaces; flickering in animated scenes.
    cause: Shadow-map bias too low or too high; camera near/far plane too wide for shadow camera.
    fix: Set renderer.shadowMap.bias = -0.001; tighten shadow camera near/far and frustum size.
  - id: failure:three/render-loop-memory
    symptom: Frame rate drops over time; objects created every frame in the animation loop.
    cause: Instantiating geometries, materials, or textures inside tick() without caching.
    fix: Pool or pre-create all GPU resources; reuse geometries with shared material references.
  - id: failure:three/texture-flip-y
    symptom: Textures appear vertically flipped on some models; correct on others.
    cause: three.js expects flipY=true (default) but glTF/GLB textures are loaded with flipY=false.
    fix: Set texture.flipY = false after loading for glTF textures, or use textureLoader.setFlipY(false).
extends:
  - concept:three/scene-graph
uses:
  - concept:three/renderer
  - concept:three/camera
  - concept:three/materials
  - concept:three/lighting
  - concept:three/animation
  - concept:three/controls
part_of: concept:domain/web-platform
depends_on:
  - package:react/patterns
  - package:typescript/nextjs
solves:
  - problem:3d-web-graphics
alternatives:
  - package:babylonjs
  - package:playcanvas
  - package:unity/webgl
---

# three.js 3D Graphics

three.js is the most widely used WebGL library for browser-based 3D graphics, providing a high-level API over raw WebGL. Its architecture centres on a **scene graph** — a tree of `Object3D` nodes (meshes, lights, cameras, groups) with inherited world transforms. The renderer traverses this tree each frame, submitting draw calls for every visible mesh.

## Scene, Camera, Renderer

Every three.js app needs these three elements. `Scene` holds the object tree; `PerspectiveCamera` defines the view frustum via FOV, aspect ratio, and near/far clipping planes; `WebGLRenderer` creates the WebGL context and drives the frame loop. Call `renderer.render(scene, camera)` inside `setAnimationLoop` to begin drawing. Always resize the renderer and update the camera aspect when the window resizes.

## Materials & Lighting

three.js uses a physically-based rendering (PBR) workflow. `MeshStandardMaterial` uses roughness/metalness inputs and responds to scene lights. `MeshPhysicalMaterial` adds clearcoat, transmission, sheen, and ior for advanced surfaces. For full control, `ShaderMaterial` accepts custom GLSL. Lighting follows physically-based falloff: `DirectionalLight` (sun), `PointLight` (bulb), `SpotLight` (cone), and `AmbientLight` (bounce fill). Use an environment map (equirectangular HDR via `RGBELoader` or PMREMGenerator) for realistic reflections.

## Animation & Controls

Use the `Clock` API to get stable `getDelta()` values and drive per-frame updates. `OrbitControls` from the examples/jsm folder provides damped camera orbit, pan, and zoom with min/max angle constraints — the standard for interactive inspection. For loaded glTF models, `AnimationMixer` plays keyframe tracks, skeletal animations, and morph targets via `clipAction()`.

## Performance & Shadows

Enable shadows selectively: set `renderer.shadowMap.enabled = true`, mark lights with `light.castShadow = true`, and meshes with `mesh.castShadow` / `mesh.receiveShadow`. Use `THREE.PCFSoftShadowMap` for quality. For performance, minimise draw calls by merging static geometry (`BufferGeometryUtils.mergeGeometries`), use `InstancedMesh` for repeated objects, and apply `THREE.LOD` for distance-based detail switching. Always dispose GPU resources when removing objects to avoid memory leaks.

## Decision Tree

### How should I render my three.js scene?

- rendering_mode: webgl → Use WebGLRenderer (universal support, stable API)
- rendering_mode: webgpu → Use WebGPURenderer (compute shaders, faster instancing, Chrome/Edge only)
- rendering_mode: css2d → Use CSS2DRenderer for overlay labels and UI

### Which camera should I use?

- camera_type: perspective → PerspectiveCamera (standard FOV, depth, game/archviz)
- camera_type: orthographic → OrthographicCamera (no perspective distortion, 2D/UI/isometric)
- camera_type: stereo → StereoCamera (VR/AR side-by-side)

### How should I define my geometry?

- geometry_source: primitives → Use built-in BoxGeometry, SphereGeometry, PlaneGeometry for blockout and simple shapes
- geometry_source: buffer → Use BufferGeometry with typed arrays for custom vertex data
- geometry_source: loaded → Use GLTFLoader / FBXLoader for complex authored models
- geometry_source: procedural → Use BufferGeometry with compute-generated vertices for terrain, particle systems, or parametric surfaces

### Which material type should I use?

- material_type: pbr_standard → MeshStandardMaterial (roughness/metalness, general-purpose PBR)
- material_type: pbr_physical → MeshPhysicalMaterial (clearcoat, ior, transmission, sheen, thin-film)
- material_type: unlit → MeshBasicMaterial (flat colour, no lighting — UI, skybox, debug)
- material_type: depth → MeshDepthMaterial (depth-buffer visualisation, shadow-map debugging)
- material_type: custom → ShaderMaterial (custom GLSL for advanced effects — vertex displacement, toon outlines, holograms)
- material_type: normal_map → MeshNormalMaterial (visualise normals, debug geometry)

### How should I light my scene?

- lighting_setup: simple → AmbientLight + DirectionalLight (fast, good enough for product display)
- lighting_setup: studio → Three-point (key DirectionalLight, fill PointLight, rim SpotLight) for character/product showcase
- lighting_setup: environment → HDR environment map via RGBELoader + PMREMGenerator (realistic IBL, PBR reflections)
- lighting_setup: volumetric → SpotLight + PointLight with shadow maps inside a volumetric fog volume

### How should I animate my scene?

- animation_method: keyframe → AnimationMixer + AnimationAction for glTF skeletal/morph animations
- animation_method: procedural → Manual transforms in setAnimationLoop (rotation, position oscillation, noise-based)
- animation_method: gsap → GSAP + three.js for timeline-based chained animations with easing
- animation_method: physics → Cannon-es / Rapier for rigid-body simulation driving three.js transforms

### How should users interact with the scene?

- control_type: orbit → OrbitControls (default — damped orbit, pan, zoom for inspection)
- control_type: fly → FlyControls (free-flight, roll/pitch/yaw, no gravity)
- control_type: first_person → PointerLockControls (FPS-style, mouse-look, WASD movement)
- control_type: transform → TransformControls (translate/rotate/scale gizmo for editing tools)

### How should I handle shadows?

- shadow_quality: off → Disable shadows entirely (maximum performance, mobile)
- shadow_quality: pcf → PCFSoftShadowMap (good quality, moderate GPU cost)
- shadow_quality: vsm → VSMShadowMap (blurred soft shadows, no acne but light bleed)
- shadow_quality: cascaded → CSM (directional light cascade shadow maps for large open worlds)

### Do I need post-processing?

- post_process: none → Straight render (no overhead, highest performance)
- post_process: bloom → UnrealBloomPass (glow effect for lights, emissive materials)
- post_process: dof → BokehPass / DepthOfFieldPass (cinematic focus blur)
- post_process: ssao → SSAOPass (ambient occlusion for contact shadows)
- post_process: tone_mapping → ToneMappingPass (ACES/Reinhard for HDR display)
- post_process: outline → OutlinePass (edge highlighting for selected objects)

### How do I optimise for performance?

- perf_strategy: instancing → InstancedMesh (thousands of identical objects, single draw call)
- perf_strategy: merging → mergeGeometries (static combined geometry for immobile objects)
- perf_strategy: lod → LOD (swap detail levels by camera distance)
- perf_strategy: frustum → Frustum culling (enabled by default — disable only if needed)
- perf_strategy: budget → GPU memory budget tracking — call dispose() on removed resources
- perf_strategy: mobile → Limit pixel ratio (renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))), disable shadows, reduce draw calls

### How should I load assets?

- load_target: textures → TextureLoader / CubeTextureLoader (jpg, png, webp, HDR)
- load_target: models_gltf → GLTFLoader + DRACOLoader (compressed glTF/GLB, recommended format)
- load_target: models_other → FBXLoader, OBJLoader, STLLoader (legacy formats, fallback)
- load_target: fonts → FontLoader (TextGeometry for 3D text)
- load_target: video → VideoTexture (live video mapped onto 3D surfaces)
