import React from "react";
import { clsx } from "clsx";
import { useTranslation } from "react-i18next";
import { Text } from "../../components/text";
import { Label } from "../../components/fieldset";

/**
 * The properties for {@link PasswordStrength}
 */
export type PasswordStrengthProps = {
    /** The score of the password */
    score: 0 | 1 | 2 | 3 | 4;
};

/**
 * An indicator for the password strength
 */
export default function PasswordStrength(props: PasswordStrengthProps) {
    const [t] = useTranslation("profile");

    const { score } = props;

    return (
        <div className={"flex flex-col gap-3"}>
            <Label>{t("Password Strength")}</Label>
            <div className={"grid h-3 grid-cols-4 gap-3"}>
                <div
                    className={clsx(
                        "rounded-lg border border-zinc-300 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                        score === 2 && "bg-orange-500",
                        (score === 1 || score === 0) && "bg-red-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded-lg border border-zinc-300 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                        score === 2 && "bg-orange-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded-lg border border-zinc-300 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded-lg border border-zinc-300 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                    )}
                />
            </div>
        </div>
    );
}
