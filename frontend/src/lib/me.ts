import { redirect } from "next/navigation";
import { authApi, Me, Shops } from "./api";

export async function getMe() {
    const response = await authApi("/staff/me");

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("ユーザー情報の取得に失敗しました");
    }

    return (await response.json()) as Me;
}

export async function getShops() {
    const response = await authApi("/staff/shops");

    if (response.status === 401 || response.status === 403) {
        redirect("/");
    }
    if (!response.ok) {
        throw new Error("屋台情報の取得に失敗しました");
    }

    return (await response.json()) as Shops;
}
