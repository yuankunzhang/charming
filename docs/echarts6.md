# Migrating to ECharts 6.1

Charming uses ECharts **6.1.0** and ECharts GL **2.1.0**. The embedded engine,
generated HTML and repository examples use the same versions. Charts adopt the
v6 default palette, component placement and label layout. No v5 theme is loaded
automatically.

## Rust and saved JSON

`Chart::calendar` now appends calendars. JSON always contains an array, even for
one calendar; saved single-object calendars must be changed to arrays before
reading them with Serde.

```rust
use charming::{Chart, component::Calendar};

let chart = Chart::new()
    .calendar(Calendar::new().range("2026-11"))
    .calendar(Calendar::new().range("2026-12"));
// {"calendar":[{"range":"2026-11"},{"range":"2026-12"}]}
```

`Custom::render_item` strings now name a registered renderer. Wrap executable
source in `JsFunction`; strings are never inspected to guess whether they contain
code. `RenderItem` exposes the `Name` and `Function` variants.

```rust
use charming::{element::JsFunction, series::Custom};

let named = Custom::new().render_item("violin");
let function = Custom::new().render_item(JsFunction::from(
    "function(params, api) { return {type:'circle', shape:{cx:100,cy:100,r:20}}; }"
));
```

Names serialize as strings. Functions retain
`{"type":"function","source":"..."}`. `item_payload` accepts a JSON object;
its strings and nested objects remain data. Custom drawing features such as
`compoundPath`, `tooltipDisabled` and `api.layout` are available inside explicit
JavaScript callbacks.

`Map` now emits `type: "map"` and has map/geo selectors and `MapData` records.
`Aria::label` takes `AriaLabel`, with description, series and data wording;
ordinary visual `Label` is no longer accepted there. `AriaData` includes
`exclude_dimension_id`. Update exhaustive matches for added series and union
variants. Roaming accepts `Roam`, and aspect preservation accepts `PreserveAspect`;
their builders also accept booleans. These changes do not add readers for old
callback formats or the old calendar shape.

## Configuration entry points

| Capability | Rust entry points |
| --- | --- |
| Chord diagrams | `Chord`, `ChordNode`, `ChordEdge`, node/edge states and labels |
| Matrix layout | `Chart::matrix`, `MatrixDimension`, hierarchical cells, `MatrixCells`, merged `MatrixCell` ranges, `length`, tooltip and `trigger_event` |
| Embedded charts | `coordinate_system`, `coordinate_system_usage`, `coord` and matrix/calendar selectors on applicable components and series; place Line/Bar in a nested `Grid` |
| Maps and navigation | `Chart::geo`, `Map`, `GeoRegion`, `Lines`/`LinesData::coords`, `Thumbnail` and its `window_style` |
| Timeline and brushing | `Chart::timeline`, appended `Chart::options`, boxed `Chart::base_option`, `BrushComponent` |
| Axes | `AxisBreak`, `BreakArea`, `BreakLabelLayout`, `AxisLine::break_line`, jitter, outer bounds, `contain_shape`, `data_min`/`data_max`, split-line endpoints and `AxisTooltip` |
| Text and interaction | Time formatter dictionaries, `rich_inherit_plain_label`, text margins/overflow, global `text_style`, tooltip transition and mousewheel triggers, `HandleLabel` and grab cursors |
| Series controls | `LineTriggerEvent`, `StackOrder`, radar direction, pie tangential rotation, candlestick/treemap cursors, Sunburst relative focus, map-region silence, clipping, roaming and marker positioning |

Time formatter dictionaries use `Formatter::TimeDictionary` with `TimeUnit`
keys, `TimeFormatterEntry` for direct or upper-unit entries, and
`TimeFormatterValue` for strings or level lists. Visual maps support
`unbounded_range`, series IDs and per-series dimension targets. Gauge progress
color `"auto"` works through the existing item style. Upstream rendering and
layout fixes come from the upgraded engine.

The [gallery examples](../gallery/src/echarts6.rs) build a matrix heatmap, chord,
violin and bar range chart. They use ordinary builders and can be saved and read
back as Chart JSON.

## Registering extensions, maps and locales

Load the engine first, then register resources, then create the chart:

| Renderer | Registration |
| --- | --- |
| HTML | Chain `HtmlRenderer::extension_source(source)` before `render`; sources run in insertion order |
| Native SVG/PNG | Call `ImageRenderer::register_extension(source)?` before rendering |
| WASM | Call `WasmRenderer::register_extension(source)?` before rendering |

These APIs execute trusted JavaScript. HTML encodes source as an HTML-safe JSON
string and uses global indirect `eval`; pages that register source must allow
that execution. WASM also uses `eval`. Synchronous native/WASM exceptions become
`JsRuntimeError`/`WasmError`; HTML reports execution errors in the browser.
Registration has no rollback. Native extensions must work without a DOM.

The gallery pins the official UMD packages `@echarts-x/custom-violin` **1.1.1**
and `@echarts-x/custom-bar-range` **1.2.1**. Each requires its installer after
loading the source:

```javascript
echarts.use(violinCustomSeriesInstaller);
echarts.use(barRangeCustomSeriesInstaller);
```

The library does not register these extensions by default. The `.auto.js`
variants depend on `window.echarts`; use the UMD installers for native rendering.
Bar range data uses `encode: {x: 0, y: [1, 2], tooltip: [1, 2]}`. See the
[official custom-series project](https://github.com/apache/echarts-custom-series)
for other renderers using the same registration interface.

Register GeoJSON or SVG maps with `echarts.registerMap` in an explicit source.
`GeoRegion` also expresses a GeoJSON feature's `properties.echartsStyle`.
Register extra languages with `echarts.registerLocale`, then use `.locale("LV")`
or the chosen name on any renderer. Additional languages are not bundled.
Resource registration belongs to the renderer environment, not saved Chart JSON.

## Themes and browser instances

HTML and WASM register the source for bundled/custom themes before `init` and
pass the selected theme and locale to the engine. For these renderers, a toolbox
`SaveAsImageType::Svg` selects the SVG painter; otherwise they use canvas. Native
rendering uses SVG internally, including PNG conversion.

ECharts 6.1 reconstructs options when changing theme. Keep the complete current
Chart in application state and replace it immediately before every theme change:

```rust,ignore
WasmRenderer::replace(&instance, &current_chart)?;
WasmRenderer::set_theme(&instance, Theme::Dark, true)?;
// To return to the v6 default:
WasmRenderer::replace(&instance, &current_chart)?;
WasmRenderer::set_theme(&instance, Theme::Default, true)?;
```

`update` continues to merge options; merge-only updates are not guaranteed to
survive a theme change. `replace` uses `notMerge: true`. `set_theme`'s final
argument is `silent`; `Theme::Default` selects ECharts' `default` theme name.

`convert_to_layout`, `convert_to_pixel` and `convert_from_pixel` take the
instance, JavaScript finder/value, and optional JavaScript options, returning
`Result<JsValue, EchartsError>`. `dispatch_action` takes a JavaScript payload.
`on` and `off` take an event name and a borrowed `js_sys::Function`. Retain the
Rust `Closure` while subscribed, and pass the same function to `off` before
dropping it. There is no subscription registry or query/context overload.

## Engine behavior to review

- `tooltip.valueFormatter`'s second argument is the original input data index,
  including when DataZoom filters the series.
- `axis.startValue` no longer supplies an implicit `min`. Set both when both
  constraints are intended.
- `axis.containShape` keeps edge bars, candlesticks, pictorial bars and boxplots
  within the grid. Check deliberately overflowing layouts.

These changes are described in the [6.1.0 release notes](https://github.com/apache/echarts/releases/tag/6.1.0).

Axis labels and names now avoid overflow and overlap by default. Rich text
inherits font and text-shadow settings from the surrounding plain label. These
can change spacing even when data is identical. See the
[v6 upgrade guide](https://echarts.apache.org/handbook/en/basics/release-note/v6-upgrade-guide/)
for the upstream options.

For view-based roaming, percentage `center` values refer to the data bounding
rectangle's width and height; that point is mapped to the center of the display
rectangle. Review percentage centers in graph, tree, sankey and map layouts.
The [fixed-version definition](https://github.com/apache/echarts/blob/6.1.0/src/coord/View.ts#L107-L121)
explains the coordinate spaces.

## Validation scope

The manual acceptance tasks cover locked Chromium, software WebGL, complete HTML
pages loading the fixed CDN, and native SVG-to-PNG rendering on Linux. Existing
SSR snapshots retain their pixel comparison assertions with reviewed v6 images.
Gallery examples with confirmed title, legend or axis collisions set explicit
spacing, centers, radii or label sizes; their data and v6 colors are preserved.
Random jitter is checked in the browser and excluded from deterministic image
baselines. Windows/macOS, other browsers, Leptos hydration and the complete Cargo
feature matrix are outside this acceptance scope. Commands and CI boundaries
are in [CONTRIBUTING.md](../CONTRIBUTING.md).
