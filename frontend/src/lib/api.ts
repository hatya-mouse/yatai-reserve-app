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

export type Member = {
    name: string;
    email: string;
    is_admin: boolean | null;
};

export type Members = {
    members: Member[];
};

export async function staffApi(path: string, init: RequestInit = {}) {
    const token = (await cookies()).get("session")?.value;

    if (!token) {
        redirect("/");
    }

    const headers = new Headers(init.headers);
    headers.set("Authorization", `Bearer ${token}`);

    const response = await fetchBackend(path, {
        ...init,
        headers,
        cache: "no-store",
    });

    console.log(`[staffApi] ${path}: ${response.status}`);

    return response;
}

export async function fetchBackend(path: string, init: RequestInit = {}) {
    return await fetch(`${process.env.BACKEND_URL}${path}`, {
        ...init,
        cache: "no-store",
    });
}
