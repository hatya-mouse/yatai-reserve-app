import { NextRequest, NextResponse } from "next/server";

export async function GET(request: NextRequest) {
    const token = request.nextUrl.searchParams.get("token");

    if (!token) {
        return NextResponse.redirect(
            new URL("/?error=missing_token", request.url),
        );
    }

    try {
        const response = await fetch(
            `${process.env.BACKEND_URL}/verify?token=${encodeURIComponent(token)}`,
            { cache: "no-store" },
        );

        if (!response.ok) {
            return NextResponse.redirect(
                new URL("/?error=verification_failed", request.url),
            );
        }

        const data: { token: string } = await response.json();

        if (!data.token) {
            throw new Error("Missing session token");
        }

        const redirectResponse = NextResponse.redirect(
            new URL("/staff", request.url),
        );

        redirectResponse.cookies.set("session", data.token, {
            httpOnly: true,
            secure: process.env.NODE_ENV === "production",
            sameSite: "lax",
            path: "/",
            maxAge: 60 * 60 * 24 * 7,
        });

        return redirectResponse;
    } catch {
        return NextResponse.redirect(
            new URL("/?error=verification_failed", request.url),
        );
    }
}
