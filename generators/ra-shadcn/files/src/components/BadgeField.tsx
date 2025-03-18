import { useRecordContext } from "react-admin";
import get from "lodash/get";
import { Badge } from "./ui/badge";

export const BadgeField = ({ source }: { source: string }) => {
	const record = useRecordContext();
    const value = get(record, source);
	return <Badge variant="outline">{value}</Badge>;
};
