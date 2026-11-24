import { register } from "node:module";

const SRC_DIR = "d:/Git/codex-plus/apps/codex-plus-manager/src";

const hooks = `
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";

const SRC_DIR = ${JSON.stringify(SRC_DIR)};

export async function resolve(specifier, context, nextResolve) {
  if (specifier.startsWith("@/")) {
    const candidate = path.join(SRC_DIR, specifier.slice(2));
    for (const ext of ["", ".ts", ".tsx", ".mts", ".js", ".mjs"]) {
      const p = candidate + ext;
      if (fs.existsSync(p) && fs.statSync(p).isFile()) {
        return nextResolve(pathToFileURL(p).href, context);
      }
    }
  }
  if ((specifier.startsWith("./") || specifier.startsWith("../")) && context.parentURL?.startsWith("file:")) {
    const base = path.dirname(fileURLToPath(context.parentURL));
    const candidate = path.resolve(base, specifier);
    if (!fs.existsSync(candidate) || !fs.statSync(candidate).isFile()) {
      for (const ext of [".ts", ".tsx", ".mts", ".js", ".mjs"]) {
        const p = candidate + ext;
        if (fs.existsSync(p) && fs.statSync(p).isFile()) {
          return nextResolve(pathToFileURL(p).href, context);
        }
      }
    }
  }
  return nextResolve(specifier, context);
}
`;

register("data:text/javascript;base64," + Buffer.from(hooks, "utf8").toString("base64"));
