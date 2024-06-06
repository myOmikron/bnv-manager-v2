import React from "react";
import {
    Field,
    FieldGroup,
    Fieldset,
    Label,
    Legend,
} from "../../components/fieldset";
import { Input } from "../../components/input";
import { Button } from "../../components/button";
import { Dialog } from "../../components/dialog";
import zxcvbn from "zxcvbn";
import { useTranslation } from "react-i18next";
import PasswordStrength from "./password-strength";

/**
 * The properties for {@link ChangePasswordDialog}
 */
export type ChangePasswordDialogProps = {
    open: boolean;
    onClose: () => void;
    onSubmit: (currentPw: string, pw: string) => void;
};

/**
 * The dialog to change the user password
 */
export default function ChangePasswordDialog(props: ChangePasswordDialogProps) {
    const [t] = useTranslation("profile");

    const [newPW, setNewPW] = React.useState("");
    const [newPW2, setNewPW2] = React.useState("");
    const [currentPW, setCurrentPW] = React.useState("");
    const res = zxcvbn(newPW);

    return (
        <Dialog
            open={props.open}
            onClose={() => {
                props.onClose();
            }}
        >
            <form
                method={"post"}
                onSubmit={(e) => {
                    e.preventDefault();

                    if (newPW !== newPW2) {
                        return;
                    }

                    props.onSubmit(currentPW, newPW);
                }}
            >
                <Fieldset>
                    <Legend>{t("Password change")}</Legend>
                    <FieldGroup>
                        <Field>
                            <Label>{t("Current Password")}</Label>
                            <Input
                                type={"password"}
                                required={true}
                                value={currentPW}
                                onChange={(e) => {
                                    setCurrentPW(e.target.value);
                                }}
                            ></Input>
                        </Field>
                        <Field>
                            <Label>{t("Password")}</Label>
                            <Input
                                type={"password"}
                                required={true}
                                value={newPW}
                                onChange={(e) => {
                                    setNewPW(e.target.value);
                                }}
                            ></Input>
                        </Field>
                        <PasswordStrength score={res.score} />
                        <Field>
                            <Label>{t("Repeat Password")}</Label>
                            <Input
                                type={"password"}
                                required={true}
                                value={newPW2}
                                onChange={(e) => {
                                    setNewPW2(e.target.value);
                                }}
                            />
                        </Field>
                        <Button color={"orange"}>{t("Change Password")}</Button>
                    </FieldGroup>
                </Fieldset>
            </form>
        </Dialog>
    );
}
