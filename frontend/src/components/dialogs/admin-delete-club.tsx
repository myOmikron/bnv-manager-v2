import React from "react";
import { useTranslation } from "react-i18next";
import { Dialog, DialogActions, DialogBody, DialogDescription, DialogTitle } from "src/components/base/dialog";
import { SimpleClub } from "src/api/generated";
import { Button } from "src/components/base/button";
import { Api } from "src/api/api";

/**
 * The properties for {@link AdminDeleteClubDialog}
 */
export type AdminDeleteClubDialogProps = {
    /** The club to delete */
    club: SimpleClub;

    /** Callback to call when the popup is closed */
    onClose: () => void;

    /** Callback when deletion was executed */
    onDelete: () => void;
};

/**
 * Dialog for deleting a club
 */
export default function AdminDeleteClubDialog(props: AdminDeleteClubDialogProps) {
    const [t] = useTranslation("dialog-delete-club");
    const [tg] = useTranslation();

    /**
     * Delete the club
     */
    const deleteClub = async () => {
        await Api.admin.clubs.delete(props.club.uuid);

        props.onDelete();
    };

    return (
        <Dialog open={true} onClose={props.onClose}>
            <DialogTitle>{t("heading.delete-club", { club: props.club.name })}</DialogTitle>
            <DialogBody>
                <DialogDescription>{t("description.delete-club")}</DialogDescription>
                <DialogActions>
                    <Button plain={true} onClick={props.onClose}>
                        {tg("button.cancel")}
                    </Button>
                    <Button color={"red"} onClick={async () => deleteClub()}>
                        {t("button.delete-club")}
                    </Button>
                </DialogActions>
            </DialogBody>
        </Dialog>
    );
}
