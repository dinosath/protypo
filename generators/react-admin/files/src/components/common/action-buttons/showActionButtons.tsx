import {TopToolbar, PrevNextButtons, EditButton} from "react-admin";

export const ShowActionButtons = () => (
    <TopToolbar>
        <PrevNextButtons linkType="show"/>
        <EditButton/>
    </TopToolbar>
);