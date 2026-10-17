import { Suspense } from "react";
import { redirect } from "next/navigation";
import { MeSettingsForm } from "./form";
import { getMe } from "@/lib/me";
import StaffHeader from "../staff-header";
import Loading from "@/app/loading";

async function MeContent() {
    const me = await getMe();

    if (!me) {
        redirect("/");
    }

    return (
        <>
            <StaffHeader me={me} />
            <h2 className="text-xl font-bold">ユーザの設定</h2>
            <MeSettingsForm me={me} />
        </>
    );
}

export default function MePage() {
    return (
        <main className="mx-auto min-h-svh w-full max-w-4xl space-y-6 px-4 py-6">
            <Suspense fallback={<Loading />}>
                <MeContent />
            </Suspense>
        </main>
    );
}
