import React, { lazy, Suspense } from "react";
import { useTranslation } from "react-i18next";
import USER_CONTEXT from "src/context/user";
import { Button } from "src/components/base/button";
import { ComputerDesktopIcon, MoonIcon, SunIcon } from "@heroicons/react/20/solid";
import { Input } from "src/components/base/input";
import { Field, FieldGroup, Fieldset, Label } from "src/components/base/fieldset";
import Form from "src/components/base/form";
import { Divider } from "src/components/base/divider";
import LanguageSelect, { Lang } from "src/components/base/language-select";
import { useForm } from "@tanstack/react-form";
import TabLayout from "src/components/base/tab-layout";
import { Tab, TabMenu } from "src/components/base/tab-menu";
import { Outlet } from "@tanstack/react-router";
import { Listbox, ListboxLabel, ListboxOption } from "src/components/base/listbox";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { Subheading } from "src/components/base/heading";

/**
 * The properties for {@link ProfileLayout}
 */
export type ProfileProps = {};

/**
 * The profile
 */
export function ProfileLayout(props: ProfileProps) {
    const [tP] = useTranslation("profile");
    const { user } = React.useContext(USER_CONTEXT);

    return (
        <TabLayout
            heading={tP("heading.profile")}
            headingDescription={tP("description.profile")}
            tabs={
                <TabMenu>
                    <Tab
                        href={
                            user.role === "User"
                                ? "/u/profile/general"
                                : user.role === "ClubAdmin"
                                  ? "/ca/profile/general"
                                  : "/a/profile/general"
                        }
                    >
                        {tP("label.general")}
                    </Tab>
                    <Tab
                        href={
                            user.role === "User"
                                ? "/u/profile/security"
                                : user.role === "ClubAdmin"
                                  ? "/ca/profile/security"
                                  : "/a/profile/security"
                        }
                    >
                        {tP("label.security")}
                    </Tab>
                </TabMenu>
            }
        >
            <Outlet />
        </TabLayout>
    );
}

/**
 * The properties for {@link GeneralProfile}
 */
export type GeneralProfileProps = {};

/**
 * The general profile
 */
export function GeneralProfile(props: GeneralProfileProps) {
    const [t, i18n] = useTranslation();
    const { user, reset } = React.useContext(USER_CONTEXT);

    const theme = localStorage.getItem("theme");

    const profileForm = useForm({
        defaultValues: {
            username: user.username,
            display_name: user.display_name,
            preferredLanguage: user.preferred_lang as Lang,
            theme: theme ? theme : "system",
        },
        // eslint-disable-next-line
        onSubmit: async ({ value, formApi }) => {
            const res = await Api.users.updateMe({
                preferred_lang: value.preferredLanguage,
                display_name: value.display_name,
            });

            res.match(
                () => {
                    i18n.changeLanguage(value.preferredLanguage.toLowerCase());
                    if (value.theme === "system") {
                        localStorage.removeItem("theme");
                    } else if (value.theme === "light") {
                        localStorage.setItem("theme", "light");
                    } else {
                        localStorage.setItem("theme", "dark");
                    }

                    if (
                        localStorage.theme === "dark" ||
                        (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches)
                    ) {
                        document.documentElement.classList.add("dark");
                    } else {
                        document.documentElement.classList.remove("dark");
                    }

                    formApi.reset();
                    reset();
                },
                (err) => toast.error(err.message),
            );
        },
    });
    const profileFormDirty = profileForm.useStore((store) => store.isDirty);

    return (
        <div className={"flex flex-col-reverse justify-between gap-6 md:flex-row md:gap-20"}>
            <Form onSubmit={profileForm.handleSubmit} className={"w-full"}>
                <Fieldset className={"flex flex-col gap-8"}>
                    <div
                        data-slot="control"
                        className="grid w-full grid-cols-1 items-center gap-x-4 gap-y-6 sm:grid-cols-2"
                    >
                        <profileForm.Field name={"display_name"}>
                            {(fieldApi) => (
                                <Field className="grid grid-cols-[subgrid] gap-3 sm:col-span-2">
                                    <Label>{t("label.name")}</Label>
                                    <Input
                                        type={"text"}
                                        value={fieldApi.state.value}
                                        onChange={(e) => fieldApi.handleChange(e.target.value)}
                                    />
                                </Field>
                            )}
                        </profileForm.Field>
                    </div>
                    <Divider soft={true} />
                    <div className="grid w-full grid-cols-1 items-center gap-x-4 gap-y-6 sm:grid-cols-2">
                        <profileForm.Field name={"preferredLanguage"}>
                            {(fieldApi) => (
                                <Field className="grid grid-cols-[subgrid] gap-3 sm:col-span-2">
                                    <Label>{t("label.preferred-lang")}</Label>
                                    <LanguageSelect lang={fieldApi.state.value} setLang={fieldApi.handleChange} />
                                </Field>
                            )}
                        </profileForm.Field>
                    </div>

                    <Divider soft={true} />
                    <div className="grid w-full grid-cols-1 items-center gap-x-4 gap-y-6 sm:grid-cols-2">
                        <profileForm.Field name={"theme"}>
                            {(fieldApi) => (
                                <Field className="grid grid-cols-[subgrid] gap-3 sm:col-span-2">
                                    <Label>{t("label.theme")}</Label>
                                    <Listbox value={fieldApi.state.value} onChange={fieldApi.handleChange}>
                                        <ListboxOption value={"system"}>
                                            <ComputerDesktopIcon />
                                            <ListboxLabel>{t("label.theme-system")}</ListboxLabel>
                                        </ListboxOption>
                                        <ListboxOption value={"light"}>
                                            <SunIcon />
                                            <ListboxLabel>{t("label.theme-light")}</ListboxLabel>
                                        </ListboxOption>
                                        <ListboxOption value={"dark"}>
                                            <MoonIcon />
                                            <ListboxLabel>{t("label.theme-dark")}</ListboxLabel>
                                        </ListboxOption>
                                    </Listbox>
                                </Field>
                            )}
                        </profileForm.Field>
                    </div>

                    <Divider soft={true} />

                    <div className={"flex justify-end gap-6"}>
                        <Button plain={true} disabled={!profileFormDirty} onClick={profileForm.reset}>
                            {t("button.reset")}
                        </Button>
                        <Button type={"submit"} color={"blue"} disabled={!profileFormDirty}>
                            {t("button.save")}
                        </Button>
                    </div>
                </Fieldset>
            </Form>
        </div>
    );
}

/**
 * The properties for {@link SecurityProfile}
 */
export type SecurityProfileProps = {};

const PasswordStrength = lazy(() => import("src/components/base/pw-strength"));

/**
 * The profile settings for security related options
 */
export default function SecurityProfile(props: SecurityProfileProps) {
    const [tP] = useTranslation("profile");

    const pwForm = useForm({
        defaultValues: {
            current: "",
            password: "",
            password2: "",
        },
        onSubmit: async ({ value, formApi }) => {
            const res = await Api.users.changePassword(value.current, value.password);

            res.match(
                (ok) => {},
                (err) => toast.error(err.message),
            );
        },
    });

    return (
        <div className={"flex flex-col gap-3"}>
            <Subheading>{tP("heading.set-new-password")}</Subheading>
            <Form onSubmit={pwForm.handleSubmit} className={"mt-3 max-w-sm"}>
                <Fieldset>
                    <FieldGroup>
                        <pwForm.Field name={"current"}>
                            {(fieldApi) => (
                                <Field>
                                    <Label>{tP("label.current-pw")}</Label>
                                    <Input
                                        type={"password"}
                                        value={fieldApi.state.value}
                                        onChange={(e) => fieldApi.setValue(e.target.value)}
                                    />
                                </Field>
                            )}
                        </pwForm.Field>
                        <pwForm.Field name={"password"}>
                            {(fieldApi) => (
                                <>
                                    <Field>
                                        <Label>{tP("label.new-pw")}</Label>
                                        <Input
                                            type={"password"}
                                            value={fieldApi.state.value}
                                            onChange={(e) => fieldApi.setValue(e.target.value)}
                                        />
                                    </Field>

                                    <Suspense fallback={null}>
                                        <PasswordStrength password={fieldApi.state.value} />
                                    </Suspense>
                                </>
                            )}
                        </pwForm.Field>
                        <pwForm.Field name={"password2"}>
                            {(fieldApi) => (
                                <Field>
                                    <Label>{tP("label.repeat-new-pw")}</Label>
                                    <Input
                                        type={"password"}
                                        value={fieldApi.state.value}
                                        onChange={(e) => fieldApi.setValue(e.target.value)}
                                    />
                                </Field>
                            )}
                        </pwForm.Field>

                        <Button type={"submit"}>{tP("button.set-password")}</Button>
                    </FieldGroup>
                </Fieldset>
            </Form>
        </div>
    );
}
