import { Suspense } from "react";
import { redirect } from "next/navigation";
import { getMe } from "@/lib/me";
import { NewShopForm } from "./form";
import StaffHeader from "../../staff-header";

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
            <StaffHeader me={me} title="屋台の作成" />
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
