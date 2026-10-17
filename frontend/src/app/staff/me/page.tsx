import { Suspense } from "react";
import { redirect } from "next/navigation";
import { Button } from "@/components/ui/button";
import { MeSettingsForm } from "./form";
import { signOut } from "@/lib/signout";
import { getMe } from "@/lib/me";

function Loading() {
    return <p className="text-muted-foreground">読み込み中...</p>;
}

async function MeContent() {
    const me = await getMe();

    if (!me) {
        redirect("/");
    }

    return (
        <>
            <header className="flex items-center justify-between gap-4">
                <h1 className="text-2xl font-bold">ユーザの設定</h1>

                <div className="flex flex-row items-center gap-4">
                    {me.name}

                    <form action={signOut}>
                        <Button type="submit" variant="outline">
                            サインアウト
                        </Button>
                    </form>
                </div>
            </header>

            <MeSettingsForm me={me} />
        </>
    );
}

export default function MePage() {
    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-8">
            <Suspense fallback={<Loading />}>
                <MeContent />
            </Suspense>
        </main>
    );
}
