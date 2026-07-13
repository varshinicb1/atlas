---
kind: Package
id: package:uploadthing
name: UploadThing
version: "7.1"
purpose: Document UploadThing — file upload infrastructure for full-stack TypeScript applications with built-in file routing, size/type validation, image optimization, and framework-specific React components for Next.js, SvelteKit, and Express.
problem_solved: Replaces the tedious integration of file uploads across storage provider (S3/R2), API endpoints, validation, progress tracking, image optimization, and security — providing a single upload button component that handles the entire pipeline from client to storage.
install: npm install uploadthing @uploadthing/react
dependencies:
  - concept:file-upload
  - concept:s3-compatible-storage
  - concept:react
concepts:
  - name: File Routes
    id: concept:uploadthing/file-routes
    description: Server-side definitions that specify what files are accepted — createUploadthing() with input validation (zod) and middleware (auth). Each file route has maxFileSize, maxFileCount, contentDisposition, and allowed MIME types. Routes are the authorization boundary — who can upload what.
  - name: Upload Button
    id: concept:uploadthing/button
    description: Client-side <UploadButton /> component from @uploadthing/react. Handles file selection, upload progress (percentage, speed, ETA), preview, error display, and completion callback. Customizable via appearance prop and content slot overrides.
  - name: Upload Dropzone
    id: concept:uploadthing/dropzone
    description: <UploadDropzone /> — drag-and-drop upload area with file preview, multi-file support, and the same progress/error handling as UploadButton. Use when you want a drop zone instead of a button trigger.
  - name: Server Upload Handler
    id: concept:uploadthing/handler
    description: "API route handler — in Next.js App Router, export { POST } from app/api/uploadthing/core.ts. The handler receives file chunks, validates the file route, checks auth (via middleware), writes to storage, and returns the file URL."
  - name: Image Optimization
    id: concept:uploadthing/image-optimization
    description: Built-in image transformation via URL parameters — ?w=400&h=300&fit=crop resizes and crops server-side. UploadThing stores originals and applies optimizations on-the-fly. Supports format conversion (webp, avif), blur, and quality adjustment.
  - name: File Router Callbacks
    id: concept:uploadthing/callbacks
    description: onUploadComplete runs after successful upload — receive file metadata (url, name, size, key). Use to update database records, send notifications, or trigger processing workflows. The callback runs server-side with the auth context.
  - name: Upload Errors
    id: concept:uploadthing/errors
    description: UploadFileError types — too large, wrong type, too many files, unauthorized, upload aborted. Client-side components display human-readable error messages. Custom error handling via onUploadError callback.
  - name: Security
    id: concept:uploadthing/security
    description: Upload routes are protected by your auth middleware. The middleware function in file route definition receives the request and returns auth context. Files are validated by size, type, and count before any storage write. ACL policies on the storage bucket restrict public access.
apis:
  - name: createUploadthing(opts)
    id: api:uploadthing/create
    signature: "const f = createUploadthing({ logLevel: 'info' }); const fileRouter = { image: f({ image: { maxFileSize: '4MB', maxFileCount: 4 } }).middleware(authCheck).onUploadComplete(handler) }"
    returns: A file router definition.
    description: Creates the file router factory. Each entry defines an upload route with size/type constraints, auth middleware, and completion callback. The middleware returns metadata passed to onUploadComplete.
  - name: <UploadButton endpoint={...} />
    id: api:uploadthing/button
    signature: "<UploadButton endpoint='image' onClientUploadComplete={(res) => setUrls(res.map(r => r.url))} onUploadError={(e) => toast.error(e.message)} appearance={{ button: 'bg-blue-500' }} />"
    returns: An upload button React component.
    description: "Client-side upload trigger. endpoint must match a file route key. Props: onClientUploadComplete, onUploadError, onBeforeUploadBegin, appearance, content (slot overrides), skipPolling, input (additional metadata)."
  - name: <UploadDropzone endpoint={...} />
    id: api:uploadthing/dropzone
    signature: "<UploadDropzone endpoint='image' onClientUploadComplete={(res) => setUrls(res.map(r => r.url))} />"
    returns: A drag-and-drop upload zone.
    description: Dropzone variant of UploadButton. Same props. Shows preview of selected files before upload. Multi-file upload starts automatically on drop or after file selection via click.
  - name: utapi (server API)
    id: api:uploadthing/server-api
    signature: "const utapi = new UTApi(); const deleted = await utapi.deleteFiles([fileKey])"
    returns: UTApi instance with file management methods.
    description: "Server-side file management API. Methods: uploadFiles, deleteFiles, renameFile, getFileUrl, listFiles, getUsageInfo. Requires UploadThing secret key. Use in webhooks, admin panels, and cleanup jobs."
  - name: generateUploadDropzone(opts)
    id: api:uploadthing/generate
    signature: "import { generateComponents } from '@uploadthing/react'; export const { UploadButton, UploadDropzone } = generateComponents<OurFileRouter>()"
    returns: Typed React components.
    description: Generates typed upload components from your file router type. Ensures endpoint prop is type-checked against your file route keys. Must be called once and exported from a shared module.
failures:
  - id: failure:uploadthing/cors-in-media-upload
    symptom: Upload fails with CORS errors when using a custom S3-compatible storage endpoint.
    cause: Custom storage endpoint does not have CORS configured to allow browser uploads.
    fix: If using UploadThing's managed storage, CORS is pre-configured. For custom endpoints, configure CORS rules to allow PUT from your app's origin. Use server-side upload endpoints as a proxy if CORS cannot be configured.
  - id: failure:uploadthing/maxsize-exceeded
    symptom: Upload aborts silently or returns "File too large" for files under the configured limit.
    cause: The actual file size after base64 encoding or multipart overhead exceeds the limit, or the limit is configured in MB but the upload checks bytes.
    fix: Check maxFileSize unit — '4MB' is proper, '4' would be 4 bytes. Account for multipart/base64 overhead (~33%). Increase limit by 20% for text-based encodings.
  - id: failure:uploadthing/auth-context-empty
    symptom: onUploadComplete receives undefined metadata even though middleware returns valid data.
    cause: Middleware function does not return an object, or the returned object is not serializable (contains functions, circular references, or class instances).
    fix: Ensure middleware returns a plain serializable object. Do not return the request or response objects. Use middleware to transform auth data into a metadata object.
  - id: failure:uploadthing/callback-not-firing
    symptom: Files upload successfully but onUploadComplete never runs.
    cause: Server-side webhook callback URL not configured, or polling interval exceeds the upload time.
    fix: "Enable onUploadComplete polling by using skipPolling: false. Ensure the server endpoint (app/api/uploadthing) is accessible from UploadThing's webhook. Check the UploadThing dashboard for webhook delivery logs."
extends: []
implements: []
uses:
  - concept:file-upload
  - concept:s3-compatible-storage
part_of: concept:full-stack-media-stack
solves:
  - problem:file-upload-infrastructure
  - problem:image-optimization-pipeline
  - problem:upload-ui-components
alternatives:
  - package:s3-direct-upload
  - package:vercel-blob
  - package:cloudflare-images-direct-upload
---
UploadThing abstracts the entire file upload pipeline behind a single component. The traditional approach requires: an S3 bucket with CORS, a presigned URL endpoint, a multipart upload handler, validation middleware, image transformation pipeline, and a progress UI. UploadThing replaces all of this with a file route definition on the server and UploadButton on the client.

The file route is the security and validation boundary. Each route defines who can upload (via middleware returning auth context), what they can upload (MIME type, size limit, count limit), and what happens after upload (onUploadComplete callback). The middleware pattern means auth logic (check session, verify permissions, rate-limit) is centralised — upload routes cannot be accessed without passing the middleware.

UploadThing stores files in its managed S3-compatible storage and provides URL-based image transformations. The ?w=400&h=300&fit=crop query parameters resize and optimize images on-the-fly without pre-generating thumbnails or running a separate image server. Format conversion (webp, avif) happens automatically based on the Accept header, delivering modern formats when the browser supports them.
