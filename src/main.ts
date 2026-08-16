import init, { main } from "../pkg/rusticon.js";

/**
 * Dynamically adjusts the mobile viewport meta tag scale factor to force
 * the terminal grid layout to zoom out and fit smaller device widths.
 * The column count is read from the meta tag's data-min-width attribute
 * (set by an inline script in <head> before any paint).
 */
function syncViewportBounds(): void {
  const meta = document.getElementById('viewport-meta') as HTMLMetaElement | null;
  if (!meta) return;

  const cols = parseInt(meta.dataset.minWidth ?? '', 10);
  const rows = parseInt(meta.dataset.minHeight ?? '', 10);
  if (!cols && !rows) return;

  const LINE_HEIGHT = 18;
  const JETBRAINS_MONO_RATIO = 600 / 1000;
  const CHAR_WIDTH = LINE_HEIGHT * JETBRAINS_MONO_RATIO;

  let scale = 1;

  if (cols) {
    const content_width = cols * CHAR_WIDTH;
    const isLandscape = window.matchMedia("(orientation: landscape)").matches;
    const device_width = isLandscape
      ? Math.max(window.screen.width, window.screen.height)
      : Math.min(window.screen.width, window.screen.height);
    if (content_width > device_width) scale = Math.min(scale, device_width / content_width);
  }

  if (rows) {
    const content_height = rows * LINE_HEIGHT;
    const device_height = window.screen.height;
    if (content_height > device_height) scale = Math.min(scale, device_height / content_height);
  }

  meta.setAttribute(
    'content',
    `width=device-width, initial-scale=${scale}, minimum-scale=${scale}, maximum-scale=5.0, user-scalable=yes`
  );
}

await init();

// Attach listeners for orientation flips and window adjustments
window.addEventListener('resize', syncViewportBounds);
window.addEventListener('orientationchange', syncViewportBounds);

main();

