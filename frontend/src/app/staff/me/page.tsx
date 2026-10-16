import { Button } from "@/components/ui/button";
import { signOut } from "@/lib/signout";
import { useState } from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { getMe } from "@/lib/me";
import Link from "next/link";
import { Me } from "@/lib/api";

export default async function StaffPage() {
    const me = await getMe();

    return <StaffPageContent me={me} />;
}

function StaffPageContent({ me }: { me: Me }) {
    "use client";

    const [name, setName] = useState(me.name);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await fetch("/api/set-name", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ name, email: me.email }),
            });

            if (!response.ok) {
                throw new Error("Request failed");
            }

            setMessage("名前を変更しました。");
        } catch {
            setMessage("名前の変更に失敗しました。もう一度お試しください。");
        } finally {
            setLoading(false);
        }
    }

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

            <form className="space-y-4" onSubmit={handleSubmit}>
                <div className="space-y-2">
                    <Label htmlFor="email">ユーザ名</Label>
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
