import { Suspense } from "react";
import { redirect } from "next/navigation";
import { Button } from "@/components/ui/button";
import { signOut } from "@/lib/signout";
import { getMe } from "@/lib/me";
import Link from "next/link";
import { NewShopForm } from "./form";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

async function NewShopContent() {
    const me = await getMe();

    if (!me) {
        redirect("/");
    }

    return (
        <>
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">屋台を作成</h1>

                <div className="flex flex-row items-center gap-4">
                    <Link href="/staff/me">{me.name}</Link>

                    <form action={signOut}>
                        <Button type="submit" variant="outline">
                            サインアウト
                        </Button>
                    </form>
                </div>
            </header>

            <NewShopForm />
        </>
    );
}

export default function NewShopPage() {
    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <Suspense fallback={<Loading />}>
                <NewShopContent />
            </Suspense>
        </main>
    );
}
