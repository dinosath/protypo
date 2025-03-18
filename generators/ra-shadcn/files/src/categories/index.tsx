import { ResourceProps } from "react-admin";
import { CategoryList } from "./CategoryList";
import { CategoryEdit } from "./CategoryEdit";

export const categories: ResourceProps = {
	name: "categories",
	list: CategoryList,
	edit: CategoryEdit,
	recordRepresentation: "name",
};
