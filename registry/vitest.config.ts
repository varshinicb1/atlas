import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const dir = fileURLToPath(new URL(".", import.meta.url));

export default defineConfig({
  test: {
    environment: "node",
    include: ["src/**/*.test.ts"],
    globals: true,
  },
  resolve: {
    alias: {
      "cloudflare:workers": fileURLToPath(
        new URL("./test/cloudflare-workers-mock.ts", import.meta.url)
      ),
    },
  },
});
