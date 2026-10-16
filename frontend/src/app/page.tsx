"use client";

import { Mail, Store } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

export default function Home() {
    const [email, setEmail] = useState("");
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(
        event: React.SubmitEvent<HTMLFormElement>,
    ) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await fetch("/api/request-signin", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ email }),
            });

            if (!response.ok) {
                throw new Error("Request failed");
            }

            setMessage(
                "ログインリンクを送信しました。メールを確認してください。",
            );
        } catch {
            setMessage(
                "送信に失敗しました。もう一度お試しください。",
            );
        } finally {
            setLoading(false);
        }
    }

    return (
        <main className="flex min-h-svh items-center justify-center bg-muted/40 px-4 py-12">
            <Card className="w-full max-w-md">
                <CardHeader className="items-center text-center">
                    <CardTitle className="flex flex-row items-center justify-center gap-2 text-lg font-bold">
                        <Store />
                        運営者向けサインイン
                    </CardTitle>
                </CardHeader>

                <CardContent>
                    <form
                        className="space-y-4"
                        onSubmit={handleSubmit}
                    >
                        <div className="space-y-2">
                            <Label htmlFor="email">
                                メールアドレス
                            </Label>
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
                            {loading
                                ? "送信中..."
                                : "サインインリンクを送信"}
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
                </CardContent>
            </Card>
        </main>
    );
}
