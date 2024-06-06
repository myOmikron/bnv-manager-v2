import React, { lazy, Suspense } from "react";
import HeadingLayout from "../../components/heading-layout";
import { useTranslation } from "react-i18next";
import { Divider } from "../../components/divider";
import { Button } from "../../components/button";
import { Field, FieldGroup, Fieldset, Label } from "../../components/fieldset";
import { Text } from "../../components/text";
import { Listbox, ListboxLabel, ListboxOption } from "../../components/listbox";
import { Api } from "../../api/api";
import { toast } from "react-toastify";

/**
 * The properties for {@link Profile}
 */
export type ProfileProps = {};

/**
 * Profile settings view
 */
export default function Profile(props: ProfileProps) {
    const [t, i18n] = useTranslation("profile");

    const [language, setLanguage] = React.useState<"de" | "en">(
        i18n.resolvedLanguage,
    );
    const [openPWChange, setOpenPWChange] = React.useState<boolean>(false);

    const save = () => {
        i18n.changeLanguage(language).then();
    };

    const ChangePwDialog = lazy(() => import("./change-password-dialog"));

    return (
        <form
            className={"mx-auto max-w-3xl"}
            method={"post"}
            onSubmit={(e) => {
                e.preventDefault();
                save();
            }}
        >
            <HeadingLayout heading={t("Profile")}>
                <Fieldset>
                    <FieldGroup>
                        <Field
                            className={"grid grid-cols-2 items-center gap-12"}
                        >
                            <span className={"flex flex-col gap-2"}>
                                <Label className={"font-bold"}>
                                    {t("Password")}
                                </Label>
                                <Text>{t("Set a new password")}</Text>
                            </span>
                            <Button
                                onClick={() => {
                                    setOpenPWChange(true);
                                }}
                            >
                                {t("Change Password")}
                            </Button>
                        </Field>
                        <Divider />
                        <Field className={"grid grid-cols-2 gap-12"}>
                            <span className={"flex flex-col gap-2"}>
                                <Label className={"font-bold"}>
                                    {t("Language")}
                                </Label>
                                <Text>
                                    {t("The language this site should use")}
                                </Text>
                            </span>
                            <Listbox
                                value={language}
                                onChange={(e) => setLanguage(e)}
                            >
                                <ListboxOption value={"de"}>
                                    <ListboxLabel className={"flex gap-3"}>
                                        <span>🇩🇪</span>
                                        {t("German")}
                                    </ListboxLabel>
                                </ListboxOption>
                                <ListboxOption value={"en"}>
                                    <ListboxLabel className={"flex gap-3"}>
                                        <span>🇺🇸</span>
                                        {t("English")}
                                    </ListboxLabel>
                                </ListboxOption>
                            </Listbox>
                        </Field>
                    </FieldGroup>
                </Fieldset>
                <Divider />
                <div className={"flex justify-end gap-6"}>
                    <Button type={"submit"} color={"orange"}>
                        {t("Save Changes")}
                    </Button>
                </div>
            </HeadingLayout>

            <Suspense fallback={<div>Loading...</div>}>
                <ChangePwDialog
                    open={openPWChange}
                    onClose={() => setOpenPWChange(false)}
                    onSubmit={(currentPw, newPW) => {
                        toast.success("TODO");
                    }}
                />
            </Suspense>
        </form>
    );
}
