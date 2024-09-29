import React from "react";
import { Dialog, DialogActions, DialogDescription, DialogTitle } from "src/components/base/dialog";
import { useTranslation } from "react-i18next";
import { Button } from "src/components/base/button";

/**
 * The properties for {@link ConfirmDialog}
 */
export type ConfirmDialogProps = {
    /** Dialog title */
    title: string;
    /** Dialog description */
    description: string;
    /** Confirm callback action */
    onConfirm: () => void;
    /** Cancel callback action */
    onCancel: () => void;
};

/**
 * A dialog that allows a confirmation and a cancel operation
 */
export default function ConfirmDialog(props: ConfirmDialogProps) {
    const [t] = useTranslation();

    return (
        <Dialog onClose={props.onCancel} open={true}>
            <DialogTitle>{props.title}</DialogTitle>
            <DialogDescription>{props.description}</DialogDescription>
            <DialogActions>
                <Button plain={true} onClick={props.onCancel}>
                    {t("button.cancel")}
                </Button>
                <Button onClick={props.onConfirm}>{t("button.confirm")}</Button>
            </DialogActions>
        </Dialog>
    );
}
