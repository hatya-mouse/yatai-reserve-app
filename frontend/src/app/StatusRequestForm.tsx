"use client";

import { Mail } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { requestStatus } from "@/actions/reservation";
import Link from "next/link";

export default function StatusRequestForm() {
    const [email, setEmail] = useState("");
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const success = await requestStatus(email);
            if (!success) throw new Error("Status request failed");

            setMessage(
                "予約確認用のリンクを送信しました。メールを確認してください。",
            );
        } catch {
            setMessage("送信に失敗しました。もう一度お試しください。");
        }

        setLoading(false);
    }

    return (
        <main className="flex min-h-svh items-center justify-center bg-muted/40 px-4 py-12">
            <Card className="w-full max-w-md">
                <CardHeader className="items-center text-center">
                    <CardTitle className="flex flex-row items-center justify-center gap-2 text-lg font-bold">
                        <Mail />
                        予約状況の確認
                    </CardTitle>
                </CardHeader>

                <CardContent className="flex flex-col items-strech">
                    <form className="space-y-4" onSubmit={handleSubmit}>
                        <div className="space-y-2">
                            <Label htmlFor="email">メールアドレス</Label>
                            <Input
                                id="email"
                                name="email"
                                type="email"
                                placeholder="you@example.com"
                                value={email}
                                onChange={(event) =>
                                    setEmail(event.target.value)
                                }
                                autoComplete="email"
                                required
                            />
                        </div>

                        <Button
                            type="submit"
                            className="w-full"
                            disabled={loading}
                        >
                            <Mail className="mr-2 size-4" />
                            {loading ? "送信中..." : "予約確認用リンクを送信"}
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

                    <Link
                        href="/staff/signin"
                        className="mt-2 text-center hover:underline"
                    >
                        運営者のサインインはこちら
                    </Link>
                </CardContent>
            </Card>
        </main>
    );
}
