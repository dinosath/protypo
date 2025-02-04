import { BadgeField } from "@/components/BadgeField";
import { Breadcrumb, BreadcrumbItem } from "@/components/Breadcrumb";
// import { DataTable } from "@/components/DataTable";
import { DataTableDemo } from "./DataTable";
import { ReferenceField } from "@/components/ReferenceField";
import { buttonVariants } from "@/components/ui/button";
import { createColumnHelper } from "@tanstack/react-table";
import { ListContextProvider, useListController } from "ra-core";
import { Link } from "react-router-dom";
import {TasksPrimaryButtons} from "@/products/components/tasks-primary-buttons.tsx";
import {tasks} from "@/products/data/tasks.ts";
import {TasksDialogs} from "@/products/components/tasks-dialogs.tsx";
import TasksProvider from "@/products/context/tasks-context.tsx";
import DynamicTable from "@/components/DynamicTable.tsx";
import {DataTable} from "@/components/tasks/data-table.tsx";

type Product = {
  id: number;
  reference: string;
  description: string;
  category_id: number;
  width: number;
  height: number;
  price: number;
  stock: number;
  thumbnail: string;
  image: string;
};

const columnHelper = createColumnHelper<Product>();

const columns = [
  columnHelper.accessor("reference", {
    header: () => "Reference",
    cell: (info) => info.getValue(),
  }),
  columnHelper.accessor("category_id", {
    header: () => "Category",
    cell: (info) => {
      return (
        <ReferenceField
          reference="categories"
          record={info.row.original}
          source="category_id"
          link="edit"
        >
          <BadgeField source="name" />
        </ReferenceField>
      );
    },
  }),
  columnHelper.accessor("stock", {
    header: () => "Stock",
    cell: (info) => info.renderValue(),
    meta: {
      headerClassName: "w-24",
    },
  }),
  columnHelper.accessor((row) => row.id, {
    id: "actions",
    cell: (info) => (
      <Link
        className={buttonVariants({ variant: "outline" })}
        to={info.getValue().toString()}
      >
        Edit
      </Link>
    ),
    enableSorting: false,
    header: () => <span />,
    meta: {
      headerClassName: "w-12",
    },
  }),
];

export const ProductList = () => {
  const context = useListController<Product>();

  if (context.isLoading) {
    return null;
  }

  return (
    <ListContextProvider value={context}>

    <TasksProvider>
          <div className='mb-2 flex flex-wrap items-center justify-between gap-x-4 space-y-2'>
            <div>
              <h2 className='text-2xl font-bold tracking-tight'>Tasks</h2>
              <p className='text-muted-foreground'>
                Here&apos;s a list of your tasks for this month!
              </p>
            </div>
            <TasksPrimaryButtons />
          </div>
          <div className='-mx-4 flex-1 overflow-auto px-4 py-1 lg:flex-row lg:space-x-12 lg:space-y-0'>
            {/*<DataTable data={tasks} columns={columns} />*/}
            <DataTableDemo />
          </div>
</TasksProvider>


        {/*<TasksDialogs />*/}
    </ListContextProvider>
  );
};
