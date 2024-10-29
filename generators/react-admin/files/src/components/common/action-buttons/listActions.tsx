import {TopToolbar, SelectColumnsButton, FilterButton, CreateButton, ExportButton} from "react-admin";
export const ListActions = (props: any) => {

    return (
        <TopToolbar>
            <FilterButton/>
            <CreateButton/>
            <ExportButton/>
            <SelectColumnsButton/>
        </TopToolbar>
    );
};