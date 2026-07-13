import { NextResponse } from "next/server";
import { registryURL } from "../../lib/cli";

export async function GET() {
  try {
    const res = await fetch(registryURL("/api/v1/packages"), {
      headers: { "User-Agent": "AtlasStudio/0.1" },
    });
    if (!res.ok) {
      return NextResponse.json({ error: "Registry unreachable" }, { status: 502 });
    }
    const data = await res.json();
    return NextResponse.json(data);
  } catch {
    return NextResponse.json({ error: "Registry unreachable" }, { status: 502 });
  }
}
