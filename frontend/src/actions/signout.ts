"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { staffApi } from "@/lib/api";

export async function signOut() {
    try {
        await staffApi("/staff/signout", {
            method: "POST",
        });
    } finally {
        (await cookies()).delete("session");
    }

    redirect("/");
}
