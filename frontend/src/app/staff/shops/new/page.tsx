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
import { Me } from "@/lib/api";

export default async function NewShopPage() {
    const me = await getMe();

    return <NewShopPageContent me={me} />;
}

function NewShopPageContent({ me }: { me: Me }) {
    const [name, setName] = useState("");
    const [notifyPrevious, setNotifyPrevious] = useState(true);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await fetch("/api/staff/shops", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ name, notify_previous: notifyPrevious }),
            });

            if (!response.ok) {
                throw new Error("Request failed");
            }

            redirect("/staff");
        } catch {
            setMessage("屋台の追加に失敗しました。もう一度お試しください。");
        } finally {
            setLoading(false);
        }
    }

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

            <form className="space-y-4" onSubmit={handleSubmit}>
                <div className="space-y-2">
                    <Label htmlFor="email">屋台名</Label>
                    <Input
                        id="name"
                        name="name"
                        type="name"
                        value={name}
                        onChange={(event) => setName(event.target.value)}
                        autoComplete="name"
                        required
                    />
                </div>

                <Field orientation="horizontal">
                    <Checkbox
                        checked={notifyPrevious}
                        onCheckedChange={setNotifyPrevious}
                    />
                    <FieldLabel>直前の客が呼ばれた際に通知</FieldLabel>
                    <FieldDescription>
                        予約順において1つ前の客が呼ばれたときに、次の客に通知を送信します。
                    </FieldDescription>
                </Field>

                <Button
                    type="submit"
                    className="w-full"
                    disabled={loading || name == ""}
                >
                    {loading ? "変更中..." : "名前を変更"}
                </Button>

                {message && (
                    <p
                        role="status"
                        className="text-center text-sm text-muted-foreground"
                    >
                        {message}
                    </p>
                )}
            </form>
        </main>
    );
}
