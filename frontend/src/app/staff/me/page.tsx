import { Button } from "@/components/ui/button";
import { MeSettingsForm } from "./form";
import { signOut } from "@/lib/signout";
import { getMe } from "@/lib/me";
import { redirect } from "next/navigation";
import Link from "next/link";
import { Suspense } from "react";

export default async function MePage() {
    return (
        <Suspense>
            <MePageContent />
        </Suspense>
    );
}

async function MePageContent() {
    const me = await getMe();
    if (!me) redirect("/");

    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">ユーザの設定</h1>

                <div className="flex flex-row gap-4">
                    <Link href="/staff/me">{me.name}</Link>
                    <form action={signOut}>
                        <Button type="submit" variant="outline">
                            サインアウト
                        </Button>
                    </form>
                </div>
            </header>

            <MeSettingsForm me={me} />
        </main>
    );
}
