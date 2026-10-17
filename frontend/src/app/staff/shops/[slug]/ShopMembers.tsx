"use client";

import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { Members } from "@/lib/api";
import { addMember } from "@/actions/add-member";
import { removeMember } from "@/actions/remove-member";
import { updateMember } from "@/actions/update-member";

export function ShopMembers({
    shopId,
    members,
    currentUserEmail,
    isAdmin,
}: {
    shopId: string;
    members: Members;
    currentUserEmail: string;
    isAdmin: boolean;
}) {
    const router = useRouter();
    const [email, setEmail] = useState("");
    const [newMemberIsAdmin, setNewMemberIsAdmin] = useState(false);
    const [message, setMessage] = useState("");
    const [pending, startTransition] = useTransition();

    function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        setMessage("");

        startTransition(async () => {
            try {
                const success = await addMember(
                    shopId,
                    email.trim(),
                    newMemberIsAdmin,
                );

                if (!success) {
                    setMessage("メンバーの追加に失敗しました。");
                    return;
                }

                setEmail("");
                setNewMemberIsAdmin(false);
                setMessage("メンバーを追加しました。");
                router.refresh();
            } catch {
                setMessage("メンバーの追加に失敗しました。もう一度お試しください。");
            }
        });
    }

    function handleRoleChange(memberEmail: string, nextIsAdmin: boolean) {
        if (memberEmail.toLowerCase() === currentUserEmail.toLowerCase()) {
            setMessage("自分自身の権限は変更できません。");
            return;
        }

        const action = nextIsAdmin ? "管理者に変更" : "管理者権限を解除";

        if (!window.confirm(`${memberEmail}を${action}しますか？`)) {
            return;
        }

        setMessage("");

        startTransition(async () => {
            try {
                const response = await updateMember(
                    shopId,
                    memberEmail,
                    nextIsAdmin,
                );

                if (!response.ok) {
                    setMessage("権限の変更に失敗しました。");
                    return;
                }

                setMessage("権限を変更しました。");
                router.refresh();
            } catch {
                setMessage("権限の変更に失敗しました。もう一度お試しください。");
            }
        });
    }

    async function handleRemove(memberEmail: string) {
        if (memberEmail.toLowerCase() === currentUserEmail.toLowerCase()) {
            setMessage("自分自身を削除することはできません。");
            return;
        }

        if (!window.confirm(`${memberEmail}をメンバーから削除しますか？`)) {
            return;
        }

        setMessage("");

        try {
            const response = await removeMember(shopId, memberEmail);

            console.log("removeMember status:", response.status);

            if (!response.ok) {
                const body = await response.text();
                console.error("removeMember error:", body);
                setMessage(`メンバーの削除に失敗しました（HTTP ${response.status}）。`);
                return;
            }

            setMessage("メンバーを削除しました。");
            router.refresh();
        } catch (error) {
            console.error("removeMember exception:", error);
            setMessage("メンバーの削除に失敗しました。もう一度お試しください。");
        }
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
                {members.members.map((member) => {
                    const isSelf =
                        member.email.toLowerCase() ===
                        currentUserEmail.toLowerCase();

                    return (
                        <div
                            key={member.email}
                            className="flex flex-wrap items-center justify-between gap-3 p-4"
                        >
                            <div className="min-w-0 flex-1 space-y-1">
                                <p className="font-medium truncate">
                                    {member.name || "不明"}
                                    {isSelf && (
                                        <span className="ml-2 text-xs text-muted-foreground">
                                            (自分)
                                        </span>
                                    )}
                                </p>
                                <p className="text-sm text-muted-foreground break-all">
                                    {member.email}
                                </p>
                            </div>

                            <div className="flex flex-wrap items-center gap-2">
                                {member.is_admin ? (
                                    <span className="shrink-0 rounded-full bg-primary/10 px-3 py-1 text-xs font-medium text-primary">
                                        管理者
                                    </span>
                                ) : (
                                    <span className="shrink-0 rounded-full bg-muted px-3 py-1 text-xs font-medium">
                                        メンバー
                                    </span>
                                )}

                                {isAdmin && !isSelf && (
                                    <>
                                        <Button
                                            type="button"
                                            variant="outline"
                                            size="sm"
                                            disabled={pending}
                                            onClick={() =>
                                                handleRoleChange(
                                                    member.email,
                                                    !member.is_admin,
                                                )
                                            }
                                        >
                                            {member.is_admin
                                                ? "管理者解除"
                                                : "管理者にする"}
                                        </Button>

                                        <Button
                                            type="button"
                                            variant="destructive"
                                            size="sm"
                                            disabled={pending}
                                            onClick={() =>
                                                handleRemove(member.email)
                                            }
                                        >
                                            削除
                                        </Button>
                                    </>
                                )}
                            </div>
                        </div>
                    );
                })}

                {members.members.length === 0 && (
                    <p className="p-6 text-center text-sm text-muted-foreground">
                        メンバーはいません。
                    </p>
                )}
            </div>

            {message && (
                <p role="status" className="text-sm text-muted-foreground">
                    {message}
                </p>
            )}

            {isAdmin && (
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
                            checked={newMemberIsAdmin}
                            onChange={(event) =>
                                setNewMemberIsAdmin(event.target.checked)
                            }
                            className="size-4"
                        />
                        管理者として追加
                    </label>

                    <Button
                        type="submit"
                        disabled={pending || !email.trim()}
                        className="w-full"
                    >
                        {pending ? "処理中..." : "メンバーを追加"}
                    </Button>
                </form>
            )}
        </section>
    );
}
