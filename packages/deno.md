---
kind: Package
id: package:deno
name: "deno Knowledge Package"
version: "0.1.0"
purpose: |
  Auto-generated knowledge package crawled from https://deno.land/.
  Covers 5 pages of documentation.
problem_solved: |
  Provides structured knowledge extracted from the official deno.land documentation
  for use in AI agent decision-making.
install: |
  ```bash
  atlas install deno.md
  ```
concepts:
  - name: "Better, faster JavaScript"
    id: concept:page_0_deno
    description: |
      Extracted from documentation: Better, faster JavaScript
  - name: "Install Deno 2.9.2"
    id: concept:page_1_deno
    description: |
      Extracted from documentation: Install Deno 2.9.2
  - name: "Everything just works"
    id: concept:page_2_deno
    description: |
      Extracted from documentation: Everything just works
  - name: "Your Node project, without the Node problems"
    id: concept:page_3_deno
    description: |
      Extracted from documentation: Your Node project, without the Node problems
  - name: "Built-in everything"
    id: concept:page_4_deno
    description: |
      Extracted from documentation: Built-in everything
  - name: "On the cutting edge of JavaScript"
    id: concept:page_5_deno
    description: |
      Extracted from documentation: On the cutting edge of JavaScript
  - name: "Granular security controls"
    id: concept:page_6_deno
    description: |
      Extracted from documentation: Granular security controls
  - name: "Faster than you think"
    id: concept:page_7_deno
    description: |
      Extracted from documentation: Faster than you think
  - name: "See anything, understand everything"
    id: concept:page_8_deno
    description: |
      Extracted from documentation: See anything, understand everything
  - name: "Deploy Deno anywhere"
    id: concept:page_9_deno
    description: |
      Extracted from documentation: Deploy Deno anywhere
  - name: "And now, the desktop"
    id: concept:page_10_deno
    description: |
      Extracted from documentation: And now, the desktop
  - name: "Loved by developers"
    id: concept:page_11_deno
    description: |
      Extracted from documentation: Loved by developers
  - name: "Better JavaScript"
    id: concept:page_12_deno
    description: |
      Extracted from documentation: Better JavaScript
  - name: "Zero-setup TypeScript and npm"
    id: concept:page_13_deno
    description: |
      Extracted from documentation: Zero-setup TypeScript and npm
  - name: "A faster package manager"
    id: concept:page_14_deno
    description: |
      Extracted from documentation: A faster package manager
  - name: "Standard library included"
    id: concept:page_15_deno
    description: |
      Extracted from documentation: Standard library included
  - name: "Three layers of defense"
    id: concept:page_16_deno
    description: |
      Extracted from documentation: Three layers of defense
  - name: "Built for the real world"
    id: concept:page_17_deno
    description: |
      Extracted from documentation: Built for the real world
  - name: "OTEL in 5 seconds"
    id: concept:page_18_deno
    description: |
      Extracted from documentation: OTEL in 5 seconds
  - name: "Profiling"
    id: concept:page_19_deno
    description: |
      Extracted from documentation: Profiling
  - name: "Debugging"
    id: concept:page_20_deno
    description: |
      Extracted from documentation: Debugging
  - name: "The hosting platform built for Deno"
    id: concept:page_21_deno
    description: |
      Extracted from documentation: The hosting platform built for Deno
  - name: "eye {"
    id: concept:page_22_deno
    description: |
      Extracted from documentation: eye {
  - name: "Now Generally AvailableDeno Deploy"
    id: concept:page_23_deno
    description: |
      Extracted from documentation: Now Generally AvailableDeno Deploy
  - name: "— What’s new —"
    id: concept:page_24_deno
    description: |
      Extracted from documentation: — What’s new —
  - name: "— What’s improved —"
    id: concept:page_25_deno
    description: |
      Extracted from documentation: — What’s improved —
  - name: "Still trusted by the best"
    id: concept:page_26_deno
    description: |
      Extracted from documentation: Still trusted by the best
  - name: "Deno Deploy success stories"
    id: concept:page_27_deno
    description: |
      Extracted from documentation: Deno Deploy success stories
  - name: "Ready to try the new Deno Deploy?"
    id: concept:page_28_deno
    description: |
      Extracted from documentation: Ready to try the new Deno Deploy?
  - name: "Made for all JavaScript"
    id: concept:page_29_deno
    description: |
      Extracted from documentation: Made for all JavaScript
  - name: "Has everything your project needs"
    id: concept:page_30_deno
    description: |
      Extracted from documentation: Has everything your project needs
  - name: "A data layer built for simplicity"
    id: concept:page_31_deno
    description: |
      Extracted from documentation: A data layer built for simplicity
  - name: "Powerful developer tooling"
    id: concept:page_32_deno
    description: |
      Extracted from documentation: Powerful developer tooling
  - name: "A fast and flexible platform"
    id: concept:page_33_deno
    description: |
      Extracted from documentation: A fast and flexible platform
  - name: "Built for builders"
    id: concept:page_34_deno
    description: |
      Extracted from documentation: Built for builders
  - name: "Managed, or self-hosted"
    id: concept:page_35_deno
    description: |
      Extracted from documentation: Managed, or self-hosted
  - name: "How Netlify used Deno Subhosting to build a successful edge functions product"
    id: concept:page_36_deno
    description: |
      Extracted from documentation: How Netlify used Deno Subhosting to build a successful edge functions product
  - name: "How Slack used Deno to save months of engineering effort in launching their new platform"
    id: concept:page_37_deno
    description: |
      Extracted from documentation: How Slack used Deno to save months of engineering effort in launching their new platform
  - name: "How Brazil's top ecommerce platform used Deno Subhosting to drive 5x faster page load speeds"
    id: concept:page_38_deno
    description: |
      Extracted from documentation: How Brazil's top ecommerce platform used Deno Subhosting to drive 5x faster page load speeds
apis:
  - name: "import { Hono } from \"hono\";"
    id: api:crawl_0_deno
    signature: |
      import { Hono } from "hono";
      
      interface Book {
        title: string;
        year: number;
      }
      
      const books: Book[] = [{ title: "Deno", year: 2018 }];
      
      const app = new Hono();
      app.get("/books", (c) => c.json(books));
      
      export default app;
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_1_deno
    signature: |
      {
        "name": "project-with-npm-deps",
        "dependencies": {
          "chalk": "^5.6.2",
          "hono": "^4.12.27"
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "{"
    id: api:crawl_2_deno
    signature: |
      {
        "workspaces": ["./api", "./web", "./core"],
        "scripts": {
          "dev": "deno run -A --watch main.ts",
          "test": "deno test -A"
        },
        "dependencies": {
          "hono": "hono^4"
        }
      }
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "import { BinarySearchTree } from \"@std/data-structures\";"
    id: api:crawl_3_deno
    signature: |
      import { BinarySearchTree } from "@std/data-structures";
      import { assertEquals } from "@std/assert";
      
      const values = [3, 10, 13, 4, 6, 7, 1, 14];
      const tree = new BinarySearchTree<number>();
      values.forEach((value) => tree.insert(value));
      
      assertEquals([...tree], [1, 3, 4, 6, 7, 10, 13, 14]);
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "minimumDependencyAge: \"P3D\""
    id: api:crawl_4_deno
    signature: |
      minimumDependencyAge: "P3D"
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "DENO_TRACE_PERMISSIONS=1"
    id: api:crawl_5_deno
    signature: |
      DENO_TRACE_PERMISSIONS=1
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "DENO_PERMISSION_BROKER_PATH"
    id: api:crawl_6_deno
    signature: |
      DENO_PERMISSION_BROKER_PATH
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "DENO_AUDIT_PERMISSIONS"
    id: api:crawl_7_deno
    signature: |
      DENO_AUDIT_PERMISSIONS
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "deno run --inspect-brk"
    id: api:crawl_8_deno
    signature: |
      deno run --inspect-brk
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "export async function handler(req) {"
    id: api:crawl_9_deno
    signature: |
      export async function handler(req) {
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "const user = await auth(req)"
    id: api:crawl_10_deno
    signature: |
      const user = await auth(req)
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "const cart = await loadCart(user)"
    id: api:crawl_11_deno
    signature: |
      const cart = await loadCart(user)
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "$ deno task --tunnel dev"
    id: api:crawl_12_deno
    signature: |
      $ deno task --tunnel dev
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "https://app--local.org.deno.net"
    id: api:crawl_13_deno
    signature: |
      https://app--local.org.deno.net
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "Send telemetry, use prod env vars,"
    id: api:crawl_14_deno
    signature: |
      Send telemetry, use prod env vars,
    returns: See documentation
    description: |
      Extracted code example from documentation.
  - name: "and more with tunneling!"
    id: api:crawl_15_deno
    signature: |
      and more with tunneling!
    returns: See documentation
    description: |
      Extracted code example from documentation.
failures:

---

# deno

Auto-generated knowledge package crawled from https://deno.land/.

**Pages crawled**: 5
**Source**: https://deno.land/

# Deno, the drop-in JavaScript runtime for Node developers

A drop-in JavaScript runtime with built-in TypeScript, npm support, and security by default. Run your existing Node project on Deno with no setup.

Deno is a fast, open-source, fully Node-compatible JS runtime with TypeScript and everything else you need baked right in.

Faster than Node; more stable than Bun.

Built-in tools: package manager, test runner, formatter, linter, task runner, type checker, coverage tool, workspace manager, benchmarker, doc generator, jupyter kernel, compiler, desktop app builder.

Available today: Temporal · Set methods · Iterator helpers · Promise.try · Float16Array

1Default denySupply-chain safetyShut down the most common ways a dependency turns hostile.Postinstall scriptsOff by defaultOpt a package in--allow-scriptsTrusted dynamic imports--allow-importAudit the dependency treedeno auditApply the fixesdeno audit fixMinimum dependency ageminimumDependencyAge: "P3D"

2Scoped grantsRuntime sandboxScope what a single program can do, and deny-list the rest.Granular allow flags--allow-netDeny-list on top--deny-netSubprocess isolationApply permissions to Deno child processesPermission stack tracesDENO_TRACE_PERMISSIONS=1Symlink-aware checksEvaluated at the link

3AuditsCentralized policy & auditGovern permissions, and keep a record of every call.Permission brokerDENO_PERMISSION_BROKER_PATHAudit logsDENO_AUDIT_PERMISSIONS

memory, large responses

p99 latency, realworld

100 concurrent connections · AMD EPYC x86_64, pinned cores · oha, median of 3 runs, uncompressed · Deno.serve (Deno 2.9), Bun.serve (Bun 1.4), node:http (Node v26). Results vary by workload and hardware.

Official Docker image

How to Deploy Deno to Digital Ocean

Run a Deno App - Fly.io Docs

anthonychu/azure-functions-deno-worker

How to Deploy to Google Cloud Run

…and any platform that runs a Linux, macOS, or Windows binary.

Or run it on the platform we built for it.

Trusted in production by teams at

## Better, faster JavaScript

## Install Deno 2.9.2

## Everything just works

## Your Node project, without the Node problems

## Built-in everything

## On the cutting edge of JavaScript

## Granular security controls

## Faster than you think

## See anything, understand everything

## Deploy Deno anywhere

## And now, the desktop

## Loved by developers

## Better JavaScript

## Zero-setup TypeScript and npm

## A faster package manager

## Standard library included

## Three layers of defense

## Built for the real world

## OTEL in 5 seconds

## Profiling

## Debugging

## The hosting platform built for Deno

```
import { Hono } from "hono";

interface Book {
  title: string;
  year: number;
}

const books: Book[] = [{ title: "Deno", year: 2018 }];

const app = new Hono();
app.get("/books", (c) => c.json(books));

export default app;
```

```
{
  "name": "project-with-npm-deps",
  "dependencies": {
    "chalk": "^5.6.2",
    "hono": "^4.12.27"
  }
}
```

```
{
  "workspaces": ["./api", "./web", "./core"],
  "scripts": {
    "dev": "deno run -A --watch main.ts",
    "test": "deno test -A"
  },
  "dependencies": {
    "hono": "hono^4"
  }
}
```

```
import { BinarySearchTree } from "@std/data-structures";
import { assertEquals } from "@std/assert";

const values = [3, 10, 13, 4, 6, 7, 1, 14];
const tree = new BinarySearchTree<number>();
values.forEach((value) => tree.insert(value));

assertEquals([...tree], [1, 3, 4, 6, 7, 10, 13, 14]);
```

```
minimumDependencyAge: "P3D"
```

```
DENO_TRACE_PERMISSIONS=1
```

```
DENO_PERMISSION_BROKER_PATH
```

```
DENO_AUDIT_PERMISSIONS
```

```
deno run --inspect-brk
```

```
export async function handler(req) {
```

```
const user = await auth(req)
```

```
const cart = await loadCart(user)
```

## eye {

            transform-origin: 50% 30%;
            animation: blink 0.15s cubic-bezier(0.5, 0, 0.5, 1);
          }
        }
        </style></svg></a><label tabindex="0" class="cursor-pointer lg:hidden touch-manipulation" for="menuToggle" onkeydown="if (event.code === &#39;Space&#39; || event.code === &#39;Enter&#39;) { this.click(); event.preventDefault(); }"><svg  width="21" height="14" viewBox="0 0 21 14" fill="none" xmlns="http://www.w3.org/2000/svg"><line x1="0.25" y1="1.4" x2="20.25" y2="1.4" stroke="currentColor" stroke-width="1.2"></line><line x1="0.25" y1="7.4" x2="20.25" y2="7.4" stroke="currentColor" stroke-width="1.2"></line><line x1="0.25" y1="13.4" x2="10.25" y2="13.4" stroke="currentColor" stroke-width="1.2"></line></svg><svg class="h-6 w-6 hidden" stroke="currentColor" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></label></div><div class="hidden flex-col w-full mt-5 gap-x-8 gap-y-4 lg:flex lg:flex-row lg:items-center lg:mx-0 lg:mt-0" data-component="nav-entries"><!--frsh:island:OramaSearch:2:--><div class="md:relative w-full lg:hidden"><div class="relative"><input type="search" placeholder="Search" id="orama-search-input"   value=""   class="w-full min-w-24 rounded-lg placeholder:text-sm text-base leading-normal p-1 pl-8 border transition-all duration-150
          border-gray-600 dark:border-gray-100 focus:outline-offset-1 disabled:opacity-50"><svg class="absolute top-2 left-2 size-4.5 text-gray-600 pointer-events-none " width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_2006_89)"><path d="M13.3008 12.1951L10.3537 9.24807C11.0632 8.30353 11.4463 7.15382 11.445 5.97248C11.445 2.95497 8.98999 0.5 5.97248 0.5C2.95497 0.5 0.5 2.95497 0.5 5.97248C0.5 8.98999 2.95497 11.445 5.97248 11.445C7.15382 11.4463 8.30353 11.0632 9.24807 10.3537L12.1951 13.3008C12.3443 13.4341 12.5389 13.5053 12.7389 13.4997C12.9389 13.4941 13.1292 13.4121 13.2707 13.2707C13.4121 13.1292 13.4941 12.9389 13.4997 12.7389C13.5053 12.5389 13.4341 12.3443 13.3008 12.1951ZM2.06357 5.97248C2.06357 5.19937 2.29282 4.44362 2.72234 3.8008C3.15185 3.15799 3.76234 2.65697 4.4766 2.36111C5.19086 2.06526 5.97682 1.98785 6.73507 2.13867C7.49333 2.2895 8.18983 2.66179 8.7365 3.20846C9.28317 3.75513 9.65546 4.45163 9.80629 5.20989C9.95711 5.96814 9.8797 6.7541 9.58385 7.46836C9.28799 8.18262 8.78697 8.79311 8.14416 9.22262C7.50134 9.65214 6.74559 9.88139 5.97248 9.88139C4.93615 9.88015 3.94263 9.46792 3.20983 8.73513C2.47704 8.00233 2.06481 7.00881 2.06357 5.97248Z" fill="currentColor"></path></g><defs><clipPath id="clip0_2006_89"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg><kbd id="search-key" class="hidden xs:flex pointer-events-none absolute font-sans rounded-sm top-1 right-1 bottom-1 w-auto border-1 border-gray-500 dark:border-gray-800 border-b-2 border-r-2 bg-white text-gray-600 dark:text-gray-100 text-center text-xs font-bold p-2 items-center justify-center dark:bg-gray-800">⌘K</kbd><div id="orama-search-loading" class="absolute left-2 top-1/2 transform -translate-y-1/2 hidden bg-white dark:bg-black"><div class="animate-spin rounded-full h-4 w-4 border-2 border-transparent border-r-black dark:border-r-white bg-white dark:bg-black"></div></div><div class="sr-only" aria-live="polite" id="orama-results-announcer" ></div></div><div id="orama-search-results" class="absolute inset-2 left-2 right-2 h-[calc(100vh-10rem)] lg:h-[calc(100vh-8rem)] top-32 lg:top-10 md:top-full md:left-auto md:right-0 mt-2 bg-white dark:bg-black dark:text-white border border-gray-500 dark:border-gray-800 rounded-xl shadow-2xl z-50 md:max-h-128 overflow-hidden hidden md:min-w-160 max-w-2xl"><div id="orama-search-results-content" class="overflow-y-auto h-full"></div><div class="border-t border-gray-500 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2 sticky bottom-0"><div class="flex items-center gap-6 text-xs text-gray-500 dark:text-gray-500"><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↑↓</span><span class="sr-only">Up or down to</span></kbd>navigate</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↵</span><span class="sr-only">Enter to</span></kbd>select</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">ESC</span><span class="sr-only">Escape to</span></kbd>close</span><img src="/images/orama-dark.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="dark:hidden h-4 w-auto ml-auto"/><img src="/images/orama-light.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="hidden dark:inline h-4 w-auto ml-auto"/></div></div></div></div><!--/frsh:island--><div class="flex flex-col leading-loose divide-incl-y lg:flex-row lg:gap-x-4 lg:select-none lg:divide-incl-y-0"><style>.entry-children li:nth-child(1) a{ transition-delay: calc(var(--delay-increment) * 1); }.entry-children li:nth-child(2) a{ transition-delay: calc(var(--delay-increment) * 2); }.entry-children li:nth-child(3) a{ transition-delay: calc(var(--delay-increment) * 3); }.entry-children li:nth-child(4) a{ transition-delay: calc(var(--delay-increment) * 4); }.entry-children li:nth-child(5) a{ transition-delay: calc(var(--delay-increment) * 5); }.entry-children li:nth-child(6) a{ transition-delay: calc(var(--delay-increment) * 6); }.entry-children li:nth-child(7) a{ transition-delay: calc(var(--delay-increment) * 7); }.entry-children li:nth-child(8) a{ transition-delay: calc(var(--delay-increment) * 8); }.entry-children li:nth-child(9) a{ transition-delay: calc(var(--delay-increment) * 9); }</style><!--frsh:island:NavItemWithChildren:3:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Products"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Products</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="lg:grid lg:grid-cols-2
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><div id="category-Open Source" class="dark:text-runtime text-[0.625rem] uppercase font-bold lg:mx-5 lg:mt-3.5">Open Source</div><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5" aria-labelledby="category-Open Source"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="true" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="/" data-ancestor="true"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Modern runtime for JavaScript and TypeScript</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://fresh.deno.dev"><div class="w-full"><div class="w-max flex gap-4 items-center">Fresh</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Web framework designed for the edge</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://clawpatrol.dev"><div class="w-full"><div class="w-max flex gap-4 items-center">Claw Patrol</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Open-source security firewall for agents</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://jsr.io"><div class="w-full"><div class="w-max flex gap-4 items-center">JSR</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">TypeScript-first ESM package registry</div></div></a></li></ul></div><div data-component="entry-children" class="lg:hidden
      lg:border-l lg:border-azure2 lg:dark:border-gray-800
      entry-children w-full flex flex-col"><div id="category-Commercial" class="dark:text-deploy text-[0.625rem] uppercase font-bold lg:mx-5 lg:mt-3.5">Commercial</div><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-0" aria-labelledby="category-Commercial"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 nav-product-tree-parent"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pl-5 lg:px-5" href="/deploy"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno Deploy</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Easy serverless hosting for your JavaScript projects</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 ml-1.5 lg:ml-5 nav-product-tree-nested nav-product-tree-nested-first"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 pl-3.5 lg:px-4" href="/deploy/sandbox"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno Sandbox</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Run untrusted code in secure Linux VMs. Built for AI agents.</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 ml-1.5 lg:ml-5 nav-product-tree-nested nav-product-tree-nested-last"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 pl-3.5 lg:px-4" href="/subhosting"><div class="w-full"><div class="w-max flex gap-4 items-center">Subhosting</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Extend your platform using Deno Deploy's secure infrastructure</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pl-5 lg:px-5" href="/enterprise"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno for Enterprise</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Enterprise support for runtime projects</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><div class="dark:border-gray-800"><a class="block w-full px-1 my-3 lg:w-auto lg:m-0 lg:px-2 lg:rounded-md lg:hover:bg-azure3 lg:hover:dark:bg-gray-800 " href="https://docs.deno.com" aria-current="false">Docs</a></div><!--frsh:island:NavItemWithChildren:4:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Modules"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Modules</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://jsr.io/@std"><div class="w-full"><div class="w-max flex gap-4 items-center">Standard Library</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://jsr.io"><div class="w-full"><div class="w-max flex gap-4 items-center">JSR</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="/npm/"><div class="w-full"><div class="w-max flex gap-4 items-center">Node.js &amp; npm</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><!--frsh:island:NavItemWithChildren:5:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Community"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Community</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://discord.gg/deno"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="Discord Logo"><g clip-path="url(#clip0_1993_195)"><path d="M11.8595 2.62936C10.9397 2.21264 9.96862 1.917 8.97117 1.75C8.83467 1.99146 8.71117 2.23988 8.60118 2.49424C7.5387 2.33581 6.45822 2.33581 5.39574 2.49424C5.28569 2.23991 5.16219 1.99149 5.02575 1.75C4.02766 1.91841 3.05598 2.21475 2.13524 2.63154C0.307332 5.30775 -0.188185 7.9175 0.0595735 10.4902C1.13004 11.2729 2.3282 11.8681 3.60197 12.25C3.88878 11.8683 4.14258 11.4633 4.36066 11.0394C3.94644 10.8863 3.54665 10.6974 3.16591 10.4749C3.26612 10.403 3.36412 10.3289 3.45882 10.257C4.56668 10.7726 5.77586 11.0399 7.00011 11.0399C8.22437 11.0399 9.43354 10.7726 10.5414 10.257C10.6372 10.3344 10.7352 10.4085 10.8343 10.4749C10.4528 10.6978 10.0523 10.887 9.63736 11.0405C9.85518 11.4642 10.109 11.8688 10.3961 12.25C11.6709 11.8696 12.87 11.2747 13.9406 10.4913C14.2314 7.50778 13.444 4.92201 11.8595 2.62936ZM4.67449 8.908C3.98407 8.908 3.41367 8.28798 3.41367 7.52522C3.41367 6.76245 3.96425 6.13699 4.67228 6.13699C5.38032 6.13699 5.94631 6.76245 5.9342 7.52522C5.92209 8.28798 5.37812 8.908 4.67449 8.908ZM9.32574 8.908C8.63422 8.908 8.06602 8.28798 8.06602 7.52522C8.06602 6.76245 8.6166 6.13699 9.32574 6.13699C10.0349 6.13699 10.5965 6.76245 10.5843 7.52522C10.5722 8.28798 10.0294 8.908 9.32574 8.908Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1993_195"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></div>Discord</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://github.com/denoland"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-5 h-5" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="GitHub Logo"><g clip-path="url(#clip0_1989_191)"><path d="M7.00001 0C3.13391 0 0 3.21295 0 7.17755C0 10.3482 2.0055 13.0388 4.7873 13.9875C5.1373 14.0534 5.26471 13.832 5.26471 13.6414C5.26471 13.4716 5.25912 13.0195 5.25561 12.4212C3.3082 12.8547 2.8973 11.4589 2.8973 11.4589C2.5795 10.6291 2.1203 10.4084 2.1203 10.4084C1.48471 9.96418 2.16861 9.97279 2.16861 9.97279C2.87071 10.0229 3.24032 10.7122 3.24032 10.7122C3.86472 11.8085 4.87903 11.4918 5.27732 11.3084C5.34171 10.8448 5.52232 10.5288 5.72251 10.3497C4.16851 10.1684 2.534 9.55218 2.534 6.80211C2.534 6.01893 2.807 5.37764 3.2543 4.87605C3.1822 4.69476 2.94211 3.96463 3.32289 2.97722C3.32289 2.97722 3.91089 2.78376 5.24789 3.71238C5.77305 3.55992 6.37629 3.47184 6.99948 3.4709C7.59448 3.47377 8.19351 3.5533 8.7528 3.71238C10.0891 2.78376 10.6757 2.97649 10.6757 2.97649C11.0579 3.9646 10.8171 4.69475 10.7457 4.87603C11.1937 5.3776 11.4653 6.0189 11.4653 6.80208C11.4653 9.55931 9.82799 10.1662 8.26908 10.3439C8.52037 10.5653 8.74368 11.0031 8.74368 11.6731C8.74368 12.6318 8.73529 13.4064 8.73529 13.6414C8.73529 13.8335 8.86129 14.057 9.21689 13.9868C12.0205 13.0032 14 10.3285 14 7.18046C14 7.17943 14 7.17841 14 7.17738C14 3.21278 10.8654 0 7.00001 0Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1989_191"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></div>GitHub</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://bsky.app/profile/deno.land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="BlueSky Logo"><path d="M12 10.8c-1.087-2.114-4.046-6.053-6.798-7.995C2.566.944 1.561 1.266.902 1.565.139 1.908 0 3.08 0 3.768c0 .69.378 5.65.624 6.479.815 2.736 3.713 3.66 6.383 3.364.136-.02.275-.039.415-.056-.138.022-.276.04-.415.056-3.912.58-7.387 2.005-2.83 7.078 5.013 5.19 6.87-1.113 7.823-4.308.953 3.195 2.05 9.271 7.733 4.308 4.267-4.308 1.172-6.498-2.74-7.078a8.741 8.741 0 0 1-.415-.056c.14.017.279.036.415.056 2.67.297 5.568-.628 6.383-3.364.246-.828.624-5.79.624-6.478 0-.69-.139-1.861-.902-2.206-.659-.298-1.664-.62-4.3 1.24C16.046 4.748 13.087 8.687 12 10.8Z" fill="currentColor"></path></svg></div>Bluesky</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://twitter.com/deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg width="300" height="300.251" class="w-4.5 h-4.5" viewBox="0 0 300 300.251" version="1.1" xmlns="http://www.w3.org/2000/svg" aria-label="X logo" fill="currentColor"><path d="M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"></path></svg></div>X</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://www.youtube.com/c/deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="YouTube Logo"><path d="M7.03863 2.5H7.10538C7.72192 2.5024 10.8459 2.52643 11.6882 2.76826C11.9428 2.84207 12.1748 2.98593 12.3611 3.18548C12.5473 3.38502 12.6813 3.63326 12.7495 3.90537C12.8252 4.20967 12.8785 4.61247 12.9145 5.02807L12.922 5.11135L12.9385 5.31956L12.9445 5.40284C12.9932 6.13475 12.9992 6.82022 13 6.96997V7.03003C12.9992 7.18538 12.9925 7.9173 12.9385 8.67964L12.9325 8.76372L12.9257 8.84701C12.8882 9.30505 12.8327 9.7599 12.7495 10.0946C12.6815 10.3669 12.5476 10.6152 12.3613 10.8148C12.1751 11.0144 11.9429 11.1582 11.6882 11.2317C10.8181 11.4816 7.51116 11.4992 7.05288 11.5H6.94637C6.71461 11.5 5.75605 11.4952 4.75098 11.4584L4.62348 11.4536L4.55822 11.4504L4.42996 11.4447L4.30171 11.4391C3.46915 11.3999 2.67635 11.3366 2.31108 11.2309C2.05643 11.1574 1.82435 11.0138 1.63808 10.8143C1.45182 10.6149 1.3179 10.3667 1.24977 10.0946C1.16651 9.7607 1.11101 9.30505 1.0735 8.84701L1.0675 8.76292L1.0615 8.67964C1.02449 8.13703 1.00398 7.59328 1 7.04925L1 6.95075C1.0015 6.77858 1.0075 6.1836 1.048 5.52696L1.05325 5.44448L1.0555 5.40284L1.0615 5.31956L1.078 5.11135L1.08551 5.02807C1.12151 4.61247 1.17476 4.20887 1.25052 3.90537C1.31854 3.63315 1.4524 3.38479 1.63867 3.18522C1.82495 2.98564 2.05709 2.84185 2.31183 2.76826C2.6771 2.66416 3.4699 2.6001 4.30246 2.56006L4.42996 2.55445L4.55897 2.54965L4.62348 2.54725L4.75173 2.54164C5.46556 2.51712 6.17968 2.5035 6.89387 2.5008L7.03863 2.5ZM6 5.02807V9L9 7.03003L6 5.02807Z" fill="currentColor"></path></svg></div>YouTube</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://fosstodon.org/@deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="-10 -5 1034 1034" xmlns="http://www.w3.org/2000/svg" version="1.1"><path fill="currentColor" d="M499 112q-93 1 -166 11q-81 11 -128 33l-14 8q-16 10 -32 25q-22 21 -38 47q-21 33 -32 73q-14 47 -14 103v37q0 77 1 119q3 113 18 188q19 95 62 154q50 67 134 89q109 29 210 24q46 -3 88 -12q30 -7 55 -17l19 -8l-4 -75l-22 6q-28 6 -57 10q-41 6 -78 4q-53 -1 -80 -7
q-43 -8 -67 -30q-29 -25 -35 -72q-2 -14 -2 -29l25 6q31 6 65 10q48 7 93 9q42 2 92 -2q32 -2 88 -9t107 -30q49 -23 81.5 -54.5t38.5 -63.5q9 -45 13 -109q4 -46 5 -97v-41q0 -56 -14 -103q-11 -40 -32 -73q-16 -26 -38 -47q-15 -15 -32 -25q-12 -8 -14 -8
q-46 -22 -127 -33q-74 -10 -166 -11h-3zM367 267q73 0 109 56l24 39l24 -39q36 -56 109 -56q63 0 101 43t38 117v239h-95v-232q0 -74 -61 -74q-69 0 -69 88v127h-94v-127q0 -88 -69 -88q-61 0 -61 74v232h-95v-239q0 -74 38 -117t101 -43z"></path></svg></div>Mastodon</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><div class="dark:border-gray-800"><a class="block w-full px-1 my-3 lg:w-auto lg:m-0 lg:px-2 lg:rounded-md lg:hover:bg-azure3 lg:hover:dark:bg-gray-800 " href="/blog" aria-current="false">Blog</a></div></div><div class="flex flex-col lg:flex-row grow gap-4 lg:items-center lg:justify-end"><!--frsh:island:OramaSearch:6:--><div class="md:relative w-full hidden lg:block max-w-64"><div class="relative"><input type="search" placeholder="Search" id="orama-search-input"   value=""   class="w-full min-w-24 rounded-lg placeholder:text-sm text-base leading-normal p-1 pl-8 border transition-all duration-150
          border-gray-600 dark:border-gray-100 focus:outline-offset-1 disabled:opacity-50"><svg class="absolute top-2 left-2 size-4.5 text-gray-600 pointer-events-none " width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_2006_89)"><path d="M13.3008 12.1951L10.3537 9.24807C11.0632 8.30353 11.4463 7.15382 11.445 5.97248C11.445 2.95497 8.98999 0.5 5.97248 0.5C2.95497 0.5 0.5 2.95497 0.5 5.97248C0.5 8.98999 2.95497 11.445 5.97248 11.445C7.15382 11.4463 8.30353 11.0632 9.24807 10.3537L12.1951 13.3008C12.3443 13.4341 12.5389 13.5053 12.7389 13.4997C12.9389 13.4941 13.1292 13.4121 13.2707 13.2707C13.4121 13.1292 13.4941 12.9389 13.4997 12.7389C13.5053 12.5389 13.4341 12.3443 13.3008 12.1951ZM2.06357 5.97248C2.06357 5.19937 2.29282 4.44362 2.72234 3.8008C3.15185 3.15799 3.76234 2.65697 4.4766 2.36111C5.19086 2.06526 5.97682 1.98785 6.73507 2.13867C7.49333 2.2895 8.18983 2.66179 8.7365 3.20846C9.28317 3.75513 9.65546 4.45163 9.80629 5.20989C9.95711 5.96814 9.8797 6.7541 9.58385 7.46836C9.28799 8.18262 8.78697 8.79311 8.14416 9.22262C7.50134 9.65214 6.74559 9.88139 5.97248 9.88139C4.93615 9.88015 3.94263 9.46792 3.20983 8.73513C2.47704 8.00233 2.06481 7.00881 2.06357 5.97248Z" fill="currentColor"></path></g><defs><clipPath id="clip0_2006_89"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg><kbd id="search-key" class="hidden xs:flex pointer-events-none absolute font-sans rounded-sm top-1 right-1 bottom-1 w-auto border-1 border-gray-500 dark:border-gray-800 border-b-2 border-r-2 bg-white text-gray-600 dark:text-gray-100 text-center text-xs font-bold p-2 items-center justify-center dark:bg-gray-800">⌘K</kbd><div id="orama-search-loading" class="absolute left-2 top-1/2 transform -translate-y-1/2 hidden bg-white dark:bg-black"><div class="animate-spin rounded-full h-4 w-4 border-2 border-transparent border-r-black dark:border-r-white bg-white dark:bg-black"></div></div><div class="sr-only" aria-live="polite" id="orama-results-announcer" ></div></div><div id="orama-search-results" class="absolute inset-2 left-2 right-2 h-[calc(100vh-10rem)] lg:h-[calc(100vh-8rem)] top-32 lg:top-10 md:top-full md:left-auto md:right-0 mt-2 bg-white dark:bg-black dark:text-white border border-gray-500 dark:border-gray-800 rounded-xl shadow-2xl z-50 md:max-h-128 overflow-hidden hidden md:min-w-160 max-w-2xl"><div id="orama-search-results-content" class="overflow-y-auto h-full"></div><div class="border-t border-gray-500 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2 sticky bottom-0"><div class="flex items-center gap-6 text-xs text-gray-500 dark:text-gray-500"><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↑↓</span><span class="sr-only">Up or down to</span></kbd>navigate</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↵</span><span class="sr-only">Enter to</span></kbd>select</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">ESC</span><span class="sr-only">Escape to</span></kbd>close</span><img src="/images/orama-dark.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="dark:hidden h-4 w-auto ml-auto"/><img src="/images/orama-light.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="hidden dark:inline h-4 w-auto ml-auto"/></div></div></div></div><!--/frsh:island--></div></div></nav></div></header><main tabindex="-1" id="main"><div class="w-full overflow-x-hidden relative flex justify-between h-full flex-col flex-wrap"><div><div class="text-center px-8 py-32 z-[3] dark:text-white mx-auto max-w-lg"><h1 class="font-bold text-5xl leading-10 tracking-tight">404</h1><h2 class="mt-4 sm:mt-5 text-2xl text-center leading-tight">Couldn&#39;t find what you&#39;re looking for.</h2><p class="mt-4"><a href="/" class="underline" data-ancestor="true" aria-current="true">Home</a> | <a href="https://docs.deno.com" class="underline">Docs</a></p></div></div><div><div class="mt-auto w-full pointer-events-none h-[200px] relative overflow-hidden"><img src="/images/ferris.gif?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Ferris" class="translate-y-[22px] w-[100px] absolute left-[60%] bottom-0 dark:filter dark:invert"/><img src="/images/deno404.gif?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Deno" class="w-[200px] relative top-[24px] animate-move dark:filter dark:invert"/></div></div></div></main><footer class="border-t text-runtime text-sm flex justify-center bg-offblack border-gray-800"><div class="section-x-inset-xl py-8 lg:pt-16 w-full"><div class="flex flex-col items-start gap-16 md:flex-row md:gap-8 md:w-full md:justify-between max-w-4xl"><a href="/" class="flex items-center" data-ancestor="true" aria-current="true"><img src="/logos/dark-transparent.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" class="h-10 w-auto max-w-none block" alt="Deno logo"/></a><div><h2 class="leading-tight font-bold" id="FooterOpen-source">Open-source</h2><ul aria-labelledby="FooterOpen-source" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/" class="block hover:underline" data-ancestor="true" aria-current="true">Deno</a></li><li><a href="https://fresh.deno.dev/" class="block hover:underline">Fresh</a></li><li><a href="https://jsr.io/" class="block hover:underline">JSR</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterProducts">Products</h2><ul aria-labelledby="FooterProducts" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/deploy" class="block hover:underline">Deno Deploy</a></li><li><a href="/deploy/sandbox" class="block hover:underline">Deno Sandbox</a></li><li><a href="/deploy/subhosting" class="block hover:underline">Subhosting</a></li><li><a href="/deploy/pricing" class="block hover:underline">Pricing</a></li><li><a href="https://docs.deno.com/deploy/" class="block hover:underline">Deno Deploy Docs</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterDeno Resources">Deno Resources</h2><ul aria-labelledby="FooterDeno Resources" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/manual" class="block hover:underline">Deno Docs</a></li><li><a href="https://docs.deno.com/api/deno/" class="block hover:underline">Deno API Reference</a></li><li><a href="https://jsr.io/@std" class="block hover:underline">Standard Library</a></li><li><a href="https://jsr.io/" class="block hover:underline">Third-Party Modules</a></li><li><a href="https://examples.deno.land" class="block hover:underline">Deno Examples</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterCompany">Company</h2><ul aria-labelledby="FooterCompany" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/blog" class="block hover:underline">Blog</a></li><li><a href="https://deno.news" class="block hover:underline">News</a></li><li><a href="/brand" class="block hover:underline">Branding</a></li><li><a href="https://docs.deno.com/deploy/privacy_policy/" class="block hover:underline">Privacy Policy</a></li><li><a href="https://docs.deno.com/deploy/terms_and_conditions/" class="block hover:underline">Terms and Conditions</a></li><li><a href="mailto:abuse@deno.com" class="block hover:underline" target="_blank">Report abuse</a></li></ul></div></div><div class="w-full text-center flex flex-col md:flex-row flex-wrap justify-between items-center gap-4 pt-8 mt-8 border-t border-gray-800"><div class="flex items-center justify-center gap-3 text-gray-300"><a href="https://github.com/denoland"><span class="sr-only">GitHub</span><svg class="size-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="GitHub Logo"><g clip-path="url(#clip0_1989_191)"><path d="M7.00001 0C3.13391 0 0 3.21295 0 7.17755C0 10.3482 2.0055 13.0388 4.7873 13.9875C5.1373 14.0534 5.26471 13.832 5.26471 13.6414C5.26471 13.4716 5.25912 13.0195 5.25561 12.4212C3.3082 12.8547 2.8973 11.4589 2.8973 11.4589C2.5795 10.6291 2.1203 10.4084 2.1203 10.4084C1.48471 9.96418 2.16861 9.97279 2.16861 9.97279C2.87071 10.0229 3.24032 10.7122 3.24032 10.7122C3.86472 11.8085 4.87903 11.4918 5.27732 11.3084C5.34171 10.8448 5.52232 10.5288 5.72251 10.3497C4.16851 10.1684 2.534 9.55218 2.534 6.80211C2.534 6.01893 2.807 5.37764 3.2543 4.87605C3.1822 4.69476 2.94211 3.96463 3.32289 2.97722C3.32289 2.97722 3.91089 2.78376 5.24789 3.71238C5.77305 3.55992 6.37629 3.47184 6.99948 3.4709C7.59448 3.47377 8.19351 3.5533 8.7528 3.71238C10.0891 2.78376 10.6757 2.97649 10.6757 2.97649C11.0579 3.9646 10.8171 4.69475 10.7457 4.87603C11.1937 5.3776 11.4653 6.0189 11.4653 6.80208C11.4653 9.55931 9.82799 10.1662 8.26908 10.3439C8.52037 10.5653 8.74368 11.0031 8.74368 11.6731C8.74368 12.6318 8.73529 13.4064 8.73529 13.6414C8.73529 13.8335 8.86129 14.057 9.21689 13.9868C12.0205 13.0032 14 10.3285 14 7.18046C14 7.17943 14 7.17841 14 7.17738C14 3.21278 10.8654 0 7.00001 0Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1989_191"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></a><a href="https://discord.gg/deno"><span class="sr-only">Discord</span><svg class="size-7" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="Discord Logo"><g clip-path="url(#clip0_1993_195)"><path d="M11.8595 2.62936C10.9397 2.21264 9.96862 1.917 8.97117 1.75C8.83467 1.99146 8.71117 2.23988 8.60118 2.49424C7.5387 2.33581 6.45822 2.33581 5.39574 2.49424C5.28569 2.23991 5.16219 1.99149 5.02575 1.75C4.02766 1.91841 3.05598 2.21475 2.13524 2.63154C0.307332 5.30775 -0.188185 7.9175 0.0595735 10.4902C1.13004 11.2729 2.3282 11.8681 3.60197 12.25C3.88878 11.8683 4.14258 11.4633 4.36066 11.0394C3.94644 10.8863 3.54665 10.6974 3.16591 10.4749C3.26612 10.403 3.36412 10.3289 3.45882 10.257C4.56668 10.7726 5.77586 11.0399 7.00011 11.0399C8.22437 11.0399 9.43354 10.7726 10.5414 10.257C10.6372 10.3344 10.7352 10.4085 10.8343 10.4749C10.4528 10.6978 10.0523 10.887 9.63736 11.0405C9.85518 11.4642 10.109 11.8688 10.3961 12.25C11.6709 11.8696 12.87 11.2747 13.9406 10.4913C14.2314 7.50778 13.444 4.92201 11.8595 2.62936ZM4.67449 8.908C3.98407 8.908 3.41367 8.28798 3.41367 7.52522C3.41367 6.76245 3.96425 6.13699 4.67228 6.13699C5.38032 6.13699 5.94631 6.76245 5.9342 7.52522C5.92209 8.28798 5.37812 8.908 4.67449 8.908ZM9.32574 8.908C8.63422 8.908 8.06602 8.28798 8.06602 7.52522C8.06602 6.76245 8.6166 6.13699 9.32574 6.13699C10.0349 6.13699 10.5965 6.76245 10.5843 7.52522C10.5722 8.28798 10.0294 8.908 9.32574 8.908Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1993_195"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></a><a href="https://bsky.app/profile/deno.land"><span class="sr-only">Bluesky</span><svg class="size-6" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="BlueSky Logo"><path d="M12 10.8c-1.087-2.114-4.046-6.053-6.798-7.995C2.566.944 1.561 1.266.902 1.565.139 1.908 0 3.08 0 3.768c0 .69.378 5.65.624 6.479.815 2.736 3.713 3.66 6.383 3.364.136-.02.275-.039.415-.056-.138.022-.276.04-.415.056-3.912.58-7.387 2.005-2.83 7.078 5.013 5.19 6.87-1.113 7.823-4.308.953 3.195 2.05 9.271 7.733 4.308 4.267-4.308 1.172-6.498-2.74-7.078a8.741 8.741 0 0 1-.415-.056c.14.017.279.036.415.056 2.67.297 5.568-.628 6.383-3.364.246-.828.624-5.79.624-6.478 0-.69-.139-1.861-.902-2.206-.659-.298-1.664-.62-4.3 1.24C16.046 4.748 13.087 8.687 12 10.8Z" fill="currentColor"></path></svg></a><a href="https://fosstodon.org/@deno_land"><span class="sr-only">Mastodon</span><svg class="size-6" width="14" height="14" viewBox="-10 -5 1034 1034" xmlns="http://www.w3.org/2000/svg" version="1.1"><path fill="currentColor" d="M499 112q-93 1 -166 11q-81 11 -128 33l-14 8q-16 10 -32 25q-22 21 -38 47q-21 33 -32 73q-14 47 -14 103v37q0 77 1 119q3 113 18 188q19 95 62 154q50 67 134 89q109 29 210 24q46 -3 88 -12q30 -7 55 -17l19 -8l-4 -75l-22 6q-28 6 -57 10q-41 6 -78 4q-53 -1 -80 -7
q-43 -8 -67 -30q-29 -25 -35 -72q-2 -14 -2 -29l25 6q31 6 65 10q48 7 93 9q42 2 92 -2q32 -2 88 -9t107 -30q49 -23 81.5 -54.5t38.5 -63.5q9 -45 13 -109q4 -46 5 -97v-41q0 -56 -14 -103q-11 -40 -32 -73q-16 -26 -38 -47q-15 -15 -32 -25q-12 -8 -14 -8
q-46 -22 -127 -33q-74 -10 -166 -11h-3zM367 267q73 0 109 56l24 39l24 -39q36 -56 109 -56q63 0 101 43t38 117v239h-95v-232q0 -74 -61 -74q-69 0 -69 88v127h-94v-127q0 -88 -69 -88q-61 0 -61 74v232h-95v-239q0 -74 38 -117t101 -43z"></path></svg></a><a href="https://twitter.com/deno_land"><span class="sr-only">Twitter or X or whatever</span><svg width="300" height="300.251" class="size-5" viewBox="0 0 300 300.251" version="1.1" xmlns="http://www.w3.org/2000/svg" aria-label="X logo" fill="currentColor"><path d="M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"></path></svg></a><a href="https://youtube.com/@deno_land"><span class="sr-only">YouTube</span><svg class="size-7" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="YouTube Logo"><path d="M7.03863 2.5H7.10538C7.72192 2.5024 10.8459 2.52643 11.6882 2.76826C11.9428 2.84207 12.1748 2.98593 12.3611 3.18548C12.5473 3.38502 12.6813 3.63326 12.7495 3.90537C12.8252 4.20967 12.8785 4.61247 12.9145 5.02807L12.922 5.11135L12.9385 5.31956L12.9445 5.40284C12.9932 6.13475 12.9992 6.82022 13 6.96997V7.03003C12.9992 7.18538 12.9925 7.9173 12.9385 8.67964L12.9325 8.76372L12.9257 8.84701C12.8882 9.30505 12.8327 9.7599 12.7495 10.0946C12.6815 10.3669 12.5476 10.6152 12.3613 10.8148C12.1751 11.0144 11.9429 11.1582 11.6882 11.2317C10.8181 11.4816 7.51116 11.4992 7.05288 11.5H6.94637C6.71461 11.5 5.75605 11.4952 4.75098 11.4584L4.62348 11.4536L4.55822 11.4504L4.42996 11.4447L4.30171 11.4391C3.46915 11.3999 2.67635 11.3366 2.31108 11.2309C2.05643 11.1574 1.82435 11.0138 1.63808 10.8143C1.45182 10.6149 1.3179 10.3667 1.24977 10.0946C1.16651 9.7607 1.11101 9.30505 1.0735 8.84701L1.0675 8.76292L1.0615 8.67964C1.02449 8.13703 1.00398 7.59328 1 7.04925L1 6.95075C1.0015 6.77858 1.0075 6.1836 1.048 5.52696L1.05325 5.44448L1.0555 5.40284L1.0615 5.31956L1.078 5.11135L1.08551 5.02807C1.12151 4.61247 1.17476 4.20887 1.25052 3.90537C1.31854 3.63315 1.4524 3.38479 1.63867 3.18522C1.82495 2.98564 2.05709 2.84185 2.31183 2.76826C2.6771 2.66416 3.4699 2.6001 4.30246 2.56006L4.42996 2.55445L4.55897 2.54965L4.62348 2.54725L4.75173 2.54164C5.46556 2.51712 6.17968 2.5035 6.89387 2.5008L7.03863 2.5ZM6 5.02807V9L9 7.03003L6 5.02807Z" fill="currentColor"></path></svg></a></div><span class="block text-xs text-gray-500 leading-tight">Copyright © 2026 Deno Land Inc. <span class="whitespace-nowrap">All rights reserved.</span>  | <!--frsh:island:CookieOptOut:7:--><!--/frsh:island--></span><a href="https://denostatus.com/" class="transition-colors shadow-md px-4 py-2 rounded-lg font-bold inline-flex items-center gap-2 text-base bg-gray-800 border border-gray-800 hover:border-runtime text-gray-50"><div class="w-3 h-3 rounded-full bg-runtime"></div><span>All systems operational</span></a></div></div></footer></div></div><script type="module" nonce="3957dec3944845f7bdb91ba14cd6c81e">import { boot } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/fresh-runtime.js";import { SkipToMainLink } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/SkipToMainLink.js";import { HelloBar } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/HelloBar.js";import { OramaSearch } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/OramaSearch.js";import { NavItemWithChildren } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/NavItemWithChildren.js";import { CookieOptOut } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/AnalyticsOptOut.js";boot({SkipToMainLink,HelloBar,OramaSearch,NavItemWithChildren,CookieOptOut},"[[1,4,11,19,62,75,100,105],{\"slots\":2,\"props\":3},[],{},{\"slots\":5,\"props\":6},[],{\"id\":7,\"href\":8,\"ctaText\":9,\"text\":10},\"clawpatrol-2026-05\",\"https://deno.com/blog/clawpatrol\",\"Read the post\",\"Introducing Claw Patrol: an open-source security firewall for agents\",{\"slots\":12,\"props\":13},[],{\"projectId\":14,\"apiKey\":15,\"dataSourceIDs\":16,\"class\":18},\"af2a2800-bfac-4b67-a58e-51ada826e1c0\",\"c1_SHxojDzeso5ITKAGMDjvgpZk43Ff0pFiQhmdGfiLBtnUWKKJFnWGSNs_Zb7\",[17],\"ad8a33af-4669-444e-9bed-b0716b0a9e63\",\"lg:hidden\",{\"slots\":20,\"props\":21},[],{\"entry\":22,\"path\":60,\"activeNav\":61},{\"content\":23,\"children\":24},\"Products\",[25,30,34,38,42,47,52,56],{\"href\":26,\"content\":27,\"subhead\":28,\"category\":29},\"/\",\"Deno\",\"Modern runtime for JavaScript and TypeScript\",\"Open Source\",{\"href\":31,\"content\":32,\"subhead\":33,\"category\":29},\"https://fresh.deno.dev\",\"Fresh\",\"Web framework designed for the edge\",{\"href\":35,\"content\":36,\"subhead\":37,\"category\":29},\"https://clawpatrol.dev\",\"Claw Patrol\",\"Open-source security firewall for agents\",{\"href\":39,\"content\":40,\"subhead\":41,\"category\":29},\"https://jsr.io\",\"JSR\",\"TypeScript-first ESM package registry\",{\"href\":43,\"content\":44,\"subhead\":45,\"category\":46},\"/deploy\",\"Deno Deploy\",\"Easy serverless hosting for your JavaScript projects\",\"Commercial\",{\"href\":48,\"content\":49,\"subhead\":50,\"category\":46,\"nested\":51},\"/deploy/sandbox\",\"Deno Sandbox\",\"Run untrusted code in secure Linux VMs. Built for AI agents.\",true,{\"href\":53,\"content\":54,\"subhead\":55,\"category\":46,\"nested\":51},\"/subhosting\",\"Subhosting\",\"Extend your platform using Deno Deploy's secure infrastructure\",{\"href\":57,\"content\":58,\"subhead\":59,\"category\":46},\"/enterprise\",\"Deno for Enterprise\",\"Enterprise support for runtime projects\",\"/llms.txt\",[\"Signal\",-2],{\"slots\":63,\"props\":64},[],{\"entry\":65,\"path\":60,\"activeNav\":61},{\"content\":66,\"children\":67},\"Modules\",[68,71,72],{\"href\":69,\"content\":70},\"https://jsr.io/@std\",\"Standard Library\",{\"href\":39,\"content\":40},{\"href\":73,\"content\":74},\"/npm/\",\"Node.js & npm\",{\"slots\":76,\"props\":77},[],{\"entry\":78,\"path\":60,\"activeNav\":61},{\"content\":79,\"children\":80},\"Community\",[81,84,87,91,94,97],{\"href\":82,\"content\":83,\"icon\":83},\"https://discord.gg/deno\",\"Discord\",{\"href\":85,\"content\":86,\"icon\":86},\"https://github.com/denoland\",\"GitHub\",{\"href\":88,\"content\":89,\"icon\":90},\"https://bsky.app/profile/deno.land\",\"Bluesky\",\"BlueSky\",{\"href\":92,\"content\":93,\"icon\":93},\"https://twitter.com/deno_land\",\"X\",{\"href\":95,\"content\":96,\"icon\":96},\"https://www.youtube.com/c/deno_land\",\"YouTube\",{\"href\":98,\"content\":99,\"icon\":99},\"https://fosstodon.org/@deno_land\",\"Mastodon\",{\"slots\":101,\"props\":102},[],{\"projectId\":14,\"apiKey\":15,\"dataSourceIDs\":103,\"class\":104},[17],\"hidden lg:block max-w-64\",{\"slots\":106,\"props\":107},[],{}]");</script></body></html>

## eye {

            transform-origin: 50% 30%;
            animation: blink 0.15s cubic-bezier(0.5, 0, 0.5, 1);
          }
        }
        </style></svg></a><label tabindex="0" class="cursor-pointer lg:hidden touch-manipulation" for="menuToggle" onkeydown="if (event.code === &#39;Space&#39; || event.code === &#39;Enter&#39;) { this.click(); event.preventDefault(); }"><svg  width="21" height="14" viewBox="0 0 21 14" fill="none" xmlns="http://www.w3.org/2000/svg"><line x1="0.25" y1="1.4" x2="20.25" y2="1.4" stroke="currentColor" stroke-width="1.2"></line><line x1="0.25" y1="7.4" x2="20.25" y2="7.4" stroke="currentColor" stroke-width="1.2"></line><line x1="0.25" y1="13.4" x2="10.25" y2="13.4" stroke="currentColor" stroke-width="1.2"></line></svg><svg class="h-6 w-6 hidden" stroke="currentColor" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg></label></div><div class="hidden flex-col w-full mt-5 gap-x-8 gap-y-4 lg:flex lg:flex-row lg:items-center lg:mx-0 lg:mt-0" data-component="nav-entries"><!--frsh:island:OramaSearch:2:--><div class="md:relative w-full lg:hidden"><div class="relative"><input type="search" placeholder="Search" id="orama-search-input"   value=""   class="w-full min-w-24 rounded-lg placeholder:text-sm text-base leading-normal p-1 pl-8 border transition-all duration-150
          border-gray-600 dark:border-gray-100 focus:outline-offset-1 disabled:opacity-50"><svg class="absolute top-2 left-2 size-4.5 text-gray-600 pointer-events-none " width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_2006_89)"><path d="M13.3008 12.1951L10.3537 9.24807C11.0632 8.30353 11.4463 7.15382 11.445 5.97248C11.445 2.95497 8.98999 0.5 5.97248 0.5C2.95497 0.5 0.5 2.95497 0.5 5.97248C0.5 8.98999 2.95497 11.445 5.97248 11.445C7.15382 11.4463 8.30353 11.0632 9.24807 10.3537L12.1951 13.3008C12.3443 13.4341 12.5389 13.5053 12.7389 13.4997C12.9389 13.4941 13.1292 13.4121 13.2707 13.2707C13.4121 13.1292 13.4941 12.9389 13.4997 12.7389C13.5053 12.5389 13.4341 12.3443 13.3008 12.1951ZM2.06357 5.97248C2.06357 5.19937 2.29282 4.44362 2.72234 3.8008C3.15185 3.15799 3.76234 2.65697 4.4766 2.36111C5.19086 2.06526 5.97682 1.98785 6.73507 2.13867C7.49333 2.2895 8.18983 2.66179 8.7365 3.20846C9.28317 3.75513 9.65546 4.45163 9.80629 5.20989C9.95711 5.96814 9.8797 6.7541 9.58385 7.46836C9.28799 8.18262 8.78697 8.79311 8.14416 9.22262C7.50134 9.65214 6.74559 9.88139 5.97248 9.88139C4.93615 9.88015 3.94263 9.46792 3.20983 8.73513C2.47704 8.00233 2.06481 7.00881 2.06357 5.97248Z" fill="currentColor"></path></g><defs><clipPath id="clip0_2006_89"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg><kbd id="search-key" class="hidden xs:flex pointer-events-none absolute font-sans rounded-sm top-1 right-1 bottom-1 w-auto border-1 border-gray-500 dark:border-gray-800 border-b-2 border-r-2 bg-white text-gray-600 dark:text-gray-100 text-center text-xs font-bold p-2 items-center justify-center dark:bg-gray-800">⌘K</kbd><div id="orama-search-loading" class="absolute left-2 top-1/2 transform -translate-y-1/2 hidden bg-white dark:bg-black"><div class="animate-spin rounded-full h-4 w-4 border-2 border-transparent border-r-black dark:border-r-white bg-white dark:bg-black"></div></div><div class="sr-only" aria-live="polite" id="orama-results-announcer" ></div></div><div id="orama-search-results" class="absolute inset-2 left-2 right-2 h-[calc(100vh-10rem)] lg:h-[calc(100vh-8rem)] top-32 lg:top-10 md:top-full md:left-auto md:right-0 mt-2 bg-white dark:bg-black dark:text-white border border-gray-500 dark:border-gray-800 rounded-xl shadow-2xl z-50 md:max-h-128 overflow-hidden hidden md:min-w-160 max-w-2xl"><div id="orama-search-results-content" class="overflow-y-auto h-full"></div><div class="border-t border-gray-500 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2 sticky bottom-0"><div class="flex items-center gap-6 text-xs text-gray-500 dark:text-gray-500"><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↑↓</span><span class="sr-only">Up or down to</span></kbd>navigate</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↵</span><span class="sr-only">Enter to</span></kbd>select</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">ESC</span><span class="sr-only">Escape to</span></kbd>close</span><img src="/images/orama-dark.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="dark:hidden h-4 w-auto ml-auto"/><img src="/images/orama-light.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="hidden dark:inline h-4 w-auto ml-auto"/></div></div></div></div><!--/frsh:island--><div class="flex flex-col leading-loose divide-incl-y lg:flex-row lg:gap-x-4 lg:select-none lg:divide-incl-y-0"><style>.entry-children li:nth-child(1) a{ transition-delay: calc(var(--delay-increment) * 1); }.entry-children li:nth-child(2) a{ transition-delay: calc(var(--delay-increment) * 2); }.entry-children li:nth-child(3) a{ transition-delay: calc(var(--delay-increment) * 3); }.entry-children li:nth-child(4) a{ transition-delay: calc(var(--delay-increment) * 4); }.entry-children li:nth-child(5) a{ transition-delay: calc(var(--delay-increment) * 5); }.entry-children li:nth-child(6) a{ transition-delay: calc(var(--delay-increment) * 6); }.entry-children li:nth-child(7) a{ transition-delay: calc(var(--delay-increment) * 7); }.entry-children li:nth-child(8) a{ transition-delay: calc(var(--delay-increment) * 8); }.entry-children li:nth-child(9) a{ transition-delay: calc(var(--delay-increment) * 9); }</style><!--frsh:island:NavItemWithChildren:3:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Products"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Products</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="lg:grid lg:grid-cols-2
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><div id="category-Open Source" class="dark:text-runtime text-[0.625rem] uppercase font-bold lg:mx-5 lg:mt-3.5">Open Source</div><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5" aria-labelledby="category-Open Source"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="true" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="/" data-ancestor="true"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Modern runtime for JavaScript and TypeScript</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://fresh.deno.dev"><div class="w-full"><div class="w-max flex gap-4 items-center">Fresh</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Web framework designed for the edge</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://clawpatrol.dev"><div class="w-full"><div class="w-max flex gap-4 items-center">Claw Patrol</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Open-source security firewall for agents</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pr-5 lg:px-5" href="https://jsr.io"><div class="w-full"><div class="w-max flex gap-4 items-center">JSR</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">TypeScript-first ESM package registry</div></div></a></li></ul></div><div data-component="entry-children" class="lg:hidden
      lg:border-l lg:border-azure2 lg:dark:border-gray-800
      entry-children w-full flex flex-col"><div id="category-Commercial" class="dark:text-deploy text-[0.625rem] uppercase font-bold lg:mx-5 lg:mt-3.5">Commercial</div><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-0" aria-labelledby="category-Commercial"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 nav-product-tree-parent"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pl-5 lg:px-5" href="/deploy"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno Deploy</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Easy serverless hosting for your JavaScript projects</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 ml-1.5 lg:ml-5 nav-product-tree-nested nav-product-tree-nested-first"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 pl-3.5 lg:px-4" href="/deploy/sandbox"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno Sandbox</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Run untrusted code in secure Linux VMs. Built for AI agents.</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900 ml-1.5 lg:ml-5 nav-product-tree-nested nav-product-tree-nested-last"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 pl-3.5 lg:px-4" href="/subhosting"><div class="w-full"><div class="w-max flex gap-4 items-center">Subhosting</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Extend your platform using Deno Deploy's secure infrastructure</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:pl-5 lg:px-5" href="/enterprise"><div class="w-full"><div class="w-max flex gap-4 items-center">Deno for Enterprise</div><div class="text-xs w-max max-w-full mt-1 font-normal text-gray-600 dark:text-gray-500">Enterprise support for runtime projects</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><div class="dark:border-gray-800"><a class="block w-full px-1 my-3 lg:w-auto lg:m-0 lg:px-2 lg:rounded-md lg:hover:bg-azure3 lg:hover:dark:bg-gray-800 " href="https://docs.deno.com" aria-current="false">Docs</a></div><!--frsh:island:NavItemWithChildren:4:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Modules"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Modules</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://jsr.io/@std"><div class="w-full"><div class="w-max flex gap-4 items-center">Standard Library</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://jsr.io"><div class="w-full"><div class="w-max flex gap-4 items-center">JSR</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="/npm/"><div class="w-full"><div class="w-max flex gap-4 items-center">Node.js &amp; npm</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><!--frsh:island:NavItemWithChildren:5:--><div class="z-10 dark:border-gray-800 grid grid-cols-1 transition-all duration-300 lg:block" style="grid-template-rows:max-content 0fr;" ><button type="button"  id="Community"  class="rounded-md flex items-center justify-between px-1 my-3 lg:px-2 lg:my-0 hover:bg-azure3 dark:hover:bg-gray-800 touch-manipulation" ><span>Community</span><div><svg class="transition-transform ml-2 duration-200 " width="14" height="14" viewBox="0 0 14 14" version="1.1" xmlns="http://www.w3.org/2000/svg" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;fill:currentColor;"><g transform="matrix(6.12323e-17,-1,1,6.12323e-17,0,14)"><path d="M10.03,1.47C10.323,1.763 10.323,2.237 10.03,2.53L5.561,7L10.03,11.47C10.323,11.763 10.323,12.237 10.03,12.53C9.737,12.823 9.263,12.823 8.97,12.53L3.97,7.53C3.677,7.237 3.677,6.763 3.97,6.47L8.97,1.47C9.263,1.177 9.737,1.177 10.03,1.47Z"></path></g></svg></div></button><div class="lg:absolute z-10 lg:top-16 lg:pt-[5px] overflow-hidden lg:overflow-visible"><div class="
          
          lg:bg-neutral-50 lg:dark:bg-gray-900 lg:rounded-md lg:overflow-hidden lg:shadow-lg"><div data-component="entry-children" class="lg:hidden
      
      entry-children w-full flex flex-col"><ul class="pb-2 pl-2 mb-3 lg:pl-0 lg:py-0 lg:-mt-px lg:mb-0 lg:space-y-0 space-y-1.5"><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://discord.gg/deno"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="Discord Logo"><g clip-path="url(#clip0_1993_195)"><path d="M11.8595 2.62936C10.9397 2.21264 9.96862 1.917 8.97117 1.75C8.83467 1.99146 8.71117 2.23988 8.60118 2.49424C7.5387 2.33581 6.45822 2.33581 5.39574 2.49424C5.28569 2.23991 5.16219 1.99149 5.02575 1.75C4.02766 1.91841 3.05598 2.21475 2.13524 2.63154C0.307332 5.30775 -0.188185 7.9175 0.0595735 10.4902C1.13004 11.2729 2.3282 11.8681 3.60197 12.25C3.88878 11.8683 4.14258 11.4633 4.36066 11.0394C3.94644 10.8863 3.54665 10.6974 3.16591 10.4749C3.26612 10.403 3.36412 10.3289 3.45882 10.257C4.56668 10.7726 5.77586 11.0399 7.00011 11.0399C8.22437 11.0399 9.43354 10.7726 10.5414 10.257C10.6372 10.3344 10.7352 10.4085 10.8343 10.4749C10.4528 10.6978 10.0523 10.887 9.63736 11.0405C9.85518 11.4642 10.109 11.8688 10.3961 12.25C11.6709 11.8696 12.87 11.2747 13.9406 10.4913C14.2314 7.50778 13.444 4.92201 11.8595 2.62936ZM4.67449 8.908C3.98407 8.908 3.41367 8.28798 3.41367 7.52522C3.41367 6.76245 3.96425 6.13699 4.67228 6.13699C5.38032 6.13699 5.94631 6.76245 5.9342 7.52522C5.92209 8.28798 5.37812 8.908 4.67449 8.908ZM9.32574 8.908C8.63422 8.908 8.06602 8.28798 8.06602 7.52522C8.06602 6.76245 8.6166 6.13699 9.32574 6.13699C10.0349 6.13699 10.5965 6.76245 10.5843 7.52522C10.5722 8.28798 10.0294 8.908 9.32574 8.908Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1993_195"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></div>Discord</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://github.com/denoland"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-5 h-5" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="GitHub Logo"><g clip-path="url(#clip0_1989_191)"><path d="M7.00001 0C3.13391 0 0 3.21295 0 7.17755C0 10.3482 2.0055 13.0388 4.7873 13.9875C5.1373 14.0534 5.26471 13.832 5.26471 13.6414C5.26471 13.4716 5.25912 13.0195 5.25561 12.4212C3.3082 12.8547 2.8973 11.4589 2.8973 11.4589C2.5795 10.6291 2.1203 10.4084 2.1203 10.4084C1.48471 9.96418 2.16861 9.97279 2.16861 9.97279C2.87071 10.0229 3.24032 10.7122 3.24032 10.7122C3.86472 11.8085 4.87903 11.4918 5.27732 11.3084C5.34171 10.8448 5.52232 10.5288 5.72251 10.3497C4.16851 10.1684 2.534 9.55218 2.534 6.80211C2.534 6.01893 2.807 5.37764 3.2543 4.87605C3.1822 4.69476 2.94211 3.96463 3.32289 2.97722C3.32289 2.97722 3.91089 2.78376 5.24789 3.71238C5.77305 3.55992 6.37629 3.47184 6.99948 3.4709C7.59448 3.47377 8.19351 3.5533 8.7528 3.71238C10.0891 2.78376 10.6757 2.97649 10.6757 2.97649C11.0579 3.9646 10.8171 4.69475 10.7457 4.87603C11.1937 5.3776 11.4653 6.0189 11.4653 6.80208C11.4653 9.55931 9.82799 10.1662 8.26908 10.3439C8.52037 10.5653 8.74368 11.0031 8.74368 11.6731C8.74368 12.6318 8.73529 13.4064 8.73529 13.6414C8.73529 13.8335 8.86129 14.057 9.21689 13.9868C12.0205 13.0032 14 10.3285 14 7.18046C14 7.17943 14 7.17841 14 7.17738C14 3.21278 10.8654 0 7.00001 0Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1989_191"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></div>GitHub</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://bsky.app/profile/deno.land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="BlueSky Logo"><path d="M12 10.8c-1.087-2.114-4.046-6.053-6.798-7.995C2.566.944 1.561 1.266.902 1.565.139 1.908 0 3.08 0 3.768c0 .69.378 5.65.624 6.479.815 2.736 3.713 3.66 6.383 3.364.136-.02.275-.039.415-.056-.138.022-.276.04-.415.056-3.912.58-7.387 2.005-2.83 7.078 5.013 5.19 6.87-1.113 7.823-4.308.953 3.195 2.05 9.271 7.733 4.308 4.267-4.308 1.172-6.498-2.74-7.078a8.741 8.741 0 0 1-.415-.056c.14.017.279.036.415.056 2.67.297 5.568-.628 6.383-3.364.246-.828.624-5.79.624-6.478 0-.69-.139-1.861-.902-2.206-.659-.298-1.664-.62-4.3 1.24C16.046 4.748 13.087 8.687 12 10.8Z" fill="currentColor"></path></svg></div>Bluesky</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://twitter.com/deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg width="300" height="300.251" class="w-4.5 h-4.5" viewBox="0 0 300 300.251" version="1.1" xmlns="http://www.w3.org/2000/svg" aria-label="X logo" fill="currentColor"><path d="M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"></path></svg></div>X</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://www.youtube.com/c/deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="YouTube Logo"><path d="M7.03863 2.5H7.10538C7.72192 2.5024 10.8459 2.52643 11.6882 2.76826C11.9428 2.84207 12.1748 2.98593 12.3611 3.18548C12.5473 3.38502 12.6813 3.63326 12.7495 3.90537C12.8252 4.20967 12.8785 4.61247 12.9145 5.02807L12.922 5.11135L12.9385 5.31956L12.9445 5.40284C12.9932 6.13475 12.9992 6.82022 13 6.96997V7.03003C12.9992 7.18538 12.9925 7.9173 12.9385 8.67964L12.9325 8.76372L12.9257 8.84701C12.8882 9.30505 12.8327 9.7599 12.7495 10.0946C12.6815 10.3669 12.5476 10.6152 12.3613 10.8148C12.1751 11.0144 11.9429 11.1582 11.6882 11.2317C10.8181 11.4816 7.51116 11.4992 7.05288 11.5H6.94637C6.71461 11.5 5.75605 11.4952 4.75098 11.4584L4.62348 11.4536L4.55822 11.4504L4.42996 11.4447L4.30171 11.4391C3.46915 11.3999 2.67635 11.3366 2.31108 11.2309C2.05643 11.1574 1.82435 11.0138 1.63808 10.8143C1.45182 10.6149 1.3179 10.3667 1.24977 10.0946C1.16651 9.7607 1.11101 9.30505 1.0735 8.84701L1.0675 8.76292L1.0615 8.67964C1.02449 8.13703 1.00398 7.59328 1 7.04925L1 6.95075C1.0015 6.77858 1.0075 6.1836 1.048 5.52696L1.05325 5.44448L1.0555 5.40284L1.0615 5.31956L1.078 5.11135L1.08551 5.02807C1.12151 4.61247 1.17476 4.20887 1.25052 3.90537C1.31854 3.63315 1.4524 3.38479 1.63867 3.18522C1.82495 2.98564 2.05709 2.84185 2.31183 2.76826C2.6771 2.66416 3.4699 2.6001 4.30246 2.56006L4.42996 2.55445L4.55897 2.54965L4.62348 2.54725L4.75173 2.54164C5.46556 2.51712 6.17968 2.5035 6.89387 2.5008L7.03863 2.5ZM6 5.02807V9L9 7.03003L6 5.02807Z" fill="currentColor"></path></svg></div>YouTube</div></div></a></li><li class="relative text-sm font-bold lg:top-1 lg:text-base lg:font-normal lg:flex hover:bg-azure3 dark:hover:bg-gray-800 focus-within:bg-azure3 dark:focus-within:bg-gray-800 lg:hover:bg-neutral-100 lg:dark:hover:bg-gray-800 lg:focus-within:bg-neutral-100 lg:dark:focus-within:bg-gray-900"><a aria-current="false" tabindex="-1" class="py-1.5 lg:py-3.5 leading-none flex flex-wrap w-full pl-1 lg:px-5" href="https://fosstodon.org/@deno_land"><div class="w-full"><div class="w-max flex gap-4 items-center"><div class="flex justify-center items-center w-6 opacity-50"><svg class="w-6 h-6" width="14" height="14" viewBox="-10 -5 1034 1034" xmlns="http://www.w3.org/2000/svg" version="1.1"><path fill="currentColor" d="M499 112q-93 1 -166 11q-81 11 -128 33l-14 8q-16 10 -32 25q-22 21 -38 47q-21 33 -32 73q-14 47 -14 103v37q0 77 1 119q3 113 18 188q19 95 62 154q50 67 134 89q109 29 210 24q46 -3 88 -12q30 -7 55 -17l19 -8l-4 -75l-22 6q-28 6 -57 10q-41 6 -78 4q-53 -1 -80 -7
q-43 -8 -67 -30q-29 -25 -35 -72q-2 -14 -2 -29l25 6q31 6 65 10q48 7 93 9q42 2 92 -2q32 -2 88 -9t107 -30q49 -23 81.5 -54.5t38.5 -63.5q9 -45 13 -109q4 -46 5 -97v-41q0 -56 -14 -103q-11 -40 -32 -73q-16 -26 -38 -47q-15 -15 -32 -25q-12 -8 -14 -8
q-46 -22 -127 -33q-74 -10 -166 -11h-3zM367 267q73 0 109 56l24 39l24 -39q36 -56 109 -56q63 0 101 43t38 117v239h-95v-232q0 -74 -61 -74q-69 0 -69 88v127h-94v-127q0 -88 -69 -88q-61 0 -61 74v232h-95v-239q0 -74 38 -117t101 -43z"></path></svg></div>Mastodon</div></div></a></li></ul></div></div></div></div><!--/frsh:island--><div class="dark:border-gray-800"><a class="block w-full px-1 my-3 lg:w-auto lg:m-0 lg:px-2 lg:rounded-md lg:hover:bg-azure3 lg:hover:dark:bg-gray-800 " href="/blog" aria-current="false">Blog</a></div></div><div class="flex flex-col lg:flex-row grow gap-4 lg:items-center lg:justify-end"><!--frsh:island:OramaSearch:6:--><div class="md:relative w-full hidden lg:block max-w-64"><div class="relative"><input type="search" placeholder="Search" id="orama-search-input"   value=""   class="w-full min-w-24 rounded-lg placeholder:text-sm text-base leading-normal p-1 pl-8 border transition-all duration-150
          border-gray-600 dark:border-gray-100 focus:outline-offset-1 disabled:opacity-50"><svg class="absolute top-2 left-2 size-4.5 text-gray-600 pointer-events-none " width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg"><g clip-path="url(#clip0_2006_89)"><path d="M13.3008 12.1951L10.3537 9.24807C11.0632 8.30353 11.4463 7.15382 11.445 5.97248C11.445 2.95497 8.98999 0.5 5.97248 0.5C2.95497 0.5 0.5 2.95497 0.5 5.97248C0.5 8.98999 2.95497 11.445 5.97248 11.445C7.15382 11.4463 8.30353 11.0632 9.24807 10.3537L12.1951 13.3008C12.3443 13.4341 12.5389 13.5053 12.7389 13.4997C12.9389 13.4941 13.1292 13.4121 13.2707 13.2707C13.4121 13.1292 13.4941 12.9389 13.4997 12.7389C13.5053 12.5389 13.4341 12.3443 13.3008 12.1951ZM2.06357 5.97248C2.06357 5.19937 2.29282 4.44362 2.72234 3.8008C3.15185 3.15799 3.76234 2.65697 4.4766 2.36111C5.19086 2.06526 5.97682 1.98785 6.73507 2.13867C7.49333 2.2895 8.18983 2.66179 8.7365 3.20846C9.28317 3.75513 9.65546 4.45163 9.80629 5.20989C9.95711 5.96814 9.8797 6.7541 9.58385 7.46836C9.28799 8.18262 8.78697 8.79311 8.14416 9.22262C7.50134 9.65214 6.74559 9.88139 5.97248 9.88139C4.93615 9.88015 3.94263 9.46792 3.20983 8.73513C2.47704 8.00233 2.06481 7.00881 2.06357 5.97248Z" fill="currentColor"></path></g><defs><clipPath id="clip0_2006_89"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg><kbd id="search-key" class="hidden xs:flex pointer-events-none absolute font-sans rounded-sm top-1 right-1 bottom-1 w-auto border-1 border-gray-500 dark:border-gray-800 border-b-2 border-r-2 bg-white text-gray-600 dark:text-gray-100 text-center text-xs font-bold p-2 items-center justify-center dark:bg-gray-800">⌘K</kbd><div id="orama-search-loading" class="absolute left-2 top-1/2 transform -translate-y-1/2 hidden bg-white dark:bg-black"><div class="animate-spin rounded-full h-4 w-4 border-2 border-transparent border-r-black dark:border-r-white bg-white dark:bg-black"></div></div><div class="sr-only" aria-live="polite" id="orama-results-announcer" ></div></div><div id="orama-search-results" class="absolute inset-2 left-2 right-2 h-[calc(100vh-10rem)] lg:h-[calc(100vh-8rem)] top-32 lg:top-10 md:top-full md:left-auto md:right-0 mt-2 bg-white dark:bg-black dark:text-white border border-gray-500 dark:border-gray-800 rounded-xl shadow-2xl z-50 md:max-h-128 overflow-hidden hidden md:min-w-160 max-w-2xl"><div id="orama-search-results-content" class="overflow-y-auto h-full"></div><div class="border-t border-gray-500 dark:border-gray-800 bg-gray-50 dark:bg-gray-900 px-4 py-2 sticky bottom-0"><div class="flex items-center gap-6 text-xs text-gray-500 dark:text-gray-500"><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↑↓</span><span class="sr-only">Up or down to</span></kbd>navigate</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">↵</span><span class="sr-only">Enter to</span></kbd>select</span><span><kbd class="px-1.5 py-0.5 text-xs font-semibold text-gray-600 bg-white border-r-2 border-b-2 border border-gray-500 dark:border-gray-800 rounded mr-1"><span aria-hidden="true">ESC</span><span class="sr-only">Escape to</span></kbd>close</span><img src="/images/orama-dark.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="dark:hidden h-4 w-auto ml-auto"/><img src="/images/orama-light.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Search powered by Orama" class="hidden dark:inline h-4 w-auto ml-auto"/></div></div></div></div><!--/frsh:island--></div></div></nav></div></header><main tabindex="-1" id="main"><div class="w-full overflow-x-hidden relative flex justify-between h-full flex-col flex-wrap"><div><div class="text-center px-8 py-32 z-[3] dark:text-white mx-auto max-w-lg"><h1 class="font-bold text-5xl leading-10 tracking-tight">404</h1><h2 class="mt-4 sm:mt-5 text-2xl text-center leading-tight">Couldn&#39;t find what you&#39;re looking for.</h2><p class="mt-4"><a href="/" class="underline" data-ancestor="true" aria-current="true">Home</a> | <a href="https://docs.deno.com" class="underline">Docs</a></p></div></div><div><div class="mt-auto w-full pointer-events-none h-[200px] relative overflow-hidden"><img src="/images/ferris.gif?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Ferris" class="translate-y-[22px] w-[100px] absolute left-[60%] bottom-0 dark:filter dark:invert"/><img src="/images/deno404.gif?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" alt="Deno" class="w-[200px] relative top-[24px] animate-move dark:filter dark:invert"/></div></div></div></main><footer class="border-t text-runtime text-sm flex justify-center bg-offblack border-gray-800"><div class="section-x-inset-xl py-8 lg:pt-16 w-full"><div class="flex flex-col items-start gap-16 md:flex-row md:gap-8 md:w-full md:justify-between max-w-4xl"><a href="/" class="flex items-center" data-ancestor="true" aria-current="true"><img src="/logos/dark-transparent.svg?__frsh_c=d132bb87bfeb7e692861ab9f04abf11c0d7c549e" class="h-10 w-auto max-w-none block" alt="Deno logo"/></a><div><h2 class="leading-tight font-bold" id="FooterOpen-source">Open-source</h2><ul aria-labelledby="FooterOpen-source" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/" class="block hover:underline" data-ancestor="true" aria-current="true">Deno</a></li><li><a href="https://fresh.deno.dev/" class="block hover:underline">Fresh</a></li><li><a href="https://jsr.io/" class="block hover:underline">JSR</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterProducts">Products</h2><ul aria-labelledby="FooterProducts" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/deploy" class="block hover:underline">Deno Deploy</a></li><li><a href="/deploy/sandbox" class="block hover:underline">Deno Sandbox</a></li><li><a href="/deploy/subhosting" class="block hover:underline">Subhosting</a></li><li><a href="/deploy/pricing" class="block hover:underline">Pricing</a></li><li><a href="https://docs.deno.com/deploy/" class="block hover:underline">Deno Deploy Docs</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterDeno Resources">Deno Resources</h2><ul aria-labelledby="FooterDeno Resources" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/manual" class="block hover:underline">Deno Docs</a></li><li><a href="https://docs.deno.com/api/deno/" class="block hover:underline">Deno API Reference</a></li><li><a href="https://jsr.io/@std" class="block hover:underline">Standard Library</a></li><li><a href="https://jsr.io/" class="block hover:underline">Third-Party Modules</a></li><li><a href="https://examples.deno.land" class="block hover:underline">Deno Examples</a></li></ul></div><div><h2 class="leading-tight font-bold" id="FooterCompany">Company</h2><ul aria-labelledby="FooterCompany" class="text-gray-300 flex-col flex flex-wrap p-0 gap-3 m-0 mt-4"><li><a href="/blog" class="block hover:underline">Blog</a></li><li><a href="https://deno.news" class="block hover:underline">News</a></li><li><a href="/brand" class="block hover:underline">Branding</a></li><li><a href="https://docs.deno.com/deploy/privacy_policy/" class="block hover:underline">Privacy Policy</a></li><li><a href="https://docs.deno.com/deploy/terms_and_conditions/" class="block hover:underline">Terms and Conditions</a></li><li><a href="mailto:abuse@deno.com" class="block hover:underline" target="_blank">Report abuse</a></li></ul></div></div><div class="w-full text-center flex flex-col md:flex-row flex-wrap justify-between items-center gap-4 pt-8 mt-8 border-t border-gray-800"><div class="flex items-center justify-center gap-3 text-gray-300"><a href="https://github.com/denoland"><span class="sr-only">GitHub</span><svg class="size-6" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="GitHub Logo"><g clip-path="url(#clip0_1989_191)"><path d="M7.00001 0C3.13391 0 0 3.21295 0 7.17755C0 10.3482 2.0055 13.0388 4.7873 13.9875C5.1373 14.0534 5.26471 13.832 5.26471 13.6414C5.26471 13.4716 5.25912 13.0195 5.25561 12.4212C3.3082 12.8547 2.8973 11.4589 2.8973 11.4589C2.5795 10.6291 2.1203 10.4084 2.1203 10.4084C1.48471 9.96418 2.16861 9.97279 2.16861 9.97279C2.87071 10.0229 3.24032 10.7122 3.24032 10.7122C3.86472 11.8085 4.87903 11.4918 5.27732 11.3084C5.34171 10.8448 5.52232 10.5288 5.72251 10.3497C4.16851 10.1684 2.534 9.55218 2.534 6.80211C2.534 6.01893 2.807 5.37764 3.2543 4.87605C3.1822 4.69476 2.94211 3.96463 3.32289 2.97722C3.32289 2.97722 3.91089 2.78376 5.24789 3.71238C5.77305 3.55992 6.37629 3.47184 6.99948 3.4709C7.59448 3.47377 8.19351 3.5533 8.7528 3.71238C10.0891 2.78376 10.6757 2.97649 10.6757 2.97649C11.0579 3.9646 10.8171 4.69475 10.7457 4.87603C11.1937 5.3776 11.4653 6.0189 11.4653 6.80208C11.4653 9.55931 9.82799 10.1662 8.26908 10.3439C8.52037 10.5653 8.74368 11.0031 8.74368 11.6731C8.74368 12.6318 8.73529 13.4064 8.73529 13.6414C8.73529 13.8335 8.86129 14.057 9.21689 13.9868C12.0205 13.0032 14 10.3285 14 7.18046C14 7.17943 14 7.17841 14 7.17738C14 3.21278 10.8654 0 7.00001 0Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1989_191"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></a><a href="https://discord.gg/deno"><span class="sr-only">Discord</span><svg class="size-7" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="Discord Logo"><g clip-path="url(#clip0_1993_195)"><path d="M11.8595 2.62936C10.9397 2.21264 9.96862 1.917 8.97117 1.75C8.83467 1.99146 8.71117 2.23988 8.60118 2.49424C7.5387 2.33581 6.45822 2.33581 5.39574 2.49424C5.28569 2.23991 5.16219 1.99149 5.02575 1.75C4.02766 1.91841 3.05598 2.21475 2.13524 2.63154C0.307332 5.30775 -0.188185 7.9175 0.0595735 10.4902C1.13004 11.2729 2.3282 11.8681 3.60197 12.25C3.88878 11.8683 4.14258 11.4633 4.36066 11.0394C3.94644 10.8863 3.54665 10.6974 3.16591 10.4749C3.26612 10.403 3.36412 10.3289 3.45882 10.257C4.56668 10.7726 5.77586 11.0399 7.00011 11.0399C8.22437 11.0399 9.43354 10.7726 10.5414 10.257C10.6372 10.3344 10.7352 10.4085 10.8343 10.4749C10.4528 10.6978 10.0523 10.887 9.63736 11.0405C9.85518 11.4642 10.109 11.8688 10.3961 12.25C11.6709 11.8696 12.87 11.2747 13.9406 10.4913C14.2314 7.50778 13.444 4.92201 11.8595 2.62936ZM4.67449 8.908C3.98407 8.908 3.41367 8.28798 3.41367 7.52522C3.41367 6.76245 3.96425 6.13699 4.67228 6.13699C5.38032 6.13699 5.94631 6.76245 5.9342 7.52522C5.92209 8.28798 5.37812 8.908 4.67449 8.908ZM9.32574 8.908C8.63422 8.908 8.06602 8.28798 8.06602 7.52522C8.06602 6.76245 8.6166 6.13699 9.32574 6.13699C10.0349 6.13699 10.5965 6.76245 10.5843 7.52522C10.5722 8.28798 10.0294 8.908 9.32574 8.908Z" fill="currentColor"></path></g><defs><clipPath id="clip0_1993_195"><rect width="14" height="14" fill="white"></rect></clipPath></defs></svg></a><a href="https://bsky.app/profile/deno.land"><span class="sr-only">Bluesky</span><svg class="size-6" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="BlueSky Logo"><path d="M12 10.8c-1.087-2.114-4.046-6.053-6.798-7.995C2.566.944 1.561 1.266.902 1.565.139 1.908 0 3.08 0 3.768c0 .69.378 5.65.624 6.479.815 2.736 3.713 3.66 6.383 3.364.136-.02.275-.039.415-.056-.138.022-.276.04-.415.056-3.912.58-7.387 2.005-2.83 7.078 5.013 5.19 6.87-1.113 7.823-4.308.953 3.195 2.05 9.271 7.733 4.308 4.267-4.308 1.172-6.498-2.74-7.078a8.741 8.741 0 0 1-.415-.056c.14.017.279.036.415.056 2.67.297 5.568-.628 6.383-3.364.246-.828.624-5.79.624-6.478 0-.69-.139-1.861-.902-2.206-.659-.298-1.664-.62-4.3 1.24C16.046 4.748 13.087 8.687 12 10.8Z" fill="currentColor"></path></svg></a><a href="https://fosstodon.org/@deno_land"><span class="sr-only">Mastodon</span><svg class="size-6" width="14" height="14" viewBox="-10 -5 1034 1034" xmlns="http://www.w3.org/2000/svg" version="1.1"><path fill="currentColor" d="M499 112q-93 1 -166 11q-81 11 -128 33l-14 8q-16 10 -32 25q-22 21 -38 47q-21 33 -32 73q-14 47 -14 103v37q0 77 1 119q3 113 18 188q19 95 62 154q50 67 134 89q109 29 210 24q46 -3 88 -12q30 -7 55 -17l19 -8l-4 -75l-22 6q-28 6 -57 10q-41 6 -78 4q-53 -1 -80 -7
q-43 -8 -67 -30q-29 -25 -35 -72q-2 -14 -2 -29l25 6q31 6 65 10q48 7 93 9q42 2 92 -2q32 -2 88 -9t107 -30q49 -23 81.5 -54.5t38.5 -63.5q9 -45 13 -109q4 -46 5 -97v-41q0 -56 -14 -103q-11 -40 -32 -73q-16 -26 -38 -47q-15 -15 -32 -25q-12 -8 -14 -8
q-46 -22 -127 -33q-74 -10 -166 -11h-3zM367 267q73 0 109 56l24 39l24 -39q36 -56 109 -56q63 0 101 43t38 117v239h-95v-232q0 -74 -61 -74q-69 0 -69 88v127h-94v-127q0 -88 -69 -88q-61 0 -61 74v232h-95v-239q0 -74 38 -117t101 -43z"></path></svg></a><a href="https://twitter.com/deno_land"><span class="sr-only">Twitter or X or whatever</span><svg width="300" height="300.251" class="size-5" viewBox="0 0 300 300.251" version="1.1" xmlns="http://www.w3.org/2000/svg" aria-label="X logo" fill="currentColor"><path d="M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"></path></svg></a><a href="https://youtube.com/@deno_land"><span class="sr-only">YouTube</span><svg class="size-7" width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="YouTube Logo"><path d="M7.03863 2.5H7.10538C7.72192 2.5024 10.8459 2.52643 11.6882 2.76826C11.9428 2.84207 12.1748 2.98593 12.3611 3.18548C12.5473 3.38502 12.6813 3.63326 12.7495 3.90537C12.8252 4.20967 12.8785 4.61247 12.9145 5.02807L12.922 5.11135L12.9385 5.31956L12.9445 5.40284C12.9932 6.13475 12.9992 6.82022 13 6.96997V7.03003C12.9992 7.18538 12.9925 7.9173 12.9385 8.67964L12.9325 8.76372L12.9257 8.84701C12.8882 9.30505 12.8327 9.7599 12.7495 10.0946C12.6815 10.3669 12.5476 10.6152 12.3613 10.8148C12.1751 11.0144 11.9429 11.1582 11.6882 11.2317C10.8181 11.4816 7.51116 11.4992 7.05288 11.5H6.94637C6.71461 11.5 5.75605 11.4952 4.75098 11.4584L4.62348 11.4536L4.55822 11.4504L4.42996 11.4447L4.30171 11.4391C3.46915 11.3999 2.67635 11.3366 2.31108 11.2309C2.05643 11.1574 1.82435 11.0138 1.63808 10.8143C1.45182 10.6149 1.3179 10.3667 1.24977 10.0946C1.16651 9.7607 1.11101 9.30505 1.0735 8.84701L1.0675 8.76292L1.0615 8.67964C1.02449 8.13703 1.00398 7.59328 1 7.04925L1 6.95075C1.0015 6.77858 1.0075 6.1836 1.048 5.52696L1.05325 5.44448L1.0555 5.40284L1.0615 5.31956L1.078 5.11135L1.08551 5.02807C1.12151 4.61247 1.17476 4.20887 1.25052 3.90537C1.31854 3.63315 1.4524 3.38479 1.63867 3.18522C1.82495 2.98564 2.05709 2.84185 2.31183 2.76826C2.6771 2.66416 3.4699 2.6001 4.30246 2.56006L4.42996 2.55445L4.55897 2.54965L4.62348 2.54725L4.75173 2.54164C5.46556 2.51712 6.17968 2.5035 6.89387 2.5008L7.03863 2.5ZM6 5.02807V9L9 7.03003L6 5.02807Z" fill="currentColor"></path></svg></a></div><span class="block text-xs text-gray-500 leading-tight">Copyright © 2026 Deno Land Inc. <span class="whitespace-nowrap">All rights reserved.</span>  | <!--frsh:island:CookieOptOut:7:--><!--/frsh:island--></span><a href="https://denostatus.com/" class="transition-colors shadow-md px-4 py-2 rounded-lg font-bold inline-flex items-center gap-2 text-base bg-gray-800 border border-gray-800 hover:border-runtime text-gray-50"><div class="w-3 h-3 rounded-full bg-runtime"></div><span>All systems operational</span></a></div></div></footer></div></div><script type="module" nonce="dd1743cf2fc849b8bd93705fe6956636">import { boot } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/fresh-runtime.js";import { SkipToMainLink } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/SkipToMainLink.js";import { HelloBar } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/HelloBar.js";import { OramaSearch } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/OramaSearch.js";import { NavItemWithChildren } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/NavItemWithChildren.js";import { CookieOptOut } from "/_fresh/js/d132bb87bfeb7e692861ab9f04abf11c0d7c549e/AnalyticsOptOut.js";boot({SkipToMainLink,HelloBar,OramaSearch,NavItemWithChildren,CookieOptOut},"[[1,4,11,19,62,75,100,105],{\"slots\":2,\"props\":3},[],{},{\"slots\":5,\"props\":6},[],{\"id\":7,\"href\":8,\"ctaText\":9,\"text\":10},\"clawpatrol-2026-05\",\"https://deno.com/blog/clawpatrol\",\"Read the post\",\"Introducing Claw Patrol: an open-source security firewall for agents\",{\"slots\":12,\"props\":13},[],{\"projectId\":14,\"apiKey\":15,\"dataSourceIDs\":16,\"class\":18},\"af2a2800-bfac-4b67-a58e-51ada826e1c0\",\"c1_SHxojDzeso5ITKAGMDjvgpZk43Ff0pFiQhmdGfiLBtnUWKKJFnWGSNs_Zb7\",[17],\"ad8a33af-4669-444e-9bed-b0716b0a9e63\",\"lg:hidden\",{\"slots\":20,\"props\":21},[],{\"entry\":22,\"path\":60,\"activeNav\":61},{\"content\":23,\"children\":24},\"Products\",[25,30,34,38,42,47,52,56],{\"href\":26,\"content\":27,\"subhead\":28,\"category\":29},\"/\",\"Deno\",\"Modern runtime for JavaScript and TypeScript\",\"Open Source\",{\"href\":31,\"content\":32,\"subhead\":33,\"category\":29},\"https://fresh.deno.dev\",\"Fresh\",\"Web framework designed for the edge\",{\"href\":35,\"content\":36,\"subhead\":37,\"category\":29},\"https://clawpatrol.dev\",\"Claw Patrol\",\"Open-source security firewall for agents\",{\"href\":39,\"content\":40,\"subhead\":41,\"category\":29},\"https://jsr.io\",\"JSR\",\"TypeScript-first ESM package registry\",{\"href\":43,\"content\":44,\"subhead\":45,\"category\":46},\"/deploy\",\"Deno Deploy\",\"Easy serverless hosting for your JavaScript projects\",\"Commercial\",{\"href\":48,\"content\":49,\"subhead\":50,\"category\":46,\"nested\":51},\"/deploy/sandbox\",\"Deno Sandbox\",\"Run untrusted code in secure Linux VMs. Built for AI agents.\",true,{\"href\":53,\"content\":54,\"subhead\":55,\"category\":46,\"nested\":51},\"/subhosting\",\"Subhosting\",\"Extend your platform using Deno Deploy's secure infrastructure\",{\"href\":57,\"content\":58,\"subhead\":59,\"category\":46},\"/enterprise\",\"Deno for Enterprise\",\"Enterprise support for runtime projects\",\"/llms-full.txt\",[\"Signal\",-2],{\"slots\":63,\"props\":64},[],{\"entry\":65,\"path\":60,\"activeNav\":61},{\"content\":66,\"children\":67},\"Modules\",[68,71,72],{\"href\":69,\"content\":70},\"https://jsr.io/@std\",\"Standard Library\",{\"href\":39,\"content\":40},{\"href\":73,\"content\":74},\"/npm/\",\"Node.js & npm\",{\"slots\":76,\"props\":77},[],{\"entry\":78,\"path\":60,\"activeNav\":61},{\"content\":79,\"children\":80},\"Community\",[81,84,87,91,94,97],{\"href\":82,\"content\":83,\"icon\":83},\"https://discord.gg/deno\",\"Discord\",{\"href\":85,\"content\":86,\"icon\":86},\"https://github.com/denoland\",\"GitHub\",{\"href\":88,\"content\":89,\"icon\":90},\"https://bsky.app/profile/deno.land\",\"Bluesky\",\"BlueSky\",{\"href\":92,\"content\":93,\"icon\":93},\"https://twitter.com/deno_land\",\"X\",{\"href\":95,\"content\":96,\"icon\":96},\"https://www.youtube.com/c/deno_land\",\"YouTube\",{\"href\":98,\"content\":99,\"icon\":99},\"https://fosstodon.org/@deno_land\",\"Mastodon\",{\"slots\":101,\"props\":102},[],{\"projectId\":14,\"apiKey\":15,\"dataSourceIDs\":103,\"class\":104},[17],\"hidden lg:block max-w-64\",{\"slots\":106,\"props\":107},[],{}]");</script></body></html>

One simple platform for anything that runs with JavaScript or Typescript.

Seamlessly integrate with your production database, and get an isolated clone with every PR!

Or, provision a new connected database, like our simple managed KV store, Prisma, and more, right from the dashboard.

Deno Deploy is built on standard OpenTelemetry protocols, giving you auto-instrumented logs, traces, and metrics out of the box.

Explore through the Deno Deploy dashboard, or easily forward to your own observability platform.

Build, deploy, manage, and observe your apps locally from the command line.

Tunnels automatically send telemetry to your Deno Deploy dashboard, and pull down production env vars to use locally!

Instantly spin up new projects right in the browser, complete with a live preview URL!

Add files, configure builds, and even set unique .env variables for each playground

Interested in self-hosting Deno Deploy?

## Now Generally AvailableDeno Deploy

## — What’s new —

## — What’s improved —

## Still trusted by the best

## Deno Deploy success stories

## Ready to try the new Deno Deploy?

## Made for all JavaScript

## Has everything your project needs

## A data layer built for simplicity

## Powerful developer tooling

## A fast and flexible platform

## Built for builders

## Managed, or self-hosted

## How Netlify used Deno Subhosting to build a successful edge functions product

## How Slack used Deno to save months of engineering effort in launching their new platform

## How Brazil's top ecommerce platform used Deno Subhosting to drive 5x faster page load speeds

```
$ deno task --tunnel dev
```

```
https://app--local.org.deno.net
```

```
Send telemetry, use prod env vars,
```

```
and more with tunneling!
```


