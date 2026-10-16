"use client";

import { useState } from "react";
import { redirect } from "next/navigation";
import { Button } from "@/components/ui/button";
import { signOut } from "@/lib/signout";
import { getMe } from "@/lib/me";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Checkbox } from "@/components/ui/checkbox";
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field";
import Link from "next/link";
import { NewShopForm } from "./form";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

export default async function NewShopPage() {
    const me = await getMe();

    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">屋台を作成</h1>

                <div className="flex flex-row gap-4">
                    <Link href="/staff/me">{me.name}</Link>
                    <form action={signOut}>
                        <Button type="submit" variant="outline">
                            サインアウト
                        </Button>
                    </form>
                </div>
            </header>

            <NewShopForm />
        </main>
    );
}
