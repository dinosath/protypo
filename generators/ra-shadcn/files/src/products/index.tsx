import { ResourceProps } from "react-admin";
import { ProductList } from "./ProductList";
import { ProductEdit } from "./ProductEdit";

export const products: ResourceProps = {
	name: "products",
	list: ProductList,
	edit: ProductEdit,
	recordRepresentation: "reference",
};
