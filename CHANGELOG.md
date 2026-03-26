# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.2.0](https://github.com/azais-corentin/cmakefmt/compare/9cf8de673944ba88c2847550f67db55b208a28fc..v0.2.0) - 2026-03-26




### Added

- **devenv:** add cargo-pgo and llvm packages - ([cbb8aaa](https://github.com/azais-corentin/cmakefmt/commit/cbb8aaadf3b7e2c862bf145beac67b11e30131e4))



### Fixed

- resolve all clippy warnings - ([54a84b5](https://github.com/azais-corentin/cmakefmt/commit/54a84b559e7ec56868608f177e5bed0daf335199))
- collapse multi-line keyword commands to single line when they fit (#7) - ([c6371a2](https://github.com/azais-corentin/cmakefmt/commit/c6371a2ead1770c6ed74e502e4f71afc861d1860))
- **ci:** redeploy docs after release via workflow_run trigger - ([9cf8de6](https://github.com/azais-corentin/cmakefmt/commit/9cf8de673944ba88c2847550f67db55b208a28fc))





<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **documentation(website):** update playground example to match README screenshots - ([6e50180](https://github.com/azais-corentin/cmakefmt/commit/6e501807d642b24ab512251811782b41e4ae0a58))
- **refactoring:** remove unused tokenize function in token.rs - ([6b2ee20](https://github.com/azais-corentin/cmakefmt/commit/6b2ee20c95ce56137549b9a48b7f3bacd14df4d8))
- **tests:** add unit and integration tests for core modules and CLI - ([84c46fc](https://github.com/azais-corentin/cmakefmt/commit/84c46fc9a198e5fc373229c3f3002378e6fb5081))

</details>


- - -

## [v0.1.5](https://github.com/azais-corentin/cmakefmt/compare/22b2159ead7117d8458979e40b6350da8cd82172..v0.1.5) - 2026-03-19






### Fixed

- restore CHANGELOG.md and exclude from dprint - ([5210a24](https://github.com/azais-corentin/cmakefmt/commit/5210a24c6aab111eabab78c1b292d5e0e07f3b62))
- treat target_include_directories keyword as group header (#4) - ([2d42b04](https://github.com/azais-corentin/cmakefmt/commit/2d42b04da4c3a8ff6000c542d0fc98fcd083ebf4))
- use day-precision dates in changelog instead of second-precision - ([92a29a2](https://github.com/azais-corentin/cmakefmt/commit/92a29a20b75f8c013fed0d911fad9b6855f5fa23))
- serve WASM plugin same-origin from GitHub Pages - ([22b2159](https://github.com/azais-corentin/cmakefmt/commit/22b2159ead7117d8458979e40b6350da8cd82172))





<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **documentation(website):** make Configuration sidebar item clickable - ([d962a54](https://github.com/azais-corentin/cmakefmt/commit/d962a54576a9e7d52e58d6e258737714fde6eed9))
- **documentation:** add resolving formatting issues workflow to AGENTS.md - ([cdd0997](https://github.com/azais-corentin/cmakefmt/commit/cdd0997c6a3b68aa834b7f3d47a36464af2957dc))
- **documentation(website):** fix errors and add missing behavioral details to config pages - ([058c361](https://github.com/azais-corentin/cmakefmt/commit/058c3617f82c92de5a03ffb780800955cd1bc321))
- **documentation:** remove screenshots from pull request - ([0a70269](https://github.com/azais-corentin/cmakefmt/commit/0a7026973618219ca4b2fd347f22616a4cd6eb69))
- **documentation:** correct dprint configuration filename from .dprintrc.json to dprint.json - ([815d631](https://github.com/azais-corentin/cmakefmt/commit/815d631cfaa3c789a4d4652f1a487dc4941e3194))
- **documentation:** split configuration documentation into TOML and JSON sections with screenshots - ([00ec746](https://github.com/azais-corentin/cmakefmt/commit/00ec746e9825c18226a9a20f233e4224a7f88b4e))
- **documentation:** verify website builds successfully - ([41e2cfd](https://github.com/azais-corentin/cmakefmt/commit/41e2cfd9d04be04e3c12513f49e5866b98146673))
- **documentation:** split configuration into TOML and JSON sections - ([46df933](https://github.com/azais-corentin/cmakefmt/commit/46df9331374a7450dd19940069a6d322130dbe3c))
- **miscellaneous chores(website):** remove package-lock.json - ([b6e360c](https://github.com/azais-corentin/cmakefmt/commit/b6e360c174cb2a1f2a6520476086d9ccae0641b6))

</details>


- - -

## [v0.1.4](https://github.com/azais-corentin/cmakefmt/compare/e0d9c91c9682c2cb99719a3ba870654aaf4a8c62..v0.1.4) - 2026-03-18










<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **continuous integration:** install cocogitto using mise in release workflow - ([e0d9c91](https://github.com/azais-corentin/cmakefmt/commit/e0d9c91c9682c2cb99719a3ba870654aaf4a8c62))

</details>


- - -

## [v0.1.3](https://github.com/azais-corentin/cmakefmt/compare/023b2ed4742f403574faf6a5775d8f3e88cd4335..v0.1.3) - 2026-03-18










<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **continuous integration:** replace manual cocogitto install with mise-action in release workflow - ([023b2ed](https://github.com/azais-corentin/cmakefmt/commit/023b2ed4742f403574faf6a5775d8f3e88cd4335))

</details>


- - -

## [v0.1.2](https://github.com/azais-corentin/cmakefmt/compare/36cef39984d04edb05aa4ce793c7eecc85a584ef..v0.1.2) - 2026-03-18










<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **continuous integration:** pin cocogitto to v7.0.0 in release workflow - ([f61993b](https://github.com/azais-corentin/cmakefmt/commit/f61993b14ddba52a6614be96e4d1881258aaedaa))
- **continuous integration:** add retry logic to cargo publish in release workflow - ([5577115](https://github.com/azais-corentin/cmakefmt/commit/55771150e1d7eced0a9f9097d64a2a9e7955f9ab))
- **continuous integration:** filter benchmark-fixtures workflow to main branch pushes only - ([36cef39](https://github.com/azais-corentin/cmakefmt/commit/36cef39984d04edb05aa4ce793c7eecc85a584ef))

</details>


- - -

## [v0.1.1](https://github.com/azais-corentin/cmakefmt/compare/0d9d720c6ba783d2967c046c8bb326efc59667de..v0.1.1) - 2026-03-18




### Added

- add release:patch, release:minor, and release:major tasks to mise.toml - ([2eaa6e6](https://github.com/azais-corentin/cmakefmt/commit/2eaa6e67523a5b675eb449f32aa0fda6ae174224))







<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **documentation:** use absolute URLs for README images - ([0d9d720](https://github.com/azais-corentin/cmakefmt/commit/0d9d720c6ba783d2967c046c8bb326efc59667de))
- **miscellaneous chores(release):** stop publishing cmakefmt-cli to crates.io - ([8b25eaa](https://github.com/azais-corentin/cmakefmt/commit/8b25eaadfef67a93a90241926c64c9208da78ec9))

</details>


- - -

## [v0.1.0](https://github.com/azais-corentin/cmakefmt/compare/236dd7b0f132bb30f13e32e6d41d267e52172955..v0.1.0) - 2026-03-18


### Breaking Changes

- refactor cascade wrapping, alignment scoping, and atomic genex - ([29b21d2](https://github.com/azais-corentin/cmakefmt/commit/29b21d21c4800ab2d6ad8da67bcbb5a9857844c7))
Overhaul the cascade wrapping algorithm, alignment system, and generator
  expression handling. This is a breaking change that removes three config
  options related to genex formatting.

- remove magicTrailingNewline option - ([375f7c4](https://github.com/azais-corentin/cmakefmt/commit/375f7c449bfb383a636cfecdab408d0b00a3a9b3))
Drop the `magicTrailingNewline` configuration option and all associated
  logic. The input-layout signal that prevented single-line collapse when
  the closing `)` appeared on its own line is no longer supported.

  - Remove `magic_trailing_newline` from Configuration/CommandConfiguration
  - Remove `has_magic_trailing_newline_signal()` and all call sites in
    gen_command.rs
  - Delete `06_magic_trailing_newline/` fixture directory
  - Update specs (§1.5 removed), appendices, and website docs
  - Update affected fixture expectations (alignment, wrapping, synthetic)

- **config:** support command list for spaceBeforeParen - ([141c18d](https://github.com/azais-corentin/cmakefmt/commit/141c18d9c2a00c3d727a0004bca843820c757e0c))



### Added

- **release:** automate release workflow with cocogitto - ([1d56766](https://github.com/azais-corentin/cmakefmt/commit/1d567665b06b3323659d9bd66f02e53ca1dce79c))
- add proptest agent skill - ([cc76d08](https://github.com/azais-corentin/cmakefmt/commit/cc76d088b2ef3469b7b0cc2229c82c3330c4ff65))
- **website:** add interactive WASM playground - ([5f89237](https://github.com/azais-corentin/cmakefmt/commit/5f89237375243afd929a53387596d7df37aba2e2))
- align keyword columns across groups and flow-wrap keyword values - ([50b6d68](https://github.com/azais-corentin/cmakefmt/commit/50b6d68fbdd8e41dac97014877a2fb5dd2ab58eb))
- implement condition genex inline rendering - ([ed25a72](https://github.com/azais-corentin/cmakefmt/commit/ed25a72c0d04016ba6cd3df14f943cbaf78a53b5))
- **website:** use time-proportional X axis with regular-interval grid - ([9e6b58e](https://github.com/azais-corentin/cmakefmt/commit/9e6b58eaa53c5f4d4e81cc312daad34170f41965))
- **website:** replace baseline lines with horizontal bar comparison charts - ([032ca2c](https://github.com/azais-corentin/cmakefmt/commit/032ca2c45475939293f5d1cbffa6a0a13f2d9323))
- add cmake_format and gersemi baseline benchmarks - ([8c289c8](https://github.com/azais-corentin/cmakefmt/commit/8c289c812b0fcadd0712fac71f4dcf3553388939))
- **tracing:** add runtime tracing with summary generation - ([d149ef9](https://github.com/azais-corentin/cmakefmt/commit/d149ef9cce7654be9ecbc322093629dc08edd985))
- **cli:** implement CLI contract parity - ([e43285a](https://github.com/azais-corentin/cmakefmt/commit/e43285a5eb25162729aeea119485a244269bbfdc))
- **interactions:** complete Appendix E interaction conformance - ([f21a897](https://github.com/azais-corentin/cmakefmt/commit/f21a89765bd1b621432dddf11a1d2702c7d952e3))
- **interactions:** implement Appendix E Interaction Conformance - ([4ea1989](https://github.com/azais-corentin/cmakefmt/commit/4ea1989eed688234213a184b56a05bcb7de9e23d))
- **whitespace:** implement line endings, EOF finalization, and spacing controls - ([bd2dfe4](https://github.com/azais-corentin/cmakefmt/commit/bd2dfe42b8e487117c53f58086833aa3b83fe94a))
- **comments:** implement comment formatting engine - ([2c004f9](https://github.com/azais-corentin/cmakefmt/commit/2c004f984f2d5ea40147d005a2b3b4468cb371e7))
- **alignment:** implement alignment modes - ([4b6cca5](https://github.com/azais-corentin/cmakefmt/commit/4b6cca51b40dbc4826f22b1308ebcf8213f5cd98))
- implement formatting controls (genex, casing, sorting, per-command, vertical-rhythm) - ([8b169f4](https://github.com/azais-corentin/cmakefmt/commit/8b169f478918659a3fe12b638eee431d607ef821))
- **pragmas:** implement inline pragma engine - ([20a5601](https://github.com/azais-corentin/cmakefmt/commit/20a56016514c4c8e5c97a5a6c353c72b5d89fee8))
- implement wrapping and indentation engines - ([32daeeb](https://github.com/azais-corentin/cmakefmt/commit/32daeebc3d3f9ca26e998052cd6074c08f631f30))
- implement keyword classification and suppression gate - ([83e5a4d](https://github.com/azais-corentin/cmakefmt/commit/83e5a4d0827f940d21c7499ddc8a0df54f8a371f))
- **config:** add config discovery types and fixture-level loading - ([d9da6e5](https://github.com/azais-corentin/cmakefmt/commit/d9da6e521e77b6057a27c2ae46162211f2bbd63b))
- **config:** add extends resolution and config discovery logic - ([92d2a55](https://github.com/azais-corentin/cmakefmt/commit/92d2a5528f4fe8963a69daa7acb44f6a967b788b))
- **config:** implement Configuration Model Parity - ([731eb79](https://github.com/azais-corentin/cmakefmt/commit/731eb79a29024396e97756923fcc97e4521a66e8))
- **parser:** improve parse error reporting and test handling - ([00e1d6f](https://github.com/azais-corentin/cmakefmt/commit/00e1d6f2e46a182bceddfc200de296624778a778))
- **generation:** improve command layout and boolean handling - ([135cd56](https://github.com/azais-corentin/cmakefmt/commit/135cd56826d9ca8c021001cc546e40d7202aafff))
- **formatter:** migrate to fixture-based tests and stabilize command formatting - ([e18160d](https://github.com/azais-corentin/cmakefmt/commit/e18160dffd20315d831a7d9e4c501b62e05ed7ac))
- **core:** bootstrap cmakefmt formatter and cli defaults - ([236dd7b](https://github.com/azais-corentin/cmakefmt/commit/236dd7b0f132bb30f13e32e6d41d267e52172955))



### Fixed

- **website:** simplify WASM fetch error messages in playground - ([9c025b9](https://github.com/azais-corentin/cmakefmt/commit/9c025b956ccd408501f4f2c0afa1ec5d5b1cf672))
- **ci:** force checkout in benchmark loop to handle overlay-dirtied tree - ([208082d](https://github.com/azais-corentin/cmakefmt/commit/208082d3ae9bce2ff86119e6c7dc16d53121ecdc))
- show descriptive error for 404 in benchmark charts - ([34b62b3](https://github.com/azais-corentin/cmakefmt/commit/34b62b3a2a893c9dd6bd643efec750bf8c84222b))
- **bench:** replace REGEX REPLACE backreference in synthetic fixture - ([1097b70](https://github.com/azais-corentin/cmakefmt/commit/1097b70e7e31c9098d97d48db12346daae01135a))
- **ci:** handle boolean input in reset_history condition - ([9547260](https://github.com/azais-corentin/cmakefmt/commit/95472606580617c31e14fd37870de94ee65a3f48))
- replace bc with awk in size task for Nix compatibility - ([c947721](https://github.com/azais-corentin/cmakefmt/commit/c9477212c124357eddfbed310b7107a88de555b1))
- correct package name in build:debug task - ([7c91c54](https://github.com/azais-corentin/cmakefmt/commit/7c91c549b03d823b996072e71f76cea9ed55c3ce))
- **website:** reorder benchmark comparison bars - ([f2bedf8](https://github.com/azais-corentin/cmakefmt/commit/f2bedf824e20a931fc9b9fc449dba9adc55eee2e))
- track genex depth during sort to preserve multi-line genex units - ([a17f728](https://github.com/azais-corentin/cmakefmt/commit/a17f728f349afaffd45a468396274b9d8238c4f0))
- apply basic formatting to single-line unknown commands - ([0f15a42](https://github.com/azais-corentin/cmakefmt/commit/0f15a42ae6164e29982b231f3cda55bc7f95a680))
- correct blankLineBetweenSections for non-section commands - ([de31311](https://github.com/azais-corentin/cmakefmt/commit/de313115b164bafe0b2b0b3a1a59ee835611c400))
- account for base indent in pack_tokens width calculation - ([b27f0a8](https://github.com/azais-corentin/cmakefmt/commit/b27f0a87dae8e28eb15db66652fbbee4fa452cd3))
- relax property key detection to accept single-word uppercase identifiers - ([98a6c1b](https://github.com/azais-corentin/cmakefmt/commit/98a6c1b5a013c84221f8cd13640d97d3bcc7a9ec))
- **signatures:** remove install from canonical section order - ([a559086](https://github.com/azais-corentin/cmakefmt/commit/a5590865b75ee89028aacdf81c39704ab001563e))
- **signatures:** correct string command back_positional to 0 - ([01905ed](https://github.com/azais-corentin/cmakefmt/commit/01905edceacd90754c6efd879a2176b89ba8645c))
- **website:** use descriptive legend labels in benchmark line charts - ([f48ee78](https://github.com/azais-corentin/cmakefmt/commit/f48ee78c338fb2fd8a1502451fc967196ebe44cf))
- **website:** reset VitePress table background on uPlot legend cells - ([0ff466f](https://github.com/azais-corentin/cmakefmt/commit/0ff466f5643df74917eee16450ad2c6e0e82fe75))
- **ci:** use correct script name for docs build - ([ba0383e](https://github.com/azais-corentin/cmakefmt/commit/ba0383efa7d4ee9631aff044f0fd7de327bb646a))
- **ci:** don't skip workflow on workflow_dispatch with 0 commits - ([48f3cf5](https://github.com/azais-corentin/cmakefmt/commit/48f3cf5da0d16b67d4544a6e04281cbdc2190099))
- **bench:** replace gersemi hyperfine baseline with pytest-benchmark - ([2c36dd2](https://github.com/azais-corentin/cmakefmt/commit/2c36dd28aedecd515dbbefa7adf5eea26d309af3))
- sort source arguments in add_library and add_executable - ([7a2a8ff](https://github.com/azais-corentin/cmakefmt/commit/7a2a8ff61abb10b1fbfa491fb34f7060ebcfbc88))
- recompile test binary when fixture files change - ([f728116](https://github.com/azais-corentin/cmakefmt/commit/f728116dec0bd39a3b03a44a366be6ff6ca87bf1))
- **website:** cache-bust benchmark history JSON fetch - ([43ceb28](https://github.com/azais-corentin/cmakefmt/commit/43ceb2855c47e067e84ddc4e7cde9537c080574f))
- **ci:** persist checkout credentials for benchmark history push - ([c3e1e29](https://github.com/azais-corentin/cmakefmt/commit/c3e1e290e91aa08116e9ad54dbc6dd3181163bbb))
- **ci:** use orphan branch for benchmark history - ([056040e](https://github.com/azais-corentin/cmakefmt/commit/056040e0165df0b9548faa2847059436d5079d14))
- **ci:** publish core crate with correct package id - ([45fc5a4](https://github.com/azais-corentin/cmakefmt/commit/45fc5a453ac3ccd4e7174e56120c2c5b6f1ca60d))
- **build:** add missing size-optimized WASM release profile - ([840b502](https://github.com/azais-corentin/cmakefmt/commit/840b502264332022c8d4ddc0250a16b3ce102411))
- prevent caching of benchmark data by appending timestamp to fetch URL - ([57a4d9d](https://github.com/azais-corentin/cmakefmt/commit/57a4d9d86f6be37c74569cdeb61734e9bcc22a15))
- stop magic_trailing_newline from suppressing front-positional packing - ([2c5d109](https://github.com/azais-corentin/cmakefmt/commit/2c5d1094b5790fc6b9b1c411eaa58a40fdd44386))
- **gen:** preserve source section order for non-canonical target commands - ([f149f92](https://github.com/azais-corentin/cmakefmt/commit/f149f921f4c46411a744b48dbdb83dd9c04f2825))
- **generation:** normalize option and condition genex formatting - ([666c4a0](https://github.com/azais-corentin/cmakefmt/commit/666c4a059e9997f2fb583e2df5921b451e5045bd))
- **formatter:** correct keyword and comment handling in command formatting - ([99e09a5](https://github.com/azais-corentin/cmakefmt/commit/99e09a55d4f6c8477ae22de7d4f5328d7c415985))



### Performance

- eliminate allocations across formatter pipeline (-20%) - ([fc55bd1](https://github.com/azais-corentin/cmakefmt/commit/fc55bd14482cb3e63babc1cfb4c6dc37f806559d))
- replace hand-written byte scanning with memchr SIMD routines - ([20bd9b5](https://github.com/azais-corentin/cmakefmt/commit/20bd9b566c755172949a07cbb776acd7aeb20581))
- reduce allocations in formatter hot paths (-9.5%) - ([d1218bd](https://github.com/azais-corentin/cmakefmt/commit/d1218bd35abf08d9f9aec092eeb9883989bc9693))
- reduce allocations and redundant computation across pipeline - ([977628f](https://github.com/azais-corentin/cmakefmt/commit/977628f96a2f2f5270f2b190eb6043f4a436d41d))
- **gen-command:** optimize hot path checks and reduce allocations - ([557513d](https://github.com/azais-corentin/cmakefmt/commit/557513d96c023c1df21b3f23cd9fff6b86a7d01f))
- reduce formatter allocation overhead - ([8f927be](https://github.com/azais-corentin/cmakefmt/commit/8f927bec89c17f4a14789c714eeb4a9e2fa72e11))
- **printer:** optimize indentation and static fragments - ([9c6b6a6](https://github.com/azais-corentin/cmakefmt/commit/9c6b6a6118cad7cf1bce1c6dd0cf5238756da7c4))



<details>
<summary><h3 style="display:inline">Internal Changes</h3></summary>


- **build system:** consolidate dependencies into workspace.dependencies - ([b221ad9](https://github.com/azais-corentin/cmakefmt/commit/b221ad91a76881f4b458ee217f81eab2da6561ba))
- **build system:** post-restructure build and tooling fixes - ([02f13d1](https://github.com/azais-corentin/cmakefmt/commit/02f13d149f5a4a420251cfd0ade68c42a6b9ae4a))
- **build system(mise):** add test-diff task - ([3395c81](https://github.com/azais-corentin/cmakefmt/commit/3395c819b4d1ac60b2d100220ba4ffd36176085a))
- **continuous integration:** benchmark both XNNPACK and synthetic fixtures everywhere - ([ee41c7f](https://github.com/azais-corentin/cmakefmt/commit/ee41c7f9789b12038e05b228f8546813d8f6145c))
- **continuous integration:** force baseline re-measurement on workflow_dispatch - ([2336eb5](https://github.com/azais-corentin/cmakefmt/commit/2336eb5d607d218e651673f7d0e8362ee2974ffd))
- **continuous integration:** make benchmark-fixtures workflow idempotent with backfill - ([b16af8f](https://github.com/azais-corentin/cmakefmt/commit/b16af8fa94c7dca856729834e05db4c1e7db190b))
- **continuous integration:** harden workflows with pinned actions and least-privilege permissions - ([a6d79aa](https://github.com/azais-corentin/cmakefmt/commit/a6d79aa93e5f604d9a6b548c2f998d4681838e7f))
- **continuous integration(benchmark):** harden workflow and remove Gist upload - ([1aef059](https://github.com/azais-corentin/cmakefmt/commit/1aef0591049f7998a513e0d4746add2be3aae625))
- **continuous integration:** add CI and release workflows - ([77227f6](https://github.com/azais-corentin/cmakefmt/commit/77227f62a1e46294c5aa6907b0b1220cf5dce466))
- **continuous integration:** add benchmark publishing and visualization - ([ebfbd9f](https://github.com/azais-corentin/cmakefmt/commit/ebfbd9f5822941cab0894852804d26e76733a531))
- **continuous integration(bench):** add fixture benchmark automation and configuration - ([e08b672](https://github.com/azais-corentin/cmakefmt/commit/e08b672e54e862bba7da9940f935d4e9ce69b47d))
- **documentation(website):** update benchmark description for two test fixtures - ([b9631c9](https://github.com/azais-corentin/cmakefmt/commit/b9631c94c8285a895f6ef7397970d7c57a8d2934))
- **documentation:** fix factual errors in README, getting-started, and configuration - ([976640f](https://github.com/azais-corentin/cmakefmt/commit/976640f101fd3364acfd943d22cf308e3307bc04))
- **documentation:** fix section cross-reference §15.2 → §15.1 in specs 13 and 16 - ([91c8707](https://github.com/azais-corentin/cmakefmt/commit/91c8707c5e92db669d28755f8209a83cb103f0cf))
- **documentation(website):** use purple cmakefmt benchmark chart colors - ([07b4e3d](https://github.com/azais-corentin/cmakefmt/commit/07b4e3df084ceb5afffce919b1adeefa614409f9))
- **documentation:** fix test command syntax in AGENTS.md - ([fc34dbf](https://github.com/azais-corentin/cmakefmt/commit/fc34dbf885a03c2966af5a9426215af5a798e2e7))
- **documentation:** add spec-change propagation rule to AGENTS.md - ([1b21eb7](https://github.com/azais-corentin/cmakefmt/commit/1b21eb746b6e8431860be1c6cf3a154c89f298f5))
- **documentation:** add dprint fmt step to agent workflow - ([12446f6](https://github.com/azais-corentin/cmakefmt/commit/12446f6d0bc6ca1437cfb13b7a34db47fba79304))
- **documentation(website):** add inline pragmas page, enrich config descriptions, fix CLI flag mapping - ([9e743c7](https://github.com/azais-corentin/cmakefmt/commit/9e743c76075972366d76d88e2560b91a7bb4b8b3))
- **documentation(website):** fix CLI reference page to match spec and implementation - ([dd51723](https://github.com/azais-corentin/cmakefmt/commit/dd517238911257740155f48683de270bae0d2294))
- **documentation:** add website dev commands to AGENTS.md - ([6791890](https://github.com/azais-corentin/cmakefmt/commit/6791890b96a3799a265c98e7f91b27e9e733a047))
- **documentation:** add Puppeteer verification step for website changes - ([c6fcb7e](https://github.com/azais-corentin/cmakefmt/commit/c6fcb7e4c0288409102638b92a1ed31d738b1f1c))
- **documentation(website):** fix benchmark file size description from ~190 KB to ~350 KB - ([e78f42f](https://github.com/azais-corentin/cmakefmt/commit/e78f42f402e93ea3e6dad13aabdc691d5f3fdc80))
- **documentation:** add benchmark methodology paragraph to tool comparison section - ([97ecf8c](https://github.com/azais-corentin/cmakefmt/commit/97ecf8ce9da0b6a8eaf046a5c7b3f8c58ada66d1))
- **documentation:** simplify Before/After section layout in README - ([cbdd7a8](https://github.com/azais-corentin/cmakefmt/commit/cbdd7a8277eaf5edb00059dc58647cd4e4fe0125))
- **documentation:** expand README features list and fix pragma description - ([94db517](https://github.com/azais-corentin/cmakefmt/commit/94db51787d8edf724aeecd000e7d926e98f5357d))
- **documentation:** expand AGENTS.md with detailed architecture and conventions - ([3d733e0](https://github.com/azais-corentin/cmakefmt/commit/3d733e083035a5623e89cd2ab4786203fd8d337b))
- **documentation:** add README and logo branding - ([494d249](https://github.com/azais-corentin/cmakefmt/commit/494d2497dbd7c6b105924c7a9176e4b98475cf78))
- **documentation:** add screenshot generation with charmbracelet/freeze - ([7eb08ed](https://github.com/azais-corentin/cmakefmt/commit/7eb08edc3e2b1afd67c26e038ec99ba41eca344f))
- **documentation:** add VitePress documentation website with benchmarks - ([54c8079](https://github.com/azais-corentin/cmakefmt/commit/54c8079b7a2b6cf12c73c15c05962f0bb0b027d4))
- **documentation(agents):** use mise run commands in development section - ([f70ef33](https://github.com/azais-corentin/cmakefmt/commit/f70ef3376dd9644c67ad31eb9e178f48bdc7c82c))
- **documentation:** document CMAKEFMT_TEST_FILTER env var in AGENTS.md - ([d6caa43](https://github.com/azais-corentin/cmakefmt/commit/d6caa43bc4211261ea4a697e742253fcfd83a7b2))
- **documentation:** add feature phase roadmap - ([2e6182b](https://github.com/azais-corentin/cmakefmt/commit/2e6182b3956f3e0afe95fda04d2c74e8679da72a))
- **documentation:** move analysis to docs directory - ([1008454](https://github.com/azais-corentin/cmakefmt/commit/1008454882a4141be2f669a8a225bc08bf81d1ec))
- **documentation:** simplify AGENTS.md guidance - ([3cce131](https://github.com/azais-corentin/cmakefmt/commit/3cce131f40d73aabc787a5137caac83e33f54ecf))
- **documentation:** update implementation plan - ([36f1ac4](https://github.com/azais-corentin/cmakefmt/commit/36f1ac46e46b1d3cdf800770d39530b3853ed981))
- **documentation:** add project analysis - ([43ebcdb](https://github.com/azais-corentin/cmakefmt/commit/43ebcdb9951e870a6f6e3db960e4049cc24c6666))
- **documentation:** refresh implementation plan from current specs - ([6b26c71](https://github.com/azais-corentin/cmakefmt/commit/6b26c713274a20fc52f9b36a8a2d27cb075f4d25))
- **documentation(specs):** close gaps and add targeted fixtures - ([ef46157](https://github.com/azais-corentin/cmakefmt/commit/ef4615750dfbc0ee8148a6a907a17f4faf6ef491))
- **documentation(agents):** add file-reading delegation guideline - ([7b156cc](https://github.com/azais-corentin/cmakefmt/commit/7b156cc8342af7c3a5eaf929ccbe62b6ae974536))
- **documentation(plan):** refresh plan with full spec validation - ([35d2931](https://github.com/azais-corentin/cmakefmt/commit/35d2931079fd980a07ea036dbd3aa15f05704336))
- **documentation(agents):** remove FEATURES.md references - ([f3902f2](https://github.com/azais-corentin/cmakefmt/commit/f3902f26166d2376d3853e7405a374d6ca16b23e))
- **documentation(plan):** add phased implementation roadmap - ([d957bcc](https://github.com/azais-corentin/cmakefmt/commit/d957bcc3d0ff67d9d55e92105ff8e60c991412b2))
- **documentation(specs):** split monolithic specs into section files - ([7db8be3](https://github.com/azais-corentin/cmakefmt/commit/7db8be3dc3d3b851395c84a8331c1c9d98b8fef4))
- **documentation:** remove FEATURES.md - ([0399def](https://github.com/azais-corentin/cmakefmt/commit/0399defad6cc79f643a7807604a3955415310500))
- **documentation(agents):** add AGENTS workflow guide - ([cdaf6c9](https://github.com/azais-corentin/cmakefmt/commit/cdaf6c939f71ff69932cf5a1370c467876522e72))
- **documentation(specs):** fix contradictions and ambiguous examples - ([c28faa6](https://github.com/azais-corentin/cmakefmt/commit/c28faa694dc4d0f2a7957129a42868484c7b70dc))
- **documentation(agents):** remove AGENTS.md - ([f19b25a](https://github.com/azais-corentin/cmakefmt/commit/f19b25aa608906be7b21c4ab08202d91b631d2b9))
- **documentation(spec):** resolve spec review issues and remove invalid options - ([7f2c315](https://github.com/azais-corentin/cmakefmt/commit/7f2c3152ee2437be65cc5bdb0686abbf8624fb4b))
- **documentation(spec):** move specs to docs directory - ([93ba13e](https://github.com/azais-corentin/cmakefmt/commit/93ba13e7ca456f51a480be624f5a4d2809baf218))
- **documentation(spec):** add examples and polish wording - ([df0bbb3](https://github.com/azais-corentin/cmakefmt/commit/df0bbb38305108ea30bf9c3877a1a8b8fb408874))
- **documentation(spec):** improve accuracy, add appendix F, and fix bugs - ([6cf84e3](https://github.com/azais-corentin/cmakefmt/commit/6cf84e3f9cb1427cdd839a22a6eede37eb987e63))
- **documentation(spec):** remove dead options and tighten semantics - ([ae2de77](https://github.com/azais-corentin/cmakefmt/commit/ae2de771b4c5f291ada3f63b66decdcaa4d4f031))
- **documentation(spec):** tighten structure and resolve ambiguities - ([0fc5919](https://github.com/azais-corentin/cmakefmt/commit/0fc591978280926a4c5a7b925230bd08b653326e))
- **documentation(spec):** add input handling and configuration rules - ([a017cc3](https://github.com/azais-corentin/cmakefmt/commit/a017cc3435ae90754fba0c5b6d65af2c8d1d6cb5))
- **documentation(spec):** add initial specs document - ([4744d98](https://github.com/azais-corentin/cmakefmt/commit/4744d989ad794cb8e5b55c91f196e8232de33fcf))
- **documentation(repo):** rewrite AGENTS.md with actionable repository guidelines - ([bf521f0](https://github.com/azais-corentin/cmakefmt/commit/bf521f0a3f235104a44f1ebf3e11790c9b895733))
- **documentation(repo):** refresh repository guidance and feature status - ([d8c3b39](https://github.com/azais-corentin/cmakefmt/commit/d8c3b397430e91db92a64b668514f4f360776d7e))
- **documentation(repo):** add and evolve AGENTS contributor guidance - ([4d87b03](https://github.com/azais-corentin/cmakefmt/commit/4d87b03eb06e42211765cf5f36a6b8ca773f7137))
- **miscellaneous chores(release):** add custom changelog template prioritizing user-facing changes - ([89fb63b](https://github.com/azais-corentin/cmakefmt/commit/89fb63b26a3e53c8d6d5572c1e2b89fe972d146f))
- **miscellaneous chores:** reset package version to 0.0.0 - ([e64ce08](https://github.com/azais-corentin/cmakefmt/commit/e64ce08b79fe763033baff84573894b0da41df13))
- **miscellaneous chores:** remove redundant sed pre-bump hook from cog.toml - ([6787c10](https://github.com/azais-corentin/cmakefmt/commit/6787c10a56c453bffa38fb547020a955173f0c48))
- **miscellaneous chores:** add default formatter for GitHub Actions workflow files - ([895e1dc](https://github.com/azais-corentin/cmakefmt/commit/895e1dc75720d401412cc1e346a04e3abaafe5c0))
- **miscellaneous chores:** add synthetic benchmark fixture - ([d92c27a](https://github.com/azais-corentin/cmakefmt/commit/d92c27aa454a79d7fb0a77002c5b8043659078dd))
- **miscellaneous chores:** add .gitattributes to mark test fixtures as vendored - ([02261b1](https://github.com/azais-corentin/cmakefmt/commit/02261b1e3a8988947f5a9b916463be40d8961b31))
- **miscellaneous chores(dprint):** add malva CSS formatter plugin - ([717fd97](https://github.com/azais-corentin/cmakefmt/commit/717fd97bff55c6ef8bd88e00ef68c47f3d01457f))
- **miscellaneous chores(docs):** simplify vitepress script names - ([d49c1f8](https://github.com/azais-corentin/cmakefmt/commit/d49c1f804ed720264e1d4cc6228b858ffb9f5ba9))
- **miscellaneous chores(dprint):** add rustfmt formatter for Rust files - ([1a9be94](https://github.com/azais-corentin/cmakefmt/commit/1a9be9410c4e539d1518ee8ccf064b3b0093ddb5))
- **miscellaneous chores:** remove redundant fix fields from hk.pkl - ([7e40d58](https://github.com/azais-corentin/cmakefmt/commit/7e40d588e9f71a4ec3b41d36bd3201071e9f9c0a))
- **miscellaneous chores(vscode):** add pkl extension recommendation - ([aea17c1](https://github.com/azais-corentin/cmakefmt/commit/aea17c14f37f8466ae4113d4eccecc42720971d9))
- **miscellaneous chores:** add pkl formatting and exclude test fixtures from hygiene linters - ([ecaecdf](https://github.com/azais-corentin/cmakefmt/commit/ecaecdf0ab0f8a3515ace0dc5a8d214696c803d5))
- **miscellaneous chores(hk):** work around nix-fmt Windows no-op until jdx/hk#741 - ([6345af5](https://github.com/azais-corentin/cmakefmt/commit/6345af52fb1db27ac27c462ee65910bbff59a862))
- **miscellaneous chores(vscode):** recommend GitHub Actions extension - ([ccba205](https://github.com/azais-corentin/cmakefmt/commit/ccba205bd679d0068563bb353565c9cb89224de4))
- **miscellaneous chores:** add dev tooling (dprint, hk, vscode) - ([5f12fb0](https://github.com/azais-corentin/cmakefmt/commit/5f12fb0a7634d17200fa4525ecb7fedf96a27379))
- **miscellaneous chores:** release v0.1.1 - ([c4e1e23](https://github.com/azais-corentin/cmakefmt/commit/c4e1e23e174f49ff2afdef2010e4ab27f7bc34ce))
- **miscellaneous chores:** rename published crate to cmakefmt-rs - ([76b3875](https://github.com/azais-corentin/cmakefmt/commit/76b3875ac73e21be24f13de8a4d976775cb2f9d0))
- **miscellaneous chores(docs):** remove obsolete documentation files - ([7c19f0c](https://github.com/azais-corentin/cmakefmt/commit/7c19f0c958db69b31d5c663186a41f1a28e360d5))
- **miscellaneous chores:** add project tooling configuration - ([db316ab](https://github.com/azais-corentin/cmakefmt/commit/db316ab60aecbf06bccba2bc6094b75550b9d085))
- **miscellaneous chores:** update devenv lock - ([511dbc6](https://github.com/azais-corentin/cmakefmt/commit/511dbc6b8ef5b27e09f49084cbe8f275003e6a17))
- **miscellaneous chores:** add .omp to .gitignore - ([5ad9d50](https://github.com/azais-corentin/cmakefmt/commit/5ad9d50caa98d30f349238e577bd6571aff89384))
- **miscellaneous chores:** update .gitignore - ([379e6fc](https://github.com/azais-corentin/cmakefmt/commit/379e6fc54890d5096984da4994adb2b17319a7ef))
- **miscellaneous chores(project):** rename crate and binary to cmakefmt - ([c1951ec](https://github.com/azais-corentin/cmakefmt/commit/c1951ec70cca50f0d33be152747ac3dbebd16d36))
- **miscellaneous chores(devenv):** add local development environment configuration - ([056b87d](https://github.com/azais-corentin/cmakefmt/commit/056b87d26a111d9f3d76a8000e14fe7af6fc432b))
- **refactoring:** remove $schema configuration option - ([4d8afbc](https://github.com/azais-corentin/cmakefmt/commit/4d8afbc0a30394e07de442eac3fe100e66f37a08))
- **refactoring(mise):** restructure tasks with namespaced builds and filtered tests - ([294b665](https://github.com/azais-corentin/cmakefmt/commit/294b66502485a205426b6902c1d3441faec2bddd))
- **refactoring:** restructure into workspace, replace dprint-core with custom printer - ([4b0fd05](https://github.com/azais-corentin/cmakefmt/commit/4b0fd05aa9384297b264d49b12dff8d56a8e2ab2))
- **refactoring(tests):** add CMAKEFMT_TEST_FILTER and replace header parsing with pragmas - ([ac44150](https://github.com/azais-corentin/cmakefmt/commit/ac4415053c734ec3415302bf6293b6f287806bb1))
- **refactoring(configuration):** consolidate loading, diagnostics, and parsing - ([a4530a1](https://github.com/azais-corentin/cmakefmt/commit/a4530a1a66fa8b289cb8b74f145434e81027ee88))
- **refactoring(config):** centralize configuration default values - ([bc957ce](https://github.com/azais-corentin/cmakefmt/commit/bc957ce1a0494284fe449bb135d079dd49f8dd81))
- **revert(website):** use sequential commit indices as X axis for line charts - ([c5b1809](https://github.com/azais-corentin/cmakefmt/commit/c5b180961024b4f9d07a94a12f33bf44a68deea6))
- **style:** add trailing newline to .gitignore - ([900155b](https://github.com/azais-corentin/cmakefmt/commit/900155b8d7ab6f679936aed96fe8336d7b833af0))
- **style:** formatting cleanup - ([df8f5ef](https://github.com/azais-corentin/cmakefmt/commit/df8f5eff40f70dd04b8d7c93d3136de54c4c3635))
- **tests:** add dprint WASM plugin integration tests - ([cd8179d](https://github.com/azais-corentin/cmakefmt/commit/cd8179d22eecd362d4398ec56c8c73d5fbcea05b))
- **tests:** replace monolithic test with per-fixture rstest harness - ([a78888e](https://github.com/azais-corentin/cmakefmt/commit/a78888e9c6206c4fa8b35e53796500e6fd426ad3))
- **tests:** add compliance and XNNPACK formatter fixtures - ([a6a6492](https://github.com/azais-corentin/cmakefmt/commit/a6a6492cf57d7276024a312f3380880502e6dfd3))
- **tests:** add CPM repository formatter fixture - ([a9cf420](https://github.com/azais-corentin/cmakefmt/commit/a9cf42005fe4f57796ec042751d4be8437a55c2b))
- **tests(fixtures):** add missing coverage cases and refresh golden fixtures - ([10e78f1](https://github.com/azais-corentin/cmakefmt/commit/10e78f14fcf0dae768004f112cca1bb47c145bb8))
- **tests(fixtures):** add broad formatter fixture coverage - ([e2a2c05](https://github.com/azais-corentin/cmakefmt/commit/e2a2c05fed25649bf51cb0e2fc2e35badfa3183a))
- **tests(fixtures):** restore and expand formatter fixtures - ([f0c00df](https://github.com/azais-corentin/cmakefmt/commit/f0c00dfe7b776d8b955b60ec75585dd4d64520db))
- **tests(fixtures):** remove formatter fixtures - ([377fcc5](https://github.com/azais-corentin/cmakefmt/commit/377fcc59fa2e00c8fdd3ffe4bf2bf479382981cd))
- **tests(formatter):** update fmt fixture for spaceBeforeParen list - ([f5bad50](https://github.com/azais-corentin/cmakefmt/commit/f5bad504f9c313a2237c798a42ee40c181ba0765))
- **tests(fixtures):** add spdlog-utils and fmt fixtures - ([bc3a34b](https://github.com/azais-corentin/cmakefmt/commit/bc3a34b9d7c4f8f6aeedc97221c00fd125ebc6b3))
- **tests(fixtures):** add spdlog and glaze fixtures - ([5da1cab](https://github.com/azais-corentin/cmakefmt/commit/5da1caba09c65cf67989b917ef561fc14c8303a6))
- **tests(fixtures):** add mixed, nested, and repository CMakeLists cases - ([2552b75](https://github.com/azais-corentin/cmakefmt/commit/2552b7562bd023b81abe1a31aa51c48d91ec5621))
- **tests(harness):** visualize invisible characters in diffs - ([13e009d](https://github.com/azais-corentin/cmakefmt/commit/13e009d4782dabf824249786a0d2e6458b56e04e))
- **tests(harness):** expand formatter diff diagnostics and empty fixtures - ([5eadf13](https://github.com/azais-corentin/cmakefmt/commit/5eadf13940664c74753b44ad352a42ff9f5e2e7d))
- **tests(fixtures):** prune legacy corpus and enforce fixture pairing - ([4e8c22a](https://github.com/azais-corentin/cmakefmt/commit/4e8c22a89263d96f3da6635a7a0cc9c08ba4a929))
- **tests(harness):** add formatter fixture corpus and runner baseline - ([9389119](https://github.com/azais-corentin/cmakefmt/commit/9389119eb788aa3750de8dd12f8a729b3506b177))

</details>


- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
