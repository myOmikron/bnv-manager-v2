import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Cell, Pie, PieChart, ResponsiveContainer, Tooltip } from "recharts";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { FullClub } from "src/api/generated";
import { Subheading } from "src/components/base/heading";

/**
 * The properties for {@link AdminDashboard}
 */
export type AdminDashboardProps = {};

const COLORS = ["#0088FE", "#3dc7ff", "#55ffec", "#34ffa6"];

/**
 * A Dashboard for the admins
 */
function AdminDashboard(props: AdminDashboardProps) {
    const [tA] = useTranslation("admin-dashboard");

    const [clubs, setClubs] = React.useState<Array<FullClub>>([]);

    /**
     * Refresh the clubs
     */
    const refreshClubs = async () => {
        const res = await Api.admin.clubs.all();

        res.match(
            (clubs) => setClubs(clubs.clubs),
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        refreshClubs().then();
    }, []);

    return (
        <div className={"grid grid-cols-1 gap-3 lg:grid-cols-3"}>
            <Panel heading={tA("heading.users-in-clubs")}>
                <ResponsiveContainer>
                    <PieChart>
                        <Pie
                            data={clubs}
                            innerRadius={60}
                            outerRadius={80}
                            fill="#8884d8"
                            paddingAngle={5}
                            dataKey="users"
                        >
                            {clubs.map((entry, index) => (
                                <Cell key={`cell-${entry.uuid}`} fill={COLORS[index % COLORS.length]} />
                            ))}
                            <Tooltip />
                        </Pie>
                    </PieChart>
                </ResponsiveContainer>
            </Panel>
        </div>
    );
}

/**
 * A panel
 */
function Panel(props: { children: React.ReactNode; heading: string }) {
    return (
        <div className={"h-72 rounded-lg bg-neutral-50 p-6 shadow-lg"}>
            <Subheading>{props.heading}</Subheading>
            {props.children}
        </div>
    );
}

export const Route = createFileRoute("/_admin/a/dashboard")({
    component: AdminDashboard,
});
