import "server-only";
import { cookies } from "next/headers";

export async function serverApi(
    path: string,
    init: RequestInit = {},
) {
    const token = (await cookies()).get("session")?.value;

    console.log("[serverApi] session cookie exists:", token);

    if (!token) {
        throw new Error("Unauthorized");
    }

    const headers = new Headers(init.headers);
    headers.set("Authorization", `Bearer ${token}`);

    const response = await fetch(`${process.env.BACKEND_URL}${path}`, {
        ...init,
        headers,
        cache: "no-store",
    });

    console.log(`[serverApi] ${path}: ${response.status}`);

    return response;
}
