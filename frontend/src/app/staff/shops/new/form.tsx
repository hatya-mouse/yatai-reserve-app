"use client";

import { createNewShop } from "@/actions/create-shop";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useRouter } from "next/navigation";
import { useState } from "react";

export function NewShopForm() {
    const router = useRouter();

    const [name, setName] = useState("");
    const [notifyPrevious, setNotifyPrevious] = useState(true);
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
        event.preventDefault();
        setLoading(true);
        setMessage("");

        try {
            const response = await createNewShop(name, notifyPrevious);

            if (!response.ok) {
                setMessage("屋台の追加に失敗しました。もう一度お試しください。");
                return;
            }

            router.push("/staff");
            router.refresh();
        } catch (error) {
            console.error("屋台の追加に失敗:", error);
            setMessage("屋台の追加に失敗しました。もう一度お試しください。");
        } finally {
            setLoading(false);
        }
    }

    return (
        <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="space-y-2">
                <Label htmlFor="name">屋台名</Label>
                <Input
                    id="name"
                    name="name"
                    value={name}
                    onChange={(event) => setName(event.target.value)}
                    autoComplete="organization"
                    required
                />
            </div>

            <Field orientation="horizontal">
                <Checkbox
                    checked={notifyPrevious}
                    onCheckedChange={(checked) =>
                        setNotifyPrevious(checked === true)
                    }
                />
                <FieldLabel>直前の客が呼ばれた際に通知</FieldLabel>
                <FieldDescription>
                    予約順において1つ前の客が呼ばれたときに、次の客に通知を送信します。
                </FieldDescription>
            </Field>

            <Button
                type="submit"
                className="w-full"
                disabled={loading || name.trim() === ""}
            >
                {loading ? "作成中..." : "屋台を作成"}
            </Button>

            {message && (
                <p role="status" className="text-center text-sm text-muted-foreground">
                    {message}
                </p>
            )}
        </form>
    );
}
