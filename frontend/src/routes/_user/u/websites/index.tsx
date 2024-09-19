import { createFileRoute } from "@tanstack/react-router";
import React, { useEffect } from "react";
import { SimpleWebsite } from "src/api/generated";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import HeadingLayout from "src/components/base/heading-layout";
import { Button } from "src/components/base/button";
import { PlusIcon } from "@heroicons/react/20/solid";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "src/components/base/table";

/**
 * The properties for {@link Websites}
 */
export type WebsitesProps = {};

/**
 * The configuration for the websites
 */
export default function Websites(props: WebsitesProps) {
    const [open, setOpen] = React.useState(false);
    const [websites, setWebsites] = React.useState<Array<SimpleWebsite>>([]);

    useEffect(() => {
        Api.websites.getAll().then((res) =>
            res.match(
                (websites) => {
                    setWebsites(() => websites.websites);
                },
                (err) => toast.error(err.message),
            ),
        );
    }, [open]);

    return (
        <>
            <CreateWebsiteDialog
                open={open}
                onClose={() => setOpen(false)}
                onSubmit={(name) => {
                    Api.websites.create(name).then((res) =>
                        res.match(
                            () => setOpen(false),
                            (err) => err.message,
                        ),
                    );
                }}
            />
            <HeadingLayout
                heading={"Website configuration"}
                headingChildren={
                    <Button type={"button"} onClick={() => setOpen(true)}>
                        <PlusIcon />
                        Create
                    </Button>
                }
            >
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableHeader>Name</TableHeader>
                            <TableHeader>Created at</TableHeader>
                            <TableHeader>Status</TableHeader>
                            <TableHeader>Last deployed</TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {websites.map((x) => (
                            <TableRow
                                key={x.uuid}
                                href={"/websites/$websiteId"}
                                params={{ websiteId: x.uuid }}
                                className={"dark:text-white"}
                            >
                                <TableCell>{x.name}</TableCell>
                                <TableCell>
                                    {x.createdAt.toLocaleDateString()}
                                </TableCell>
                                <TableCell>
                                    <DeployStateBadge state={x.deployState} />
                                </TableCell>
                                <TableCell>
                                    {x.lastDeployment?.toLocaleString()}
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </HeadingLayout>
        </>
    );
}

export const Route = createFileRoute("/_user/u/websites/")({
    component: () => <Websites />,
});
