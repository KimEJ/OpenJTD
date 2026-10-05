import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';

const html = fs.readFileSync(new URL('../openjtd.github.io/index.html', import.meta.url), 'utf8');
const render = html.slice(html.indexOf('function renderPage()'), html.indexOf('function showText()'));

async function check(stale) {
  let finish, applied;
  const fonts = new Promise(resolve => { finish = resolve; });
  const runs = [117, 222].map((unit, index) => ({
    dataset: { sourceUnitStart: String(unit) },
    getComputedTextLength: () => 120 + index * 20,
  }));
  const container = { innerHTML: '', querySelectorAll: () => runs };
  const doc = {
    renderPageSvg: () => '<svg/>',
    renderPageSvgWithTextWidths: (page, units, widths) => {
      applied = [page, [...units], [...widths]];
      return '<svg measured/>';
    },
  };
  const context = {
    doc, currentPage: 0, totalPages: 1, currentTab: 'svg',
    Uint32Array, Float32Array, Number, setStatus: () => {},
    document: { fonts: { ready: fonts }, getElementById: id => id === 'svg-container' ? container : {} },
  };
  vm.createContext(context);
  vm.runInContext(`${render}; renderPage();`, context);
  if (stale) context.doc = null;
  finish();
  await new Promise(resolve => setImmediate(resolve));
  assert.deepEqual(applied, stale ? undefined : [0, [117, 222], [120, 140]]);
}

await check(false);
await check(true);
console.log('Viewer font measurements and stale-document guard passed');
