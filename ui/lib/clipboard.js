import { t } from "./i18n.js";
import { toast } from "./toast.js";

async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    const area = document.createElement("textarea");
    area.value = text;
    document.body.append(area);
    area.select();
    document.execCommand("copy");
    area.remove();
  }
}

/** Copies a table as tab-separated text and shows a toast. */
export async function copyTable(header, rows, message = t("Table copied")) {
  const tsv = [header, ...rows].filter((r) => r.length).map((r) => r.join("\t")).join("\n");
  await copyText(tsv);
  toast(message);
}
