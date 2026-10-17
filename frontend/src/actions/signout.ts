"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { authApi } from "@/lib/api";

export async function signOut() {
    try {
        await authApi("/staff/signout", {
            method: "POST",
        });
    } finally {
        (await cookies()).delete("session");
    }

    redirect("/");
}
