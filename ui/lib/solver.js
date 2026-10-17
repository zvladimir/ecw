import { useEffect, useState } from "./preact.js";
import { invoke } from "./backend.js";
import { useSettings } from "./settings.js";

/**
 * Sends `request` (plus the decimals setting) to a backend command whenever it changes.
 * Returns the latest reply (`null` until the first one); replies to outdated requests are dropped.
 */
export function useSolver(command, request) {
  const { settings } = useSettings();
  const full = { ...request, decimals: settings.decimals };
  const key = JSON.stringify(full);
  const [resp, setResp] = useState(null);

  useEffect(() => {
    let live = true;
    invoke("solve", { id: command, request: full }).then((r) => live && setResp(r));
    return () => { live = false; };
  }, [command, key]);

  return resp;
}
