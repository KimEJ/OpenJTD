#!/usr/bin/env ts-node
/**
 * JTD → Lawtext converter
 *
 * Usage:
 *   rjtd export FILE.jtd --format json | npx ts-node jtd2lawtext.ts
 *   npx ts-node jtd2lawtext.ts FILE.json
 *   npx ts-node jtd2lawtext.ts FILE.json > FILE.law.txt
 *
 * Reads JSON produced by `rjtd export --format json` and outputs Lawtext
 * (https://github.com/yamachig/lawtext) compatible plain text.
 *
 * Classification is heuristic because rjtd does not yet decode paragraph-style
 * IDs (TODO 459/465). Text patterns and full-width space indent depth are used.
 */

import * as fs from "fs";
import * as readline from "readline";

// -------------------------------------------------------------------------- //
// Types mirroring rjtd JSON export schema
// -------------------------------------------------------------------------- //

interface Inline {
  type: string;
  text?: string;
}

interface Block {
  type: string;
  style: unknown;
  inlines: Inline[];
}

interface ExportDoc {
  blocks: Block[];
}

// -------------------------------------------------------------------------- //
// Patterns
// -------------------------------------------------------------------------- //

const LAW_NUM_RE   = /^(政令|勅令|府令|省令|規則|条例|告示|訓令|通達)第\s*[〇一二三四五六七八九十百千万０-９0-9]+号/;
const ARTICLE_RE   = /^第[〇一二三四五六七八九十百千万]+条([　\s]|$)/;
const ARTICLE_CAP_RE = /^（.+）\s*$/;
const SUPP_PROV_RE = /^附[　\s]*則/;
const ITEM_RE      = /^[一二三四五六七八九十]+[　\s]/;
const PARA_NUM_RE  = /^[２３４５６７８９0-9]+[　\s]/;
const REMARKS_RE   = /^[　\s]{2,}理[　\s]*由/;
const BLANK_RE     = /^[　\s]*$/;
const CHAPTER_RE   = /^第[〇一二三四五六七八九十百千万]+章/;
const SECTION_RE   = /^第[〇一二三四五六七八九十百千万]+節/;
const TABLE_RE     = /^別表/;

// -------------------------------------------------------------------------- //
// Helpers
// -------------------------------------------------------------------------- //

function extractText(block: Block): string {
  return block.inlines
    .filter(i => i.type === "text" && i.text !== undefined)
    .map(i => i.text!)
    .join("");
}

function indentDepth(text: string): number {
  let count = 0;
  for (const ch of text) {
    if (ch === "　" || ch === " ") count++;
    else break;
  }
  return count;
}

function stripIndent(text: string): string {
  return text.replace(/^[　 ]+/, "");
}

// -------------------------------------------------------------------------- //
// Classifier
// -------------------------------------------------------------------------- //

type Tag =
  | "LawNum" | "LawTitle" | "Chapter" | "Section"
  | "Article" | "ArticleCaption" | "Paragraph" | "Item"
  | "Sentence" | "SupplProvision" | "Remarks" | "TableStruct"
  | "OutlineItem" | "blank";

class Classifier {
  private titleDone = false;
  private inSupplProv = false;

  classify(text: string, depth: number): [Tag, string] {
    const s = stripIndent(text);

    if (BLANK_RE.test(text)) return ["blank", ""];

    if (!this.titleDone) {
      if (LAW_NUM_RE.test(s)) {
        this.titleDone = true;
        return ["LawNum", s];
      }
      if (depth === 0 && !ARTICLE_RE.test(s) && !CHAPTER_RE.test(s)) {
        this.titleDone = true;
        return ["LawTitle", s];
      }
    }

    if (SUPP_PROV_RE.test(s)) {
      this.inSupplProv = true;
      return ["SupplProvision", s];
    }

    if (REMARKS_RE.test(text)) return ["Remarks", s];
    if (CHAPTER_RE.test(s))    return ["Chapter", s];
    if (SECTION_RE.test(s))    return ["Section", s];
    if (TABLE_RE.test(s))      return ["TableStruct", s];
    if (ARTICLE_RE.test(s))    return ["Article", s];

    if (ARTICLE_CAP_RE.test(s) && depth === 0) return ["ArticleCaption", s];
    if (PARA_NUM_RE.test(s) && depth <= 1)     return ["Paragraph", s];
    if (ITEM_RE.test(s) && depth >= 1)         return ["Item", s];
    if (depth === 0)                            return ["Paragraph", s];

    return ["Sentence", s];
  }
}

// -------------------------------------------------------------------------- //
// Formatter
// -------------------------------------------------------------------------- //

const INDENT: Record<Tag, string> = {
  LawTitle:       "",
  LawNum:         "",
  Chapter:        "",
  Section:        "  ",
  Article:        "",
  ArticleCaption: "  ",
  Paragraph:      "  ",
  Item:           "    ",
  Sentence:       "    ",
  SupplProvision: "",
  Remarks:        "      ",
  TableStruct:    "",
  OutlineItem:    "",
  blank:          "",
};

const NEEDS_BLANK_BEFORE = new Set<Tag>([
  "Chapter", "Section", "Article", "SupplProvision", "TableStruct",
]);

function formatLine(tag: Tag, text: string, prevTag: Tag | null): string {
  if (tag === "blank") return "";

  const line = INDENT[tag] + text;

  const sep =
    prevTag !== null &&
    NEEDS_BLANK_BEFORE.has(tag) &&
    !(tag === "ArticleCaption" && prevTag === "Article") &&
    !(tag === "Paragraph" && (prevTag === "Article" || prevTag === "ArticleCaption"))
      ? "\n"
      : "";

  return sep + line;
}

// -------------------------------------------------------------------------- //
// Conversion
// -------------------------------------------------------------------------- //

function convert(data: ExportDoc): string {
  const classifier = new Classifier();
  const lines: string[] = [];
  let prevTag: Tag | null = null;

  for (const block of data.blocks) {
    const text = extractText(block);
    const depth = indentDepth(text);
    const [tag, body] = classifier.classify(text, depth);

    const formatted = formatLine(tag, body, prevTag);
    if (formatted) lines.push(formatted);

    prevTag = tag;
  }

  return lines.join("\n");
}

// -------------------------------------------------------------------------- //
// Entry point
// -------------------------------------------------------------------------- //

async function readStdin(): Promise<string> {
  const chunks: string[] = [];
  const rl = readline.createInterface({ input: process.stdin });
  for await (const line of rl) chunks.push(line);
  return chunks.join("\n");
}

async function main(): Promise<void> {
  let raw: string;

  if (process.argv[2]) {
    raw = fs.readFileSync(process.argv[2], "utf-8");
  } else {
    raw = await readStdin();
  }

  const data: ExportDoc = JSON.parse(raw);
  process.stdout.write(convert(data) + "\n");
}

main().catch(e => { console.error(e); process.exit(1); });
