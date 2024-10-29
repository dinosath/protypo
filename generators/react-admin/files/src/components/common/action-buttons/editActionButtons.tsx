import {TopToolbar, PrevNextButtons, ShowButton} from "react-admin";

export const EditActionButtons = () => (
    <TopToolbar>
        <PrevNextButtons linkType="edit"/>
        <ShowButton/>
    </TopToolbar>
);