# Changelog

## 0.6.0
- Add deserialization support [#172](https://github.com/yuankunzhang/charming/pull/172)
- Add a custom derive macro to reduce internal code [#181](https://github.com/yuankunzhang/charming/pull/181)
- Add a sycamore example [#184](https://github.com/yuankunzhang/charming/pull/184)
- Update CI to check for wasm builds [#185](https://github.com/yuankunzhang/charming/pull/185)
- Replace RawString with JsFunction [#186](https://github.com/yuankunzhang/charming/pull/186)
- Update docs (especially user facing macros) [#188](https://github.com/yuankunzhang/charming/pull/188)
- Add devenv - Nix developer environment [#191](https://github.com/yuankunzhang/charming/pull/191)
- Add calendar support [#192](https://github.com/yuankunzhang/charming/pull/192)
- Update dependencies and many more small changes

### Breaking changes
- Name change in GeoMap [#181](https://github.com/yuankunzhang/charming/pull/181)
- Change GraphNodeModel FontSize type [#181](https://github.com/yuankunzhang/charming/pull/181)
- Change dataset_index type [#181](https://github.com/yuankunzhang/charming/pull/181)
- More implicit call of `.into` [#181](https://github.com/yuankunzhang/charming/pull/181)
- Remove FormatterFunction [#186](https://github.com/yuankunzhang/charming/pull/186)
- Replace RawString with JsFunction in components [#186](https://github.com/yuankunzhang/charming/pull/186)

## 0.5.1
- Updated leptos example to fix warnings [#173](https://github.com/yuankunzhang/charming/pull/173)
- Updated image generation to include themes [#174](https://github.com/yuankunzhang/charming/pull/174)
- Fix nightly clippy lints [#175](https://github.com/yuankunzhang/charming/pull/175)
- Updated README.md to show images on crates.io [#176](https://github.com/yuankunzhang/charming/pull/176)
- Added image snapshot testing [#177](https://github.com/yuankunzhang/charming/pull/177)

## 0.5.0
- Start of Changelog
- Deriving a lot of common functionality (Debug, Clone, etc.) [#86](https://github.com/yuankunzhang/charming/pull/86)
- The wonderful dz! macro [#100](https://github.com/yuankunzhang/charming/pull/100)
- Some new fields, e.g. Label for Scatter plot, ItemSyle for BoxPlot, MarkLine's precision, Tooltips [#96](https://github.com/yuankunzhang/charming/pull/96), [#99](https://github.com/yuankunzhang/charming/pull/99), [#102](https://github.com/yuankunzhang/charming/pull/102)
- Updated the leptos example to the latest stable version [#103](https://github.com/yuankunzhang/charming/pull/103)
- Upgraded to echarts 5.5.1 [#122](https://github.com/yuankunzhang/charming/pull/122)
- Added custome font setting [#112](https://github.com/yuankunzhang/charming/pull/112)
- Fixed some serialization bugs [#97](https://github.com/yuankunzhang/charming/pull/97)
- Fixed a bug where the ssr renderer did not work [#88](https://github.com/yuankunzhang/charming/pull/88)
- Added startValue to axis types [#123](https://github.com/yuankunzhang/charming/pull/123)
- Added step line support [#107](https://github.com/yuankunzhang/charming/pull/107)
- Updated dependencies [#153](https://github.com/yuankunzhang/charming/pull/153)
- Added animation fields to chart [#162](https://github.com/yuankunzhang/charming/pull/162) [#165](https://github.com/yuankunzhang/charming/pull/165)
- Added sampling configuration [#166](https://github.com/yuankunzhang/charming/pull/166)
- Updated dioxus examples [#149](https://github.com/yuankunzhang/charming/pull/149) [#150](https://github.com/yuankunzhang/charming/pull/150)
- Updated leptos examples [#134](https://github.com/yuankunzhang/charming/pull/134)
- Many more smaller changes

### Breaking changes
- Enable the use of formatter in wasm [#148](https://github.com/yuankunzhang/charming/pull/148)
- Disable default image import with ssr feature [#154](https://github.com/yuankunzhang/charming/pull/154)
