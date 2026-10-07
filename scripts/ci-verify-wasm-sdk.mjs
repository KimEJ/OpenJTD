#!/usr/bin/env node
import assert from 'node:assert/strict';
import { cp, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const sdk = process.argv[2];
assert(sdk, 'usage: node scripts/ci-verify-wasm-sdk.mjs <generated-sdk-dir> [native-document]');
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const temporary = await mkdtemp(join(tmpdir(), 'rjtd-sdk-contract-'));
try {
  await cp(resolve(sdk), join(temporary, 'pkg'), { recursive: true });
  await cp(join(root, 'openjtd.github.io/rjtd.mjs'), join(temporary, 'rjtd.mjs'));
  const packagePath = join(temporary, 'pkg/package.json');
  const metadata = JSON.parse(await readFile(packagePath, 'utf8'));
  await writeFile(packagePath, JSON.stringify({ ...metadata, type: 'module' }));
  const api = await import(pathToFileURL(join(temporary, 'rjtd.mjs')));
  await api.default({ module_or_path: await readFile(join(temporary, 'pkg/rjtd_wasm_bg.wasm')) });
  assert.equal(api.JtdDocument, api.HwpDocument, 'canonical and legacy JS constructors must be identical');
  const document = api.JtdDocument.createEmpty();
  try {
    assert(document instanceof api.HwpDocument);
    document.setFileName('contract.jtd');
    document.setPrintDate('2026/10/07');
    assert.equal(document.pageCount(), 1);
    assert.equal(document.plainText(), '\n');
    assert.equal(document.getSourceFormat(), 'jtd');
    assert.equal(JSON.parse(document.getDocumentInfo()).fileName, 'contract.jtd');
    assert.match(document.renderPageSvg(0), /^<svg /);
    assert.match(document.renderPageSvgWithTextWidths(0, new Uint32Array(), new Float32Array()), /^<svg /);
    assert.throws(() => document.setPrintDate('2026/02/30'), /valid YYYY\/MM\/DD/);
    assert.equal(document.exportHwp().length, 0);
    assert.equal(document.exportHwpx().length, 0);
    assert.equal(JSON.parse(document.exportHwpVerify()).ok, false);
  } finally { document.free(); }
  assert.throws(() => new api.JtdDocument(new Uint8Array([0])), /.+/);
  if (process.argv[3]) {
    const bytes = await readFile(process.argv[3]);
    const document = new api.JtdDocument(bytes);
    try {
      assert(document instanceof api.HwpDocument);
      assert(document.pageCount() > 0);
      assert(document.plainText().length > 0);
      for (let page = 0; page < document.pageCount(); page++) assert.match(document.renderPageSvg(page), /^<svg /);
    } finally { document.free(); }
  }
  console.log('WASM SDK: canonical/legacy identity, viewer calls, errors and retained unsupported exports passed.');
} finally { await rm(temporary, { recursive: true, force: true }); }
