import { clsx } from "clsx";
import { Text } from "src/components/base/text.tsx";
import * as zxcvbnCommonPackage from "@zxcvbn-ts/language-common";
import * as zxcvbnEnPackage from "@zxcvbn-ts/language-en";
import * as zxcvbnDePackage from "@zxcvbn-ts/language-de";
import { zxcvbn, zxcvbnOptions } from "@zxcvbn-ts/core";
import { useTranslation } from "react-i18next";

const options = {
    dictionary: {
        ...zxcvbnCommonPackage.dictionary,
        ...zxcvbnEnPackage.dictionary,
        ...zxcvbnDePackage.dictionary,
    },
    graphs: zxcvbnCommonPackage.adjacencyGraphs,
    translations: zxcvbnEnPackage.translations,
};
zxcvbnOptions.setOptions(options);

/**
 * The properties for {@link PasswordStrength}
 */
export type PasswordStrengthProps = {
    /** The password to calculate the strength for */
    password: string;
};

/**
 * A display for the password strength
 */
export default function PasswordStrength(props: PasswordStrengthProps) {
    const [t] = useTranslation();

    const { password } = props;

    const { score, crackTimesSeconds } = zxcvbn(password);

    const seconds = crackTimesSeconds.offlineSlowHashing1e4PerSecond;

    const century = 315360000;
    const year = 31536000;
    const month = 2678400;
    const week = 604800;
    const day = 86400;
    const hour = 3600;
    const minute = 60;

    const display =
        seconds / century > 1
            ? t("label.centuries")
            : seconds / year > 1
              ? t("label.years")
              : seconds / month > 1
                ? t("label.months")
                : seconds / week > 1
                  ? t("label.weeks")
                  : seconds / day > 1
                    ? t("label.days")
                    : seconds / hour > 1
                      ? t("label.hours")
                      : seconds / minute > 1
                        ? t("label.minutes")
                        : t("label.seconds");

    return (
        <div>
            <div className={"mt-3 mb-2 grid h-1 w-full grid-cols-4 gap-3"}>
                <div
                    className={clsx(
                        "rounded border border-zinc-400 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                        score === 2 && "bg-orange-500",
                        (score === 1 || score === 0) && "bg-purple-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded border border-zinc-400 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                        score === 2 && "bg-orange-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded border border-zinc-400 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                        score === 3 && "bg-yellow-500",
                    )}
                />
                <div
                    className={clsx(
                        "rounded border border-zinc-400 dark:border-zinc-700",
                        score === 4 && "bg-green-500",
                    )}
                />
            </div>
            <Text>{t("label.estimated-crack-time", { time: display })}</Text>
        </div>
    );
}
