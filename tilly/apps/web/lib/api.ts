const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";

export async function getApiHealth(): Promise<string> {
  try {
    const response = await fetch(`${API_URL}/health`, { cache: "no-store" });
    if (!response.ok) {
      return "offline";
    }

    const body = (await response.json()) as { status?: string };
    return body.status ?? "unknown";
  } catch {
    return "offline";
  }
}
