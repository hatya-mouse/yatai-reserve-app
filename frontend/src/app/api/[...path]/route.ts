import { NextRequest } from "next/server";

const BACKEND_URL = process.env.BACKEND_URL;

const ALLOWED_METHODS = ["GET", "POST", "PATCH", "DELETE"] as const;

async function proxy(request: NextRequest): Promise<Response> {
    if (!BACKEND_URL) {
        return Response.json(
            { error: "Backend is not configured" },
            { status: 500 },
        );
    }

    const path = request.nextUrl.pathname.replace(/^\/api\//, "");

    // Keep verification separate because it handles session tokens.
    if (path === "verify") {
        return Response.json(
            { error: "Use the dedicated verification endpoint" },
            { status: 404 },
        );
    }

    const backendBase = BACKEND_URL.replace(/\/+$/, "");
    const targetUrl = `${backendBase}/${path}${request.nextUrl.search}`;

    const headers = new Headers();
    const contentType = request.headers.get("content-type");

    if (contentType) {
        headers.set("content-type", contentType);
    }

    if (path.startsWith("staff/")) {
        const session = request.cookies.get("session");

        if (session) {
            headers.set("cookie", `session=${session.value}`);
        }
    }

    const method = request.method;
    const body =
        method === "GET" || method === "HEAD"
            ? undefined
            : await request.arrayBuffer();

    let upstream: Response;

    try {
        upstream = await fetch(targetUrl, {
            method,
            headers,
            body,
            cache: "no-store",
            redirect: "manual",
        });
    } catch {
        return Response.json({ error: "Backend unavailable" }, { status: 502 });
    }

    const responseHeaders = new Headers();

    for (const name of [
        "content-type",
        "cache-control",
        "content-disposition",
    ]) {
        const value = upstream.headers.get(name);

        if (value) {
            responseHeaders.set(name, value);
        }
    }

    return new Response(
        method === "HEAD" || upstream.status === 204 || upstream.status === 304
            ? null
            : upstream.body,
        {
            status: upstream.status,
            headers: responseHeaders,
        },
    );
}

export const GET = proxy;
export const POST = proxy;
export const PATCH = proxy;
export const DELETE = proxy;
