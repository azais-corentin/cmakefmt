import { deflateSync, inflateSync } from "fflate";
import { onMounted, ref } from "vue";

// ---------------------------------------------------------------------------
// Encoding helpers
// ---------------------------------------------------------------------------

/**
 * standard base64 → base64url: swap chars that are unsafe in URLs/fragments,
 * strip `=` padding (RFC 4648 §5).
 */
function toBase64Url(b64: string): string {
  return b64.replace(/\+/g, "-").replace(/\//g, "_").replace(/=/g, "");
}

/**
 * base64url → standard base64, re-add padding so atob() is happy.
 */
function fromBase64Url(s: string): string {
  const b64 = s.replace(/-/g, "+").replace(/_/g, "/");
  const pad = b64.length % 4;
  return pad ? b64 + "=".repeat(4 - pad) : b64;
}

// ---------------------------------------------------------------------------
// Public encode / decode
// ---------------------------------------------------------------------------

export function encodeState(input: string, config: string): string {
  const json = JSON.stringify({ input, config });
  const bytes = new TextEncoder().encode(json);
  const compressed = deflateSync(bytes);
  // btoa operates on binary strings; convert Uint8Array → binary string first
  let binary = "";
  for (let i = 0; i < compressed.length; i++) {
    binary += String.fromCharCode(compressed[i]);
  }
  return toBase64Url(btoa(binary));
}

export function decodeState(hash: string): { input: string; config: string } | null {
  if (!hash) return null;
  try {
    const b64 = fromBase64Url(hash);
    const binary = atob(b64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i);
    }
    const decompressed = inflateSync(bytes);
    const json = new TextDecoder().decode(decompressed);
    const parsed: unknown = JSON.parse(json);
    if (
      parsed !== null
      && typeof parsed === "object"
      && "input" in parsed
      && "config" in parsed
      && typeof (parsed as Record<string, unknown>).input === "string"
      && typeof (parsed as Record<string, unknown>).config === "string"
    ) {
      return parsed as { input: string; config: string };
    }
    return null;
  } catch {
    return null;
  }
}

// ---------------------------------------------------------------------------
// Composable
// ---------------------------------------------------------------------------

export function useUrlState() {
  const initialInput = ref<string | null>(null);
  const initialConfig = ref<string | null>(null);

  onMounted(() => {
    // strip the leading '#'
    const raw = window.location.hash.slice(1);
    const decoded = decodeState(raw);
    if (decoded) {
      initialInput.value = decoded.input;
      initialConfig.value = decoded.config;
    }
  });

  /**
   * Encode current editor state into the URL fragment without triggering
   * a navigation or a history entry push.
   */
  function updateHash(input: string, config: string): void {
    const encoded = encodeState(input, config);
    history.replaceState(null, "", `#${encoded}`);
  }

  return { initialInput, initialConfig, updateHash };
}
