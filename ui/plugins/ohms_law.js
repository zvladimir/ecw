import { html, useEffect, useState } from "../lib/preact.js";
import { t } from "../lib/i18n.js";

/** The picture is as tall as the form next to it, whatever the width of the window. */
function Formulas({ formRef }) {
  const [height, setHeight] = useState(0);
  useEffect(() => {
    const observer = new ResizeObserver(([entry]) => setHeight(Math.round(entry.contentRect.height)));
    observer.observe(formRef.current);
    return () => observer.disconnect();
  }, []);
  return html`
    <div class="ohm-pic">
      <img src="img/ohm_law.png" alt=${t("Ohm's law formulas")} style=${{ height: height ? `${height}px` : "auto" }} />
    </div>`;
}

export default {
  id: "ohms_law",
  title: "Ohm's Law",
  category: "Basic",
  doc: "U = I · R",
  page: "form",
  command: "ohms_law",
  storeKey: "ecw.ohms",
  side: ({ formRef }) => html`<${Formulas} formRef=${formRef} />`,
};
