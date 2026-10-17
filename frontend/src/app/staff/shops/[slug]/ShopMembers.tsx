"use client";

import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { Members } from "@/lib/api";
import { addMember } from "@/actions/add-member";

export function ShopMembers({
    shopId,
    members,
}: {
    shopId: string;
    members: Members;
}) {
    const router = useRouter();
    const [email, setEmail] = useState("");
    const [isAdmin, setIsAdmin] = useState(false);
    const [message, setMessage] = useState("");
    const [pending, startTransition] = useTransition();

    function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        setMessage("");

        startTransition(async () => {
            try {
                const success = await addMember(shopId, email.trim(), isAdmin);

                if (!success) {
                    setMessage("メンバーの追加に失敗しました。");
                    return;
                }

                setEmail("");
                setIsAdmin(false);
                setMessage("メンバーを追加しました。");
                router.refresh();
            } catch {
                setMessage(
                    "メンバーの追加に失敗しました。もう一度お試しください。",
                );
            }
        });
    }

    return (
        <section className="space-y-4">
            <div className="flex items-center justify-between">
                <h2 className="text-xl font-semibold">メンバー</h2>
                <span className="text-sm text-muted-foreground">
                    {members.members.length}人
                </span>
            </div>

            <div className="rounded-lg border divide-y">
                {members.members.map((member) => (
                    <div
                        key={member.email}
                        className="flex items-center justify-between gap-3 p-4"
                    >
                        <div className="min-w-0 space-y-1">
                            <p className="font-medium truncate">
                                {member.name || member.email}
                            </p>
                            <p className="text-sm text-muted-foreground break-all">
                                {member.email}
                            </p>
                        </div>

                        {member.is_admin && (
                            <span className="shrink-0 rounded-full bg-primary/10 px-3 py-1 text-xs font-medium text-primary">
                                管理者
                            </span>
                        )}
                    </div>
                ))}

                {members.members.length === 0 && (
                    <p className="p-6 text-center text-sm text-muted-foreground">
                        メンバーはいません。
                    </p>
                )}
            </div>

            <form
                onSubmit={handleSubmit}
                className="space-y-3 rounded-lg border p-4"
            >
                <h3 className="font-semibold">メンバーを追加</h3>

                <div className="space-y-2">
                    <Label htmlFor="member-email">メールアドレス</Label>
                    <Input
                        id="member-email"
                        type="email"
                        placeholder="you@example.com"
                        value={email}
                        onChange={(event) => setEmail(event.target.value)}
                        autoComplete="email"
                        required
                    />
                </div>

                <label className="flex items-center gap-2 text-sm">
                    <input
                        type="checkbox"
                        checked={isAdmin}
                        onChange={(event) => setIsAdmin(event.target.checked)}
                        className="size-4"
                    />
                    管理者として追加
                </label>

                <Button
                    type="submit"
                    disabled={pending || !email.trim()}
                    className="w-full"
                >
                    {pending ? "追加中..." : "メンバーを追加"}
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
        </section>
    );
}
