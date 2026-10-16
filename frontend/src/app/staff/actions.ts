"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { serverApi } from "@/lib/api";

export async function signOut() {
    try {
        await serverApi("/staff/signout", {
            method: "POST",
        });
    } finally {
        (await cookies()).delete("session");
    }

    redirect("/");
}
