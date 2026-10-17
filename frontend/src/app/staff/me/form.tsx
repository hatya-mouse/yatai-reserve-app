"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Me, staffApi } from "@/lib/api";
import { useState } from "react";

export function MeSettingsForm({ me }: { me: Me }) {
    const [name, setName] = useState(me.name);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await staffApi("/api/set-name", {
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
    );
}
