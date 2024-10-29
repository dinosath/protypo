import { BulkActionsToolbar, BulkExportButton, BulkDeleteWithConfirmButton, BulkDeleteButton } from "react-admin";

export const UserBulkActionButtons = props => (

    <BulkActionsToolbar>
        <BulkExportButton />
        <BulkDeleteButton />
        <BulkDeleteWithConfirmButton {...props} />
    </BulkActionsToolbar>

);