"use client";

import { reserve } from "@/actions/reservation";
import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Shop } from "@/lib/api";
import { Check, Clock } from "lucide-react";
import { useState } from "react";

export default function ReserveForm({ shop }: { shop: Shop }) {
    const [email, setEmail] = useState("");
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await reserve(shop.id, email);

            if (response.status === 409) {
                setMessage("メールアドレスがすでに使用されています。");
                setLoading(false);
                return;
            } else if (!response.ok) {
                throw new Error("Sign-in request failed");
            }

            setMessage("予約を受け付けました。");
        } catch {
            setMessage("予約に失敗しました。もう一度お試しください。");
        }

        setLoading(false);
    }

    return (
        <Card className="w-full max-w-md">
            <CardHeader className="items-center text-center">
                <CardTitle className="flex flex-row items-center justify-center gap-2 text-lg font-bold">
                    <Clock />
                    &quot;{shop.name}&quot; を予約
                </CardTitle>

                <CardDescription>
                    メールアドレスを登録して予約ができます。その後順番が来ると、登録したメールアドレスに通知が送信されます。
                </CardDescription>
            </CardHeader>

            <CardContent>
                <form className="space-y-4" onSubmit={handleSubmit}>
                    <div className="space-y-2">
                        <Label htmlFor="email">メールアドレス</Label>
                        <Input
                            id="email"
                            name="email"
                            type="email"
                            placeholder="you@example.com"
                            value={email}
                            onChange={(event) => setEmail(event.target.value)}
                            autoComplete="email"
                            required
                        />
                    </div>

                    <Button type="submit" className="w-full" disabled={loading}>
                        <Check className="mr-2 size-4" />
                        {loading ? "予約中..." : "予約を確定"}
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
    );
}
