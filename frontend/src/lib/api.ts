import "server-only";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export type Shop = {
    id: string;
    name: string;
    notify_previous: boolean;
};

export type Shops = {
    shops: Shop[];
};

export type Me = {
    name: string;
    email: string;
};

export async function staffApi(path: string, init: RequestInit = {}) {
    const token = (await cookies()).get("session")?.value;

    if (!token) {
        redirect("/");
    }

    const headers = new Headers(init.headers);
    headers.set("Authorization", `Bearer ${token}`);

    const response = await fetch(`${process.env.BACKEND_URL}${path}`, {
        ...init,
        headers,
        cache: "no-store",
    });

    console.log(`[staffApi] ${path}: ${response.status}`);

    return response;
}
