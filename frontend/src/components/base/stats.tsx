import React from "react";

/**
 * The properties for {@link Stats}
 */
export type StatsProps = {
    /** the label of the stat */
    label: string;
    /** Value */
    value: string | number;
    /** Additional information */
    additional?: React.ReactNode;
};

/**
 * A stats panel
 */
export default function Stats(props: StatsProps) {
    return (
        <div>
            <hr role="presentation" className="w-full border-t border-zinc-950/10 dark:border-white/10" />
            <dd className="mt-6 text-lg/6 font-medium sm:text-sm/6">{props.label}</dd>
            <dt className="mt-3 text-3xl/8 font-semibold sm:text-2xl/8">{props.value}</dt>
            {props.additional && <div className="mt-3 text-sm/6 sm:text-xs/6 ">{props.additional}</div>}
        </div>
    );
}
